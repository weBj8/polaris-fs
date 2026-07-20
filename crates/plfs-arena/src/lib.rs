//! ChunkArena core storage engine (on-disk format v1).
//!
//! One arena = one sparse image file or block device. Fixed-size slots in two
//! classes (L large / S small), a duplicated superblock, and a duplicated
//! allocation bitmap that is only a hint: the SEALED slot header is the source
//! of truth and the bitmap is rebuilt from a full header scan at every
//! [`Arena::open`]. Writes publish atomically header-last (payload first, one
//! 64-byte SEALED header write, then fdatasync).
//!
//! The I/O engine prefers `O_DIRECT` + io_uring (single-threaded software
//! pipeline: SQ fill, immediate submit, CQE reap, every result checked) and
//! falls back to plain buffered `pread`/`pwrite` where `O_DIRECT` is
//! unavailable (e.g. tmpfs). Both paths are equally correct.
//!
//! The binding contract is `docs/format-arena.md` v1; this crate implements
//! exactly that document.

mod arena;
mod bitmap;
mod geom;
mod header;
mod index;
mod io;
mod sb;

pub use arena::{Arena, CorruptionEvidence, PutOutcome, SparsifyReport};
pub use geom::{Geometry, MkfsConfig, MkfsReport};
pub use index::{ChunkMeta, SlotClass};

use std::fmt;

/// Errors returned by ChunkArena operations.
#[derive(Debug, thiserror::Error)]
pub enum ArenaError {
    /// Neither superblock copy passed magic/crc validation (or fields are
    /// internally inconsistent).
    #[error("no valid superblock found")]
    BadSuperblock,
    /// The superblock is valid but names a format version this build rejects.
    #[error("unsupported format version {0}")]
    UnsupportedVersion(u32),
    /// On-disk state for a live chunk failed validation (header or payload).
    #[error("chunk {chunk_id} corrupt: {detail}")]
    Corrupt {
        /// The chunk whose bytes failed validation.
        chunk_id: ChunkId,
        /// What failed.
        detail: &'static str,
    },
    /// No live chunk with this id.
    #[error("chunk not found: {0}")]
    NotFound(ChunkId),
    /// The id is already sealed with a different (version, payload_crc32c).
    #[error("chunk already exists with different version/payload: {0}")]
    AlreadyExists(ChunkId),
    /// Payload exceeds the L-slot payload capacity.
    #[error("payload too large: {len} bytes (max {max})")]
    Oversize {
        /// Offered payload length.
        len: usize,
        /// L-slot payload capacity.
        max: usize,
    },
    /// No free slot of the required class (no cross-class fallback in v1).
    #[error("out of space: no free {class:?}-class slot")]
    OutOfSpace {
        /// The exhausted slot class.
        class: SlotClass,
    },
    /// Underlying I/O failure (including short transfers and ring failures,
    /// surfaced as `ErrorKind::Other`/`UnexpectedEof`).
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// Convenience result alias.
pub type Result<T> = std::result::Result<T, ArenaError>;

/// Chunk identifier: 16 bytes, UUIDv7 layout (RFC 4122). The all-zero (nil)
/// id is forbidden on disk; mutating calls reject it.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ChunkId([u8; 16]);

impl ChunkId {
    /// Fresh time-ordered UUIDv7 chunk id.
    pub fn new_v7() -> Self {
        Self(uuid::Uuid::now_v7().into_bytes())
    }

    /// The raw 16 bytes (RFC 4122 layout).
    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }

    /// True for the all-zero id (never valid for a live chunk).
    pub fn is_nil(&self) -> bool {
        self.0 == [0; 16]
    }
}

impl From<[u8; 16]> for ChunkId {
    fn from(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }
}

impl From<ChunkId> for [u8; 16] {
    fn from(id: ChunkId) -> Self {
        id.0
    }
}

impl fmt::Display for ChunkId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        uuid::Uuid::from_bytes(self.0).fmt(f)
    }
}

impl fmt::Debug for ChunkId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ChunkId({self})")
    }
}

/// Probe whether io_uring is usable in this environment (kernel + seccomp).
/// When false the arena silently uses the buffered syscall path.
pub fn io_uring_available() -> bool {
    io::io_uring_available()
}

/// Directory for test device files: `CARGO_TARGET_TMPDIR` (real disk) when
/// available, else the workspace `target/tmp` derived from the manifest dir.
/// (/tmp is tmpfs RAM on the dev box; device files must live on real disk.)
#[cfg(test)]
pub(crate) fn test_tmpdir() -> std::path::PathBuf {
    let dir = std::env::var("CARGO_TARGET_TMPDIR")
        .unwrap_or_else(|_| concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/tmp").to_string());
    let dir = std::path::PathBuf::from(dir);
    std::fs::create_dir_all(&dir).expect("create test tmpdir");
    dir
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_id_v7_is_non_nil_and_unique() {
        let a = ChunkId::new_v7();
        let b = ChunkId::new_v7();
        assert!(!a.is_nil());
        assert_ne!(a, b);
    }

    #[test]
    fn chunk_id_bytes_roundtrip() {
        let bytes = [0xAB; 16];
        let id = ChunkId::from(bytes);
        assert_eq!(id.as_bytes(), &bytes);
        assert_eq!(<[u8; 16]>::from(id), bytes);
        assert!(ChunkId::from([0; 16]).is_nil());
    }
}
