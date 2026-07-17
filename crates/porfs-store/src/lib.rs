//! PolarisFS append-only extent store (on-disk format v0) on io_uring.
//!
//! The store owns its device file and an io_uring ring (no async runtime).
//! Writes are self-contained 4KiB-aligned blocks appended at the tail; extents
//! become *confirmed* only after [`ExtentStore::sync`]. On [`ExtentStore::open`]
//! the log is scanned from `DATA_START` and truncated at the first bad record
//! (salvage semantics: anything past the first bad record is treated as an
//! unconfirmed write and dropped). See `docs/format.md` for the contract.

mod aligned;
mod engine;
mod ops;
mod scan;
mod store;

pub use store::{DEFAULT_QUEUE_DEPTH, ExtentStore};

/// Extent identifier: monotonically increasing per device, assigned by the store.
/// A tombstone record shares the id of the extent it deletes.
pub type ExtentId = u64;

/// Probe whether io_uring can be used in this environment (kernel + seccomp).
///
/// When this returns `false` the store silently falls back to plain
/// `pread`/`pwrite` syscalls; tests use it to skip io_uring-specific coverage.
pub fn io_uring_available() -> bool {
    engine::io_uring_available()
}

/// Errors returned by the extent store.
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    /// Underlying I/O failure.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    /// On-disk structure failed to parse or validate.
    #[error("format error: {0}")]
    Format(#[from] porfs_format::FormatError),
    /// Neither superblock copy passed validation.
    #[error("no valid superblock found")]
    NoValidSuperblock,
    /// Device size is below the minimum or cannot be determined.
    #[error("invalid device size {size} bytes (need at least {min})")]
    InvalidDeviceSize {
        /// Actual/requested size in bytes.
        size: u64,
        /// Required minimum in bytes.
        min: u64,
    },
    /// Payload exceeds `EXTENT_DATA_MAX`.
    #[error("extent data too large: {len} bytes (max {max})")]
    DataTooLarge {
        /// Offered payload length.
        len: usize,
        /// Maximum payload length.
        max: usize,
    },
    /// No extent (live or tombstoned) with this id exists.
    #[error("unknown extent id {0}")]
    UnknownExtent(ExtentId),
    /// The extent has been logically deleted.
    #[error("extent id {0} is tombstoned")]
    Tombstoned(ExtentId),
    /// Data CRC32C mismatch on read (silent corruption detected).
    #[error("extent id {0} data crc32c mismatch")]
    CorruptData(ExtentId),
    /// On-disk record no longer matches the in-memory index entry.
    #[error("extent id {0} header mismatch on read")]
    CorruptHeader(ExtentId),
    /// The append would exceed the device size.
    #[error("device full: need {need} bytes at tail {tail}, device size {size}")]
    DeviceFull {
        /// Bytes requested.
        need: u64,
        /// Current tail offset.
        tail: u64,
        /// Device size in bytes.
        size: u64,
    },
    /// A completed I/O transferred fewer bytes than requested.
    #[error("short io: expected {expected} bytes, got {got}")]
    ShortIo {
        /// Expected transfer length.
        expected: usize,
        /// Actual transfer length.
        got: usize,
    },
    /// `read_batch_into` was called with a `dsts` slice of the wrong length.
    #[error("dsts length {dsts} does not match ids length {ids}")]
    LengthMismatch {
        /// Number of extent ids requested.
        ids: usize,
        /// Number of destination buffers provided.
        dsts: usize,
    },
    /// A destination buffer's capacity is too small for the extent payload.
    #[error("extent id {id} payload {need} bytes exceeds dst capacity {have}")]
    CapacityTooSmall {
        /// The offending extent id.
        id: ExtentId,
        /// Payload bytes required.
        need: usize,
        /// Destination capacity available.
        have: usize,
    },
    /// An SQE could not be pushed onto an empty submission queue.
    #[error("io_uring submission failed")]
    SubmissionFailed,
    /// A violated internal invariant (a bug, not user error).
    #[error("internal error: {0}")]
    Internal(&'static str),
}
