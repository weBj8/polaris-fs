//! FUSE 桥接：把 [`crate::PolarisFs`] 包成 fuser::Filesystem 实例，可以直接挂载。
//!
//! 因为 `fuser::Filesystem` 要求 `&self`，但 [`crate::PolarisFs`] 的写操作要 `&mut self`，
//! 所以这里用 `Mutex<PolarisFs>` 做内部可变性。单节点 v0.1 没有并发优化，
//! FUSE 内核回调天然串行化（单 session），互斥锁不会成为瓶颈。
//!
//! inode 号映射：v0.1 直接用 [`crate::inode::Inode::ino`] 作为 FUSE inode 号。
//! 根目录 ino = [`crate::superblock::Superblock::root_ino`] = 1。FUSE 协议用 ino=1 作根，
//! 这两个数字恰好一致。

use std::ffi::OsStr;
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use fuser::{
    FopenFlags, FileAttr, FileHandle, FileType, Filesystem, Generation, INodeNo, KernelConfig,
    OpenFlags, ReplyAttr, ReplyData, ReplyDirectory, ReplyEmpty, ReplyEntry, ReplyStatfs,
    ReplyWrite, Request,
};

use crate::fs::PolarisFs;
use crate::inode::{S_IFDIR, S_IFREG, is_dir, is_reg};

const TTL: Duration = Duration::from_secs(1);

pub struct PolarisFuse {
    fs: Mutex<PolarisFs>,
}

impl PolarisFuse {
    pub fn new(fs: PolarisFs) -> Self {
        Self { fs: Mutex::new(fs) }
    }
}

fn inode_to_attr(ino: u64, i: &crate::inode::Inode) -> FileAttr {
    let kind = if is_dir(i.mode) {
        FileType::Directory
    } else if is_reg(i.mode) {
        FileType::RegularFile
    } else {
        FileType::RegularFile
    };
    let perm = (i.mode & 0o7777) as u16;
    let blksize: u32 = 4096;
    let blocks = i.blocks * (BLOCK_SIZE / 512) as u64;
    FileAttr {
        ino: INodeNo(ino),
        size: i.size,
        blocks,
        atime: unix_ts(i.atime),
        mtime: unix_ts(i.mtime),
        ctime: unix_ts(i.ctime),
        crtime: unix_ts(i.ctime),
        kind,
        perm,
        nlink: i.nlink,
        uid: i.uid,
        gid: i.gid,
        rdev: 0,
        blksize,
        flags: 0,
    }
}

fn unix_ts(secs: u64) -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(secs)
}

fn errno_from_io(e: std::io::Error) -> fuser::Errno {
    use std::io::ErrorKind::*;
    match e.kind() {
        NotFound => fuser::Errno::ENOENT,
        AlreadyExists => fuser::Errno::EEXIST,
        PermissionDenied => fuser::Errno::EACCES,
        IsADirectory => fuser::Errno::EISDIR,
        NotADirectory => fuser::Errno::ENOTDIR,
        DirectoryNotEmpty => fuser::Errno::ENOTEMPTY,
        StorageFull => fuser::Errno::ENOSPC,
        InvalidInput | InvalidData => fuser::Errno::EINVAL,
        _ => fuser::Errno::EIO,
    }
}

const BLOCK_SIZE: usize = 4096;

impl Filesystem for PolarisFuse {
    fn init(
        &mut self,
        _req: &Request,
        _config: &mut KernelConfig,
    ) -> std::io::Result<()> {
        Ok(())
    }

    fn destroy(&mut self) {}

    fn lookup(&self, _req: &Request, parent: INodeNo, name: &OsStr, reply: ReplyEntry) {
        let fs = self.fs.lock().unwrap();
        let name = match name.to_str() {
            Some(n) => n,
            None => {
                reply.error(fuser::Errno::EINVAL);
                return;
            }
        };
        match fs.dir_lookup(parent.into(), name) {
            Ok(Some(child_ino)) => match fs.getattr(child_ino) {
                Ok(inode) => {
                    reply.entry(&TTL, &inode_to_attr(child_ino, &inode), Generation(0));
                }
                Err(e) => reply.error(errno_from_io(e)),
            },
            Ok(None) => reply.error(fuser::Errno::ENOENT),
            Err(e) => reply.error(errno_from_io(e)),
        }
    }

    fn forget(&self, _req: &Request, _ino: INodeNo, _nlookup: u64) {}

