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

/// Maximum length of a symbolic-link target in bytes (Linux `PATH_MAX - 1`).
pub const MAX_SYMLINK_LEN: usize = 4095;

/// Maximum length of an extended-attribute name in bytes (Linux `XATTR_NAME_MAX`).
pub const MAX_XATTR_NAME_LEN: usize = 255;

/// Maximum size of an extended-attribute value in bytes (Linux `XATTR_SIZE_MAX`).
pub const MAX_XATTR_VALUE_LEN: usize = 64 * 1024;

/// `setxattr` flag: create-only — fail with `EEXIST` when the attribute
/// already exists (Linux `XATTR_CREATE`).
pub const XATTR_CREATE: u32 = 1;

/// `setxattr` flag: replace-only — fail with `ENODATA` when the attribute
/// does not exist (Linux `XATTR_REPLACE`).
pub const XATTR_REPLACE: u32 = 2;

/// Kind of an inode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeKind {
    /// Regular file; payload lives in the extent store via `file_extents`.
    File,
    /// Directory; children live in `dir_entries`.
    Dir,
    /// Symbolic link; the target is stored in the inode record.
    Symlink,
    /// FIFO (named pipe); no payload.
    Fifo,
    /// Unix-domain socket node; no payload.
    Socket,
    /// Character device; `rdev` lives in the inode record.
    Chr,
    /// Block device; `rdev` lives in the inode record.
    Blk,
}

impl NodeKind {
    /// True for every non-directory kind (the POSIX rename/unlink rules
    /// treat them uniformly).
    pub(crate) fn is_dir(self) -> bool {
        self == NodeKind::Dir
    }
}

/// Public attributes of an inode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InodeAttr {
    /// Inode number.
    pub ino: Ino,
    /// Node kind.
    pub kind: NodeKind,
    /// Permission bits.
    pub mode: u32,
    /// Owner user id.
    pub uid: u32,
    /// Owner group id.
    pub gid: u32,
    /// Logical size in bytes (symlinks report the target length, directories
    /// and special nodes report 0).
    pub size: u64,
    /// Access time as `(secs, nsecs)` since the epoch.
    pub atime: (i64, u32),
    /// Modification time as `(secs, nsecs)` since the epoch.
    pub mtime: (i64, u32),
    /// Change time as `(secs, nsecs)` since the epoch.
    pub ctime: (i64, u32),
    /// Hard-link count (directories always report 1).
    pub nlink: u32,
    /// Device id for [`NodeKind::Chr`]/[`NodeKind::Blk`], 0 otherwise.
    pub rdev: u32,
}

/// Field-optional attribute update for [`crate::Mds::setattr`].
/// Attribute bundle for node creation ([`crate::Mds::create`],
/// [`crate::Mds::mkdir`], [`crate::Mds::mknod`]): one struct so a future
/// RPC request maps to a single field.
#[derive(Debug, Clone, Copy)]
pub struct NodeSpec {
    /// Node kind (File/Dir via create/mkdir; Fifo/Socket/Chr/Blk via mknod).
    pub kind: NodeKind,
    /// Permission bits.
    pub mode: u32,
    /// Device id for Chr/Blk, 0 otherwise.
    pub rdev: u32,
    /// Owner user id.
    pub uid: u32,
    /// Owner group id.
    pub gid: u32,
}

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
    /// New logical size (truncate; regular files only).
    pub size: Option<u64>,
    /// New access time.
    pub atime: Option<(i64, u32)>,
    /// New modification time.
    pub mtime: Option<(i64, u32)>,
}

/// One entry of a streaming directory listing ([`crate::Mds::readdir_batch`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirEntry {
    /// Entry name.
    pub name: String,
    /// Child inode number.
    pub ino: Ino,
    /// Child node kind.
    pub kind: NodeKind,
    /// The raw `dir_entries` key of this entry: a paginating consumer
    /// checkpoints it after every entry handed downstream, so a resume
    /// never repeats or skips.
    pub cookie: Vec<u8>,
}

/// One batch of a streaming directory listing ([`crate::Mds::readdir_batch`]).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReaddirBatch {
    /// Up to the requested number of entries, in hash order.
    pub entries: Vec<DirEntry>,
    /// `None` means the directory is exhausted. (The resume token itself
    /// lives on each [`DirEntry::cookie`]: consumers checkpoint per entry,
    /// not per batch.)
    pub next_cookie: Option<Vec<u8>>,
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
    /// Number of extended-attribute rows checked.
    pub xattrs: u64,
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
    /// Symlink target (`Symlink` only), `None` for every other kind.
    pub(crate) link_target: Option<Vec<u8>>,
    /// Device id (`Chr`/`Blk` only), 0 for every other kind.
    pub(crate) rdev: u32,
}

impl InodeRec {
    /// Freshly created inode of `kind` with `mode`, all times set to `now`.
    pub(crate) fn new(kind: NodeKind, mode: u32, uid: u32, gid: u32, now: (i64, u32)) -> Self {
        Self {
            kind,
            mode,
            uid,
            gid,
            size: 0,
            atime: now,
            mtime: now,
            ctime: now,
            nlink: 1,
            link_target: None,
            rdev: 0,
        }
    }

    /// Freshly created symlink inode: mode 0o777, size = target length.
    pub(crate) fn new_symlink(target: &[u8], uid: u32, gid: u32, now: (i64, u32)) -> Self {
        let mut rec = Self::new(NodeKind::Symlink, 0o777, uid, gid, now);
        rec.size = target.len() as u64;
        rec.link_target = Some(target.to_vec());
        rec
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
            rdev: self.rdev,
        }
    }
}
