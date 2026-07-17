//! Mechanical mappings between the MDS types and the FUSE wire types:
//! attributes, timestamps, node kinds, and the errno table (the single
//! place where MDS errors become POSIX errnos).

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use fuser::{Errno, FileAttr, FileType, INodeNo};

use porfs_mds::{InodeAttr, MdsError, NodeKind};

/// MDS error -> POSIX errno. `Store`/`Db`/`Corrupt` collapse to EIO: the
/// kernel has no richer vocabulary for "our backing store failed".
pub(crate) fn errno(err: &MdsError) -> Errno {
    match err {
        MdsError::NotFound(_) => Errno::ENOENT,
        MdsError::NotDir(_) => Errno::ENOTDIR,
        MdsError::IsDir(_) => Errno::EISDIR,
        MdsError::NotEmpty(_) => Errno::ENOTEMPTY,
        MdsError::Exists(_) => Errno::EEXIST,
        MdsError::InvalidName(_) => Errno::EINVAL,
        MdsError::Store(_) | MdsError::Db(_) | MdsError::Corrupt(_) => Errno::EIO,
    }
}

/// MDS `(secs, nsecs)` -> `SystemTime` (epoch-relative; pre-epoch times
/// clamp to the epoch — the MDS never produces them in v0).
pub(crate) fn to_system_time((secs, nsecs): (i64, u32)) -> SystemTime {
    if secs >= 0 {
        UNIX_EPOCH + Duration::new(secs as u64, nsecs)
    } else {
        UNIX_EPOCH - Duration::new(secs.unsigned_abs(), nsecs)
    }
}

/// `SystemTime` -> MDS `(secs, nsecs)`.
pub(crate) fn from_system_time(time: SystemTime) -> (i64, u32) {
    match time.duration_since(UNIX_EPOCH) {
        Ok(d) => (d.as_secs() as i64, d.subsec_nanos()),
        Err(err) => {
            let d = err.duration();
            (-(d.as_secs() as i64), d.subsec_nanos())
        }
    }
}

/// MDS node kind -> FUSE file type.
pub(crate) fn file_type(kind: NodeKind) -> FileType {
    match kind {
        NodeKind::File => FileType::RegularFile,
        NodeKind::Dir => FileType::Directory,
    }
}

/// MDS inode attributes -> FUSE wire attributes.
pub(crate) fn file_attr(attr: &InodeAttr) -> FileAttr {
    FileAttr {
        ino: INodeNo(attr.ino),
        size: attr.size,
        blocks: attr.size.div_ceil(512),
        atime: to_system_time(attr.atime),
        mtime: to_system_time(attr.mtime),
        ctime: to_system_time(attr.ctime),
        crtime: UNIX_EPOCH, // v0 tracks no birth time
        kind: file_type(attr.kind),
        perm: (attr.mode & 0o7777) as u16,
        nlink: attr.nlink,
        uid: attr.uid,
        gid: attr.gid,
        rdev: 0,
        blksize: 4096,
        flags: 0,
    }
}

/// Permission bits from a mode the kernel handed us (strip file-type bits).
pub(crate) fn perm_bits(mode: u32) -> u32 {
    mode & 0o7777
}
