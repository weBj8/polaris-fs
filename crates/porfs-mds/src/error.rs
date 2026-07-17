//! The MDS error type: POSIX-flavored namespace errors plus the two
//! subsystems the MDS is built on (extent store, redb) and `Corrupt` for
//! on-disk metadata that fails a consistency check.

use porfs_store::StoreError;

/// Errors returned by [`crate::Mds`] operations.
#[derive(Debug, thiserror::Error)]
pub enum MdsError {
    /// The named entry or inode does not exist.
    #[error("not found: {0}")]
    NotFound(String),
    /// The operation requires a directory (parent or target is not one).
    #[error("not a directory: {0}")]
    NotDir(String),
    /// The operation requires a non-directory but the target is a directory.
    #[error("is a directory: {0}")]
    IsDir(String),
    /// `rmdir` or `rename` target directory still has entries.
    #[error("directory not empty: {0}")]
    NotEmpty(String),
    /// The name already exists in the target directory.
    #[error("already exists: {0}")]
    Exists(String),
    /// Empty, longer than 255 bytes, contains `/`, or is `.` / `..`.
    #[error("invalid name: {0:?}")]
    InvalidName(String),
    /// Extent-store failure.
    #[error("store error: {0}")]
    Store(#[from] StoreError),
    /// Metadata-database (redb) failure.
    #[error("metadata db error: {0}")]
    Db(#[from] redb::Error),
    /// On-disk metadata failed a consistency check.
    #[error("corrupt metadata: {0}")]
    Corrupt(String),
}

/// Result alias for MDS operations.
pub type Result<T> = std::result::Result<T, MdsError>;

/// Convert any concrete redb error (`StorageError`, `TableError`,
/// `TransactionError`, `CommitError`, `DatabaseError`, ...) into
/// [`MdsError::Db`] via the umbrella `redb::Error`.
pub(crate) fn dberr<E: Into<redb::Error>>(err: E) -> MdsError {
    MdsError::Db(err.into())
}
