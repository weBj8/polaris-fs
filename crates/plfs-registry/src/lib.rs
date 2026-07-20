//! The Registry (design doc §4.3): cluster control plane. A 3-node openraft
//! group (single-node in standalone) holding node inventory + liveness
//! leases, cluster config, checkpoint pointers, a pub/sub event log, and the
//! metadata lease issuer. **No file metadata lives here** — registry loss
//! pauses membership changes only.
//!
//! Layout: `state` (redb state machine), `raft` (openraft group, same stack
//! as plfs-meta::raft per design doc §2), `service` (the gRPC surface with
//! leader hints), `client` (a small leader-tracking client).

pub mod client;
pub mod raft;
pub mod service;
pub mod state;
pub mod transport;

use serde::{Deserialize, Serialize};

/// Liveness lease TTL (design doc §10.2: "Registry lease ~5 s").
pub const LEASE_TTL_SECS: i64 = 5;

/// Node role in the cluster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeKind {
    /// ChunkArena host (capacity unit).
    Data,
    /// Metadata raft group member.
    Meta,
    /// FUSE client (volume owner).
    Client,
    /// Registry group member.
    Registry,
}

/// One registered node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeRecord {
    /// Caller-chosen unique key (e.g. "data-0").
    pub key: String,
    /// Role.
    pub kind: NodeKind,
    /// host:port of its service endpoint.
    pub addr: String,
    /// Meta/client: owned volume name ("" otherwise).
    pub volume: String,
    /// Data: total payload capacity.
    pub capacity_bytes: u64,
    /// Data: used payload bytes.
    pub used_bytes: u64,
    /// Registry-side last heartbeat (unix seconds).
    pub last_seen: i64,
    /// Liveness lease deadline (unix seconds).
    pub lease_until: i64,
}

/// A pub/sub event (Watch stream; S18 consumes these).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    /// Event sequence number.
    pub seq: u64,
    /// Subject node.
    pub node_key: String,
    /// "node_up" | "node_down" | "leader".
    pub kind: String,
    /// Free-form detail (e.g. the new leader for "leader").
    pub detail: String,
}

/// A metadata lease row (S18 escape hatch for cross-client writes).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeaseRow {
    /// Current holder.
    pub holder: String,
    /// Deadline (unix seconds).
    pub until: i64,
}

/// Registry mutations — the raft log entry shape.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RegistryOp {
    /// Announce/refresh a node (upsert; emits node_up when new or revived).
    Register { node: NodeRecord },
    /// Liveness heartbeat.
    Heartbeat { node_key: String, used_bytes: u64 },
    /// Acquire the metadata lease of a volume (granted if free or expired).
    AcquireLease { volume: String, holder: String },
    /// Renew a held metadata lease.
    RenewLease { volume: String, holder: String },
    /// Release a held metadata lease.
    ReleaseLease { volume: String, holder: String },
}

/// Result of an applied [`RegistryOp`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RegistryResult {
    /// No payload.
    None,
    /// Granted liveness lease deadline.
    LeaseUntil(i64),
    /// Metadata lease outcome (granted, deadline, current holder).
    Lease {
        /// Granted?
        granted: bool,
        /// Deadline (0 when not granted).
        until: i64,
        /// Current holder ("" when granted to the caller).
        holder: String,
    },
}

/// Registry errors.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error, Serialize, Deserialize)]
pub enum RegistryError {
    /// Node key unknown (heartbeat before register).
    #[error("node not found: {0}")]
    NotFound(String),
    /// redb storage failure.
    #[error("storage: {0}")]
    Storage(String),
    /// bincode failure.
    #[error("codec: {0}")]
    Codec(String),
}
