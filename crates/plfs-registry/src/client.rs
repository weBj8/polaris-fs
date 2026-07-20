//! A small leader-tracking client for the Registry (used by data nodes for
//! Register/Heartbeat and by anything needing the control plane). Follows
//! "not the leader; hint=..." responses and rotates endpoints on transport
//! failure.

use std::sync::Mutex;

use plfs_common::registry::v1 as pb;
use plfs_common::registry::v1::registry_client::RegistryClient as RpcClient;

use crate::RegistryError;

/// The Registry client.
pub struct RegistryClient {
    endpoints: Vec<String>,
    leader: Mutex<Option<String>>,
}

impl RegistryClient {
    /// Client over the given registry endpoints (host:port list).
    pub fn new(endpoints: Vec<String>) -> Self {
        Self {
            endpoints,
            leader: Mutex::new(None),
        }
    }

    fn pick(&self, attempt: usize) -> String {
        // The hinted leader is the best target whenever we know it.
        if let Some(l) = self.leader.lock().expect("leader lock").clone() {
            return l;
        }
        self.endpoints[attempt % self.endpoints.len()].clone()
    }

    fn clear_hint_if(&self, addr: &str) {
        let mut g = self.leader.lock().expect("leader lock");
        if g.as_deref() == Some(addr) {
            *g = None;
        }
    }

    fn note_hint(&self, status: &tonic::Status) {
        let msg = status.message();
        if let Some(pos) = msg.find("hint=") {
            let hint = &msg[pos + 5..];
            if let Some((_, addr)) = hint.split_once('@') {
                *self.leader.lock().expect("leader lock") = Some(addr.to_string());
            }
        }
    }

    async fn call<F, Fut, T>(&self, f: F) -> Result<T, RegistryError>
    where
        F: Fn(RpcClient<tonic::transport::Channel>) -> Fut,
        Fut: std::future::Future<Output = Result<tonic::Response<T>, tonic::Status>>,
    {
        let mut last_err = RegistryError::Storage("no endpoints".into());
        for attempt in 0..self.endpoints.len() * 2 {
            let addr = self.pick(attempt);
            let Ok(client) = RpcClient::connect(format!("http://{addr}")).await else {
                self.clear_hint_if(&addr);
                last_err = RegistryError::Storage(format!("connect {addr} failed"));
                continue;
            };
            match f(client).await {
                Ok(resp) => return Ok(resp.into_inner()),
                Err(status) => {
                    self.note_hint(&status);
                    last_err = RegistryError::Storage(status.to_string());
                }
            }
        }
        Err(last_err)
    }

    /// Register this node (idempotent re-register).
    pub async fn register(&self, node: pb::NodeInfo) -> Result<i64, RegistryError> {
        let reply = self
            .call(|mut c| {
                let node = node.clone();
                async move { c.register(pb::RegisterRequest { node: Some(node) }).await }
            })
            .await?;
        Ok(reply.lease_until)
    }

    /// Liveness heartbeat; returns the granted deadline.
    pub async fn heartbeat(&self, node_key: &str, used_bytes: u64) -> Result<i64, RegistryError> {
        let node_key = node_key.to_string();
        let reply = self
            .call(move |mut c| {
                let node_key = node_key.clone();
                async move {
                    c.heartbeat(pb::HeartbeatRequest {
                        node_key,
                        used_bytes,
                    })
                    .await
                }
            })
            .await?;
        Ok(reply.lease_until)
    }

    /// List registered nodes (any kind).
    pub async fn list_nodes(&self, kind: pb::NodeKind) -> Result<Vec<pb::NodeInfo>, RegistryError> {
        let reply = self
            .call(move |mut c| async move {
                c.list_nodes(pb::ListNodesRequest { kind: kind.into() })
                    .await
            })
            .await?;
        Ok(reply.nodes)
    }

    /// Aggregated cluster stats.
    pub async fn stats(&self) -> Result<pb::StatsReply, RegistryError> {
        self.call(move |mut c| async move { c.stats(pb::StatsRequest {}).await })
            .await
    }

    /// Acquire the metadata lease of a volume.
    pub async fn acquire_lease(
        &self,
        volume: &str,
        holder: &str,
    ) -> Result<pb::AcquireLeaseReply, RegistryError> {
        let volume = volume.to_string();
        let holder = holder.to_string();
        self.call(move |mut c| {
            let volume = volume.clone();
            let holder = holder.clone();
            async move {
                c.acquire_lease(pb::AcquireLeaseRequest { volume, holder })
                    .await
            }
        })
        .await
    }

    /// Renew a held metadata lease.
    pub async fn renew_lease(
        &self,
        volume: &str,
        holder: &str,
    ) -> Result<pb::RenewLeaseReply, RegistryError> {
        let volume = volume.to_string();
        let holder = holder.to_string();
        self.call(move |mut c| {
            let volume = volume.clone();
            let holder = holder.clone();
            async move {
                c.renew_lease(pb::RenewLeaseRequest { volume, holder })
                    .await
            }
        })
        .await
    }
}
