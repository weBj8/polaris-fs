//! Single-node openraft group driving the metadata state machine (design
//! doc §7.3). The Raft log and vote live in two redb tables of the SAME
//! database as the state machine, so "apply entry + record last-applied +
//! purge log" can be one atomic transaction; log compaction reuses the S15
//! snapshot machinery (disabled at S8 — `SnapshotPolicy::Never`).
//!
//! S12 turns this into 3-node groups by implementing a real network; the
//! storage and state-machine code paths are already final.

use std::collections::BTreeMap;
use std::fmt::Debug;
use std::io::Cursor;
use std::ops::RangeBounds;
use std::sync::Arc;

use openraft::error::{InstallSnapshotError, RPCError, RaftError};
use openraft::raft::{
    AppendEntriesRequest, AppendEntriesResponse, InstallSnapshotRequest, VoteRequest, VoteResponse,
};
use openraft::storage::LogFlushed;
use openraft::storage::{RaftLogReader, RaftLogStorage, RaftSnapshotBuilder, RaftStateMachine};
use openraft::{
    BasicNode, Config, EntryPayload, ErrorSubject, ErrorVerb, LogId, Raft, RaftNetwork,
    RaftNetworkFactory, Snapshot, SnapshotMeta, SnapshotPolicy, StorageError, StoredMembership,
    Vote,
};
use redb::{Database, ReadableTable, TableDefinition};

use crate::{MetaError, MetaOp, MetaState, OpResult, Time};

openraft::declare_raft_types!(
    /// Raft types for the per-volume metadata group.
    pub MetaRaftConfig:
        D = MetaOp,
        R = Result<OpResult, MetaError>,
        NodeId = u64,
        Node = BasicNode,
        SnapshotData = Cursor<Vec<u8>>,
);

/// Raft-internal key/value table (vote, last_purged, last_log,
/// last_applied, last_membership) — lives beside the state machine tables.
pub(crate) const RAFT_KV: TableDefinition<&str, &[u8]> = TableDefinition::new("raft_kv");
const RAFT_LOG: TableDefinition<u64, &[u8]> = TableDefinition::new("raft_log");

pub(crate) fn enc_bytes<T: serde::Serialize>(v: &T) -> Result<Vec<u8>, MetaError> {
    bincode::serialize(v).map_err(|e| MetaError::Codec(e.to_string()))
}

fn dec_bytes<T: serde::de::DeserializeOwned>(b: &[u8]) -> Result<T, MetaError> {
    bincode::deserialize(b).map_err(|e| MetaError::Codec(e.to_string()))
}

fn io_err(e: MetaError) -> std::io::Error {
    std::io::Error::other(e.to_string())
}

fn se_logs(e: MetaError) -> StorageError<u64> {
    StorageError::from_io_error(ErrorSubject::Logs, ErrorVerb::Write, io_err(e))
}

fn se_apply(e: MetaError, log_id: LogId<u64>) -> StorageError<u64> {
    StorageError::from_io_error(ErrorSubject::Apply(log_id), ErrorVerb::Write, io_err(e))
}

fn log_id_parts(id: LogId<u64>) -> (u64, u64, u64) {
    (id.index, id.leader_id.term, id.leader_id.node_id)
}

fn make_log_id(index: u64, term: u64, node_id: u64) -> LogId<u64> {
    LogId::new(openraft::LeaderId { term, node_id }, index)
}

/// (last_log as (index, term, node_id), raft_log row count) debug dump.
pub type LogStateDump = (Option<(u64, u64, u64)>, u64);

/// Raft log + vote + applied-state on the state machine's redb database.
/// Single-writer by raft core; all writes are single redb transactions
/// (durable on commit).
#[derive(Clone)]
pub struct RedbRaftStore {
    db: Arc<Database>,
    db_path: std::path::PathBuf,
}

impl RedbRaftStore {
    /// Store on the same database as `state`; creates the raft tables on
    /// first use so read paths never race a missing table.
    pub fn new(state: &MetaState) -> Result<Self, MetaError> {
        let db = state.db_handle();
        let txn = db.begin_write().map_err(storage)?;
        txn.open_table(RAFT_KV).map_err(storage)?;
        txn.open_table(RAFT_LOG).map_err(storage)?;
        txn.commit().map_err(storage)?;
        Ok(Self {
            db,
            db_path: state.db_path().to_path_buf(),
        })
    }

