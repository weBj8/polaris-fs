//! The redb-backed registry state machine: node inventory + liveness,
//! config, checkpoint pointers, metadata leases, and the pub/sub event log.
//! Every op applies in ONE redb write transaction (atomic by construction).

use std::path::Path;
use std::sync::Arc;

use redb::{Database, ReadableTable, Table, TableDefinition};

use crate::{
    Event, LEASE_TTL_SECS, LeaseRow, NodeKind, NodeRecord, RegistryError, RegistryOp,
    RegistryResult,
};

const NODES: TableDefinition<&str, &[u8]> = TableDefinition::new("nodes");
const CONFIG: TableDefinition<&str, &str> = TableDefinition::new("config");
const CHECKPOINTS: TableDefinition<&str, &[u8]> = TableDefinition::new("checkpoints");
const LEASES: TableDefinition<&str, &[u8]> = TableDefinition::new("leases");
const EVENTS: TableDefinition<u64, &[u8]> = TableDefinition::new("events");
const COUNTERS: TableDefinition<&str, u64> = TableDefinition::new("counters");

fn storage(err: impl std::fmt::Display) -> RegistryError {
    RegistryError::Storage(err.to_string())
}

fn enc<T: serde::Serialize>(v: &T) -> Result<Vec<u8>, RegistryError> {
    bincode::serialize(v).map_err(|e| RegistryError::Codec(e.to_string()))
}

fn dec<T: serde::de::DeserializeOwned>(b: &[u8]) -> Result<T, RegistryError> {
    bincode::deserialize(b).map_err(|e| RegistryError::Codec(e.to_string()))
}

fn unix_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() as i64)
}

fn bump(counters: &mut Table<'_, &str, u64>, name: &'static str) -> Result<u64, RegistryError> {
    let cur = counters
        .get(name)
        .map_err(storage)?
        .map_or(0, |v| v.value());
    counters.insert(name, cur + 1).map_err(storage)?;
    Ok(cur)
}

/// The registry state machine. All mutations go through
/// [`RegistryState::apply`] — one redb write transaction per op.
#[derive(Clone)]
pub struct RegistryState {
    db: Arc<Database>,
}

impl RegistryState {
    /// Create a fresh registry store.
    pub fn create(path: impl AsRef<Path>) -> Result<Self, RegistryError> {
        let state = Self {
            db: Arc::new(Database::create(path).map_err(storage)?),
        };
        let txn = state.db.begin_write().map_err(storage)?;
        {
            txn.open_table(NODES).map_err(storage)?;
            txn.open_table(CONFIG).map_err(storage)?;
            txn.open_table(CHECKPOINTS).map_err(storage)?;
            txn.open_table(LEASES).map_err(storage)?;
            txn.open_table(EVENTS).map_err(storage)?;
            let mut counters = txn.open_table(COUNTERS).map_err(storage)?;
            counters.insert("next_event_seq", 1u64).map_err(storage)?;
        }
        txn.commit().map_err(storage)?;
        Ok(state)
    }

