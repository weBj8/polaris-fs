//! Client-facing Registry service (`plfs.registry.v1.Registry`): writes
//! (Register/Heartbeat/Lease ops) commit through the raft group; reads
//! (ListNodes/Stats/Watch) are leader-local. Followers never answer —
//! they reply with the current leader's address (leader_hint), same
//! linearizable discipline as the metadata groups (design doc §7.3).

use std::collections::BTreeMap;

use plfs_common::registry::v1 as pb;
use plfs_common::registry::v1::registry_server::{Registry, RegistryServer};

use crate::raft::{RegistryRaft, RegistryRaftConfig};
use crate::state::RegistryState;
use crate::{Event, NodeKind, NodeRecord, RegistryError, RegistryOp, RegistryResult};

fn status(err: RegistryError) -> tonic::Status {
    match err {
        RegistryError::NotFound(_) => tonic::Status::not_found(err.to_string()),
        _ => tonic::Status::internal(err.to_string()),
    }
}

fn kind_to_proto(kind: NodeKind) -> pb::NodeKind {
    match kind {
        NodeKind::Data => pb::NodeKind::Data,
        NodeKind::Meta => pb::NodeKind::Meta,
        NodeKind::Client => pb::NodeKind::Client,
        NodeKind::Registry => pb::NodeKind::Registry,
    }
}

fn kind_from_proto(kind: pb::NodeKind) -> Option<NodeKind> {
    match kind {
        pb::NodeKind::Unspecified => None,
        pb::NodeKind::Data => Some(NodeKind::Data),
        pb::NodeKind::Meta => Some(NodeKind::Meta),
        pb::NodeKind::Client => Some(NodeKind::Client),
        pb::NodeKind::Registry => Some(NodeKind::Registry),
    }
}

fn record_to_proto(rec: &NodeRecord) -> pb::NodeInfo {
    pb::NodeInfo {
        node_key: rec.key.clone(),
        kind: kind_to_proto(rec.kind).into(),
        addr: rec.addr.clone(),
        volume: rec.volume.clone(),
        capacity_bytes: rec.capacity_bytes,
        used_bytes: rec.used_bytes,
        last_seen: rec.last_seen,
        lease_until: rec.lease_until,
    }
}

fn record_from_proto(info: &pb::NodeInfo) -> Result<NodeRecord, RegistryError> {
    let kind = pb::NodeKind::try_from(info.kind)
        .ok()
        .and_then(kind_from_proto)
        .ok_or_else(|| RegistryError::Codec("unknown node kind".into()))?;
    Ok(NodeRecord {
        key: info.node_key.clone(),
        kind,
        addr: info.addr.clone(),
        volume: info.volume.clone(),
        capacity_bytes: info.capacity_bytes,
        used_bytes: info.used_bytes,
        last_seen: 0,
        lease_until: 0,
    })
}

/// The Registry service for one raft-group member.
#[derive(Clone)]
pub struct RegistrySvc {
    group: std::sync::Arc<RegistryRaft>,
    state: RegistryState,
    node_id: u64,
    members: std::sync::Arc<BTreeMap<u64, String>>,
}

impl RegistrySvc {
    /// Service over this node's raft group and state machine.
    pub fn new(
        group: RegistryRaft,
        state: RegistryState,
        node_id: u64,
        members: BTreeMap<u64, String>,
    ) -> Self {
        Self {
            group: std::sync::Arc::new(group),
            state,
            node_id,
            members: std::sync::Arc::new(members),
        }
    }

    /// The tonic server wrapper.
    pub fn into_server(self) -> RegistryServer<Self> {
        RegistryServer::new(self)
    }

    fn raft(&self) -> &openraft::Raft<RegistryRaftConfig> {
        self.group.raft()
    }

    fn leader_hint(&self) -> String {
        let metrics = self.raft().metrics().borrow().clone();
        match metrics.current_leader {
            Some(id) => self
                .members
                .get(&id)
                .map(|addr| format!("{id}@{addr}"))
                .unwrap_or_default(),
            None => String::new(),
        }
    }

    fn is_leader(&self) -> bool {
        self.raft().metrics().borrow().clone().current_leader == Some(self.node_id)
    }

    async fn apply(&self, op: RegistryOp) -> Result<RegistryResult, tonic::Status> {
        if !self.is_leader() {
            return Err(tonic::Status::unavailable(format!(
                "not the leader; hint={}",
                self.leader_hint()
            )));
        }
        self.group.write(op).await.map_err(status)
    }

    fn require_leader(&self) -> Result<(), tonic::Status> {
        if self.is_leader() {
            Ok(())
        } else {
            Err(tonic::Status::unavailable(format!(
                "not the leader; hint={}",
                self.leader_hint()
            )))
        }
    }
}

#[tonic::async_trait]
impl Registry for RegistrySvc {
    async fn register(
        &self,
        request: tonic::Request<pb::RegisterRequest>,
    ) -> Result<tonic::Response<pb::RegisterReply>, tonic::Status> {
        let info = request
            .into_inner()
            .node
            .ok_or_else(|| tonic::Status::invalid_argument("missing node info"))?;
        let rec = record_from_proto(&info).map_err(status)?;
        match self.apply(RegistryOp::Register { node: rec }).await? {
            RegistryResult::LeaseUntil(until) => Ok(tonic::Response::new(pb::RegisterReply {
                lease_until: until,
            })),
            other => Err(tonic::Status::internal(format!(
                "unexpected result: {other:?}"
            ))),
        }
    }

