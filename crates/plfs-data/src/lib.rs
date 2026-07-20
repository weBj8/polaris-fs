//! Data node gRPC service (design doc §5, wire contract v1): the five
//! `ChunkStore` RPCs over a ChunkArena. The arena (io_uring engine, `!Send`
//! I/O state) never leaves its dedicated actor thread; async handlers
//! exchange commands with it over a channel, one in-flight op at a time per
//! connection — the arena itself is single-threaded by design (§6.3).
//!
//! Semantics anchored to the contract:
//! - `Put` is idempotent on (chunk_id, version, crc): a retry of the same
//!   request succeeds as a no-op; the request crc32c is verified before the
//!   payload is trusted. `seal = false` is reserved (UNIMPLEMENTED in v1).
//! - `Get` streams 64 KiB frames, first frame carrying version/len/crc;
//!   `if_version` equal to the stored version yields a single
//!   `not_modified` frame without reading the payload (the 304 semantic
//!   behind cache validation, §7.1).
//! - `Delete` is exact-version and idempotent: absent or mismatched version
//!   is a silent success (GC vs re-Put race protection).

use std::net::SocketAddr;
use std::path::Path;
use std::sync::mpsc::{self, Sender};

use bytes::Bytes;
use plfs_arena::{Arena, ArenaError, ChunkId, ChunkMeta, PutOutcome};
use plfs_common::data::v1 as pb;
use plfs_common::data::v1::chunk_store_server::{ChunkStore, ChunkStoreServer};
use tokio::sync::oneshot;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

/// Get stream frame size (design doc §5: 64 KiB frames keep memory flat).
const GET_FRAME: usize = 64 * 1024;

