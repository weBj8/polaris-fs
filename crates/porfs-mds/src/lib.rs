//! PolarisFS metadata server v0 (library form, single node).
//!
//! One [`Mds`] instance = two files: a **redb database** holding all
//! metadata (ACID, one write transaction per namespace mutation) and an
//! **extent-store device** (`porfs-store`) holding file payloads.
//!
//! Metadata schema (all integers inside keys are big-endian so byte order
//! equals numeric order — redb range scans depend on it):
//!
//! | table          | key                                          | value                          |
//! |----------------|----------------------------------------------|--------------------------------|
//! | `inodes`       | ino (u64 BE)                                 | bincode inode record           |
//! | `dir_entries`  | parent ino BE ++ xxhash64(name) BE ++ name   | child ino (u64 BE)             |
//! | `file_extents` | ino BE ++ logical offset BE                  | extent id BE ++ length BE      |
//! | `xattrs`       | ino BE ++ attribute name bytes               | value bytes (<= 64 KiB)        |
//! | `meta`         | `"next_ino"` / `"schema_version"`            | counters                       |
//!
//! `file_extents` is an interval map: row `(off) -> (id, len)` covers
//! `[off, off + len)` of the file with extent `id`; rows of one file never
//! overlap and holes read back as zeros. `dir_entries` is hash-ordered
//! within a directory (the GPFS extensible-hashing principle; P7+ shards
//! hash ranges across chunkservers), so `readdir` returns entries in hash
//! order — POSIX allows any order.
//!
//! Durability contract:
//! - Namespace mutations commit as one redb transaction (durable per commit),
//!   so `rename` is atomic.
//! - File data is log-structured COW: `write` appends new extents **first**
//!   and commits the map **second**; replaced extents are tombstoned after
//!   the commit. No fdatasync happens per write.
//! - [`Mds::fsync`] confirms appended extents (store group commit). After it
//!   returns, namespace + data both survive a crash.
//! - A crash without `fsync` can leave map rows pointing at extents the
//!   store salvaged away on remount; [`Mds::open`] reconciles those rows
//!   into holes before serving (see [`Mds::last_reconcile_repairs`]).

mod check;
mod data;
mod error;
mod keys;
mod mds;
mod namespace;
mod types;
mod xattr;

pub use error::{MdsError, Result};
pub use mds::Mds;
pub use types::{
    CheckReport, DirEntry, Ino, InodeAttr, MAX_NAME_LEN, MAX_SYMLINK_LEN, MAX_XATTR_NAME_LEN,
    MAX_XATTR_VALUE_LEN, NodeKind, NodeSpec, ROOT_INO, ReaddirBatch, SetAttr, Statfs, XATTR_CREATE,
    XATTR_REPLACE,
};
