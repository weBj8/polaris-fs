//! Foreign-volume read view (design doc §4.3, §8.2): read-only cross-client
//! access — metadata reads go to the volume owner's MetaOps endpoint
//! (discovered through the registry's CLIENT-kind node directory), data
//! reads hit the shared cluster data plane. Eventually consistent by
//! construction: a read sees whatever the owner had committed when the RPC
//! lands (well under 1 s in practice).

use plfs_common::meta::v1::RpcEnvelope;
use plfs_common::meta::v1::meta_ops_client::MetaOpsClient;
use plfs_common::registry::v1 as rpb;
use plfs_meta::service::{MetaRead, MetaReadReply};
use plfs_meta::{ChunkRef, Inode};
use plfs_registry::client::RegistryClient;

use crate::cluster::ClusterSink;
use crate::core::{CHUNK_SIZE, ClientError};

fn grpc_err(e: impl std::fmt::Display) -> ClientError {
    ClientError::Codec(format!("grpc: {e}"))
}

/// A read-only view of another client's volume (§8.2).
pub struct ForeignClient {
    meta_addr: String,
    sink: ClusterSink,
}

impl ForeignClient {
    /// Discover the volume's live owner through the registry and prepare
    /// the data plane.
    pub async fn connect(registry: &str, volume: &str, rf: usize) -> Result<Self, ClientError> {
        let reg = RegistryClient::new(vec![registry.to_string()]);
        let nodes = reg
            .list_nodes(rpb::NodeKind::Client)
            .await
            .map_err(grpc_err)?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs() as i64);
        let addr = nodes
            .iter()
            .find(|n| n.volume == volume && n.lease_until > now)
            .map(|n| n.addr.clone())
            .ok_or_else(|| ClientError::Codec(format!("no live owner for volume {volume}")))?;
        let mut sink = ClusterSink::new(registry, rf);
        sink.refresh_live().await?;
        Ok(Self {
            meta_addr: addr,
            sink,
        })
    }

    async fn meta_read(&self, q: &MetaRead) -> Result<MetaReadReply, ClientError> {
        let mut client = MetaOpsClient::connect(format!("http://{}", self.meta_addr))
            .await
            .map_err(grpc_err)?;
        let payload = bincode::serialize(q).map_err(|e| ClientError::Codec(e.to_string()))?;
        let reply = client
            .read(RpcEnvelope { payload })
            .await
            .map_err(grpc_err)?
            .into_inner();
        if reply.payload.is_empty() {
            return Err(ClientError::Codec(format!(
                "meta not leader (hint: {})",
                reply.leader_hint
            )));
        }
        let result: Result<MetaReadReply, plfs_meta::MetaError> =
            bincode::deserialize(&reply.payload).map_err(|e| ClientError::Codec(e.to_string()))?;
        result.map_err(ClientError::Meta)
    }

    /// Resolve a directory entry.
    pub async fn lookup(&self, parent: u64, name: &str) -> Result<Option<u64>, ClientError> {
        match self
            .meta_read(&MetaRead::Lookup(parent, name.to_string()))
            .await?
        {
            MetaReadReply::Lookup(v) => Ok(v),
            other => Err(ClientError::Codec(format!("bad reply: {other:?}"))),
        }
    }

    /// Inode attributes.
    pub async fn getattr(&self, ino: u64) -> Result<Option<Inode>, ClientError> {
        match self.meta_read(&MetaRead::GetAttr(ino)).await? {
            MetaReadReply::GetAttr(v) => Ok(v),
            other => Err(ClientError::Codec(format!("bad reply: {other:?}"))),
        }
    }

    /// List a directory (name, child ino).
    pub async fn listdir(&self, ino: u64) -> Result<Vec<(String, u64)>, ClientError> {
        match self.meta_read(&MetaRead::Listdir(ino)).await? {
            MetaReadReply::Listdir(v) => Ok(v),
            other => Err(ClientError::Codec(format!("bad reply: {other:?}"))),
        }
    }

    /// The file's layout from an index onward.
    pub async fn layout_from(
        &self,
        ino: u64,
        idx: u64,
    ) -> Result<Vec<(u64, ChunkRef)>, ClientError> {
        match self.meta_read(&MetaRead::LayoutFrom(ino, idx)).await? {
            MetaReadReply::Layout(v) => Ok(v),
            other => Err(ClientError::Codec(format!("bad reply: {other:?}"))),
        }
    }

    /// Read file bytes (chunks come from the shared cluster data plane).
    pub async fn read(&mut self, ino: u64, offset: u64, len: u64) -> Result<Vec<u8>, ClientError> {
        let size = self.getattr(ino).await?.ok_or(ClientError::NotFound)?.size;
        if offset >= size {
            return Ok(Vec::new());
        }
        let end = (offset + len).min(size);
        let layout = self.layout_from(ino, offset / CHUNK_SIZE).await?;
        let mut out = vec![0u8; (end - offset) as usize];
        let mut base = (offset / CHUNK_SIZE) * CHUNK_SIZE;
        for (_, chunk) in layout {
            let cstart = base;
            let cend = base + chunk.len;
            base = cend;
            if cend <= offset || cstart >= end {
                continue;
            }
            let (_, data) = self.sink.get(chunk.chunk_id, &chunk.replicas).await?;
            let from = offset.max(cstart) - cstart;
            let to = end.min(cend) - cstart;
            let dst = (cstart.max(offset) - offset) as usize;
            out[dst..dst + (to - from) as usize].copy_from_slice(&data[from as usize..to as usize]);
        }
        Ok(out)
    }
}