    /// Open an existing registry store.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, RegistryError> {
        Ok(Self {
            db: Arc::new(Database::open(path).map_err(storage)?),
        })
    }

    /// Clone the underlying redb handle (the raft storage shares it).
    pub(crate) fn db_handle(&self) -> Arc<Database> {
        self.db.clone()
    }

    /// Wrap an already-open redb handle (the raft storage impls share the
    /// state machine's database — one file, one lock).
    pub(crate) fn with_db(db: Arc<Database>) -> Self {
        Self { db }
    }

    /// Apply a raft-committed op, recording the applied log id in the SAME
    /// transaction (crash-safe applied-state tracking, like plfs-meta).
    pub(crate) fn apply_logged(
        &self,
        op: &RegistryOp,
        index: u64,
        term: u64,
        node_id: u64,
    ) -> Result<RegistryResult, RegistryError> {
        let txn = self.db.begin_write().map_err(storage)?;
        let result = self.apply_in(&txn, op);
        {
            let mut kv = txn.open_table(crate::raft::RAFT_KV).map_err(storage)?;
            kv.insert(
                "last_applied",
                crate::raft::enc_bytes(&(index, term, node_id))?.as_slice(),
            )
            .map_err(storage)?;
        }
        match result {
            Ok(r) => {
                txn.commit().map_err(storage)?;
                Ok(r)
            }
            Err(e) => Err(e),
        }
    }

    /// Apply one op in a single redb write transaction.
    pub fn apply(&self, op: &RegistryOp) -> Result<RegistryResult, RegistryError> {
        let txn = self.db.begin_write().map_err(storage)?;
        let result = self.apply_in(&txn, op);
        match result {
            Ok(r) => {
                txn.commit().map_err(storage)?;
                Ok(r)
            }
            Err(e) => Err(e),
        }
    }

    fn apply_in(
        &self,
        txn: &redb::WriteTransaction,
        op: &RegistryOp,
    ) -> Result<RegistryResult, RegistryError> {
        let now = unix_now();
        match op {
            RegistryOp::Register { node } => {
                let mut nodes = txn.open_table(NODES).map_err(storage)?;
                let prior_lease_until = {
                    let prior = nodes.get(node.key.as_str()).map_err(storage)?;
                    match &prior {
                        None => None,
                        Some(v) => Some(dec::<NodeRecord>(v.value())?.lease_until),
                    }
                };
                let revived = prior_lease_until.is_none_or(|u| u <= now);
                let mut rec = node.clone();
                rec.last_seen = now;
                rec.lease_until = now + LEASE_TTL_SECS;
                let until = rec.lease_until;
                nodes
                    .insert(rec.key.as_str(), enc(&rec)?.as_slice())
                    .map_err(storage)?;
                drop(nodes);
                if revived {
                    self.emit(txn, &rec.key, "node_up", &rec.addr)?;
                }
                Ok(RegistryResult::LeaseUntil(until))
            }
            RegistryOp::Heartbeat {
                node_key,
                used_bytes,
            } => {
                let mut nodes = txn.open_table(NODES).map_err(storage)?;
                let mut rec = {
                    let Some(v) = nodes.get(node_key.as_str()).map_err(storage)? else {
                        return Err(RegistryError::NotFound(node_key.clone()));
                    };
                    dec::<NodeRecord>(v.value())?
                };
                rec.last_seen = now;
                rec.lease_until = now + LEASE_TTL_SECS;
                rec.used_bytes = *used_bytes;
                let until = rec.lease_until;
                nodes
                    .insert(rec.key.as_str(), enc(&rec)?.as_slice())
                    .map_err(storage)?;
                Ok(RegistryResult::LeaseUntil(until))
            }
            RegistryOp::AcquireLease { volume, holder } => {
                let mut leases = txn.open_table(LEASES).map_err(storage)?;
                let existing = leases
                    .get(volume.as_str())
                    .map_err(storage)?
                    .map(|v| dec::<LeaseRow>(v.value()))
                    .transpose()?;
                let granted = match &existing {
                    None => true,
                    Some(row) => row.holder == *holder || row.until <= now,
                };
                if granted {
                    let row = LeaseRow {
                        holder: holder.clone(),
                        until: now + LEASE_TTL_SECS,
                    };
                    let until = row.until;
                    leases
                        .insert(volume.as_str(), enc(&row)?.as_slice())
                        .map_err(storage)?;
                    drop(leases);
                    self.emit(txn, volume, "leader", holder)?;
                    Ok(RegistryResult::Lease {
                        granted: true,
                        until,
                        holder: holder.clone(),
                    })
                } else {
                    let row = existing.expect("checked");
                    Ok(RegistryResult::Lease {
                        granted: false,
                        until: 0,
                        holder: row.holder,
                    })
                }
            }
            RegistryOp::RenewLease { volume, holder } => {
                let mut leases = txn.open_table(LEASES).map_err(storage)?;
                let existing = leases
                    .get(volume.as_str())
                    .map_err(storage)?
                    .map(|v| dec::<LeaseRow>(v.value()))
                    .transpose()?;
                match existing {
                    Some(row) if row.holder == *holder && row.until > now => {
                        let until = now + LEASE_TTL_SECS;
                        leases
                            .insert(
                                volume.as_str(),
                                enc(&LeaseRow {
                                    holder: holder.clone(),
                                    until,
                                })?
                                .as_slice(),
                            )
                            .map_err(storage)?;
                        Ok(RegistryResult::Lease {
                            granted: true,
                            until,
                            holder: holder.clone(),
                        })
                    }
                    other => Ok(RegistryResult::Lease {
                        granted: false,
                        until: 0,
                        holder: other.map_or_else(String::new, |r| r.holder),
                    }),
                }
            }
            RegistryOp::ReleaseLease { volume, holder } => {
                let mut leases = txn.open_table(LEASES).map_err(storage)?;
                let existing = leases
                    .get(volume.as_str())
                    .map_err(storage)?
                    .map(|v| dec::<LeaseRow>(v.value()))
                    .transpose()?;
                if matches!(&existing, Some(row) if row.holder == *holder) {
                    leases.remove(volume.as_str()).map_err(storage)?;
                }
                Ok(RegistryResult::None)
            }
        }
    }

    fn emit(
        &self,
        txn: &redb::WriteTransaction,
        node_key: &str,
        kind: &str,
        detail: &str,
    ) -> Result<(), RegistryError> {
        let mut counters = txn.open_table(COUNTERS).map_err(storage)?;
        let seq = bump(&mut counters, "next_event_seq")?;
        let mut events = txn.open_table(EVENTS).map_err(storage)?;
        events
            .insert(
                seq,
                enc(&Event {
                    seq,
                    node_key: node_key.to_string(),
                    kind: kind.to_string(),
                    detail: detail.to_string(),
                })?
                .as_slice(),
            )
            .map_err(storage)?;
        Ok(())
    }

    /// All registered nodes (any kind).
    pub fn list_nodes(&self, kind: Option<NodeKind>) -> Result<Vec<NodeRecord>, RegistryError> {
        let txn = self.db.begin_read().map_err(storage)?;
        let nodes = txn.open_table(NODES).map_err(storage)?;
        let mut out = Vec::new();
        for row in nodes.iter().map_err(storage)? {
            let (_, v) = row.map_err(storage)?;
            let rec = dec::<NodeRecord>(v.value())?;
            if kind.is_none() || kind == Some(rec.kind) {
                out.push(rec);
            }
        }
        Ok(out)
    }

    /// Aggregated capacity over live data nodes.
    pub fn stats(&self) -> Result<(u64, u64, u64), RegistryError> {
        let now = unix_now();
        let mut capacity = 0u64;
        let mut used = 0u64;
        let mut live = 0u64;
        for rec in self.list_nodes(Some(NodeKind::Data))? {
            if rec.lease_until > now {
                capacity += rec.capacity_bytes;
                used += rec.used_bytes;
                live += 1;
            }
        }
        Ok((capacity, used, live))
    }

    /// Events with seq >= `since` (Watch backfill).
    pub fn events_since(&self, since: u64) -> Result<Vec<Event>, RegistryError> {
        let txn = self.db.begin_read().map_err(storage)?;
        let events = txn.open_table(EVENTS).map_err(storage)?;
        let mut out = Vec::new();
        for row in events.range(since..).map_err(storage)? {
            let (_, v) = row.map_err(storage)?;
            out.push(dec::<Event>(v.value())?);
        }
        Ok(out)
    }

    /// Latest event sequence number (0 when empty).
    pub fn last_event_seq(&self) -> Result<u64, RegistryError> {
        let txn = self.db.begin_read().map_err(storage)?;
        let counters = txn.open_table(COUNTERS).map_err(storage)?;
        Ok(counters
            .get("next_event_seq")
            .map_err(storage)?
            .map_or(0, |v| v.value() - 1))
    }
}