    fn kv_put(
        &self,
        txn: &redb::WriteTransaction,
        key: &'static str,
        bytes: Vec<u8>,
    ) -> Result<(), MetaError> {
        let mut kv = txn.open_table(RAFT_KV).map_err(storage)?;
        kv.insert(key, bytes.as_slice()).map_err(storage)?;
        Ok(())
    }

    fn kv_get(&self, key: &str) -> Result<Option<Vec<u8>>, MetaError> {
        let txn = self.db.begin_read().map_err(storage)?;
        let kv = txn.open_table(RAFT_KV).map_err(storage)?;
        Ok(kv.get(key).map_err(storage)?.map(|v| v.value().to_vec()))
    }

    fn last_log_id(&self) -> Result<Option<LogId<u64>>, MetaError> {
        match self.kv_get("last_log")? {
            Some(b) => {
                let (index, term, node_id) = dec_bytes(&b)?;
                Ok(Some(make_log_id(index, term, node_id)))
            }
            None => Ok(None),
        }
    }

    /// Debug dump: last_log parts (index, term, node) and raft_log row count.
    pub fn debug_log_state(&self) -> Result<LogStateDump, MetaError> {
        let last = self.last_log_id()?.map(log_id_parts);
        let txn = self.db.begin_read().map_err(storage)?;
        let log = txn.open_table(RAFT_LOG).map_err(storage)?;
        let count = log.iter().map_err(storage)?.count() as u64;
        Ok((last, count))
    }

    fn last_purged_id(&self) -> Result<Option<LogId<u64>>, MetaError> {
        match self.kv_get("last_purged")? {
            Some(b) => {
                let (index, term, node_id) = dec_bytes(&b)?;
                Ok(Some(make_log_id(index, term, node_id)))
            }
            None => Ok(None),
        }
    }
}

fn storage(err: impl std::fmt::Display) -> MetaError {
    MetaError::Storage(err.to_string())
}

impl RaftLogReader<MetaRaftConfig> for RedbRaftStore {
    async fn try_get_log_entries<RB: RangeBounds<u64> + Clone + Debug + Send>(
        &mut self,
        range: RB,
    ) -> Result<Vec<openraft::Entry<MetaRaftConfig>>, StorageError<u64>> {
        let txn = self.db.begin_read().map_err(storage).map_err(se_logs)?;
        let log = txn.open_table(RAFT_LOG).map_err(storage).map_err(se_logs)?;
        let mut out = Vec::new();
        for row in log.range(range).map_err(storage).map_err(se_logs)? {
            let (_, bytes) = row.map_err(storage).map_err(se_logs)?;
            out.push(dec_bytes(bytes.value()).map_err(se_logs)?);
        }
        Ok(out)
    }
}

impl RaftLogStorage<MetaRaftConfig> for RedbRaftStore {
    type LogReader = Self;

    async fn get_log_state(
        &mut self,
    ) -> Result<openraft::LogState<MetaRaftConfig>, StorageError<u64>> {
        Ok(openraft::LogState {
            last_purged_log_id: self.last_purged_id().map_err(se_logs)?,
            last_log_id: self.last_log_id().map_err(se_logs)?,
        })
    }

    async fn get_log_reader(&mut self) -> Self::LogReader {
        self.clone()
    }

    async fn save_vote(&mut self, vote: &Vote<u64>) -> Result<(), StorageError<u64>> {
        let bytes = enc_bytes(vote).map_err(se_logs)?;
        let txn = self.db.begin_write().map_err(storage).map_err(se_logs)?;
        self.kv_put(&txn, "vote", bytes).map_err(se_logs)?;
        txn.commit().map_err(storage).map_err(se_logs)?;
        Ok(())
    }

    async fn read_vote(&mut self) -> Result<Option<Vote<u64>>, StorageError<u64>> {
        match self.kv_get("vote").map_err(se_logs)? {
            Some(b) => Ok(Some(dec_bytes(&b).map_err(se_logs)?)),
            None => Ok(None),
        }
    }

