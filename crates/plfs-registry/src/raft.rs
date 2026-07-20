//! Registry raft group (design doc §4.3, §2: "Registry store | openraft +
//! redb backend | same stack as metadata groups"). The Raft log and vote
//! live in two redb tables of the SAME database as the registry state
//! machine, so apply + last-applied is one atomic transaction. Snapshot
//! policy: Never (compaction is not needed at registry scale).

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

use crate::state::RegistryState;
use crate::{RegistryError, RegistryOp, RegistryResult};

openraft::declare_raft_types!(
    /// Raft types for the registry group.
    pub RegistryRaftConfig:
        D = RegistryOp,
        R = RegistryResult,
        NodeId = u64,
        Node = BasicNode,
        SnapshotData = Cursor<Vec<u8>>,
);

/// Raft-internal key/value table (vote, last_purged, last_log,
/// last_applied, last_membership).
pub(crate) const RAFT_KV: TableDefinition<&str, &[u8]> = TableDefinition::new("raft_kv");
const RAFT_LOG: TableDefinition<u64, &[u8]> = TableDefinition::new("raft_log");

pub(crate) fn enc_bytes<T: serde::Serialize>(v: &T) -> Result<Vec<u8>, RegistryError> {
    bincode::serialize(v).map_err(|e| RegistryError::Codec(e.to_string()))
}

fn dec_bytes<T: serde::de::DeserializeOwned>(b: &[u8]) -> Result<T, RegistryError> {
    bincode::deserialize(b).map_err(|e| RegistryError::Codec(e.to_string()))
}

fn io_err(e: RegistryError) -> std::io::Error {
    std::io::Error::other(e.to_string())
}

fn se_logs(e: RegistryError) -> StorageError<u64> {
    StorageError::from_io_error(ErrorSubject::Logs, ErrorVerb::Write, io_err(e))
}

fn se_store(e: RegistryError) -> StorageError<u64> {
    StorageError::from_io_error(ErrorSubject::Store, ErrorVerb::Write, io_err(e))
}

fn log_id_parts(id: LogId<u64>) -> (u64, u64, u64) {
    (id.index, id.leader_id.term, id.leader_id.node_id)
}

fn make_log_id(index: u64, term: u64, node_id: u64) -> LogId<u64> {
    LogId::new(openraft::LeaderId { term, node_id }, index)
}

fn storage(err: impl std::fmt::Display) -> RegistryError {
    RegistryError::Storage(err.to_string())
}

/// Raft log + vote + applied-state on the registry's redb database.
#[derive(Clone)]
pub struct RegistryRaftStore {
    db: Arc<Database>,
}

impl RegistryRaftStore {
    /// Store on the same database as `state`; creates the raft tables on
    /// first use so read paths never race a missing table.
    pub fn new(state: &RegistryState) -> Result<Self, RegistryError> {
        let db = state.db_handle();
        let txn = db.begin_write().map_err(storage)?;
        txn.open_table(RAFT_KV).map_err(storage)?;
        txn.open_table(RAFT_LOG).map_err(storage)?;
        txn.commit().map_err(storage)?;
        Ok(Self { db })
    }

    fn kv_put(
        &self,
        txn: &redb::WriteTransaction,
        key: &'static str,
        bytes: Vec<u8>,
    ) -> Result<(), RegistryError> {
        let mut kv = txn.open_table(RAFT_KV).map_err(storage)?;
        kv.insert(key, bytes.as_slice()).map_err(storage)?;
        Ok(())
    }

    fn kv_get(&self, key: &str) -> Result<Option<Vec<u8>>, RegistryError> {
        let txn = self.db.begin_read().map_err(storage)?;
        let kv = txn.open_table(RAFT_KV).map_err(storage)?;
        Ok(kv.get(key).map_err(storage)?.map(|v| v.value().to_vec()))
    }

