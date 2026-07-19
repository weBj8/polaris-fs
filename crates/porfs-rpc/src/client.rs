//! The chunkserver client: one lazily-established connection, in-order
//! requests, transparent single retry on a broken connection, exponential
//! backoff on connect failures (`docs/protocol.md` §4).

use std::net::SocketAddr;
use std::sync::Mutex as StdMutex;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use crate::proto::{self, MAX_FRAME, PROTOCOL_VERSION, Request, Response};
use crate::{ErrorCode, RpcError};

const BACKOFF_MIN: Duration = Duration::from_millis(50);
const BACKOFF_MAX: Duration = Duration::from_secs(2);

/// Client of one chunkserver. Cheap to clone: all clones share one
/// connection (ops are serialized on it, responses are in order).
#[derive(Clone)]
pub struct ChunkClient {
    inner: std::sync::Arc<Inner>,
}

struct Inner {
    addr: SocketAddr,
    conn: Mutex<Option<Framed<TcpStream, LengthDelimitedCodec>>>,
    backoff: StdMutex<Duration>,
    nonce: u64,
    /// Pinned store identity: when set, every (re)connect must answer
    /// `HelloAck` with exactly this `store_uuid` (protocol v3 §3).
    expected_uuid: Option<[u8; 16]>,
    /// The `store_uuid` of the last successful `HelloAck` (`None` until
    /// the first connect).
    store_uuid: StdMutex<Option<[u8; 16]>>,
}

/// Snapshot of a chunkserver's extent-store state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreStats {
    pub device_size: u64,
    pub tail: u64,
    pub live_bytes: u64,
    pub extent_count: u64,
    pub confirmed_id: u64,
}

impl StoreStats {
    /// Bytes not yet allocated in the append-only log.
    pub fn free_bytes(self) -> u64 {
        self.device_size.saturating_sub(self.tail)
    }
}

impl ChunkClient {
    /// A client for `addr` (connects lazily on the first op).
    pub fn new(addr: SocketAddr) -> Self {
        Self::build(addr, None)
    }

    /// A client for `addr` that refuses to talk to a store whose
    /// `HelloAck.store_uuid` differs from `expected` (protocol v3
    /// server-identity pinning). The check runs at every (re)connect and
    /// fails immediately — a mismatch is deterministic, so no backoff.
    pub fn new_pinned(addr: SocketAddr, expected: [u8; 16]) -> Self {
        Self::build(addr, Some(expected))
    }

    fn build(addr: SocketAddr, expected_uuid: Option<[u8; 16]>) -> Self {
        Self {
            inner: std::sync::Arc::new(Inner {
                addr,
                conn: Mutex::new(None),
                backoff: StdMutex::new(BACKOFF_MIN),
                nonce: rand_nonce(),
                expected_uuid,
                store_uuid: StdMutex::new(None),
            }),
        }
    }

    /// The store identity learned at the last successful `HelloAck`
    /// (`None` until the client has connected).
    pub fn store_uuid(&self) -> Option<[u8; 16]> {
        *self
            .inner
            .store_uuid
            .lock()
            .unwrap_or_else(|p| p.into_inner())
    }

    /// Append one extent; the returned id references it from now on.
    /// `write_id` makes retries safe (server-side dedup, protocol §3).
    pub async fn write_extent(
        &self,
        write_id: u128,
        inode: u64,
        logical_offset: u64,
        data: Vec<u8>,
    ) -> Result<u64, RpcError> {
        self.write_extent_replicated(write_id, inode, logical_offset, data, None)
            .await
            .map(|(extent_id, _)| extent_id)
    }

    /// Append one extent through this primary and, when `secondary` is set,
    /// require the primary to append it to that secondary before acknowledging.
    pub async fn write_extent_replicated(
        &self,
        write_id: u128,
        inode: u64,
        logical_offset: u64,
        data: Vec<u8>,
        secondary: Option<SocketAddr>,
    ) -> Result<(u64, Option<u64>), RpcError> {
        match self
            .call(&Request::WriteExtent {
                write_id,
                inode,
                logical_offset,
                data,
                next: secondary,
            })
            .await?
        {
            Response::WriteAck {
                extent_id,
                replica_extent_id,
            } => Ok((extent_id, replica_extent_id)),
            other => Err(unexpected("WriteAck", &other)),
        }
    }

    /// Read one extent back (CRC32C-verified by the server).
    pub async fn read_extent(&self, extent_id: u64) -> Result<Vec<u8>, RpcError> {
        match self.call(&Request::ReadExtent { extent_id }).await? {
            Response::ReadAck { data } => Ok(data),
            other => Err(unexpected("ReadAck", &other)),
        }
    }

