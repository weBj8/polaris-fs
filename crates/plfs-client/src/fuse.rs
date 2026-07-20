//! FUSE daemon (design doc §8): POSIX over the client core. The `!Send`
//! core (arena io_uring) is owned by a dedicated worker thread with a
//! current-thread tokio runtime; fuser callbacks ship jobs to it over a
//! channel — the same seam as plfs-data's arena actor.
//!
//! Semantics notes:
//! - Regular-file opens use `FOPEN_DIRECT_IO`: reads always reach the
//!   client core (pending overlay or arena), never a stale page cache.
//!   Read-only handles get `FOPEN_NOFLUSH` so a reader's close stays
//!   instant and error-free; write handles keep the close-flush barrier.
//! - Writes always route through the WAL (design doc §8.2: O_DIRECT is
//!   accepted but never bypasses durability — it can't, there is no
//!   other path).
//! - Errors map to POSIX errnos in one place (`errno`).

use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::sync::mpsc::{self, Sender};
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use fuser::{
    BsdFileFlags, Errno, FileAttr, FileHandle, FileType, Filesystem, FopenFlags, Generation,
    INodeNo, LockOwner, MountOption, OpenAccMode, OpenFlags, RenameFlags, ReplyAttr, ReplyCreate,
    ReplyData, ReplyDirectory, ReplyEmpty, ReplyEntry, ReplyOpen, ReplyStatfs, ReplyWrite, Request,
    TimeOrNow, WriteFlags,
};
use plfs_meta::{Inode, Kind, MetaError, MetaOp, OpResult};

use crate::core::{ClientCore, ClientError};

const TTL: Duration = Duration::from_secs(1);
const GENERATION: Generation = Generation(0);