    async fn append<I>(
        &mut self,
        entries: I,
        callback: LogFlushed<MetaRaftConfig>,
    ) -> Result<(), StorageError<u64>>
    where
        I: IntoIterator<Item = openraft::Entry<MetaRaftConfig>> + Send,
        I::IntoIter: Send,
    {
        let txn = self.db.begin_write().map_err(storage).map_err(se_logs)?;
        {
            let mut log = txn.open_table(RAFT_LOG).map_err(storage).map_err(se_logs)?;
            for entry in entries {
                let bytes = enc_bytes(&entry).map_err(se_logs)?;
                log.insert(entry.log_id.index, bytes.as_slice())
                    .map_err(storage)
                    .map_err(se_logs)?;
                self.kv_put(
                    &txn,
                    "last_log",
                    enc_bytes(&log_id_parts(entry.log_id)).map_err(se_logs)?,
                )
                .map_err(se_logs)?;
            }
        }
        // Durable on commit; the flush callback fires afterwards.
        let res = txn.commit().map_err(storage).map_err(se_logs);
        match res {
            Ok(()) => {
                callback.log_io_completed(Ok(()));
                Ok(())
            }
            Err(e) => {
                callback.log_io_completed(Err(std::io::Error::other(e.to_string())));
                Err(e)
            }
        }
    }

    async fn truncate(&mut self, log_id: LogId<u64>) -> Result<(), StorageError<u64>> {
        let txn = self.db.begin_write().map_err(storage).map_err(se_logs)?;
        {
            let mut log = txn.open_table(RAFT_LOG).map_err(storage).map_err(se_logs)?;
            let keys: Vec<u64> = log
                .range(log_id.index..)
                .map_err(storage)
                .map_err(se_logs)?
                .filter_map(|r| r.ok().map(|(k, _)| k.value()))
                .collect();
            for k in keys {
                log.remove(k).map_err(storage).map_err(se_logs)?;
            }
            // Recompute last_log from the remaining tail.
            let new_last = log
                .iter()
                .map_err(storage)
                .map_err(se_logs)?
                .next_back()
                .transpose()
                .map_err(storage)
                .map_err(se_logs)?
                .map(|(_, b)| dec_bytes::<openraft::Entry<MetaRaftConfig>>(b.value()))
                .transpose()
                .map_err(se_logs)?
                .map(|e| e.log_id);
            match new_last {
                Some(id) => self
                    .kv_put(
                        &txn,
                        "last_log",
                        enc_bytes(&log_id_parts(id)).map_err(se_logs)?,
                    )
                    .map_err(se_logs)?,
                None => {
                    let purged = self.last_purged_id().map_err(se_logs)?;
                    match purged {
                        Some(id) => self
                            .kv_put(
                                &txn,
                                "last_log",
                                enc_bytes(&log_id_parts(id)).map_err(se_logs)?,
                            )
                            .map_err(se_logs)?,
                        None => {
                            let mut kv =
                                txn.open_table(RAFT_KV).map_err(storage).map_err(se_logs)?;
                            kv.remove("last_log").map_err(storage).map_err(se_logs)?;
                        }
                    }
                }
            }
        }
        txn.commit().map_err(storage).map_err(se_logs)?;
        Ok(())
    }

    async fn purge(&mut self, log_id: LogId<u64>) -> Result<(), StorageError<u64>> {
        let txn = self.db.begin_write().map_err(storage).map_err(se_logs)?;
        {
            let mut log = txn.open_table(RAFT_LOG).map_err(storage).map_err(se_logs)?;
            let keys: Vec<u64> = log
                .range(..=log_id.index)
                .map_err(storage)
                .map_err(se_logs)?
                .filter_map(|r| r.ok().map(|(k, _)| k.value()))
                .collect();
            for k in keys {
                log.remove(k).map_err(storage).map_err(se_logs)?;
            }
        }
        self.kv_put(
            &txn,
            "last_purged",
            enc_bytes(&log_id_parts(log_id)).map_err(se_logs)?,
        )
        .map_err(se_logs)?;
        txn.commit().map_err(storage).map_err(se_logs)?;
        Ok(())
    }
}

/// Snapshot building is disabled at S8 (`SnapshotPolicy::Never`); S15
/// implements the real checkpoint (= redb savepoint = Raft snapshot).
#[derive(Clone)]
pub struct DisabledSnapshotBuilder;

impl RaftSnapshotBuilder<MetaRaftConfig> for DisabledSnapshotBuilder {
    async fn build_snapshot(&mut self) -> Result<Snapshot<MetaRaftConfig>, StorageError<u64>> {
        Err(StorageError::from_io_error(
            ErrorSubject::Store,
            ErrorVerb::Read,
            std::io::Error::other("snapshots are disabled at S8"),
        ))
    }
}

impl RaftStateMachine<MetaRaftConfig> for RedbRaftStore {
    type SnapshotBuilder = DisabledSnapshotBuilder;

