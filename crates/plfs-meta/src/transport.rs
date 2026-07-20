//! Raft peer transport (design doc §7.3): openraft AppendEntries / Vote /
//! InstallSnapshot over gRPC (`plfs.meta.v1.MetaRaft`), payloads bincode.
//! Transport failures map to `NetworkError`; remote raft errors round-trip
//! as `RemoteError` (so ForwardToLeader and vote conflicts survive the hop).

use std::sync::Arc;

use openraft::error::{InstallSnapshotError, NetworkError, RPCError, RaftError, RemoteError};
use openraft::raft::{
    AppendEntriesRequest, AppendEntriesResponse, InstallSnapshotRequest, InstallSnapshotResponse,
    VoteRequest, VoteResponse,
};
use openraft::{BasicNode, Raft, RaftNetwork, RaftNetworkFactory};
use plfs_common::meta::v1 as pb;
use plfs_common::meta::v1::meta_raft_client::MetaRaftClient;
use plfs_common::meta::v1::meta_raft_server::{MetaRaft, MetaRaftServer};
use tokio::sync::Mutex;
use tonic::transport::Channel;

use crate::MetaError;
use crate::raft::MetaRaftConfig;

fn enc<T: serde::Serialize>(v: &T) -> Result<Vec<u8>, MetaError> {
    bincode::serialize(v).map_err(|e| MetaError::Codec(e.to_string()))
}

fn dec<T: serde::de::DeserializeOwned>(b: &[u8]) -> Result<T, MetaError> {
    bincode::deserialize(b).map_err(|e| MetaError::Codec(e.to_string()))
}

type AppendRpcError = RPCError<u64, BasicNode, RaftError<u64>>;
type VoteRpcError = RPCError<u64, BasicNode, RaftError<u64>>;
type SnapshotRpcError = RPCError<u64, BasicNode, RaftError<u64, InstallSnapshotError>>;

/// Server side: decode a peer RPC, drive it through the local raft core,
/// and encode the outcome (including remote raft errors) back.
#[derive(Clone)]
pub struct RaftTransportSvc {
    raft: Raft<MetaRaftConfig>,
}

impl RaftTransportSvc {
    /// Transport service for this raft node.
    pub fn new(raft: Raft<MetaRaftConfig>) -> Self {
        Self { raft }
    }

    /// The tonic server wrapper.
    pub fn into_server(self) -> MetaRaftServer<Self> {
        MetaRaftServer::new(self)
    }
}

#[tonic::async_trait]
impl MetaRaft for RaftTransportSvc {
    async fn append_entries(
        &self,
        request: tonic::Request<pb::RpcEnvelope>,
    ) -> Result<tonic::Response<pb::RpcEnvelope>, tonic::Status> {
        let rpc: AppendEntriesRequest<MetaRaftConfig> = dec(&request.into_inner().payload)
            .map_err(|e| tonic::Status::invalid_argument(e.to_string()))?;
        let result: Result<AppendEntriesResponse<u64>, RaftError<u64>> =
            self.raft.append_entries(rpc).await;
        let payload = enc(&result).map_err(|e| tonic::Status::internal(e.to_string()))?;
        Ok(tonic::Response::new(pb::RpcEnvelope { payload }))
    }

    async fn vote(
        &self,
        request: tonic::Request<pb::RpcEnvelope>,
    ) -> Result<tonic::Response<pb::RpcEnvelope>, tonic::Status> {
        let rpc: VoteRequest<u64> = dec(&request.into_inner().payload)
            .map_err(|e| tonic::Status::invalid_argument(e.to_string()))?;
        let result: Result<VoteResponse<u64>, RaftError<u64>> = self.raft.vote(rpc).await;
        let payload = enc(&result).map_err(|e| tonic::Status::internal(e.to_string()))?;
        Ok(tonic::Response::new(pb::RpcEnvelope { payload }))
    }

    async fn install_snapshot(
        &self,
        request: tonic::Request<pb::RpcEnvelope>,
    ) -> Result<tonic::Response<pb::RpcEnvelope>, tonic::Status> {
        let rpc: InstallSnapshotRequest<MetaRaftConfig> = dec(&request.into_inner().payload)
            .map_err(|e| tonic::Status::invalid_argument(e.to_string()))?;
        let result: Result<InstallSnapshotResponse<u64>, RaftError<u64, InstallSnapshotError>> =
            self.raft.install_snapshot(rpc).await;
        let payload = enc(&result).map_err(|e| tonic::Status::internal(e.to_string()))?;
        Ok(tonic::Response::new(pb::RpcEnvelope { payload }))
    }
}