    /// Read the `[offset, offset+len)` subrange of one extent (v3): the
    /// server reads and CRC32C-verifies the full record, then ships only
    /// `min(len, data_len - offset)` bytes; `offset == data_len` reads
    /// empty and `offset > data_len` is a `BadRequest`.
    pub async fn read_extent_range(
        &self,
        extent_id: u64,
        offset: u64,
        len: u32,
    ) -> Result<Vec<u8>, RpcError> {
        match self
            .call(&Request::ReadExtentRange {
                extent_id,
                offset,
                len,
            })
            .await?
        {
            Response::ReadAck { data } => Ok(data),
            other => Err(unexpected("ReadAck", &other)),
        }
    }

    /// Logically delete one extent (idempotent).
    pub async fn tombstone(&self, extent_id: u64) -> Result<(), RpcError> {
        match self.call(&Request::Tombstone { extent_id }).await? {
            Response::TombstoneAck => Ok(()),
            other => Err(unexpected("TombstoneAck", &other)),
        }
    }

    /// Group-commit barrier; the returned horizon covers durable extents.
    pub async fn sync(&self) -> Result<u64, RpcError> {
        self.sync_replicated(None)
            .await
            .map(|(confirmed_id, _)| confirmed_id)
    }

    /// Confirm this primary and, when `secondary` is set, its replica.
    pub async fn sync_replicated(
        &self,
        secondary: Option<SocketAddr>,
    ) -> Result<(u64, Option<u64>), RpcError> {
        match self.call(&Request::Sync { next: secondary }).await? {
            Response::SyncAck {
                confirmed_id,
                replica_confirmed_id,
            } => Ok((confirmed_id, replica_confirmed_id)),
            other => Err(unexpected("SyncAck", &other)),
        }
    }

    /// Store counters.
    pub async fn stats(&self) -> Result<StoreStats, RpcError> {
        match self.call(&Request::Stats).await? {
            Response::StatsAck {
                device_size,
                tail,
                live_bytes,
                extent_count,
                confirmed_id,
            } => Ok(StoreStats {
                device_size,
                tail,
                live_bytes,
                extent_count,
                confirmed_id,
            }),
            other => Err(unexpected("StatsAck", &other)),
        }
    }

    /// Drop the current connection, forcing a reconnect on the next op.
    /// Callers MUST do this after cancelling an in-flight op with an
    /// outer timeout: the cancelled request may have a response still in
    /// flight, and reusing the connection would hand that stale response
    /// to the next request (responses carry no ids, §4).
    pub async fn disconnect(&self) {
        *self.inner.conn.lock().await = None;
    }

    /// One request round trip, transparently re-established once when the
    /// connection breaks (op-level idempotency makes that retry safe,
    /// protocol §4).
    async fn call(&self, request: &Request) -> Result<Response, RpcError> {
        let mut guard = self.inner.conn.lock().await;
        let mut retried = false;
        loop {
            if guard.is_none() {
                *guard = Some(self.connect().await?);
            }
            let conn = guard.as_mut().expect("just connected");
            match roundtrip(conn, request).await {
                Ok(response) => return Ok(response),
                Err(err) if err.is_io() && !retried => {
                    *guard = None;
                    retried = true;
                }
                Err(err) => return Err(err),
            }
        }
    }

    /// Several requests pipelined on the wire (protocol §4: the server
    /// executes in arrival order, so responses arrive in order) with a
    /// sliding window of up to `window` in flight — the connection never
    /// drains between batches. This is the throughput path for striped
    /// reads.
    pub async fn call_many(
        &self,
        requests: &[Request],
        window: usize,
    ) -> Result<Vec<Response>, RpcError> {
        if requests.is_empty() {
            return Ok(Vec::new());
        }
        let mut guard = self.inner.conn.lock().await;
        let mut retried = false;
        loop {
            if guard.is_none() {
                *guard = Some(self.connect().await?);
            }
            let conn = guard.take().expect("just connected");
            match roundtrip_many(conn, requests, window.max(1)).await {
                Ok((conn, responses)) => {
                    *guard = Some(conn);
                    return Ok(responses);
                }
                // A dead connection is dropped; the transparent retry
                // reconnects once (ops are idempotent, protocol §4).
                Err(err) if err.is_io() && !retried => {
                    retried = true;
                }
                Err(err) => return Err(err),
            }
        }
    }