/// Errors from running a data node.
#[derive(Debug, thiserror::Error)]
pub enum DataNodeError {
    /// Arena open/operation failure.
    #[error("arena: {0}")]
    Arena(#[from] ArenaError),
    /// gRPC transport failure.
    #[error("transport: {0}")]
    Transport(#[from] tonic::transport::Error),
}

static QUEUE_DEPTH: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(0);

enum Cmd {
    Put {
        id: ChunkId,
        version: u64,
        payload: Bytes,
        reply: oneshot::Sender<Result<PutOutcome, ArenaError>>,
    },
    Get {
        id: ChunkId,
        reply: oneshot::Sender<Result<(u64, Vec<u8>), ArenaError>>,
    },
    Stat {
        id: ChunkId,
        reply: oneshot::Sender<Option<ChunkMeta>>,
    },
    Delete {
        id: ChunkId,
        version: u64,
        reply: oneshot::Sender<Result<bool, ArenaError>>,
    },
    List {
        reply: oneshot::Sender<Vec<ChunkMeta>>,
    },
    Capacity {
        reply: oneshot::Sender<(u64, u64)>,
    },
}

/// Owns the arena on a dedicated thread; all ops are serialized through the
/// command channel. The arena (`!Send`: io_uring ring + aligned buffers) is
/// created ON its thread and never crosses it. When every sender is dropped
/// (server shutdown) the loop ends and the arena's Drop flushes the bitmaps.
fn spawn_actor(arena_path: std::path::PathBuf) -> Result<Sender<Cmd>, ArenaError> {
    let (tx, rx) = mpsc::channel::<Cmd>();
    let (init_tx, init_rx) = mpsc::channel::<Result<(), ArenaError>>();
    std::thread::Builder::new()
        .name("plfs-arena".into())
        .spawn(move || {
            let mut arena = match Arena::open(&arena_path) {
                Ok(arena) => {
                    let _ = init_tx.send(Ok(()));
                    arena
                }
                Err(err) => {
                    let _ = init_tx.send(Err(err));
                    return;
                }
            };
            while let Ok(cmd) = rx.recv() {
                let depth = QUEUE_DEPTH.fetch_sub(1, std::sync::atomic::Ordering::Relaxed) - 1;
                metrics::gauge!("plfs_arena_queue_depth").set(depth as f64);
                match cmd {
                    Cmd::Put {
                        id,
                        version,
                        payload,
                        reply,
                    } => {
                        let _ = reply.send(arena.put(id, version, &payload));
                    }
                    Cmd::Get { id, reply } => {
                        let _ = reply.send(arena.get(&id));
                    }
                    Cmd::Stat { id, reply } => {
                        let _ = reply.send(arena.stat(&id));
                    }
                    Cmd::Delete { id, version, reply } => {
                        let _ = reply.send(arena.delete(&id, version));
                    }
                    Cmd::List { reply } => {
                        let _ = reply.send(arena.list());
                    }
                    Cmd::Capacity { reply } => {
                        let geom = arena.geometry();
                        let l_cap = u64::from(geom.l_slot_size) - 64;
                        let s_cap = u64::from(geom.s_slot_size) - 64;
                        let total = geom.l_slot_count * l_cap + geom.s_slot_count * s_cap;
                        let free = arena.free_slots(plfs_arena::SlotClass::L) * l_cap
                            + arena.free_slots(plfs_arena::SlotClass::S) * s_cap;
                        let _ = reply.send((total, total - free));
                    }
                }
            }
            let _ = arena.close();
        })
        .expect("spawn arena actor thread");
    init_rx
        .recv()
        .expect("actor thread died during arena open")?;
    Ok(tx)
}

/// The `ChunkStore` service: one ChunkArena behind a command channel.
#[derive(Clone)]
pub struct ChunkStoreSvc {
    tx: Sender<Cmd>,
}

impl ChunkStoreSvc {
    /// Open the arena at `path` (on its actor thread) and spawn the actor.
    pub fn open(arena_path: impl AsRef<Path>) -> Result<Self, ArenaError> {
        Ok(Self {
            tx: spawn_actor(arena_path.as_ref().to_path_buf())?,
        })
    }

    /// The tonic server for this service (composition point for `serve` and
    /// tests binding their own listener).
    pub fn into_server(self) -> ChunkStoreServer<Self> {
        ChunkStoreServer::new(self)
    }

    /// (capacity_bytes, used_bytes) of the hosted arena (registry
    /// registration + heartbeat).
    pub async fn capacity_used(&self) -> Result<(u64, u64), Status> {
        self.call(|reply| Cmd::Capacity { reply }).await
    }

    async fn call<R>(&self, bind: impl FnOnce(oneshot::Sender<R>) -> Cmd) -> Result<R, Status> {
        let (reply, rx) = oneshot::channel();
        let depth = QUEUE_DEPTH.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
        metrics::gauge!("plfs_arena_queue_depth").set(depth as f64);
        self.tx
            .send(bind(reply))
            .map_err(|_| Status::unavailable("arena actor is gone"))?;
        rx.await
            .map_err(|_| Status::unavailable("arena actor dropped the reply"))
    }
}

fn chunk_id(bytes: &[u8]) -> Result<ChunkId, Status> {
    let raw: [u8; 16] = bytes
        .try_into()
        .map_err(|_| Status::invalid_argument("chunk_id must be 16 bytes"))?;
    let id = ChunkId::from(raw);
    if id.is_nil() {
        return Err(Status::invalid_argument("nil chunk_id is forbidden"));
    }
    Ok(id)
}

fn status(err: ArenaError) -> Status {
    match err {
        ArenaError::NotFound(_) => Status::not_found(err.to_string()),
        ArenaError::AlreadyExists(_) => Status::already_exists(err.to_string()),
        ArenaError::Oversize { .. } => Status::out_of_range(err.to_string()),
        ArenaError::OutOfSpace { .. } => Status::resource_exhausted(err.to_string()),
        ArenaError::Corrupt { .. } => Status::data_loss(err.to_string()),
        ArenaError::BadSuperblock | ArenaError::UnsupportedVersion(_) | ArenaError::Io(_) => {
            Status::internal(err.to_string())
        }
    }
}

#[tonic::async_trait]
impl ChunkStore for ChunkStoreSvc {
    async fn put(
        &self,
        request: Request<pb::PutRequest>,
    ) -> Result<Response<pb::PutResponse>, Status> {
        let req = request.into_inner();
        if !req.seal {
            return Err(Status::unimplemented(
                "unsealed put is reserved (wire/format v1 supports sealed only)",
            ));
        }
        let id = chunk_id(&req.chunk_id)?;
        if crc32fast::hash(&req.payload) != req.crc32c {
            return Err(Status::invalid_argument("payload crc32c mismatch"));
        }
        self.call(|reply| Cmd::Put {
            id,
            version: req.version,
            payload: Bytes::from(req.payload),
            reply,
        })
        .await?
        .map_err(status)?;
        Ok(Response::new(pb::PutResponse {}))
    }

    type GetStream = ReceiverStream<Result<pb::GetReply, Status>>;

    async fn get(
        &self,
        request: Request<pb::GetRequest>,
    ) -> Result<Response<Self::GetStream>, Status> {
        let req = request.into_inner();
        let id = chunk_id(&req.chunk_id)?;
        // Stat first: an if_version hit must NOT read the payload (the cheap
        // 304 behind cache validation, design doc §7.1).
        let meta = self
            .call(|reply| Cmd::Stat { id, reply })
            .await?
            .ok_or_else(|| Status::not_found("chunk not found"))?;
        if req.if_version != 0 && req.if_version == meta.version {
            let (tx, rx) = tokio::sync::mpsc::channel(1);
            let _ = tx
                .try_send(Ok(pb::GetReply {
                    version: meta.version,
                    total_len: 0,
                    crc32c: meta.payload_crc32c,
                    not_modified: true,
                    data: Vec::new(),
                }))
                .ok();
            return Ok(Response::new(ReceiverStream::new(rx)));
        }
        let (version, payload) = self
            .call(|reply| Cmd::Get { id, reply })
            .await?
            .map_err(status)?;
        // ≤ 1 MiB payload ⇒ ≤ 17 frames; a pre-sized channel keeps the
        // handler synchronous and memory bounded by one chunk.
        let frames = payload.len().div_ceil(GET_FRAME).max(1);
        let (tx, rx) = tokio::sync::mpsc::channel(frames);
        for w in 0..frames {
            let start = w * GET_FRAME;
            let end = (start + GET_FRAME).min(payload.len());
            let _ = tx
                .try_send(Ok(pb::GetReply {
                    version,
                    total_len: payload.len() as u64,
                    crc32c: meta.payload_crc32c,
                    not_modified: false,
                    data: payload[start..end].to_vec(),
                }))
                .ok();
        }
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn stat(
        &self,
        request: Request<pb::StatRequest>,
    ) -> Result<Response<pb::StatReply>, Status> {
        let req = request.into_inner();
        let id = chunk_id(&req.chunk_id)?;
        let meta = self
            .call(|reply| Cmd::Stat { id, reply })
            .await?
            .ok_or_else(|| Status::not_found("chunk not found"))?;
        Ok(Response::new(pb::StatReply {
            version: meta.version,
            len: meta.payload_len,
            crc32c: meta.payload_crc32c,
            sealed: true,
        }))
    }

    async fn delete(
        &self,
        request: Request<pb::DeleteRequest>,
    ) -> Result<Response<pb::DeleteReply>, Status> {
        let req = request.into_inner();
        let id = chunk_id(&req.chunk_id)?;
        // Exact-version, idempotent: `false` (absent or version mismatch) is
        // still success — the GC re-Put race protection of design doc §5.
        self.call(|reply| Cmd::Delete {
            id,
            version: req.version,
            reply,
        })
        .await?
        .map_err(status)?;
        Ok(Response::new(pb::DeleteReply {}))
    }

    type ListStream = ReceiverStream<Result<pb::ListReply, Status>>;

    async fn list(
        &self,
        _request: Request<pb::ListRequest>,
    ) -> Result<Response<Self::ListStream>, Status> {
        let metas = self.call(|reply| Cmd::List { reply }).await?;
        let (tx, rx) = tokio::sync::mpsc::channel(64);
        tokio::spawn(async move {
            for m in metas {
                let item = Ok(pb::ListReply {
                    chunk_id: m.chunk_id.as_bytes().to_vec(),
                    version: m.version,
                    len: m.payload_len,
                    crc32c: m.payload_crc32c,
                });
                if tx.send(item).await.is_err() {
                    break;
                }
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

/// Run a data node on `addr` serving the arena at `arena_path`, until
/// SIGINT/SIGTERM. On shutdown the service is dropped, the actor channel
/// closes, and the arena closes cleanly (v2 checkpoint, format-arena §3.5).
pub async fn serve(arena_path: &Path, addr: SocketAddr) -> Result<(), DataNodeError> {
    let svc = ChunkStoreSvc::open(arena_path)?;
    tracing::info!(%addr, arena = %arena_path.display(), "data node serving");
    tonic::transport::Server::builder()
        .add_service(svc.into_server())
        .serve_with_shutdown(addr, async {
            let mut term =
                tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                    .expect("install SIGTERM handler");
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {}
                _ = term.recv() => {}
            }
        })
        .await?;
    Ok(())
}
