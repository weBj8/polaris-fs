//! Chunkserver RPC (protocol v1, `docs/protocol.md`): the transport,
//! client, and server halves of "extent read/write as a service".

pub mod client;
pub mod proto;
pub mod server;

pub use client::ChunkClient;
pub use proto::{ErrorCode, MAX_FRAME, PROTOCOL_VERSION, Request, Response};

/// Errors surfaced by the client (the server never fails a connection
/// without first sending `Response::Error` when it can).
#[derive(Debug, thiserror::Error)]
pub enum RpcError {
    /// Connection-level I/O failure (dropped, refused, reset).
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    /// Local protocol violation (codec failure, unexpected frame).
    #[error("protocol error: {0}")]
    Protocol(String),
    /// Frame encode/decode failure.
    #[error("codec error: {0}")]
    Codec(String),
    /// The peer answered `Response::Error`.
    #[error("remote error {code:?}: {message}")]
    Remote {
        /// Machine-readable failure code.
        code: ErrorCode,
        /// Human-readable detail.
        message: String,
    },
}

impl RpcError {
    /// True when the failure is a broken connection (safe to reconnect).
    pub fn is_io(&self) -> bool {
        matches!(self, RpcError::Io(_))
    }
}