    fn last_log_id(&self) -> Result<Option<LogId<u64>>, RegistryError> {
        match self.kv_get("last_log")? {
            Some(b) => {
                let (index, term, node_id) = dec_bytes(&b)?;
                Ok(Some(make_log_id(index, term, node_id)))
            }
            None => Ok(None),
        }
    }

    fn last_purged_id(&self) -> Result<Option<LogId<u64>>, RegistryError> {
        match self.kv_get("last_purged")? {
            Some(b) => {
                let (index, term, node_id) = dec_bytes(&b)?;
                Ok(Some(make_log_id(index, term, node_id)))
            }
            None => Ok(None),
        }
    }
}

impl RaftLogReader<RegistryRaftConfig> for RegistryRaftStore {
    async fn try_get_log_entries<RB: RangeBounds<u64> + Clone + Debug + Send>(
        &mut self,
        range: RB,
    ) -> Result<Vec<openraft::Entry<RegistryRaftConfig>>, StorageError<u64>> {
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

impl RaftLogStorage<RegistryRaftConfig> for RegistryRaftStore {
    type LogReader = Self;

    async fn get_log_state(
        &mut self,
    ) -> Result<openraft::LogState<RegistryRaftConfig>, StorageError<u64>> {
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
        callback: LogFlushed<RegistryRaftConfig>,
    ) -> Result<(), StorageError<u64>>
    where
        I: IntoIterator<Item = openraft::Entry<RegistryRaftConfig>> + Send,
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
            let new_last = log
                .iter()
                .map_err(storage)
                .map_err(se_logs)?
                .next_back()
                .transpose()
                .map_err(storage)
                .map_err(se_logs)?
                .map(|(_, b)| dec_bytes::<openraft::Entry<RegistryRaftConfig>>(b.value()))
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

/// Snapshot building is disabled at registry scale (`SnapshotPolicy::Never`).
#[derive(Clone)]
pub struct DisabledSnapshotBuilder;

impl RaftSnapshotBuilder<RegistryRaftConfig> for DisabledSnapshotBuilder {
    async fn build_snapshot(&mut self) -> Result<Snapshot<RegistryRaftConfig>, StorageError<u64>> {
        Err(StorageError::from_io_error(
            ErrorSubject::Store,
            ErrorVerb::Read,
            std::io::Error::other("snapshots are disabled at registry scale"),
        ))
    }
}

impl RaftStateMachine<RegistryRaftConfig> for RegistryRaftStore {
    type SnapshotBuilder = DisabledSnapshotBuilder;

    async fn applied_state(
        &mut self,
    ) -> Result<(Option<LogId<u64>>, StoredMembership<u64, BasicNode>), StorageError<u64>> {
        let applied = match self.kv_get("last_applied").map_err(se_store)? {
            Some(b) => {
                let (index, term, node_id) = dec_bytes(&b).map_err(se_store)?;
                Some(make_log_id(index, term, node_id))
            }
            None => None,
        };
        let membership = match self.kv_get("last_membership").map_err(se_store)? {
            Some(b) => dec_bytes(&b).map_err(se_store)?,
            None => StoredMembership::default(),
        };
        Ok((applied, membership))
    }

    async fn apply<I>(&mut self, entries: I) -> Result<Vec<RegistryResult>, StorageError<u64>>
    where
        I: IntoIterator<Item = openraft::Entry<RegistryRaftConfig>> + Send,
        I::IntoIter: Send,
    {
        let state = RegistryState::with_db(self.db.clone());
        let mut out = Vec::new();
        for entry in entries {
            let (index, term, node_id) = log_id_parts(entry.log_id);
            match entry.payload {
                EntryPayload::Normal(op) => {
                    let r = state
                        .apply_logged(&op, index, term, node_id)
                        .map_err(se_store)?;
                    out.push(r);
                }
                EntryPayload::Blank => {
                    let txn = self.db.begin_write().map_err(storage).map_err(se_store)?;
                    self.kv_put(
                        &txn,
                        "last_applied",
                        enc_bytes(&(index, term, node_id)).map_err(se_store)?,
                    )
                    .map_err(se_store)?;
                    txn.commit().map_err(storage).map_err(se_store)?;
                    out.push(RegistryResult::None);
                }
                EntryPayload::Membership(m) => {
                    let txn = self.db.begin_write().map_err(storage).map_err(se_store)?;
                    self.kv_put(
                        &txn,
                        "last_applied",
                        enc_bytes(&(index, term, node_id)).map_err(se_store)?,
                    )
                    .map_err(se_store)?;
                    self.kv_put(
                        &txn,
                        "last_membership",
                        enc_bytes(&StoredMembership::new(Some(entry.log_id), m))
                            .map_err(se_store)?,
                    )
                    .map_err(se_store)?;
                    txn.commit().map_err(storage).map_err(se_store)?;
                    out.push(RegistryResult::None);
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
            std::io::Error::other("snapshots are disabled at registry scale"),
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
            std::io::Error::other("snapshots are disabled at registry scale"),
        ))
    }

    async fn get_current_snapshot(
        &mut self,
    ) -> Result<Option<Snapshot<RegistryRaftConfig>>, StorageError<u64>> {
        Ok(None)
    }
}

/// Single-node network stub (registry group's replication transport reuses
/// plfs-meta's gRPC factory).
#[derive(Clone, Default)]
pub struct RegistryNetwork;

pub struct UnreachableConnection;

impl RaftNetworkFactory<RegistryRaftConfig> for RegistryNetwork {
    type Network = UnreachableConnection;

    async fn new_client(&mut self, _target: u64, _node: &BasicNode) -> Self::Network {
        UnreachableConnection
    }
}

impl RaftNetwork<RegistryRaftConfig> for UnreachableConnection {
    async fn append_entries(
        &mut self,
        _rpc: AppendEntriesRequest<RegistryRaftConfig>,
        _option: openraft::network::RPCOption,
    ) -> Result<AppendEntriesResponse<u64>, RPCError<u64, BasicNode, RaftError<u64>>> {
        unreachable!("replication transport is provided by plfs-meta::transport")
    }

    async fn install_snapshot(
        &mut self,
        _rpc: InstallSnapshotRequest<RegistryRaftConfig>,
        _option: openraft::network::RPCOption,
    ) -> Result<
        openraft::raft::InstallSnapshotResponse<u64>,
        RPCError<u64, BasicNode, RaftError<u64, InstallSnapshotError>>,
    > {
        unreachable!("replication transport is provided by plfs-meta::transport")
    }

    async fn vote(
        &mut self,
        _rpc: VoteRequest<u64>,
        _option: openraft::network::RPCOption,
    ) -> Result<VoteResponse<u64>, RPCError<u64, BasicNode, RaftError<u64>>> {
        unreachable!("replication transport is provided by plfs-meta::transport")
    }
}

/// The registry raft group.
pub struct RegistryRaft {
    raft: Raft<RegistryRaftConfig>,
}

impl RegistryRaft {
    /// Boot the group on the given state store; initialize the membership on
    /// first run (same-membership initialize on all members is safe).
    pub async fn bootstrap_cluster(
        state: &RegistryState,
        node_id: u64,
        members: &BTreeMap<u64, String>,
    ) -> Result<Self, RegistryError> {
        let store = RegistryRaftStore::new(state)?;
        let config = Config {
            cluster_name: "plfs-registry".into(),
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
            let _ = raft.initialize(nodes).await;
        }
        Ok(Self { raft })
    }

    /// The inner raft handle (transport service and metrics).
    pub fn raft(&self) -> &Raft<RegistryRaftConfig> {
        &self.raft
    }

    /// Commit one op through the group.
    pub async fn write(&self, op: RegistryOp) -> Result<RegistryResult, RegistryError> {
        let resp = self.raft.client_write(op).await.map_err(storage)?;
        Ok(resp.data)
    }
}
