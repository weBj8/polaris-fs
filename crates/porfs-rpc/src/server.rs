//! The chunkserver: accepts TCP connections, enforces the hello handshake,
//! and executes ops on a dedicated store-owner thread (the `!Send`
//! [`ExtentStore`] never crosses threads — same seam as the FUSE worker).

use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::sync::mpsc;
use std::time::{SystemTime, UNIX_EPOCH};

use futures_util::stream::FuturesOrdered;
use futures_util::{SinkExt, StreamExt};
use porfs_store::{ExtentStore, StoreError};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{oneshot, watch};
use tokio_util::codec::LengthDelimitedCodec;

use crate::proto::{self, ErrorCode, MAX_FRAME, PROTOCOL_VERSION, Request, Response};
use crate::ChunkClient;

/// One unit of work for the store thread: op + reply slot.
type Job = (Request, oneshot::Sender<Response>);

/// A running chunkserver. Dropping it stops the accept loop; the store is
/// shut down (clean superblock) on the worker thread before it exits.
pub struct Server {
    addr: SocketAddr,
    shutdown: watch::Sender<bool>,
    accept_task: tokio::task::JoinHandle<()>,
    _worker_task: std::thread::JoinHandle<()>,
}

impl Server {
    /// The address the server is listening on.
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.shutdown.send(true);
        self.accept_task.abort();
        // The worker exits when every connection's job sender is gone.
    }
}

/// Serve the store opened by `open` on `listener` until `shutdown` fires.
/// `open` runs entirely on the store thread, so the `!Send`
/// [`ExtentStore`] never moves between threads.
pub async fn serve(
    listener: TcpListener,
    open: impl FnOnce() -> Result<ExtentStore, StoreError> + Send + 'static,
    mut shutdown: watch::Receiver<bool>,
) -> io::Result<Server> {
    let addr = listener.local_addr()?;
    let (shutdown_tx, _) = watch::channel(false);
    let (job_tx, job_rx) = mpsc::channel::<Job>();
    let (init_tx, init_rx) = mpsc::channel::<Result<(), StoreError>>();
    let worker = std::thread::spawn(move || match open() {
        Ok(store) => {
            if init_tx.send(Ok(())).is_ok() {
                store_worker(store, job_rx);
            }
        }
        Err(err) => {
            let _ = init_tx.send(Err(err));
        }
    });
    // A dead channel here means the worker thread panicked before
    // reporting the open outcome.
    init_rx
        .recv()
        .map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "store worker died during open"))?
        .map_err(io::Error::other)?;
    let accept_shutdown = shutdown_tx.subscribe();
    let accept_task = tokio::spawn(accept_loop(listener, job_tx, accept_shutdown));
    // Forward external shutdown into our own channel.
    let forward_tx = shutdown_tx.clone();
    tokio::spawn(async move {
        let _ = shutdown.changed().await;
        let _ = forward_tx.send(true);
    });
    Ok(Server {
        addr,
        shutdown: shutdown_tx,
        accept_task,
        _worker_task: worker,
    })
}

