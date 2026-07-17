//! Public value types of the MDS API plus the serialized inode record.
//!
//! Timestamps are `(secs, nsecs)` pairs relative to the Unix epoch, matching
//! the granularity POSIX `stat` exposes.

use serde::{Deserialize, Serialize};

/// Inode number. The root directory is always [`ROOT_INO`].
pub type Ino = u64;

/// Inode number of the root directory, created by [`crate::Mds::format`].
pub const ROOT_INO: Ino = 1;

/// Maximum file-name length in bytes (POSIX `NAME_MAX`).
pub const MAX_NAME_LEN: usize = 255;

/// Kind of an inode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeKind {
    /// Regular file; payload lives in the extent store via `file_extents`.
    File,
    /// Directory; children live in `dir_entries`.
    Dir,
}

/// Public attributes of an inode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InodeAttr {
    /// Inode number.
    pub ino: Ino,
    /// File or directory.
    pub kind: NodeKind,
    /// Permission bits.
    pub mode: u32,
    /// Owner user id.
    pub uid: u32,
    /// Owner group id.
    pub gid: u32,
    /// Logical size in bytes (directories report 0 in v0).
    pub size: u64,
    /// Access time as `(secs, nsecs)` since the epoch.
    pub atime: (i64, u32),
    /// Modification time as `(secs, nsecs)` since the epoch.
    pub mtime: (i64, u32),
    /// Change time as `(secs, nsecs)` since the epoch.
    pub ctime: (i64, u32),
    /// Hard-link count (directories always report 1 in v0).
    pub nlink: u32,
}

/// Field-optional attribute update for [`crate::Mds::setattr`].
///
/// `size` shrinks or grows the file (truncate semantics; growth reads back
/// as zeros). The rest are plain field assignments; `ctime` is always
/// refreshed by the call.
#[derive(Debug, Clone, Default)]
pub struct SetAttr {
    /// New permission bits.
    pub mode: Option<u32>,
    /// New owner user id.
    pub uid: Option<u32>,
    /// New owner group id.
    pub gid: Option<u32>,
    /// New logical size (truncate).
    pub size: Option<u64>,
    /// New access time.
    pub atime: Option<(i64, u32)>,
    /// New modification time.
    pub mtime: Option<(i64, u32)>,
}

/// Counters produced by [`crate::Mds::self_check`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CheckReport {
    /// Number of inode rows checked.
    pub inodes: u64,
    /// Number of directory entries checked.
    pub entries: u64,
    /// Number of file-extent map rows checked.
    pub extents: u64,
    /// Dangling extent-map rows removed by the mount-time reconcile that ran
    /// inside [`crate::Mds::open`] (self_check itself repairs nothing).
    pub orphans_repaired: u64,
}

/// Storage and namespace totals reported by [`crate::Mds::statfs`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Statfs {
    /// Total device bytes.
    pub total_bytes: u64,
    /// Bytes still appendable in the extent-log region (device size minus
    /// the log tail; tombstoned space is not reclaimed until P27's GC).
    pub free_bytes: u64,
    /// Sum of payload bytes over live extents.
    pub live_bytes: u64,
    /// Number of inode rows.
    pub inodes: u64,
    /// Number of live extents in the store.
    pub extents: u64,
}

/// Serialized form of an inode (bincode, stored in the `inodes` table).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct InodeRec {
    pub(crate) kind: NodeKind,
    pub(crate) mode: u32,
    pub(crate) uid: u32,
    pub(crate) gid: u32,
    pub(crate) size: u64,
    pub(crate) atime: (i64, u32),
    pub(crate) mtime: (i64, u32),
    pub(crate) ctime: (i64, u32),
    pub(crate) nlink: u32,
}

impl InodeRec {
    /// Freshly created inode of `kind` with `mode`, all times set to `now`.
    pub(crate) fn new(kind: NodeKind, mode: u32, now: (i64, u32)) -> Self {
        Self {
            kind,
            mode,
            uid: 0,
            gid: 0,
            size: 0,
            atime: now,
            mtime: now,
            ctime: now,
            nlink: 1,
        }
    }

    /// Project to the public attribute view.
    pub(crate) fn attr(&self, ino: Ino) -> InodeAttr {
        InodeAttr {
            ino,
            kind: self.kind,
            mode: self.mode,
            uid: self.uid,
            gid: self.gid,
            size: self.size,
            atime: self.atime,
            mtime: self.mtime,
            ctime: self.ctime,
            nlink: self.nlink,
        }
    }
}
