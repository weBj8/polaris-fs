//! PolarisFS on-disk format v2 (`FORMAT_VERSION = 2`).
//!
//! This crate is the executable form of `docs/format.md`; the document is the
//! contract and this code is its servant. Any format change must land in
//! `docs/format.md` first and bump [`FORMAT_VERSION`].
//!
//! Layout: superblock copy A at `[0..4KiB)`, copy B at `[4KiB..8KiB)`,
//! checkpoint slot A at `[8KiB..32MiB)`, checkpoint slot B at `[32MiB..64MiB)`,
//! then the append-only extent log from `DATA_START` (64MiB). All multi-byte
//! integers are little-endian; the `repr(C)` Pod structs below are the exact
//! serialized form (PolarisFS targets little-endian platforms only).
//!
//! v2 has no migration path: v1 devices are rejected with
//! [`FormatError::UnsupportedVersion`].

pub mod checkpoint;
pub mod extent;
pub mod superblock;

#[cfg(target_arch = "x86_64")]
mod crc32c_hw;

pub use checkpoint::{CheckpointEntry, CheckpointHeader};
pub use extent::ExtentHeader;
pub use superblock::Superblock;

/// I/O block size; every on-disk structure and every I/O is aligned to this.
pub const BLOCK_SIZE: u64 = 4096;
/// Byte offset where the extent log region starts.
pub const DATA_START: u64 = 64 * 1024 * 1024;
/// Maximum payload bytes carried by a single extent record.
pub const EXTENT_DATA_MAX: u64 = 4 * 1024 * 1024;
/// Superblock magic at offset 0 of each superblock copy.
pub const SB_MAGIC: &[u8; 8] = b"PORFS_SB";
/// Extent header magic (`"EXT1"` little-endian).
pub const EXT_MAGIC: u32 = 0x4558_5431;
/// Checkpoint header magic (`"CKP1"` little-endian).
pub const CKPT_MAGIC: u32 = 0x3150_4B43;
/// On-disk format version.
pub const FORMAT_VERSION: u32 = 2;

/// Byte offset of superblock copy A (the authoritative copy).
pub const SB_A_OFFSET: u64 = 0;
/// Byte offset of superblock copy B (written first during sync).
pub const SB_B_OFFSET: u64 = BLOCK_SIZE;
/// Byte offset of checkpoint slot A.
pub const CKPT_SLOT_A_OFF: u64 = 8 * 1024;
/// Byte offset of checkpoint slot B.
pub const CKPT_SLOT_B_OFF: u64 = 32 * 1024 * 1024;
/// Capacity of one checkpoint slot in bytes. Slot B physically extends to
/// 64MiB but is capped to slot A's capacity by definition (format.md §8).
pub const CKPT_SLOT_CAP: u64 = 32 * 1024 * 1024 - 8 * 1024;
/// Automatic checkpoint interval: one checkpoint every this many syncs.
pub const CKPT_INTERVAL: u64 = 8;
/// Superblock flag bit0: clean unmount (set on normal unmount, cleared while mounted).
pub const SB_FLAG_CLEAN: u32 = 1 << 0;
/// Extent header flag bit0: tombstone (logical delete).
pub const EXT_FLAG_TOMBSTONE: u16 = 1 << 0;

/// CRC32C (Castagnoli) checksum used for superblocks, headers, and extent data.
/// Uses the PCLMULQDQ hardware path on x86_64 when available, with the
/// `crc32c` crate (SSE4.2) as fallback.
#[cfg(target_arch = "x86_64")]
pub fn checksum(data: &[u8]) -> u32 {
    crc32c_hw::append(0, data)
}

/// CRC32C (Castagnoli) checksum (portable fallback path).
#[cfg(not(target_arch = "x86_64"))]
pub fn checksum(data: &[u8]) -> u32 {
    crc32c::crc32c(data)
}

/// Incremental CRC32C: extend a running checksum with more data.
#[cfg(target_arch = "x86_64")]
pub fn checksum_append(crc: u32, data: &[u8]) -> u32 {
    crc32c_hw::append(crc, data)
}

/// Incremental CRC32C (portable fallback path).
#[cfg(not(target_arch = "x86_64"))]
pub fn checksum_append(crc: u32, data: &[u8]) -> u32 {
    crc32c::crc32c_append(crc, data)
}

/// Total on-disk size of an extent record carrying `data_len` payload bytes:
/// 64-byte header + data + zero padding up to a 4KiB multiple.
pub const fn extent_disk_len(data_len: u64) -> u64 {
    (ExtentHeader::LEN as u64 + data_len).div_ceil(BLOCK_SIZE) * BLOCK_SIZE
}

/// Errors detected while parsing or validating on-disk structures.
#[derive(Debug, thiserror::Error)]
pub enum FormatError {
    /// Superblock magic mismatch.
    #[error("bad superblock magic")]
    BadSuperblockMagic,
    /// Unsupported `format_version` field.
    #[error("unsupported format version {0}")]
    UnsupportedVersion(u32),
    /// Superblock CRC32C mismatch.
    #[error("superblock crc32c mismatch")]
    BadSuperblockCrc,
    /// Extent header magic mismatch.
    #[error("bad extent header magic")]
    BadExtentMagic,
    /// Extent header `header_len` field is not 64.
    #[error("bad extent header length {0}")]
    BadHeaderLen(u16),
    /// Extent header CRC32C mismatch.
    #[error("extent header crc32c mismatch")]
    BadHeaderCrc,
    /// Checkpoint header magic mismatch.
    #[error("bad checkpoint magic")]
    BadCheckpointMagic,
    /// Checkpoint header `header_len` field is not 64.
    #[error("bad checkpoint header length {0}")]
    BadCheckpointHeaderLen(u16),
    /// Checkpoint header CRC32C mismatch.
    #[error("checkpoint header crc32c mismatch")]
    BadCheckpointHeaderCrc,
    /// Checkpoint payload CRC32C mismatch.
    #[error("checkpoint payload crc32c mismatch")]
    BadCheckpointPayloadCrc,
    /// Byte slice has the wrong length for the structure being decoded.
    #[error("truncated structure: need {need} bytes, got {got}")]
    Truncated {
        /// Required length in bytes.
        need: usize,
        /// Actual length in bytes.
        got: usize,
    },
}
