//! `PorfsFs`: the `fuser::Filesystem` implementation over [`Mds`].
//!
//! Every handler forwards one job to the MDS worker thread (`worker`
//! module), mapping inputs and results/errnos mechanically (`map` module).
//! State beyond the MDS itself:
//! - `parents`: an in-memory child->parent inode map fed by successful
//!   lookups/creates/renames so `readdir` can fill in the `..` entry (the
//!   MDS stores no parent pointers);
//! - `dir_states`: per-open-directory stream state for the streaming
//!   `readdir` (the MDS paginates via continuation cookies; the kernel's
//!   u64 offsets are synthetic and only 0 means "rewind").

use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;

use fuser::{
    AccessFlags, BsdFileFlags, Errno, FileHandle, FileType, Filesystem, FopenFlags, Generation,
    INodeNo, LockOwner, OpenAccMode, OpenFlags, RenameFlags, ReplyAttr, ReplyCreate, ReplyData,
    ReplyDirectory, ReplyEmpty, ReplyEntry, ReplyLseek, ReplyOpen, ReplyStatfs, ReplyWrite,
    ReplyXattr, Request, TimeOrNow, WriteFlags,
};
use porfs_mds::{
    ClusterTimeouts, Ino, InodeAttr, MAX_SYMLINK_LEN, Mds, MdsError, NodeKind, ROOT_INO, SetAttr,
};

use crate::config::MountConfig;
use crate::map::{errno, file_attr, file_type, from_system_time, perm_bits};
use crate::worker::Worker;

/// FUSE inode-generation value: v0 does no generation validation (inode
/// numbers are never reused, so stale-handle detection needs none).
const GENERATION: Generation = Generation(0);

/// Entries fetched per MDS roundtrip while streaming a directory. 128
/// entries keeps a 1M-entry `ls` at ~8k channel hops — sub-second — while
/// bounding per-call memory.
const READDIR_BATCH: usize = 128;

/// `mode & S_IFMT` values the kernel passes to `mknod` (stable Linux ABI).
const S_IFMT: u32 = 0o170000;
const S_IFREG: u32 = 0o100000;
const S_IFIFO: u32 = 0o010000;
const S_IFSOCK: u32 = 0o140000;
const S_IFCHR: u32 = 0o020000;
const S_IFBLK: u32 = 0o060000;

/// `lseek` whence values (Linux).
const SEEK_DATA: i32 = 3;
const SEEK_HOLE: i32 = 4;

/// `fallocate` mode bits (Linux).
const FALLOC_FL_KEEP_SIZE: i32 = 0x01;
const FALLOC_FL_PUNCH_HOLE: i32 = 0x02;

/// Stream state of one open directory handle: the raw `dir_entries` key of
/// the last entry handed to the kernel (the resume cookie), the next
/// synthetic offset to hand out, and whether the scan is exhausted. The
/// synthetic offsets only get meaning 0 (rewind), 1 (after "."), 2 (after
/// ".."); every later offset just says "keep streaming from the cookie" —
/// `seekdir` to a mid-stream position is not reconstructible, a documented
/// v1 limitation (POSIX-conformant sequential scans, which is what `ls`,
/// `find`, and pjdfstest do, are exact).
#[derive(Default)]
struct DirState {
    cookie: Option<Vec<u8>>,
    next_offset: u64,
    done: bool,
}

/// The PolarisFS filesystem exposed to the kernel.
pub struct PorfsFs {
    worker: Worker,
    parents: Mutex<HashMap<Ino, Ino>>,
    dir_states: Mutex<HashMap<u64, DirState>>,
    next_fh: AtomicU64,
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

    /// Open the cluster-mode MDS instance at `meta` on the worker thread:
    /// membership, UUID pins, and the replica count come from the
    /// persisted `cluster_config` ([`Mds::open_cluster`]), so a remount
    /// needs no chunkserver flags.
    ///
    /// # Errors
    /// Whatever [`Mds::open_cluster`] returns (a local-mode database,
    /// unreachable or misidentified chunkservers, metadata corruption).
    pub fn open_cluster(meta: impl AsRef<Path>, config: MountConfig) -> Result<Self, MdsError> {
        let meta = meta.as_ref().to_path_buf();
        Self::with_opener(move || Mds::open_cluster(&meta), config)
    }

