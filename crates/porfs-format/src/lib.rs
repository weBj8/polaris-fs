//! PolarisFS on-disk format v0 (`FORMAT_VERSION = 1`).
//!
//! This crate is the executable form of `docs/format.md`; the document is the
//! contract and this code is its servant. Any format change must land in
//! `docs/format.md` first and bump [`FORMAT_VERSION`].
//!
//! Layout: superblock copy A at `[0..4KiB)`, copy B at `[4KiB..8KiB)`, a zeroed
//! reserved gap up to `1MiB`, then the append-only extent log. All multi-byte
//! integers are little-endian; the `repr(C)` Pod structs below are the exact
//! serialized form (PolarisFS targets little-endian platforms only).

pub mod extent;
pub mod superblock;

#[cfg(target_arch = "x86_64")]
mod crc32c_hw;

pub use extent::ExtentHeader;
pub use superblock::Superblock;

/// I/O block size; every on-disk structure and every I/O is aligned to this.
pub const BLOCK_SIZE: u64 = 4096;
/// Byte offset where the extent log region starts.
pub const DATA_START: u64 = 1024 * 1024;
/// Maximum payload bytes carried by a single extent record.
pub const EXTENT_DATA_MAX: u64 = 4 * 1024 * 1024;
/// Superblock magic at offset 0 of each superblock copy.
pub const SB_MAGIC: &[u8; 8] = b"PORFS_SB";
/// Extent header magic (`"EXT1"` little-endian).
pub const EXT_MAGIC: u32 = 0x4558_5431;
/// On-disk format version.
pub const FORMAT_VERSION: u32 = 1;

/// Byte offset of superblock copy A (the authoritative copy).
pub const SB_A_OFFSET: u64 = 0;
/// Byte offset of superblock copy B (written first during sync).
pub const SB_B_OFFSET: u64 = BLOCK_SIZE;
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
    /// Byte slice has the wrong length for the structure being decoded.
    #[error("truncated structure: need {need} bytes, got {got}")]
    Truncated {
        /// Required length in bytes.
        need: usize,
        /// Actual length in bytes.
        got: usize,
    },
}
