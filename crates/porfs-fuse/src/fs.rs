//! `PorfsFs`: the `fuser::Filesystem` implementation over [`Mds`].
//!
//! Every handler forwards one job to the MDS worker thread (`worker`
//! module), mapping inputs and results/errnos mechanically (`map` module).
//! The only state beyond the MDS itself is `parents`, an in-memory
//! child->parent inode map fed by successful lookups/creates/renames so
//! `readdir` can fill in the `..` entry (the MDS stores no parent pointers;
//! P5's big-directory work may replace this with persistent ones).

use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;
use std::sync::Mutex;
use std::time::SystemTime;

use fuser::{
    AccessFlags, BsdFileFlags, Errno, FileHandle, Filesystem, FopenFlags, Generation, INodeNo,
    LockOwner, OpenFlags, RenameFlags, ReplyAttr, ReplyCreate, ReplyData, ReplyDirectory,
    ReplyEmpty, ReplyEntry, ReplyOpen, ReplyStatfs, ReplyWrite, ReplyXattr, Request, TimeOrNow,
    WriteFlags,
};
use porfs_mds::{Ino, InodeAttr, Mds, MdsError, NodeKind, ROOT_INO, SetAttr};

use crate::config::MountConfig;
use crate::map::{errno, file_attr, file_type, from_system_time, perm_bits};
use crate::worker::Worker;

/// FUSE inode-generation value: v0 does no generation validation (inode
/// numbers are never reused, so stale-handle detection needs none).
const GENERATION: Generation = Generation(0);

/// The PolarisFS filesystem exposed to the kernel.
pub struct PorfsFs {
    worker: Worker,
    parents: Mutex<HashMap<Ino, Ino>>,
    config: MountConfig,
}

impl PorfsFs {
    /// Open the MDS instance at (`meta`, `data`) on the worker thread and
    /// wrap it in a filesystem with the given attribute-cache config.
    ///
    /// # Errors
    /// Whatever [`Mds::open`] returns (unformatted device, salvage
    /// failures, metadata corruption).
    pub fn open(
        meta: impl AsRef<Path>,
        data: impl AsRef<Path>,
        config: MountConfig,
    ) -> Result<Self, MdsError> {
        let meta = meta.as_ref().to_path_buf();
        let data = data.as_ref().to_path_buf();
        Self::with_opener(move || Mds::open(&meta, &data), config)
    }

    /// Wrap a custom MDS opener: the closure runs on the worker thread, so
    /// the `!Send` MDS never crosses threads (see `worker` module docs).
    ///
    /// # Errors
    /// Whatever the opener returns.
    pub fn with_opener(
        open: impl FnOnce() -> Result<Mds, MdsError> + Send + 'static,
        config: MountConfig,
    ) -> Result<Self, MdsError> {
        Ok(Self {
            worker: Worker::spawn(open)?,
            parents: Mutex::new(HashMap::new()),
            config,
        })
    }

    /// The configured attribute-cache TTLs.
    pub fn config(&self) -> MountConfig {
        self.config
    }

    /// Run `f` on the MDS worker; a dead worker maps to EIO.
    fn call<R: Send + 'static>(
        &self,
        f: impl FnOnce(&mut Mds) -> R + Send + 'static,
    ) -> Result<R, Errno> {
        self.worker.call(f).ok_or(Errno::EIO)
    }

    /// Record (or refresh) the parent of `child` for future `..` entries.
    fn note_parent(&self, child: Ino, parent: Ino) {
        match self.parents.lock() {
            Ok(mut map) => {
                map.insert(child, parent);
            }
            Err(poisoned) => {
                poisoned.into_inner().insert(child, parent);
            }
        }
    }

    /// Parent of `ino` for the `..` entry; the root is its own parent, and
    /// an unknown parent degrades to the root (post-remount the map
    /// repopulates through path lookups before a nested `readdir` happens).
    fn parent_of(&self, ino: Ino) -> Ino {
        if ino == ROOT_INO {
            return ROOT_INO;
        }
        match self.parents.lock() {
            Ok(map) => map.get(&ino).copied().unwrap_or(ROOT_INO),
            Err(poisoned) => poisoned.into_inner().get(&ino).copied().unwrap_or(ROOT_INO),
        }
    }

    /// Drop the parent note of a removed node.
    fn forget_parent(&self, child: Ino) {
        match self.parents.lock() {
            Ok(mut map) => {
                map.remove(&child);
            }
            Err(poisoned) => {
                poisoned.into_inner().remove(&child);
            }
        }
    }
}

/// A FUSE name must be valid UTF-8 (v0 MDS names are `&str`); anything else
/// is rejected with EINVAL.
fn name_string(name: &OsStr) -> Result<String, Errno> {
    name.to_str().map(str::to_owned).ok_or(Errno::EINVAL)
}

/// `TimeOrNow` -> MDS timestamp.
fn resolve_time(time: TimeOrNow) -> (i64, u32) {
    match time {
        TimeOrNow::SpecificTime(t) => from_system_time(t),
        TimeOrNow::Now => from_system_time(SystemTime::now()),
    }
}