    /// Connect + Hello, with exponential backoff on failures.
    async fn connect(&self) -> Result<Framed<TcpStream, LengthDelimitedCodec>, RpcError> {
        loop {
            let attempt = TcpStream::connect(self.inner.addr).await;
            match attempt {
                Ok(socket) => {
                    // Request/response ping-pong: no Nagle stalls.
                    let _ = socket.set_nodelay(true);
                    let mut framed = LengthDelimitedCodec::builder()
                        .little_endian()
                        .max_frame_length(MAX_FRAME as usize)
                        .new_framed(socket);
                    let hello = Request::Hello {
                        protocol_version: PROTOCOL_VERSION,
                        client_nonce: self.inner.nonce,
                    };
                    match roundtrip(&mut framed, &hello).await {
                        Ok(Response::HelloAck {
                            protocol_version,
                            store_uuid,
                            ..
                        }) if protocol_version == PROTOCOL_VERSION => {
                            // A pinned identity mismatch is deterministic:
                            // fail at once instead of backing off (probing
                            // the wrong store would answer NotFound for
                            // every extent, protocol §3).
                            if let Some(expected) = self.inner.expected_uuid
                                && store_uuid != expected
                            {
                                return Err(RpcError::Protocol(format!(
                                    "store identity mismatch at {}: got {store_uuid:02x?}, expected {expected:02x?}",
                                    self.inner.addr
                                )));
                            }
                            *self
                                .inner
                                .store_uuid
                                .lock()
                                .unwrap_or_else(|p| p.into_inner()) = Some(store_uuid);
                            self.reset_backoff();
                            return Ok(framed);
                        }
                        Ok(Response::HelloAck { .. }) => {
                            return Err(RpcError::Remote {
                                code: ErrorCode::VersionMismatch,
                                message: format!(
                                    "server speaks another protocol than {PROTOCOL_VERSION}"
                                ),
                            });
                        }
                        Ok(other) => return Err(unexpected("HelloAck", &other)),
                        Err(err) if err.is_io() => self.sleep_backoff().await,
                        Err(err) => return Err(err),
                    }
                }
                Err(_) => self.sleep_backoff().await,
            }
        }
    }

    /// Wait one backoff slot and grow it (± jitter derived from the slot).
    async fn sleep_backoff(&self) {
        let current = {
            let mut slot = self.inner.backoff.lock().unwrap_or_else(|p| p.into_inner());
            let current = *slot;
            *slot = (*slot * 2).min(BACKOFF_MAX);
            current
        };
        tokio::time::sleep(current).await;
    }

    /// Successful connect resets the backoff to the floor.
    fn reset_backoff(&self) {
        let mut slot = self.inner.backoff.lock().unwrap_or_else(|p| p.into_inner());
        *slot = BACKOFF_MIN;
    }
}

/// One encoded request -> one decoded response over an open connection.
async fn roundtrip(
    conn: &mut Framed<TcpStream, LengthDelimitedCodec>,
    request: &Request,
) -> Result<Response, RpcError> {
    let payload = proto::encode(request)?;
    conn.send(bytes::Bytes::from(payload)).await?;
    let frame = conn
        .next()
        .await
        .ok_or_else(|| RpcError::Io(std::io::Error::from(std::io::ErrorKind::UnexpectedEof)))??;
    match proto::decode::<Response>(&frame)? {
        Response::Error { code, message } => Err(RpcError::Remote { code, message }),
        response => Ok(response),
    }
}

/// Sliding-window pipelining: keep up to `window` requests in flight,
/// reading responses in arrival order until all are answered. The
/// connection is consumed and reunited (split lets reads and writes
/// proceed independently); on error it is dropped and the caller
/// reconnects.
async fn roundtrip_many(
    conn: Framed<TcpStream, LengthDelimitedCodec>,
    requests: &[Request],
    window: usize,
) -> Result<(Framed<TcpStream, LengthDelimitedCodec>, Vec<Response>), RpcError> {
    let (mut sink, mut stream) = conn.split();
    let mut next = 0usize;
    let mut in_flight = 0usize;
    let mut out = Vec::with_capacity(requests.len());
    loop {
        tokio::select! {
            result = async {
                let payload = proto::encode(&requests[next])?;
                sink.feed(bytes::Bytes::from(payload)).await?;
                if next + 1 == requests.len() || in_flight + 1 == window {
                    sink.flush().await?;
                }
                Ok::<_, RpcError>(())
            }, if next < requests.len() && in_flight < window => {
                result?;
                next += 1;
                in_flight += 1;
            }
            frame = stream.next(), if in_flight > 0 => {
                let frame = frame
                    .ok_or_else(|| RpcError::Io(std::io::Error::from(std::io::ErrorKind::UnexpectedEof)))??;
                match proto::decode::<Response>(&frame)? {
                    Response::Error { code, message } => {
                        return Err(RpcError::Remote { code, message });
                    }
                    response => out.push(response),
                }
                in_flight -= 1;
                if out.len() == requests.len() {
                    let conn = sink.reunite(stream).map_err(|_| {
                        RpcError::Protocol("framed halves drifted apart".to_string())
                    })?;
                    return Ok((conn, out));
                }
            }
        }
    }
}

/// A response variant other than the expected one is a protocol violation.
fn unexpected(want: &str, got: &Response) -> RpcError {
    RpcError::Protocol(format!("expected {want}, got {got:?}"))
}

fn rand_nonce() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    secs ^ (std::process::id() as u64) << 32
}