    async fn applied_state(
        &mut self,
    ) -> Result<(Option<LogId<u64>>, StoredMembership<u64, BasicNode>), StorageError<u64>> {
        let applied = match self
            .kv_get("last_applied")
            .map_err(|e| se_apply(e, LogId::default()))?
        {
            Some(b) => {
                let (index, term, node_id) =
                    dec_bytes(&b).map_err(|e| se_apply(e, LogId::default()))?;
                Some(make_log_id(index, term, node_id))
            }
            None => None,
        };
        let membership = match self
            .kv_get("last_membership")
            .map_err(|e| se_apply(e, LogId::default()))?
        {
            Some(b) => dec_bytes(&b).map_err(|e| se_apply(e, LogId::default()))?,
            None => StoredMembership::default(),
        };
        Ok((applied, membership))
    }

    async fn apply<I>(
        &mut self,
        entries: I,
    ) -> Result<Vec<Result<OpResult, MetaError>>, StorageError<u64>>
    where
        I: IntoIterator<Item = openraft::Entry<MetaRaftConfig>> + Send,
        I::IntoIter: Send,
    {
        let mut state = MetaState::with_db(self.db.clone(), self.db_path.clone());
        let mut out = Vec::new();
        for entry in entries {
            let (index, term, node_id) = log_id_parts(entry.log_id);
            let now: Time = (0, 0);
            match entry.payload {
                EntryPayload::Normal(op) => {
                    let r = state
                        .apply_logged(&op, index, term, node_id, now)
                        .map_err(|e| se_apply(e, entry.log_id))?;
                    let r = match (&op, r) {
                        (MetaOp::CreateSnap { .. }, Ok(OpResult::SnapId(id))) => {
                            match state.checkpoint_copy(id) {
                                Ok(()) => Ok(OpResult::SnapId(id)),
                                Err(e) => Err(e),
                            }
                        }
                        (MetaOp::DeleteSnap { id }, Ok(res)) => {
                            let _ = state.delete_checkpoint_file(*id);
                            Ok(res)
                        }
                        (_, r) => r,
                    };
                    out.push(r);
                }
                EntryPayload::Blank => {
                    let r = state
                        .apply_logged(&MetaOp::GcDone { seqs: vec![] }, index, term, node_id, now)
                        .map_err(|e| se_apply(e, entry.log_id))?;
                    let _ = r;
                    out.push(Ok(OpResult::None));
                }
                EntryPayload::Membership(m) => {
                    let txn = self
                        .db
                        .begin_write()
                        .map_err(storage)
                        .map_err(|e| se_apply(e, entry.log_id))?;
                    self.kv_put(
                        &txn,
                        "last_applied",
                        enc_bytes(&(index, term, node_id))
                            .map_err(|e| se_apply(e, entry.log_id))?,
                    )
                    .map_err(|e| se_apply(e, entry.log_id))?;
                    self.kv_put(
                        &txn,
                        "last_membership",
                        enc_bytes(&StoredMembership::new(Some(entry.log_id), m))
                            .map_err(|e| se_apply(e, entry.log_id))?,
                    )
                    .map_err(|e| se_apply(e, entry.log_id))?;
                    txn.commit()
                        .map_err(storage)
                        .map_err(|e| se_apply(e, entry.log_id))?;
                    out.push(Ok(OpResult::None));
                }
            }
        }
        Ok(out)
    }

    async fn get_snapshot_builder(&mut self) -> Self::SnapshotBuilder {
        DisabledSnapshotBuilder
    }

    async fn begin_receiving_snapshot(
        &mut self,
    ) -> Result<Box<Cursor<Vec<u8>>>, StorageError<u64>> {
        Err(StorageError::from_io_error(
            ErrorSubject::Store,
            ErrorVerb::Read,
            std::io::Error::other("snapshots are disabled at S8"),
        ))
    }

    async fn install_snapshot(
        &mut self,
        _meta: &SnapshotMeta<u64, BasicNode>,
        _snapshot: Box<Cursor<Vec<u8>>>,
    ) -> Result<(), StorageError<u64>> {
        Err(StorageError::from_io_error(
            ErrorSubject::Store,
            ErrorVerb::Write,
            std::io::Error::other("snapshots are disabled at S8"),
        ))
    }

    async fn get_current_snapshot(
        &mut self,
    ) -> Result<Option<Snapshot<MetaRaftConfig>>, StorageError<u64>> {
        Ok(None)
    }
}

/// Single-node network: with one member there are no RPCs; the stubs only
/// satisfy the factory trait.
#[derive(Clone, Default)]
pub struct SingleNodeNetwork;

pub struct UnreachableConnection;

impl RaftNetworkFactory<MetaRaftConfig> for SingleNodeNetwork {
    type Network = UnreachableConnection;

