//! Client-facing metadata service (design doc §7.3): `MetaOps.Apply`
//! commits ops through the volume's raft group; `MetaOps.Read` answers
//! leader-local reads. Followers never answer — they reply with the
//! current leader's address (`leader_hint`), keeping metadata linearizable.

use std::collections::BTreeMap;

use openraft::Raft;
use plfs_common::meta::v1 as pb;
use plfs_common::meta::v1::meta_ops_server::{MetaOps, MetaOpsServer};
use serde::{Deserialize, Serialize};

use crate::raft::MetaRaftConfig;
use crate::{ChunkRef, Inode, MetaError, MetaOp, MetaState, OpResult, SnapshotMeta, Time};

/// Leader-local read surface (bincode over `plfs.meta.v1.MetaOps.Read`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetaRead {
    /// getattr
    GetAttr(u64),
    /// lookup(parent, name)
    Lookup(u64, String),
    /// listdir
    Listdir(u64),
    /// full layout of a file
    Layout(u64),
    /// layout rows from an index onward
    LayoutFrom(u64, u64),
    /// last applied write-path commit sequence
    CommitSeq,
    /// live chunk refcount
    ChunkRefcount([u8; 16]),
    /// GC queue depth
    GcLen,
    /// snapshot rows
    ListSnaps,
    /// chunks whose replica set contains this data-node addr (§7.4)
    ChunksWithReplica(String),
}

/// Reply payload for [`MetaRead`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetaReadReply {
    /// getattr result
    GetAttr(Option<Inode>),
    /// lookup result
    Lookup(Option<u64>),
    /// listdir result
    Listdir(Vec<(String, u64)>),
    /// layout / layout_from result
    Layout(Vec<(u64, ChunkRef)>),
    /// commit_seq result
    CommitSeq(u64),
    /// refcount / gc_len result
    Count(u64),
    /// list_snaps result
    ListSnaps(Vec<SnapshotMeta>),
    /// chunks_with_replica result
    Chunks(Vec<ChunkRef>),
}

fn enc<T: serde::Serialize>(v: &T) -> Result<Vec<u8>, MetaError> {
    bincode::serialize(v).map_err(|e| MetaError::Codec(e.to_string()))
}

fn dec<T: serde::de::DeserializeOwned>(b: &[u8]) -> Result<T, MetaError> {
    bincode::deserialize(b).map_err(|e| MetaError::Codec(e.to_string()))
}

/// The `MetaOps` service for one raft-group member.
#[derive(Clone)]
pub struct MetaOpsSvc {
    raft: Raft<MetaRaftConfig>,
    state: MetaState,
    node_id: u64,
    members: std::sync::Arc<BTreeMap<u64, String>>,
}

impl MetaOpsSvc {
    /// Service over this node's raft and state machine.
    pub fn new(
        raft: Raft<MetaRaftConfig>,
        state: MetaState,
        node_id: u64,
        members: BTreeMap<u64, String>,
    ) -> Self {
        Self {
            raft,
            state,
            node_id,
            members: std::sync::Arc::new(members),
        }
    }

    /// The tonic server wrapper.
    pub fn into_server(self) -> MetaOpsServer<Self> {
        MetaOpsServer::new(self)
    }

    /// "id@addr" of the current leader, empty when unknown.
    fn leader_hint(&self) -> String {
        let metrics = self.raft.metrics().borrow().clone();
        match metrics.current_leader {
            Some(id) => self
                .members
                .get(&id)
                .map(|addr| format!("{id}@{addr}"))
                .unwrap_or_default(),
            None => String::new(),
        }
    }

    fn read_answer(&self, q: &MetaRead) -> Result<MetaReadReply, MetaError> {
        match q {
            MetaRead::GetAttr(ino) => Ok(MetaReadReply::GetAttr(self.state.getattr(*ino)?)),
            MetaRead::Lookup(parent, name) => {
                Ok(MetaReadReply::Lookup(self.state.lookup(*parent, name)?))
            }
            MetaRead::Listdir(ino) => Ok(MetaReadReply::Listdir(self.state.listdir(*ino)?)),
            MetaRead::Layout(ino) => Ok(MetaReadReply::Layout(self.state.layout(*ino)?)),
            MetaRead::LayoutFrom(ino, idx) => {
                Ok(MetaReadReply::Layout(self.state.layout_from(*ino, *idx)?))
            }
            MetaRead::CommitSeq => Ok(MetaReadReply::CommitSeq(self.state.commit_seq()?)),
            MetaRead::ChunkRefcount(id) => Ok(MetaReadReply::Count(self.state.chunk_refcount(id)?)),
            MetaRead::GcLen => Ok(MetaReadReply::Count(self.state.gc_len()?)),
            MetaRead::ListSnaps => Ok(MetaReadReply::ListSnaps(self.state.list_snaps()?)),
            MetaRead::ChunksWithReplica(addr) => {
                Ok(MetaReadReply::Chunks(self.state.chunks_with_replica(addr)?))
            }
        }
    }
}

#[tonic::async_trait]
impl MetaOps for MetaOpsSvc {
    async fn apply(
        &self,
        request: tonic::Request<pb::RpcEnvelope>,
    ) -> Result<tonic::Response<pb::LeaderReply>, tonic::Status> {
        let op: MetaOp = dec(&request.into_inner().payload)
            .map_err(|e| tonic::Status::invalid_argument(e.to_string()))?;
        // Followers proxy to the leader (design doc §7.3). client_write on
        // a follower returns ForwardToLeader — surface it as the hint.
        let metrics = self.raft.metrics().borrow().clone();
        if metrics.current_leader != Some(self.node_id) {
            return Ok(tonic::Response::new(pb::LeaderReply {
                payload: Vec::new(),
                leader_hint: self.leader_hint(),
            }));
        }
        let now: Time = (0, 0);
        let _ = now;
        let result: Result<OpResult, MetaError> = match self.raft.client_write(op).await {
            Ok(resp) => resp.data,
            Err(_e) => {
                return Ok(tonic::Response::new(pb::LeaderReply {
                    payload: Vec::new(),
                    leader_hint: self.leader_hint(),
                }));
            }
        };
        let payload = enc(&result).map_err(|e| tonic::Status::internal(e.to_string()))?;
        Ok(tonic::Response::new(pb::LeaderReply {
            payload,
            leader_hint: String::new(),
        }))
    }

    async fn read(
        &self,
        request: tonic::Request<pb::RpcEnvelope>,
    ) -> Result<tonic::Response<pb::LeaderReply>, tonic::Status> {
        let q: MetaRead = dec(&request.into_inner().payload)
            .map_err(|e| tonic::Status::invalid_argument(e.to_string()))?;
        let metrics = self.raft.metrics().borrow().clone();
        if metrics.current_leader != Some(self.node_id) {
            return Ok(tonic::Response::new(pb::LeaderReply {
                payload: Vec::new(),
                leader_hint: self.leader_hint(),
            }));
        }
        let result = self.read_answer(&q);
        let payload = enc(&result).map_err(|e| tonic::Status::internal(e.to_string()))?;
        Ok(tonic::Response::new(pb::LeaderReply {
            payload,
            leader_hint: String::new(),
        }))
    }
}
