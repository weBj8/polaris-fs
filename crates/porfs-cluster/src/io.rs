//! Striped parallel I/O over [`crate::StripeMap`] and [`porfs_rpc`].

use std::sync::Arc;

use tokio::sync::Semaphore;

use porfs_rpc::{ChunkClient, Request, Response, RpcError};

use crate::{ChunkLoc, Layout, Membership, ReplicaLoc, ReplicatedLayout, STRIPE_UNIT, StripeMap};

/// Errors of the striped I/O layer.
#[derive(Debug, thiserror::Error)]
pub enum ClusterError {
    /// One chunkserver call failed.
    #[error("rpc error: {0}")]
    Rpc(#[from] RpcError),
    /// A spawned chunk task failed to join.
    #[error("task join error: {0}")]
    Join(#[from] tokio::task::JoinError),
    /// The concurrency semaphore was closed mid-operation.
    #[error("concurrency semaphore closed")]
    Acquire,
    /// A response had a different shape than the op requires.
    #[error("protocol error: {0}")]
    Protocol(String),
    /// A chunk read returned fewer/more bytes than its chunk covers.
    #[error("chunk {chunk} length mismatch: got {got}, want {want}")]
    LengthMismatch {
        /// Chunk index.
        chunk: u64,
        /// Bytes returned.
        got: usize,
        /// Bytes expected.
        want: usize,
    },
}

/// One lazily-connected RPC client per chunkserver.
#[derive(Clone)]
pub struct Pool {
    clients: Vec<ChunkClient>,
    addrs: Vec<std::net::SocketAddr>,
}

impl Pool {
    /// A pool over the membership (connections open lazily on first use).
    pub fn new(members: &Membership) -> Self {
        Self {
            clients: members
                .addrs()
                .iter()
                .map(|addr| ChunkClient::new(*addr))
                .collect(),
            addrs: members.addrs().to_vec(),
        }
    }

    /// Client for server `index`.
    pub fn client(&self, index: usize) -> &ChunkClient {
        &self.clients[index]
    }

    /// Address of server `index`, used to pass the secondary to a primary.
    pub fn client_addr(&self, index: usize) -> std::net::SocketAddr {
        self.addrs[index]
    }

    /// Number of servers in the pool.
    pub fn len(&self) -> usize {
        self.clients.len()
    }

    /// Always false: a pool mirrors the (non-empty) membership.
    pub fn is_empty(&self) -> bool {
        self.clients.is_empty()
    }
}

/// Deterministic write id of one chunk (retry-safe replay of the SAME
/// content; overwrites must vary it — a generation lands with P9).
fn write_id(inode: u64, chunk_index: u64, generation: u64) -> u128 {
    use std::hash::Hasher;
    let mut hasher = twox_hash::XxHash64::with_seed(0x7091_5A1D_5EED_0002);
    hasher.write_u64(inode);
    hasher.write_u64(chunk_index);
    hasher.write_u64(generation);
    let high = hasher.finish() as u128;
    (high << 64) | generation as u128
}

/// Write `data` as 1 MiB stripe chunks, each on its rendezvous server,
/// with up to `concurrency` chunk writes in flight. Returns the layout
/// (chunk → server+extent id) the metadata layer owns from P9 on.
pub async fn stripe_write(
    pool: &Pool,
    map: &StripeMap,
    inode: u64,
    data: Vec<u8>,
    concurrency: usize,
) -> Result<Layout, ClusterError> {
    let len = data.len() as u64;
    let chunk_count = StripeMap::chunk_count(len);
    let semaphore = Arc::new(Semaphore::new(concurrency.max(1)));
    let mut tasks = Vec::with_capacity(chunk_count as usize);
    for idx in 0..chunk_count {
        let permit = semaphore
            .clone()
            .acquire_owned()
            .await
            .map_err(|_| ClusterError::Acquire)?;
        let start = idx * STRIPE_UNIT;
        let end = (start + STRIPE_UNIT).min(len) as usize;
        let chunk = data[start as usize..end].to_vec();
        let server = map.place(inode, idx);
        let client = pool.client(server).clone();
        let write_id = write_id(inode, idx, 0);
        tasks.push(tokio::spawn(async move {
            let _permit = permit;
            let extent_id = client.write_extent(write_id, inode, start, chunk).await?;
            Ok::<_, RpcError>(ChunkLoc { server, extent_id })
        }));
    }

    let mut chunks = Vec::with_capacity(tasks.len());
    for task in tasks {
        chunks.push(task.await??);
    }
    Ok(Layout { inode, len, chunks })
}

/// Write a generation through primary-to-secondary chains and confirm both
/// copies before returning their layout.
pub async fn stripe_write_replicated(
    pool: &Pool,
    map: &StripeMap,
    inode: u64,
    generation: u64,
    data: Vec<u8>,
    concurrency: usize,
) -> Result<ReplicatedLayout, ClusterError> {
    let len = data.len() as u64;
    let semaphore = Arc::new(Semaphore::new(concurrency.max(1)));
    let mut tasks = Vec::new();
    for idx in 0..StripeMap::chunk_count(len) {
        let permit = semaphore
            .clone()
            .acquire_owned()
            .await
            .map_err(|_| ClusterError::Acquire)?;
        let start = idx * STRIPE_UNIT;
        let chunk = data[start as usize..((start + STRIPE_UNIT).min(len)) as usize].to_vec();
        let (primary, secondary) = map.place_replicas(inode, idx);
        let client = pool.client(primary).clone();
        let secondary_addr = pool.client_addr(secondary);
        tasks.push(tokio::spawn(async move {
            let _permit = permit;
            let (primary_id, secondary_id) = client
                .write_extent_replicated(
                    write_id(inode, idx, generation),
                    inode,
                    start,
                    chunk,
                    Some(secondary_addr),
                )
                .await?;
            let secondary_id = secondary_id.ok_or_else(|| {
                RpcError::Protocol("missing secondary write acknowledgement".to_string())
            })?;
            if client
                .sync_replicated(Some(secondary_addr))
                .await?
                .1
                .is_none()
            {
                return Err(RpcError::Protocol(
                    "missing secondary durability acknowledgement".to_string(),
                ));
            }
            Ok::<_, RpcError>(ReplicaLoc {
                primary: ChunkLoc {
                    server: primary,
                    extent_id: primary_id,
                },
                secondary: ChunkLoc {
                    server: secondary,
                    extent_id: secondary_id,
                },
            })
        }));
    }
    let mut chunks = Vec::with_capacity(tasks.len());
    for task in tasks {
        chunks.push(task.await??);
    }
    Ok(ReplicatedLayout {
        inode,
        generation,
        len,
        chunks,
    })
}

/// Reassemble a file, falling back to its secondary copy on a primary error.
pub async fn stripe_read_replicated(
    pool: &Pool,
    layout: &ReplicatedLayout,
    concurrency: usize,
) -> Result<Vec<u8>, ClusterError> {
    let semaphore = Arc::new(Semaphore::new(concurrency.max(1)));
    let mut tasks = Vec::new();
    for (idx, loc) in layout.chunks.iter().copied().enumerate() {
        let permit = semaphore
            .clone()
            .acquire_owned()
            .await
            .map_err(|_| ClusterError::Acquire)?;
        let primary = pool.client(loc.primary.server).clone();
        let secondary = pool.client(loc.secondary.server).clone();
        let want = STRIPE_UNIT.min(layout.len - idx as u64 * STRIPE_UNIT) as usize;
        tasks.push(tokio::spawn(async move {
            let _permit = permit;
            let data = match tokio::time::timeout(
                std::time::Duration::from_secs(2),
                primary.read_extent(loc.primary.extent_id),
            )
            .await
            {
                Ok(Ok(data)) if data.len() == want => data,
                _ => secondary.read_extent(loc.secondary.extent_id).await?,
            };
            if data.len() != want {
                return Err(ClusterError::LengthMismatch {
                    chunk: idx as u64,
                    got: data.len(),
                    want,
                });
            }
            Ok::<_, ClusterError>((idx, data))
        }));
    }
    let mut out = vec![0; layout.len as usize];
    for task in tasks {
        let (idx, data) = task.await??;
        let start = idx * STRIPE_UNIT as usize;
        out[start..start + data.len()].copy_from_slice(&data);
    }
    Ok(out)
}

/// Read a whole striped file back, reassembled in order. Parallelism is
/// per-server: one task per chunkserver pipelines its chunk reads in
/// windows of `pipeline` requests (protocol §4 in-order responses), so
/// every connection streams instead of ping-ponging.
pub async fn stripe_read(
    pool: &Pool,
    layout: &Layout,
    pipeline: usize,
) -> Result<Vec<u8>, ClusterError> {
    let pipeline = pipeline.max(1);
    let mut by_server: Vec<Vec<(u64, u64)>> = vec![Vec::new(); pool.len()];
    for (idx, loc) in layout.chunks.iter().enumerate() {
        by_server[loc.server].push((idx as u64, loc.extent_id));
    }
    let mut tasks = Vec::new();
    for (server, chunks) in by_server.into_iter().enumerate() {
        if chunks.is_empty() {
            continue;
        }
        let client = pool.client(server).clone();
        let file_len = layout.len;
        tasks.push(tokio::spawn(async move {
            let requests: Vec<Request> = chunks
                .iter()
                .map(|&(_, extent_id)| Request::ReadExtent { extent_id })
                .collect();
            let responses = client.call_many(&requests, pipeline).await?;
            let mut pieces = Vec::with_capacity(chunks.len());
            for (&(idx, _), response) in chunks.iter().zip(responses) {
                let Response::ReadAck { data } = response else {
                    return Err(ClusterError::Protocol(format!(
                        "expected ReadAck, got {response:?}"
                    )));
                };
                let want = STRIPE_UNIT.min(file_len - idx * STRIPE_UNIT) as usize;
                if data.len() != want {
                    return Err(ClusterError::LengthMismatch {
                        chunk: idx,
                        got: data.len(),
                        want,
                    });
                }
                pieces.push((idx * STRIPE_UNIT, data));
            }
            Ok::<_, ClusterError>(pieces)
        }));
    }
    let mut out = vec![0u8; layout.len as usize];
    for task in tasks {
        for (start, data) in task.await?? {
            out[start as usize..start as usize + data.len()].copy_from_slice(&data);
        }
    }
    Ok(out)
}

/// Read `range` (start, len) of a striped file: only the chunks
/// intersecting the range are fetched.
pub async fn stripe_read_range(
    pool: &Pool,
    layout: &Layout,
    start: u64,
    len: u64,
    concurrency: usize,
) -> Result<Vec<u8>, ClusterError> {
    let end = (start + len).min(layout.len);
    if start >= end {
        return Ok(Vec::new());
    }
    let first = start / STRIPE_UNIT;
    let last = (end - 1) / STRIPE_UNIT;
    let semaphore = Arc::new(Semaphore::new(concurrency.max(1)));
    let mut tasks = Vec::with_capacity((last - first + 1) as usize);
    for idx in first..=last {
        let loc = layout.chunks[idx as usize];
        // Expected payload of an absolute chunk: full unit except for a
        // short tail chunk at the file end.
        let want = (STRIPE_UNIT).min(layout.len - idx * STRIPE_UNIT) as usize;
        let client = pool.client(loc.server).clone();
        let permit = semaphore
            .clone()
            .acquire_owned()
            .await
            .map_err(|_| ClusterError::Acquire)?;
        tasks.push(tokio::spawn(async move {
            let _permit = permit;
            let data = client.read_extent(loc.extent_id).await?;
            if data.len() != want {
                return Err(ClusterError::LengthMismatch {
                    chunk: idx,
                    got: data.len(),
                    want,
                });
            }
            Ok::<_, ClusterError>((idx, data))
        }));
    }
    let window = ((last - first + 1) * STRIPE_UNIT) as usize;
    let mut whole = vec![0u8; window];
    for task in tasks {
        let (idx, data) = task.await??;
        let off = (idx - first) as usize * STRIPE_UNIT as usize;
        whole[off..off + data.len()].copy_from_slice(&data);
    }
    let head = (start - first * STRIPE_UNIT) as usize;
    let want = (end - start) as usize;
    Ok(whole[head..head + want].to_vec())
}