/// Client side: a tonic-backed `RaftNetworkFactory`. Connections are opened
/// lazily per peer and dropped on failure (reconnect next call).
#[derive(Clone, Default)]
pub struct GrpcNetworkFactory;

impl RaftNetworkFactory<MetaRaftConfig> for GrpcNetworkFactory {
    type Network = GrpcNetworkConnection;

    async fn new_client(&mut self, target: u64, node: &BasicNode) -> Self::Network {
        GrpcNetworkConnection {
            target,
            node: node.clone(),
            client: Arc::new(Mutex::new(None)),
        }
    }
}

/// One peer connection (lazy).
pub struct GrpcNetworkConnection {
    target: u64,
    node: BasicNode,
    client: Arc<Mutex<Option<MetaRaftClient<Channel>>>>,
}

impl GrpcNetworkConnection {
    async fn client(&self) -> Result<MetaRaftClient<Channel>, NetworkError> {
        let mut guard = self.client.lock().await;
        if let Some(c) = guard.as_ref() {
            return Ok(c.clone());
        }
        let endpoint = format!("http://{}", self.node.addr);
        let client = MetaRaftClient::connect(endpoint)
            .await
            .map_err(|e| NetworkError::new(&e))?;
        *guard = Some(client.clone());
        Ok(client)
    }

    async fn drop_client(&self) {
        *self.client.lock().await = None;
    }

    fn remote<E: std::error::Error>(&self, e: E) -> RemoteError<u64, BasicNode, E> {
        RemoteError::new_with_node(self.target, self.node.clone(), e)
    }
}

impl RaftNetwork<MetaRaftConfig> for GrpcNetworkConnection {
    async fn append_entries(
        &mut self,
        rpc: AppendEntriesRequest<MetaRaftConfig>,
        _option: openraft::network::RPCOption,
    ) -> Result<AppendEntriesResponse<u64>, AppendRpcError> {
        let payload = enc(&rpc).map_err(|e| NetworkError::new(&e))?;
        let mut client = self.client().await?;
        let result = client.append_entries(pb::RpcEnvelope { payload }).await;
        match result {
            Ok(resp) => {
                let decoded: Result<AppendEntriesResponse<u64>, RaftError<u64>> =
                    dec(&resp.into_inner().payload).map_err(|e| NetworkError::new(&e))?;
                decoded.map_err(|e| self.remote(e).into())
            }
            Err(e) => {
                self.drop_client().await;
                Err(NetworkError::new(&e).into())
            }
        }
    }

    async fn vote(
        &mut self,
        rpc: VoteRequest<u64>,
        _option: openraft::network::RPCOption,
    ) -> Result<VoteResponse<u64>, VoteRpcError> {
        let payload = enc(&rpc).map_err(|e| NetworkError::new(&e))?;
        let mut client = self.client().await?;
        let result = client.vote(pb::RpcEnvelope { payload }).await;
        match result {
            Ok(resp) => {
                let decoded: Result<VoteResponse<u64>, RaftError<u64>> =
                    dec(&resp.into_inner().payload).map_err(|e| NetworkError::new(&e))?;
                decoded.map_err(|e| self.remote(e).into())
            }
            Err(e) => {
                self.drop_client().await;
                Err(NetworkError::new(&e).into())
            }
        }
    }

    async fn install_snapshot(
        &mut self,
        rpc: InstallSnapshotRequest<MetaRaftConfig>,
        _option: openraft::network::RPCOption,
    ) -> Result<InstallSnapshotResponse<u64>, SnapshotRpcError> {
        let payload = enc(&rpc).map_err(|e| NetworkError::new(&e))?;
        let mut client = self.client().await?;
        let result = client.install_snapshot(pb::RpcEnvelope { payload }).await;
        match result {
            Ok(resp) => {
                let decoded: Result<
                    InstallSnapshotResponse<u64>,
                    RaftError<u64, InstallSnapshotError>,
                > = dec(&resp.into_inner().payload).map_err(|e| NetworkError::new(&e))?;
                decoded.map_err(|e| self.remote(e).into())
            }
            Err(e) => {
                self.drop_client().await;
                Err(NetworkError::new(&e).into())
            }
        }
    }
}
