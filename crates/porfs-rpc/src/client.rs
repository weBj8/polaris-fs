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
}

impl ChunkClient {
    /// A client for `addr` (connects lazily on the first op).
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            inner: std::sync::Arc::new(Inner {
                addr,
                conn: Mutex::new(None),
                backoff: StdMutex::new(BACKOFF_MIN),
                nonce: rand_nonce(),
            }),
        }
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
        match self
            .call(&Request::WriteExtent {
                write_id,
                inode,
                logical_offset,
                data,
            })
            .await?
        {
            Response::WriteAck { extent_id } => Ok(extent_id),
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

    /// Logically delete one extent (idempotent).
    pub async fn tombstone(&self, extent_id: u64) -> Result<(), RpcError> {
        match self.call(&Request::Tombstone { extent_id }).await? {
            Response::TombstoneAck => Ok(()),
            other => Err(unexpected("TombstoneAck", &other)),
        }
    }

    /// Group-commit barrier; the returned horizon covers durable extents.
    pub async fn sync(&self) -> Result<u64, RpcError> {
        match self.call(&Request::Sync).await? {
            Response::SyncAck { confirmed_id } => Ok(confirmed_id),
            other => Err(unexpected("SyncAck", &other)),
        }
    }

    /// Store counters `(device_size, tail, live_bytes, extent_count,
    /// confirmed_id)`.
    pub async fn stats(&self) -> Result<(u64, u64, u64, u64, u64), RpcError> {
        match self.call(&Request::Stats).await? {
            Response::StatsAck {
                device_size,
                tail,
                live_bytes,
                extent_count,
                confirmed_id,
            } => Ok((device_size, tail, live_bytes, extent_count, confirmed_id)),
            other => Err(unexpected("StatsAck", &other)),
        }
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

    /// Connect + Hello, with exponential backoff on failures.
    async fn connect(&self) -> Result<Framed<TcpStream, LengthDelimitedCodec>, RpcError> {
        loop {
            let attempt = TcpStream::connect(self.inner.addr).await;
            match attempt {
                Ok(socket) => {
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
                            protocol_version, ..
                        }) if protocol_version == PROTOCOL_VERSION => {
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
