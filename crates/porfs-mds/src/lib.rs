//! PolarisFS metadata server v0 (library form, single node).
//!
//! One [`Mds`] instance = a **redb database** holding all metadata (ACID,
//! one write transaction per namespace mutation) plus a **data plane** for
//! file payloads: a local extent-store device (`porfs-store`) in local
//! mode, or a cluster of chunkservers (wire protocol v3, `porfs-rpc`) in
//! cluster mode.
//!
//! Metadata schema v2 (all integers inside keys are big-endian so byte
//! order equals numeric order — redb range scans depend on it):
//!
//! | table             | key                                          | value                          |
//! |-------------------|----------------------------------------------|--------------------------------|
//! | `inodes`          | ino (u64 BE)                                 | bincode inode record           |
//! | `dir_entries`     | parent ino BE ++ xxhash64(name) BE ++ name   | child ino (u64 BE)             |
//! | `file_extents_v2` | ino BE ++ logical offset BE                  | extent ref codec ++ length     |
//! | `xattrs`          | ino BE ++ attribute name bytes               | value bytes (<= 64 KiB)        |
//! | `meta`            | `"next_ino"` / `"schema_version"` / ...      | counters                       |
//! | `cluster_config`  | `"membership"`                               | bincode-2 `Vec<MemberSpec>`    |
//!
//! `file_extents_v2` is an interval map: row `(off) -> (ref, len)` covers
//! `[off, off + len)` of the file; rows of one file never overlap and holes
//! read back as zeros. The value is length-discriminated: 16 bytes locates
//! a local extent (`id BE ++ len BE`, byte-identical to the schema-1
//! `file_extents` encoding, which the open path migrates verbatim); 32
//! bytes locates cluster copies (primary server+extent, optional replica
//! server+extent, len — all little-endian). `dir_entries` is hash-ordered
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
//! - [`Mds::fsync`] confirms appended extents (store group commit; cluster
//!   mode broadcasts a sync to every chunkserver and requires all to
//!   succeed). After it returns, namespace + data both survive a crash.
//! - A crash without `fsync` can leave map rows pointing at extents the
//!   store salvaged away on remount; [`Mds::open`] reconciles those rows
//!   into holes before serving (see [`Mds::last_reconcile_repairs`]).
//!   Cluster mode deletes a row ONLY when every copy answers `NotFound`
//!   from a reachable, UUID-verified server — an unreachable or
//!   misidentified server fails the mount instead.

mod check;
mod data;
mod error;
mod keys;
mod mds;
mod namespace;
mod plane;
mod types;
mod xattr;

pub use error::{MdsError, Result};
pub use mds::Mds;
pub use plane::ClusterTimeouts;
pub use types::{
    CheckReport, DirEntry, Ino, InodeAttr, MAX_NAME_LEN, MAX_SYMLINK_LEN, MAX_XATTR_NAME_LEN,
    MAX_XATTR_VALUE_LEN, NodeKind, NodeSpec, ROOT_INO, ReaddirBatch, SetAttr, Statfs, XATTR_CREATE,
    XATTR_REPLACE,
};