    async fn heartbeat(
        &self,
        request: tonic::Request<pb::HeartbeatRequest>,
    ) -> Result<tonic::Response<pb::HeartbeatReply>, tonic::Status> {
        let req = request.into_inner();
        match self
            .apply(RegistryOp::Heartbeat {
                node_key: req.node_key,
                used_bytes: req.used_bytes,
            })
            .await?
        {
            RegistryResult::LeaseUntil(until) => Ok(tonic::Response::new(pb::HeartbeatReply {
                lease_until: until,
            })),
            other => Err(tonic::Status::internal(format!(
                "unexpected result: {other:?}"
            ))),
        }
    }

    async fn list_nodes(
        &self,
        request: tonic::Request<pb::ListNodesRequest>,
    ) -> Result<tonic::Response<pb::ListNodesReply>, tonic::Status> {
        self.require_leader()?;
        let kind = pb::NodeKind::try_from(request.into_inner().kind)
            .ok()
            .and_then(kind_from_proto);
        let nodes = self.state.list_nodes(kind).map_err(status)?;
        Ok(tonic::Response::new(pb::ListNodesReply {
            nodes: nodes.iter().map(record_to_proto).collect(),
        }))
    }

    async fn stats(
        &self,
        _request: tonic::Request<pb::StatsRequest>,
    ) -> Result<tonic::Response<pb::StatsReply>, tonic::Status> {
        self.require_leader()?;
        let (capacity, used, live) = self.state.stats().map_err(status)?;
        Ok(tonic::Response::new(pb::StatsReply {
            capacity_bytes: capacity,
            used_bytes: used,
            live_data_nodes: live,
        }))
    }

    type WatchStream = std::pin::Pin<
        Box<dyn futures_core::Stream<Item = Result<pb::WatchEvent, tonic::Status>> + Send>,
    >;

    async fn watch(
        &self,
        request: tonic::Request<pb::WatchRequest>,
    ) -> Result<tonic::Response<Self::WatchStream>, tonic::Status> {
        self.require_leader()?;
        let since = request.into_inner().since_seq;
        let state = self.state.clone();
        let (tx, rx) = tokio::sync::mpsc::channel(256);
        tokio::spawn(async move {
            let mut cursor = since;
            loop {
                match state.events_since(cursor.max(1)) {
                    Ok(events) => {
                        let mut advanced = false;
                        for event in events {
                            if event.seq >= cursor.max(1) {
                                advanced = true;
                                cursor = cursor.max(1).max(event.seq + 1);
                                if tx.send(Ok(to_proto_event(&event))).await.is_err() {
                                    return;
                                }
                            }
                        }
                        let _ = advanced;
                    }
                    Err(_) => return,
                }
                tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            }
        });
        Ok(tonic::Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx),
        )))
    }

    async fn acquire_lease(
        &self,
        request: tonic::Request<pb::AcquireLeaseRequest>,
    ) -> Result<tonic::Response<pb::AcquireLeaseReply>, tonic::Status> {
        let req = request.into_inner();
        match self
            .apply(RegistryOp::AcquireLease {
                volume: req.volume,
                holder: req.holder,
            })
            .await?
        {
            RegistryResult::Lease {
                granted,
                until,
                holder,
            } => Ok(tonic::Response::new(pb::AcquireLeaseReply {
                granted,
                lease_until: until,
                holder,
            })),
            other => Err(tonic::Status::internal(format!(
                "unexpected result: {other:?}"
            ))),
        }
    }

    async fn renew_lease(
        &self,
        request: tonic::Request<pb::RenewLeaseRequest>,
    ) -> Result<tonic::Response<pb::RenewLeaseReply>, tonic::Status> {
        let req = request.into_inner();
        match self
            .apply(RegistryOp::RenewLease {
                volume: req.volume,
                holder: req.holder,
            })
            .await?
        {
            RegistryResult::Lease { granted, until, .. } => {
                Ok(tonic::Response::new(pb::RenewLeaseReply {
                    granted,
                    lease_until: until,
                }))
            }
            other => Err(tonic::Status::internal(format!(
                "unexpected result: {other:?}"
            ))),
        }
    }

    async fn release_lease(
        &self,
        request: tonic::Request<pb::ReleaseLeaseRequest>,
    ) -> Result<tonic::Response<pb::ReleaseLeaseReply>, tonic::Status> {
        let req = request.into_inner();
        self.apply(RegistryOp::ReleaseLease {
            volume: req.volume,
            holder: req.holder,
        })
        .await?;
        Ok(tonic::Response::new(pb::ReleaseLeaseReply {}))
    }
}

fn to_proto_event(event: &Event) -> pb::WatchEvent {
    pb::WatchEvent {
        seq: event.seq,
        node_key: event.node_key.clone(),
        kind: event.kind.clone(),
        detail: event.detail.clone(),
    }
}
