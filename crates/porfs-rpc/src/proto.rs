//! Message types of wire protocol v1 (`docs/protocol.md`): a
//! length-prefixed frame carries one bincode-2 (standard config, serde)
//! `Request` or `Response`.

use serde::{Deserialize, Serialize};

/// Wire protocol version implemented by this crate.
pub const PROTOCOL_VERSION: u16 = 2;

/// Maximum accepted frame payload (an extent payload plus envelope slack);
/// larger frames get the connection dropped.
pub const MAX_FRAME: u32 = 8 * 1024 * 1024;

/// Client -> server requests (variant indices are the contract).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Request {
    /// First frame on a new connection.
    Hello {
        protocol_version: u16,
        client_nonce: u64,
    },
    /// Append one extent record (idempotent per server lifetime via
    /// `write_id`).
    WriteExtent {
        write_id: u128,
        inode: u64,
        logical_offset: u64,
        /// serde_bytes: one length-prefixed memcpy, not element-wise serde.
        #[serde(with = "serde_bytes")]
        data: Vec<u8>,
        /// Optional secondary receiving the same write from this primary.
        next: Option<std::net::SocketAddr>,
    },
    /// Read one extent back, CRC32C-verified.
    ReadExtent { extent_id: u64 },
    /// Logically delete one extent (idempotent).
    Tombstone { extent_id: u64 },
    /// Durability barrier (group commit).
    Sync {
        /// Optional secondary that must confirm the same durability barrier.
        next: Option<std::net::SocketAddr>,
    },
    /// Store counters.
    Stats,
}

/// Server -> client responses (variant indices are the contract).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Response {
    /// Hello accepted; `Error(VersionMismatch)` + close otherwise.
    HelloAck {
        protocol_version: u16,
        server_nonce: u64,
        started_unix: u64,
    },
    /// WriteExtent accepted (appended, not yet durable — see Sync).
    WriteAck {
        extent_id: u64,
        replica_extent_id: Option<u64>,
    },
    /// Extent payload, CRC32C-verified by the server.
    ReadAck {
        /// serde_bytes: one length-prefixed memcpy, not element-wise serde.
        #[serde(with = "serde_bytes")]
        data: Vec<u8>,
    },
    /// Tombstone applied (or already applied).
    TombstoneAck,
    /// Durability horizon after a successful group commit.
    SyncAck {
        confirmed_id: u64,
        replica_confirmed_id: Option<u64>,
    },
    /// Store counters.
    StatsAck {
        device_size: u64,
        tail: u64,
        live_bytes: u64,
        extent_count: u64,
        confirmed_id: u64,
    },
    /// Any request failed.
    Error { code: ErrorCode, message: String },
}

/// Machine-readable failure codes (indices are the contract).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorCode {
    /// Malformed request (bad op order, empty/oversize payload, ...).
    BadRequest,
    /// Unknown or tombstoned extent id.
    NotFound,
    /// CRC32C mismatch or salvage failure on the device.
    Corrupt,
    /// Device cannot fit the write.
    StoreFull,
    /// Unsupported protocol version.
    VersionMismatch,
    /// Anything else.
    Internal,
    /// A required secondary could not append or confirm the operation.
    ReplicaUnavailable,
}

impl Response {
    /// Construct an `Error` response.
    pub fn error(code: ErrorCode, message: impl Into<String>) -> Self {
        Self::Error {
            code,
            message: message.into(),
        }
    }
}

/// Encode one message into its frame payload.
pub fn encode<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, crate::RpcError> {
    bincode::serde::encode_to_vec(value, bincode::config::standard())
        .map_err(|err| crate::RpcError::Codec(err.to_string()))
}

/// Decode one frame payload into `T`, rejecting trailing bytes.
pub fn decode<T: serde::de::DeserializeOwned>(buf: &[u8]) -> Result<T, crate::RpcError> {
    let (value, read): (T, usize) =
        bincode::serde::decode_from_slice(buf, bincode::config::standard())
            .map_err(|err| crate::RpcError::Codec(err.to_string()))?;
    if read != buf.len() {
        return Err(crate::RpcError::Codec(format!(
            "trailing {} bytes after message",
            buf.len() - read
        )));
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip<T>(value: T)
    where
        T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
    {
        let buf = encode(&value).unwrap();
        let back: T = decode(&buf).unwrap();
        assert_eq!(back, value);
    }

    #[test]
    fn all_messages_roundtrip() {
        roundtrip(Request::Hello {
            protocol_version: PROTOCOL_VERSION,
            client_nonce: 42,
        });
        roundtrip(Request::WriteExtent {
            write_id: u128::MAX,
            inode: 7,
            logical_offset: 1 << 40,
            data: vec![0xAB; 100],
            next: None,
        });
        roundtrip(Request::ReadExtent { extent_id: 9 });
        roundtrip(Request::Tombstone { extent_id: 9 });
        roundtrip(Request::Sync { next: None });
        roundtrip(Request::Stats);

        roundtrip(Response::HelloAck {
            protocol_version: PROTOCOL_VERSION,
            server_nonce: 1,
            started_unix: 1_700_000_000,
        });
        roundtrip(Response::WriteAck {
            extent_id: 3,
            replica_extent_id: Some(4),
        });
        roundtrip(Response::ReadAck { data: Vec::new() });
        roundtrip(Response::TombstoneAck);
        roundtrip(Response::SyncAck {
            confirmed_id: 11,
            replica_confirmed_id: Some(12),
        });
        roundtrip(Response::StatsAck {
            device_size: 1 << 30,
            tail: 1 << 20,
            live_bytes: 1 << 19,
            extent_count: 5,
            confirmed_id: 11,
        });
        roundtrip(Response::error(ErrorCode::StoreFull, "full"));
    }

    #[test]
    fn decode_rejects_trailing_garbage() {
        let mut buf = encode(&Request::Sync { next: None }).unwrap();
        buf.push(0);
        assert!(decode::<Request>(&buf).is_err());
    }

    #[test]
    fn decode_rejects_truncated() {
        let buf = encode(&Request::Stats).unwrap();
        assert!(decode::<Request>(&buf[..buf.len() - 1]).is_err());
    }
}