/// Accept connections until shutdown.
async fn accept_loop(
    listener: TcpListener,
    jobs: mpsc::Sender<Job>,
    mut shutdown: watch::Receiver<bool>,
) {
    let started_unix = now_unix();
    loop {
        tokio::select! {
            accept = listener.accept() => {
                match accept {
                    Ok((socket, _)) => {
                        // RPC is request/response ping-pong; Nagle +
                        // delayed-ACK would stall every exchange.
                        let _ = socket.set_nodelay(true);
                        let jobs = jobs.clone();
                        tokio::spawn(handle_conn(socket, jobs, started_unix));
                    }
                    Err(_) => tokio::time::sleep(std::time::Duration::from_millis(10)).await,
                }
            }
            _ = shutdown.changed() => return,
        }
    }
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Encode `response` and send it as one frame; `false` when the
/// connection is broken.
async fn send(
    sink: &mut (impl SinkExt<bytes::Bytes, Error = std::io::Error> + Unpin),
    response: &Response,
) -> bool {
    let payload = match proto::encode(response) {
        Ok(payload) => payload,
        Err(_) => return false,
    };
    sink.send(bytes::Bytes::from(payload)).await.is_ok()
}

/// In-flight dispatches per connection (bounds buffered payloads).
const MAX_INFLIGHT: usize = 32;

/// One client connection: Hello first, then frames are read continuously
/// while their responses stream back IN ORDER (FuturesOrdered) — the
/// store-thread dispatch latency of one op overlaps the framing of the
/// next, so a connection streams instead of round-tripping per op.
async fn handle_conn(socket: TcpStream, jobs: mpsc::Sender<Job>, started_unix: u64) {
    let framed = LengthDelimitedCodec::builder()
        .little_endian()
        .max_frame_length(MAX_FRAME as usize)
        .new_framed(socket);
    let (mut sink, mut stream) = framed.split();
    if !handshake(&mut sink, &mut stream, started_unix).await {
        return;
    }
    let mut responses = FuturesOrdered::new();
    loop {
        tokio::select! {
            frame = stream.next(), if responses.len() < MAX_INFLIGHT => {
                let Some(frame) = frame else { return };
                let Ok(bytes) = frame else { return };
                let request: Request = match proto::decode(&bytes) {
                    Ok(request) => request,
                    Err(_) => {
                        send(
                            &mut sink,
                            &Response::error(ErrorCode::BadRequest, "undecodable frame"),
                        )
                        .await;
                        return;
                    }
                };
                if matches!(request, Request::Hello { .. }) {
                    send(
                        &mut sink,
                        &Response::error(ErrorCode::BadRequest, "duplicate hello"),
                    )
                    .await;
                    return;
                }
                let jobs = jobs.clone();
                responses.push_back(async move { dispatch_chain(&jobs, request).await });
            }
            Some(response) = responses.next(), if !responses.is_empty() => {
                if !send(&mut sink, &response).await {
                    return;
                }
            }
        }
    }
}

/// Enforce Hello-first and the protocol version.
async fn handshake(
    sink: &mut (impl SinkExt<bytes::Bytes, Error = std::io::Error> + Unpin),
    stream: &mut (impl StreamExt<Item = Result<bytes::BytesMut, std::io::Error>> + Unpin),
    started_unix: u64,
) -> bool {
    let Some(frame) = stream.next().await else {
        return false;
    };
    let Ok(bytes) = frame else { return false };
    match proto::decode::<Request>(&bytes) {
        Ok(Request::Hello {
            protocol_version, ..
        }) if protocol_version == PROTOCOL_VERSION => {
            send(
                sink,
                &Response::HelloAck {
                    protocol_version: PROTOCOL_VERSION,
                    server_nonce: rand_nonce(),
                    started_unix,
                },
            )
            .await
        }
        Ok(Request::Hello { .. }) => {
            send(
                sink,
                &Response::error(
                    ErrorCode::VersionMismatch,
                    format!("server speaks protocol {PROTOCOL_VERSION}"),
                ),
            )
            .await;
            false
        }
        _ => {
            send(sink, &Response::error(ErrorCode::BadRequest, "hello first")).await;
            false
        }
    }
}

fn rand_nonce() -> u64 {
    // Cheap connection nonce for logs (not security): time ^ address.
    now_unix() ^ (std::process::id() as u64) << 32
}

/// Hand the op to the store thread and await its response.
async fn dispatch(jobs: &mpsc::Sender<Job>, request: Request) -> Response {
    let (tx, rx) = oneshot::channel();
    if jobs.send((request, tx)).is_err() {
        return Response::error(ErrorCode::Internal, "store worker gone");
    }

    /// Execute the local half of a chain operation, then forward its immutable
    /// payload or durability barrier to the secondary. The forwarded request uses
    /// the ordinary client API with no secondary of its own, keeping P9 strictly
    /// two-way and avoiding forwarding loops.
    async fn dispatch_chain(jobs: &mpsc::Sender<Job>, request: Request) -> Response {
        match request {
            Request::WriteExtent {
                write_id,
                inode,
                logical_offset,
                data,
                next: Some(secondary),
            } => {
                let local = dispatch(
                    jobs,
                    Request::WriteExtent {
                        write_id,
                        inode,
                        logical_offset,
                        data: data.clone(),
                        next: None,
                    },
                )
                .await;
                let Response::WriteAck { extent_id, .. } = local else {
                    return local;
                };
                match ChunkClient::new(secondary)
                    .write_extent(write_id, inode, logical_offset, data)
                    .await
                {
                    Ok(replica_extent_id) => Response::WriteAck {
                        extent_id,
                        replica_extent_id: Some(replica_extent_id),
                    },
                    Err(err) => Response::error(ErrorCode::ReplicaUnavailable, err.to_string()),
                }
            }
            Request::Sync {
                next: Some(secondary),
            } => {
                let local = dispatch(jobs, Request::Sync { next: None }).await;
                let Response::SyncAck { confirmed_id, .. } = local else {
                    return local;
                };
                match ChunkClient::new(secondary).sync().await {
                    Ok(replica_confirmed_id) => Response::SyncAck {
                        confirmed_id,
                        replica_confirmed_id: Some(replica_confirmed_id),
                    },
                    Err(err) => Response::error(ErrorCode::ReplicaUnavailable, err.to_string()),
                }
            }
            request => dispatch(jobs, request).await,
        }
    }
    rx.await
        .unwrap_or_else(|_| Response::error(ErrorCode::Internal, "store worker died"))
}

/// The store thread: serializes every op against the extent store.
fn store_worker(mut store: ExtentStore, rx: mpsc::Receiver<Job>) {
    let mut dedup: HashMap<u128, u64> = HashMap::new();
    for (request, reply) in rx {
        let response = execute(&mut store, &mut dedup, request);
        if reply.send(response).is_err() {
            // Client went away; keep serving others.
        }
    }
}

/// Execute one op, mapping store errors to protocol errors.
fn execute(store: &mut ExtentStore, dedup: &mut HashMap<u128, u64>, request: Request) -> Response {
    match request {
        Request::Hello { .. } => Response::error(ErrorCode::BadRequest, "duplicate hello"),
        Request::WriteExtent {
            write_id,
            inode,
            logical_offset,
            data,
            next: _,
        } => {
            if data.is_empty() {
                return Response::error(ErrorCode::BadRequest, "empty extent payload");
            }
            if data.len() as u64 > porfs_format::EXTENT_DATA_MAX {
                return Response::error(
                    ErrorCode::BadRequest,
                    format!(
                        "payload {} bytes exceeds EXTENT_DATA_MAX {}",
                        data.len(),
                        porfs_format::EXTENT_DATA_MAX
                    ),
                );
            }
            if let Some(extent_id) = dedup.get(&write_id) {
                return Response::WriteAck {
                    extent_id: *extent_id,
                    replica_extent_id: None,
                };
            }
            match store.append(inode, logical_offset, &data) {
                Ok(extent_id) => {
                    dedup.insert(write_id, extent_id);
                    Response::WriteAck {
                        extent_id,
                        replica_extent_id: None,
                    }
                }
                Err(err) => store_error(err),
            }
        }
        Request::ReadExtent { extent_id } => match store.read(extent_id) {
            Ok(data) => Response::ReadAck { data },
            Err(err) => store_error(err),
        },
        Request::Tombstone { extent_id } => match store.discard(extent_id) {
            Ok(()) => Response::TombstoneAck,
            Err(StoreError::Tombstoned(_)) => Response::TombstoneAck, // idempotent
            Err(err) => store_error(err),
        },
        Request::Sync { next: _ } => match store.sync() {
            Ok(()) => Response::SyncAck {
                confirmed_id: store.confirmed_id(),
                replica_confirmed_id: None,
            },
            Err(err) => store_error(err),
        },
        Request::Stats => Response::StatsAck {
            device_size: store.device_size(),
            tail: store.tail(),
            live_bytes: store.live_bytes(),
            extent_count: store.extent_count(),
            confirmed_id: store.confirmed_id(),
        },
    }
}

/// Map store failures onto protocol error codes (`docs/protocol.md` §2).
fn store_error(err: StoreError) -> Response {
    match err {
        StoreError::UnknownExtent(_) | StoreError::Tombstoned(_) => {
            Response::error(ErrorCode::NotFound, err.to_string())
        }
        StoreError::CorruptData(_) | StoreError::CorruptHeader(_) => {
            Response::error(ErrorCode::Corrupt, err.to_string())
        }
        StoreError::DataTooLarge { .. } => Response::error(ErrorCode::BadRequest, err.to_string()),
        StoreError::DeviceFull { .. } => Response::error(ErrorCode::StoreFull, err.to_string()),
        other => Response::error(ErrorCode::Internal, other.to_string()),
    }
}