    fn getattr(&self, _req: &Request, ino: INodeNo, _fh: Option<FileHandle>, reply: ReplyAttr) {
        let fs = self.fs.lock().unwrap();
        match fs.getattr(ino.into()) {
            Ok(inode) => reply.attr(&TTL, &inode_to_attr(ino.into(), &inode)),
            Err(e) => reply.error(errno_from_io(e)),
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
        _atime: Option<fuser::TimeOrNow>,
        _mtime: Option<fuser::TimeOrNow>,
        _ctime: Option<SystemTime>,
        _fh: Option<FileHandle>,
        _crtime: Option<SystemTime>,
        _chgtime: Option<SystemTime>,
        _bkuptime: Option<SystemTime>,
        _flags: Option<fuser::BsdFileFlags>,
        reply: ReplyAttr,
    ) {
        let mut fs = self.fs.lock().unwrap();
        match fs.setattr(ino.into(), size, mode, uid, gid) {
            Ok(inode) => reply.attr(&TTL, &inode_to_attr(ino.into(), &inode)),
            Err(e) => reply.error(errno_from_io(e)),
        }
    }

    fn mknod(
        &self,
        req: &Request,
        parent: INodeNo,
        name: &OsStr,
        mode: u32,
        _umask: u32,
        _rdev: u32,
        reply: ReplyEntry,
    ) {
        let mut fs = self.fs.lock().unwrap();
        let name = match name.to_str() {
            Some(n) => n,
            None => {
                reply.error(fuser::Errno::EINVAL);
                return;
            }
        };
        match fs.create(parent.into(), name, mode, req.uid(), req.gid()) {
            Ok(inode) => reply.entry(&TTL, &inode_to_attr(inode.ino, &inode), Generation(0)),
            Err(e) => reply.error(errno_from_io(e)),
        }
    }

    fn mkdir(
        &self,
        req: &Request,
        parent: INodeNo,
        name: &OsStr,
        mode: u32,
        _umask: u32,
        reply: ReplyEntry,
    ) {
        let mut fs = self.fs.lock().unwrap();
        let name = match name.to_str() {
            Some(n) => n,
            None => {
                reply.error(fuser::Errno::EINVAL);
                return;
            }
        };
        match fs.mkdir(parent.into(), name, mode, req.uid(), req.gid()) {
            Ok(inode) => reply.entry(&TTL, &inode_to_attr(inode.ino, &inode), Generation(0)),
            Err(e) => reply.error(errno_from_io(e)),
        }
    }

    fn unlink(&self, _req: &Request, parent: INodeNo, name: &OsStr, reply: ReplyEmpty) {
        let mut fs = self.fs.lock().unwrap();
        let name = match name.to_str() {
            Some(n) => n,
            None => {
                reply.error(fuser::Errno::EINVAL);
                return;
            }
        };
        match fs.unlink(parent.into(), name) {
            Ok(()) => reply.ok(),
            Err(e) => reply.error(errno_from_io(e)),
        }
    }

    fn rmdir(&self, _req: &Request, parent: INodeNo, name: &OsStr, reply: ReplyEmpty) {
        let mut fs = self.fs.lock().unwrap();
        let name = match name.to_str() {
            Some(n) => n,
            None => {
                reply.error(fuser::Errno::EINVAL);
                return;
            }
        };
        match fs.rmdir(parent.into(), name) {
            Ok(()) => reply.ok(),
            Err(e) => reply.error(errno_from_io(e)),
        }
    }

    fn open(
        &self,
        _req: &Request,
        _ino: INodeNo,
        _flags: OpenFlags,
        reply: fuser::ReplyOpen,
    ) {
        reply.opened(FileHandle(0), FopenFlags::empty());
    }

    fn read(
        &self,
        _req: &Request,
        ino: INodeNo,
        _fh: FileHandle,
        offset: u64,
        size: u32,
        _flags: OpenFlags,
        _lock_owner: Option<fuser::LockOwner>,
        reply: ReplyData,
    ) {
        let fs = self.fs.lock().unwrap();
        match fs.read(ino.into(), offset, size) {
            Ok(buf) => reply.data(&buf),
            Err(e) => reply.error(errno_from_io(e)),
        }
    }

    fn write(
        &self,
        _req: &Request,
        ino: INodeNo,
        _fh: FileHandle,
        offset: u64,
        data: &[u8],
        _write_flags: fuser::WriteFlags,
        _flags: OpenFlags,
        _lock_owner: Option<fuser::LockOwner>,
        reply: ReplyWrite,
    ) {
        let mut fs = self.fs.lock().unwrap();
        match fs.write(ino.into(), offset, data) {
            Ok(n) => reply.written(n as u32),
            Err(e) => reply.error(errno_from_io(e)),
        }
    }

    fn opendir(
        &self,
        _req: &Request,
        _ino: INodeNo,
        _flags: OpenFlags,
        reply: fuser::ReplyOpen,
    ) {
        reply.opened(FileHandle(0), FopenFlags::empty());
    }

    fn readdir(
        &self,
        _req: &Request,
        ino: INodeNo,
        _fh: FileHandle,
        offset: u64,
        mut reply: ReplyDirectory,
    ) {
        let fs = self.fs.lock().unwrap();
        let entries = match fs.readdir(ino.into()) {
            Ok(e) => e,
            Err(e) => {
                reply.error(errno_from_io(e));
                return;
            }
        };
        for (i, (name, child_ino, mode)) in entries.iter().enumerate().skip(offset as usize) {
            let kind = if is_dir(*mode) {
                FileType::Directory
            } else if is_reg(*mode) {
                FileType::RegularFile
            } else {
                FileType::RegularFile
            };
            // 索引 (i+1) 作为下一次 readdir 的偏移
            if reply.add(INodeNo(*child_ino), (i + 1) as u64, kind, name) {
                break;
            }
        }
        reply.ok();
    }

    fn statfs(&self, _req: &Request, _ino: INodeNo, reply: ReplyStatfs) {
        let fs = self.fs.lock().unwrap();
        let total_blocks = fs.sb.scm_data_blocks + fs.sb.qlc_data_blocks;
        let free = fs.scm_alloc.free_blocks() + fs.qlc_alloc.free_blocks();
        let total_inodes = fs.sb.inode_count_max;
        // inode 计数：v0.1 简单按 sb.last_ino 报告
        let used_inodes = fs.sb.last_ino;
        reply.statfs(
            total_blocks,
            free,
            free,
            total_inodes,
            total_inodes.saturating_sub(used_inodes),
            BLOCK_SIZE as u32,
            256,
            0,
        );
    }

    fn fsync(
        &self,
        _req: &Request,
        _ino: INodeNo,
        _fh: FileHandle,
        _datasync: bool,
        reply: ReplyEmpty,
    ) {
        let mut fs = self.fs.lock().unwrap();
        match fs.sync_all() {
            Ok(()) => reply.ok(),
            Err(e) => reply.error(errno_from_io(e)),
        }
    }

    fn flush(
        &self,
        _req: &Request,
        _ino: INodeNo,
        _fh: FileHandle,
        _lock_owner: fuser::LockOwner,
        reply: ReplyEmpty,
    ) {
        reply.ok();
    }

    fn release(
        &self,
        _req: &Request,
        _ino: INodeNo,
        _fh: FileHandle,
        _flags: OpenFlags,
        _lock_owner: Option<fuser::LockOwner>,
        _flush: bool,
        reply: ReplyEmpty,
    ) {
        reply.ok();
    }

    fn releasedir(
        &self,
        _req: &Request,
        _ino: INodeNo,
        _fh: FileHandle,
        _flags: OpenFlags,
        reply: ReplyEmpty,
    ) {
        reply.ok();
    }

    fn rename(
        &self,
        _req: &Request,
        parent: INodeNo,
        name: &OsStr,
        newparent: INodeNo,
        newname: &OsStr,
        _flags: fuser::RenameFlags,
        reply: ReplyEmpty,
    ) {
        let mut fs = self.fs.lock().unwrap();
        let name = match name.to_str() {
            Some(n) => n,
            None => {
                reply.error(fuser::Errno::EINVAL);
                return;
            }
        };
        let newname = match newname.to_str() {
            Some(n) => n,
            None => {
                reply.error(fuser::Errno::EINVAL);
                return;
            }
        };
        match fs.rename(parent.into(), name, newparent.into(), newname) {
            Ok(()) => reply.ok(),
            Err(e) => reply.error(errno_from_io(e)),
        }
    }

    fn create(
        &self,
        req: &Request,
        parent: INodeNo,
        name: &OsStr,
        mode: u32,
        _umask: u32,
        _flags: i32,
        reply: fuser::ReplyCreate,
    ) {
        let mut fs = self.fs.lock().unwrap();
        let name = match name.to_str() {
            Some(n) => n,
            None => {
                reply.error(fuser::Errno::EINVAL);
                return;
            }
        };
        match fs.create(parent.into(), name, mode, req.uid(), req.gid()) {
            Ok(inode) => {
                let attr = inode_to_attr(inode.ino, &inode);
                reply.created(&TTL, &attr, Generation(0), FileHandle(0), FopenFlags::empty());
            }
            Err(e) => reply.error(errno_from_io(e)),
        }
    }
}

#[allow(dead_code)]
const _: u32 = S_IFDIR | S_IFREG;

impl Drop for PolarisFuse {
    fn drop(&mut self) {
        if let Ok(mut fs) = self.fs.lock() {
            let _ = fs.sync_all();
        }
    }
}