/// Shared resolve-and-getattr job: look `name` up under `parent` and fetch
/// the child's attributes.
fn lookup_with_attr(mds: &mut Mds, parent: Ino, name: &str) -> porfs_mds::Result<(Ino, InodeAttr)> {
    let ino = mds.lookup(parent, name)?;
    let attr = mds.getattr(ino)?;
    Ok((ino, attr))
}

impl Filesystem for PorfsFs {
    fn lookup(&self, _req: &Request, parent: INodeNo, name: &OsStr, reply: ReplyEntry) {
        let name = match name_string(name) {
            Ok(name) => name,
            Err(err) => return reply.error(err),
        };
        match self.call(move |mds| lookup_with_attr(mds, parent.0, &name)) {
            Ok(Ok((ino, attr))) => {
                self.note_parent(ino, parent.0);
                reply.entry(&self.config.entry_ttl, &file_attr(&attr), GENERATION);
            }
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn forget(&self, _req: &Request, _ino: INodeNo, _nlookup: u64) {
        // Inode numbers are never reused in v0; there is nothing to reclaim.
    }

    fn getattr(&self, _req: &Request, ino: INodeNo, _fh: Option<FileHandle>, reply: ReplyAttr) {
        match self.call(move |mds| mds.getattr(ino.0)) {
            Ok(Ok(attr)) => reply.attr(&self.config.attr_ttl, &file_attr(&attr)),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    #[allow(clippy::too_many_arguments)] // signature fixed by the fuser trait
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
        let set = SetAttr {
            mode: mode.map(perm_bits),
            uid,
            gid,
            size,
            atime: atime.map(resolve_time),
            mtime: mtime.map(resolve_time),
        };
        match self.call(move |mds| mds.setattr(ino.0, set)) {
            Ok(Ok(attr)) => reply.attr(&self.config.attr_ttl, &file_attr(&attr)),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn mkdir(
        &self,
        _req: &Request,
        parent: INodeNo,
        name: &OsStr,
        mode: u32,
        umask: u32,
        reply: ReplyEntry,
    ) {
        let name = match name_string(name) {
            Ok(name) => name,
            Err(err) => return reply.error(err),
        };
        let mode = perm_bits(mode) & !perm_bits(umask);
        let job = move |mds: &mut Mds| {
            let ino = mds.mkdir(parent.0, &name, mode)?;
            let attr = mds.getattr(ino)?;
            Ok((ino, attr))
        };
        match self.call(job) {
            Ok(Ok((ino, attr))) => {
                self.note_parent(ino, parent.0);
                reply.entry(&self.config.entry_ttl, &file_attr(&attr), GENERATION);
            }
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn unlink(&self, _req: &Request, parent: INodeNo, name: &OsStr, reply: ReplyEmpty) {
        let name = match name_string(name) {
            Ok(name) => name,
            Err(err) => return reply.error(err),
        };
        let job = move |mds: &mut Mds| {
            let child = mds.lookup(parent.0, &name).ok();
            mds.unlink(parent.0, &name)?;
            Ok(child)
        };
        match self.call(job) {
            Ok(Ok(child)) => {
                if let Some(child) = child {
                    self.forget_parent(child);
                }
                reply.ok();
            }
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn rmdir(&self, _req: &Request, parent: INodeNo, name: &OsStr, reply: ReplyEmpty) {
        let name = match name_string(name) {
            Ok(name) => name,
            Err(err) => return reply.error(err),
        };
        let job = move |mds: &mut Mds| {
            let child = mds.lookup(parent.0, &name).ok();
            mds.rmdir(parent.0, &name)?;
            Ok(child)
        };
        match self.call(job) {
            Ok(Ok(child)) => {
                if let Some(child) = child {
                    self.forget_parent(child);
                }
                reply.ok();
            }
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
        flags: RenameFlags,
        reply: ReplyEmpty,
    ) {
        if !flags.is_empty() {
            // RENAME_EXCHANGE / RENAME_NOREPLACE / RENAME_WHITEOUT are P5+.
            return reply.error(Errno::EINVAL);
        }
        let name = match name_string(name) {
            Ok(name) => name,
            Err(err) => return reply.error(err),
        };
        let newname = match name_string(newname) {
            Ok(name) => name,
            Err(err) => return reply.error(err),
        };
        let job = move |mds: &mut Mds| {
            let src = mds.lookup(parent.0, &name).ok();
            mds.rename(parent.0, &name, newparent.0, &newname)?;
            Ok(src)
        };
        match self.call(job) {
            Ok(Ok(src)) => {
                if let Some(src) = src {
                    self.note_parent(src, newparent.0);
                }
                reply.ok();
            }
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
        let newname = match name_string(newname) {
            Ok(name) => name,
            Err(err) => return reply.error(err),
        };
        let job = move |mds: &mut Mds| {
            mds.link(ino.0, newparent.0, &newname)?;
            mds.getattr(ino.0)
        };
        match self.call(job) {
            Ok(Ok(attr)) => {
                self.note_parent(ino.0, newparent.0);
                reply.entry(&self.config.entry_ttl, &file_attr(&attr), GENERATION);
            }
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn open(&self, _req: &Request, ino: INodeNo, _flags: OpenFlags, reply: ReplyOpen) {
        match self.call(move |mds| mds.getattr(ino.0)) {
            Ok(Ok(attr)) if attr.kind == NodeKind::Dir => reply.error(Errno::EISDIR),
            Ok(Ok(_)) => reply.opened(FileHandle(0), FopenFlags::empty()),
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
        match self.call(move |mds| mds.read(ino.0, offset, u64::from(size))) {
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
        let data = data.to_vec();
        match self.call(move |mds| mds.write(ino.0, offset, &data)) {
            Ok(Ok(written)) => reply.written(written as u32),
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
        // No fsync-on-close in v0: close-to-open consistency is P11.
        reply.ok();
    }

    fn fsync(
        &self,
        _req: &Request,
        ino: INodeNo,
        _fh: FileHandle,
        _datasync: bool,
        reply: ReplyEmpty,
    ) {
        match self.call(move |mds| mds.fsync(ino.0)) {
            Ok(Ok(())) => reply.ok(),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn opendir(&self, _req: &Request, ino: INodeNo, _flags: OpenFlags, reply: ReplyOpen) {
        match self.call(move |mds| mds.getattr(ino.0)) {
            Ok(Ok(attr)) if attr.kind != NodeKind::Dir => reply.error(Errno::ENOTDIR),
            Ok(Ok(_)) => reply.opened(FileHandle(0), FopenFlags::empty()),
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
        let entries = match self.call(move |mds| mds.readdir(ino.0)) {
            Ok(Ok(entries)) => entries,
            Ok(Err(err)) => return reply.error(errno(&err)),
            Err(err) => return reply.error(err),
        };
        let parent = self.parent_of(ino.0);
        // The full listing is rebuilt per call (v0); the kernel resumes at
        // the offset of the first entry that did not fit.
        let mut all: Vec<(String, Ino, NodeKind)> = Vec::with_capacity(entries.len() + 2);
        all.push((".".to_string(), ino.0, NodeKind::Dir));
        all.push(("..".to_string(), parent, NodeKind::Dir));
        all.extend(entries);
        for (index, (name, child, kind)) in all.iter().enumerate().skip(offset as usize) {
            let next_offset = (index + 1) as u64;
            if reply.add(INodeNo(*child), next_offset, file_type(*kind), name) {
                break;
            }
        }
        reply.ok();
    }

    fn fsyncdir(
        &self,
        _req: &Request,
        ino: INodeNo,
        _fh: FileHandle,
        _datasync: bool,
        reply: ReplyEmpty,
    ) {
        match self.call(move |mds| mds.fsync(ino.0)) {
            Ok(Ok(())) => reply.ok(),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn statfs(&self, _req: &Request, _ino: INodeNo, reply: ReplyStatfs) {
        match self.call(|mds| mds.statfs()) {
            Ok(Ok(stats)) => {
                let total_blocks = stats.total_bytes / 512;
                let free_blocks = stats.free_bytes / 512;
                reply.statfs(
                    total_blocks,
                    free_blocks,
                    free_blocks,
                    stats.inodes,
                    u64::from(u32::MAX), // u64 inode space: effectively unlimited
                    4096,
                    255,
                    4096,
                );
            }
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn getxattr(
        &self,
        _req: &Request,
        _ino: INodeNo,
        _name: &OsStr,
        _size: u32,
        reply: ReplyXattr,
    ) {
        // xattrs are P5; report "not supported" rather than "no attribute".
        reply.error(Errno::ENOTSUP)
    }

    fn listxattr(&self, _req: &Request, _ino: INodeNo, _size: u32, reply: ReplyXattr) {
        reply.error(Errno::ENOTSUP)
    }

    fn access(&self, _req: &Request, _ino: INodeNo, _mask: AccessFlags, reply: ReplyEmpty) {
        // v0 performs no permission checks (no default_permissions mount
        // option either); NFSv4 ACLs land in P24.
        reply.ok();
    }

    fn create(
        &self,
        _req: &Request,
        parent: INodeNo,
        name: &OsStr,
        mode: u32,
        umask: u32,
        _flags: i32,
        reply: ReplyCreate,
    ) {
        let name = match name_string(name) {
            Ok(name) => name,
            Err(err) => return reply.error(err),
        };
        let mode = perm_bits(mode) & !perm_bits(umask);
        let job = move |mds: &mut Mds| {
            let ino = mds.create(parent.0, &name, mode)?;
            let attr = mds.getattr(ino)?;
            Ok((ino, attr))
        };
        match self.call(job) {
            Ok(Ok((ino, attr))) => {
                self.note_parent(ino, parent.0);
                reply.created(
                    &self.config.entry_ttl,
                    &file_attr(&attr),
                    GENERATION,
                    FileHandle(0),
                    FopenFlags::empty(),
                );
            }
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }
}