    /// [`PorfsFs::open_cluster`] with explicit cluster network timeouts:
    /// integration tests shorten the failure paths (the production
    /// defaults wait out real network partitions).
    ///
    /// # Errors
    /// Whatever [`Mds::open_cluster_with_timeouts`] returns.
    #[doc(hidden)]
    pub fn open_cluster_with_timeouts(
        meta: impl AsRef<Path>,
        config: MountConfig,
        timeouts: ClusterTimeouts,
    ) -> Result<Self, MdsError> {
        let meta = meta.as_ref().to_path_buf();
        Self::with_opener(
            move || Mds::open_cluster_with_timeouts(&meta, timeouts),
            config,
        )
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
            dir_states: Mutex::new(HashMap::new()),
            next_fh: AtomicU64::new(1),
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

    /// Allocate a kernel file handle id.
    fn alloc_fh(&self) -> FileHandle {
        FileHandle(self.next_fh.fetch_add(1, Ordering::Relaxed))
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

/// The getxattr/listxattr size-query protocol: with `size == 0` the caller
/// wants the required buffer size; otherwise the value must fit or the
/// reply is ERANGE.
fn reply_xattr(reply: ReplyXattr, size: u32, payload: Vec<u8>) {
    if size == 0 {
        reply.size(payload.len() as u32);
    } else if payload.len() > size as usize {
        reply.error(Errno::ERANGE);
    } else {
        reply.data(&payload);
    }
}

impl Filesystem for PorfsFs {
    /// Clean-unmount barrier: confirm every appended extent (redb commits
    /// are durable per transaction already), so a remount never has to
    /// salvage writes that were never fsynced by the application. Without
    /// this a polite `fusermount3 -u` loses un-fsynced data — legal on
    /// crash, wrong on clean unmount.
    fn destroy(&mut self) {
        if self.call(|mds| mds.fsync(ROOT_INO)).is_err() {
            eprintln!("porfs: destroy: mds worker gone, unconfirmed tail may be salvaged");
        }
    }

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

    fn readlink(&self, _req: &Request, ino: INodeNo, reply: ReplyData) {
        match self.call(move |mds| mds.readlink(ino.0)) {
            Ok(Ok(target)) => reply.data(target.as_bytes()),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn mknod(
        &self,
        req: &Request,
        parent: INodeNo,
        name: &OsStr,
        mode: u32,
        umask: u32,
        rdev: u32,
        reply: ReplyEntry,
    ) {
        let kind = match mode & S_IFMT {
            S_IFREG => NodeKind::File,
            S_IFIFO => NodeKind::Fifo,
            S_IFSOCK => NodeKind::Socket,
            S_IFCHR => NodeKind::Chr,
            S_IFBLK => NodeKind::Blk,
            _ => return reply.error(Errno::EINVAL),
        };
        let name = match name_string(name) {
            Ok(name) => name,
            Err(err) => return reply.error(err),
        };
        let (uid, gid) = (req.uid(), req.gid());
        let mode = perm_bits(mode) & !perm_bits(umask);
        let job = move |mds: &mut Mds| {
            let spec = porfs_mds::NodeSpec {
                kind,
                mode,
                rdev,
                uid,
                gid,
            };
            let ino = mds.mknod(parent.0, &name, spec)?;
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

    fn mkdir(
        &self,
        req: &Request,
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
        let (uid, gid) = (req.uid(), req.gid());
        let mode = perm_bits(mode) & !perm_bits(umask);
        let job = move |mds: &mut Mds| {
            let ino = mds.mkdir(parent.0, &name, mode, uid, gid)?;
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

    fn symlink(
        &self,
        req: &Request,
        parent: INodeNo,
        link_name: &OsStr,
        target: &Path,
        reply: ReplyEntry,
    ) {
        let name = match name_string(link_name) {
            Ok(name) => name,
            Err(err) => return reply.error(err),
        };
        let Some(target) = target.to_str() else {
            return reply.error(Errno::EINVAL);
        };
        if target.is_empty() {
            return reply.error(Errno::ENOENT); // Linux: symlink("") -> ENOENT
        }
        if target.len() > MAX_SYMLINK_LEN {
            return reply.error(Errno::ENAMETOOLONG);
        }
        let target = target.to_owned();
        let (uid, gid) = (req.uid(), req.gid());
        let job = move |mds: &mut Mds| {
            let ino = mds.symlink(parent.0, &name, &target, uid, gid)?;
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
            // RENAME_EXCHANGE / RENAME_NOREPLACE / RENAME_WHITEOUT are P6+.
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

    fn open(&self, _req: &Request, ino: INodeNo, flags: OpenFlags, reply: ReplyOpen) {
        match self.call(move |mds| mds.getattr(ino.0)) {
            Ok(Ok(attr)) => match attr.kind {
                NodeKind::Dir => reply.error(Errno::EISDIR),
                // Fifos must open successfully: the kernel pipes I/O through
                // its own fifo implementation, but it still calls our open.
                NodeKind::File => {
                    // Revalidating above and bypassing the kernel page cache
                    // gives every open a fresh view of data closed by another
                    // client. P17 replaces this conservative policy with
                    // lease-backed coherent caching.
                    let mut fopen = FopenFlags::FOPEN_DIRECT_IO;
                    if flags.acc_mode() == OpenAccMode::O_RDONLY {
                        // A read-only handle carries no dirty data, so its
                        // close has nothing to flush. Suppressing FUSE_FLUSH
                        // keeps a reader's close() instant and error-free
                        // when a chunkserver is down — the strict sync
                        // broadcast would otherwise stall EVERY close for
                        // the full sync timeout and then report EIO on a
                        // handle that never wrote, which contradicts the
                        // "the mount keeps serving all reads" failover
                        // contract. Write handles keep the close-flush
                        // barrier (P11 close-to-open), unchanged.
                        fopen |= FopenFlags::FOPEN_NOFLUSH;
                    }
                    reply.opened(FileHandle(0), fopen);
                }
                NodeKind::Fifo => reply.opened(FileHandle(0), FopenFlags::empty()),
                NodeKind::Symlink | NodeKind::Socket | NodeKind::Chr | NodeKind::Blk => {
                    reply.error(Errno::ENXIO)
                }
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
        ino: INodeNo,
        _fh: FileHandle,
        _lock_owner: LockOwner,
        reply: ReplyEmpty,
    ) {
        match self.call(move |mds| mds.fsync(ino.0)) {
            Ok(Ok(())) => reply.ok(),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn fsync(
        &self,
        _req: &Request,
        ino: INodeNo,
        _fh: FileHandle,
        datasync: bool,
        reply: ReplyEmpty,
    ) {
        let job = move |mds: &mut Mds| {
            if datasync {
                mds.fdatasync(ino.0)
            } else {
                mds.fsync(ino.0)
            }
        };
        match self.call(job) {
            Ok(Ok(())) => reply.ok(),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn opendir(&self, _req: &Request, ino: INodeNo, _flags: OpenFlags, reply: ReplyOpen) {
        match self.call(move |mds| mds.getattr(ino.0)) {
            Ok(Ok(attr)) if attr.kind != NodeKind::Dir => reply.error(Errno::ENOTDIR),
            Ok(Ok(_)) => {
                let fh = self.alloc_fh();
                match self.dir_states.lock() {
                    Ok(mut map) => {
                        map.insert(fh.0, DirState::default());
                    }
                    Err(poisoned) => {
                        poisoned.into_inner().insert(fh.0, DirState::default());
                    }
                }
                reply.opened(fh, FopenFlags::empty());
            }
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn readdir(
        &self,
        _req: &Request,
        ino: INodeNo,
        fh: FileHandle,
        offset: u64,
        mut reply: ReplyDirectory,
    ) {
        let mut guard = match self.dir_states.lock() {
            Ok(map) => map,
            Err(poisoned) => poisoned.into_inner(),
        };
        let state = guard.entry(fh.0).or_default();
        if offset == 0 {
            *state = DirState::default();
        }
        // The `off` carried by each reply.add entry is the position the
        // kernel passes back to fetch the NEXT entry: "." is position 0
        // (next = 1), ".." position 1 (next = 2), MDS entries start at
        // position 2 (cookie = None) and resume from state.cookie at
        // positions >= 3. Each dot entry is served exactly once: an
        // offset past a dot's position means the kernel already consumed
        // it — re-serving it (with the same off) would make the kernel
        // request that same offset forever (livelock).
        if offset == 0 && reply.add(ino, 1, FileType::Directory, ".") {
            return reply.ok();
        }
        if offset <= 1 {
            let parent = self.parent_of(ino.0);
            if reply.add(INodeNo(parent), 2, FileType::Directory, "..") {
                return reply.ok();
            }
        }
        if offset <= 2 {
            state.cookie = None;
            state.next_offset = 3;
            state.done = false;
        }
        loop {
            if state.done {
                break;
            }
            let after = state.cookie.clone();
            let batch = match self
                .call(move |mds| mds.readdir_batch(ino.0, after.as_deref(), READDIR_BATCH))
            {
                Ok(Ok(batch)) => batch,
                Ok(Err(err)) => return reply.error(errno(&err)),
                Err(err) => return reply.error(err),
            };
            let exhausted = batch.next_cookie.is_none();
            let fetched = batch.entries.len();
            for entry in batch.entries {
                let name = entry.name.as_str();
                if reply.add(
                    INodeNo(entry.ino),
                    state.next_offset,
                    file_type(entry.kind),
                    name,
                ) {
                    // Buffer full: the next kernel call resumes after the
                    // last entry that DID fit (state.cookie) — total work
                    // across a full listing stays O(n).
                    return reply.ok();
                }
                state.cookie = Some(entry.cookie);
                state.next_offset += 1;
            }
            if exhausted || fetched == 0 {
                state.done = true;
            }
        }
        reply.ok();
    }

    fn releasedir(
        &self,
        _req: &Request,
        _ino: INodeNo,
        fh: FileHandle,
        _flags: OpenFlags,
        reply: ReplyEmpty,
    ) {
        match self.dir_states.lock() {
            Ok(mut map) => {
                map.remove(&fh.0);
            }
            Err(poisoned) => {
                poisoned.into_inner().remove(&fh.0);
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
        // fsync on a directory is a metadata barrier (legal per POSIX).
        match self.call(move |mds| mds.fsync(ino.0)) {
            Ok(Ok(())) => reply.ok(),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn statfs(&self, _req: &Request, _ino: INodeNo, reply: ReplyStatfs) {
        match self.call(|mds| mds.statfs()) {
            Ok(Ok(stats)) => {
                let total_blocks = stats.total_bytes / 4096;
                let free_blocks = stats.free_bytes / 4096;
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

    fn setxattr(
        &self,
        _req: &Request,
        ino: INodeNo,
        name: &OsStr,
        value: &[u8],
        flags: i32,
        _position: u32,
        reply: ReplyEmpty,
    ) {
        let name = match name_string(name) {
            Ok(name) => name,
            Err(err) => return reply.error(err),
        };
        let value = value.to_vec();
        let flags = flags as u32;
        match self.call(move |mds| mds.setxattr(ino.0, &name, &value, flags)) {
            Ok(Ok(())) => reply.ok(),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn getxattr(&self, _req: &Request, ino: INodeNo, name: &OsStr, size: u32, reply: ReplyXattr) {
        let name = match name_string(name) {
            Ok(name) => name,
            Err(err) => return reply.error(err),
        };
        match self.call(move |mds| mds.getxattr(ino.0, &name)) {
            Ok(Ok(value)) => reply_xattr(reply, size, value),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn listxattr(&self, _req: &Request, ino: INodeNo, size: u32, reply: ReplyXattr) {
        match self.call(move |mds| mds.listxattr(ino.0)) {
            Ok(Ok(names)) => {
                // Linux wire format: NUL-separated name list.
                let mut payload = Vec::new();
                for name in &names {
                    payload.extend_from_slice(name.as_bytes());
                    payload.push(0);
                }
                reply_xattr(reply, size, payload);
            }
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn removexattr(&self, _req: &Request, ino: INodeNo, name: &OsStr, reply: ReplyEmpty) {
        let name = match name_string(name) {
            Ok(name) => name,
            Err(err) => return reply.error(err),
        };
        match self.call(move |mds| mds.removexattr(ino.0, &name)) {
            Ok(Ok(())) => reply.ok(),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn fallocate(
        &self,
        _req: &Request,
        ino: INodeNo,
        _fh: FileHandle,
        offset: u64,
        length: u64,
        mode: i32,
        reply: ReplyEmpty,
    ) {
        if mode != FALLOC_FL_KEEP_SIZE | FALLOC_FL_PUNCH_HOLE {
            // v1 supports hole-punching only; plain preallocation and every
            // other mode are ENOSYS.
            return reply.error(Errno::ENOSYS);
        }
        match self.call(move |mds| mds.punch_hole(ino.0, offset, length)) {
            Ok(Ok(())) => reply.ok(),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn lseek(
        &self,
        _req: &Request,
        ino: INodeNo,
        _fh: FileHandle,
        offset: i64,
        whence: i32,
        reply: ReplyLseek,
    ) {
        if offset < 0 || (whence != SEEK_DATA && whence != SEEK_HOLE) {
            return reply.error(Errno::EINVAL);
        }
        let data = whence == SEEK_DATA;
        match self.call(move |mds| mds.seek(ino.0, offset as u64, data)) {
            Ok(Ok(pos)) => reply.offset(pos as i64),
            Ok(Err(MdsError::OutOfRange(_))) => reply.error(Errno::ENXIO),
            Ok(Err(err)) => reply.error(errno(&err)),
            Err(err) => reply.error(err),
        }
    }

    fn access(&self, _req: &Request, _ino: INodeNo, _mask: AccessFlags, reply: ReplyEmpty) {
        // With default_permissions the kernel performs permission checks
        // itself and never calls this; without it v0 allows everything
        // (NFSv4 ACLs land in P24).
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
        let name = match name_string(name) {
            Ok(name) => name,
            Err(err) => return reply.error(err),
        };
        let (uid, gid) = (req.uid(), req.gid());
        let mode = perm_bits(mode) & !perm_bits(umask);
        let job = move |mds: &mut Mds| {
            let ino = mds.create(parent.0, &name, mode, uid, gid)?;
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