/// Mount errors.
#[derive(Debug, thiserror::Error)]
pub enum FuseError {
    /// Client-core boot failure.
    #[error("client: {0}")]
    Client(#[from] ClientError),
    /// Mount syscall / session failure.
    #[error("mount: {0}")]
    Mount(#[from] std::io::Error),
}

type Job = Box<dyn FnOnce(&mut ClientCore, &tokio::runtime::Runtime) + Send>;

/// Owns the client core on a dedicated thread (the `!Send` arena never
/// crosses it). Dropping closes the job channel and joins the thread, so
/// the redb lock and arena file are fully released before drop returns.
struct Worker {
    tx: Option<Sender<Job>>,
    handle: Option<JoinHandle<()>>,
}

impl Worker {
    fn spawn<F>(open: impl FnOnce() -> F + Send + 'static) -> Result<Self, ClientError>
    where
        F: std::future::Future<Output = Result<ClientCore, ClientError>> + 'static,
    {
        let (job_tx, job_rx) = mpsc::channel::<Job>();
        let (init_tx, init_rx) = mpsc::channel::<Result<(), ClientError>>();
        let handle = std::thread::Builder::new()
            .name("plfs-fuse-worker".into())
            .spawn(move || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("worker runtime");
                let mut core = match rt.block_on(open()) {
                    Ok(core) => {
                        let _ = init_tx.send(Ok(()));
                        core
                    }
                    Err(err) => {
                        let _ = init_tx.send(Err(err));
                        return;
                    }
                };
                for job in job_rx {
                    job(&mut core, &rt);
                }
                // Job channel closed: clean shutdown of the raft core so
                // the redb handle frees before the thread exits.
                let _ = rt.block_on(core.shutdown());
            })
            .expect("spawn fuse worker");
        init_rx
            .recv()
            .map_err(|_| ClientError::Codec("fuse worker died during open".into()))??;
        Ok(Self {
            tx: Some(job_tx),
            handle: Some(handle),
        })
    }

    fn call<R: Send + 'static>(
        &self,
        f: impl FnOnce(&mut ClientCore, &tokio::runtime::Runtime) -> R + Send + 'static,
    ) -> Option<R> {
        let (reply_tx, reply_rx) = mpsc::channel::<R>();
        let job = Box::new(move |core: &mut ClientCore, rt: &tokio::runtime::Runtime| {
            let _ = reply_tx.send(f(core, rt));
        });
        self.tx.as_ref()?.send(job).ok()?;
        reply_rx.recv().ok()
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.tx.take();
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

fn errno(err: &ClientError) -> Errno {
    match err {
        ClientError::NotFound => Errno::ENOENT,
        ClientError::Meta(MetaError::NotFound) => Errno::ENOENT,
        ClientError::Meta(MetaError::Exists) => Errno::EEXIST,
        ClientError::Meta(MetaError::NotEmpty) => Errno::ENOTEMPTY,
        ClientError::Meta(MetaError::NotDir) => Errno::ENOTDIR,
        ClientError::Meta(MetaError::IsDir) => Errno::EISDIR,
        ClientError::Meta(MetaError::Invalid(_)) => Errno::EINVAL,
        ClientError::Arena(plfs_arena::ArenaError::OutOfSpace { .. }) => Errno::ENOSPC,
        ClientError::Arena(plfs_arena::ArenaError::Oversize { .. }) => Errno::EFBIG,
        _ => Errno::EIO,
    }
}

fn to_system_time((secs, nsecs): (i64, u32)) -> SystemTime {
    if secs >= 0 {
        UNIX_EPOCH + Duration::new(secs as u64, nsecs)
    } else {
        UNIX_EPOCH - Duration::new(secs.unsigned_abs(), nsecs)
    }
}

fn from_system_time(time: SystemTime) -> (i64, u32) {
    match time.duration_since(UNIX_EPOCH) {
        Ok(d) => (d.as_secs() as i64, d.subsec_nanos()),
        Err(err) => {
            let d = err.duration();
            (-(d.as_secs() as i64), d.subsec_nanos())
        }
    }
}

fn resolve_time(t: TimeOrNow) -> (i64, u32) {
    match t {
        TimeOrNow::SpecificTime(t) => from_system_time(t),
        TimeOrNow::Now => from_system_time(SystemTime::now()),
    }
}

fn file_attr(inode: &Inode) -> FileAttr {
    let kind = match inode.kind {
        Kind::File => FileType::RegularFile,
        Kind::Dir => FileType::Directory,
        Kind::Symlink => FileType::Symlink,
    };
    FileAttr {
        ino: INodeNo(0), // filled by the caller
        size: inode.size,
        blocks: inode.size.div_ceil(512),
        atime: to_system_time(inode.atime),
        mtime: to_system_time(inode.mtime),
        ctime: to_system_time(inode.ctime),
        crtime: to_system_time(inode.ctime),
        kind,
        perm: (inode.mode & 0o7777) as u16,
        nlink: inode.nlink,
        uid: inode.uid,
        gid: inode.gid,
        rdev: 0,
        blksize: crate::core::CHUNK_SIZE as u32,
        flags: 0,
    }
}

fn with_ino(mut attr: FileAttr, ino: u64) -> FileAttr {
    attr.ino = INodeNo(ino);
    attr
}

fn name_string(name: &OsStr) -> Result<String, Errno> {
    name.to_str().map(str::to_owned).ok_or(Errno::EINVAL)
}

/// The fuser filesystem: stateless, every call is a job to the worker.
pub struct PlfsFs {
    worker: Worker,
}

impl PlfsFs {
    /// Open the volume at `dir` on the worker thread.
    pub fn open(dir: &Path) -> Result<Self, ClientError> {
        let dir = dir.to_path_buf();
        Worker::spawn(move || ClientCore::open(dir)).map(|worker| Self { worker })
    }

    /// Open with an explicit data-plane sink (standalone: loopback gRPC).
    pub fn open_with_sink(dir: &Path, sink: crate::core::SinkConfig) -> Result<Self, ClientError> {
        let dir = dir.to_path_buf();
        Worker::spawn(move || ClientCore::open_with_sink(dir, sink)).map(|worker| Self { worker })
    }

    /// Create a fresh volume at `dir` and mount it.
    pub fn create(dir: &Path, arena_bytes: u64) -> Result<Self, ClientError> {
        let dir = dir.to_path_buf();
        Worker::spawn(move || ClientCore::create(dir, arena_bytes)).map(|worker| Self { worker })
    }

    /// Create a fresh volume with an explicit data-plane sink.
    pub fn create_with_sink(
        dir: &Path,
        arena_bytes: u64,
        sink: crate::core::SinkConfig,
    ) -> Result<Self, ClientError> {
        let dir = dir.to_path_buf();
        Worker::spawn(move || ClientCore::create_with_sink(dir, arena_bytes, sink))
            .map(|worker| Self { worker })
    }

    fn call<R: Send + 'static>(
        &self,
        f: impl FnOnce(&mut ClientCore, &tokio::runtime::Runtime) -> R + Send + 'static,
    ) -> Result<R, Errno> {
        self.worker.call(f).ok_or(Errno::EIO)
    }

    fn attr_of(core: &ClientCore, ino: u64) -> Result<FileAttr, ClientError> {
        let inode = core.state().getattr(ino)?.ok_or(ClientError::NotFound)?;
        Ok(with_ino(file_attr(&inode), ino))
    }
}

/// `.snapshots` virtual-tree inode namespacing: live inos start at 1 and
/// stay below 2^40; snapshot-space inos set the top bits so handlers can
/// tell the two trees apart.
const SNAP_ROOT: u64 = u64::MAX;
const SNAP_DIR_BIT: u64 = 1 << 63;
const SNAP_CONTENT_BIT: u64 = 1 << 62;

enum SnapIno {
    Root,
    Dir(u64),
    Content(u64, u64),
}

fn snap_dir_ino(snap: u64) -> u64 {
    SNAP_DIR_BIT | snap
}

fn snap_content_ino(snap: u64, ino: u64) -> u64 {
    SNAP_CONTENT_BIT | (snap << 40) | ino
}

fn decode_snap(ino: u64) -> Option<SnapIno> {
    if ino == SNAP_ROOT {
        Some(SnapIno::Root)
    } else if ino & SNAP_DIR_BIT != 0 {
        Some(SnapIno::Dir(ino & !SNAP_DIR_BIT))
    } else if ino & SNAP_CONTENT_BIT != 0 {
        Some(SnapIno::Content(
            (ino >> 40) & 0x3f_ffff,
            ino & 0xff_ffff_ffff,
        ))
    } else {
        None
    }
}

fn snap_dir_attr(ino: u64) -> FileAttr {
    with_ino(
        file_attr(&plfs_meta::Inode {
            kind: plfs_meta::Kind::Dir,
            mode: 0o555,
            uid: 0,
            gid: 0,
            size: 0,
            atime: (0, 0),
            mtime: (0, 0),
            ctime: (0, 0),
            nlink: 2,
            symlink_target: None,
        }),
        ino,
    )
}

fn snap_listdir_typed(
    core: &ClientCore,
    snap: u64,
    dir: u64,
) -> Result<Vec<(u64, OsString, FileType)>, ClientError> {
    let mut out = Vec::new();
    for (name, child) in core.snapshot_listdir(snap, dir)? {
        let kind = match core.snapshot_getattr(snap, child)? {
            Some(inode) => match inode.kind {
                plfs_meta::Kind::Dir => FileType::Directory,
                plfs_meta::Kind::Symlink => FileType::Symlink,
                plfs_meta::Kind::File => FileType::RegularFile,
            },
            None => FileType::RegularFile,
        };
        out.push((snap_content_ino(snap, child), OsString::from(name), kind));
    }
    Ok(out)
}

impl Filesystem for PlfsFs {
    fn lookup(&self, _req: &Request, parent: INodeNo, name: &OsStr, reply: ReplyEntry) {
        let Ok(name) = name_string(name) else {
            reply.error(Errno::EINVAL);
            return;
        };
        if parent.0 == plfs_meta::ROOT_INO && name == ".snapshots" {
            reply.entry(&TTL, &snap_dir_attr(SNAP_ROOT), GENERATION);
            return;
        }
        if let Some(snap_parent) = decode_snap(parent.0) {
            let r = self.call(move |core, _| {
                let (snap, dir) = match snap_parent {
                    SnapIno::Root => {
                        let snaps = core.snapshot_list()?;
                        let Some(s) = snaps.iter().find(|s| s.name == name) else {
                            return Err(ClientError::NotFound);
                        };
                        return Ok(snap_dir_attr(snap_dir_ino(s.id)));
                    }
                    SnapIno::Dir(snap) => (snap, plfs_meta::ROOT_INO),
                    SnapIno::Content(snap, ino) => (snap, ino),
                };
                let Some(child) = core.snapshot_lookup(snap, dir, &name)? else {
                    return Err(ClientError::NotFound);
                };
                let inode = core
                    .snapshot_getattr(snap, child)?
                    .ok_or(ClientError::NotFound)?;
                Ok(with_ino(file_attr(&inode), snap_content_ino(snap, child)))
            });
            match r {
                Ok(Ok(attr)) => reply.entry(&TTL, &attr, GENERATION),
                Ok(Err(err)) => reply.error(errno(&err)),
                Err(err) => reply.error(err),
            }
            return;
        }
        let r = self.call(move |core, _| {
            let Some(ino) = core.state().lookup(parent.0, &name)? else {
                return Err(ClientError::NotFound);
            };
            Self::attr_of(core, ino)
        });
        match r {
            Ok(Ok(attr)) => reply.entry(&TTL, &attr, GENERATION),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn getattr(&self, _req: &Request, ino: INodeNo, _fh: Option<FileHandle>, reply: ReplyAttr) {
        match decode_snap(ino.0) {
            Some(SnapIno::Root | SnapIno::Dir(_)) => {
                reply.attr(&TTL, &snap_dir_attr(ino.0));
                return;
            }
            Some(SnapIno::Content(snap, real)) => {
                let r = self.call(move |core, _| {
                    let inode = core
                        .snapshot_getattr(snap, real)?
                        .ok_or(ClientError::NotFound)?;
                    Ok(with_ino(file_attr(&inode), ino.0))
                });
                match r {
                    Ok(Ok(attr)) => reply.attr(&TTL, &attr),
                    Ok(Err(err)) => reply.error(errno(&err)),
                    Err(err) => reply.error(err),
                }
                return;
            }
            None => {}
        }
        let r = self.call(move |core, _| Self::attr_of(core, ino.0));
        match r {
            Ok(Ok(attr)) => reply.attr(&TTL, &attr),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn setattr(
        &self,
        _req: &Request,
        ino: INodeNo,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<TimeOrNow>,
        mtime: Option<TimeOrNow>,
        _ctime: Option<SystemTime>,
        _fh: Option<FileHandle>,
        _crtime: Option<SystemTime>,
        _chgtime: Option<SystemTime>,
        _bkuptime: Option<SystemTime>,
        _flags: Option<BsdFileFlags>,
        reply: ReplyAttr,
    ) {
        let op = MetaOp::SetAttr {
            ino: ino.0,
            size,
            mode,
            uid,
            gid,
            atime: atime.map(resolve_time),
            mtime: mtime.map(resolve_time),
        };
        let r = self.call(move |core, rt| {
            rt.block_on(core.meta_op(op))??;
            Self::attr_of(core, ino.0)
        });
        match r {
            Ok(Ok(attr)) => reply.attr(&TTL, &attr),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn readlink(&self, _req: &Request, ino: INodeNo, reply: ReplyData) {
        let r = self.call(move |core, _| {
            let inode = core.state().getattr(ino.0)?.ok_or(ClientError::NotFound)?;
            match inode.symlink_target {
                Some(target) => Ok(target.into_bytes()),
                None => Err(ClientError::NotFound),
            }
        });
        match r {
            Ok(Ok(data)) => reply.data(&data),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn mkdir(
        &self,
        req: &Request,
        parent: INodeNo,
        name: &OsStr,
        mode: u32,
        umask: u32,
        reply: ReplyEntry,
    ) {
        if decode_snap(parent.0).is_some() {
            reply.error(Errno::EROFS);
            return;
        }
        let Ok(name) = name_string(name) else {
            reply.error(Errno::EINVAL);
            return;
        };
        let op = MetaOp::Mkdir {
            parent: parent.0,
            name,
            mode: mode & !umask,
            uid: req.uid(),
            gid: req.gid(),
        };
        let r = self.call(move |core, rt| match rt.block_on(core.meta_op(op))? {
            Ok(OpResult::Ino(ino)) => Self::attr_of(core, ino),
            Ok(_) => unreachable!("Mkdir returns Ino"),
            Err(e) => Err(ClientError::Meta(e)),
        });
        match r {
            Ok(Ok(attr)) => reply.entry(&TTL, &attr, GENERATION),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn unlink(&self, _req: &Request, parent: INodeNo, name: &OsStr, reply: ReplyEmpty) {
        let Ok(name) = name_string(name) else {
            reply.error(Errno::EINVAL);
            return;
        };
        let op = MetaOp::Unlink {
            parent: parent.0,
            name,
        };
        let r = self.call(move |core, rt| match rt.block_on(core.meta_op(op))? {
            Ok(_) => Ok(()),
            Err(e) => Err(ClientError::Meta(e)),
        });
        match r {
            Ok(Ok(())) => reply.ok(),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn rmdir(&self, _req: &Request, parent: INodeNo, name: &OsStr, reply: ReplyEmpty) {
        let Ok(name) = name_string(name) else {
            reply.error(Errno::EINVAL);
            return;
        };
        let op = MetaOp::Rmdir {
            parent: parent.0,
            name,
        };
        let r = self.call(move |core, rt| match rt.block_on(core.meta_op(op))? {
            Ok(_) => Ok(()),
            Err(e) => Err(ClientError::Meta(e)),
        });
        match r {
            Ok(Ok(())) => reply.ok(),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn symlink(
        &self,
        req: &Request,
        parent: INodeNo,
        name: &OsStr,
        link: &Path,
        reply: ReplyEntry,
    ) {
        if decode_snap(parent.0).is_some() {
            reply.error(Errno::EROFS);
            return;
        }
        let Ok(name) = name_string(name) else {
            reply.error(Errno::EINVAL);
            return;
        };
        let Some(target) = link.to_str().map(str::to_owned) else {
            reply.error(Errno::EINVAL);
            return;
        };
        let op = MetaOp::Symlink {
            parent: parent.0,
            name,
            target,
            uid: req.uid(),
            gid: req.gid(),
        };
        let r = self.call(move |core, rt| match rt.block_on(core.meta_op(op))? {
            Ok(OpResult::Ino(ino)) => Self::attr_of(core, ino),
            Ok(_) => unreachable!("Symlink returns Ino"),
            Err(e) => Err(ClientError::Meta(e)),
        });
        match r {
            Ok(Ok(attr)) => reply.entry(&TTL, &attr, GENERATION),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn rename(
        &self,
        _req: &Request,
        parent: INodeNo,
        name: &OsStr,
        newparent: INodeNo,
        newname: &OsStr,
        _flags: RenameFlags,
        reply: ReplyEmpty,
    ) {
        let (Ok(name), Ok(newname)) = (name_string(name), name_string(newname)) else {
            reply.error(Errno::EINVAL);
            return;
        };
        let op = MetaOp::Rename {
            src_parent: parent.0,
            src_name: name,
            dst_parent: newparent.0,
            dst_name: newname,
        };
        let r = self.call(move |core, rt| match rt.block_on(core.meta_op(op))? {
            Ok(_) => Ok(()),
            Err(e) => Err(ClientError::Meta(e)),
        });
        match r {
            Ok(Ok(())) => reply.ok(),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn link(
        &self,
        _req: &Request,
        ino: INodeNo,
        newparent: INodeNo,
        newname: &OsStr,
        reply: ReplyEntry,
    ) {
        if decode_snap(newparent.0).is_some() {
            reply.error(Errno::EROFS);
            return;
        }
        let Ok(newname) = name_string(newname) else {
            reply.error(Errno::EINVAL);
            return;
        };
        let op = MetaOp::Link {
            parent: newparent.0,
            name: newname,
            ino: ino.0,
        };
        let r = self.call(move |core, rt| {
            rt.block_on(core.meta_op(op))??;
            Self::attr_of(core, ino.0)
        });
        match r {
            Ok(Ok(attr)) => reply.entry(&TTL, &attr, GENERATION),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn open(&self, _req: &Request, ino: INodeNo, flags: OpenFlags, reply: ReplyOpen) {
        if let Some(snap_ino) = decode_snap(ino.0) {
            let SnapIno::Content(snap, real) = snap_ino else {
                reply.error(Errno::EISDIR);
                return;
            };
            if flags.acc_mode() != OpenAccMode::O_RDONLY {
                reply.error(Errno::EROFS);
                return;
            }
            let r = self.call(move |core, _| {
                let inode = core
                    .snapshot_getattr(snap, real)?
                    .ok_or(ClientError::NotFound)?;
                Ok(inode.kind)
            });
            match r {
                Ok(Ok(plfs_meta::Kind::File)) => reply.opened(
                    FileHandle(0),
                    FopenFlags::FOPEN_DIRECT_IO | FopenFlags::FOPEN_NOFLUSH,
                ),
                Ok(Ok(plfs_meta::Kind::Dir)) => reply.error(Errno::EISDIR),
                Ok(Ok(_)) => reply.error(Errno::ENXIO),
                Ok(Err(err)) => reply.error(errno(&err)),
                Err(err) => reply.error(err),
            }
            return;
        }
        let r = self.call(move |core, _| Self::attr_of(core, ino.0));
        match r {
            Ok(Ok(attr)) => match attr.kind {
                FileType::Directory => reply.error(Errno::EISDIR),
                FileType::RegularFile => {
                    let mut fopen = FopenFlags::FOPEN_DIRECT_IO;
                    if flags.acc_mode() == OpenAccMode::O_RDONLY {
                        fopen |= FopenFlags::FOPEN_NOFLUSH;
                    }
                    reply.opened(FileHandle(0), fopen);
                }
                _ => reply.error(Errno::ENXIO),
            },
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn read(
        &self,
        _req: &Request,
        ino: INodeNo,
        _fh: FileHandle,
        offset: u64,
        size: u32,
        _flags: OpenFlags,
        _lock_owner: Option<LockOwner>,
        reply: ReplyData,
    ) {
        if let Some(SnapIno::Content(snap, real)) = decode_snap(ino.0) {
            let r = self.call(move |core, rt| {
                rt.block_on(core.snapshot_read(snap, real, offset, u64::from(size)))
            });
            match r {
                Ok(Ok(data)) => reply.data(&data),
                Ok(Err(err)) => reply.error(errno(&err)),
                Err(err) => reply.error(err),
            }
            return;
        }
        let r = self.call(move |core, rt| rt.block_on(core.read(ino.0, offset, u64::from(size))));
        match r {
            Ok(Ok(data)) => reply.data(&data),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn write(
        &self,
        _req: &Request,
        ino: INodeNo,
        _fh: FileHandle,
        offset: u64,
        data: &[u8],
        _write_flags: WriteFlags,
        _flags: OpenFlags,
        _lock_owner: Option<LockOwner>,
        reply: ReplyWrite,
    ) {
        if decode_snap(ino.0).is_some() {
            reply.error(Errno::EROFS);
            return;
        }
        let data = data.to_vec();
        let r = self.call(move |core, rt| rt.block_on(core.write(ino.0, offset, &data)));
        match r {
            Ok(Ok(n)) => reply.written(n as u32),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn flush(
        &self,
        _req: &Request,
        _ino: INodeNo,
        _fh: FileHandle,
        _lock_owner: LockOwner,
        reply: ReplyEmpty,
    ) {
        let r = self.call(|core, rt| rt.block_on(core.fsync()));
        match r {
            Ok(Ok(())) => reply.ok(),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn fsync(
        &self,
        _req: &Request,
        _ino: INodeNo,
        _fh: FileHandle,
        _datasync: bool,
        reply: ReplyEmpty,
    ) {
        let r = self.call(|core, rt| rt.block_on(core.fsync()));
        match r {
            Ok(Ok(())) => reply.ok(),
            Ok(Err(err)) => {
                tracing::error!(?err, "fsync failed");
                reply.error(errno(&err))
            }
            Err(err) => reply.error(err),
        }
    }

    fn release(
        &self,
        _req: &Request,
        _ino: INodeNo,
        _fh: FileHandle,
        _flags: OpenFlags,
        _lock_owner: Option<LockOwner>,
        _flush: bool,
        reply: ReplyEmpty,
    ) {
        reply.ok();
    }

    fn opendir(&self, _req: &Request, ino: INodeNo, _flags: OpenFlags, reply: ReplyOpen) {
        if let Some(snap_ino) = decode_snap(ino.0) {
            let r = match snap_ino {
                SnapIno::Root | SnapIno::Dir(_) => Ok(Ok(plfs_meta::Kind::Dir)),
                SnapIno::Content(snap, real) => self.call(move |core, _| {
                    let inode = core
                        .snapshot_getattr(snap, real)?
                        .ok_or(ClientError::NotFound)?;
                    Ok(inode.kind)
                }),
            };
            match r {
                Ok(Ok(plfs_meta::Kind::Dir)) => reply.opened(FileHandle(0), FopenFlags::empty()),
                Ok(Ok(_)) => reply.error(Errno::ENOTDIR),
                Ok(Err(err)) => reply.error(errno(&err)),
                Err(err) => reply.error(err),
            }
            return;
        }
        let r = self.call(move |core, _| Self::attr_of(core, ino.0));
        match r {
            Ok(Ok(attr)) if attr.kind == FileType::Directory => {
                reply.opened(FileHandle(0), FopenFlags::empty())
            }
            Ok(Ok(_)) => reply.error(Errno::ENOTDIR),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn readdir(
        &self,
        _req: &Request,
        ino: INodeNo,
        _fh: FileHandle,
        offset: u64,
        mut reply: ReplyDirectory,
    ) {
        let r = self.call(move |core, _| {
            let typed = match decode_snap(ino.0) {
                Some(SnapIno::Root) => core
                    .snapshot_list()?
                    .into_iter()
                    .map(|s| {
                        (
                            snap_dir_ino(s.id),
                            OsString::from(s.name),
                            FileType::Directory,
                        )
                    })
                    .collect(),
                Some(SnapIno::Dir(snap)) => snap_listdir_typed(core, snap, plfs_meta::ROOT_INO)?,
                Some(SnapIno::Content(snap, dir)) => snap_listdir_typed(core, snap, dir)?,
                None => {
                    let entries = core.state().listdir(ino.0)?;
                    let mut typed = Vec::with_capacity(entries.len());
                    for (name, child) in entries {
                        let kind = match core.state().getattr(child)? {
                            Some(inode) => match inode.kind {
                                plfs_meta::Kind::Dir => FileType::Directory,
                                plfs_meta::Kind::Symlink => FileType::Symlink,
                                plfs_meta::Kind::File => FileType::RegularFile,
                            },
                            None => FileType::RegularFile,
                        };
                        typed.push((child, OsString::from(name), kind));
                    }
                    typed
                }
            };
            Ok::<_, ClientError>(typed)
        });
        let items = match r {
            Ok(Ok(mut typed)) => {
                // ".", ".." (VFS resolves them via the dcache; the ino shown
                // is the dir itself), then entries in byte order. Each entry
                // is served exactly once with the NEXT position as offset.
                let mut items: Vec<(u64, OsString, FileType)> = vec![
                    (ino.0, OsString::from("."), FileType::Directory),
                    (ino.0, OsString::from(".."), FileType::Directory),
                ];
                items.append(&mut typed);
                items
            }
            Ok(Err(err)) => {
                reply.error(errno(&err));
                return;
            }
            Err(err) => {
                reply.error(err);
                return;
            }
        };
        for (i, (child, name, kind)) in items.iter().enumerate().skip(offset as usize) {
            if reply.add(INodeNo(*child), (i + 1) as u64, *kind, name) {
                return; // buffer full
            }
        }
        reply.ok();
    }

    fn statfs(&self, _req: &Request, _ino: INodeNo, reply: ReplyStatfs) {
        let r = self.call(|core, rt| rt.block_on(core.statfs()));
        match r {
            Ok(Ok(stats)) => reply.statfs(
                stats.total_bytes / 4096,
                stats.free_bytes / 4096,
                stats.free_bytes / 4096,
                0,
                u64::from(u32::MAX),
                4096,
                255,
                4096,
            ),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn fsyncdir(
        &self,
        _req: &Request,
        _ino: INodeNo,
        _fh: FileHandle,
        _datasync: bool,
        reply: ReplyEmpty,
    ) {
        reply.ok();
    }

    fn create(
        &self,
        req: &Request,
        parent: INodeNo,
        name: &OsStr,
        mode: u32,
        umask: u32,
        _flags: i32,
        reply: ReplyCreate,
    ) {
        if decode_snap(parent.0).is_some() {
            reply.error(Errno::EROFS);
            return;
        }
        let Ok(name) = name_string(name) else {
            reply.error(Errno::EINVAL);
            return;
        };
        let op = MetaOp::CreateFile {
            parent: parent.0,
            name,
            mode: mode & !umask,
            uid: req.uid(),
            gid: req.gid(),
        };
        let r = self.call(move |core, rt| match rt.block_on(core.meta_op(op))? {
            Ok(OpResult::Ino(ino)) => Self::attr_of(core, ino),
            Ok(_) => unreachable!("CreateFile returns Ino"),
            Err(e) => Err(ClientError::Meta(e)),
        });
        match r {
            Ok(Ok(attr)) => reply.created(
                &TTL,
                &attr,
                GENERATION,
                FileHandle(0),
                FopenFlags::FOPEN_DIRECT_IO,
            ),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }
}

fn mount_config() -> fuser::Config {
    let mut config = fuser::Config::default();
    config.mount_options = vec![
        MountOption::FSName("plfs".to_string()),
        MountOption::CUSTOM("max_read=1048576".to_string()),
    ];
    config
}

/// A live mount; dropping it unmounts and releases the volume.
pub struct MountGuard {
    session: Option<fuser::BackgroundSession>,
}

impl MountGuard {
    /// Mount the volume at `dir` on `mountpoint`.
    pub fn mount(dir: &Path, mountpoint: &Path) -> Result<Self, FuseError> {
        let fs = PlfsFs::open(dir)?;
        let session = fuser::spawn_mount2(fs, mountpoint, &mount_config())?;
        Ok(Self {
            session: Some(session),
        })
    }

    /// Mount with an explicit data-plane sink (standalone: loopback gRPC).
    pub fn mount_with_sink(
        dir: &Path,
        mountpoint: &Path,
        sink: crate::core::SinkConfig,
    ) -> Result<Self, FuseError> {
        let fs = PlfsFs::open_with_sink(dir, sink)?;
        let session = fuser::spawn_mount2(fs, mountpoint, &mount_config())?;
        Ok(Self {
            session: Some(session),
        })
    }

    /// Format a fresh volume at `dir` and mount it.
    pub fn mount_fresh(dir: &Path, mountpoint: &Path, arena_bytes: u64) -> Result<Self, FuseError> {
        let fs = PlfsFs::create(dir, arena_bytes)?;
        let session = fuser::spawn_mount2(fs, mountpoint, &mount_config())?;
        Ok(Self {
            session: Some(session),
        })
    }

    /// Unmount and join the session.
    pub fn unmount(mut self) -> std::io::Result<()> {
        if let Some(session) = self.session.take() {
            session.umount_and_join()?;
        }
        Ok(())
    }
}

impl Drop for MountGuard {
    fn drop(&mut self) {
        if let Some(session) = self.session.take() {
            let _ = session.umount_and_join();
        }
    }
}