    async fn new_client(&mut self, _target: u64, _node: &BasicNode) -> Self::Network {
        UnreachableConnection
    }
}

impl RaftNetwork<MetaRaftConfig> for UnreachableConnection {
    async fn append_entries(
        &mut self,
        _rpc: AppendEntriesRequest<MetaRaftConfig>,
        _option: openraft::network::RPCOption,
    ) -> Result<AppendEntriesResponse<u64>, RPCError<u64, BasicNode, RaftError<u64>>> {
        unreachable!("single-node group never replicates")
    }

    async fn install_snapshot(
        &mut self,
        _rpc: InstallSnapshotRequest<MetaRaftConfig>,
        _option: openraft::network::RPCOption,
    ) -> Result<
        openraft::raft::InstallSnapshotResponse<u64>,
        RPCError<u64, BasicNode, RaftError<u64, InstallSnapshotError>>,
    > {
        unreachable!("single-node group never replicates")
    }

    async fn vote(
        &mut self,
        _rpc: VoteRequest<u64>,
        _option: openraft::network::RPCOption,
    ) -> Result<VoteResponse<u64>, RPCError<u64, BasicNode, RaftError<u64>>> {
        unreachable!("single-node group never votes remotely")
    }
}

/// A single-node metadata Raft group (design doc §7.3: one group per
/// volume, 1 member in standalone).
pub struct MetaRaft {
    raft: Raft<MetaRaftConfig>,
}

impl MetaRaft {
    /// Boot the group on the given state machine store; initialize the
    /// single-node membership on first run.
    pub async fn bootstrap(state: &MetaState, node_id: u64) -> Result<Self, MetaError> {
        let store = RedbRaftStore::new(state)?;
        let config = Config {
            cluster_name: format!("plfs-meta-{node_id}"),
            snapshot_policy: SnapshotPolicy::Never,
            ..Default::default()
        };
        let config = Arc::new(config.validate().map_err(storage)?);
        let raft = Raft::new(node_id, config, SingleNodeNetwork, store.clone(), store)
            .await
            .map_err(storage)?;
        if !raft.is_initialized().await.map_err(storage)? {
            raft.initialize(BTreeMap::from([(node_id, BasicNode::new("loopback"))]))
                .await
                .map_err(storage)?;
        }
        Ok(Self { raft })
    }

    /// Boot a multi-member group (design doc §7.3: 3 members per volume).
    /// `members` maps node id → transport address and must include this
    /// node; every member calls initialize with the same membership (safe
    /// per openraft's cluster-formation contract). Election tuning targets
    /// the §7.3 failover budget (< 1 s).
    pub async fn bootstrap_cluster(
        state: &MetaState,
        node_id: u64,
        members: &BTreeMap<u64, String>,
    ) -> Result<Self, MetaError> {
        let store = RedbRaftStore::new(state)?;
        let config = Config {
            cluster_name: "plfs-meta".into(),
            snapshot_policy: SnapshotPolicy::Never,
            heartbeat_interval: 50,
            election_timeout_min: 150,
            election_timeout_max: 400,
            ..Default::default()
        };
        let config = Arc::new(config.validate().map_err(storage)?);
        let raft = Raft::new(
            node_id,
            config,
            crate::transport::GrpcNetworkFactory,
            store.clone(),
            store,
        )
        .await
        .map_err(storage)?;
        if !raft.is_initialized().await.map_err(storage)? {
            let nodes: BTreeMap<u64, BasicNode> = members
                .iter()
                .map(|(id, addr)| (*id, BasicNode::new(addr.clone())))
                .collect();
            // Already-initialized is fine (a peer initialized first); any
            // other error is fatal.
            let _ = raft.initialize(nodes).await;
        }
        Ok(Self { raft })
    }

    /// The inner raft handle (transport service and metrics).
    pub fn raft(&self) -> &Raft<MetaRaftConfig> {
        &self.raft
    }

    /// Commit one metadata op through the group; the inner Result is the
    /// op's business outcome (e.g. Exists).
    pub async fn write(&self, op: MetaOp) -> Result<Result<OpResult, MetaError>, MetaError> {
        let resp = self.raft.client_write(op).await.map_err(storage)?;
        Ok(resp.data)
    }

    /// Stop the raft core (releases the store handles so the database can
    /// be reopened elsewhere).
    pub async fn shutdown(&self) -> Result<(), MetaError> {
        self.raft.shutdown().await.map_err(storage)
    }
}
