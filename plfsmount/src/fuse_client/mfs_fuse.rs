//! mfs_fuse — FUSE lowlevel operations: ANNOTATED UNSAFE BOUNDARY (P4).
//!
//! Every function in this module is a libfuse kernel callback: requests
//! arrive as raw fuse_req_t/fuse_ctx/buffer pointers, reply buffers are
//! borrowed from libfuse, and path/name arguments are C strings whose
//! lifetimes libfuse owns. The request lifecycle (fuse_reply_* exactly
//! once per request) is a C protocol with no safe Rust model short of a
//! full FUSE wrapper crate, so this module intentionally stays an unsafe
//! boundary (P1 sockets precedent). All cache/data-structure logic it
//! relies on HAS been migrated to safe Rust in the other P4 modules
//! (dirattrcache, symlinkcache, negentrycache, xattrcache, fdcache,
//! dentry_invalidator, sustained_*, oplog, dirblob_*_index).

pub enum fuse_session {}
pub enum fuse_req {}
pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use crate::src::fuse_client::dirattrcache as dcache;
use crate::src::fuse_client::dirbuf;
use crate::src::fuse_client::fdcache;
use crate::src::fuse_client::finfo as finfo_core;
use crate::src::fuse_client::getgroups;
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn fuse_reply_err(req: fuse_req_t, err: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn fuse_reply_entry(req: fuse_req_t, e: *const fuse_entry_param) -> ::core::ffi::c_int;
    unsafe fn fuse_reply_create(
        req: fuse_req_t,
        e: *const fuse_entry_param,
        fi: *const fuse_file_info,
    ) -> ::core::ffi::c_int;
    unsafe fn fuse_reply_attr(
        req: fuse_req_t,
        attr: *const stat,
        attr_timeout: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    unsafe fn fuse_reply_readlink(
        req: fuse_req_t,
        link: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn fuse_reply_open(req: fuse_req_t, fi: *const fuse_file_info) -> ::core::ffi::c_int;
    unsafe fn fuse_reply_write(req: fuse_req_t, count: size_t) -> ::core::ffi::c_int;
    unsafe fn fuse_reply_buf(
        req: fuse_req_t,
        buf: *const ::core::ffi::c_char,
        size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn fuse_reply_iov(
        req: fuse_req_t,
        iov: *const iovec,
        count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn fuse_reply_statfs(req: fuse_req_t, stbuf: *const statvfs) -> ::core::ffi::c_int;
    unsafe fn fuse_reply_xattr(req: fuse_req_t, count: size_t) -> ::core::ffi::c_int;
    unsafe fn fuse_reply_lock(req: fuse_req_t, lock: *const flock) -> ::core::ffi::c_int;
    unsafe fn fuse_add_direntry(
        req: fuse_req_t,
        buf: *mut ::core::ffi::c_char,
        bufsize: size_t,
        name: *const ::core::ffi::c_char,
        stbuf: *const stat,
        off: off_t,
    ) -> size_t;
    unsafe fn fuse_add_direntry_plus(
        req: fuse_req_t,
        buf: *mut ::core::ffi::c_char,
        bufsize: size_t,
        name: *const ::core::ffi::c_char,
        e: *const fuse_entry_param,
        off: off_t,
    ) -> size_t;
    unsafe fn fuse_lowlevel_notify_inval_inode(
        se: *mut fuse_session,
        ino: fuse_ino_t,
        off: off_t,
        len: off_t,
    ) -> ::core::ffi::c_int;
    unsafe fn fuse_lowlevel_notify_inval_entry(
        se: *mut fuse_session,
        parent: fuse_ino_t,
        name: *const ::core::ffi::c_char,
        namelen: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn fuse_req_ctx(req: fuse_req_t) -> *const fuse_ctx;
    unsafe fn fuse_req_interrupt_func(
        req: fuse_req_t,
        func: fuse_interrupt_func_t,
        data: *mut ::core::ffi::c_void,
    );
    unsafe fn fuse_req_interrupted(req: fuse_req_t) -> ::core::ffi::c_int;
    static mut stderr: *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strerror_r(
        __errnum: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_char,
        __buflen: size_t,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn getpid() -> __pid_t;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_key_create(
        __key: *mut pthread_key_t,
        __destr_function: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_getspecific(__key: pthread_key_t) -> *mut ::core::ffi::c_void;
    unsafe fn pthread_setspecific(
        __key: pthread_key_t,
        __pointer: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn oplog_printf(ctx: *const fuse_ctx, format: *const ::core::ffi::c_char, ...);
    unsafe fn oplog_newhandle(hflag: ::core::ffi::c_int) -> ::core::ffi::c_ulong;
    unsafe fn oplog_releasehandle(fh: ::core::ffi::c_ulong);
    unsafe fn oplog_getdata(
        fh: ::core::ffi::c_ulong,
        buff: *mut *mut uint8_t,
        leng: *mut uint32_t,
        maxleng: uint32_t,
    );
    unsafe fn oplog_releasedata(fh: ::core::ffi::c_ulong);
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
    unsafe fn monotonic_useconds() -> uint64_t;
    unsafe fn fs_getmasterlocation(loc: *mut uint8_t);
    unsafe fn fs_atime(inode: uint32_t);
    unsafe fn fs_mtime(inode: uint32_t);
    unsafe fn fs_no_atime(inode: uint32_t);
    unsafe fn fs_no_mtime(inode: uint32_t);
    unsafe fn fs_fix_amtime(inode: uint32_t, atime: *mut uint32_t, mtime: *mut uint32_t);
    unsafe fn fs_clr_working_flags(cflags: uint8_t);
    unsafe fn fs_isopen(inode: uint32_t) -> ::core::ffi::c_int;
    unsafe fn fs_inc_acnt(inode: uint32_t);
    unsafe fn fs_dec_acnt(inode: uint32_t);
    unsafe fn fs_read_notify(bytes: uint64_t);
    unsafe fn fs_write_notify(bytes: uint64_t);
    unsafe fn fs_fsync_notify();
    unsafe fn fs_statfs(
        totalspace: *mut uint64_t,
        availspace: *mut uint64_t,
        freespace: *mut uint64_t,
        trashspace: *mut uint64_t,
        sustainedspace: *mut uint64_t,
        inodes: *mut uint32_t,
    );
    unsafe fn fs_access(
        inode: uint32_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        modemask: uint16_t,
    ) -> uint8_t;
    unsafe fn fs_lookup(
        parent: uint32_t,
        nleng: uint8_t,
        name: *const uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        inode: *mut uint32_t,
        attr: *mut uint8_t,
        lflags: *mut uint16_t,
        csdataver: *mut uint8_t,
        chunkid: *mut uint64_t,
        version: *mut uint32_t,
        csdata: *mut *const uint8_t,
        csdatasize: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_getattr(
        inode: uint32_t,
        opened: uint8_t,
        uid: uint32_t,
        gid: uint32_t,
        attr: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_setattr(
        inode: uint32_t,
        opened: uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        setmask: uint8_t,
        attrmode: uint16_t,
        attruid: uint32_t,
        attrgid: uint32_t,
        attratime: uint32_t,
        attrmtime: uint32_t,
        winattr: uint8_t,
        sugidclearmode: uint8_t,
        attr: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_readlink(inode: uint32_t, path: *mut *const uint8_t) -> uint8_t;
    unsafe fn fs_symlink(
        parent: uint32_t,
        nleng: uint8_t,
        name: *const uint8_t,
        path: *const uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        inode: *mut uint32_t,
        attr: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_mknod(
        parent: uint32_t,
        nleng: uint8_t,
        name: *const uint8_t,
        r#type: uint8_t,
        mode: uint16_t,
        cumask: uint16_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        rdev: uint32_t,
        inode: *mut uint32_t,
        attr: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_mkdir(
        parent: uint32_t,
        nleng: uint8_t,
        name: *const uint8_t,
        mode: uint16_t,
        cumask: uint16_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        copysgid: uint8_t,
        inode: *mut uint32_t,
        attr: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_unlink(
        parent: uint32_t,
        nleng: uint8_t,
        name: *const uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        inode: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_rmdir(
        parent: uint32_t,
        nleng: uint8_t,
        name: *const uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        inode: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_rename(
        parent_src: uint32_t,
        nleng_src: uint8_t,
        name_src: *const uint8_t,
        parent_dst: uint32_t,
        nleng: uint8_t,
        name_dst: *const uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        mfsflags: uint8_t,
        inode: *mut uint32_t,
        attr: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_link(
        inode_src: uint32_t,
        parent_dst: uint32_t,
        nleng_dst: uint8_t,
        name_dst: *const uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        inode: *mut uint32_t,
        attr: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_readdir(
        inode: uint32_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        edgeid: *mut uint64_t,
        edgelimit: uint32_t,
        wantattr: uint8_t,
        addtocache: uint8_t,
        dbuff: *mut *const uint8_t,
        dbuffsize: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_create(
        parent: uint32_t,
        nleng: uint8_t,
        name: *const uint8_t,
        mode: uint16_t,
        cumask: uint16_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        inode: *mut uint32_t,
        attr: *mut uint8_t,
        oflags: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_opencheck(
        inode: uint32_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        flags: uint8_t,
        attr: *mut uint8_t,
        oflags: *mut uint8_t,
    ) -> uint8_t;
    unsafe fn fs_flock(inode: uint32_t, reqid: uint32_t, owner: uint64_t, cmd: uint8_t) -> uint8_t;
    unsafe fn fs_posixlock(
        inode: uint32_t,
        reqid: uint32_t,
        owner: uint64_t,
        cmd: uint8_t,
        r#type: uint8_t,
        start: uint64_t,
        end: uint64_t,
        pid: uint32_t,
        rtype: *mut uint8_t,
        rstart: *mut uint64_t,
        rend: *mut uint64_t,
        rpid: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_getfacl(
        inode: uint32_t,
        acltype: uint8_t,
        userperm: *mut uint16_t,
        groupperm: *mut uint16_t,
        otherperm: *mut uint16_t,
        maskperm: *mut uint16_t,
        namedusers: *mut uint16_t,
        namedgroups: *mut uint16_t,
        namedacls: *mut *const uint8_t,
        namedaclssize: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_setfacl(
        inode: uint32_t,
        uid: uint32_t,
        acltype: uint8_t,
        userperm: uint16_t,
        groupperm: uint16_t,
        otherperm: uint16_t,
        maskperm: uint16_t,
        namedusers: uint16_t,
        namedgroups: uint16_t,
        namedacls: *mut uint8_t,
        namedaclssize: uint32_t,
    ) -> uint8_t;
    unsafe fn fs_getxattr(
        inode: uint32_t,
        opened: uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        nleng: uint8_t,
        name: *const uint8_t,
        mode: uint8_t,
        vbuff: *mut *const uint8_t,
        vleng: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_listxattr(
        inode: uint32_t,
        opened: uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        mode: uint8_t,
        dbuff: *mut *const uint8_t,
        dleng: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_setxattr(
        inode: uint32_t,
        opened: uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        nleng: uint8_t,
        name: *const uint8_t,
        vleng: uint32_t,
        value: *const uint8_t,
        mode: uint8_t,
    ) -> uint8_t;
    unsafe fn fs_removexattr(
        inode: uint32_t,
        opened: uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        nleng: uint8_t,
        name: *const uint8_t,
    ) -> uint8_t;
    unsafe fn master_version() -> uint32_t;
    unsafe fn master_attrsize() -> uint8_t;
    unsafe fn masterproxy_getlocation(masterinfo: *mut uint8_t);
    unsafe fn read_data(
        vid: *mut ::core::ffi::c_void,
        offset: uint64_t,
        size: *mut uint32_t,
        rhead: *mut *mut ::core::ffi::c_void,
        iov: *mut *mut iovec,
        iovcnt: *mut uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn read_data_free_buff(
        vid: *mut ::core::ffi::c_void,
        vrhead: *mut ::core::ffi::c_void,
        iov: *mut iovec,
    );
    unsafe fn read_inode_clear_cache(inode: uint32_t, offset: uint64_t, leng: uint64_t);
    unsafe fn read_inode_set_length_active(inode: uint32_t, newlength: uint64_t);
    unsafe fn read_inode_set_length_passive(inode: uint32_t, newlength: uint64_t);
    unsafe fn read_data_new(inode: uint32_t, fleng: uint64_t) -> *mut ::core::ffi::c_void;
    unsafe fn read_data_end(vid: *mut ::core::ffi::c_void);
    unsafe fn write_data_new(inode: uint32_t, fleng: uint64_t) -> *mut ::core::ffi::c_void;
    unsafe fn write_data_end(vid: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    unsafe fn write_data_chunk_wait(vid: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    unsafe fn write_data_flush(vid: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    unsafe fn write_data_inode_setmaxfleng(inode: uint32_t, maxfleng: uint64_t);
    unsafe fn write_data_inode_getmaxfleng(inode: uint32_t) -> uint64_t;
    unsafe fn write_data_getmaxfleng(vid: *mut ::core::ffi::c_void) -> uint64_t;
    unsafe fn write_data_flush_inode(inode: uint32_t) -> ::core::ffi::c_int;
    unsafe fn write_data(
        vid: *mut ::core::ffi::c_void,
        offset: uint64_t,
        size: uint32_t,
        buff: *const uint8_t,
        superuser: uint8_t,
    ) -> ::core::ffi::c_int;
    unsafe fn write_cache_almost_full() -> uint8_t;
    unsafe fn do_truncate(
        inode: uint32_t,
        flags: uint8_t,
        uid: uint32_t,
        gids: uint32_t,
        gid: *mut uint32_t,
        attrlength: uint64_t,
        attr: *mut uint8_t,
        prevlength: *mut uint64_t,
    ) -> uint8_t;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn main_snprint_parameters(buff: *mut ::core::ffi::c_char, size: uint32_t) -> uint32_t;
    unsafe fn main_kernelversion() -> uint32_t;
    unsafe fn sstats_get(
        inode: uint32_t,
        attr: *mut uint8_t,
        forceok: uint8_t,
    ) -> ::core::ffi::c_int;
    unsafe fn sstats_set(inode: uint32_t, attr: *const uint8_t, createflag: uint8_t);
    unsafe fn sparents_add(inode: uint32_t, parent: uint32_t, timeout: uint32_t);
    unsafe fn symlink_cache_insert(inode: uint32_t, path: *const uint8_t);
    unsafe fn symlink_cache_search(inode: uint32_t) -> *mut uint8_t;
    unsafe fn negentry_cache_remove(inode: uint32_t, nleng: uint8_t, name: *const uint8_t);
    unsafe fn negentry_cache_insert(inode: uint32_t, nleng: uint8_t, name: *const uint8_t);
    unsafe fn negentry_cache_search(
        inode: uint32_t,
        nleng: uint8_t,
        name: *const uint8_t,
    ) -> uint8_t;
    unsafe fn xattr_cache_get(
        node: uint32_t,
        uid: uint32_t,
        gid: uint32_t,
        nleng: uint32_t,
        name: *const uint8_t,
        value: *mut *const uint8_t,
        vleng: *mut uint32_t,
        status: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn xattr_cache_set(
        node: uint32_t,
        uid: uint32_t,
        gid: uint32_t,
        nleng: uint32_t,
        name: *const uint8_t,
        value: *const uint8_t,
        vleng: uint32_t,
        status: ::core::ffi::c_int,
    );
    unsafe fn xattr_cache_del(node: uint32_t, nleng: uint32_t, name: *const uint8_t);
    unsafe fn xattr_cache_rel(vv: *mut ::core::ffi::c_void);
    unsafe fn xattr_cache_term();
    unsafe fn xattr_cache_init(timeout: ::core::ffi::c_double);
    unsafe fn dinval_add(parent: uint32_t, nleng: uint8_t, name: *const uint8_t, inode: uint32_t);
    unsafe fn dinval_remove(parent: uint32_t, nleng: uint8_t, name: *const uint8_t);
    unsafe fn dinval_init(timeout: ::core::ffi::c_double);
}
pub type __uint64_t = u64;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __fsblkcnt64_t = ::core::ffi::c_ulong;
pub type __fsfilcnt64_t = ::core::ffi::c_ulong;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type pthread_key_t = ::core::ffi::c_uint;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct fuse_file_info {
    pub flags: int32_t,
    #[bitfield(name = "writepage", ty = "uint32_t", bits = "0..=0")]
    #[bitfield(name = "direct_io", ty = "uint32_t", bits = "1..=1")]
    #[bitfield(name = "keep_cache", ty = "uint32_t", bits = "2..=2")]
    #[bitfield(name = "flush", ty = "uint32_t", bits = "3..=3")]
    #[bitfield(name = "nonseekable", ty = "uint32_t", bits = "4..=4")]
    #[bitfield(name = "flock_release", ty = "uint32_t", bits = "5..=5")]
    #[bitfield(name = "cache_readdir", ty = "uint32_t", bits = "6..=6")]
    #[bitfield(name = "noflush", ty = "uint32_t", bits = "7..=7")]
    #[bitfield(name = "parallel_direct_writes", ty = "uint32_t", bits = "8..=8")]
    #[bitfield(name = "padding", ty = "uint32_t", bits = "9..=31")]
    #[bitfield(name = "padding2", ty = "uint32_t", bits = "32..=63")]
    #[bitfield(name = "padding3", ty = "uint32_t", bits = "64..=95")]
    pub writepage_direct_io_keep_cache_flush_nonseekable_flock_release_cache_readdir_noflush_parallel_direct_writes_padding_padding2_padding3:
        [u8; 12],
    pub fh: uint64_t,
    pub lock_owner: uint64_t,
    pub poll_events: uint32_t,
    pub backing_id: int32_t,
    pub compat_flags: uint64_t,
    pub reserved: [uint64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flock {
    pub l_type: ::core::ffi::c_short,
    pub l_whence: ::core::ffi::c_short,
    pub l_start: __off64_t,
    pub l_len: __off64_t,
    pub l_pid: __pid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut ::core::ffi::c_void,
    pub iov_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statvfs {
    pub f_bsize: ::core::ffi::c_ulong,
    pub f_frsize: ::core::ffi::c_ulong,
    pub f_blocks: __fsblkcnt64_t,
    pub f_bfree: __fsblkcnt64_t,
    pub f_bavail: __fsblkcnt64_t,
    pub f_files: __fsfilcnt64_t,
    pub f_ffree: __fsfilcnt64_t,
    pub f_favail: __fsfilcnt64_t,
    pub f_fsid: ::core::ffi::c_ulong,
    pub f_flag: ::core::ffi::c_ulong,
    pub f_namemax: ::core::ffi::c_ulong,
    pub f_type: ::core::ffi::c_uint,
    pub __f_spare: [::core::ffi::c_int; 5],
}
pub type fuse_ino_t = uint64_t;
pub type fuse_req_t = *mut fuse_req;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_entry_param {
    pub ino: fuse_ino_t,
    pub generation: uint64_t,
    pub attr: stat,
    pub attr_timeout: ::core::ffi::c_double,
    pub entry_timeout: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_ctx {
    pub uid: uid_t,
    pub gid: gid_t,
    pub pid: pid_t,
    pub umask: mode_t,
}
pub type fuse_interrupt_func_t =
    Option<unsafe extern "C" fn(fuse_req_t, *mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused3: ::core::ffi::c_int,
    pub _total_written: __uint64_t,
    pub _unused2: [::core::ffi::c_char; 8],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type C2Rust_Unnamed_1 = ::core::ffi::c_uint;
pub const XATTR_REPLACE: C2Rust_Unnamed_1 = 2;
pub const XATTR_CREATE: C2Rust_Unnamed_1 = 1;
struct Sinfo {
    buff: Vec<u8>,
    reset: bool,
    valid: bool,
}
pub type C2Rust_Unnamed_3 = ::core::ffi::c_uint;
pub const STATNODES: C2Rust_Unnamed_3 = 42;
pub const OP_GETDIR_PLUS: C2Rust_Unnamed_3 = 41;
pub const OP_READDIRPLUS: C2Rust_Unnamed_3 = 40;
pub const OP_GETDIR_SMALL: C2Rust_Unnamed_3 = 39;
pub const OP_GETDIR_FULL: C2Rust_Unnamed_3 = 38;
pub const OP_REMOVEXATTR: C2Rust_Unnamed_3 = 37;
pub const OP_LISTXATTR: C2Rust_Unnamed_3 = 36;
pub const OP_GETXATTR: C2Rust_Unnamed_3 = 35;
pub const OP_SETXATTR: C2Rust_Unnamed_3 = 34;
pub const OP_SETLK: C2Rust_Unnamed_3 = 33;
pub const OP_GETLK: C2Rust_Unnamed_3 = 32;
pub const OP_FLOCK: C2Rust_Unnamed_3 = 31;
pub const OP_FSYNC: C2Rust_Unnamed_3 = 30;
pub const OP_FLUSH: C2Rust_Unnamed_3 = 29;
pub const OP_WRITE: C2Rust_Unnamed_3 = 28;
pub const OP_READ: C2Rust_Unnamed_3 = 27;
pub const OP_RELEASE: C2Rust_Unnamed_3 = 26;
pub const OP_OPEN: C2Rust_Unnamed_3 = 25;
pub const OP_CREATE: C2Rust_Unnamed_3 = 24;
pub const OP_RELEASEDIR: C2Rust_Unnamed_3 = 23;
pub const OP_READDIR: C2Rust_Unnamed_3 = 22;
pub const OP_OPENDIR: C2Rust_Unnamed_3 = 21;
pub const OP_LINK: C2Rust_Unnamed_3 = 20;
pub const OP_RENAME: C2Rust_Unnamed_3 = 19;
pub const OP_READLINK_CACHED: C2Rust_Unnamed_3 = 18;
pub const OP_READLINK_MASTER: C2Rust_Unnamed_3 = 17;
pub const OP_SYMLINK: C2Rust_Unnamed_3 = 16;
pub const OP_RMDIR: C2Rust_Unnamed_3 = 15;
pub const OP_MKDIR: C2Rust_Unnamed_3 = 14;
pub const OP_UNLINK: C2Rust_Unnamed_3 = 13;
pub const OP_MKNOD: C2Rust_Unnamed_3 = 12;
pub const OP_SETATTR: C2Rust_Unnamed_3 = 11;
pub const OP_DIRCACHE_GETATTR: C2Rust_Unnamed_3 = 10;
pub const OP_GETATTR: C2Rust_Unnamed_3 = 9;
pub const OP_NEGCACHE_LOOKUP: C2Rust_Unnamed_3 = 8;
pub const OP_DIRCACHE_LOOKUP: C2Rust_Unnamed_3 = 7;
pub const OP_LOOKUP_INTERNAL: C2Rust_Unnamed_3 = 6;
pub const OP_NEGLOOKUP: C2Rust_Unnamed_3 = 5;
pub const OP_POSLOOKUP: C2Rust_Unnamed_3 = 4;
pub const OP_ERRLOOKUP: C2Rust_Unnamed_3 = 3;
pub const OP_LOOKUP: C2Rust_Unnamed_3 = 2;
pub const OP_ACCESS: C2Rust_Unnamed_3 = 1;
pub const OP_STATFS: C2Rust_Unnamed_3 = 0;

fn lookup_cached_entry(
    ctx: &fuse_ctx,
    parent: u32,
    name: &[u8],
    inode: &mut u32,
    attr: &mut [u8; 36],
) -> bool {
    let Some((cached_inode, cached_attr)) = dcache::lookup(ctx.pid, ctx.uid, ctx.gid, parent, name)
    else {
        return false;
    };
    *inode = cached_inode;
    *attr = cached_attr;
    true
}

fn lookup_cached_attr(ctx: &fuse_ctx, inode: u32, attr: &mut [u8; 36]) -> bool {
    let Some(cached_attr) = dcache::getattr(ctx.pid, ctx.uid, ctx.gid, inode) else {
        return false;
    };
    *attr = cached_attr;
    true
}
pub struct FlockData {
    pub reqid: uint32_t,
    pub inode: uint32_t,
    pub owner: uint64_t,
}
pub struct PlockData {
    pub reqid: uint32_t,
    pub inode: uint32_t,
    pub owner: uint64_t,
    pub start: uint64_t,
    pub end: uint64_t,
    pub ctype: ::core::ffi::c_char,
}
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const __S_IFDIR: ::core::ffi::c_int = 0o40000 as ::core::ffi::c_int;
pub const __S_IFCHR: ::core::ffi::c_int = 0o20000 as ::core::ffi::c_int;
pub const __S_IFBLK: ::core::ffi::c_int = 0o60000 as ::core::ffi::c_int;
pub const __S_IFREG: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const __S_IFIFO: ::core::ffi::c_int = 0o10000 as ::core::ffi::c_int;
pub const __S_IFLNK: ::core::ffi::c_int = 0o120000 as ::core::ffi::c_int;
pub const __S_IFSOCK: ::core::ffi::c_int = 0o140000 as ::core::ffi::c_int;
pub const __S_ISUID: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const __S_ISGID: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const __S_ISVTX: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const O_ACCMODE: ::core::ffi::c_int = 0o3 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const O_TRUNC: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const O_APPEND: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const F_RDLCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const F_WRLCK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const F_UNLCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const S_IFMT: ::core::ffi::c_int = __S_IFMT;
pub const S_IFDIR: ::core::ffi::c_int = __S_IFDIR;
pub const S_IFCHR: ::core::ffi::c_int = __S_IFCHR;
pub const S_IFBLK: ::core::ffi::c_int = __S_IFBLK;
pub const S_IFREG: ::core::ffi::c_int = __S_IFREG;
pub const S_IFIFO: ::core::ffi::c_int = __S_IFIFO;
pub const S_IFLNK: ::core::ffi::c_int = __S_IFLNK;
pub const S_IFSOCK: ::core::ffi::c_int = __S_IFSOCK;
pub const S_ISUID: ::core::ffi::c_int = __S_ISUID;
pub const S_ISGID: ::core::ffi::c_int = __S_ISGID;
pub const S_ISVTX: ::core::ffi::c_int = __S_ISVTX;
pub const FUSE_ROOT_ID: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FUSE_SET_ATTR_MODE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const FUSE_SET_ATTR_UID: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const FUSE_SET_ATTR_GID: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const FUSE_SET_ATTR_SIZE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const FUSE_SET_ATTR_ATIME: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const FUSE_SET_ATTR_MTIME: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
pub const FUSE_SET_ATTR_ATIME_NOW: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int;
pub const FUSE_SET_ATTR_MTIME_NOW: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const FUSE_SET_ATTR_CTIME: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const RENAME_NOREPLACE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const RENAME_EXCHANGE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const ENOTSUP: ::core::ffi::c_int = EOPNOTSUPP;
pub const RELEASE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const VERSMAJ: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const VERSMID: ::core::ffi::c_int = 59 as ::core::ffi::c_int;
pub const VERSMIN: ::core::ffi::c_int = 2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EIO: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const ENXIO: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const EBADF: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const EEXIST: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const ENOTDIR: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const EFBIG: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const ENOSPC: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const EROFS: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const EMLINK: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const ENAMETOOLONG: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const ENOSYS: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const ENOTEMPTY: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
pub const ENODATA: ::core::ffi::c_int = 61 as ::core::ffi::c_int;
pub const EOPNOTSUPP: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const EDQUOT: ::core::ffi::c_int = 122 as ::core::ffi::c_int;
pub const ECANCELED: ::core::ffi::c_int = 125 as ::core::ffi::c_int;
pub const W_OK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const X_OK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSCHUNKSIZE: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;
pub const MFSBLOCKSIZE: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const MFS_NAME_MAX: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
pub const MFS_SYMLINK_MAX: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const MFS_MAX_FILE_SIZE: uint64_t = (MFSCHUNKSIZE as uint64_t) << 31 as ::core::ffi::c_int;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0;
pub const MFS_ERROR_EPERM: ::core::ffi::c_int = 1;
pub const MFS_ERROR_ENOTDIR: ::core::ffi::c_int = 2;
pub const MFS_ERROR_ENOENT: ::core::ffi::c_int = 3;
pub const MFS_ERROR_EACCES: ::core::ffi::c_int = 4;
pub const MFS_ERROR_EEXIST: ::core::ffi::c_int = 5;
pub const MFS_ERROR_EINVAL: ::core::ffi::c_int = 6;
pub const MFS_ERROR_ENOTEMPTY: ::core::ffi::c_int = 7;
pub const MFS_ERROR_CHUNKLOST: ::core::ffi::c_int = 8;
pub const MFS_ERROR_INDEXTOOBIG: ::core::ffi::c_int = 10;
pub const MFS_ERROR_NOCHUNKSERVERS: ::core::ffi::c_int = 12;
pub const MFS_ERROR_NOTOPENED: ::core::ffi::c_int = 17;
pub const MFS_ERROR_NOSPACE: ::core::ffi::c_int = 21;
pub const MFS_ERROR_IO: ::core::ffi::c_int = 22;
pub const MFS_ERROR_EROFS: ::core::ffi::c_int = 33;
pub const MFS_ERROR_QUOTA: ::core::ffi::c_int = 34;
pub const MFS_ERROR_ENOATTR: ::core::ffi::c_int = 38;
pub const MFS_ERROR_ENOTSUP: ::core::ffi::c_int = 39;
pub const MFS_ERROR_ERANGE: ::core::ffi::c_int = 40;
pub const MFS_ERROR_CSNOTPRESENT: ::core::ffi::c_int = 43;
pub const MFS_ERROR_EAGAIN: ::core::ffi::c_int = 45;
pub const MFS_ERROR_EINTR: ::core::ffi::c_int = 46;
pub const MFS_ERROR_ECANCELED: ::core::ffi::c_int = 47;
pub const MFS_ERROR_ENOENT_NOCACHE: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
pub const MFS_ERROR_ENAMETOOLONG: ::core::ffi::c_int = 58;
pub const MFS_ERROR_EMLINK: ::core::ffi::c_int = 59;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DISP_TYPE_FILE: ::core::ffi::c_int = 102;
pub const DISP_TYPE_DIRECTORY: ::core::ffi::c_int = 100;
pub const DISP_TYPE_SYMLINK: ::core::ffi::c_int = 108;
pub const DISP_TYPE_FIFO: ::core::ffi::c_int = 113;
pub const DISP_TYPE_BLOCKDEV: ::core::ffi::c_int = 98;
pub const DISP_TYPE_CHARDEV: ::core::ffi::c_int = 99;
pub const DISP_TYPE_SOCKET: ::core::ffi::c_int = 115;
pub const DISP_TYPE_TRASH: ::core::ffi::c_int = 116;
pub const DISP_TYPE_SUSTAINED: ::core::ffi::c_int = 114;
pub const TYPE_FILE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TYPE_DIRECTORY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TYPE_SYMLINK: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const TYPE_FIFO: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const TYPE_BLOCKDEV: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const TYPE_CHARDEV: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const TYPE_SOCKET: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const TYPE_TRASH: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const TYPE_SUSTAINED: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const MODE_MASK_R: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MODE_MASK_W: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LOOKUP_RO_FILESYSTEM: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const LOOKUP_KEEPCACHE: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const LOOKUP_IMMUTABLE: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const LOOKUP_DIRECTMODE: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const LOOKUP_APPENDONLY: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const SET_MODE_FLAG: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SET_UID_FLAG: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SET_GID_FLAG: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SET_MTIME_NOW_FLAG: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SET_MTIME_FLAG: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SET_ATIME_FLAG: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SET_ATIME_NOW_FLAG: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const MFS_RENAME_EXCHANGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_RENAME_NOREPLACE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FLOCK_UNLOCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FLOCK_TRY_SHARED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FLOCK_LOCK_SHARED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FLOCK_TRY_EXCLUSIVE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const FLOCK_LOCK_EXCLUSIVE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const FLOCK_INTERRUPT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const FLOCK_RELEASE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const POSIX_LOCK_CMD_GET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const POSIX_LOCK_CMD_SET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const POSIX_LOCK_CMD_TRY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const POSIX_LOCK_CMD_INT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const POSIX_LOCK_UNLCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const POSIX_LOCK_RDLCK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const POSIX_LOCK_WRLCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MATTR_NOACACHE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MATTR_NOECACHE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MATTR_ALLOWDATACACHE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const MATTR_NOXATTR: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const MATTR_DIRECTMODE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const MATTR_UNDELETABLE: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const POSIX_ACL_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const POSIX_ACL_ACCESS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const POSIX_ACL_DEFAULT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TRUNCATE_FLAG_OPENED: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const TRUNCATE_FLAG_UPDATE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const TRUNCATE_FLAG_RESERVE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SUGID_CLEAR_MODE_OPTIONS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const OPEN_READ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const OPEN_WRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const OPEN_AFTER_CREATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const OPEN_TRUNCATE: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const OPEN_CACHE_CLEARED: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const OPEN_KEEPCACHE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const OPEN_DIRECTMODE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const OPEN_APPENDONLY: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const MFS_XATTR_CREATE_OR_REPLACE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_XATTR_CREATE_ONLY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_XATTR_REPLACE_ONLY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFS_XATTR_GETA_DATA: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_XATTR_LENGTH_ONLY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_XATTR_NAME_MAX: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
pub const MFS_XATTR_SIZE_MAX: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
pub const WFLAG_INVALIDATE_CACHE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DISABLE_BIT_READDIR: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const DISABLE_BIT_READ: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const DISABLE_BIT_WRITE: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const DISABLE_READDIR: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_READDIR;
pub const DISABLE_READ: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_READ;
pub const DISABLE_WRITE: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_WRITE;
pub const LOCK_SH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LOCK_EX: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LOCK_UN: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const LOCK_NB: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn put64bit(mut ptr: *mut *mut uint8_t, mut val: uint64_t) {
    unsafe {
        val = val.swap_bytes() as uint64_t;
        memcpy(
            *ptr as *mut ::core::ffi::c_void,
            &raw mut val as *const ::core::ffi::c_void,
            8 as size_t,
        );
        *ptr = (*ptr).offset(8 as ::core::ffi::c_int as isize);
    }
}
#[inline]
unsafe extern "C" fn put32bit(mut ptr: *mut *mut uint8_t, mut val: uint32_t) {
    unsafe {
        val = val.swap_bytes() as uint32_t;
        memcpy(
            *ptr as *mut ::core::ffi::c_void,
            &raw mut val as *const ::core::ffi::c_void,
            4 as size_t,
        );
        *ptr = (*ptr).offset(4 as ::core::ffi::c_int as isize);
    }
}
#[inline]
unsafe extern "C" fn put16bit(mut ptr: *mut *mut uint8_t, mut val: uint16_t) {
    unsafe {
        val = val.swap_bytes() as uint16_t;
        memcpy(
            *ptr as *mut ::core::ffi::c_void,
            &raw mut val as *const ::core::ffi::c_void,
            2 as size_t,
        );
        *ptr = (*ptr).offset(2 as ::core::ffi::c_int as isize);
    }
}
#[inline]
unsafe extern "C" fn put8bit(mut ptr: *mut *mut uint8_t, mut val: uint8_t) {
    unsafe {
        *(*ptr).offset(0 as isize) =
            (val as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
        *ptr = (*ptr).offset(1);
    }
}
#[inline]
unsafe extern "C" fn get64bit(mut ptr: *mut *const uint8_t) -> uint64_t {
    unsafe {
        let mut t64: uint64_t = 0;
        memcpy(
            &raw mut t64 as *mut ::core::ffi::c_void,
            *ptr as *const ::core::ffi::c_void,
            8 as size_t,
        );
        *ptr = (*ptr).offset(8 as ::core::ffi::c_int as isize);
        return t64.swap_bytes();
    }
}
#[inline]
unsafe extern "C" fn get32bit(mut ptr: *mut *const uint8_t) -> uint32_t {
    unsafe {
        let mut t32: uint32_t = 0;
        memcpy(
            &raw mut t32 as *mut ::core::ffi::c_void,
            *ptr as *const ::core::ffi::c_void,
            4 as size_t,
        );
        *ptr = (*ptr).offset(4 as ::core::ffi::c_int as isize);
        return t32.swap_bytes();
    }
}
#[inline]
unsafe extern "C" fn get16bit(mut ptr: *mut *const uint8_t) -> uint16_t {
    unsafe {
        let mut t16: uint16_t = 0;
        memcpy(
            &raw mut t16 as *mut ::core::ffi::c_void,
            *ptr as *const ::core::ffi::c_void,
            2 as size_t,
        );
        *ptr = (*ptr).offset(2 as ::core::ffi::c_int as isize);
        return t16.swap_bytes();
    }
}
#[inline]
unsafe extern "C" fn get8bit(mut ptr: *mut *const uint8_t) -> uint8_t {
    unsafe {
        let mut t8: uint8_t = 0;
        t8 = *(*ptr).offset(0 as isize);
        *ptr = (*ptr).offset(1);
        return t8;
    }
}
#[inline]
unsafe extern "C" fn portable_usleep(mut usec: uint64_t) {
    unsafe {
        let mut req: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let mut rem: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let mut s: ::core::ffi::c_int = 0;
        req.tv_sec = usec.wrapping_div(1000000 as uint64_t) as __time_t;
        req.tv_nsec = usec
            .wrapping_rem(1000000 as uint64_t)
            .wrapping_mul(1000 as uint64_t) as __syscall_slong_t;
        loop {
            s = nanosleep(&raw mut req, &raw mut rem);
            if s < 0 as ::core::ffi::c_int {
                req = rem;
            }
            if s >= 0 as ::core::ffi::c_int {
                break;
            }
        }
    }
}
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn mfsrealloc(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pptr: *mut ::core::ffi::c_void = realloc(ptr, size);
        if pptr.is_null() {
            free(ptr);
        }
        return pptr;
    }
}
pub const READDIR_BUFFSIZE: ::core::ffi::c_int = 50000 as ::core::ffi::c_int;
pub const READDIR_EDGELIMIT: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const MAX_FILE_SIZE: int64_t = MFS_MAX_FILE_SIZE as int64_t;
pub const PKGVERSION: ::core::ffi::c_int = VERSMAJ * 1000000 as ::core::ffi::c_int
    + VERSMID * 10000 as ::core::ffi::c_int
    + (VERSMIN >> 1 as ::core::ffi::c_int) * 100 as ::core::ffi::c_int
    + RELEASE;
pub const MASTERINFO_NAME: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b".masterinfo\0") };
pub const MASTERINFO_INODE: ::core::ffi::c_int = 0x7fffffff as ::core::ffi::c_int;
static mut masterinfoattr: [uint8_t; 36] = [
    0 as uint8_t,
    (TYPE_FILE << 4 as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int) as uint8_t,
    0x24 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    1 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    22 as uint8_t,
    0 as uint8_t,
];
pub const STATS_NAME: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b".stats\0") };
pub const STATS_INODE: ::core::ffi::c_int = 0x7ffffff0 as ::core::ffi::c_int;
static mut statsattr: [uint8_t; 36] = [
    0 as uint8_t,
    (TYPE_FILE << 4 as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int) as uint8_t,
    0xa4 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    1 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
];
pub const OPLOG_NAME: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b".oplog\0") };
pub const OPLOG_INODE: ::core::ffi::c_int = 0x7ffffff1 as ::core::ffi::c_int;
pub const OPHISTORY_NAME: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b".ophistory\0") };
pub const OPHISTORY_INODE: ::core::ffi::c_int = 0x7ffffff2 as ::core::ffi::c_int;
static mut oplogattr: [uint8_t; 36] = [
    0 as uint8_t,
    (TYPE_FILE << 4 as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int) as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    1 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
];
pub const MOOSE_NAME: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b".mooseart\0") };
pub const MOOSE_INODE: ::core::ffi::c_int = 0x7ffffff3 as ::core::ffi::c_int;
static mut mooseattr: [uint8_t; 36] = [
    0 as uint8_t,
    (TYPE_FILE << 4 as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int) as uint8_t,
    0xa4 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    1 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
];
pub const RANDOM_NAME: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b".random\0") };
pub const RANDOM_INODE: ::core::ffi::c_int = 0x7ffffff4 as ::core::ffi::c_int;
static mut randomattr: [uint8_t; 36] = [
    0 as uint8_t,
    (TYPE_FILE << 4 as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int) as uint8_t,
    0x24 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    1 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
];
pub const PARAMS_NAME: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b".params\0") };
pub const PARAMS_INODE: ::core::ffi::c_int = 0x7ffffff5 as ::core::ffi::c_int;
static mut paramsattr: [uint8_t; 36] = [
    0 as uint8_t,
    (TYPE_FILE << 4 as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int) as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    1 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
];
pub const PARAMS_BUFFSIZE: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
pub const IO_RW: u32 = 0;
pub const IO_RO: u32 = 1;
pub const IO_RA: u32 = 2;
pub const MIN_SPECIAL_INODE: ::core::ffi::c_int = 0x7ffffff0 as ::core::ffi::c_int;
static mut dinval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut rndz: uint32_t = 362436069 as uint32_t;
static mut rndw: uint32_t = 521288629 as uint32_t;
static mut rndjsr: uint32_t = 123456789 as uint32_t;
static mut rndjcong: uint32_t = 380116160 as uint32_t;
static RANDOM_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
static SINFO: std::sync::Mutex<Vec<Option<std::sync::Arc<std::sync::Mutex<Sinfo>>>>> =
    std::sync::Mutex::new(Vec::new());

fn sinfo_new() -> uint32_t {
    let mut table = SINFO.lock().unwrap();
    let entry = std::sync::Arc::new(std::sync::Mutex::new(Sinfo {
        buff: Vec::new(),
        reset: false,
        valid: true,
    }));
    if let Some((index, slot)) = table
        .iter_mut()
        .enumerate()
        .find(|(_, slot)| slot.is_none())
    {
        *slot = Some(entry);
        index as uint32_t + 1
    } else {
        table.push(Some(entry));
        table.len() as uint32_t
    }
}

fn sinfo_get(index: uint32_t) -> Option<std::sync::Arc<std::sync::Mutex<Sinfo>>> {
    if index == 0 {
        return None;
    }
    SINFO
        .lock()
        .unwrap()
        .get(index as usize - 1)
        .and_then(Option::clone)
}

fn sinfo_release(index: uint32_t) {
    if index == 0 {
        return;
    }
    if let Some(slot) = SINFO.lock().unwrap().get_mut(index as usize - 1) {
        *slot = None;
    }
}

fn sinfo_freeall() {
    SINFO.lock().unwrap().clear();
}

// Opaque readdata/writedata ownership remains an FFI boundary. Backend calls stay here.
struct ReadDataHandle(std::ptr::NonNull<::core::ffi::c_void>);

impl ReadDataHandle {
    unsafe fn new(inode: u32, length: u64) -> Option<Self> {
        unsafe { std::ptr::NonNull::new(read_data_new(inode, length)).map(Self) }
    }

    fn as_ptr(&self) -> *mut ::core::ffi::c_void {
        self.0.as_ptr()
    }
}

// SAFETY: ownership moves only under FileInfo's mutex; active backend use is
// bracketed by the inode IO gate, which release drains before final Drop.
unsafe impl Send for ReadDataHandle {}

impl Drop for ReadDataHandle {
    fn drop(&mut self) {
        unsafe { read_data_end(self.as_ptr()) }
    }
}

struct WriteDataHandle(std::ptr::NonNull<::core::ffi::c_void>);

impl WriteDataHandle {
    unsafe fn new(inode: u32, length: u64) -> Option<Self> {
        unsafe { std::ptr::NonNull::new(write_data_new(inode, length)).map(Self) }
    }

    fn as_ptr(&self) -> *mut ::core::ffi::c_void {
        self.0.as_ptr()
    }
}

// SAFETY: ownership moves only under FileInfo's mutex; active backend use is
// bracketed by the inode IO gate, which release drains before final Drop.
unsafe impl Send for WriteDataHandle {}

impl Drop for WriteDataHandle {
    fn drop(&mut self) {
        unsafe {
            write_data_end(self.as_ptr());
        }
    }
}

type FileInfo = finfo_core::FileInfo<ReadDataHandle, WriteDataHandle>;
type FileRegistry = finfo_core::Registry<ReadDataHandle, WriteDataHandle>;

fn file_registry() -> &'static std::sync::Mutex<FileRegistry> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<FileRegistry>> =
        std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(FileRegistry::default()))
}

fn fileinfo_get(handle: u64) -> Option<std::sync::Arc<FileInfo>> {
    let handle = finfo_core::FileHandle::from_fuse(handle)?;
    file_registry().lock().unwrap().get(handle)
}

unsafe fn mfs_newfileinfo(
    accmode: uint8_t,
    inode: uint32_t,
    length: uint64_t,
    open_in_master: uint8_t,
    append_only: uint8_t,
) -> uint32_t {
    let length_handle = plfsclient::inoleng::acquire(inode);
    plfsclient::inoleng::set_length(&length_handle, length);
    let mode = if accmode as ::core::ffi::c_int == O_RDONLY {
        finfo_core::IoMode::ReadOnly
    } else if append_only != 0 {
        finfo_core::IoMode::ReadAppend
    } else {
        finfo_core::IoMode::ReadWrite
    };
    let file = FileInfo::new(
        inode,
        Some(length_handle),
        unsafe { monotonic_seconds() },
        mode,
        open_in_master != 0,
    );
    file_registry().lock().unwrap().insert(file).get()
}

fn mfs_removefileinfo(handle: u64) {
    let Some(handle) = finfo_core::FileHandle::from_fuse(handle) else {
        return;
    };
    let Some(removed) = file_registry().lock().unwrap().remove(handle) else {
        return;
    };
    // Backend resources belong to FileInfo and drop with its final in-flight Arc.
    let file = {
        let mut registry = file_registry().lock().unwrap();
        registry.recycle(removed)
    };
    drop(file);
}

fn finfo_freeall() {
    // FUSE session teardown is quiescent; any defensive in-flight Arc still
    // keeps its backend resources alive until that callback returns.
    let files = { file_registry().lock().unwrap().take_all() };
    drop(files);
}

// Directory buffers live in the safe `dirbuf` module.
unsafe extern "C" fn finfo_change_fleng(mut inode: uint32_t, mut fleng: uint64_t) {
    unsafe {
        plfsclient::inoleng::update_length(inode, fleng);
    }
}
static mut fuse_comm: *mut fuse_session = ::core::ptr::null_mut::<fuse_session>();
static mut debug_mode: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut usedircache: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut keep_cache: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut readdirplus_cache_min_timeout: ::core::ffi::c_double = 0.0001f64;
static mut direntry_cache_timeout: ::core::ffi::c_double = 0.1f64;
static mut entry_cache_timeout: ::core::ffi::c_double = 0.0f64;
static mut attr_cache_timeout: ::core::ffi::c_double = 0.1f64;
static mut mkdir_copy_sgid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut sugid_clear_mode: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut xattr_cache_on: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut xattr_acl_support: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut fsync_before_close_min_time: ::core::ffi::c_double = 10.0f64;
static mut no_xattrs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut no_posix_locks: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut no_bsd_locks: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut full_permissions: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut mfs_disables: uint32_t = 0 as uint32_t;
const MFS_STATS_LEN: usize = STATNODES as usize;
static STATS: std::sync::OnceLock<[Option<plfsclient::stats::StatsHandle>; MFS_STATS_LEN]> =
    std::sync::OnceLock::new();

fn set_stat(
    stats: &mut [Option<plfsclient::stats::StatsHandle>; MFS_STATS_LEN],
    id: u32,
    parent: &plfsclient::stats::StatsHandle,
    name: &str,
) {
    stats[id as usize] = Some(plfsclient::stats::subnode(Some(parent), name, false, true));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_statsptr_init() {
    let root = plfsclient::stats::subnode(None, "fuse_ops", false, true);
    let mut stats: [Option<plfsclient::stats::StatsHandle>; MFS_STATS_LEN] =
        std::array::from_fn(|_| None);
    for (id, name) in [
        (OP_SETXATTR, "setxattr"),
        (OP_GETXATTR, "getxattr"),
        (OP_LISTXATTR, "listxattr"),
        (OP_REMOVEXATTR, "removexattr"),
        (OP_FLOCK, "flock"),
        (OP_GETLK, "getlk"),
        (OP_SETLK, "setlk"),
        (OP_FSYNC, "fsync"),
        (OP_FLUSH, "flush"),
        (OP_WRITE, "write"),
        (OP_READ, "read"),
        (OP_RELEASE, "release"),
        (OP_OPEN, "open"),
        (OP_CREATE, "create"),
        (OP_RELEASEDIR, "releasedir"),
        (OP_READDIR, "readdir"),
        (OP_READDIRPLUS, "readdirplus"),
        (OP_OPENDIR, "opendir"),
        (OP_LINK, "link"),
        (OP_RENAME, "rename"),
        (OP_SYMLINK, "symlink"),
        (OP_RMDIR, "rmdir"),
        (OP_MKDIR, "mkdir"),
        (OP_UNLINK, "unlink"),
        (OP_MKNOD, "mknod"),
        (OP_SETATTR, "setattr"),
        (OP_GETATTR, "getattr"),
        (OP_DIRCACHE_GETATTR, "getattr-cached"),
        (OP_ACCESS, "access"),
        (OP_STATFS, "statfs"),
    ] {
        set_stat(&mut stats, id, &root, name);
    }

    let readlink = plfsclient::stats::subnode(Some(&root), "readlink", false, true);
    set_stat(&mut stats, OP_READLINK_MASTER, &readlink, "master");
    set_stat(&mut stats, OP_READLINK_CACHED, &readlink, "cached");

    let lookup = plfsclient::stats::subnode(Some(&root), "lookup", false, true);
    let cached = plfsclient::stats::subnode(Some(&lookup), "cached", false, true);
    let master = plfsclient::stats::subnode(Some(&lookup), "master", false, true);
    set_stat(&mut stats, OP_LOOKUP_INTERNAL, &lookup, "internal");
    set_stat(&mut stats, OP_POSLOOKUP, &master, "positive");
    set_stat(&mut stats, OP_NEGLOOKUP, &master, "negative");
    set_stat(&mut stats, OP_ERRLOOKUP, &master, "error");
    if usedircache != 0 {
        set_stat(&mut stats, OP_DIRCACHE_LOOKUP, &cached, "readdir");
    }
    set_stat(&mut stats, OP_NEGCACHE_LOOKUP, &cached, "negative");

    let readdir = plfsclient::stats::subnode(Some(&root), "readdir", false, true);
    if usedircache != 0 {
        set_stat(&mut stats, OP_GETDIR_FULL, &readdir, "with_attrs");
    }
    set_stat(&mut stats, OP_GETDIR_SMALL, &readdir, "without_attrs");
    set_stat(&mut stats, OP_GETDIR_PLUS, &readdir, "with_attrs+");

    assert!(STATS.set(stats).is_ok(), "FUSE stats initialized twice");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_stats_inc(id: uint8_t) {
    if let Some(node) = STATS
        .get()
        .and_then(|stats| stats.get(id as usize))
        .and_then(Option::as_ref)
    {
        plfsclient::stats::counter_inc(node);
    }
}
static mut aclstorage: pthread_key_t = 0;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_aclstorage_free(mut ptr: *mut ::core::ffi::c_void) {
    unsafe {
        if !ptr.is_null() {
            free(ptr);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_aclstorage_init() {
    unsafe {
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_key_create(
            &raw mut aclstorage,
            Some(mfs_aclstorage_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        );
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfs_fuse.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    802 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_key_create(&aclstorage,mfs_aclstorage_free)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfs_fuse.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    802 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_key_create(&aclstorage,mfs_aclstorage_free)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfs_fuse.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    802 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_key_create(&aclstorage,mfs_aclstorage_free)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfs_fuse.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    802 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_key_create(&aclstorage,mfs_aclstorage_free)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfs_fuse.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    802 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_key_create(&aclstorage,mfs_aclstorage_free)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfs_fuse.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    802 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_key_create(&aclstorage,mfs_aclstorage_free)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_0: ::core::ffi::c_int =
            pthread_setspecific(aclstorage, ::core::ptr::null::<::core::ffi::c_void>());
        if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfs_fuse.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    803 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_setspecific(aclstorage,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfs_fuse.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    803 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_setspecific(aclstorage,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
            } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_2: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfs_fuse.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    803 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_setspecific(aclstorage,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfs_fuse.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    803 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_setspecific(aclstorage,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
            } else {
                let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfs_fuse.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    803 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_setspecific(aclstorage,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfs_fuse.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    803 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_setspecific(aclstorage,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_aclstorage_get(mut size: uint32_t) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut buff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut p: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut cp: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut s: uint32_t = 0;
        buff = pthread_getspecific(aclstorage) as *mut uint8_t;
        if !buff.is_null() {
            p = buff;
            cp = p;
            s = get32bit(&raw mut cp);
            if size <= s {
                return buff.offset(4 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void;
            }
            free(buff as *mut ::core::ffi::c_void);
        }
        buff = malloc(size.wrapping_add(4 as uint32_t) as size_t) as *mut uint8_t;
        if buff.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfs_fuse.c\0".as_ptr() as *const ::core::ffi::c_char,
                820 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"buff\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfs_fuse.c\0".as_ptr() as *const ::core::ffi::c_char,
                820 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"buff\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if buff
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut uint8_t
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfs_fuse.c\0".as_ptr() as *const ::core::ffi::c_char,
                820 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfs_fuse.c\0".as_ptr() as *const ::core::ffi::c_char,
                820 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        p = buff;
        put32bit(&raw mut p, size);
        let mut _mfs_assert_ret: ::core::ffi::c_int =
            pthread_setspecific(aclstorage, buff as *const ::core::ffi::c_void);
        if _mfs_assert_ret != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfs_fuse.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    823 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_setspecific(aclstorage,buff)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfs_fuse.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    823 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_setspecific(aclstorage,buff)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring_0,
                );
            } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfs_fuse.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    823 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_setspecific(aclstorage,buff)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfs_fuse.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    823 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_setspecific(aclstorage,buff)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_1,
                );
            } else {
                let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfs_fuse.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    823 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_setspecific(aclstorage,buff)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfs_fuse.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    823 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_setspecific(aclstorage,buff)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        return p as *mut ::core::ffi::c_void;
    }
}
pub const ENOATTR: ::core::ffi::c_int = ENODATA;
unsafe extern "C" fn mfs_errorconv(mut status: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        let mut ret: ::core::ffi::c_int = 0;
        match status {
            MFS_STATUS_OK => {
                ret = 0 as ::core::ffi::c_int;
            }
            MFS_ERROR_EPERM => {
                ret = EPERM;
            }
            MFS_ERROR_ENOTDIR => {
                ret = ENOTDIR;
            }
            MFS_ERROR_ENOENT => {
                ret = ENOENT;
            }
            MFS_ERROR_EACCES => {
                ret = EACCES;
            }
            MFS_ERROR_EEXIST => {
                ret = EEXIST;
            }
            MFS_ERROR_EINVAL => {
                ret = EINVAL;
            }
            MFS_ERROR_ENOTEMPTY => {
                ret = ENOTEMPTY;
            }
            MFS_ERROR_IO => {
                ret = EIO;
            }
            MFS_ERROR_EROFS => {
                ret = EROFS;
            }
            MFS_ERROR_EINTR => {
                ret = EINTR;
            }
            MFS_ERROR_EAGAIN => {
                ret = EAGAIN;
            }
            MFS_ERROR_ECANCELED => {
                ret = ECANCELED;
            }
            MFS_ERROR_QUOTA => {
                ret = EDQUOT;
            }
            MFS_ERROR_ENOATTR => {
                ret = ENOATTR;
            }
            MFS_ERROR_ENOTSUP => {
                ret = ENOTSUP;
            }
            MFS_ERROR_ERANGE => {
                ret = ERANGE;
            }
            MFS_ERROR_NOSPACE => {
                ret = ENOSPC;
            }
            MFS_ERROR_CHUNKLOST => {
                ret = ENXIO;
            }
            MFS_ERROR_NOCHUNKSERVERS => {
                ret = ENOSPC;
            }
            MFS_ERROR_CSNOTPRESENT => {
                ret = ENXIO;
            }
            MFS_ERROR_NOTOPENED => {
                ret = EBADF;
            }
            MFS_ERROR_INDEXTOOBIG => {
                ret = EFBIG;
            }
            MFS_ERROR_ENAMETOOLONG => {
                ret = ENAMETOOLONG;
            }
            MFS_ERROR_EMLINK => {
                ret = EMLINK;
            }
            _ => {
                ret = EINVAL;
            }
        }
        if debug_mode != 0 && ret != 0 as ::core::ffi::c_int {
            let mut errorbuff: [::core::ffi::c_char; 500] = [0; 500];
            fprintf(
                stderr,
                b"status: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                strerror_r(
                    ret,
                    &raw mut errorbuff as *mut ::core::ffi::c_char,
                    500 as size_t,
                ),
            );
        }
        return ret;
    }
}
#[inline]
unsafe extern "C" fn mfs_fix_amtime(
    mut inode: uint32_t,
    mut atime: *mut time_t,
    mut mtime: *mut time_t,
) {
    unsafe {
        let mut at: uint32_t = 0;
        let mut mt: uint32_t = 0;
        at = *atime as uint32_t;
        mt = *mtime as uint32_t;
        fs_fix_amtime(inode, &raw mut at, &raw mut mt);
        *atime = at as time_t;
        *mtime = mt as time_t;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_type_convert(mut r#type: uint8_t) -> uint8_t {
    match r#type as ::core::ffi::c_int {
        DISP_TYPE_FILE => return TYPE_FILE as uint8_t,
        DISP_TYPE_DIRECTORY => return TYPE_DIRECTORY as uint8_t,
        DISP_TYPE_SYMLINK => return TYPE_SYMLINK as uint8_t,
        DISP_TYPE_FIFO => return TYPE_FIFO as uint8_t,
        DISP_TYPE_BLOCKDEV => return TYPE_BLOCKDEV as uint8_t,
        DISP_TYPE_CHARDEV => return TYPE_CHARDEV as uint8_t,
        DISP_TYPE_SOCKET => return TYPE_SOCKET as uint8_t,
        DISP_TYPE_TRASH => return TYPE_TRASH as uint8_t,
        DISP_TYPE_SUSTAINED => return TYPE_SUSTAINED as uint8_t,
        _ => {}
    }
    return 0 as uint8_t;
}
unsafe extern "C" fn mfs_type_to_stat(
    mut inode: uint32_t,
    mut r#type: uint8_t,
    mut stbuf: *mut stat,
) {
    unsafe {
        memset(
            stbuf as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<stat>(),
        );
        (*stbuf).st_ino = inode as __ino_t;
        match r#type as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int {
            DISP_TYPE_DIRECTORY | TYPE_DIRECTORY => {
                (*stbuf).st_mode = S_IFDIR as __mode_t;
            }
            DISP_TYPE_SYMLINK | TYPE_SYMLINK => {
                (*stbuf).st_mode = S_IFLNK as __mode_t;
            }
            DISP_TYPE_FILE | TYPE_FILE => {
                (*stbuf).st_mode = S_IFREG as __mode_t;
            }
            DISP_TYPE_FIFO | TYPE_FIFO => {
                (*stbuf).st_mode = S_IFIFO as __mode_t;
            }
            DISP_TYPE_SOCKET | TYPE_SOCKET => {
                (*stbuf).st_mode = S_IFSOCK as __mode_t;
            }
            DISP_TYPE_BLOCKDEV | TYPE_BLOCKDEV => {
                (*stbuf).st_mode = S_IFBLK as __mode_t;
            }
            DISP_TYPE_CHARDEV | TYPE_CHARDEV => {
                (*stbuf).st_mode = S_IFCHR as __mode_t;
            }
            _ => {
                (*stbuf).st_mode = 0 as __mode_t;
            }
        };
    }
}
#[inline]
unsafe extern "C" fn mfs_attr_get_type(mut attr: *const uint8_t) -> uint8_t {
    unsafe {
        if (*attr.offset(0 as isize) as ::core::ffi::c_int) < 64 as ::core::ffi::c_int {
            return (*attr.offset(1 as isize) as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
                as uint8_t;
        } else {
            return fsnodes_type_convert(
                (*attr.offset(0 as isize) as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int)
                    as uint8_t,
            );
        };
    }
}
#[inline]
unsafe extern "C" fn mfs_attr_get_mattr(mut attr: *const uint8_t) -> uint8_t {
    unsafe {
        if (*attr.offset(0 as isize) as ::core::ffi::c_int) < 64 as ::core::ffi::c_int {
            return *attr.offset(0 as isize);
        } else {
            return (*attr.offset(1 as isize) as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
                as uint8_t;
        };
    }
}
#[inline]
unsafe extern "C" fn mfs_attr_get_fleng(mut attr: *const uint8_t) -> uint64_t {
    unsafe {
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        ptr = attr.offset(27 as ::core::ffi::c_int as isize) as *const uint8_t;
        return get64bit(&raw mut ptr);
    }
}
#[inline]
unsafe extern "C" fn mfs_attr_set_fleng(mut attr: *mut uint8_t, mut fleng: uint64_t) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        ptr = attr.offset(27 as ::core::ffi::c_int as isize) as *mut uint8_t;
        put64bit(&raw mut ptr, fleng);
    }
}
unsafe extern "C" fn mfs_attr_modify(
    mut to_set: uint32_t,
    mut attr: *mut uint8_t,
    mut stbuf: *mut stat,
) {
    unsafe {
        let mut mattr: uint8_t = 0;
        let mut attrmode: uint16_t = 0;
        let mut attrtype: uint8_t = 0;
        let mut attruid: uint32_t = 0;
        let mut attrgid: uint32_t = 0;
        let mut attratime: uint32_t = 0;
        let mut attrmtime: uint32_t = 0;
        let mut attrctime: uint32_t = 0;
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        ptr = attr as *const uint8_t;
        if (*attr.offset(0 as isize) as ::core::ffi::c_int) < 64 as ::core::ffi::c_int {
            mattr = get8bit(&raw mut ptr);
            attrmode = get16bit(&raw mut ptr);
            attrtype = (attrmode as ::core::ffi::c_int >> 12 as ::core::ffi::c_int) as uint8_t;
        } else {
            attrtype = get8bit(&raw mut ptr);
            attrtype = fsnodes_type_convert(
                (attrtype as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int) as uint8_t,
            );
            attrmode = get16bit(&raw mut ptr);
            mattr = (attrmode as ::core::ffi::c_int >> 12 as ::core::ffi::c_int) as uint8_t;
        }
        attrmode = (attrmode as ::core::ffi::c_int & 0xfff as ::core::ffi::c_int) as uint16_t;
        attruid = get32bit(&raw mut ptr);
        attrgid = get32bit(&raw mut ptr);
        attratime = get32bit(&raw mut ptr);
        attrmtime = get32bit(&raw mut ptr);
        attrctime = get32bit(&raw mut ptr);
        if to_set & FUSE_SET_ATTR_MODE as uint32_t != 0 {
            attrmode = ((*stbuf).st_mode & 0o7777 as __mode_t) as uint16_t;
            attrctime = time(::core::ptr::null_mut::<time_t>()) as uint32_t;
        }
        if to_set & FUSE_SET_ATTR_UID as uint32_t != 0 {
            attruid = (*stbuf).st_uid as uint32_t;
            attrctime = time(::core::ptr::null_mut::<time_t>()) as uint32_t;
        }
        if to_set & FUSE_SET_ATTR_GID as uint32_t != 0 {
            attrgid = (*stbuf).st_gid as uint32_t;
            attrctime = time(::core::ptr::null_mut::<time_t>()) as uint32_t;
        }
        if to_set & FUSE_SET_ATTR_ATIME as uint32_t != 0 {
            attratime = (*stbuf).st_atim.tv_sec as uint32_t;
            attrctime = time(::core::ptr::null_mut::<time_t>()) as uint32_t;
        }
        if to_set & FUSE_SET_ATTR_MTIME as uint32_t != 0 {
            attratime = (*stbuf).st_mtim.tv_sec as uint32_t;
            attrctime = time(::core::ptr::null_mut::<time_t>()) as uint32_t;
        }
        wptr = attr as *mut uint8_t;
        put8bit(&raw mut wptr, mattr);
        attrmode = (attrmode as ::core::ffi::c_int
            | (attrtype as uint16_t as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
            as uint16_t;
        put16bit(&raw mut wptr, attrmode);
        put32bit(&raw mut wptr, attruid);
        put32bit(&raw mut wptr, attrgid);
        put32bit(&raw mut wptr, attratime);
        put32bit(&raw mut wptr, attrmtime);
        put32bit(&raw mut wptr, attrctime);
    }
}
unsafe extern "C" fn mfs_attr_to_stat(
    mut inode: uint32_t,
    mut attr: *const uint8_t,
    mut stbuf: *mut stat,
) {
    unsafe {
        let mut attrmode: uint16_t = 0;
        let mut attrtype: uint8_t = 0;
        let mut attruid: uint32_t = 0;
        let mut attrgid: uint32_t = 0;
        let mut attratime: uint32_t = 0;
        let mut attrmtime: uint32_t = 0;
        let mut attrctime: uint32_t = 0;
        let mut attrnlink: uint32_t = 0;
        let mut attrrdev: uint32_t = 0;
        let mut attrlength: uint64_t = 0;
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        ptr = attr as *const uint8_t;
        if (*attr.offset(0 as isize) as ::core::ffi::c_int) < 64 as ::core::ffi::c_int {
            ptr = ptr.offset(1);
            attrmode = get16bit(&raw mut ptr);
            attrtype = (attrmode as ::core::ffi::c_int >> 12 as ::core::ffi::c_int) as uint8_t;
        } else {
            attrtype = get8bit(&raw mut ptr);
            attrtype = fsnodes_type_convert(
                (attrtype as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int) as uint8_t,
            );
            attrmode = get16bit(&raw mut ptr);
        }
        attrmode = (attrmode as ::core::ffi::c_int & 0xfff as ::core::ffi::c_int) as uint16_t;
        attruid = get32bit(&raw mut ptr);
        attrgid = get32bit(&raw mut ptr);
        attratime = get32bit(&raw mut ptr);
        attrmtime = get32bit(&raw mut ptr);
        attrctime = get32bit(&raw mut ptr);
        attrnlink = get32bit(&raw mut ptr);
        (*stbuf).st_ino = inode as __ino_t;
        (*stbuf).st_blksize = MFSBLOCKSIZE as __blksize_t;
        match attrtype as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int {
            TYPE_DIRECTORY => {
                (*stbuf).st_mode = (S_IFDIR | attrmode as ::core::ffi::c_int) as __mode_t;
                attrlength = get64bit(&raw mut ptr);
                (*stbuf).st_size = attrlength as __off_t;
                (*stbuf).st_blocks = attrlength
                    .wrapping_add(511 as uint64_t)
                    .wrapping_div(512 as uint64_t)
                    as __blkcnt_t;
            }
            TYPE_SYMLINK => {
                (*stbuf).st_mode = (S_IFLNK | attrmode as ::core::ffi::c_int) as __mode_t;
                attrlength = get64bit(&raw mut ptr);
                (*stbuf).st_size = attrlength as __off_t;
                (*stbuf).st_blocks = attrlength
                    .wrapping_add(511 as uint64_t)
                    .wrapping_div(512 as uint64_t)
                    as __blkcnt_t;
            }
            TYPE_FILE => {
                (*stbuf).st_mode = (S_IFREG | attrmode as ::core::ffi::c_int) as __mode_t;
                attrlength = get64bit(&raw mut ptr);
                (*stbuf).st_size = attrlength as __off_t;
                (*stbuf).st_blocks = attrlength
                    .wrapping_add(511 as uint64_t)
                    .wrapping_div(512 as uint64_t)
                    as __blkcnt_t;
            }
            TYPE_FIFO => {
                (*stbuf).st_mode = (S_IFIFO | attrmode as ::core::ffi::c_int) as __mode_t;
                (*stbuf).st_size = 0 as __off_t;
                (*stbuf).st_blocks = 0 as __blkcnt_t;
            }
            TYPE_SOCKET => {
                (*stbuf).st_mode = (S_IFSOCK | attrmode as ::core::ffi::c_int) as __mode_t;
                (*stbuf).st_size = 0 as __off_t;
                (*stbuf).st_blocks = 0 as __blkcnt_t;
            }
            TYPE_BLOCKDEV => {
                (*stbuf).st_mode = (S_IFBLK | attrmode as ::core::ffi::c_int) as __mode_t;
                attrrdev = get32bit(&raw mut ptr);
                (*stbuf).st_rdev = attrrdev as __dev_t;
                (*stbuf).st_size = 0 as __off_t;
                (*stbuf).st_blocks = 0 as __blkcnt_t;
            }
            TYPE_CHARDEV => {
                (*stbuf).st_mode = (S_IFCHR | attrmode as ::core::ffi::c_int) as __mode_t;
                attrrdev = get32bit(&raw mut ptr);
                (*stbuf).st_rdev = attrrdev as __dev_t;
                (*stbuf).st_size = 0 as __off_t;
                (*stbuf).st_blocks = 0 as __blkcnt_t;
            }
            _ => {
                (*stbuf).st_mode = 0 as __mode_t;
            }
        }
        (*stbuf).st_uid = attruid as __uid_t;
        (*stbuf).st_gid = attrgid as __gid_t;
        (*stbuf).st_atim.tv_sec = attratime as __time_t;
        (*stbuf).st_mtim.tv_sec = attrmtime as __time_t;
        (*stbuf).st_ctim.tv_sec = attrctime as __time_t;
        (*stbuf).st_nlink = attrnlink as __nlink_t;
    }
}
#[inline]
unsafe extern "C" fn mfs_makemodestr(mut modestr: *mut ::core::ffi::c_char, mut mode: uint16_t) {
    unsafe {
        let mut i: uint32_t = 0;
        strcpy(
            modestr as *mut ::core::ffi::c_char,
            b"?rwxrwxrwx\0".as_ptr() as *const ::core::ffi::c_char,
        );
        match mode as ::core::ffi::c_int & S_IFMT {
            S_IFSOCK => {
                *modestr.offset(0 as isize) = 's' as ::core::ffi::c_char;
            }
            S_IFLNK => {
                *modestr.offset(0 as isize) = 'l' as ::core::ffi::c_char;
            }
            S_IFREG => {
                *modestr.offset(0 as isize) = '-' as ::core::ffi::c_char;
            }
            S_IFBLK => {
                *modestr.offset(0 as isize) = 'b' as ::core::ffi::c_char;
            }
            S_IFDIR => {
                *modestr.offset(0 as isize) = 'd' as ::core::ffi::c_char;
            }
            S_IFCHR => {
                *modestr.offset(0 as isize) = 'c' as ::core::ffi::c_char;
            }
            S_IFIFO => {
                *modestr.offset(0 as isize) = 'f' as ::core::ffi::c_char;
            }
            _ => {}
        }
        if mode as ::core::ffi::c_int & S_ISUID != 0 {
            *modestr.offset(3 as isize) = 's' as ::core::ffi::c_char;
        }
        if mode as ::core::ffi::c_int & S_ISGID != 0 {
            *modestr.offset(6 as isize) = 's' as ::core::ffi::c_char;
        }
        if mode as ::core::ffi::c_int & S_ISVTX != 0 {
            *modestr.offset(9 as isize) = 't' as ::core::ffi::c_char;
        }
        i = 0 as uint32_t;
        while i < 9 as uint32_t {
            if mode as ::core::ffi::c_int & (1 as ::core::ffi::c_int) << i
                == 0 as ::core::ffi::c_int
            {
                if *modestr.offset((9 as uint32_t).wrapping_sub(i) as isize) as ::core::ffi::c_int
                    == 's' as ::core::ffi::c_int
                    || *modestr.offset((9 as uint32_t).wrapping_sub(i) as isize)
                        as ::core::ffi::c_int
                        == 't' as ::core::ffi::c_int
                {
                    *modestr.offset((9 as uint32_t).wrapping_sub(i) as isize) = (*modestr
                        .offset((9 as uint32_t).wrapping_sub(i) as isize)
                        as ::core::ffi::c_int
                        & 0xdf as ::core::ffi::c_int)
                        as ::core::ffi::c_char;
                } else {
                    *modestr.offset((9 as uint32_t).wrapping_sub(i) as isize) =
                        '-' as ::core::ffi::c_char;
                }
            }
            i = i.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn mfs_makeattrstr(
    mut buff: *mut ::core::ffi::c_char,
    mut size: uint32_t,
    mut stbuf: *mut stat,
) {
    unsafe {
        let mut modestr: [::core::ffi::c_char; 11] = [0; 11];
        mfs_makemodestr(
            &raw mut modestr as *mut ::core::ffi::c_char,
            (*stbuf).st_mode as uint16_t,
        );
        if modestr[0 as usize] as ::core::ffi::c_int == 'b' as ::core::ffi::c_int
            || modestr[0 as usize] as ::core::ffi::c_int == 'c' as ::core::ffi::c_int
        {
            snprintf(
                buff,
                size as size_t,
                b"[%s:0%06o,%u,%ld,%ld,%lu,%lu,%lu,%llu,%08lX]\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut modestr as *mut ::core::ffi::c_char,
                (*stbuf).st_mode,
                (*stbuf).st_nlink as ::core::ffi::c_uint,
                (*stbuf).st_uid as ::core::ffi::c_long,
                (*stbuf).st_gid as ::core::ffi::c_long,
                (*stbuf).st_atim.tv_sec as ::core::ffi::c_ulong,
                (*stbuf).st_mtim.tv_sec as ::core::ffi::c_ulong,
                (*stbuf).st_ctim.tv_sec as ::core::ffi::c_ulong,
                (*stbuf).st_size as ::core::ffi::c_ulonglong,
                (*stbuf).st_rdev,
            );
        } else {
            snprintf(
                buff,
                size as size_t,
                b"[%s:0%06o,%u,%ld,%ld,%lu,%lu,%lu,%llu]\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut modestr as *mut ::core::ffi::c_char,
                (*stbuf).st_mode,
                (*stbuf).st_nlink as ::core::ffi::c_uint,
                (*stbuf).st_uid as ::core::ffi::c_long,
                (*stbuf).st_gid as ::core::ffi::c_long,
                (*stbuf).st_atim.tv_sec as ::core::ffi::c_ulong,
                (*stbuf).st_mtim.tv_sec as ::core::ffi::c_ulong,
                (*stbuf).st_ctim.tv_sec as ::core::ffi::c_ulong,
                (*stbuf).st_size as ::core::ffi::c_ulonglong,
            );
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_statfs(mut req: fuse_req_t, mut ino: fuse_ino_t) {
    unsafe {
        let mut totalspace: uint64_t = 0;
        let mut availspace: uint64_t = 0;
        let mut freespace: uint64_t = 0;
        let mut trashspace: uint64_t = 0;
        let mut sustainedspace: uint64_t = 0;
        let mut inodes: uint32_t = 0;
        let mut bsize: uint32_t = 0;
        let mut stfsbuf: statvfs = statvfs {
            f_bsize: 0,
            f_frsize: 0,
            f_blocks: 0,
            f_bfree: 0,
            f_bavail: 0,
            f_files: 0,
            f_ffree: 0,
            f_favail: 0,
            f_fsid: 0,
            f_flag: 0,
            f_namemax: 0,
            f_type: 0,
            __f_spare: [0; 5],
        };
        memset(
            &raw mut stfsbuf as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<statvfs>(),
        );
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        ctx = *fuse_req_ctx(req);
        mfs_stats_inc(OP_STATFS as ::core::ffi::c_int as uint8_t);
        if debug_mode != 0 {
            oplog_printf(
                &raw mut ctx,
                b"statfs (%lu)\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
            );
        }
        fs_statfs(
            &raw mut totalspace,
            &raw mut availspace,
            &raw mut freespace,
            &raw mut trashspace,
            &raw mut sustainedspace,
            &raw mut inodes,
        );
        bsize = 0x10000 as uint32_t;
        stfsbuf.f_namemax = MFS_NAME_MAX as ::core::ffi::c_ulong;
        stfsbuf.f_frsize = bsize as ::core::ffi::c_ulong;
        stfsbuf.f_bsize = bsize as ::core::ffi::c_ulong;
        stfsbuf.f_blocks = totalspace.wrapping_div(bsize as uint64_t) as __fsblkcnt64_t;
        stfsbuf.f_bfree = freespace.wrapping_div(bsize as uint64_t) as __fsblkcnt64_t;
        stfsbuf.f_bavail = availspace.wrapping_div(bsize as uint64_t) as __fsblkcnt64_t;
        stfsbuf.f_files = ((1000000000 as ::core::ffi::c_int + PKGVERSION) as uint32_t)
            .wrapping_add(inodes) as __fsfilcnt64_t;
        stfsbuf.f_ffree = (1000000000 as ::core::ffi::c_int + PKGVERSION) as __fsfilcnt64_t;
        stfsbuf.f_favail = (1000000000 as ::core::ffi::c_int + PKGVERSION) as __fsfilcnt64_t;
        oplog_printf(
            &raw mut ctx,
            b"statfs (%lu): OK (%lu,%lu,%lu,%lu,%lu,%u)\0".as_ptr() as *const ::core::ffi::c_char,
            ino as ::core::ffi::c_ulong,
            totalspace,
            availspace,
            freespace,
            trashspace,
            sustainedspace,
            inodes,
        );
        fuse_reply_statfs(req, &raw mut stfsbuf);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_access_test(
    mut attr: *const uint8_t,
    mut mmode: ::core::ffi::c_int,
    mut uid: uint32_t,
    mut gidcnt: uint32_t,
    mut gidtab: *mut uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut modebits: uint8_t = 0;
        let mut gok: uint8_t = 0;
        let mut attrmode: uint16_t = 0;
        let mut attruid: uint32_t = 0;
        let mut attrgid: uint32_t = 0;
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        if uid == 0 as uint32_t {
            return 0 as ::core::ffi::c_int;
        }
        ptr = attr.offset(1 as ::core::ffi::c_int as isize) as *const uint8_t;
        attrmode = get16bit(&raw mut ptr);
        attruid = get32bit(&raw mut ptr);
        attrgid = get32bit(&raw mut ptr);
        modebits = 0 as uint8_t;
        if uid == attruid {
            modebits = (attrmode as ::core::ffi::c_int >> 6 as ::core::ffi::c_int
                & 7 as ::core::ffi::c_int) as uint8_t;
        } else {
            gok = 0 as uint8_t;
            while gidcnt > 0 as uint32_t {
                gidcnt = gidcnt.wrapping_sub(1);
                if *gidtab.offset(gidcnt as isize) != attrgid {
                    continue;
                }
                modebits = (attrmode as ::core::ffi::c_int >> 3 as ::core::ffi::c_int
                    & 7 as ::core::ffi::c_int) as uint8_t;
                gok = 1 as uint8_t;
                break;
            }
            if gok as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                modebits = (attrmode as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as uint8_t;
            }
        }
        if mmode & modebits as ::core::ffi::c_int == mmode {
            return 0 as ::core::ffi::c_int;
        }
        return EACCES;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_access(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut mask: ::core::ffi::c_int,
) {
    unsafe {
        let mut status: ::core::ffi::c_int = 0;
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut mmode: ::core::ffi::c_int = 0;
        let mut lflags: uint16_t = 0;
        let mut force_mode: ::core::ffi::c_int = 0;
        ctx = *fuse_req_ctx(req);
        if debug_mode != 0 {
            oplog_printf(
                &raw mut ctx,
                b"access (%lu,0x%X) ...\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                mask,
            );
            fprintf(
                stderr,
                b"access (%lu,0x%X)\n\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                mask,
            );
        }
        mfs_stats_inc(OP_ACCESS as ::core::ffi::c_int as uint8_t);
        mmode = mask;
        if ino >= MIN_SPECIAL_INODE as fuse_ino_t {
            if mask & (W_OK | X_OK) != 0 {
                fuse_reply_err(req, EACCES);
            } else {
                fuse_reply_err(req, 0 as ::core::ffi::c_int);
            }
            return;
        }
        if let Some((_, cached_lflags)) = fdcache::find(
            ctx.uid as u32,
            ctx.gid as u32,
            ctx.pid as i32,
            ino as uint32_t,
        ) {
            lflags = cached_lflags;
            if lflags as ::core::ffi::c_int & LOOKUP_RO_FILESYSTEM != 0 && mmode & MODE_MASK_W != 0
            {
                status = MFS_ERROR_EROFS;
            } else if lflags as ::core::ffi::c_int & LOOKUP_IMMUTABLE != 0
                && mmode & MODE_MASK_W != 0
            {
                status = MFS_ERROR_EPERM;
            } else {
                status = if lflags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << (mmode & 0x7 as ::core::ffi::c_int)
                    != 0
                {
                    MFS_STATUS_OK
                } else {
                    MFS_ERROR_EACCES
                };
            }
        } else if full_permissions != 0 {
            let gids = getgroups::get_common(ctx.pid, ctx.uid, ctx.gid, false);
            status = fs_access(
                ino as uint32_t,
                ctx.uid as uint32_t,
                gids.len() as u32,
                gids.as_slice().as_ptr() as *mut u32,
                mmode as uint16_t,
            ) as ::core::ffi::c_int;
        } else {
            let mut gidtmp: uint32_t = ctx.gid as uint32_t;
            status = fs_access(
                ino as uint32_t,
                ctx.uid as uint32_t,
                1 as uint32_t,
                &raw mut gidtmp,
                mmode as uint16_t,
            ) as ::core::ffi::c_int;
        }
        force_mode = 0 as ::core::ffi::c_int;
        if status == MFS_ERROR_ENOENT {
            if ctx.pid == getpid() {
                force_mode = 1 as ::core::ffi::c_int;
            }
            if sstats_get(
                ino as uint32_t,
                &raw mut attr as *mut uint8_t,
                force_mode as uint8_t,
            ) == MFS_STATUS_OK
            {
                if force_mode == 0 as ::core::ffi::c_int {
                    force_mode = 2 as ::core::ffi::c_int;
                }
            }
        }
        if force_mode != 0 {
            if force_mode == 1 as ::core::ffi::c_int {
                if debug_mode != 0 {
                    fprintf(
                        stderr,
                        b"special case: internal access (%lu,0x%X) - positive answer\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ino as ::core::ffi::c_ulong,
                        mask,
                    );
                }
                oplog_printf(
                    &raw mut ctx,
                    b"special case: internal access (%lu,0x%X): OK\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    mask,
                );
                status = 0 as ::core::ffi::c_int;
            } else {
                if debug_mode != 0 {
                    fprintf(
                        stderr,
                        b"special case: sustained access (%lu,0x%X) - using stored data\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        ino as ::core::ffi::c_ulong,
                        mask,
                    );
                }
                if full_permissions != 0 {
                    let gids = getgroups::get_common(ctx.pid, ctx.uid, ctx.gid, false);
                    status = mfs_access_test(
                        &raw mut attr as *mut uint8_t as *const uint8_t,
                        mmode,
                        ctx.uid as uint32_t,
                        gids.len() as u32,
                        gids.as_slice().as_ptr() as *mut u32,
                    );
                } else {
                    let mut gidtmp_0: uint32_t = ctx.gid as uint32_t;
                    status = mfs_access_test(
                        &raw mut attr as *mut uint8_t as *const uint8_t,
                        mmode,
                        ctx.uid as uint32_t,
                        1 as uint32_t,
                        &raw mut gidtmp_0,
                    );
                }
            }
        } else {
            status = mfs_errorconv(status);
            if status != 0 as ::core::ffi::c_int {
                oplog_printf(
                    &raw mut ctx,
                    b"access (%lu,0x%X): %s\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    mask,
                    strerr(status),
                );
            } else {
                oplog_printf(
                    &raw mut ctx,
                    b"access (%lu,0x%X): OK\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    mask,
                );
            }
        }
        fuse_reply_err(req, status);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_lookup(
    mut req: fuse_req_t,
    mut parent: fuse_ino_t,
    mut name: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut e: fuse_entry_param = fuse_entry_param {
            ino: 0,
            generation: 0,
            attr: stat {
                st_dev: 0,
                st_ino: 0,
                st_nlink: 0,
                st_mode: 0,
                st_uid: 0,
                st_gid: 0,
                __pad0: 0,
                st_rdev: 0,
                st_size: 0,
                st_blksize: 0,
                st_blocks: 0,
                st_atim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_mtim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_ctim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                __glibc_reserved: [0; 3],
            },
            attr_timeout: 0.,
            entry_timeout: 0.,
        };
        let mut maxfleng: uint64_t = 0;
        let mut inode: uint32_t = 0;
        let mut nleng: uint32_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut csdataver: uint8_t = 0;
        let mut lflags: uint16_t = 0;
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut csdata: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut csdatasize: uint32_t = 0;
        let mut attrstr: [::core::ffi::c_char; 256] = [0; 256];
        let mut mattr: uint8_t = 0;
        let mut r#type: uint8_t = 0;
        let mut icacheflag: uint8_t = 0;
        let mut status: ::core::ffi::c_int = 0;
        let mut nocache: ::core::ffi::c_int = 0;
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        ctx = *fuse_req_ctx(req);
        if debug_mode != 0 {
            oplog_printf(
                &raw mut ctx,
                b"lookup (%lu,%s) ...\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
            );
            fprintf(
                stderr,
                b"lookup (%lu,%s)\n\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
            );
        }
        nleng = strlen(name) as uint32_t;
        if nleng > MFS_NAME_MAX as uint32_t {
            mfs_stats_inc(OP_ERRLOOKUP as ::core::ffi::c_int as uint8_t);
            oplog_printf(
                &raw mut ctx,
                b"lookup (%lu,%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                strerr(ENAMETOOLONG),
            );
            fuse_reply_err(req, ENAMETOOLONG);
            return;
        }
        if parent == FUSE_ROOT_ID as fuse_ino_t {
            if nleng == 2 as uint32_t
                && *name.offset(0 as isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                && *name.offset(1 as isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            {
                nleng = 1 as uint32_t;
            }
            if strcmp(name, MASTERINFO_NAME.as_ptr()) == 0 as ::core::ffi::c_int {
                memset(
                    &raw mut e as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<fuse_entry_param>(),
                );
                e.ino = MASTERINFO_INODE as fuse_ino_t;
                e.generation = 1 as uint64_t;
                e.attr_timeout = 3600.0f64;
                e.entry_timeout = 3600.0f64;
                mfs_attr_to_stat(
                    MASTERINFO_INODE as uint32_t,
                    &raw mut masterinfoattr as *mut uint8_t as *const uint8_t,
                    &raw mut e.attr,
                );
                mfs_stats_inc(OP_LOOKUP_INTERNAL as ::core::ffi::c_int as uint8_t);
                mfs_makeattrstr(
                    &raw mut attrstr as *mut ::core::ffi::c_char,
                    256 as uint32_t,
                    &raw mut e.attr,
                );
                oplog_printf(
                    &raw mut ctx,
                    b"lookup (%lu,%s) (internal node: MASTERINFO): OK (%.1lf,%lu,%.1lf,%s)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    parent as ::core::ffi::c_ulong,
                    name,
                    e.entry_timeout,
                    e.ino as ::core::ffi::c_ulong,
                    e.attr_timeout,
                    &raw mut attrstr as *mut ::core::ffi::c_char,
                );
                fuse_reply_entry(req, &raw mut e);
                return;
            }
            if strcmp(name, STATS_NAME.as_ptr()) == 0 as ::core::ffi::c_int {
                memset(
                    &raw mut e as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<fuse_entry_param>(),
                );
                e.ino = STATS_INODE as fuse_ino_t;
                e.generation = 1 as uint64_t;
                e.attr_timeout = 3600.0f64;
                e.entry_timeout = 3600.0f64;
                mfs_attr_to_stat(
                    STATS_INODE as uint32_t,
                    &raw mut statsattr as *mut uint8_t as *const uint8_t,
                    &raw mut e.attr,
                );
                mfs_stats_inc(OP_LOOKUP_INTERNAL as ::core::ffi::c_int as uint8_t);
                mfs_makeattrstr(
                    &raw mut attrstr as *mut ::core::ffi::c_char,
                    256 as uint32_t,
                    &raw mut e.attr,
                );
                oplog_printf(
                    &raw mut ctx,
                    b"lookup (%lu,%s) (internal node: STATS): OK (%.1lf,%lu,%.1lf,%s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    parent as ::core::ffi::c_ulong,
                    name,
                    e.entry_timeout,
                    e.ino as ::core::ffi::c_ulong,
                    e.attr_timeout,
                    &raw mut attrstr as *mut ::core::ffi::c_char,
                );
                fuse_reply_entry(req, &raw mut e);
                return;
            }
            if strcmp(name, PARAMS_NAME.as_ptr()) == 0 as ::core::ffi::c_int {
                memset(
                    &raw mut e as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<fuse_entry_param>(),
                );
                e.ino = PARAMS_INODE as fuse_ino_t;
                e.generation = 1 as uint64_t;
                e.attr_timeout = 3600.0f64;
                e.entry_timeout = 3600.0f64;
                mfs_attr_to_stat(
                    PARAMS_INODE as uint32_t,
                    &raw mut paramsattr as *mut uint8_t as *const uint8_t,
                    &raw mut e.attr,
                );
                mfs_stats_inc(OP_LOOKUP_INTERNAL as ::core::ffi::c_int as uint8_t);
                mfs_makeattrstr(
                    &raw mut attrstr as *mut ::core::ffi::c_char,
                    256 as uint32_t,
                    &raw mut e.attr,
                );
                oplog_printf(
                    &raw mut ctx,
                    b"lookup (%lu,%s) (internal node: PARAMS): OK (%.1lf,%lu,%.1lf,%s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    parent as ::core::ffi::c_ulong,
                    name,
                    e.entry_timeout,
                    e.ino as ::core::ffi::c_ulong,
                    e.attr_timeout,
                    &raw mut attrstr as *mut ::core::ffi::c_char,
                );
                fuse_reply_entry(req, &raw mut e);
                return;
            }
            if strcmp(name, RANDOM_NAME.as_ptr()) == 0 as ::core::ffi::c_int {
                memset(
                    &raw mut e as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<fuse_entry_param>(),
                );
                e.ino = RANDOM_INODE as fuse_ino_t;
                e.generation = 1 as uint64_t;
                e.attr_timeout = 3600.0f64;
                e.entry_timeout = 3600.0f64;
                mfs_attr_to_stat(
                    RANDOM_INODE as uint32_t,
                    &raw mut randomattr as *mut uint8_t as *const uint8_t,
                    &raw mut e.attr,
                );
                mfs_stats_inc(OP_LOOKUP_INTERNAL as ::core::ffi::c_int as uint8_t);
                mfs_makeattrstr(
                    &raw mut attrstr as *mut ::core::ffi::c_char,
                    256 as uint32_t,
                    &raw mut e.attr,
                );
                oplog_printf(
                    &raw mut ctx,
                    b"lookup (%lu,%s) (internal node: RANDOM): OK (%.1lf,%lu,%.1lf,%s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    parent as ::core::ffi::c_ulong,
                    name,
                    e.entry_timeout,
                    e.ino as ::core::ffi::c_ulong,
                    e.attr_timeout,
                    &raw mut attrstr as *mut ::core::ffi::c_char,
                );
                fuse_reply_entry(req, &raw mut e);
                return;
            }
            if strcmp(name, MOOSE_NAME.as_ptr()) == 0 as ::core::ffi::c_int {
                memset(
                    &raw mut e as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<fuse_entry_param>(),
                );
                e.ino = MOOSE_INODE as fuse_ino_t;
                e.generation = 1 as uint64_t;
                e.attr_timeout = 3600.0f64;
                e.entry_timeout = 3600.0f64;
                mfs_attr_to_stat(
                    MOOSE_INODE as uint32_t,
                    &raw mut mooseattr as *mut uint8_t as *const uint8_t,
                    &raw mut e.attr,
                );
                mfs_stats_inc(OP_LOOKUP_INTERNAL as ::core::ffi::c_int as uint8_t);
                mfs_makeattrstr(
                    &raw mut attrstr as *mut ::core::ffi::c_char,
                    256 as uint32_t,
                    &raw mut e.attr,
                );
                oplog_printf(
                    &raw mut ctx,
                    b"lookup (%lu,%s) (internal node: MOOSE): OK (%.1lf,%lu,%.1lf,%s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    parent as ::core::ffi::c_ulong,
                    name,
                    e.entry_timeout,
                    e.ino as ::core::ffi::c_ulong,
                    e.attr_timeout,
                    &raw mut attrstr as *mut ::core::ffi::c_char,
                );
                fuse_reply_entry(req, &raw mut e);
                return;
            }
            if strcmp(name, OPLOG_NAME.as_ptr()) == 0 as ::core::ffi::c_int {
                memset(
                    &raw mut e as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<fuse_entry_param>(),
                );
                e.ino = OPLOG_INODE as fuse_ino_t;
                e.generation = 1 as uint64_t;
                e.attr_timeout = 3600.0f64;
                e.entry_timeout = 3600.0f64;
                mfs_attr_to_stat(
                    OPLOG_INODE as uint32_t,
                    &raw mut oplogattr as *mut uint8_t as *const uint8_t,
                    &raw mut e.attr,
                );
                mfs_stats_inc(OP_LOOKUP_INTERNAL as ::core::ffi::c_int as uint8_t);
                mfs_makeattrstr(
                    &raw mut attrstr as *mut ::core::ffi::c_char,
                    256 as uint32_t,
                    &raw mut e.attr,
                );
                oplog_printf(
                    &raw mut ctx,
                    b"lookup (%lu,%s) (internal node: OPLOG): OK (%.1lf,%lu,%.1lf,%s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    parent as ::core::ffi::c_ulong,
                    name,
                    e.entry_timeout,
                    e.ino as ::core::ffi::c_ulong,
                    e.attr_timeout,
                    &raw mut attrstr as *mut ::core::ffi::c_char,
                );
                fuse_reply_entry(req, &raw mut e);
                return;
            }
            if strcmp(name, OPHISTORY_NAME.as_ptr()) == 0 as ::core::ffi::c_int {
                memset(
                    &raw mut e as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<fuse_entry_param>(),
                );
                e.ino = OPHISTORY_INODE as fuse_ino_t;
                e.generation = 1 as uint64_t;
                e.attr_timeout = 3600.0f64;
                e.entry_timeout = 3600.0f64;
                mfs_attr_to_stat(
                    OPHISTORY_INODE as uint32_t,
                    &raw mut oplogattr as *mut uint8_t as *const uint8_t,
                    &raw mut e.attr,
                );
                mfs_stats_inc(OP_LOOKUP_INTERNAL as ::core::ffi::c_int as uint8_t);
                mfs_makeattrstr(
                    &raw mut attrstr as *mut ::core::ffi::c_char,
                    256 as uint32_t,
                    &raw mut e.attr,
                );
                oplog_printf(
                    &raw mut ctx,
                    b"lookup (%lu,%s) (internal node: OPHISTORY): OK (%.1lf,%lu,%.1lf,%s)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    parent as ::core::ffi::c_ulong,
                    name,
                    e.entry_timeout,
                    e.ino as ::core::ffi::c_ulong,
                    e.attr_timeout,
                    &raw mut attrstr as *mut ::core::ffi::c_char,
                );
                fuse_reply_entry(req, &raw mut e);
                return;
            }
        }
        if usedircache != 0
            && lookup_cached_entry(
                &ctx,
                parent as uint32_t,
                std::slice::from_raw_parts(name as *const uint8_t, nleng as usize),
                &mut inode,
                &mut attr,
            )
        {
            if debug_mode != 0 {
                fprintf(
                    stderr,
                    b"lookup: sending data from dircache\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            mfs_stats_inc(OP_DIRCACHE_LOOKUP as ::core::ffi::c_int as uint8_t);
            status = 0 as ::core::ffi::c_int;
            lflags = 0xffff as uint16_t;
            icacheflag = 1 as uint8_t;
        } else {
            if negentry_cache_search(parent as uint32_t, nleng as uint8_t, name as *const uint8_t)
                != 0
            {
                if debug_mode != 0 {
                    fprintf(
                        stderr,
                        b"lookup: sending data from negcache\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                oplog_printf(
                    &raw mut ctx,
                    b"lookup (%lu,%s) (using negative entry cache): %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    parent as ::core::ffi::c_ulong,
                    name,
                    strerr(ENOENT),
                );
                mfs_stats_inc(OP_NEGCACHE_LOOKUP as ::core::ffi::c_int as uint8_t);
                fuse_reply_err(req, ENOENT);
                return;
            }
            if full_permissions != 0 {
                let gids = getgroups::get_common(ctx.pid, ctx.uid, ctx.gid, false);
                status = fs_lookup(
                    parent as uint32_t,
                    nleng as uint8_t,
                    name as *const uint8_t,
                    ctx.uid as uint32_t,
                    gids.len() as u32,
                    gids.as_slice().as_ptr() as *mut u32,
                    &raw mut inode,
                    &raw mut attr as *mut uint8_t,
                    &raw mut lflags,
                    &raw mut csdataver,
                    &raw mut chunkid,
                    &raw mut version,
                    &raw mut csdata,
                    &raw mut csdatasize,
                ) as ::core::ffi::c_int;
            } else {
                let mut gidtmp: uint32_t = ctx.gid as uint32_t;
                status = fs_lookup(
                    parent as uint32_t,
                    nleng as uint8_t,
                    name as *const uint8_t,
                    ctx.uid as uint32_t,
                    1 as uint32_t,
                    &raw mut gidtmp,
                    &raw mut inode,
                    &raw mut attr as *mut uint8_t,
                    &raw mut lflags,
                    &raw mut csdataver,
                    &raw mut chunkid,
                    &raw mut version,
                    &raw mut csdata,
                    &raw mut csdatasize,
                ) as ::core::ffi::c_int;
            }
            if status == MFS_ERROR_ENOENT_NOCACHE {
                status = MFS_ERROR_ENOENT;
                nocache = 1 as ::core::ffi::c_int;
            } else {
                nocache = 0 as ::core::ffi::c_int;
            }
            status = mfs_errorconv(status);
            icacheflag = 0 as uint8_t;
            if status == 0 as ::core::ffi::c_int {
                mfs_stats_inc(OP_POSLOOKUP as ::core::ffi::c_int as uint8_t);
            } else if status == ENOENT {
                if strcmp(name, MASTERINFO_NAME.as_ptr()) == 0 as ::core::ffi::c_int {
                    memset(
                        &raw mut e as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<fuse_entry_param>(),
                    );
                    e.ino = MASTERINFO_INODE as fuse_ino_t;
                    e.generation = 1 as uint64_t;
                    e.attr_timeout = 3600.0f64;
                    e.entry_timeout = 3600.0f64;
                    mfs_attr_to_stat(
                        MASTERINFO_INODE as uint32_t,
                        &raw mut masterinfoattr as *mut uint8_t as *const uint8_t,
                        &raw mut e.attr,
                    );
                    mfs_stats_inc(OP_LOOKUP_INTERNAL as ::core::ffi::c_int as uint8_t);
                    mfs_makeattrstr(
                        &raw mut attrstr as *mut ::core::ffi::c_char,
                        256 as uint32_t,
                        &raw mut e.attr,
                    );
                    oplog_printf(
                        &raw mut ctx,
                        b"lookup (%lu,%s) (internal node: MASTERINFO): OK (%.1lf,%lu,%.1lf,%s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        parent as ::core::ffi::c_ulong,
                        name,
                        e.entry_timeout,
                        e.ino as ::core::ffi::c_ulong,
                        e.attr_timeout,
                        &raw mut attrstr as *mut ::core::ffi::c_char,
                    );
                    fuse_reply_entry(req, &raw mut e);
                    return;
                } else {
                    mfs_stats_inc(OP_NEGLOOKUP as ::core::ffi::c_int as uint8_t);
                    if nocache == 0 as ::core::ffi::c_int {
                        negentry_cache_insert(
                            parent as uint32_t,
                            nleng as uint8_t,
                            name as *const uint8_t,
                        );
                    }
                }
            } else {
                mfs_stats_inc(OP_ERRLOOKUP as ::core::ffi::c_int as uint8_t);
            }
        }
        if status != 0 as ::core::ffi::c_int {
            oplog_printf(
                &raw mut ctx,
                b"lookup (%lu,%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                strerr(status),
            );
            fuse_reply_err(req, status);
            return;
        }
        r#type = mfs_attr_get_type(&raw mut attr as *mut uint8_t as *const uint8_t);
        if r#type as ::core::ffi::c_int == TYPE_FILE {
            maxfleng = write_data_inode_getmaxfleng(inode);
        } else {
            maxfleng = 0 as uint64_t;
        }
        if r#type as ::core::ffi::c_int == TYPE_DIRECTORY {
            sstats_set(
                inode,
                &raw mut attr as *mut uint8_t as *const uint8_t,
                1 as uint8_t,
            );
            sparents_add(
                inode,
                parent as uint32_t,
                (direntry_cache_timeout + 60 as ::core::ffi::c_int as ::core::ffi::c_double)
                    as uint32_t,
            );
        }
        memset(
            &raw mut e as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<fuse_entry_param>(),
        );
        e.ino = inode as fuse_ino_t;
        e.generation = 1 as uint64_t;
        mattr = mfs_attr_get_mattr(&raw mut attr as *mut uint8_t as *const uint8_t);
        e.attr_timeout = if mattr as ::core::ffi::c_int & MATTR_NOACACHE != 0 {
            0.0f64
        } else {
            attr_cache_timeout
        };
        e.entry_timeout = if mattr as ::core::ffi::c_int & MATTR_NOECACHE != 0 {
            0.0f64
        } else if r#type as ::core::ffi::c_int == TYPE_DIRECTORY {
            direntry_cache_timeout
        } else {
            entry_cache_timeout
        };
        if dinval != 0
            && mattr as ::core::ffi::c_int & MATTR_UNDELETABLE == 0 as ::core::ffi::c_int
            && r#type as ::core::ffi::c_int == TYPE_DIRECTORY
        {
            dinval_add(
                parent as uint32_t,
                nleng as uint8_t,
                name as *const uint8_t,
                inode,
            );
        }
        mfs_attr_to_stat(
            inode,
            &raw mut attr as *mut uint8_t as *const uint8_t,
            &raw mut e.attr,
        );
        if maxfleng > e.attr.st_size as uint64_t {
            e.attr.st_size = maxfleng as __off_t;
            mfs_attr_set_fleng(&raw mut attr as *mut uint8_t, maxfleng);
        }
        if lflags as ::core::ffi::c_int != 0xffff as ::core::ffi::c_int {
            let csdata_slice = if csdata.is_null() || csdatasize == 0 {
                &[][..]
            } else {
                std::slice::from_raw_parts(csdata, csdatasize as usize)
            };
            fdcache::insert(
                ctx.uid as u32,
                ctx.gid as u32,
                ctx.pid as i32,
                inode,
                attr[..35].try_into().unwrap(),
                lflags,
                csdataver,
                chunkid,
                version,
                csdata_slice,
            );
        }
        if r#type as ::core::ffi::c_int == TYPE_FILE {
            read_inode_set_length_passive(inode, e.attr.st_size as uint64_t);
            finfo_change_fleng(inode, e.attr.st_size as uint64_t);
        }
        mfs_fix_amtime(
            inode,
            &raw mut e.attr.st_atim.tv_sec,
            &raw mut e.attr.st_mtim.tv_sec,
        );
        mfs_makeattrstr(
            &raw mut attrstr as *mut ::core::ffi::c_char,
            256 as uint32_t,
            &raw mut e.attr,
        );
        oplog_printf(
            &raw mut ctx,
            b"lookup (%lu,%s)%s: OK (%.1lf,%lu,%.1lf,%s)\0".as_ptr() as *const ::core::ffi::c_char,
            parent as ::core::ffi::c_ulong,
            name,
            if icacheflag as ::core::ffi::c_int != 0 {
                b" (using open dir cache)\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            e.entry_timeout,
            e.ino as ::core::ffi::c_ulong,
            e.attr_timeout,
            &raw mut attrstr as *mut ::core::ffi::c_char,
        );
        fuse_reply_entry(req, &raw mut e);
        if debug_mode != 0 {
            fprintf(
                stderr,
                b"lookup: positive answer timeouts (attr:%.3lf,entry:%.3lf)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                e.attr_timeout,
                e.entry_timeout,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_getattr(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut fi: *mut fuse_file_info,
) {
    unsafe {
        let mut maxfleng: uint64_t = 0;
        let mut attr_timeout: ::core::ffi::c_double = 0.;
        let mut o_stbuf: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut r#type: uint8_t = 0;
        let mut attrstr: [::core::ffi::c_char; 256] = [0; 256];
        let mut status: ::core::ffi::c_int = 0;
        let mut icacheflag: uint8_t = 0;
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        let mut force_mode: ::core::ffi::c_int = 0;
        ctx = *fuse_req_ctx(req);
        if debug_mode != 0 {
            if !fi.is_null() {
                oplog_printf(
                    &raw mut ctx,
                    b"getattr (%lu) [handle:%08X] ...\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    (*fi).fh as uint32_t,
                );
            } else {
                oplog_printf(
                    &raw mut ctx,
                    b"getattr (%lu) ...\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                );
            }
            fprintf(
                stderr,
                b"getattr (%lu)\n\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
            );
        }
        if ino == MASTERINFO_INODE as fuse_ino_t {
            memset(
                &raw mut o_stbuf as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<stat>(),
            );
            mfs_attr_to_stat(
                ino as uint32_t,
                &raw mut masterinfoattr as *mut uint8_t as *const uint8_t,
                &raw mut o_stbuf,
            );
            mfs_stats_inc(OP_GETATTR as ::core::ffi::c_int as uint8_t);
            mfs_makeattrstr(
                &raw mut attrstr as *mut ::core::ffi::c_char,
                256 as uint32_t,
                &raw mut o_stbuf,
            );
            oplog_printf(
                &raw mut ctx,
                b"getattr (%lu) (internal node: MASTERINFO): OK (3600,%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                &raw mut attrstr as *mut ::core::ffi::c_char,
            );
            fuse_reply_attr(req, &raw mut o_stbuf, 3600.0f64);
            return;
        }
        if ino == STATS_INODE as fuse_ino_t {
            memset(
                &raw mut o_stbuf as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<stat>(),
            );
            mfs_attr_to_stat(
                ino as uint32_t,
                &raw mut statsattr as *mut uint8_t as *const uint8_t,
                &raw mut o_stbuf,
            );
            mfs_stats_inc(OP_GETATTR as ::core::ffi::c_int as uint8_t);
            mfs_makeattrstr(
                &raw mut attrstr as *mut ::core::ffi::c_char,
                256 as uint32_t,
                &raw mut o_stbuf,
            );
            oplog_printf(
                &raw mut ctx,
                b"getattr (%lu) (internal node: STATS): OK (3600,%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                &raw mut attrstr as *mut ::core::ffi::c_char,
            );
            fuse_reply_attr(req, &raw mut o_stbuf, 3600.0f64);
            return;
        }
        if ino == PARAMS_INODE as fuse_ino_t {
            memset(
                &raw mut o_stbuf as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<stat>(),
            );
            mfs_attr_to_stat(
                ino as uint32_t,
                &raw mut paramsattr as *mut uint8_t as *const uint8_t,
                &raw mut o_stbuf,
            );
            mfs_stats_inc(OP_GETATTR as ::core::ffi::c_int as uint8_t);
            mfs_makeattrstr(
                &raw mut attrstr as *mut ::core::ffi::c_char,
                256 as uint32_t,
                &raw mut o_stbuf,
            );
            oplog_printf(
                &raw mut ctx,
                b"getattr (%lu) (internal node: PARAMS): OK (3600,%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                &raw mut attrstr as *mut ::core::ffi::c_char,
            );
            fuse_reply_attr(req, &raw mut o_stbuf, 3600.0f64);
            return;
        }
        if ino == RANDOM_INODE as fuse_ino_t {
            memset(
                &raw mut o_stbuf as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<stat>(),
            );
            mfs_attr_to_stat(
                ino as uint32_t,
                &raw mut randomattr as *mut uint8_t as *const uint8_t,
                &raw mut o_stbuf,
            );
            mfs_stats_inc(OP_GETATTR as ::core::ffi::c_int as uint8_t);
            mfs_makeattrstr(
                &raw mut attrstr as *mut ::core::ffi::c_char,
                256 as uint32_t,
                &raw mut o_stbuf,
            );
            oplog_printf(
                &raw mut ctx,
                b"getattr (%lu) (internal node: RANDOM): OK (3600,%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                &raw mut attrstr as *mut ::core::ffi::c_char,
            );
            fuse_reply_attr(req, &raw mut o_stbuf, 3600.0f64);
            return;
        }
        if ino == MOOSE_INODE as fuse_ino_t {
            memset(
                &raw mut o_stbuf as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<stat>(),
            );
            mfs_attr_to_stat(
                ino as uint32_t,
                &raw mut mooseattr as *mut uint8_t as *const uint8_t,
                &raw mut o_stbuf,
            );
            mfs_stats_inc(OP_GETATTR as ::core::ffi::c_int as uint8_t);
            mfs_makeattrstr(
                &raw mut attrstr as *mut ::core::ffi::c_char,
                256 as uint32_t,
                &raw mut o_stbuf,
            );
            oplog_printf(
                &raw mut ctx,
                b"getattr (%lu) (internal node: MOOSE): OK (3600,%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                &raw mut attrstr as *mut ::core::ffi::c_char,
            );
            fuse_reply_attr(req, &raw mut o_stbuf, 3600.0f64);
            return;
        }
        if ino == OPLOG_INODE as fuse_ino_t || ino == OPHISTORY_INODE as fuse_ino_t {
            memset(
                &raw mut o_stbuf as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<stat>(),
            );
            mfs_attr_to_stat(
                ino as uint32_t,
                &raw mut oplogattr as *mut uint8_t as *const uint8_t,
                &raw mut o_stbuf,
            );
            mfs_stats_inc(OP_GETATTR as ::core::ffi::c_int as uint8_t);
            mfs_makeattrstr(
                &raw mut attrstr as *mut ::core::ffi::c_char,
                256 as uint32_t,
                &raw mut o_stbuf,
            );
            oplog_printf(
                &raw mut ctx,
                b"getattr (%lu) (internal node: %s): OK (3600,%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                if ino == OPLOG_INODE as fuse_ino_t {
                    b"OPLOG\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"OPHISTORY\0".as_ptr() as *const ::core::ffi::c_char
                },
                &raw mut attrstr as *mut ::core::ffi::c_char,
            );
            fuse_reply_attr(req, &raw mut o_stbuf, 3600.0f64);
            return;
        }
        force_mode = 0 as ::core::ffi::c_int;
        if usedircache != 0 && lookup_cached_attr(&ctx, ino as uint32_t, &mut attr) {
            if debug_mode != 0 {
                fprintf(
                    stderr,
                    b"getattr: sending data from dircache\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            mfs_stats_inc(OP_DIRCACHE_GETATTR as ::core::ffi::c_int as uint8_t);
            status = 0 as ::core::ffi::c_int;
            icacheflag = 1 as uint8_t;
        } else {
            mfs_stats_inc(OP_GETATTR as ::core::ffi::c_int as uint8_t);
            if let Some((cached_attr, _)) = fdcache::find(
                ctx.uid as u32,
                ctx.gid as u32,
                ctx.pid as i32,
                ino as uint32_t,
            ) {
                attr[..35].copy_from_slice(&cached_attr);
                if debug_mode != 0 {
                    fprintf(
                        stderr,
                        b"getattr: sending data from fdcache\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                status = MFS_STATUS_OK;
            } else {
                status = fs_getattr(
                    ino as uint32_t,
                    (if !fi.is_null() || fs_isopen(ino as uint32_t) != 0 {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint8_t,
                    ctx.uid as uint32_t,
                    ctx.gid as uint32_t,
                    &raw mut attr as *mut uint8_t,
                ) as ::core::ffi::c_int;
            }
            if status == MFS_ERROR_ENOENT {
                if ctx.pid == getpid() {
                    force_mode = 1 as ::core::ffi::c_int;
                }
                status = sstats_get(
                    ino as uint32_t,
                    &raw mut attr as *mut uint8_t,
                    force_mode as uint8_t,
                );
                if status == MFS_STATUS_OK && force_mode == 0 as ::core::ffi::c_int {
                    force_mode = 2 as ::core::ffi::c_int;
                }
            }
            status = mfs_errorconv(status);
            icacheflag = 0 as uint8_t;
        }
        if status != 0 as ::core::ffi::c_int {
            if !fi.is_null() {
                oplog_printf(
                    &raw mut ctx,
                    b"getattr (%lu) [handle:%08X]: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    (*fi).fh as uint32_t,
                    strerr(status),
                );
            } else {
                oplog_printf(
                    &raw mut ctx,
                    b"getattr (%lu) [no handle]: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    strerr(status),
                );
            }
            fuse_reply_err(req, status);
            return;
        }
        r#type = mfs_attr_get_type(&raw mut attr as *mut uint8_t as *const uint8_t);
        if r#type as ::core::ffi::c_int == TYPE_FILE {
            maxfleng = write_data_inode_getmaxfleng(ino as uint32_t);
        } else {
            maxfleng = 0 as uint64_t;
        }
        if r#type as ::core::ffi::c_int == TYPE_DIRECTORY && force_mode == 0 as ::core::ffi::c_int {
            sstats_set(
                ino as uint32_t,
                &raw mut attr as *mut uint8_t as *const uint8_t,
                1 as uint8_t,
            );
        }
        memset(
            &raw mut o_stbuf as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<stat>(),
        );
        mfs_attr_to_stat(
            ino as uint32_t,
            &raw mut attr as *mut uint8_t as *const uint8_t,
            &raw mut o_stbuf,
        );
        if maxfleng > o_stbuf.st_size as uint64_t {
            o_stbuf.st_size = maxfleng as __off_t;
            mfs_attr_set_fleng(&raw mut attr as *mut uint8_t, maxfleng);
        }
        if r#type as ::core::ffi::c_int == TYPE_FILE {
            read_inode_set_length_passive(ino as uint32_t, o_stbuf.st_size as uint64_t);
            finfo_change_fleng(ino as uint32_t, o_stbuf.st_size as uint64_t);
            fdcache::invalidate(ino as uint32_t);
        }
        mfs_fix_amtime(
            ino as uint32_t,
            &raw mut o_stbuf.st_atim.tv_sec,
            &raw mut o_stbuf.st_mtim.tv_sec,
        );
        attr_timeout = if mfs_attr_get_mattr(&raw mut attr as *mut uint8_t as *const uint8_t)
            as ::core::ffi::c_int
            & MATTR_NOACACHE
            != 0
            || force_mode != 0
        {
            0.0f64
        } else {
            attr_cache_timeout
        };
        mfs_makeattrstr(
            &raw mut attrstr as *mut ::core::ffi::c_char,
            256 as uint32_t,
            &raw mut o_stbuf,
        );
        if !fi.is_null() {
            oplog_printf(
                &raw mut ctx,
                b"getattr (%lu) [handle:%08X]%s: OK (%.1lf,%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                (*fi).fh as uint32_t,
                if icacheflag as ::core::ffi::c_int != 0 {
                    b" (using open dir cache)\0".as_ptr() as *const ::core::ffi::c_char
                } else if force_mode == 1 as ::core::ffi::c_int {
                    b" (internal getattr)\0".as_ptr() as *const ::core::ffi::c_char
                } else if force_mode == 2 as ::core::ffi::c_int {
                    b" (sustained nodes)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
                attr_timeout,
                &raw mut attrstr as *mut ::core::ffi::c_char,
            );
        } else {
            oplog_printf(
                &raw mut ctx,
                b"getattr (%lu) [no handle]%s: OK (%.1lf,%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                if icacheflag as ::core::ffi::c_int != 0 {
                    b" (using open dir cache)\0".as_ptr() as *const ::core::ffi::c_char
                } else if force_mode == 1 as ::core::ffi::c_int {
                    b" (internal getattr)\0".as_ptr() as *const ::core::ffi::c_char
                } else if force_mode == 2 as ::core::ffi::c_int {
                    b" (sustained nodes)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
                attr_timeout,
                &raw mut attrstr as *mut ::core::ffi::c_char,
            );
        }
        fuse_reply_attr(req, &raw mut o_stbuf, attr_timeout);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_make_setattr_str(
    mut strbuff: *mut ::core::ffi::c_char,
    mut strsize: uint32_t,
    mut stbuf: *mut stat,
    mut to_set: ::core::ffi::c_int,
) {
    unsafe {
        let mut strleng: uint32_t = 0 as uint32_t;
        let mut modestr: [::core::ffi::c_char; 11] = [0; 11];
        if strleng < strsize && to_set & FUSE_SET_ATTR_MODE != 0 {
            mfs_makemodestr(
                &raw mut modestr as *mut ::core::ffi::c_char,
                (*stbuf).st_mode as uint16_t,
            );
            strleng = strleng.wrapping_add(snprintf(
                strbuff.offset(strleng as isize),
                strsize.wrapping_sub(strleng) as size_t,
                b"mode=%s:0%04o;\0".as_ptr() as *const ::core::ffi::c_char,
                (&raw mut modestr as *mut ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize),
                (*stbuf).st_mode & 0o7777 as __mode_t,
            ) as uint32_t);
        }
        if strleng < strsize && to_set & FUSE_SET_ATTR_UID != 0 {
            strleng = strleng.wrapping_add(snprintf(
                strbuff.offset(strleng as isize),
                strsize.wrapping_sub(strleng) as size_t,
                b"uid=%ld;\0".as_ptr() as *const ::core::ffi::c_char,
                (*stbuf).st_uid as ::core::ffi::c_long,
            ) as uint32_t);
        }
        if strleng < strsize && to_set & FUSE_SET_ATTR_GID != 0 {
            strleng = strleng.wrapping_add(snprintf(
                strbuff.offset(strleng as isize),
                strsize.wrapping_sub(strleng) as size_t,
                b"gid=%ld;\0".as_ptr() as *const ::core::ffi::c_char,
                (*stbuf).st_gid as ::core::ffi::c_long,
            ) as uint32_t);
        }
        if strleng < strsize
            && (to_set & FUSE_SET_ATTR_ATIME_NOW != 0
                || to_set & FUSE_SET_ATTR_ATIME != 0 && (*stbuf).st_atim.tv_sec < 0 as __time_t)
        {
            strleng = strleng.wrapping_add(snprintf(
                strbuff.offset(strleng as isize),
                strsize.wrapping_sub(strleng) as size_t,
                b"atime=NOW;\0".as_ptr() as *const ::core::ffi::c_char,
            ) as uint32_t);
        } else if strleng < strsize && to_set & FUSE_SET_ATTR_ATIME != 0 {
            strleng = strleng.wrapping_add(snprintf(
                strbuff.offset(strleng as isize),
                strsize.wrapping_sub(strleng) as size_t,
                b"atime=%lu;\0".as_ptr() as *const ::core::ffi::c_char,
                (*stbuf).st_atim.tv_sec as ::core::ffi::c_ulong,
            ) as uint32_t);
        }
        if strleng < strsize
            && (to_set & FUSE_SET_ATTR_MTIME_NOW != 0
                || to_set & FUSE_SET_ATTR_MTIME != 0 && (*stbuf).st_mtim.tv_sec < 0 as __time_t)
        {
            strleng = strleng.wrapping_add(snprintf(
                strbuff.offset(strleng as isize),
                strsize.wrapping_sub(strleng) as size_t,
                b"mtime=NOW;\0".as_ptr() as *const ::core::ffi::c_char,
            ) as uint32_t);
        } else if strleng < strsize && to_set & FUSE_SET_ATTR_MTIME != 0 {
            strleng = strleng.wrapping_add(snprintf(
                strbuff.offset(strleng as isize),
                strsize.wrapping_sub(strleng) as size_t,
                b"mtime=%lu;\0".as_ptr() as *const ::core::ffi::c_char,
                (*stbuf).st_mtim.tv_sec as ::core::ffi::c_ulong,
            ) as uint32_t);
        }
        if strleng < strsize && to_set & FUSE_SET_ATTR_CTIME != 0 {
            strleng = strleng.wrapping_add(snprintf(
                strbuff.offset(strleng as isize),
                strsize.wrapping_sub(strleng) as size_t,
                b"ctime=%lu;\0".as_ptr() as *const ::core::ffi::c_char,
                (*stbuf).st_ctim.tv_sec as ::core::ffi::c_ulong,
            ) as uint32_t);
        }
        if strleng < strsize && to_set & FUSE_SET_ATTR_SIZE != 0 {
            strleng = strleng.wrapping_add(snprintf(
                strbuff.offset(strleng as isize),
                strsize.wrapping_sub(strleng) as size_t,
                b"size=%llu;\0".as_ptr() as *const ::core::ffi::c_char,
                (*stbuf).st_size as ::core::ffi::c_ulonglong,
            ) as uint32_t);
        }
        if strleng > 0 as uint32_t {
            strleng = strleng.wrapping_sub(1);
        }
        *strbuff.offset(strleng as isize) = '\0' as ::core::ffi::c_char;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_setattr(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut stbuf: *mut stat,
    mut to_set: ::core::ffi::c_int,
    mut fi: *mut fuse_file_info,
) {
    unsafe {
        let mut o_stbuf: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        let mut maxfleng: uint64_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut setattr_str: [::core::ffi::c_char; 150] = [0; 150];
        let mut attrstr: [::core::ffi::c_char; 256] = [0; 256];
        let mut attr_timeout: ::core::ffi::c_double = 0.;
        let mut status: ::core::ffi::c_int = 0;
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        let mut setmask: uint8_t = 0 as uint8_t;
        ctx = *fuse_req_ctx(req);
        mfs_make_setattr_str(
            &raw mut setattr_str as *mut ::core::ffi::c_char,
            150 as uint32_t,
            stbuf,
            to_set,
        );
        mfs_stats_inc(OP_SETATTR as ::core::ffi::c_int as uint8_t);
        if debug_mode != 0 {
            if !fi.is_null() {
                oplog_printf(
                    &raw mut ctx,
                    b"setattr (%lu,0x%X,[%s]) [handle:%08X] ...\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    to_set,
                    &raw mut setattr_str as *mut ::core::ffi::c_char,
                    (*fi).fh as uint32_t,
                );
            } else {
                oplog_printf(
                    &raw mut ctx,
                    b"setattr (%lu,0x%X,[%s]) ...\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    to_set,
                    &raw mut setattr_str as *mut ::core::ffi::c_char,
                );
            }
            fprintf(
                stderr,
                b"setattr (%lu,0x%X,[%s])\n\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                to_set,
                &raw mut setattr_str as *mut ::core::ffi::c_char,
            );
        }
        if ino == MASTERINFO_INODE as fuse_ino_t {
            oplog_printf(
                &raw mut ctx,
                b"setattr (%lu,0x%X,[%s]): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                to_set,
                &raw mut setattr_str as *mut ::core::ffi::c_char,
                strerr(EPERM),
            );
            fuse_reply_err(req, EPERM);
            return;
        }
        if ino == STATS_INODE as fuse_ino_t {
            memset(
                &raw mut o_stbuf as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<stat>(),
            );
            mfs_attr_to_stat(
                ino as uint32_t,
                &raw mut statsattr as *mut uint8_t as *const uint8_t,
                &raw mut o_stbuf,
            );
            mfs_makeattrstr(
                &raw mut attrstr as *mut ::core::ffi::c_char,
                256 as uint32_t,
                &raw mut o_stbuf,
            );
            oplog_printf(
                &raw mut ctx,
                b"setattr (%lu,0x%X,[%s]) (internal node: STATS): OK (3600,%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                to_set,
                &raw mut setattr_str as *mut ::core::ffi::c_char,
                &raw mut attrstr as *mut ::core::ffi::c_char,
            );
            fuse_reply_attr(req, &raw mut o_stbuf, 3600.0f64);
            return;
        }
        if ino == PARAMS_INODE as fuse_ino_t {
            memset(
                &raw mut o_stbuf as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<stat>(),
            );
            mfs_attr_to_stat(
                ino as uint32_t,
                &raw mut paramsattr as *mut uint8_t as *const uint8_t,
                &raw mut o_stbuf,
            );
            mfs_makeattrstr(
                &raw mut attrstr as *mut ::core::ffi::c_char,
                256 as uint32_t,
                &raw mut o_stbuf,
            );
            oplog_printf(
                &raw mut ctx,
                b"setattr (%lu,0x%X,[%s]) (internal node: PARAMS): OK (3600,%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                to_set,
                &raw mut setattr_str as *mut ::core::ffi::c_char,
                &raw mut attrstr as *mut ::core::ffi::c_char,
            );
            fuse_reply_attr(req, &raw mut o_stbuf, 3600.0f64);
            return;
        }
        if ino == RANDOM_INODE as fuse_ino_t {
            memset(
                &raw mut o_stbuf as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<stat>(),
            );
            mfs_attr_to_stat(
                ino as uint32_t,
                &raw mut randomattr as *mut uint8_t as *const uint8_t,
                &raw mut o_stbuf,
            );
            mfs_makeattrstr(
                &raw mut attrstr as *mut ::core::ffi::c_char,
                256 as uint32_t,
                &raw mut o_stbuf,
            );
            oplog_printf(
                &raw mut ctx,
                b"setattr (%lu,0x%X,[%s]) (internal node: RANDOM): OK (3600,%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                to_set,
                &raw mut setattr_str as *mut ::core::ffi::c_char,
                &raw mut attrstr as *mut ::core::ffi::c_char,
            );
            fuse_reply_attr(req, &raw mut o_stbuf, 3600.0f64);
            return;
        }
        if ino == MOOSE_INODE as fuse_ino_t {
            memset(
                &raw mut o_stbuf as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<stat>(),
            );
            mfs_attr_to_stat(
                ino as uint32_t,
                &raw mut mooseattr as *mut uint8_t as *const uint8_t,
                &raw mut o_stbuf,
            );
            mfs_makeattrstr(
                &raw mut attrstr as *mut ::core::ffi::c_char,
                256 as uint32_t,
                &raw mut o_stbuf,
            );
            oplog_printf(
                &raw mut ctx,
                b"setattr (%lu,0x%X,[%s]) (internal node: MOOSE): OK (3600,%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                to_set,
                &raw mut setattr_str as *mut ::core::ffi::c_char,
                &raw mut attrstr as *mut ::core::ffi::c_char,
            );
            fuse_reply_attr(req, &raw mut o_stbuf, 3600.0f64);
            return;
        }
        if ino == OPLOG_INODE as fuse_ino_t || ino == OPHISTORY_INODE as fuse_ino_t {
            memset(
                &raw mut o_stbuf as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<stat>(),
            );
            mfs_attr_to_stat(
                ino as uint32_t,
                &raw mut oplogattr as *mut uint8_t as *const uint8_t,
                &raw mut o_stbuf,
            );
            mfs_makeattrstr(
                &raw mut attrstr as *mut ::core::ffi::c_char,
                256 as uint32_t,
                &raw mut o_stbuf,
            );
            oplog_printf(
                &raw mut ctx,
                b"setattr (%lu,0x%X,[%s]) (internal node: %s): OK (3600,%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                to_set,
                &raw mut setattr_str as *mut ::core::ffi::c_char,
                if ino == OPLOG_INODE as fuse_ino_t {
                    b"OPLOG\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"OPHISTORY\0".as_ptr() as *const ::core::ffi::c_char
                },
                &raw mut attrstr as *mut ::core::ffi::c_char,
            );
            fuse_reply_attr(req, &raw mut o_stbuf, 3600.0f64);
            return;
        }
        status = EINVAL;
        if to_set
            & (FUSE_SET_ATTR_MODE
                | FUSE_SET_ATTR_UID
                | FUSE_SET_ATTR_GID
                | FUSE_SET_ATTR_ATIME
                | FUSE_SET_ATTR_MTIME
                | FUSE_SET_ATTR_SIZE
                | FUSE_SET_ATTR_ATIME_NOW
                | FUSE_SET_ATTR_MTIME_NOW)
            == 0 as ::core::ffi::c_int
        {
            if full_permissions != 0 {
                let gids = getgroups::get_common(ctx.pid, ctx.uid, ctx.gid, false);
                status = fs_setattr(
                    ino as uint32_t,
                    (if !fi.is_null() || fs_isopen(ino as uint32_t) != 0 {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint8_t,
                    ctx.uid as uint32_t,
                    gids.len() as u32,
                    gids.as_slice().as_ptr() as *mut u32,
                    0 as uint8_t,
                    0 as uint16_t,
                    0 as uint32_t,
                    0 as uint32_t,
                    0 as uint32_t,
                    0 as uint32_t,
                    0 as uint8_t,
                    0 as uint8_t,
                    &raw mut attr as *mut uint8_t,
                ) as ::core::ffi::c_int;
            } else {
                let mut gidtmp: uint32_t = ctx.gid as uint32_t;
                status = fs_setattr(
                    ino as uint32_t,
                    (if !fi.is_null() || fs_isopen(ino as uint32_t) != 0 {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint8_t,
                    ctx.uid as uint32_t,
                    1 as uint32_t,
                    &raw mut gidtmp,
                    0 as uint8_t,
                    0 as uint16_t,
                    0 as uint32_t,
                    0 as uint32_t,
                    0 as uint32_t,
                    0 as uint32_t,
                    0 as uint8_t,
                    0 as uint8_t,
                    &raw mut attr as *mut uint8_t,
                ) as ::core::ffi::c_int;
            }
            if status == MFS_ERROR_ENOENT {
                status = sstats_get(ino as uint32_t, &raw mut attr as *mut uint8_t, 0 as uint8_t);
                if status == MFS_STATUS_OK {
                    mfs_attr_modify(to_set as uint32_t, &raw mut attr as *mut uint8_t, stbuf);
                }
            }
            if status == MFS_STATUS_OK {
                sstats_set(
                    ino as uint32_t,
                    &raw mut attr as *mut uint8_t as *const uint8_t,
                    0 as uint8_t,
                );
            }
            status = mfs_errorconv(status);
            if status != 0 as ::core::ffi::c_int {
                if !fi.is_null() {
                    oplog_printf(
                        &raw mut ctx,
                        b"setattr (%lu,0x%X,[%s]) [handle:%08X]: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ino as ::core::ffi::c_ulong,
                        to_set,
                        &raw mut setattr_str as *mut ::core::ffi::c_char,
                        (*fi).fh as uint32_t,
                        strerr(status),
                    );
                } else {
                    oplog_printf(
                        &raw mut ctx,
                        b"setattr (%lu,0x%X,[%s]) [no handle]: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ino as ::core::ffi::c_ulong,
                        to_set,
                        &raw mut setattr_str as *mut ::core::ffi::c_char,
                        strerr(status),
                    );
                }
                fuse_reply_err(req, status);
                return;
            }
        }
        if to_set & FUSE_SET_ATTR_SIZE != 0 {
            if (*stbuf).st_size < 0 as __off_t {
                if !fi.is_null() {
                    oplog_printf(
                        &raw mut ctx,
                        b"setattr (%lu,0x%X,[%s]) [handle:%08X]: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ino as ::core::ffi::c_ulong,
                        to_set,
                        &raw mut setattr_str as *mut ::core::ffi::c_char,
                        (*fi).fh as uint32_t,
                        strerr(EINVAL),
                    );
                } else {
                    oplog_printf(
                        &raw mut ctx,
                        b"setattr (%lu,0x%X,[%s]) [no handle]: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ino as ::core::ffi::c_ulong,
                        to_set,
                        &raw mut setattr_str as *mut ::core::ffi::c_char,
                        strerr(EINVAL),
                    );
                }
                fuse_reply_err(req, EINVAL);
                return;
            }
            if (*stbuf).st_size as int64_t >= MAX_FILE_SIZE {
                if !fi.is_null() {
                    oplog_printf(
                        &raw mut ctx,
                        b"setattr (%lu,0x%X,[%s]) [handle:%08X]: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ino as ::core::ffi::c_ulong,
                        to_set,
                        &raw mut setattr_str as *mut ::core::ffi::c_char,
                        (*fi).fh as uint32_t,
                        strerr(EFBIG),
                    );
                } else {
                    oplog_printf(
                        &raw mut ctx,
                        b"setattr (%lu,0x%X,[%s]) [no handle]: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ino as ::core::ffi::c_ulong,
                        to_set,
                        &raw mut setattr_str as *mut ::core::ffi::c_char,
                        strerr(EFBIG),
                    );
                }
                fuse_reply_err(req, EFBIG);
                return;
            }
            if !fi.is_null() {
                let mut fileinfo: Option<std::sync::Arc<FileInfo>> = None;
                fileinfo = fileinfo_get((*fi).fh);
                if (*fi).fh == 0 as uint64_t || fileinfo.is_none() {
                    oplog_printf(
                        &raw mut ctx,
                        b"setattr (%lu,0x%X,[%s]) [handle:%08X]: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ino as ::core::ffi::c_ulong,
                        to_set,
                        &raw mut setattr_str as *mut ::core::ffi::c_char,
                        (*fi).fh as uint32_t,
                        strerr(EBADF),
                    );
                    fuse_reply_err(req, EBADF);
                    return;
                }
                let mut state = fileinfo.as_ref().unwrap().state();
                if fileinfo.as_ref().unwrap().inode as fuse_ino_t != ino {
                    drop(state);
                    oplog_printf(
                        &raw mut ctx,
                        b"setattr (%lu,0x%X,[%s]) [handle:%08X]: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ino as ::core::ffi::c_ulong,
                        to_set,
                        &raw mut setattr_str as *mut ::core::ffi::c_char,
                        (*fi).fh as uint32_t,
                        strerr(EBADF),
                    );
                    fuse_reply_err(req, EBADF);
                    return;
                }
                if state.mode as ::core::ffi::c_int == IO_RO as ::core::ffi::c_int {
                    drop(state);
                    oplog_printf(
                        &raw mut ctx,
                        b"setattr (%lu,0x%X,[%s]) [handle:%08X]: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ino as ::core::ffi::c_ulong,
                        to_set,
                        &raw mut setattr_str as *mut ::core::ffi::c_char,
                        (*fi).fh as uint32_t,
                        strerr(EACCES),
                    );
                    fuse_reply_err(req, EACCES);
                    return;
                }
                drop(state);
            }
            write_data_flush_inode(ino as uint32_t);
            if full_permissions != 0 {
                let gids = getgroups::get_common(ctx.pid, ctx.uid, ctx.gid, false);
                status = do_truncate(
                    ino as uint32_t,
                    (if !fi.is_null() {
                        TRUNCATE_FLAG_OPENED
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint8_t,
                    ctx.uid as uint32_t,
                    gids.len() as u32,
                    gids.as_slice().as_ptr() as *mut u32,
                    (*stbuf).st_size as uint64_t,
                    &raw mut attr as *mut uint8_t,
                    ::core::ptr::null_mut::<uint64_t>(),
                ) as ::core::ffi::c_int;
            } else {
                let mut gidtmp_0: uint32_t = ctx.gid as uint32_t;
                status = do_truncate(
                    ino as uint32_t,
                    (if !fi.is_null() {
                        TRUNCATE_FLAG_OPENED
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint8_t,
                    ctx.uid as uint32_t,
                    1 as uint32_t,
                    &raw mut gidtmp_0,
                    (*stbuf).st_size as uint64_t,
                    &raw mut attr as *mut uint8_t,
                    ::core::ptr::null_mut::<uint64_t>(),
                ) as ::core::ffi::c_int;
            }
            status = mfs_errorconv(status);
            if status != 0 as ::core::ffi::c_int {
                if !fi.is_null() {
                    oplog_printf(
                        &raw mut ctx,
                        b"setattr (%lu,0x%X,[%s]) [handle:%08X]: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ino as ::core::ffi::c_ulong,
                        to_set,
                        &raw mut setattr_str as *mut ::core::ffi::c_char,
                        (*fi).fh as uint32_t,
                        strerr(status),
                    );
                } else {
                    oplog_printf(
                        &raw mut ctx,
                        b"setattr (%lu,0x%X,[%s]) [no handle]: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ino as ::core::ffi::c_ulong,
                        to_set,
                        &raw mut setattr_str as *mut ::core::ffi::c_char,
                        strerr(status),
                    );
                }
                fuse_reply_err(req, status);
                return;
            }
            plfsclient::chunksdatacache::clear_inode(
                ino as uint32_t,
                ((*stbuf).st_size / MFSCHUNKSIZE as __off_t) as uint32_t,
            );
            finfo_change_fleng(ino as uint32_t, (*stbuf).st_size as uint64_t);
            write_data_inode_setmaxfleng(ino as uint32_t, (*stbuf).st_size as uint64_t);
            read_inode_set_length_active(ino as uint32_t, (*stbuf).st_size as uint64_t);
        }
        if to_set
            & (FUSE_SET_ATTR_MODE
                | FUSE_SET_ATTR_UID
                | FUSE_SET_ATTR_GID
                | FUSE_SET_ATTR_ATIME
                | FUSE_SET_ATTR_MTIME
                | FUSE_SET_ATTR_ATIME_NOW
                | FUSE_SET_ATTR_MTIME_NOW)
            != 0
        {
            let mut masterversion: uint32_t = master_version();
            setmask = 0 as uint8_t;
            if to_set & FUSE_SET_ATTR_MODE != 0 {
                setmask = (setmask as ::core::ffi::c_int | SET_MODE_FLAG) as uint8_t;
                if no_xattrs == 0 as ::core::ffi::c_int && xattr_cache_on != 0 {
                    xattr_cache_del(
                        ino as uint32_t,
                        (6 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            + 5 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            + 6 as ::core::ffi::c_int) as uint32_t,
                        b"system.posix_acl_access\0".as_ptr() as *const ::core::ffi::c_char
                            as *const uint8_t,
                    );
                    xattr_cache_del(
                        ino as uint32_t,
                        0 as uint32_t,
                        ::core::ptr::null::<uint8_t>(),
                    );
                }
            }
            if to_set & FUSE_SET_ATTR_UID != 0 {
                setmask = (setmask as ::core::ffi::c_int | SET_UID_FLAG) as uint8_t;
            }
            if to_set & FUSE_SET_ATTR_GID != 0 {
                setmask = (setmask as ::core::ffi::c_int | SET_GID_FLAG) as uint8_t;
            }
            if (to_set & FUSE_SET_ATTR_ATIME_NOW != 0
                || to_set & FUSE_SET_ATTR_ATIME != 0 && (*stbuf).st_atim.tv_sec < 0 as __time_t)
                && masterversion
                    >= (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            13 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            13 as ::core::ffi::c_int
                        })) as uint32_t
            {
                setmask = (setmask as ::core::ffi::c_int | SET_ATIME_NOW_FLAG) as uint8_t;
            } else if to_set & FUSE_SET_ATTR_ATIME != 0 {
                setmask = (setmask as ::core::ffi::c_int | SET_ATIME_FLAG) as uint8_t;
            }
            if (to_set & FUSE_SET_ATTR_MTIME_NOW != 0
                || to_set & FUSE_SET_ATTR_MTIME != 0 && (*stbuf).st_mtim.tv_sec < 0 as __time_t)
                && masterversion
                    >= (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            13 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            13 as ::core::ffi::c_int
                        })) as uint32_t
            {
                setmask = (setmask as ::core::ffi::c_int | SET_MTIME_NOW_FLAG) as uint8_t;
            } else if to_set & FUSE_SET_ATTR_MTIME != 0 {
                setmask = (setmask as ::core::ffi::c_int | SET_MTIME_FLAG) as uint8_t;
            }
            if setmask as ::core::ffi::c_int & (SET_ATIME_NOW_FLAG | SET_ATIME_FLAG) != 0 {
                fs_no_atime(ino as uint32_t);
            }
            if setmask as ::core::ffi::c_int & (SET_MTIME_NOW_FLAG | SET_MTIME_FLAG) != 0 {
                fs_no_mtime(ino as uint32_t);
            }
            if full_permissions != 0 {
                let gids = getgroups::get_common(ctx.pid, ctx.uid, ctx.gid, false);
                status = fs_setattr(
                    ino as uint32_t,
                    (if !fi.is_null() || fs_isopen(ino as uint32_t) != 0 {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint8_t,
                    ctx.uid as uint32_t,
                    gids.len() as u32,
                    gids.as_slice().as_ptr() as *mut u32,
                    setmask,
                    ((*stbuf).st_mode & 0o7777 as __mode_t) as uint16_t,
                    (*stbuf).st_uid as uint32_t,
                    (*stbuf).st_gid as uint32_t,
                    (*stbuf).st_atim.tv_sec as uint32_t,
                    (*stbuf).st_mtim.tv_sec as uint32_t,
                    0 as uint8_t,
                    sugid_clear_mode as uint8_t,
                    &raw mut attr as *mut uint8_t,
                ) as ::core::ffi::c_int;
            } else {
                let mut gidtmp_1: uint32_t = ctx.gid as uint32_t;
                status = fs_setattr(
                    ino as uint32_t,
                    (if !fi.is_null() || fs_isopen(ino as uint32_t) != 0 {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint8_t,
                    ctx.uid as uint32_t,
                    1 as uint32_t,
                    &raw mut gidtmp_1,
                    setmask,
                    ((*stbuf).st_mode & 0o7777 as __mode_t) as uint16_t,
                    (*stbuf).st_uid as uint32_t,
                    (*stbuf).st_gid as uint32_t,
                    (*stbuf).st_atim.tv_sec as uint32_t,
                    (*stbuf).st_mtim.tv_sec as uint32_t,
                    0 as uint8_t,
                    sugid_clear_mode as uint8_t,
                    &raw mut attr as *mut uint8_t,
                ) as ::core::ffi::c_int;
            }
            if status == MFS_ERROR_ENOENT {
                status = sstats_get(ino as uint32_t, &raw mut attr as *mut uint8_t, 0 as uint8_t);
                if status == MFS_STATUS_OK {
                    mfs_attr_modify(to_set as uint32_t, &raw mut attr as *mut uint8_t, stbuf);
                }
            }
            if status == MFS_STATUS_OK {
                sstats_set(
                    ino as uint32_t,
                    &raw mut attr as *mut uint8_t as *const uint8_t,
                    0 as uint8_t,
                );
            }
            status = mfs_errorconv(status);
            if status != 0 as ::core::ffi::c_int {
                if !fi.is_null() {
                    oplog_printf(
                        &raw mut ctx,
                        b"setattr (%lu,0x%X,[%s]) [handle:%08X]: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ino as ::core::ffi::c_ulong,
                        to_set,
                        &raw mut setattr_str as *mut ::core::ffi::c_char,
                        (*fi).fh as uint32_t,
                        strerr(status),
                    );
                } else {
                    oplog_printf(
                        &raw mut ctx,
                        b"setattr (%lu,0x%X,[%s]) [no handle]: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ino as ::core::ffi::c_ulong,
                        to_set,
                        &raw mut setattr_str as *mut ::core::ffi::c_char,
                        strerr(status),
                    );
                }
                fuse_reply_err(req, status);
                return;
            }
        }
        if status != 0 as ::core::ffi::c_int {
            if !fi.is_null() {
                oplog_printf(
                    &raw mut ctx,
                    b"setattr (%lu,0x%X,[%s]) [handle:%08X]: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    to_set,
                    &raw mut setattr_str as *mut ::core::ffi::c_char,
                    (*fi).fh as uint32_t,
                    strerr(status),
                );
            } else {
                oplog_printf(
                    &raw mut ctx,
                    b"setattr (%lu,0x%X,[%s]) [no handle]: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    to_set,
                    &raw mut setattr_str as *mut ::core::ffi::c_char,
                    strerr(status),
                );
            }
            fuse_reply_err(req, status);
            return;
        }
        dcache::setattr(ino as uint32_t, &attr);
        if mfs_attr_get_type(&raw mut attr as *mut uint8_t as *const uint8_t) as ::core::ffi::c_int
            == TYPE_FILE
        {
            maxfleng = write_data_inode_getmaxfleng(ino as uint32_t);
        } else {
            maxfleng = 0 as uint64_t;
        }
        memset(
            &raw mut o_stbuf as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<stat>(),
        );
        mfs_attr_to_stat(
            ino as uint32_t,
            &raw mut attr as *mut uint8_t as *const uint8_t,
            &raw mut o_stbuf,
        );
        if maxfleng > o_stbuf.st_size as uint64_t {
            o_stbuf.st_size = maxfleng as __off_t;
            mfs_attr_set_fleng(&raw mut attr as *mut uint8_t, maxfleng);
        }
        fdcache::invalidate(ino as uint32_t);
        attr_timeout = if mfs_attr_get_mattr(&raw mut attr as *mut uint8_t as *const uint8_t)
            as ::core::ffi::c_int
            & MATTR_NOACACHE
            != 0
        {
            0.0f64
        } else {
            attr_cache_timeout
        };
        mfs_makeattrstr(
            &raw mut attrstr as *mut ::core::ffi::c_char,
            256 as uint32_t,
            &raw mut o_stbuf,
        );
        if !fi.is_null() {
            oplog_printf(
                &raw mut ctx,
                b"setattr (%lu,0x%X,[%s]) [handle:%08X]: OK (%.1lf,%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                to_set,
                &raw mut setattr_str as *mut ::core::ffi::c_char,
                (*fi).fh as uint32_t,
                attr_timeout,
                &raw mut attrstr as *mut ::core::ffi::c_char,
            );
        } else {
            oplog_printf(
                &raw mut ctx,
                b"setattr (%lu,0x%X,[%s]) [no handle]: OK (%.1lf,%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                to_set,
                &raw mut setattr_str as *mut ::core::ffi::c_char,
                attr_timeout,
                &raw mut attrstr as *mut ::core::ffi::c_char,
            );
        }
        fuse_reply_attr(req, &raw mut o_stbuf, attr_timeout);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_mknod(
    mut req: fuse_req_t,
    mut parent: fuse_ino_t,
    mut name: *const ::core::ffi::c_char,
    mut mode: mode_t,
    mut rdev: dev_t,
) {
    unsafe {
        let mut e: fuse_entry_param = fuse_entry_param {
            ino: 0,
            generation: 0,
            attr: stat {
                st_dev: 0,
                st_ino: 0,
                st_nlink: 0,
                st_mode: 0,
                st_uid: 0,
                st_gid: 0,
                __pad0: 0,
                st_rdev: 0,
                st_size: 0,
                st_blksize: 0,
                st_blocks: 0,
                st_atim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_mtim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_ctim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                __glibc_reserved: [0; 3],
            },
            attr_timeout: 0.,
            entry_timeout: 0.,
        };
        let mut inode: uint32_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut modestr: [::core::ffi::c_char; 11] = [0; 11];
        let mut attrstr: [::core::ffi::c_char; 256] = [0; 256];
        let mut mattr: uint8_t = 0;
        let mut nleng: uint32_t = 0;
        let mut cumask: uint16_t = 0;
        let mut status: ::core::ffi::c_int = 0;
        let mut r#type: uint8_t = 0;
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        ctx = *fuse_req_ctx(req);
        mfs_makemodestr(
            &raw mut modestr as *mut ::core::ffi::c_char,
            mode as uint16_t,
        );
        mfs_stats_inc(OP_MKNOD as ::core::ffi::c_int as uint8_t);
        if debug_mode != 0 {
            let mut umaskstr: [::core::ffi::c_char; 11] = [0; 11];
            mfs_makemodestr(
                &raw mut umaskstr as *mut ::core::ffi::c_char,
                ctx.umask as uint16_t,
            );
            oplog_printf(
                &raw mut ctx,
                b"mknod (%lu,%s,%s:0%04o/%s:0%04o,0x%08lX) ...\0".as_ptr()
                    as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                &raw mut modestr as *mut ::core::ffi::c_char,
                mode,
                (&raw mut umaskstr as *mut ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize),
                ctx.umask,
                rdev,
            );
            fprintf(
                stderr,
                b"mknod (%lu,%s,%s:0%04o/%s:0%04o,0x%08lX)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                &raw mut modestr as *mut ::core::ffi::c_char,
                mode,
                (&raw mut umaskstr as *mut ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize),
                ctx.umask,
                rdev,
            );
        }
        nleng = strlen(name) as uint32_t;
        if nleng > MFS_NAME_MAX as uint32_t {
            oplog_printf(
                &raw mut ctx,
                b"mknod (%lu,%s,%s:0%04o,0x%08lX): %s\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                &raw mut modestr as *mut ::core::ffi::c_char,
                mode,
                rdev,
                strerr(ENAMETOOLONG),
            );
            fuse_reply_err(req, ENAMETOOLONG);
            return;
        }
        if mode & __S_IFMT as mode_t == 0o10000 as mode_t {
            r#type = TYPE_FIFO as uint8_t;
        } else if mode & __S_IFMT as mode_t == 0o20000 as mode_t {
            r#type = TYPE_CHARDEV as uint8_t;
        } else if mode & __S_IFMT as mode_t == 0o60000 as mode_t {
            r#type = TYPE_BLOCKDEV as uint8_t;
        } else if mode & __S_IFMT as mode_t == 0o140000 as mode_t {
            r#type = TYPE_SOCKET as uint8_t;
        } else if mode & __S_IFMT as mode_t == 0o100000 as mode_t
            || mode & 0o170000 as mode_t == 0 as mode_t
        {
            r#type = TYPE_FILE as uint8_t;
        } else {
            oplog_printf(
                &raw mut ctx,
                b"mknod (%lu,%s,%s:0%04o,0x%08lX): %s\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                &raw mut modestr as *mut ::core::ffi::c_char,
                mode,
                rdev,
                strerr(EPERM),
            );
            fuse_reply_err(req, EPERM);
            return;
        }
        if parent == FUSE_ROOT_ID as fuse_ino_t {
            if *name.offset(0 as isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                && (strcmp(STATS_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(MASTERINFO_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(OPLOG_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(OPHISTORY_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(MOOSE_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(RANDOM_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(PARAMS_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int)
            {
                oplog_printf(
                    &raw mut ctx,
                    b"mknod (%lu,%s,%s:0%04o,0x%08lX): %s\0".as_ptr() as *const ::core::ffi::c_char,
                    parent as ::core::ffi::c_ulong,
                    name,
                    &raw mut modestr as *mut ::core::ffi::c_char,
                    mode,
                    rdev,
                    strerr(EACCES),
                );
                fuse_reply_err(req, EACCES);
                return;
            }
        }
        cumask = ctx.umask as uint16_t;
        if full_permissions != 0 {
            let gids = getgroups::get_common(ctx.pid, ctx.uid, ctx.gid, false);
            status = fs_mknod(
                parent as uint32_t,
                nleng as uint8_t,
                name as *const uint8_t,
                r#type,
                (mode & 0o7777 as mode_t) as uint16_t,
                cumask,
                ctx.uid as uint32_t,
                gids.len() as u32,
                gids.as_slice().as_ptr() as *mut u32,
                rdev as uint32_t,
                &raw mut inode,
                &raw mut attr as *mut uint8_t,
            ) as ::core::ffi::c_int;
        } else {
            let mut gidtmp: uint32_t = ctx.gid as uint32_t;
            status = fs_mknod(
                parent as uint32_t,
                nleng as uint8_t,
                name as *const uint8_t,
                r#type,
                (mode & 0o7777 as mode_t) as uint16_t,
                cumask,
                ctx.uid as uint32_t,
                1 as uint32_t,
                &raw mut gidtmp,
                rdev as uint32_t,
                &raw mut inode,
                &raw mut attr as *mut uint8_t,
            ) as ::core::ffi::c_int;
        }
        status = mfs_errorconv(status);
        if status != 0 as ::core::ffi::c_int {
            oplog_printf(
                &raw mut ctx,
                b"mknod (%lu,%s,%s:0%04o,0x%08lX): %s\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                &raw mut modestr as *mut ::core::ffi::c_char,
                mode,
                rdev,
                strerr(status),
            );
            fuse_reply_err(req, status);
        } else {
            negentry_cache_remove(parent as uint32_t, nleng as uint8_t, name as *const uint8_t);
            dcache::invalidate_attr(parent as uint32_t);
            memset(
                &raw mut e as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<fuse_entry_param>(),
            );
            e.ino = inode as fuse_ino_t;
            e.generation = 1 as uint64_t;
            mattr = mfs_attr_get_mattr(&raw mut attr as *mut uint8_t as *const uint8_t);
            e.attr_timeout = if mattr as ::core::ffi::c_int & MATTR_NOACACHE != 0 {
                0.0f64
            } else {
                attr_cache_timeout
            };
            e.entry_timeout = if mattr as ::core::ffi::c_int & MATTR_NOECACHE != 0 {
                0.0f64
            } else {
                entry_cache_timeout
            };
            mfs_attr_to_stat(
                inode,
                &raw mut attr as *mut uint8_t as *const uint8_t,
                &raw mut e.attr,
            );
            mfs_makeattrstr(
                &raw mut attrstr as *mut ::core::ffi::c_char,
                256 as uint32_t,
                &raw mut e.attr,
            );
            oplog_printf(
                &raw mut ctx,
                b"mknod (%lu,%s,%s:0%04o,0x%08lX): OK (%.1lf,%lu,%.1lf,%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                &raw mut modestr as *mut ::core::ffi::c_char,
                mode,
                rdev,
                e.entry_timeout,
                e.ino as ::core::ffi::c_ulong,
                e.attr_timeout,
                &raw mut attrstr as *mut ::core::ffi::c_char,
            );
            fuse_reply_entry(req, &raw mut e);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_unlink(
    mut req: fuse_req_t,
    mut parent: fuse_ino_t,
    mut name: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut nleng: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut status: ::core::ffi::c_int = 0;
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        ctx = *fuse_req_ctx(req);
        mfs_stats_inc(OP_UNLINK as ::core::ffi::c_int as uint8_t);
        if debug_mode != 0 {
            oplog_printf(
                &raw mut ctx,
                b"unlink (%lu,%s) ...\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
            );
            fprintf(
                stderr,
                b"unlink (%lu,%s)\n\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
            );
        }
        if parent == FUSE_ROOT_ID as fuse_ino_t {
            if *name.offset(0 as isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                && (strcmp(STATS_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(MASTERINFO_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(OPLOG_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(OPHISTORY_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(MOOSE_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(RANDOM_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(PARAMS_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int)
            {
                oplog_printf(
                    &raw mut ctx,
                    b"unlink (%lu,%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
                    parent as ::core::ffi::c_ulong,
                    name,
                    strerr(EACCES),
                );
                fuse_reply_err(req, EACCES);
                return;
            }
        }
        nleng = strlen(name) as uint32_t;
        if nleng > MFS_NAME_MAX as uint32_t {
            oplog_printf(
                &raw mut ctx,
                b"unlink (%lu,%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                strerr(ENAMETOOLONG),
            );
            fuse_reply_err(req, ENAMETOOLONG);
            return;
        }
        if full_permissions != 0 {
            let gids = getgroups::get_common(ctx.pid, ctx.uid, ctx.gid, false);
            status = fs_unlink(
                parent as uint32_t,
                nleng as uint8_t,
                name as *const uint8_t,
                ctx.uid as uint32_t,
                gids.len() as u32,
                gids.as_slice().as_ptr() as *mut u32,
                &raw mut inode,
            ) as ::core::ffi::c_int;
        } else {
            let mut gidtmp: uint32_t = ctx.gid as uint32_t;
            status = fs_unlink(
                parent as uint32_t,
                nleng as uint8_t,
                name as *const uint8_t,
                ctx.uid as uint32_t,
                1 as uint32_t,
                &raw mut gidtmp,
                &raw mut inode,
            ) as ::core::ffi::c_int;
        }
        status = mfs_errorconv(status);
        if status != 0 as ::core::ffi::c_int {
            oplog_printf(
                &raw mut ctx,
                b"unlink (%lu,%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                strerr(status),
            );
            fuse_reply_err(req, status);
        } else {
            negentry_cache_insert(parent as uint32_t, nleng as uint8_t, name as *const uint8_t);
            fdcache::invalidate(inode);
            dcache::invalidate_attr(parent as uint32_t);
            dcache::invalidate_name(
                parent as uint32_t,
                std::slice::from_raw_parts(name as *const uint8_t, nleng as usize),
            );
            oplog_printf(
                &raw mut ctx,
                b"unlink (%lu,%s): OK\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
            );
            fuse_reply_err(req, 0 as ::core::ffi::c_int);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_mkdir(
    mut req: fuse_req_t,
    mut parent: fuse_ino_t,
    mut name: *const ::core::ffi::c_char,
    mut mode: mode_t,
) {
    unsafe {
        let mut e: fuse_entry_param = fuse_entry_param {
            ino: 0,
            generation: 0,
            attr: stat {
                st_dev: 0,
                st_ino: 0,
                st_nlink: 0,
                st_mode: 0,
                st_uid: 0,
                st_gid: 0,
                __pad0: 0,
                st_rdev: 0,
                st_size: 0,
                st_blksize: 0,
                st_blocks: 0,
                st_atim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_mtim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_ctim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                __glibc_reserved: [0; 3],
            },
            attr_timeout: 0.,
            entry_timeout: 0.,
        };
        let mut inode: uint32_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut modestr: [::core::ffi::c_char; 11] = [0; 11];
        let mut attrstr: [::core::ffi::c_char; 256] = [0; 256];
        let mut mattr: uint8_t = 0;
        let mut nleng: uint32_t = 0;
        let mut cumask: uint16_t = 0;
        let mut status: ::core::ffi::c_int = 0;
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        ctx = *fuse_req_ctx(req);
        mfs_makemodestr(
            &raw mut modestr as *mut ::core::ffi::c_char,
            mode as uint16_t,
        );
        mfs_stats_inc(OP_MKDIR as ::core::ffi::c_int as uint8_t);
        if debug_mode != 0 {
            let mut umaskstr: [::core::ffi::c_char; 11] = [0; 11];
            mfs_makemodestr(
                &raw mut umaskstr as *mut ::core::ffi::c_char,
                ctx.umask as uint16_t,
            );
            oplog_printf(
                &raw mut ctx,
                b"mkdir (%lu,%s,d%s:0%04o/%s:0%04o) ...\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                (&raw mut modestr as *mut ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize),
                mode,
                (&raw mut umaskstr as *mut ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize),
                ctx.umask,
            );
            fprintf(
                stderr,
                b"mkdir (%lu,%s,d%s:0%04o/%s:0%04o)\n\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                (&raw mut modestr as *mut ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize),
                mode,
                (&raw mut umaskstr as *mut ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize),
                ctx.umask,
            );
        }
        if parent == FUSE_ROOT_ID as fuse_ino_t {
            if *name.offset(0 as isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                && (strcmp(STATS_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(MASTERINFO_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(OPLOG_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(OPHISTORY_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(MOOSE_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(RANDOM_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(PARAMS_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int)
            {
                oplog_printf(
                    &raw mut ctx,
                    b"mkdir (%lu,%s,d%s:0%04o): %s\0".as_ptr() as *const ::core::ffi::c_char,
                    parent as ::core::ffi::c_ulong,
                    name,
                    (&raw mut modestr as *mut ::core::ffi::c_char)
                        .offset(1 as ::core::ffi::c_int as isize),
                    mode,
                    strerr(EACCES),
                );
                fuse_reply_err(req, EACCES);
                return;
            }
        }
        nleng = strlen(name) as uint32_t;
        if nleng > MFS_NAME_MAX as uint32_t {
            oplog_printf(
                &raw mut ctx,
                b"mkdir (%lu,%s,d%s:0%04o): %s\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                (&raw mut modestr as *mut ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize),
                mode,
                strerr(ENAMETOOLONG),
            );
            fuse_reply_err(req, ENAMETOOLONG);
            return;
        }
        cumask = ctx.umask as uint16_t;
        if full_permissions != 0 {
            let gids = getgroups::get_common(ctx.pid, ctx.uid, ctx.gid, false);
            status = fs_mkdir(
                parent as uint32_t,
                nleng as uint8_t,
                name as *const uint8_t,
                mode as uint16_t,
                cumask,
                ctx.uid as uint32_t,
                gids.len() as u32,
                gids.as_slice().as_ptr() as *mut u32,
                mkdir_copy_sgid as uint8_t,
                &raw mut inode,
                &raw mut attr as *mut uint8_t,
            ) as ::core::ffi::c_int;
        } else {
            let mut gidtmp: uint32_t = ctx.gid as uint32_t;
            status = fs_mkdir(
                parent as uint32_t,
                nleng as uint8_t,
                name as *const uint8_t,
                mode as uint16_t,
                cumask,
                ctx.uid as uint32_t,
                1 as uint32_t,
                &raw mut gidtmp,
                mkdir_copy_sgid as uint8_t,
                &raw mut inode,
                &raw mut attr as *mut uint8_t,
            ) as ::core::ffi::c_int;
        }
        status = mfs_errorconv(status);
        if status != 0 as ::core::ffi::c_int {
            oplog_printf(
                &raw mut ctx,
                b"mkdir (%lu,%s,d%s:0%04o): %s\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                (&raw mut modestr as *mut ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize),
                mode,
                strerr(status),
            );
            fuse_reply_err(req, status);
        } else {
            sstats_set(
                inode,
                &raw mut attr as *mut uint8_t as *const uint8_t,
                1 as uint8_t,
            );
            sparents_add(
                inode,
                parent as uint32_t,
                (direntry_cache_timeout + 60 as ::core::ffi::c_int as ::core::ffi::c_double)
                    as uint32_t,
            );
            negentry_cache_remove(parent as uint32_t, nleng as uint8_t, name as *const uint8_t);
            dcache::invalidate_attr(parent as uint32_t);
            memset(
                &raw mut e as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<fuse_entry_param>(),
            );
            e.ino = inode as fuse_ino_t;
            e.generation = 1 as uint64_t;
            mattr = mfs_attr_get_mattr(&raw mut attr as *mut uint8_t as *const uint8_t);
            e.attr_timeout = if mattr as ::core::ffi::c_int & MATTR_NOACACHE != 0 {
                0.0f64
            } else {
                attr_cache_timeout
            };
            e.entry_timeout = if mattr as ::core::ffi::c_int & MATTR_NOECACHE != 0 {
                0.0f64
            } else {
                direntry_cache_timeout
            };
            if dinval != 0
                && mattr as ::core::ffi::c_int & MATTR_UNDELETABLE == 0 as ::core::ffi::c_int
            {
                dinval_add(
                    parent as uint32_t,
                    nleng as uint8_t,
                    name as *const uint8_t,
                    inode,
                );
            }
            mfs_attr_to_stat(
                inode,
                &raw mut attr as *mut uint8_t as *const uint8_t,
                &raw mut e.attr,
            );
            mfs_makeattrstr(
                &raw mut attrstr as *mut ::core::ffi::c_char,
                256 as uint32_t,
                &raw mut e.attr,
            );
            oplog_printf(
                &raw mut ctx,
                b"mkdir (%lu,%s,d%s:0%04o): OK (%.1lf,%lu,%.1lf,%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                (&raw mut modestr as *mut ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize),
                mode,
                e.entry_timeout,
                e.ino as ::core::ffi::c_ulong,
                e.attr_timeout,
                &raw mut attrstr as *mut ::core::ffi::c_char,
            );
            fuse_reply_entry(req, &raw mut e);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_rmdir(
    mut req: fuse_req_t,
    mut parent: fuse_ino_t,
    mut name: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut nleng: uint32_t = 0;
        let mut inode: uint32_t = 0;
        let mut status: ::core::ffi::c_int = 0;
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        ctx = *fuse_req_ctx(req);
        mfs_stats_inc(OP_RMDIR as ::core::ffi::c_int as uint8_t);
        if debug_mode != 0 {
            oplog_printf(
                &raw mut ctx,
                b"rmdir (%lu,%s) ...\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
            );
            fprintf(
                stderr,
                b"rmdir (%lu,%s)\n\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
            );
        }
        if parent == FUSE_ROOT_ID as fuse_ino_t {
            if *name.offset(0 as isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                && (strcmp(STATS_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(MASTERINFO_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(OPLOG_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(OPHISTORY_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(MOOSE_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(RANDOM_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(PARAMS_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int)
            {
                oplog_printf(
                    &raw mut ctx,
                    b"rmdir (%lu,%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
                    parent as ::core::ffi::c_ulong,
                    name,
                    strerr(EACCES),
                );
                fuse_reply_err(req, EACCES);
                return;
            }
        }
        nleng = strlen(name) as uint32_t;
        if nleng > MFS_NAME_MAX as uint32_t {
            oplog_printf(
                &raw mut ctx,
                b"rmdir (%lu,%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                strerr(ENAMETOOLONG),
            );
            fuse_reply_err(req, ENAMETOOLONG);
            return;
        }
        if full_permissions != 0 {
            let gids = getgroups::get_common(ctx.pid, ctx.uid, ctx.gid, false);
            status = fs_rmdir(
                parent as uint32_t,
                nleng as uint8_t,
                name as *const uint8_t,
                ctx.uid as uint32_t,
                gids.len() as u32,
                gids.as_slice().as_ptr() as *mut u32,
                &raw mut inode,
            ) as ::core::ffi::c_int;
        } else {
            let mut gidtmp: uint32_t = ctx.gid as uint32_t;
            status = fs_rmdir(
                parent as uint32_t,
                nleng as uint8_t,
                name as *const uint8_t,
                ctx.uid as uint32_t,
                1 as uint32_t,
                &raw mut gidtmp,
                &raw mut inode,
            ) as ::core::ffi::c_int;
        }
        status = mfs_errorconv(status);
        if status != 0 as ::core::ffi::c_int {
            oplog_printf(
                &raw mut ctx,
                b"rmdir (%lu,%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                strerr(status),
            );
            fuse_reply_err(req, status);
        } else {
            negentry_cache_insert(parent as uint32_t, nleng as uint8_t, name as *const uint8_t);
            dcache::invalidate_attr(parent as uint32_t);
            dcache::invalidate_name(
                parent as uint32_t,
                std::slice::from_raw_parts(name as *const uint8_t, nleng as usize),
            );
            if dinval != 0 {
                dinval_remove(parent as uint32_t, nleng as uint8_t, name as *const uint8_t);
            }
            oplog_printf(
                &raw mut ctx,
                b"rmdir (%lu,%s): OK\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
            );
            fuse_reply_err(req, 0 as ::core::ffi::c_int);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_symlink(
    mut req: fuse_req_t,
    mut path: *const ::core::ffi::c_char,
    mut parent: fuse_ino_t,
    mut name: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut e: fuse_entry_param = fuse_entry_param {
            ino: 0,
            generation: 0,
            attr: stat {
                st_dev: 0,
                st_ino: 0,
                st_nlink: 0,
                st_mode: 0,
                st_uid: 0,
                st_gid: 0,
                __pad0: 0,
                st_rdev: 0,
                st_size: 0,
                st_blksize: 0,
                st_blocks: 0,
                st_atim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_mtim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_ctim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                __glibc_reserved: [0; 3],
            },
            attr_timeout: 0.,
            entry_timeout: 0.,
        };
        let mut inode: uint32_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut attrstr: [::core::ffi::c_char; 256] = [0; 256];
        let mut mattr: uint8_t = 0;
        let mut nleng: uint32_t = 0;
        let mut status: ::core::ffi::c_int = 0;
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        ctx = *fuse_req_ctx(req);
        mfs_stats_inc(OP_SYMLINK as ::core::ffi::c_int as uint8_t);
        if debug_mode != 0 {
            oplog_printf(
                &raw mut ctx,
                b"symlink (%s,%lu,%s) ...\0".as_ptr() as *const ::core::ffi::c_char,
                path,
                parent as ::core::ffi::c_ulong,
                name,
            );
            fprintf(
                stderr,
                b"symlink (%s,%lu,%s)\n\0".as_ptr() as *const ::core::ffi::c_char,
                path,
                parent as ::core::ffi::c_ulong,
                name,
            );
        }
        if parent == FUSE_ROOT_ID as fuse_ino_t {
            if *name.offset(0 as isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                && (strcmp(STATS_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(MASTERINFO_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(OPLOG_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(OPHISTORY_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(MOOSE_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(RANDOM_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(PARAMS_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int)
            {
                oplog_printf(
                    &raw mut ctx,
                    b"symlink (%s,%lu,%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
                    path,
                    parent as ::core::ffi::c_ulong,
                    name,
                    strerr(EACCES),
                );
                fuse_reply_err(req, EACCES);
                return;
            }
        }
        nleng = strlen(name) as uint32_t;
        if nleng > MFS_NAME_MAX as uint32_t
            || strlen(path).wrapping_add(1 as size_t) > MFS_SYMLINK_MAX as size_t
        {
            oplog_printf(
                &raw mut ctx,
                b"symlink (%s,%lu,%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
                path,
                parent as ::core::ffi::c_ulong,
                name,
                strerr(ENAMETOOLONG),
            );
            fuse_reply_err(req, ENAMETOOLONG);
            return;
        }
        if full_permissions != 0 {
            let gids = getgroups::get_common(ctx.pid, ctx.uid, ctx.gid, false);
            status = fs_symlink(
                parent as uint32_t,
                nleng as uint8_t,
                name as *const uint8_t,
                path as *const uint8_t,
                ctx.uid as uint32_t,
                gids.len() as u32,
                gids.as_slice().as_ptr() as *mut u32,
                &raw mut inode,
                &raw mut attr as *mut uint8_t,
            ) as ::core::ffi::c_int;
        } else {
            let mut gidtmp: uint32_t = ctx.gid as uint32_t;
            status = fs_symlink(
                parent as uint32_t,
                nleng as uint8_t,
                name as *const uint8_t,
                path as *const uint8_t,
                ctx.uid as uint32_t,
                1 as uint32_t,
                &raw mut gidtmp,
                &raw mut inode,
                &raw mut attr as *mut uint8_t,
            ) as ::core::ffi::c_int;
        }
        status = mfs_errorconv(status);
        if status != 0 as ::core::ffi::c_int {
            oplog_printf(
                &raw mut ctx,
                b"symlink (%s,%lu,%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
                path,
                parent as ::core::ffi::c_ulong,
                name,
                strerr(status),
            );
            fuse_reply_err(req, status);
        } else {
            negentry_cache_remove(parent as uint32_t, nleng as uint8_t, name as *const uint8_t);
            dcache::invalidate_attr(parent as uint32_t);
            memset(
                &raw mut e as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<fuse_entry_param>(),
            );
            e.ino = inode as fuse_ino_t;
            e.generation = 1 as uint64_t;
            mattr = mfs_attr_get_mattr(&raw mut attr as *mut uint8_t as *const uint8_t);
            e.attr_timeout = if mattr as ::core::ffi::c_int & MATTR_NOACACHE != 0 {
                0.0f64
            } else {
                attr_cache_timeout
            };
            e.entry_timeout = if mattr as ::core::ffi::c_int & MATTR_NOECACHE != 0 {
                0.0f64
            } else {
                entry_cache_timeout
            };
            mfs_attr_to_stat(
                inode,
                &raw mut attr as *mut uint8_t as *const uint8_t,
                &raw mut e.attr,
            );
            mfs_makeattrstr(
                &raw mut attrstr as *mut ::core::ffi::c_char,
                256 as uint32_t,
                &raw mut e.attr,
            );
            oplog_printf(
                &raw mut ctx,
                b"symlink (%s,%lu,%s): OK (%.1lf,%lu,%.1lf,%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                path,
                parent as ::core::ffi::c_ulong,
                name,
                e.entry_timeout,
                e.ino as ::core::ffi::c_ulong,
                e.attr_timeout,
                &raw mut attrstr as *mut ::core::ffi::c_char,
            );
            fuse_reply_entry(req, &raw mut e);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_readlink(mut req: fuse_req_t, mut ino: fuse_ino_t) {
    unsafe {
        let mut status: ::core::ffi::c_int = 0;
        let mut path: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut cpath: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        cpath = ::core::ptr::null::<uint8_t>();
        ctx = *fuse_req_ctx(req);
        if debug_mode != 0 {
            oplog_printf(
                &raw mut ctx,
                b"readlink (%lu) ...\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
            );
            fprintf(
                stderr,
                b"readlink (%lu)\n\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
            );
        }
        path = symlink_cache_search(ino as uint32_t);
        if !path.is_null() {
            mfs_stats_inc(OP_READLINK_CACHED as ::core::ffi::c_int as uint8_t);
            oplog_printf(
                &raw mut ctx,
                b"readlink (%lu) (using cache): OK (%s)\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                path as *mut ::core::ffi::c_char,
            );
            fuse_reply_readlink(req, path as *mut ::core::ffi::c_char);
            free(path as *mut ::core::ffi::c_void);
            return;
        }
        mfs_stats_inc(OP_READLINK_MASTER as ::core::ffi::c_int as uint8_t);
        status = fs_readlink(ino as uint32_t, &raw mut cpath) as ::core::ffi::c_int;
        status = mfs_errorconv(status);
        if status != 0 as ::core::ffi::c_int {
            oplog_printf(
                &raw mut ctx,
                b"readlink (%lu): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                strerr(status),
            );
            fuse_reply_err(req, status);
        } else {
            dcache::invalidate_attr(ino as uint32_t);
            symlink_cache_insert(ino as uint32_t, cpath);
            oplog_printf(
                &raw mut ctx,
                b"readlink (%lu): OK (%s)\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                cpath as *mut ::core::ffi::c_char,
            );
            fuse_reply_readlink(req, cpath as *mut ::core::ffi::c_char);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_rename(
    mut req: fuse_req_t,
    mut parent: fuse_ino_t,
    mut name: *const ::core::ffi::c_char,
    mut newparent: fuse_ino_t,
    mut newname: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_uint,
) {
    unsafe {
        let mut nleng: uint32_t = 0;
        let mut newnleng: uint32_t = 0;
        let mut status: ::core::ffi::c_int = 0;
        let mut inode: uint32_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        let mut mfsflags: uint8_t = 0;
        let mut mask: ::core::ffi::c_int = 0;
        ctx = *fuse_req_ctx(req);
        mfs_stats_inc(OP_RENAME as ::core::ffi::c_int as uint8_t);
        if debug_mode != 0 {
            oplog_printf(
                &raw mut ctx,
                b"rename (%lu,%s,%lu,%s,%u) ...\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                newparent as ::core::ffi::c_ulong,
                newname,
                flags,
            );
            fprintf(
                stderr,
                b"rename (%lu,%s,%lu,%s,%u)\n\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                newparent as ::core::ffi::c_ulong,
                newname,
                flags,
            );
        }
        mask = 0 as ::core::ffi::c_int;
        mask |= RENAME_EXCHANGE;
        mask |= RENAME_NOREPLACE;
        if flags & !mask as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
            || flags != 0 as ::core::ffi::c_uint
                && master_version()
                    < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 18 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })) as uint32_t
        {
            oplog_printf(
                &raw mut ctx,
                b"rename (%lu,%s,%lu,%s,%u): %s\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                newparent as ::core::ffi::c_ulong,
                newname,
                flags,
                strerr(EINVAL),
            );
            fuse_reply_err(req, EINVAL);
            return;
        }
        if parent == FUSE_ROOT_ID as fuse_ino_t {
            if *name.offset(0 as isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                && (strcmp(STATS_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(MASTERINFO_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(OPLOG_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(OPHISTORY_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(MOOSE_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(RANDOM_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(PARAMS_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int)
            {
                oplog_printf(
                    &raw mut ctx,
                    b"rename (%lu,%s,%lu,%s,%u): %s\0".as_ptr() as *const ::core::ffi::c_char,
                    parent as ::core::ffi::c_ulong,
                    name,
                    newparent as ::core::ffi::c_ulong,
                    newname,
                    flags,
                    strerr(EACCES),
                );
                fuse_reply_err(req, EACCES);
                return;
            }
        }
        if newparent == FUSE_ROOT_ID as fuse_ino_t {
            if *newname.offset(0 as isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                && (strcmp(STATS_NAME.as_ptr(), newname) == 0 as ::core::ffi::c_int
                    || strcmp(MASTERINFO_NAME.as_ptr(), newname) == 0 as ::core::ffi::c_int
                    || strcmp(OPLOG_NAME.as_ptr(), newname) == 0 as ::core::ffi::c_int
                    || strcmp(OPHISTORY_NAME.as_ptr(), newname) == 0 as ::core::ffi::c_int
                    || strcmp(MOOSE_NAME.as_ptr(), newname) == 0 as ::core::ffi::c_int
                    || strcmp(RANDOM_NAME.as_ptr(), newname) == 0 as ::core::ffi::c_int
                    || strcmp(PARAMS_NAME.as_ptr(), newname) == 0 as ::core::ffi::c_int)
            {
                oplog_printf(
                    &raw mut ctx,
                    b"rename (%lu,%s,%lu,%s,%u): %s\0".as_ptr() as *const ::core::ffi::c_char,
                    parent as ::core::ffi::c_ulong,
                    name,
                    newparent as ::core::ffi::c_ulong,
                    newname,
                    flags,
                    strerr(EACCES),
                );
                fuse_reply_err(req, EACCES);
                return;
            }
        }
        nleng = strlen(name) as uint32_t;
        if nleng > MFS_NAME_MAX as uint32_t {
            oplog_printf(
                &raw mut ctx,
                b"rename (%lu,%s,%lu,%s,%u): %s\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                newparent as ::core::ffi::c_ulong,
                newname,
                flags,
                strerr(ENAMETOOLONG),
            );
            fuse_reply_err(req, ENAMETOOLONG);
            return;
        }
        newnleng = strlen(newname) as uint32_t;
        if newnleng > MFS_NAME_MAX as uint32_t {
            oplog_printf(
                &raw mut ctx,
                b"rename (%lu,%s,%lu,%s,%u): %s\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                newparent as ::core::ffi::c_ulong,
                newname,
                flags,
                strerr(ENAMETOOLONG),
            );
            fuse_reply_err(req, ENAMETOOLONG);
            return;
        }
        mfsflags = 0 as uint8_t;
        if flags & RENAME_EXCHANGE as ::core::ffi::c_uint != 0 {
            mfsflags = (mfsflags as ::core::ffi::c_int | MFS_RENAME_EXCHANGE) as uint8_t;
        }
        if flags & RENAME_NOREPLACE as ::core::ffi::c_uint != 0 {
            mfsflags = (mfsflags as ::core::ffi::c_int | MFS_RENAME_NOREPLACE) as uint8_t;
        }
        if full_permissions != 0 {
            let gids = getgroups::get_common(ctx.pid, ctx.uid, ctx.gid, false);
            status = fs_rename(
                parent as uint32_t,
                nleng as uint8_t,
                name as *const uint8_t,
                newparent as uint32_t,
                newnleng as uint8_t,
                newname as *const uint8_t,
                ctx.uid as uint32_t,
                gids.len() as u32,
                gids.as_slice().as_ptr() as *mut u32,
                mfsflags,
                &raw mut inode,
                &raw mut attr as *mut uint8_t,
            ) as ::core::ffi::c_int;
        } else {
            let mut gidtmp: uint32_t = ctx.gid as uint32_t;
            status = fs_rename(
                parent as uint32_t,
                nleng as uint8_t,
                name as *const uint8_t,
                newparent as uint32_t,
                newnleng as uint8_t,
                newname as *const uint8_t,
                ctx.uid as uint32_t,
                1 as uint32_t,
                &raw mut gidtmp,
                mfsflags,
                &raw mut inode,
                &raw mut attr as *mut uint8_t,
            ) as ::core::ffi::c_int;
        }
        status = mfs_errorconv(status);
        if status != 0 as ::core::ffi::c_int {
            oplog_printf(
                &raw mut ctx,
                b"rename (%lu,%s,%lu,%s,%u): %s\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                newparent as ::core::ffi::c_ulong,
                newname,
                flags,
                strerr(status),
            );
            fuse_reply_err(req, status);
        } else {
            if mfs_attr_get_type(&raw mut attr as *mut uint8_t as *const uint8_t)
                as ::core::ffi::c_int
                == TYPE_DIRECTORY
            {
                sparents_add(
                    inode,
                    newparent as uint32_t,
                    (direntry_cache_timeout + 60 as ::core::ffi::c_int as ::core::ffi::c_double)
                        as uint32_t,
                );
            }
            negentry_cache_insert(parent as uint32_t, nleng as uint8_t, name as *const uint8_t);
            negentry_cache_remove(
                newparent as uint32_t,
                newnleng as uint8_t,
                newname as *const uint8_t,
            );
            dcache::invalidate_attr(parent as uint32_t);
            if newparent != parent {
                dcache::invalidate_attr(newparent as uint32_t);
            }
            dcache::invalidate_name(
                parent as uint32_t,
                std::slice::from_raw_parts(name as *const uint8_t, nleng as usize),
            );
            if dinval != 0
                && mfs_attr_get_type(&raw mut attr as *mut uint8_t as *const uint8_t)
                    as ::core::ffi::c_int
                    == TYPE_DIRECTORY
            {
                dinval_remove(parent as uint32_t, nleng as uint8_t, name as *const uint8_t);
                if mfs_attr_get_mattr(&raw mut attr as *mut uint8_t as *const uint8_t)
                    as ::core::ffi::c_int
                    & MATTR_UNDELETABLE
                    == 0 as ::core::ffi::c_int
                {
                    dinval_add(
                        newparent as uint32_t,
                        newnleng as uint8_t,
                        newname as *const uint8_t,
                        inode,
                    );
                }
            }
            oplog_printf(
                &raw mut ctx,
                b"rename (%lu,%s,%lu,%s,%u): OK\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                newparent as ::core::ffi::c_ulong,
                newname,
                flags,
            );
            fuse_reply_err(req, 0 as ::core::ffi::c_int);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_link(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut newparent: fuse_ino_t,
    mut newname: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut newnleng: uint32_t = 0;
        let mut status: ::core::ffi::c_int = 0;
        let mut e: fuse_entry_param = fuse_entry_param {
            ino: 0,
            generation: 0,
            attr: stat {
                st_dev: 0,
                st_ino: 0,
                st_nlink: 0,
                st_mode: 0,
                st_uid: 0,
                st_gid: 0,
                __pad0: 0,
                st_rdev: 0,
                st_size: 0,
                st_blksize: 0,
                st_blocks: 0,
                st_atim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_mtim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_ctim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                __glibc_reserved: [0; 3],
            },
            attr_timeout: 0.,
            entry_timeout: 0.,
        };
        let mut inode: uint32_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut attrstr: [::core::ffi::c_char; 256] = [0; 256];
        let mut mattr: uint8_t = 0;
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        ctx = *fuse_req_ctx(req);
        mfs_stats_inc(OP_LINK as ::core::ffi::c_int as uint8_t);
        if debug_mode != 0 {
            oplog_printf(
                &raw mut ctx,
                b"link (%lu,%lu,%s) ...\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                newparent as ::core::ffi::c_ulong,
                newname,
            );
            fprintf(
                stderr,
                b"link (%lu,%lu,%s)\n\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                newparent as ::core::ffi::c_ulong,
                newname,
            );
        }
        if ino >= MIN_SPECIAL_INODE as fuse_ino_t {
            oplog_printf(
                &raw mut ctx,
                b"link (%lu,%lu,%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                newparent as ::core::ffi::c_ulong,
                newname,
                strerr(EACCES),
            );
            fuse_reply_err(req, EACCES);
            return;
        }
        if newparent == FUSE_ROOT_ID as fuse_ino_t {
            if *newname.offset(0 as isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                && (strcmp(STATS_NAME.as_ptr(), newname) == 0 as ::core::ffi::c_int
                    || strcmp(MASTERINFO_NAME.as_ptr(), newname) == 0 as ::core::ffi::c_int
                    || strcmp(OPLOG_NAME.as_ptr(), newname) == 0 as ::core::ffi::c_int
                    || strcmp(OPHISTORY_NAME.as_ptr(), newname) == 0 as ::core::ffi::c_int
                    || strcmp(MOOSE_NAME.as_ptr(), newname) == 0 as ::core::ffi::c_int
                    || strcmp(RANDOM_NAME.as_ptr(), newname) == 0 as ::core::ffi::c_int
                    || strcmp(PARAMS_NAME.as_ptr(), newname) == 0 as ::core::ffi::c_int)
            {
                oplog_printf(
                    &raw mut ctx,
                    b"link (%lu,%lu,%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    newparent as ::core::ffi::c_ulong,
                    newname,
                    strerr(EACCES),
                );
                fuse_reply_err(req, EACCES);
                return;
            }
        }
        newnleng = strlen(newname) as uint32_t;
        if newnleng > MFS_NAME_MAX as uint32_t {
            oplog_printf(
                &raw mut ctx,
                b"link (%lu,%lu,%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                newparent as ::core::ffi::c_ulong,
                newname,
                strerr(ENAMETOOLONG),
            );
            fuse_reply_err(req, ENAMETOOLONG);
            return;
        }
        if full_permissions != 0 {
            let gids = getgroups::get_common(ctx.pid, ctx.uid, ctx.gid, false);
            status = fs_link(
                ino as uint32_t,
                newparent as uint32_t,
                newnleng as uint8_t,
                newname as *const uint8_t,
                ctx.uid as uint32_t,
                gids.len() as u32,
                gids.as_slice().as_ptr() as *mut u32,
                &raw mut inode,
                &raw mut attr as *mut uint8_t,
            ) as ::core::ffi::c_int;
        } else {
            let mut gidtmp: uint32_t = ctx.gid as uint32_t;
            status = fs_link(
                ino as uint32_t,
                newparent as uint32_t,
                newnleng as uint8_t,
                newname as *const uint8_t,
                ctx.uid as uint32_t,
                1 as uint32_t,
                &raw mut gidtmp,
                &raw mut inode,
                &raw mut attr as *mut uint8_t,
            ) as ::core::ffi::c_int;
        }
        status = mfs_errorconv(status);
        if status != 0 as ::core::ffi::c_int {
            oplog_printf(
                &raw mut ctx,
                b"link (%lu,%lu,%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                newparent as ::core::ffi::c_ulong,
                newname,
                strerr(status),
            );
            fuse_reply_err(req, status);
        } else {
            negentry_cache_remove(
                newparent as uint32_t,
                newnleng as uint8_t,
                newname as *const uint8_t,
            );
            if ino != inode as fuse_ino_t {
                dcache::invalidate_attr(ino as uint32_t);
            }
            dcache::invalidate_attr(newparent as uint32_t);
            dcache::setattr(inode, &attr);
            memset(
                &raw mut e as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<fuse_entry_param>(),
            );
            e.ino = inode as fuse_ino_t;
            e.generation = 1 as uint64_t;
            mattr = mfs_attr_get_mattr(&raw mut attr as *mut uint8_t as *const uint8_t);
            e.attr_timeout = if mattr as ::core::ffi::c_int & MATTR_NOACACHE != 0 {
                0.0f64
            } else {
                attr_cache_timeout
            };
            e.entry_timeout = if mattr as ::core::ffi::c_int & MATTR_NOECACHE != 0 {
                0.0f64
            } else {
                entry_cache_timeout
            };
            mfs_attr_to_stat(
                inode,
                &raw mut attr as *mut uint8_t as *const uint8_t,
                &raw mut e.attr,
            );
            mfs_makeattrstr(
                &raw mut attrstr as *mut ::core::ffi::c_char,
                256 as uint32_t,
                &raw mut e.attr,
            );
            oplog_printf(
                &raw mut ctx,
                b"link (%lu,%lu,%s): OK (%.1lf,%lu,%.1lf,%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                newparent as ::core::ffi::c_ulong,
                newname,
                e.entry_timeout,
                e.ino as ::core::ffi::c_ulong,
                e.attr_timeout,
                &raw mut attrstr as *mut ::core::ffi::c_char,
            );
            fuse_reply_entry(req, &raw mut e);
        };
    }
}
#[inline]
unsafe fn snapshot_dir_groups(ctx: &fuse_ctx) -> Vec<u32> {
    unsafe {
        if full_permissions == 0 {
            return Vec::new();
        }
        getgroups::get_common(ctx.pid, ctx.uid, ctx.gid, false)
            .as_slice()
            .to_vec()
    }
}

#[inline]
fn new_dir_handle(ino: fuse_ino_t, ctx: &fuse_ctx, groups: Vec<u32>, sustained: bool) -> u64 {
    dirbuf::insert(std::sync::Arc::new(dirbuf::DirSlot::new(
        ino as u32,
        dirbuf::Credentials {
            pid: ctx.pid,
            uid: ctx.uid,
            gid: ctx.gid,
            groups,
        },
        sustained,
    )))
}

unsafe fn fetch_dir(request: dirbuf::FetchRequest) -> dirbuf::FetchReply {
    unsafe {
        let mut edge = request.edge;
        let mut data = ::core::ptr::null::<u8>();
        let mut size = 0u32;
        let mut fallback_gid = request.credentials.gid;
        let (gid_count, gids) = if request.credentials.groups.is_empty() {
            (1, &raw mut fallback_gid)
        } else {
            (
                request.credentials.groups.len() as u32,
                request.credentials.groups.as_ptr() as *mut u32,
            )
        };
        let mut status = fs_readdir(
            request.inode,
            request.credentials.uid,
            gid_count,
            gids,
            &raw mut edge,
            READDIR_EDGELIMIT as u32,
            request.format.as_u8(),
            0,
            &raw mut data,
            &raw mut size,
        );
        // Master response storage is transient. Copy before crossing FFI boundary.
        let bytes = if status == MFS_STATUS_OK as u8 && size != 0 {
            if data.is_null() {
                status = dirbuf::MFS_ERROR_IO;
                Vec::new()
            } else {
                std::slice::from_raw_parts(data, size as usize).to_vec()
            }
        } else {
            Vec::new()
        };
        dirbuf::FetchReply {
            status,
            edge,
            bytes,
        }
    }
}

fn dir_handle(fi: *mut fuse_file_info) -> Option<(u64, std::sync::Arc<dirbuf::DirSlot>)> {
    if fi.is_null() {
        return None;
    }
    let token = unsafe { (*fi).fh };
    dirbuf::get(token).map(|slot| (token, slot))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_opendir(req: fuse_req_t, ino: fuse_ino_t, fi: *mut fuse_file_info) {
    unsafe {
        let ctx = *fuse_req_ctx(req);
        let mut attr = [0u8; 36];
        mfs_stats_inc(OP_OPENDIR as u8);
        if debug_mode != 0 {
            oplog_printf(
                &raw const ctx,
                b"opendir (%lu) ...\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
            );
            fprintf(
                stderr,
                b"opendir (%lu)\n\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
            );
        }
        if ino >= MIN_SPECIAL_INODE as fuse_ino_t {
            oplog_printf(
                &raw const ctx,
                b"opendir (%lu): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                strerr(ENOTDIR),
            );
            fuse_reply_err(req, ENOTDIR);
        }

        let mut status = if mfs_disables & DISABLE_READDIR as u32 != 0 {
            MFS_ERROR_EPERM
        } else if full_permissions != 0 {
            let gids = getgroups::get_common(ctx.pid, ctx.uid, ctx.gid, false);
            let status = fs_access(
                ino as u32,
                ctx.uid,
                gids.len() as u32,
                gids.as_slice().as_ptr() as *mut u32,
                MODE_MASK_R as u16,
            ) as i32;
            status
        } else {
            let mut gid = ctx.gid;
            fs_access(ino as u32, ctx.uid, 1, &raw mut gid, MODE_MASK_R as u16) as i32
        };

        if status == MFS_ERROR_ENOENT
            && sstats_get(ino as u32, attr.as_mut_ptr(), 0) == MFS_STATUS_OK
        {
            status = if full_permissions != 0 {
                let gids = getgroups::get_common(ctx.pid, ctx.uid, ctx.gid, false);
                let status = mfs_access_test(
                    attr.as_ptr(),
                    MODE_MASK_R,
                    ctx.uid,
                    gids.len() as u32,
                    gids.as_slice().as_ptr() as *mut u32,
                );
                status
            } else {
                let mut gid = ctx.gid;
                mfs_access_test(attr.as_ptr(), MODE_MASK_R, ctx.uid, 1, &raw mut gid)
            };
            if status != MFS_STATUS_OK {
                let error = mfs_errorconv(status);
                oplog_printf(
                    &raw const ctx,
                    b"opendir (%lu): %s\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    strerr(error),
                );
                fuse_reply_err(req, error);
                return;
            }
            let token = new_dir_handle(ino, &ctx, Vec::new(), true);
            (*fi).fh = token;
            oplog_printf(
                &raw const ctx,
                b"sustained opendir (%lu): forced OK with empty directory\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
            );
            // libfuse rejected handle: release on every negative result to avoid leaks.
            if fuse_reply_open(req, fi) < 0 {
                dirbuf::release(token);
                (*fi).fh = 0;
            }
            return;
        }

        if status != MFS_STATUS_OK {
            let error = mfs_errorconv(status);
            oplog_printf(
                &raw const ctx,
                b"opendir (%lu): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                strerr(error),
            );
            fuse_reply_err(req, error);
            return;
        }

        let token = new_dir_handle(ino, &ctx, snapshot_dir_groups(&ctx), false);
        (*fi).fh = token;
        oplog_printf(
            &raw const ctx,
            b"opendir (%lu): OK [handle:%016lX]\0".as_ptr() as *const ::core::ffi::c_char,
            ino as ::core::ffi::c_ulong,
            token as ::core::ffi::c_ulong,
        );
        // libfuse rejected handle: release on every negative result to avoid leaks.
        if fuse_reply_open(req, fi) < 0 {
            dirbuf::release(token);
            (*fi).fh = 0;
        }
    }
}

unsafe fn read_dir_chunk(
    slot: &dirbuf::DirSlot,
    attrs: bool,
    off: off_t,
) -> Result<Option<dirbuf::Chunk>, u8> {
    unsafe {
        let use_cache = usedircache != 0;
        let attr_size = master_attrsize();
        slot.next(attrs, off, use_cache, attr_size, |request| {
            fetch_dir(request)
        })
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_readdir(
    req: fuse_req_t,
    ino: fuse_ino_t,
    mut size: size_t,
    mut off: off_t,
    fi: *mut fuse_file_info,
) {
    unsafe {
        let ctx = *fuse_req_ctx(req);
        mfs_stats_inc(OP_READDIR as u8);
        if debug_mode != 0 {
            if fi.is_null() {
                oplog_printf(
                    &raw const ctx,
                    b"readdir (%lu,%llu,%llu) ...\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    size as ::core::ffi::c_ulonglong,
                    off as ::core::ffi::c_ulonglong,
                );
            } else {
                oplog_printf(
                    &raw const ctx,
                    b"readdir (%lu,%llu,%llu) [handle:%016lX] ...\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    size as ::core::ffi::c_ulonglong,
                    off as ::core::ffi::c_ulonglong,
                    (*fi).fh as ::core::ffi::c_ulong,
                );
            }
            fprintf(
                stderr,
                b"readdir (%lu,%llu,%llu)\n\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
            );
        }
        if fi.is_null() {
            oplog_printf(
                &raw const ctx,
                b"readdir (%lu,%llu,%llu): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                strerr(EBADF),
            );
            fuse_reply_err(req, EBADF);
            return;
        }
        let Some((token, slot)) = dir_handle(fi) else {
            oplog_printf(
                &raw const ctx,
                b"readdir (%lu,%llu,%llu) [handle:%016lX]: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                (*fi).fh as ::core::ffi::c_ulong,
                strerr(EBADF),
            );
            fuse_reply_err(req, EBADF);
            return;
        };
        if off < 0 {
            oplog_printf(
                &raw const ctx,
                b"readdir (%lu,%llu,%llu) [handle:%016lX]: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                token as ::core::ffi::c_ulong,
                strerr(EINVAL),
            );
            fuse_reply_err(req, EINVAL);
            return;
        }
        let _operation = slot.begin_operation();
        size = size.min(READDIR_BUFFSIZE as usize);
        let attr_size = master_attrsize() as usize;
        let mut buffer = [0 as ::core::ffi::c_char; READDIR_BUFFSIZE as usize];
        let mut output = 0usize;

        loop {
            let chunk = match read_dir_chunk(&slot, false, off) {
                Ok(Some(chunk)) => chunk,
                Ok(None) => break,
                Err(status) => {
                    let error = mfs_errorconv(status as i32);
                    oplog_printf(
                        &raw const ctx,
                        b"readdir (%lu,%llu,%llu) [handle:%016lX]: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ino as ::core::ffi::c_ulong,
                        size as ::core::ffi::c_ulonglong,
                        off as ::core::ffi::c_ulonglong,
                        token as ::core::ffi::c_ulong,
                        strerr(error),
                    );
                    fuse_reply_err(req, error);
                    return;
                }
            };
            let Some(record) =
                dirbuf::parse_record(&chunk.bytes[chunk.start..], chunk.format, attr_size)
            else {
                break;
            };
            off = record.next_offset(off);
            if record.inode == 0 {
                continue;
            }

            let mut name = [0 as ::core::ffi::c_char; MFS_NAME_MAX as usize + 1];
            std::ptr::copy_nonoverlapping(
                record.name.as_ptr(),
                name.as_mut_ptr() as *mut u8,
                record.name.len(),
            );
            let mut statbuf: stat = std::mem::zeroed();
            if chunk.format == dirbuf::DataFormat::Attrs {
                mfs_attr_to_stat(record.inode, record.data.as_ptr(), &raw mut statbuf);
            } else {
                mfs_type_to_stat(record.inode, record.data[0], &raw mut statbuf);
            }
            let entry_size = fuse_add_direntry(
                req,
                buffer.as_mut_ptr().add(output),
                size.wrapping_sub(output),
                name.as_ptr(),
                &raw const statbuf,
                off,
            );
            if output.wrapping_add(entry_size) > size {
                break;
            }
            output = output.wrapping_add(entry_size);
        }

        if output != 0 {
            oplog_printf(
                &raw const ctx,
                b"readdir (%lu,%llu,%llu) [handle:%016lX]: OK (%lu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                token as ::core::ffi::c_ulong,
                output as ::core::ffi::c_ulong,
            );
            fuse_reply_buf(req, buffer.as_ptr(), output);
        } else {
            oplog_printf(
                &raw const ctx,
                b"readdir (%lu,%llu,%llu) [handle:%016lX]: OK (no data)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                token as ::core::ffi::c_ulong,
            );
            fuse_reply_buf(req, ::core::ptr::null(), 0);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_readdirplus(
    req: fuse_req_t,
    ino: fuse_ino_t,
    mut size: size_t,
    mut off: off_t,
    fi: *mut fuse_file_info,
) {
    unsafe {
        let ctx = *fuse_req_ctx(req);
        mfs_stats_inc(OP_READDIRPLUS as u8);
        if debug_mode != 0 {
            if fi.is_null() {
                oplog_printf(
                    &raw const ctx,
                    b"readdirplus (%lu,%llu,%llu) ...\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    size as ::core::ffi::c_ulonglong,
                    off as ::core::ffi::c_ulonglong,
                );
            } else {
                oplog_printf(
                    &raw const ctx,
                    b"readdirplus (%lu,%llu,%llu) [handle:%016lX] ...\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    size as ::core::ffi::c_ulonglong,
                    off as ::core::ffi::c_ulonglong,
                    (*fi).fh as ::core::ffi::c_ulong,
                );
            }
            fprintf(
                stderr,
                b"readdirplus (%lu,%llu,%llu)\n\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
            );
        }
        if fi.is_null() {
            oplog_printf(
                &raw const ctx,
                b"readdirplus (%lu,%llu,%llu): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                strerr(EBADF),
            );
            fuse_reply_err(req, EBADF);
            return;
        }
        let Some((token, slot)) = dir_handle(fi) else {
            oplog_printf(
                &raw const ctx,
                b"readdirplus (%lu,%llu,%llu) [handle:%016lX]: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                (*fi).fh as ::core::ffi::c_ulong,
                strerr(EBADF),
            );
            fuse_reply_err(req, EBADF);
            return;
        };
        if off < 0 {
            oplog_printf(
                &raw const ctx,
                b"readdirplus (%lu,%llu,%llu) [handle:%016lX]: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                token as ::core::ffi::c_ulong,
                strerr(EINVAL),
            );
            fuse_reply_err(req, EINVAL);
            return;
        }
        let _operation = slot.begin_operation();
        size = size.min(READDIR_BUFFSIZE as usize);
        let attr_size = master_attrsize() as usize;
        let mut buffer = [0 as ::core::ffi::c_char; READDIR_BUFFSIZE as usize];
        let mut output = 0usize;

        loop {
            let chunk = match read_dir_chunk(&slot, true, off) {
                Ok(Some(chunk)) => chunk,
                Ok(None) => break,
                Err(status) => {
                    let error = mfs_errorconv(status as i32);
                    oplog_printf(
                        &raw const ctx,
                        b"readdirplus (%lu,%llu,%llu) [handle:%016lX]: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ino as ::core::ffi::c_ulong,
                        size as ::core::ffi::c_ulonglong,
                        off as ::core::ffi::c_ulonglong,
                        token as ::core::ffi::c_ulong,
                        strerr(error),
                    );
                    fuse_reply_err(req, error);
                    return;
                }
            };
            // One directory handle normally uses one FUSE read mode. A basic
            // block cannot be interpreted as a full-attribute record.
            if chunk.format != dirbuf::DataFormat::Attrs {
                break;
            }
            let Some(record) =
                dirbuf::parse_record(&chunk.bytes[chunk.start..], chunk.format, attr_size)
            else {
                break;
            };
            off = record.next_offset(off);
            if record.inode == 0 {
                continue;
            }

            let mut name = [0 as ::core::ffi::c_char; MFS_NAME_MAX as usize + 1];
            std::ptr::copy_nonoverlapping(
                record.name.as_ptr(),
                name.as_mut_ptr() as *mut u8,
                record.name.len(),
            );
            let attrs = record.data;
            let entry_type = mfs_attr_get_type(attrs.as_ptr());
            let max_length = if entry_type as i32 == TYPE_FILE {
                write_data_inode_getmaxfleng(record.inode)
            } else {
                0
            };
            let mut entry: fuse_entry_param = std::mem::zeroed();
            entry.ino = record.inode as fuse_ino_t;
            entry.generation = 1;
            let mattr = mfs_attr_get_mattr(attrs.as_ptr());
            entry.attr_timeout = if mattr as i32 & MATTR_NOACACHE != 0 {
                0.0
            } else {
                attr_cache_timeout
            };
            entry.entry_timeout = if mattr as i32 & MATTR_NOECACHE != 0 {
                0.0
            } else if entry_type as i32 == TYPE_DIRECTORY {
                direntry_cache_timeout
            } else {
                entry_cache_timeout
            };
            if entry.attr_timeout == 0.0 {
                entry.attr_timeout = readdirplus_cache_min_timeout;
            }
            if entry.entry_timeout == 0.0 {
                entry.entry_timeout = readdirplus_cache_min_timeout;
            }
            if dinval != 0
                && mattr as i32 & MATTR_UNDELETABLE == 0
                && entry_type as i32 == TYPE_DIRECTORY
            {
                dinval_add(
                    ino as u32,
                    record.name.len() as u8,
                    name.as_ptr() as *const u8,
                    record.inode,
                );
            }
            mfs_attr_to_stat(record.inode, attrs.as_ptr(), &raw mut entry.attr);
            if max_length > entry.attr.st_size as u64 {
                entry.attr.st_size = max_length as __off_t;
            }
            if entry_type as i32 == TYPE_FILE {
                read_inode_set_length_passive(record.inode, entry.attr.st_size as u64);
                finfo_change_fleng(record.inode, entry.attr.st_size as u64);
            }
            mfs_fix_amtime(
                record.inode,
                &raw mut entry.attr.st_atim.tv_sec,
                &raw mut entry.attr.st_mtim.tv_sec,
            );
            let entry_size = fuse_add_direntry_plus(
                req,
                buffer.as_mut_ptr().add(output),
                size.wrapping_sub(output),
                name.as_ptr(),
                &raw const entry,
                off,
            );
            if output.wrapping_add(entry_size) > size {
                break;
            }
            output = output.wrapping_add(entry_size);
        }

        if output != 0 {
            oplog_printf(
                &raw const ctx,
                b"readdirplus (%lu,%llu,%llu) [handle:%016lX]: OK (%lu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                token as ::core::ffi::c_ulong,
                output as ::core::ffi::c_ulong,
            );
            fuse_reply_buf(req, buffer.as_ptr(), output);
        } else {
            oplog_printf(
                &raw const ctx,
                b"readdirplus (%lu,%llu,%llu) [handle:%016lX]: OK (no data)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                token as ::core::ffi::c_ulong,
            );
            fuse_reply_buf(req, ::core::ptr::null(), 0);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_releasedir(req: fuse_req_t, ino: fuse_ino_t, fi: *mut fuse_file_info) {
    unsafe {
        let ctx = *fuse_req_ctx(req);
        mfs_stats_inc(OP_RELEASEDIR as u8);
        if debug_mode != 0 {
            if fi.is_null() {
                oplog_printf(
                    &raw const ctx,
                    b"releasedir (%lu) ...\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                );
            } else {
                oplog_printf(
                    &raw const ctx,
                    b"releasedir (%lu) [handle:%016lX] ...\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    (*fi).fh as ::core::ffi::c_ulong,
                );
            }
            fprintf(
                stderr,
                b"releasedir (%lu)\n\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
            );
        }
        if fi.is_null() {
            oplog_printf(
                &raw const ctx,
                b"releasedir (%lu): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                strerr(EBADF),
            );
            fuse_reply_err(req, EBADF);
            return;
        }
        let token = (*fi).fh;
        if !dirbuf::release(token) {
            oplog_printf(
                &raw const ctx,
                b"releasedir (%lu) [handle:%016lX]: %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                token as ::core::ffi::c_ulong,
                strerr(EBADF),
            );
            fuse_reply_err(req, EBADF);
            return;
        }
        (*fi).fh = 0;
        oplog_printf(
            &raw const ctx,
            b"releasedir (%lu) [handle:%016lX]: OK\0".as_ptr() as *const ::core::ffi::c_char,
            ino as ::core::ffi::c_ulong,
            (*fi).fh as ::core::ffi::c_ulong,
        );
        fuse_reply_err(req, 0);
    }
}
pub unsafe extern "C" fn mfs_make_oflags_string(
    mut buf: *mut ::core::ffi::c_char,
    mut size: uint32_t,
    mut flags: uint32_t,
) {
    unsafe {
        let mut leng: uint32_t = 0;
        if flags & O_ACCMODE as uint32_t == O_RDWR as uint32_t {
            leng = snprintf(
                buf,
                size as size_t,
                b"O_RDWR\0".as_ptr() as *const ::core::ffi::c_char,
            ) as uint32_t;
        } else if flags & O_ACCMODE as uint32_t == O_RDONLY as uint32_t {
            leng = snprintf(
                buf,
                size as size_t,
                b"O_RDONLY\0".as_ptr() as *const ::core::ffi::c_char,
            ) as uint32_t;
        } else if flags & O_ACCMODE as uint32_t == O_WRONLY as uint32_t {
            leng = snprintf(
                buf,
                size as size_t,
                b"O_WRONLY\0".as_ptr() as *const ::core::ffi::c_char,
            ) as uint32_t;
        } else {
            leng = snprintf(
                buf,
                size as size_t,
                b"O_NONE\0".as_ptr() as *const ::core::ffi::c_char,
            ) as uint32_t;
        }
        if flags & 0o4000 as uint32_t != 0 {
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buf.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"|O_NONBLOCK\0".as_ptr() as *const ::core::ffi::c_char,
                ) as uint32_t);
            }
        }
        if flags & 0o2000 as uint32_t != 0 {
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buf.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"|O_APPEND\0".as_ptr() as *const ::core::ffi::c_char,
                ) as uint32_t);
            }
        }
        if flags & 0o100 as uint32_t != 0 {
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buf.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"|O_CREAT\0".as_ptr() as *const ::core::ffi::c_char,
                ) as uint32_t);
            }
        }
        if flags & 0o1000 as uint32_t != 0 {
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buf.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"|O_TRUNC\0".as_ptr() as *const ::core::ffi::c_char,
                ) as uint32_t);
            }
        }
        if flags & 0o200 as uint32_t != 0 {
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buf.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"|O_EXCL\0".as_ptr() as *const ::core::ffi::c_char,
                ) as uint32_t);
            }
        }
        if flags & 0o400000 as uint32_t != 0 {
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buf.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"|O_NOFOLLOW\0".as_ptr() as *const ::core::ffi::c_char,
                ) as uint32_t);
            }
        }
        if flags & 0o2000000 as uint32_t != 0 {
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buf.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"|O_CLOEXEC\0".as_ptr() as *const ::core::ffi::c_char,
                ) as uint32_t);
            }
        }
        if flags & 0o20000 as uint32_t != 0 {
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buf.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"|O_ASYNC\0".as_ptr() as *const ::core::ffi::c_char,
                ) as uint32_t);
            }
        }
        if flags & 0o40000 as uint32_t != 0 {
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buf.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"|O_DIRECT\0".as_ptr() as *const ::core::ffi::c_char,
                ) as uint32_t);
            }
        }
        if flags & 0o200000 as uint32_t != 0 {
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buf.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"|O_DIRECTORY\0".as_ptr() as *const ::core::ffi::c_char,
                ) as uint32_t);
            }
        }
        if flags & 0o10000 as uint32_t != 0 {
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buf.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"|O_DSYNC\0".as_ptr() as *const ::core::ffi::c_char,
                ) as uint32_t);
            }
        }
        if flags & 0 as uint32_t != 0 {
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buf.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"|O_LARGEFILE\0".as_ptr() as *const ::core::ffi::c_char,
                ) as uint32_t);
            }
        }
        if flags & 0o1000000 as uint32_t != 0 {
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buf.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"|O_NOATIME\0".as_ptr() as *const ::core::ffi::c_char,
                ) as uint32_t);
            }
        }
        if flags & 0o400 as uint32_t != 0 {
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buf.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"|O_NOCTTY\0".as_ptr() as *const ::core::ffi::c_char,
                ) as uint32_t);
            }
        }
        if flags & 0o10000000 as uint32_t != 0 {
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buf.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"|O_PATH\0".as_ptr() as *const ::core::ffi::c_char,
                ) as uint32_t);
            }
        }
        if flags & 0o4010000 as uint32_t != 0 {
            if leng < size {
                leng = leng.wrapping_add(snprintf(
                    buf.offset(leng as isize),
                    size.wrapping_sub(leng) as size_t,
                    b"|O_SYNC\0".as_ptr() as *const ::core::ffi::c_char,
                ) as uint32_t);
            }
        }
        if leng < size {
            *buf.offset(leng as isize) = '\0' as ::core::ffi::c_char;
        } else {
            *buf.offset(size.wrapping_sub(1 as uint32_t) as isize) = '\0' as ::core::ffi::c_char;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_create(
    mut req: fuse_req_t,
    mut parent: fuse_ino_t,
    mut name: *const ::core::ffi::c_char,
    mut mode: mode_t,
    mut fi: *mut fuse_file_info,
) {
    unsafe {
        let mut e: fuse_entry_param = fuse_entry_param {
            ino: 0,
            generation: 0,
            attr: stat {
                st_dev: 0,
                st_ino: 0,
                st_nlink: 0,
                st_mode: 0,
                st_uid: 0,
                st_gid: 0,
                __pad0: 0,
                st_rdev: 0,
                st_size: 0,
                st_blksize: 0,
                st_blocks: 0,
                st_atim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_mtim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_ctim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                __glibc_reserved: [0; 3],
            },
            attr_timeout: 0.,
            entry_timeout: 0.,
        };
        let mut inode: uint32_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut modestr: [::core::ffi::c_char; 11] = [0; 11];
        let mut attrstr: [::core::ffi::c_char; 256] = [0; 256];
        let mut mattr: uint8_t = 0;
        let mut nleng: uint32_t = 0;
        let mut cumask: uint16_t = 0;
        let mut status: ::core::ffi::c_int = 0;
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        let mut findex: uint32_t = 0;
        let mut oflags: uint8_t = 0;
        let mut flagsstr: [::core::ffi::c_char; 512] = [0; 512];
        ctx = *fuse_req_ctx(req);
        mfs_make_oflags_string(
            &raw mut flagsstr as *mut ::core::ffi::c_char,
            512 as uint32_t,
            (*fi).flags as uint32_t,
        );
        mfs_makemodestr(
            &raw mut modestr as *mut ::core::ffi::c_char,
            mode as uint16_t,
        );
        mfs_stats_inc(OP_CREATE as ::core::ffi::c_int as uint8_t);
        if debug_mode != 0 {
            let mut umaskstr: [::core::ffi::c_char; 11] = [0; 11];
            mfs_makemodestr(
                &raw mut umaskstr as *mut ::core::ffi::c_char,
                ctx.umask as uint16_t,
            );
            oplog_printf(
                &raw mut ctx,
                b"create (%lu,%s,%s,-%s:0%04o/%s:0%04o)\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                &raw mut flagsstr as *mut ::core::ffi::c_char,
                (&raw mut modestr as *mut ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize),
                mode,
                (&raw mut umaskstr as *mut ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize),
                ctx.umask,
            );
            fprintf(
                stderr,
                b"create (%lu,%s,%s,-%s:0%04o/%s:0%04o)\n\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                &raw mut flagsstr as *mut ::core::ffi::c_char,
                (&raw mut modestr as *mut ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize),
                mode,
                (&raw mut umaskstr as *mut ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize),
                ctx.umask,
            );
        }
        if parent == FUSE_ROOT_ID as fuse_ino_t {
            if *name.offset(0 as isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                && (strcmp(STATS_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(MASTERINFO_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(OPLOG_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(OPHISTORY_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(MOOSE_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(RANDOM_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int
                    || strcmp(PARAMS_NAME.as_ptr(), name) == 0 as ::core::ffi::c_int)
            {
                oplog_printf(
                    &raw mut ctx,
                    b"create (%lu,%s,%s,-%s:0%04o): %s\0".as_ptr() as *const ::core::ffi::c_char,
                    parent as ::core::ffi::c_ulong,
                    name,
                    &raw mut flagsstr as *mut ::core::ffi::c_char,
                    (&raw mut modestr as *mut ::core::ffi::c_char)
                        .offset(1 as ::core::ffi::c_int as isize),
                    mode,
                    strerr(EACCES),
                );
                fuse_reply_err(req, EACCES);
                return;
            }
        }
        nleng = strlen(name) as uint32_t;
        if nleng > MFS_NAME_MAX as uint32_t {
            oplog_printf(
                &raw mut ctx,
                b"create (%lu,%s,%s,-%s:0%04o): %s\0".as_ptr() as *const ::core::ffi::c_char,
                parent as ::core::ffi::c_ulong,
                name,
                &raw mut flagsstr as *mut ::core::ffi::c_char,
                (&raw mut modestr as *mut ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize),
                mode,
                strerr(ENAMETOOLONG),
            );
            fuse_reply_err(req, ENAMETOOLONG);
            return;
        }
        cumask = ctx.umask as uint16_t;
        if full_permissions != 0 {
            let gids = getgroups::get_common(ctx.pid, ctx.uid, ctx.gid, false);
            status = fs_create(
                parent as uint32_t,
                nleng as uint8_t,
                name as *const uint8_t,
                (mode & 0o7777 as mode_t) as uint16_t,
                cumask,
                ctx.uid as uint32_t,
                gids.len() as u32,
                gids.as_slice().as_ptr() as *mut u32,
                &raw mut inode,
                &raw mut attr as *mut uint8_t,
                &raw mut oflags,
            ) as ::core::ffi::c_int;
        } else {
            let mut gidtmp: uint32_t = ctx.gid as uint32_t;
            status = fs_create(
                parent as uint32_t,
                nleng as uint8_t,
                name as *const uint8_t,
                (mode & 0o7777 as mode_t) as uint16_t,
                cumask,
                ctx.uid as uint32_t,
                1 as uint32_t,
                &raw mut gidtmp,
                &raw mut inode,
                &raw mut attr as *mut uint8_t,
                &raw mut oflags,
            ) as ::core::ffi::c_int;
        }
        if status != MFS_ERROR_ENOTSUP {
            status = mfs_errorconv(status);
            if status != 0 as ::core::ffi::c_int {
                oplog_printf(
                    &raw mut ctx,
                    b"create (%lu,%s,%s,-%s:0%04o): %s\0".as_ptr() as *const ::core::ffi::c_char,
                    parent as ::core::ffi::c_ulong,
                    name,
                    &raw mut flagsstr as *mut ::core::ffi::c_char,
                    (&raw mut modestr as *mut ::core::ffi::c_char)
                        .offset(1 as ::core::ffi::c_int as isize),
                    mode,
                    strerr(status),
                );
                fuse_reply_err(req, status);
                return;
            }
            negentry_cache_remove(parent as uint32_t, nleng as uint8_t, name as *const uint8_t);
            if no_xattrs == 0 as ::core::ffi::c_int && xattr_cache_on != 0 {
                xattr_cache_set(
                    inode,
                    ctx.uid as uint32_t,
                    ctx.gid as uint32_t,
                    (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 10 as ::core::ffi::c_int)
                        as uint32_t,
                    b"security.capability\0".as_ptr() as *const ::core::ffi::c_char
                        as *const uint8_t,
                    ::core::ptr::null::<uint8_t>(),
                    0 as uint32_t,
                    MFS_ERROR_ENOATTR,
                );
                xattr_cache_set(
                    inode,
                    ctx.uid as uint32_t,
                    ctx.gid as uint32_t,
                    (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int)
                        as uint32_t,
                    b"security.ima\0".as_ptr() as *const ::core::ffi::c_char as *const uint8_t,
                    ::core::ptr::null::<uint8_t>(),
                    0 as uint32_t,
                    MFS_ERROR_ENOATTR,
                );
            }
        } else {
            let mut flags: uint8_t = 0;
            let mut gidtmp_0: uint32_t = ctx.gid as uint32_t;
            flags = OPEN_AFTER_CREATE as uint8_t;
            if (*fi).flags & O_ACCMODE as int32_t == O_RDONLY as int32_t {
                flags = (flags as ::core::ffi::c_int | OPEN_READ) as uint8_t;
            } else if (*fi).flags & O_ACCMODE as int32_t == O_WRONLY as int32_t {
                flags = (flags as ::core::ffi::c_int | OPEN_WRITE) as uint8_t;
            } else if (*fi).flags & O_ACCMODE as int32_t == O_RDWR as int32_t {
                flags = (flags as ::core::ffi::c_int | (OPEN_READ | OPEN_WRITE)) as uint8_t;
            } else {
                oplog_printf(
                    &raw mut ctx,
                    b"create (%lu,%s,%s,-%s:0%04o): %s\0".as_ptr() as *const ::core::ffi::c_char,
                    parent as ::core::ffi::c_ulong,
                    name,
                    &raw mut flagsstr as *mut ::core::ffi::c_char,
                    (&raw mut modestr as *mut ::core::ffi::c_char)
                        .offset(1 as ::core::ffi::c_int as isize),
                    mode,
                    strerr(EINVAL),
                );
                fuse_reply_err(req, EINVAL);
                return;
            }
            status = fs_mknod(
                parent as uint32_t,
                nleng as uint8_t,
                name as *const uint8_t,
                TYPE_FILE as uint8_t,
                (mode & 0o7777 as mode_t) as uint16_t,
                cumask,
                ctx.uid as uint32_t,
                1 as uint32_t,
                &raw mut gidtmp_0,
                0 as uint32_t,
                &raw mut inode,
                &raw mut attr as *mut uint8_t,
            ) as ::core::ffi::c_int;
            status = mfs_errorconv(status);
            if status != 0 as ::core::ffi::c_int {
                oplog_printf(
                    &raw mut ctx,
                    b"create (%lu,%s,%s,-%s:0%04o) (mknod): %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    parent as ::core::ffi::c_ulong,
                    name,
                    &raw mut flagsstr as *mut ::core::ffi::c_char,
                    (&raw mut modestr as *mut ::core::ffi::c_char)
                        .offset(1 as ::core::ffi::c_int as isize),
                    mode,
                    strerr(status),
                );
                fuse_reply_err(req, status);
                return;
            }
            negentry_cache_remove(parent as uint32_t, nleng as uint8_t, name as *const uint8_t);
            status = fs_opencheck(
                inode,
                ctx.uid as uint32_t,
                1 as uint32_t,
                &raw mut gidtmp_0,
                flags,
                &raw mut attr as *mut uint8_t,
                &raw mut oflags,
            ) as ::core::ffi::c_int;
            status = mfs_errorconv(status);
            if status != 0 as ::core::ffi::c_int {
                oplog_printf(
                    &raw mut ctx,
                    b"create (%lu,%s,%s,-%s:0%04o) (open): %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    parent as ::core::ffi::c_ulong,
                    name,
                    &raw mut flagsstr as *mut ::core::ffi::c_char,
                    (&raw mut modestr as *mut ::core::ffi::c_char)
                        .offset(1 as ::core::ffi::c_int as isize),
                    mode,
                    strerr(status),
                );
                fuse_reply_err(req, status);
                return;
            }
        }
        mattr = mfs_attr_get_mattr(&raw mut attr as *mut uint8_t as *const uint8_t);
        if oflags as ::core::ffi::c_int == 0xff as ::core::ffi::c_int {
            oflags = 0 as uint8_t;
            if mattr as ::core::ffi::c_int & MATTR_DIRECTMODE != 0 {
                oflags = (oflags as ::core::ffi::c_int | OPEN_DIRECTMODE) as uint8_t;
            }
            if mattr as ::core::ffi::c_int & MATTR_ALLOWDATACACHE != 0 {
                oflags = (oflags as ::core::ffi::c_int | OPEN_KEEPCACHE) as uint8_t;
            }
        }
        if (*fi).flags & O_APPEND as int32_t != 0 {
            oflags = (oflags as ::core::ffi::c_int | OPEN_APPENDONLY) as uint8_t;
        }
        findex = mfs_newfileinfo(
            ((*fi).flags & O_ACCMODE as int32_t) as uint8_t,
            inode,
            0 as uint64_t,
            1 as uint8_t,
            (if oflags as ::core::ffi::c_int & OPEN_APPENDONLY != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t,
        );
        (*fi).fh = findex as uint64_t;
        if oflags as ::core::ffi::c_int & OPEN_DIRECTMODE != 0
            || mfs_disables & (DISABLE_READ as uint32_t | DISABLE_WRITE as uint32_t) != 0
        {
            (*fi).set_keep_cache(0 as uint32_t as uint32_t);
            (*fi).set_direct_io(1 as uint32_t as uint32_t);
        } else {
            if keep_cache == 1 as ::core::ffi::c_int {
                (*fi).set_keep_cache(1 as uint32_t as uint32_t);
            } else if keep_cache == 2 as ::core::ffi::c_int || keep_cache >= 3 as ::core::ffi::c_int
            {
                (*fi).set_keep_cache(0 as uint32_t as uint32_t);
            } else {
                (*fi).set_keep_cache(
                    (if oflags as ::core::ffi::c_int & OPEN_KEEPCACHE != 0 {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint32_t as uint32_t,
                );
            }
            (*fi).set_direct_io(
                (if keep_cache >= 3 as ::core::ffi::c_int {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as uint32_t as uint32_t,
            );
        }
        if debug_mode != 0 {
            fprintf(
                stderr,
                b"create (%lu,%s) ok -> use %s io ; %s data cache ; can %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                inode as ::core::ffi::c_ulong,
                &raw mut flagsstr as *mut ::core::ffi::c_char,
                if (*fi).direct_io() as ::core::ffi::c_int != 0 {
                    b"direct\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"cached\0".as_ptr() as *const ::core::ffi::c_char
                },
                if (*fi).keep_cache() as ::core::ffi::c_int != 0 {
                    b"keep\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"clear\0".as_ptr() as *const ::core::ffi::c_char
                },
                if oflags as ::core::ffi::c_int & OPEN_APPENDONLY != 0 {
                    b"append only\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"write randomly\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
        dcache::invalidate_attr(parent as uint32_t);
        memset(
            &raw mut e as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<fuse_entry_param>(),
        );
        e.ino = inode as fuse_ino_t;
        e.generation = 1 as uint64_t;
        e.attr_timeout = if mattr as ::core::ffi::c_int & MATTR_NOACACHE != 0 {
            0.0f64
        } else {
            attr_cache_timeout
        };
        e.entry_timeout = if mattr as ::core::ffi::c_int & MATTR_NOECACHE != 0 {
            0.0f64
        } else {
            entry_cache_timeout
        };
        mfs_attr_to_stat(
            inode,
            &raw mut attr as *mut uint8_t as *const uint8_t,
            &raw mut e.attr,
        );
        mfs_makeattrstr(
            &raw mut attrstr as *mut ::core::ffi::c_char,
            256 as uint32_t,
            &raw mut e.attr,
        );
        oplog_printf(
            &raw mut ctx,
            b"create (%lu,%s,%s,-%s:0%04o): OK (%.1lf,%lu,%.1lf,%s) (direct_io:%u,keep_cache:%u,append_mode:%u) [handle:%08X]\0"
                .as_ptr() as *const ::core::ffi::c_char,
            parent as ::core::ffi::c_ulong,
            name,
            &raw mut flagsstr as *mut ::core::ffi::c_char,
            (&raw mut modestr as *mut ::core::ffi::c_char)
                .offset(1 as ::core::ffi::c_int as isize),
            mode,
            e.entry_timeout,
            e.ino as ::core::ffi::c_ulong,
            e.attr_timeout,
            &raw mut attrstr as *mut ::core::ffi::c_char,
            (*fi).direct_io() as ::core::ffi::c_uint,
            (*fi).keep_cache() as ::core::ffi::c_uint,
            if oflags as ::core::ffi::c_int & OPEN_APPENDONLY != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
            findex,
        );
        fs_inc_acnt(inode);
        if fuse_reply_create(req, &raw mut e, fi) == -ENOENT {
            fs_dec_acnt(inode);
            mfs_removefileinfo(findex as u64);
            (*fi).fh = 0 as uint64_t;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_open(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut fi: *mut fuse_file_info,
) {
    unsafe {
        let mut flags: uint8_t = 0;
        let mut oflags: uint8_t = 0;
        let mut mmode: uint8_t = 0;
        let mut noatomictrunc: uint8_t = 0;
        let mut lflags: uint16_t = 0;
        let mut fdrec: Option<crate::src::fuse_client::fdcache::imp::FdEntry> = None;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut mattr: uint8_t = 0;
        let mut status: ::core::ffi::c_int = 0;
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        let mut findex: uint32_t = 0;
        let mut flagsstr: [::core::ffi::c_char; 512] = [0; 512];
        ctx = *fuse_req_ctx(req);
        mfs_make_oflags_string(
            &raw mut flagsstr as *mut ::core::ffi::c_char,
            512 as uint32_t,
            (*fi).flags as uint32_t,
        );
        mfs_stats_inc(OP_OPEN as ::core::ffi::c_int as uint8_t);
        if debug_mode != 0 {
            oplog_printf(
                &raw mut ctx,
                b"open (%lu,%s) ...\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                &raw mut flagsstr as *mut ::core::ffi::c_char,
            );
            fprintf(
                stderr,
                b"open (%lu,%s)\n\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                &raw mut flagsstr as *mut ::core::ffi::c_char,
            );
        }
        if ino == MASTERINFO_INODE as fuse_ino_t {
            if (*fi).flags & O_ACCMODE as int32_t != O_RDONLY as int32_t {
                oplog_printf(
                    &raw mut ctx,
                    b"open (%lu,%s) (internal node: MASTERINFO): %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    &raw mut flagsstr as *mut ::core::ffi::c_char,
                    strerr(EACCES),
                );
                fuse_reply_err(req, EACCES);
                return;
            }
            (*fi).fh = 0 as uint64_t;
            (*fi).set_direct_io(0 as uint32_t as uint32_t);
            (*fi).set_keep_cache(0 as uint32_t as uint32_t);
            oplog_printf(
                &raw mut ctx,
                b"open (%lu,%s) (internal node: MASTERINFO): OK (0,1)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                &raw mut flagsstr as *mut ::core::ffi::c_char,
            );
            fuse_reply_open(req, fi);
            return;
        }
        if ino == PARAMS_INODE as fuse_ino_t {
            if (*fi).flags & O_ACCMODE as int32_t != O_RDONLY as int32_t {
                oplog_printf(
                    &raw mut ctx,
                    b"open (%lu,%s) (internal node: PARAMS): %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    &raw mut flagsstr as *mut ::core::ffi::c_char,
                    strerr(EACCES),
                );
                fuse_reply_err(req, EACCES);
                return;
            }
            if ctx.uid != 0 as uid_t {
                oplog_printf(
                    &raw mut ctx,
                    b"open (%lu,%s) (internal node: PARAMS): %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    &raw mut flagsstr as *mut ::core::ffi::c_char,
                    strerr(EPERM),
                );
                fuse_reply_err(req, EPERM);
                return;
            }
        }
        if ino == STATS_INODE as fuse_ino_t || ino == PARAMS_INODE as fuse_ino_t {
            let sindex = sinfo_new();
            let Some(statsinfo) = sinfo_get(sindex) else {
                abort();
            };
            {
                let mut statsinfo = statsinfo.lock().unwrap();
                if ino == STATS_INODE as fuse_ino_t {
                    statsinfo.buff = plfsclient::stats::show_all();
                } else {
                    let mut buff = vec![0u8; PARAMS_BUFFSIZE as usize];
                    let leng = unsafe {
                        main_snprint_parameters(
                            buff.as_mut_ptr() as *mut ::core::ffi::c_char,
                            PARAMS_BUFFSIZE as uint32_t,
                        )
                    };
                    buff.truncate(leng as usize);
                    statsinfo.buff = buff;
                }
                statsinfo.reset = false;
                statsinfo.valid = true;
            }
            (*fi).fh = sindex as uint64_t;
            (*fi).set_direct_io(1 as uint32_t as uint32_t);
            (*fi).set_keep_cache(0 as uint32_t as uint32_t);
            if ino == STATS_INODE as fuse_ino_t {
                oplog_printf(
                    &raw mut ctx,
                    b"open (%lu,%s) (internal node: STATS): OK (1,0)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    &raw mut flagsstr as *mut ::core::ffi::c_char,
                );
            } else {
                oplog_printf(
                    &raw mut ctx,
                    b"open (%lu,%s) (internal node: PARAMS): OK (1,0)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    &raw mut flagsstr as *mut ::core::ffi::c_char,
                );
            }
            fuse_reply_open(req, fi);
            return;
        }
        if ino == MOOSE_INODE as fuse_ino_t || ino == RANDOM_INODE as fuse_ino_t {
            (*fi).fh = 0 as uint64_t;
            (*fi).set_direct_io(1 as uint32_t as uint32_t);
            (*fi).set_keep_cache(0 as uint32_t as uint32_t);
            oplog_printf(
                &raw mut ctx,
                b"open (%lu,%s) (internal node: %s): OK (1,0)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                &raw mut flagsstr as *mut ::core::ffi::c_char,
                if ino == MOOSE_INODE as fuse_ino_t {
                    b"MOOSE\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"RANDOM\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            fuse_reply_open(req, fi);
            return;
        }
        if ino == OPLOG_INODE as fuse_ino_t || ino == OPHISTORY_INODE as fuse_ino_t {
            if (*fi).flags & O_ACCMODE as int32_t != O_RDONLY as int32_t {
                oplog_printf(
                    &raw mut ctx,
                    b"open (%lu,%s) (internal node: %s): %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    &raw mut flagsstr as *mut ::core::ffi::c_char,
                    if ino == OPLOG_INODE as fuse_ino_t {
                        b"OPLOG\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"OPHISTORY\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    strerr(EACCES),
                );
                fuse_reply_err(req, EACCES);
                return;
            }
            if ctx.uid != 0 as uid_t {
                oplog_printf(
                    &raw mut ctx,
                    b"open (%lu,%s) (internal node: %s): %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    &raw mut flagsstr as *mut ::core::ffi::c_char,
                    if ino == OPLOG_INODE as fuse_ino_t {
                        b"OPLOG\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"OPHISTORY\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    strerr(EPERM),
                );
                fuse_reply_err(req, EPERM);
                return;
            }
            (*fi).fh = oplog_newhandle(if ino == OPHISTORY_INODE as fuse_ino_t {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint64_t;
            (*fi).set_direct_io(1 as uint32_t as uint32_t);
            (*fi).set_keep_cache(0 as uint32_t as uint32_t);
            oplog_printf(
                &raw mut ctx,
                b"open (%lu,%s) (internal node: %s): OK (1,0)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                &raw mut flagsstr as *mut ::core::ffi::c_char,
                if ino == OPLOG_INODE as fuse_ino_t {
                    b"OPLOG\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"OPHISTORY\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            fuse_reply_open(req, fi);
            return;
        }
        if (*fi).flags & O_ACCMODE as int32_t == O_RDONLY as int32_t {
            flags = OPEN_READ as uint8_t;
            mmode = MODE_MASK_R as uint8_t;
        } else if (*fi).flags & O_ACCMODE as int32_t == O_WRONLY as int32_t {
            flags = OPEN_WRITE as uint8_t;
            mmode = MODE_MASK_W as uint8_t;
        } else if (*fi).flags & O_ACCMODE as int32_t == O_RDWR as int32_t {
            flags = (OPEN_READ | OPEN_WRITE) as uint8_t;
            mmode = (MODE_MASK_R | MODE_MASK_W) as uint8_t;
        } else {
            flags = 0 as uint8_t;
            mmode = 0 as uint8_t;
        }
        if (*fi).flags & O_TRUNC as int32_t != 0 {
            let mut mver: uint32_t = master_version();
            noatomictrunc = (if mver
                < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 18 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
                && mver
                    >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })) as uint32_t
                || mver
                    < (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            113 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            113 as ::core::ffi::c_int
                        })) as uint32_t
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            flags = (flags as ::core::ffi::c_int | OPEN_TRUNCATE) as uint8_t;
            fdrec = None;
        } else {
            fdrec = fdcache::acquire(
                ctx.uid as u32,
                ctx.gid as u32,
                ctx.pid as i32,
                ino as uint32_t,
            );
            if let Some(entry) = fdrec.as_ref() {
                attr[..35].copy_from_slice(&entry.attr);
                lflags = entry.lflags;
            }
            noatomictrunc = 0 as uint8_t;
        }
        if fdrec.is_some() {
            if lflags as ::core::ffi::c_int & LOOKUP_RO_FILESYSTEM != 0
                && mmode as ::core::ffi::c_int & MODE_MASK_W != 0
            {
                status = MFS_ERROR_EROFS;
            } else if lflags as ::core::ffi::c_int & LOOKUP_IMMUTABLE != 0
                && mmode as ::core::ffi::c_int & MODE_MASK_W != 0
            {
                status = MFS_ERROR_EPERM;
            } else {
                status = if lflags as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int)
                        << (mmode as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int)
                    != 0
                {
                    MFS_STATUS_OK
                } else {
                    MFS_ERROR_EACCES
                };
            }
            if status == MFS_STATUS_OK {
                fdcache::inject_chunkdata(fdrec.as_ref().unwrap());
                oflags = 0 as uint8_t;
                if lflags as ::core::ffi::c_int & LOOKUP_APPENDONLY != 0 {
                    oflags = (oflags as ::core::ffi::c_int | OPEN_APPENDONLY) as uint8_t;
                }
                if lflags as ::core::ffi::c_int & LOOKUP_DIRECTMODE != 0 {
                    oflags = (oflags as ::core::ffi::c_int | OPEN_DIRECTMODE) as uint8_t;
                }
                if lflags as ::core::ffi::c_int & LOOKUP_KEEPCACHE != 0 {
                    oflags = (oflags as ::core::ffi::c_int | OPEN_KEEPCACHE) as uint8_t;
                }
            }
        } else {
            write_data_flush_inode(ino as uint32_t);
            if full_permissions != 0 {
                let gids = getgroups::get_common(ctx.pid, ctx.uid, ctx.gid, false);
                status = fs_opencheck(
                    ino as uint32_t,
                    ctx.uid as uint32_t,
                    gids.len() as u32,
                    gids.as_slice().as_ptr() as *mut u32,
                    flags,
                    &raw mut attr as *mut uint8_t,
                    &raw mut oflags,
                ) as ::core::ffi::c_int;
                if status == MFS_STATUS_OK
                    && (*fi).flags & O_TRUNC as int32_t != 0
                    && noatomictrunc as ::core::ffi::c_int != 0
                {
                    status = do_truncate(
                        ino as uint32_t,
                        TRUNCATE_FLAG_OPENED as uint8_t,
                        ctx.uid as uint32_t,
                        gids.len() as u32,
                        gids.as_slice().as_ptr() as *mut u32,
                        0 as uint64_t,
                        &raw mut attr as *mut uint8_t,
                        ::core::ptr::null_mut::<uint64_t>(),
                    ) as ::core::ffi::c_int;
                }
            } else {
                let mut gidtmp: uint32_t = ctx.gid as uint32_t;
                status = fs_opencheck(
                    ino as uint32_t,
                    ctx.uid as uint32_t,
                    1 as uint32_t,
                    &raw mut gidtmp,
                    flags,
                    &raw mut attr as *mut uint8_t,
                    &raw mut oflags,
                ) as ::core::ffi::c_int;
                if status == MFS_STATUS_OK
                    && (*fi).flags & O_TRUNC as int32_t != 0
                    && noatomictrunc as ::core::ffi::c_int != 0
                {
                    status = do_truncate(
                        ino as uint32_t,
                        TRUNCATE_FLAG_OPENED as uint8_t,
                        ctx.uid as uint32_t,
                        1 as uint32_t,
                        &raw mut gidtmp,
                        0 as uint64_t,
                        &raw mut attr as *mut uint8_t,
                        ::core::ptr::null_mut::<uint64_t>(),
                    ) as ::core::ffi::c_int;
                }
            }
        }
        status = mfs_errorconv(status);
        if status != 0 as ::core::ffi::c_int {
            oplog_printf(
                &raw mut ctx,
                b"open (%lu,%s)%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                &raw mut flagsstr as *mut ::core::ffi::c_char,
                if fdrec.is_some() {
                    b" (using cached data from lookup)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
                strerr(status),
            );
            fuse_reply_err(req, status);
            return;
        }
        if (*fi).flags & O_TRUNC as int32_t != 0 {
            plfsclient::chunksdatacache::clear_inode(ino as uint32_t, 0 as uint32_t);
            finfo_change_fleng(ino as uint32_t, 0 as uint64_t);
            write_data_inode_setmaxfleng(ino as uint32_t, 0 as uint64_t);
            read_inode_set_length_active(ino as uint32_t, 0 as uint64_t);
            dcache::setattr(ino as uint32_t, &attr);
            fdcache::invalidate(ino as uint32_t);
        }
        mattr = mfs_attr_get_mattr(&raw mut attr as *mut uint8_t as *const uint8_t);
        if oflags as ::core::ffi::c_int == 0xff as ::core::ffi::c_int {
            oflags = 0 as uint8_t;
            if mattr as ::core::ffi::c_int & MATTR_DIRECTMODE != 0 {
                oflags = (oflags as ::core::ffi::c_int | OPEN_DIRECTMODE) as uint8_t;
            }
            if mattr as ::core::ffi::c_int & MATTR_ALLOWDATACACHE != 0 {
                oflags = (oflags as ::core::ffi::c_int | OPEN_KEEPCACHE) as uint8_t;
            }
        }
        if (*fi).flags & O_APPEND as int32_t != 0 {
            oflags = (oflags as ::core::ffi::c_int | OPEN_APPENDONLY) as uint8_t;
        }
        findex = mfs_newfileinfo(
            ((*fi).flags & O_ACCMODE as int32_t) as uint8_t,
            ino as uint32_t,
            mfs_attr_get_fleng(&raw mut attr as *mut uint8_t as *const uint8_t),
            (if fdrec.is_some() {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as uint8_t,
            (if oflags as ::core::ffi::c_int & OPEN_APPENDONLY != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t,
        );
        (*fi).fh = findex as uint64_t;
        if oflags as ::core::ffi::c_int & OPEN_DIRECTMODE != 0
            || mfs_disables & (DISABLE_READ as uint32_t | DISABLE_WRITE as uint32_t) != 0
        {
            (*fi).set_keep_cache(0 as uint32_t as uint32_t);
            (*fi).set_direct_io(1 as uint32_t as uint32_t);
        } else {
            if keep_cache == 1 as ::core::ffi::c_int {
                (*fi).set_keep_cache(1 as uint32_t as uint32_t);
            } else if keep_cache == 2 as ::core::ffi::c_int || keep_cache >= 3 as ::core::ffi::c_int
            {
                (*fi).set_keep_cache(0 as uint32_t as uint32_t);
            } else {
                (*fi).set_keep_cache(
                    (if oflags as ::core::ffi::c_int & OPEN_KEEPCACHE != 0 {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint32_t as uint32_t,
                );
            }
            (*fi).set_direct_io(
                (if keep_cache >= 3 as ::core::ffi::c_int {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as uint32_t as uint32_t,
            );
        }
        if debug_mode != 0 {
            fprintf(
                stderr,
                b"open (%lu,%s) ok -> use %s io ; %s data cache ; can %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                &raw mut flagsstr as *mut ::core::ffi::c_char,
                if (*fi).direct_io() as ::core::ffi::c_int != 0 {
                    b"direct\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"cached\0".as_ptr() as *const ::core::ffi::c_char
                },
                if (*fi).keep_cache() as ::core::ffi::c_int != 0 {
                    b"keep\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"clear\0".as_ptr() as *const ::core::ffi::c_char
                },
                if oflags as ::core::ffi::c_int & OPEN_APPENDONLY != 0 {
                    b"append only\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"write randomly\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
        oplog_printf(
            &raw mut ctx,
            b"open (%lu,%s)%s: OK (direct_io:%u,keep_cache:%u,append_mode:%u) [handle:%08X]\0"
                .as_ptr() as *const ::core::ffi::c_char,
            ino as ::core::ffi::c_ulong,
            &raw mut flagsstr as *mut ::core::ffi::c_char,
            if fdrec.is_some() {
                b" (using cached data from lookup)\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            (*fi).direct_io() as ::core::ffi::c_uint,
            (*fi).keep_cache() as ::core::ffi::c_uint,
            if oflags as ::core::ffi::c_int & OPEN_APPENDONLY != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
            findex,
        );
        fs_inc_acnt(ino as uint32_t);
        if fuse_reply_open(req, fi) == -ENOENT {
            mfs_removefileinfo(findex as u64);
            fs_dec_acnt(ino as uint32_t);
            (*fi).fh = 0 as uint64_t;
        } else if fdrec.is_some() {
            let mut fileinfo: Option<std::sync::Arc<FileInfo>> = None;
            let mut gidtmp_0: uint32_t = ctx.gid as uint32_t;
            if (*fi).keep_cache() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                flags = (flags as ::core::ffi::c_int | OPEN_CACHE_CLEARED) as uint8_t;
            }
            status = fs_opencheck(
                ino as uint32_t,
                ctx.uid as uint32_t,
                1 as uint32_t,
                &raw mut gidtmp_0,
                (flags as ::core::ffi::c_int | OPEN_AFTER_CREATE) as uint8_t,
                ::core::ptr::null_mut::<uint8_t>(),
                ::core::ptr::null_mut::<uint8_t>(),
            ) as ::core::ffi::c_int;
            if status != MFS_STATUS_OK {
                status = mfs_errorconv(status);
                oplog_printf(
                    &raw mut ctx,
                    b"open (%lu,%s) (do actual open): %s\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    &raw mut flagsstr as *mut ::core::ffi::c_char,
                    strerr(status),
                );
            }
            fileinfo = fileinfo_get((*fi).fh);
            if let Some(fileinfo) = fileinfo {
                fileinfo.complete_open(status);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_release(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut fi: *mut fuse_file_info,
) {
    unsafe {
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        let mut fileinfo: Option<std::sync::Arc<FileInfo>> = None;
        ctx = *fuse_req_ctx(req);
        mfs_stats_inc(OP_RELEASE as ::core::ffi::c_int as uint8_t);
        if debug_mode != 0 {
            if !fi.is_null() {
                oplog_printf(
                    &raw mut ctx,
                    b"release (%lu) [handle:%08X] ...\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    (*fi).fh as uint32_t,
                );
            } else {
                oplog_printf(
                    &raw mut ctx,
                    b"release (%lu) ...\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                );
            }
            fprintf(
                stderr,
                b"release (%lu)\n\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
            );
        }
        if fi.is_null() {
            oplog_printf(
                &raw mut ctx,
                b"release (%lu): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                strerr(EBADF),
            );
            fuse_reply_err(req, EBADF);
            return;
        }
        if ino == MASTERINFO_INODE as fuse_ino_t {
            oplog_printf(
                &raw mut ctx,
                b"release (%lu) (internal node: MASTERINFO): OK\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
            );
            fuse_reply_err(req, 0 as ::core::ffi::c_int);
            return;
        }
        if ino == STATS_INODE as fuse_ino_t || ino == PARAMS_INODE as fuse_ino_t {
            if let Some(statsinfo) = sinfo_get((*fi).fh as uint32_t) {
                let reset = {
                    let mut statsinfo = statsinfo.lock().unwrap();
                    statsinfo.buff.clear();
                    statsinfo.reset
                };
                if reset {
                    plfsclient::stats::reset_all();
                }
                sinfo_release((*fi).fh as uint32_t);
            }
            oplog_printf(
                &raw mut ctx,
                b"release (%lu) (internal node: STATS): OK\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
            );
            fuse_reply_err(req, 0 as ::core::ffi::c_int);
            return;
        }
        if ino == MOOSE_INODE as fuse_ino_t || ino == RANDOM_INODE as fuse_ino_t {
            oplog_printf(
                &raw mut ctx,
                b"release (%lu) (internal node: %s): OK\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                if ino == MOOSE_INODE as fuse_ino_t {
                    b"MOOSE\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"RANDOM\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            fuse_reply_err(req, 0 as ::core::ffi::c_int);
            return;
        }
        if ino == OPLOG_INODE as fuse_ino_t || ino == OPHISTORY_INODE as fuse_ino_t {
            oplog_releasehandle((*fi).fh as ::core::ffi::c_ulong);
            oplog_printf(
                &raw mut ctx,
                b"release (%lu) (internal node: %s): OK\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                if ino == OPLOG_INODE as fuse_ino_t {
                    b"OPLOG\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"OPHISTORY\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            fuse_reply_err(req, 0 as ::core::ffi::c_int);
            return;
        }
        if (*fi).fh > 0 as uint64_t {
            fileinfo = fileinfo_get((*fi).fh);
        } else {
            fileinfo = None;
        }
        if fileinfo.is_some() {
            plfsclient::inoleng::io_wait(fileinfo.as_ref().unwrap().length.as_ref().unwrap());
            let (uselocks, posix_owners, flock_owners) = {
                let mut state = fileinfo.as_ref().unwrap().state();
                let uselocks = state.use_locks;
                let owners = state.take_lock_owners((*fi).lock_owner);
                (uselocks, owners.0, owners.1)
            };
            for owner in posix_owners {
                let status = mfs_errorconv(fs_posixlock(
                    ino as uint32_t,
                    0,
                    owner.owner,
                    POSIX_LOCK_CMD_SET as uint8_t,
                    POSIX_LOCK_UNLCK as uint8_t,
                    0,
                    UINT64_MAX,
                    0,
                    ::core::ptr::null_mut(),
                    ::core::ptr::null_mut(),
                    ::core::ptr::null_mut(),
                    ::core::ptr::null_mut(),
                ) as ::core::ffi::c_int);
                if status != 0 {
                    oplog_printf(
                        &raw mut ctx,
                        b"release (%lu) - releasing all POSIX-type locks for %016lX (left by kernel): %s\0"
                            .as_ptr() as *const _,
                        ino as ::core::ffi::c_ulong,
                        owner.owner,
                        strerr(status),
                    );
                } else {
                    oplog_printf(
                        &raw mut ctx,
                        b"release (%lu) - releasing all POSIX-type locks for %016lX (left by kernel): OK\0"
                            .as_ptr() as *const _,
                        ino as ::core::ffi::c_ulong,
                        owner.owner,
                    );
                }
            }
            if uselocks & 1 != 0 {
                let status = mfs_errorconv(fs_flock(
                    ino as uint32_t,
                    0,
                    (*fi).lock_owner,
                    FLOCK_RELEASE as uint8_t,
                ) as ::core::ffi::c_int);
                if status != 0 {
                    oplog_printf(
                        &raw mut ctx,
                        b"release (%lu) - releasing all FLOCK-type locks for %016lX (received from kernel): %s\0"
                            .as_ptr() as *const _,
                        ino as ::core::ffi::c_ulong,
                        (*fi).lock_owner,
                        strerr(status),
                    );
                } else {
                    oplog_printf(
                        &raw mut ctx,
                        b"release (%lu) - releasing all FLOCK-type locks for %016lX (received from kernel): OK\0"
                            .as_ptr() as *const _,
                        ino as ::core::ffi::c_ulong,
                        (*fi).lock_owner,
                    );
                }
            }
            for owner in flock_owners {
                let status = mfs_errorconv(fs_flock(
                    ino as uint32_t,
                    0,
                    owner.owner,
                    FLOCK_RELEASE as uint8_t,
                ) as ::core::ffi::c_int);
                if status != 0 {
                    oplog_printf(
                        &raw mut ctx,
                        b"release (%lu) - releasing all FLOCK-type locks for %016lX (left by kernel): %s\0"
                            .as_ptr() as *const _,
                        ino as ::core::ffi::c_ulong,
                        owner.owner,
                        strerr(status),
                    );
                } else {
                    oplog_printf(
                        &raw mut ctx,
                        b"release (%lu) - releasing all FLOCK-type locks for %016lX (left by kernel): OK\0"
                            .as_ptr() as *const _,
                        ino as ::core::ffi::c_ulong,
                        owner.owner,
                    );
                }
            }
        }
        dcache::invalidate_attr(ino as uint32_t);
        if fileinfo.is_some() {
            let uselocks = fileinfo.as_ref().unwrap().state().use_locks;
            oplog_printf(
                &raw mut ctx,
                b"release (%lu) [handle:%08X,uselocks:%u,lock_owner:%016lX]: OK\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                (*fi).fh as uint32_t,
                uselocks as ::core::ffi::c_int,
                (*fi).lock_owner,
            );
        } else {
            oplog_printf(
                &raw mut ctx,
                b"release (%lu) [handle:%08X,lock_owner:%016lX]: OK\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                (*fi).fh as uint32_t,
                (*fi).lock_owner,
            );
        }
        fuse_reply_err(req, 0 as ::core::ffi::c_int);
        if (*fi).fh > 0 as uint64_t {
            if fileinfo.is_some() {
                let mut state = fileinfo.as_ref().unwrap().state();
                state = fileinfo.as_ref().unwrap().wait_for_open_locked(state);
                drop(state);
            }
            mfs_removefileinfo((*fi).fh);
        }
        fs_dec_acnt(ino as uint32_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_read(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut size: size_t,
    mut off: off_t,
    mut fi: *mut fuse_file_info,
) {
    unsafe {
        let mut fileinfo: Option<std::sync::Arc<FileInfo>> = None;
        let mut buff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut ssize: uint32_t = 0;
        let mut iov: *mut iovec = ::core::ptr::null_mut::<iovec>();
        let mut iovcnt: uint32_t = 0;
        let mut buffptr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut err: ::core::ffi::c_int = 0;
        let mut oim: uint8_t = 0;
        let mut oerr: uint8_t = 0;
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        ctx = *fuse_req_ctx(req);
        mfs_stats_inc(OP_READ as ::core::ffi::c_int as uint8_t);
        if debug_mode != 0 {
            if ino != OPLOG_INODE as fuse_ino_t && ino != OPHISTORY_INODE as fuse_ino_t {
                if !fi.is_null() {
                    oplog_printf(
                        &raw mut ctx,
                        b"read (%lu,%llu,%llu) [handle:%08X] ...\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ino as ::core::ffi::c_ulong,
                        size as ::core::ffi::c_ulonglong,
                        off as ::core::ffi::c_ulonglong,
                        (*fi).fh as uint32_t,
                    );
                } else {
                    oplog_printf(
                        &raw mut ctx,
                        b"read (%lu,%llu,%llu) ...\0".as_ptr() as *const ::core::ffi::c_char,
                        ino as ::core::ffi::c_ulong,
                        size as ::core::ffi::c_ulonglong,
                        off as ::core::ffi::c_ulonglong,
                    );
                }
            }
            fprintf(
                stderr,
                b"read from inode %lu up to %llu bytes from position %llu\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
            );
        }
        if ino == MASTERINFO_INODE as fuse_ino_t {
            let mut masterinfo: [uint8_t; 22] = [0; 22];
            fs_getmasterlocation(&raw mut masterinfo as *mut uint8_t);
            masterproxy_getlocation(&raw mut masterinfo as *mut uint8_t);
            if off >= 22 as off_t {
                oplog_printf(
                    &raw mut ctx,
                    b"read (%lu,%llu,%llu): OK (no data)\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    size as ::core::ffi::c_ulonglong,
                    off as ::core::ffi::c_ulonglong,
                );
                fuse_reply_buf(req, ::core::ptr::null::<::core::ffi::c_char>(), 0 as size_t);
            } else if (off as size_t).wrapping_add(size) > 22 as size_t {
                oplog_printf(
                    &raw mut ctx,
                    b"read (%lu,%llu,%llu): OK (%lu)\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    size as ::core::ffi::c_ulonglong,
                    off as ::core::ffi::c_ulonglong,
                    (22 as off_t - off) as ::core::ffi::c_ulong,
                );
                fuse_reply_buf(
                    req,
                    (&raw mut masterinfo as *mut uint8_t).offset(off as isize)
                        as *mut ::core::ffi::c_char,
                    (22 as off_t - off) as size_t,
                );
            } else {
                oplog_printf(
                    &raw mut ctx,
                    b"read (%lu,%llu,%llu): OK (%lu)\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    size as ::core::ffi::c_ulonglong,
                    off as ::core::ffi::c_ulonglong,
                    size as ::core::ffi::c_ulong,
                );
                fuse_reply_buf(
                    req,
                    (&raw mut masterinfo as *mut uint8_t).offset(off as isize)
                        as *mut ::core::ffi::c_char,
                    size,
                );
            }
            return;
        }
        if ino == STATS_INODE as fuse_ino_t || ino == PARAMS_INODE as fuse_ino_t {
            let Some(fi_ref) = fi.as_ref() else {
                fuse_reply_buf(req, ::core::ptr::null_mut(), 0);
                return;
            };
            let Some(statsinfo) = sinfo_get((*fi_ref).fh as uint32_t) else {
                fuse_reply_buf(req, ::core::ptr::null_mut(), 0);
                return;
            };
            let statsinfo = statsinfo.lock().unwrap();
            let start = off.max(0) as usize;
            if start >= statsinfo.buff.len() {
                oplog_printf(
                    &raw mut ctx,
                    b"read (%lu,%llu,%llu): OK (no data)\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    size as ::core::ffi::c_ulonglong,
                    off as ::core::ffi::c_ulonglong,
                );
                fuse_reply_buf(req, ::core::ptr::null_mut(), 0);
            } else {
                let count = size.min(statsinfo.buff.len() - start);
                oplog_printf(
                    &raw mut ctx,
                    b"read (%lu,%llu,%llu): OK (%lu)\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    size as ::core::ffi::c_ulonglong,
                    off as ::core::ffi::c_ulonglong,
                    count as ::core::ffi::c_ulong,
                );
                fuse_reply_buf(
                    req,
                    statsinfo.buff.as_ptr().add(start) as *mut ::core::ffi::c_char,
                    count,
                );
            }
            return;
        }
        if ino == RANDOM_INODE as fuse_ino_t {
            let mut rbptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
            let mut nextr: uint32_t = 0;
            buff = malloc(size) as *mut uint8_t;
            ssize = size as uint32_t;
            rbptr = buff;
            let _random_guard = RANDOM_LOCK.lock().unwrap();
            while ssize >= 4 as uint32_t {
                rndz = (36969 as uint32_t)
                    .wrapping_mul(rndz & 65535 as uint32_t)
                    .wrapping_add(rndz >> 16 as ::core::ffi::c_int);
                rndw = (18000 as uint32_t)
                    .wrapping_mul(rndw & 65535 as uint32_t)
                    .wrapping_add(rndw >> 16 as ::core::ffi::c_int);
                rndjcong = (69069 as uint32_t)
                    .wrapping_mul(rndjcong)
                    .wrapping_add(1234567 as uint32_t);
                rndjsr ^= rndjsr << 17 as ::core::ffi::c_int;
                rndjsr ^= rndjsr >> 13 as ::core::ffi::c_int;
                rndjsr ^= rndjsr << 5 as ::core::ffi::c_int;
                nextr = ((rndz << 16 as ::core::ffi::c_int).wrapping_add(rndw) ^ rndjcong)
                    .wrapping_add(rndjsr);
                let c2rust_fresh3 = rbptr;
                rbptr = rbptr.offset(1);
                *c2rust_fresh3 = (nextr >> 24 as ::core::ffi::c_int) as uint8_t;
                let c2rust_fresh4 = rbptr;
                rbptr = rbptr.offset(1);
                *c2rust_fresh4 = (nextr >> 16 as ::core::ffi::c_int) as uint8_t;
                let c2rust_fresh5 = rbptr;
                rbptr = rbptr.offset(1);
                *c2rust_fresh5 = (nextr >> 8 as ::core::ffi::c_int) as uint8_t;
                let c2rust_fresh6 = rbptr;
                rbptr = rbptr.offset(1);
                *c2rust_fresh6 = nextr as uint8_t;
                ssize = ssize.wrapping_sub(4 as uint32_t);
            }
            if ssize > 0 as uint32_t {
                rndz = (36969 as uint32_t)
                    .wrapping_mul(rndz & 65535 as uint32_t)
                    .wrapping_add(rndz >> 16 as ::core::ffi::c_int);
                rndw = (18000 as uint32_t)
                    .wrapping_mul(rndw & 65535 as uint32_t)
                    .wrapping_add(rndw >> 16 as ::core::ffi::c_int);
                rndjcong = (69069 as uint32_t)
                    .wrapping_mul(rndjcong)
                    .wrapping_add(1234567 as uint32_t);
                rndjsr ^= rndjsr << 17 as ::core::ffi::c_int;
                rndjsr ^= rndjsr >> 13 as ::core::ffi::c_int;
                rndjsr ^= rndjsr << 5 as ::core::ffi::c_int;
                nextr = ((rndz << 16 as ::core::ffi::c_int).wrapping_add(rndw) ^ rndjcong)
                    .wrapping_add(rndjsr);
                while ssize > 0 as uint32_t {
                    let c2rust_fresh7 = rbptr;
                    rbptr = rbptr.offset(1);
                    *c2rust_fresh7 = (nextr >> 24 as ::core::ffi::c_int) as uint8_t;
                    nextr <<= 8 as ::core::ffi::c_int;
                    ssize = ssize.wrapping_sub(1);
                }
            }
            drop(_random_guard);
            fuse_reply_buf(req, buff as *mut ::core::ffi::c_char, size);
            free(buff as *mut ::core::ffi::c_void);
            return;
        }
        if ino == MOOSE_INODE as fuse_ino_t {
            static mut mooseascii: [::core::ffi::c_char; 175] = [
                0x20 as ::core::ffi::c_char,
                0x5c as ::core::ffi::c_char,
                0x5f as ::core::ffi::c_char,
                0x5c as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x2f as ::core::ffi::c_char,
                0x5f as ::core::ffi::c_char,
                0x2f as ::core::ffi::c_char,
                0xa as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x5c as ::core::ffi::c_char,
                0x5f as ::core::ffi::c_char,
                0x5c as ::core::ffi::c_char,
                0x5f as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x5f as ::core::ffi::c_char,
                0x2f as ::core::ffi::c_char,
                0x5f as ::core::ffi::c_char,
                0x2f as ::core::ffi::c_char,
                0xa as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x5c as ::core::ffi::c_char,
                0x2d as ::core::ffi::c_char,
                0x2d as ::core::ffi::c_char,
                0x2f as ::core::ffi::c_char,
                0xa as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x2f as ::core::ffi::c_char,
                0x40 as ::core::ffi::c_char,
                0x40 as ::core::ffi::c_char,
                0x5c as ::core::ffi::c_char,
                0x5f as ::core::ffi::c_char,
                0x2d as ::core::ffi::c_char,
                0x2d as ::core::ffi::c_char,
                0x5f as ::core::ffi::c_char,
                0x5f as ::core::ffi::c_char,
                0x5f as ::core::ffi::c_char,
                0x5f as ::core::ffi::c_char,
                0xa as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x28 as ::core::ffi::c_char,
                0x5f as ::core::ffi::c_char,
                0x5f as ::core::ffi::c_char,
                0x29 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x29 as ::core::ffi::c_char,
                0xa as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x60 as ::core::ffi::c_char,
                0x60 as ::core::ffi::c_char,
                0x5c as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x5f as ::core::ffi::c_char,
                0x5f as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x7c as ::core::ffi::c_char,
                0xa as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x7c as ::core::ffi::c_char,
                0x7c as ::core::ffi::c_char,
                0x2d as ::core::ffi::c_char,
                0x27 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x60 as ::core::ffi::c_char,
                0x7c as ::core::ffi::c_char,
                0x7c as ::core::ffi::c_char,
                0xa as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x7c as ::core::ffi::c_char,
                0x7c as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x7c as ::core::ffi::c_char,
                0x7c as ::core::ffi::c_char,
                0xa as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x22 as ::core::ffi::c_char,
                0x22 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x20 as ::core::ffi::c_char,
                0x22 as ::core::ffi::c_char,
                0x22 as ::core::ffi::c_char,
                0xa as ::core::ffi::c_char,
            ];
            let mut t: uint32_t =
                monotonic_useconds().wrapping_rem(5000000 as uint64_t) as uint32_t;
            if t < 150000 as uint32_t || t >= 600000 as uint32_t && t < 750000 as uint32_t {
                mooseascii[59 as usize] = 'O' as ::core::ffi::c_char;
                mooseascii[60 as usize] = 'O' as ::core::ffi::c_char;
            } else if t >= 150000 as uint32_t && t < 300000 as uint32_t
                || t >= 450000 as uint32_t && t < 600000 as uint32_t
            {
                mooseascii[59 as usize] = 'O' as ::core::ffi::c_char;
                mooseascii[60 as usize] = 'o' as ::core::ffi::c_char;
            } else if t >= 300000 as uint32_t && t < 450000 as uint32_t {
                mooseascii[59 as usize] = 'O' as ::core::ffi::c_char;
                mooseascii[60 as usize] = '-' as ::core::ffi::c_char;
            } else {
                mooseascii[59 as usize] = 'O' as ::core::ffi::c_char;
                mooseascii[60 as usize] = 'O' as ::core::ffi::c_char;
            }
            if off >= 175 as off_t {
                fuse_reply_buf(req, ::core::ptr::null::<::core::ffi::c_char>(), 0 as size_t);
            } else if (off as size_t).wrapping_add(size) as uint64_t > 175 as uint64_t {
                fuse_reply_buf(
                    req,
                    (&raw mut mooseascii as *mut ::core::ffi::c_char).offset(off as isize),
                    (175 as off_t - off) as size_t,
                );
            } else {
                fuse_reply_buf(
                    req,
                    (&raw mut mooseascii as *mut ::core::ffi::c_char).offset(off as isize),
                    size,
                );
            }
            return;
        }
        if ino == OPLOG_INODE as fuse_ino_t || ino == OPHISTORY_INODE as fuse_ino_t {
            oplog_getdata(
                (*fi).fh as ::core::ffi::c_ulong,
                &raw mut buff,
                &raw mut ssize,
                size as uint32_t,
            );
            fuse_reply_buf(req, buff as *mut ::core::ffi::c_char, ssize as size_t);
            oplog_releasedata((*fi).fh as ::core::ffi::c_ulong);
            return;
        }
        if mfs_disables & DISABLE_READ as uint32_t != 0 {
            oplog_printf(
                &raw mut ctx,
                b"read (%lu,%llu,%llu): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                strerr(EPERM),
            );
            fuse_reply_err(req, EPERM);
            return;
        }
        if fi.is_null() {
            oplog_printf(
                &raw mut ctx,
                b"read (%lu,%llu,%llu): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                strerr(EBADF),
            );
            fuse_reply_err(req, EBADF);
            return;
        }
        fileinfo = fileinfo_get((*fi).fh);
        if (*fi).fh == 0 as uint64_t || fileinfo.is_none() {
            oplog_printf(
                &raw mut ctx,
                b"read (%lu,%llu,%llu) [handle:%08X]: %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                (*fi).fh as uint32_t,
                strerr(EBADF),
            );
            fuse_reply_err(req, EBADF);
            return;
        }
        if fileinfo.as_ref().unwrap().inode as fuse_ino_t != ino {
            oplog_printf(
                &raw mut ctx,
                b"read (%lu!=%lu,%llu,%llu) [handle:%08X]: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                fileinfo.as_ref().unwrap().inode as ::core::ffi::c_ulong,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                (*fi).fh as uint32_t,
                strerr(EBADF),
            );
            fuse_reply_err(req, EBADF);
            return;
        }
        if off as int64_t >= MAX_FILE_SIZE
            || (off as size_t).wrapping_add(size) >= MAX_FILE_SIZE as size_t
        {
            oplog_printf(
                &raw mut ctx,
                b"read (%lu,%llu,%llu) [handle:%08X]: %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                (*fi).fh as uint32_t,
                strerr(EFBIG),
            );
            fuse_reply_err(req, EFBIG);
            return;
        }
        plfsclient::inoleng::read_start(fileinfo.as_ref().unwrap().length.as_ref().unwrap());
        if fileinfo.as_ref().unwrap().state().read_data.is_none() {
            let new_data = ReadDataHandle::new(
                ino as uint32_t,
                plfsclient::inoleng::get_length(
                    fileinfo.as_ref().unwrap().length.as_ref().unwrap(),
                ),
            );
            let mut state = fileinfo.as_ref().unwrap().state();
            if state.read_data.is_none() {
                state.read_data = new_data;
            }
        }
        let (read_ptr, open_in_master) = {
            let state = fileinfo.as_ref().unwrap().state();
            (
                state.read_data.as_ref().unwrap().as_ptr(),
                state.open.in_master,
            )
        };
        oim = if open_in_master { 1 } else { 0 };
        write_data_flush_inode(ino as uint32_t);
        ssize = size as uint32_t;
        fs_atime(ino as uint32_t);
        err = read_data(
            read_ptr,
            off as uint64_t,
            &raw mut ssize,
            &raw mut buffptr,
            &raw mut iov,
            &raw mut iovcnt,
        );
        fs_atime(ino as uint32_t);
        oerr = 0 as uint8_t;
        if oim as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut state = fileinfo.as_ref().unwrap().state();
            state = fileinfo.as_ref().unwrap().wait_for_open_locked(state);
            if state.open.status != 0 as ::core::ffi::c_int {
                err = state.open.status;
                oerr = 1 as uint8_t;
            }
            drop(state);
        }
        if oerr != 0 {
            oplog_printf(
                &raw mut ctx,
                b"read (%lu,%llu,%llu) [handle:%08X] (this is open error): %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                (*fi).fh as uint32_t,
                strerr(err),
            );
            fuse_reply_err(req, err);
            fs_read_notify(0 as uint64_t);
        } else if err != 0 as ::core::ffi::c_int {
            if debug_mode != 0 {
                fprintf(
                    stderr,
                    b"IO error occurred while reading inode %lu (offset:%llu,size:%llu)\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    off as ::core::ffi::c_ulonglong,
                    size as ::core::ffi::c_ulonglong,
                );
            }
            oplog_printf(
                &raw mut ctx,
                b"read (%lu,%llu,%llu) [handle:%08X]: %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                (*fi).fh as uint32_t,
                strerr(err),
            );
            fuse_reply_err(req, err);
            fs_read_notify(0 as uint64_t);
        } else {
            if debug_mode != 0 {
                fprintf(
                    stderr,
                    b"%u bytes have been read from inode %lu (offset:%llu)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ssize,
                    ino as ::core::ffi::c_ulong,
                    off as ::core::ffi::c_ulonglong,
                );
            }
            oplog_printf(
                &raw mut ctx,
                b"read (%lu,%llu,%llu) [handle:%08X]: OK (%lu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                (*fi).fh as uint32_t,
                ssize as ::core::ffi::c_ulong,
            );
            fuse_reply_iov(req, iov, iovcnt as ::core::ffi::c_int);
            fs_read_notify(ssize as uint64_t);
        }
        read_data_free_buff(read_ptr, buffptr, iov);
        plfsclient::inoleng::read_end(fileinfo.as_ref().unwrap().length.as_ref().unwrap());
        let mut state = fileinfo.as_ref().unwrap().state();
        drop(state);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_write(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut buf: *const ::core::ffi::c_char,
    mut size: size_t,
    mut off: off_t,
    mut fi: *mut fuse_file_info,
) {
    unsafe {
        let mut fileinfo: Option<std::sync::Arc<FileInfo>> = None;
        let mut err: ::core::ffi::c_int = 0;
        let mut original_off: off_t = 0;
        let mut appendonly: uint8_t = 0;
        let mut leng: off_t = 0;
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        ctx = *fuse_req_ctx(req);
        mfs_stats_inc(OP_WRITE as ::core::ffi::c_int as uint8_t);
        if debug_mode != 0 {
            if !fi.is_null() {
                oplog_printf(
                    &raw mut ctx,
                    b"write (%lu,%llu,%llu) [handle:%08X] ...\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    size as ::core::ffi::c_ulonglong,
                    off as ::core::ffi::c_ulonglong,
                    (*fi).fh as uint32_t,
                );
            } else {
                oplog_printf(
                    &raw mut ctx,
                    b"write (%lu,%llu,%llu) ...\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    size as ::core::ffi::c_ulonglong,
                    off as ::core::ffi::c_ulonglong,
                );
            }
            fprintf(
                stderr,
                b"write to inode %lu %llu bytes at position %llu\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
            );
        }
        if ino == MASTERINFO_INODE as fuse_ino_t
            || ino == OPLOG_INODE as fuse_ino_t
            || ino == OPHISTORY_INODE as fuse_ino_t
            || ino == MOOSE_INODE as fuse_ino_t
            || ino == RANDOM_INODE as fuse_ino_t
            || ino == PARAMS_INODE as fuse_ino_t
        {
            oplog_printf(
                &raw mut ctx,
                b"write (%lu,%llu,%llu): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                strerr(EACCES),
            );
            fuse_reply_err(req, EACCES);
            return;
        }
        if ino == STATS_INODE as fuse_ino_t {
            if !fi.is_null() {
                if let Some(statsinfo) = sinfo_get((*fi).fh as uint32_t) {
                    statsinfo.lock().unwrap().reset = true;
                }
            }
            oplog_printf(
                &raw mut ctx,
                b"write (%lu,%llu,%llu): OK (%lu)\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                size as ::core::ffi::c_ulong,
            );
            fuse_reply_write(req, size);
            return;
        }
        if mfs_disables & DISABLE_WRITE as uint32_t != 0 {
            oplog_printf(
                &raw mut ctx,
                b"write (%lu,%llu,%llu): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                strerr(EPERM),
            );
            fuse_reply_err(req, EPERM);
            return;
        }
        if fi.is_null() {
            oplog_printf(
                &raw mut ctx,
                b"write (%lu,%llu,%llu): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                strerr(EBADF),
            );
            fuse_reply_err(req, EBADF);
            return;
        }
        fileinfo = fileinfo_get((*fi).fh);
        if (*fi).fh == 0 as uint64_t || fileinfo.is_none() {
            oplog_printf(
                &raw mut ctx,
                b"write (%lu,%llu,%llu) [handle:%08X]: %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                (*fi).fh as uint32_t,
                strerr(EBADF),
            );
            fuse_reply_err(req, EBADF);
            return;
        }
        if fileinfo.as_ref().unwrap().inode as fuse_ino_t != ino {
            oplog_printf(
                &raw mut ctx,
                b"write (%lu!=%lu,%llu,%llu) [handle:%08X]: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                fileinfo.as_ref().unwrap().inode as ::core::ffi::c_ulong,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                (*fi).fh as uint32_t,
                strerr(EBADF),
            );
            fuse_reply_err(req, EBADF);
            return;
        }
        if off as int64_t >= MAX_FILE_SIZE
            || (off as size_t).wrapping_add(size) >= MAX_FILE_SIZE as size_t
        {
            oplog_printf(
                &raw mut ctx,
                b"write (%lu,%llu,%llu) [handle:%08X]: %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                (*fi).fh as uint32_t,
                strerr(EFBIG),
            );
            fuse_reply_err(req, EFBIG);
            return;
        }
        let mut state = fileinfo.as_ref().unwrap().state();
        state = fileinfo.as_ref().unwrap().wait_for_open_locked(state);
        if state.open.status != 0 as ::core::ffi::c_int {
            err = state.open.status;
            drop(state);
            oplog_printf(
                &raw mut ctx,
                b"write (%lu,%llu,%llu) [handle:%08X] (this is open error): %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                (*fi).fh as uint32_t,
                strerr(err),
            );
            fuse_reply_err(req, err);
            return;
        }
        appendonly = (if state.mode as ::core::ffi::c_int == IO_RA as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
        if state.mode as ::core::ffi::c_int == IO_RO as ::core::ffi::c_int {
            drop(state);
            oplog_printf(
                &raw mut ctx,
                b"write (%lu,%llu,%llu) [handle:%08X]: %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
                (*fi).fh as uint32_t,
                strerr(EACCES),
            );
            fuse_reply_err(req, EACCES);
            return;
        }
        drop(state);
        plfsclient::inoleng::write_start(fileinfo.as_ref().unwrap().length.as_ref().unwrap());
        original_off = off;
        leng = plfsclient::inoleng::get_length(fileinfo.as_ref().unwrap().length.as_ref().unwrap())
            as off_t;
        err = 0 as ::core::ffi::c_int;
        if appendonly != 0 {
            let mut mver: uint32_t = master_version();
            if mver
                >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        113 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        113 as ::core::ffi::c_int
                    })) as uint32_t
                && mver
                    < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })) as uint32_t
                || mver
                    >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 22 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })) as uint32_t
            {
                let mut status: uint8_t = 0;
                let mut prevleng: uint64_t = 0;
                let mut gid: uint32_t = 0 as uint32_t;
                status = do_truncate(
                    ino as uint32_t,
                    (TRUNCATE_FLAG_OPENED | TRUNCATE_FLAG_UPDATE | TRUNCATE_FLAG_RESERVE)
                        as uint8_t,
                    0 as uint32_t,
                    1 as uint32_t,
                    &raw mut gid,
                    size as uint64_t,
                    ::core::ptr::null_mut::<uint8_t>(),
                    &raw mut prevleng,
                );
                if status as ::core::ffi::c_int != MFS_STATUS_OK {
                    err = mfs_errorconv(status as ::core::ffi::c_int);
                } else {
                    off = prevleng as off_t;
                }
            } else {
                off = leng;
                if (off as size_t).wrapping_add(size) >= MAX_FILE_SIZE as size_t {
                    err = EFBIG;
                }
            }
            leng = (off as size_t).wrapping_add(size) as off_t;
        }
        if err == 0 as ::core::ffi::c_int {
            if fileinfo.as_ref().unwrap().state().write_data.is_none() {
                let new_data = WriteDataHandle::new(ino as uint32_t, leng as uint64_t);
                let mut state = fileinfo.as_ref().unwrap().state();
                if state.write_data.is_none() {
                    state.write_data = new_data;
                }
            }
            let write_ptr = fileinfo
                .as_ref()
                .unwrap()
                .state()
                .write_data
                .as_ref()
                .unwrap()
                .as_ptr();
            if off as int64_t >= MAX_FILE_SIZE
                || (off as size_t).wrapping_add(size) >= MAX_FILE_SIZE as size_t
            {
                err = EFBIG;
            } else {
                fs_mtime(ino as uint32_t);
                err = write_data(
                    write_ptr,
                    off as uint64_t,
                    size as uint32_t,
                    buf as *const uint8_t,
                    (if ctx.uid == 0 as uid_t {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint8_t,
                );
                fs_mtime(ino as uint32_t);
            }
        }
        if debug_mode != 0 && off != original_off {
            fprintf(
                stderr,
                b"file offset has changed due to append mode (kernel offset: %llu , actual offset: %llu)\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                original_off as ::core::ffi::c_ulonglong,
                off as ::core::ffi::c_ulonglong,
            );
        }
        if err != 0 as ::core::ffi::c_int {
            if debug_mode != 0 {
                fprintf(
                    stderr,
                    b"IO error occurred while writing inode %lu (offset:%llu,size:%llu)\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    off as ::core::ffi::c_ulonglong,
                    size as ::core::ffi::c_ulonglong,
                );
            }
            if off != original_off {
                oplog_printf(
                    &raw mut ctx,
                    b"write (%lu,%llu,%llu->%llu) [handle:%08X]: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    size as ::core::ffi::c_ulonglong,
                    original_off as ::core::ffi::c_ulonglong,
                    off as ::core::ffi::c_ulonglong,
                    (*fi).fh as uint32_t,
                    strerr(err),
                );
            } else {
                oplog_printf(
                    &raw mut ctx,
                    b"write (%lu,%llu,%llu) [handle:%08X]: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    size as ::core::ffi::c_ulonglong,
                    off as ::core::ffi::c_ulonglong,
                    (*fi).fh as uint32_t,
                    strerr(err),
                );
            }
            fuse_reply_err(req, err);
            fs_write_notify(0 as uint64_t);
        } else {
            let mut newfleng: uint64_t = 0;
            if (off as size_t).wrapping_add(size) as uint64_t
                > plfsclient::inoleng::get_length(
                    fileinfo.as_ref().unwrap().length.as_ref().unwrap(),
                )
            {
                plfsclient::inoleng::set_length(
                    fileinfo.as_ref().unwrap().length.as_ref().unwrap(),
                    (off as uint64_t).wrapping_add(size as uint64_t),
                );
                newfleng = (off as size_t).wrapping_add(size) as uint64_t;
            } else {
                newfleng = 0 as uint64_t;
            }
            if debug_mode != 0 {
                fprintf(
                    stderr,
                    b"%llu bytes have been written to inode %lu (offset:%llu)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    size as ::core::ffi::c_ulonglong,
                    ino as ::core::ffi::c_ulong,
                    off as ::core::ffi::c_ulonglong,
                );
            }
            if off != original_off {
                oplog_printf(
                    &raw mut ctx,
                    b"write (%lu,%llu,%llu->%llu) [handle:%08X]: OK (%llu)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    size as ::core::ffi::c_ulonglong,
                    original_off as ::core::ffi::c_ulonglong,
                    off as ::core::ffi::c_ulonglong,
                    (*fi).fh as uint32_t,
                    size as ::core::ffi::c_ulonglong,
                );
            } else {
                oplog_printf(
                    &raw mut ctx,
                    b"write (%lu,%llu,%llu) [handle:%08X]: OK (%llu)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    size as ::core::ffi::c_ulonglong,
                    off as ::core::ffi::c_ulonglong,
                    (*fi).fh as uint32_t,
                    size as ::core::ffi::c_ulonglong,
                );
            }
            if newfleng > 0 as uint64_t {
                read_inode_set_length_passive(ino as uint32_t, newfleng);
                write_data_inode_setmaxfleng(ino as uint32_t, newfleng);
                finfo_change_fleng(ino as uint32_t, newfleng);
            }
            read_inode_clear_cache(ino as uint32_t, off as uint64_t, size as uint64_t);
            fdcache::invalidate(ino as uint32_t);
            fuse_reply_write(req, size);
            fs_write_notify(size as uint64_t);
        }
        plfsclient::inoleng::write_end(fileinfo.as_ref().unwrap().length.as_ref().unwrap());
    }
}
#[inline]
unsafe fn mfs_do_fsync(fileinfo: std::sync::Arc<FileInfo>) -> ::core::ffi::c_int {
    let length = fileinfo.length.as_ref().unwrap();
    plfsclient::inoleng::write_start(length);
    let inode = fileinfo.inode;
    let write_ptr = {
        let state = fileinfo.state();
        if state.mode.writable() {
            state.write_data.as_ref().map(WriteDataHandle::as_ptr)
        } else {
            None
        }
    };
    let err = write_ptr.map_or(0, |ptr| unsafe { write_data_flush(ptr) });
    if err == 0 {
        fdcache::invalidate(inode);
        dcache::invalidate_attr(inode);
    }
    plfsclient::inoleng::write_end(length);
    err
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_flush(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut fi: *mut fuse_file_info,
) {
    unsafe {
        let mut fileinfo: Option<std::sync::Arc<FileInfo>> = None;
        let mut err: ::core::ffi::c_int = 0;
        let mut uselocks: uint8_t = 0;
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        ctx = *fuse_req_ctx(req);
        mfs_stats_inc(OP_FLUSH as ::core::ffi::c_int as uint8_t);
        if debug_mode != 0 {
            if !fi.is_null() {
                oplog_printf(
                    &raw mut ctx,
                    b"flush (%lu) [handle:%08X] ...\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    (*fi).fh as uint32_t,
                );
            } else {
                oplog_printf(
                    &raw mut ctx,
                    b"flush (%lu) ...\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                );
            }
            fprintf(
                stderr,
                b"flush (%lu)\n\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
            );
        }
        if ino >= MIN_SPECIAL_INODE as fuse_ino_t {
            oplog_printf(
                &raw mut ctx,
                b"flush (%lu): OK\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
            );
            fuse_reply_err(req, 0 as ::core::ffi::c_int);
            return;
        }
        if fi.is_null() {
            oplog_printf(
                &raw mut ctx,
                b"flush (%lu): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                strerr(EBADF),
            );
            fuse_reply_err(req, EBADF);
            return;
        }
        fileinfo = fileinfo_get((*fi).fh);
        if (*fi).fh == 0 as uint64_t || fileinfo.is_none() {
            oplog_printf(
                &raw mut ctx,
                b"flush (%lu) [handle:%08XX]: %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                (*fi).fh as uint32_t,
                strerr(EBADF),
            );
            fuse_reply_err(req, EBADF);
            return;
        }
        if fileinfo.as_ref().unwrap().inode as fuse_ino_t != ino {
            oplog_printf(
                &raw mut ctx,
                b"flush (%lu!=%lu) [handle:%08X]: %s\0".as_ptr() as *const ::core::ffi::c_char,
                fileinfo.as_ref().unwrap().inode as ::core::ffi::c_ulong,
                ino as ::core::ffi::c_ulong,
                (*fi).fh as uint32_t,
                strerr(EBADF),
            );
            fuse_reply_err(req, EBADF);
            return;
        }
        err = 0 as ::core::ffi::c_int;
        uselocks = fileinfo.as_ref().unwrap().state().use_locks;
        plfsclient::inoleng::write_start(fileinfo.as_ref().unwrap().length.as_ref().unwrap());
        let write_ptr = {
            let state = fileinfo.as_ref().unwrap().state();
            if state.mode.writable() {
                state.write_data.as_ref().map(WriteDataHandle::as_ptr)
            } else {
                None
            }
        };
        if let Some(write_ptr) = write_ptr {
            if uselocks as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0
                || master_version()
                    < (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            43 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            43 as ::core::ffi::c_int
                        })) as uint32_t
                || fileinfo.as_ref().unwrap().created + fsync_before_close_min_time
                    < monotonic_seconds()
                || write_cache_almost_full() as ::core::ffi::c_int != 0
            {
                err = write_data_flush(write_ptr);
            } else {
                let gids = getgroups::get_common(ctx.pid, ctx.uid, ctx.gid, false);
                do_truncate(
                    ino as uint32_t,
                    (TRUNCATE_FLAG_OPENED | TRUNCATE_FLAG_UPDATE) as uint8_t,
                    ctx.uid as uint32_t,
                    gids.len() as u32,
                    gids.as_slice().as_ptr() as *mut u32,
                    write_data_getmaxfleng(write_ptr),
                    ::core::ptr::null_mut::<uint8_t>(),
                    ::core::ptr::null_mut::<uint64_t>(),
                );
                err = write_data_chunk_wait(write_ptr);
            }
        }
        if uselocks as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0 {
            let mut status: ::core::ffi::c_int = 0;
            status = fs_posixlock(
                ino as uint32_t,
                0 as uint32_t,
                (*fi).lock_owner,
                POSIX_LOCK_CMD_SET as uint8_t,
                POSIX_LOCK_UNLCK as uint8_t,
                0 as uint64_t,
                UINT64_MAX as uint64_t,
                0 as uint32_t,
                ::core::ptr::null_mut::<uint8_t>(),
                ::core::ptr::null_mut::<uint64_t>(),
                ::core::ptr::null_mut::<uint64_t>(),
                ::core::ptr::null_mut::<uint32_t>(),
            ) as ::core::ffi::c_int;
            status = mfs_errorconv(status);
            if status != 0 as ::core::ffi::c_int {
                oplog_printf(
                    &raw mut ctx,
                    b"flush (%lu) - releasing all POSIX-type locks for %016lX (received from kernel): %s\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    (*fi).lock_owner,
                    strerr(status),
                );
            } else {
                oplog_printf(
                    &raw mut ctx,
                    b"flush (%lu) - releasing all POSIX-type locks for %016lX (received from kernel): OK\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    (*fi).lock_owner,
                );
            }
        }
        if err != 0 as ::core::ffi::c_int {
            oplog_printf(
                &raw mut ctx,
                b"flush (%lu) [handle:%08X,uselocks:%u,lock_owner:%016lX]: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                (*fi).fh as uint32_t,
                uselocks as ::core::ffi::c_int,
                (*fi).lock_owner,
                strerr(err),
            );
        } else {
            fdcache::invalidate(ino as uint32_t);
            dcache::invalidate_attr(ino as uint32_t);
            oplog_printf(
                &raw mut ctx,
                b"flush (%lu) [handle:%08X,uselocks:%u,lock_owner:%016lX]: OK\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                (*fi).fh as uint32_t,
                uselocks as ::core::ffi::c_int,
                (*fi).lock_owner,
            );
        }
        plfsclient::inoleng::write_end(fileinfo.as_ref().unwrap().length.as_ref().unwrap());
        fuse_reply_err(req, err);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_fsync(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut datasync: ::core::ffi::c_int,
    mut fi: *mut fuse_file_info,
) {
    unsafe {
        let mut fileinfo: Option<std::sync::Arc<FileInfo>> = None;
        let mut err: ::core::ffi::c_int = 0;
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        ctx = *fuse_req_ctx(req);
        mfs_stats_inc(OP_FSYNC as ::core::ffi::c_int as uint8_t);
        if debug_mode != 0 {
            if !fi.is_null() {
                oplog_printf(
                    &raw mut ctx,
                    b"fsync (%lu,%d) [handle:%08X] ...\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    datasync,
                    (*fi).fh as uint32_t,
                );
            } else {
                oplog_printf(
                    &raw mut ctx,
                    b"fsync (%lu,%d) ...\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    datasync,
                );
            }
            fprintf(
                stderr,
                b"fsync (%lu,%d)\n\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                datasync,
            );
        }
        if ino >= MIN_SPECIAL_INODE as fuse_ino_t {
            oplog_printf(
                &raw mut ctx,
                b"fsync (%lu,%d): OK\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                datasync,
            );
            fuse_reply_err(req, 0 as ::core::ffi::c_int);
            return;
        }
        if fi.is_null() {
            oplog_printf(
                &raw mut ctx,
                b"fsync (%lu,%d): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                datasync,
                strerr(EBADF),
            );
            fuse_reply_err(req, EBADF);
            return;
        }
        fileinfo = fileinfo_get((*fi).fh);
        if (*fi).fh == 0 as uint64_t || fileinfo.is_none() {
            oplog_printf(
                &raw mut ctx,
                b"fsync (%lu,%d) [handle:%08X]: %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                datasync,
                (*fi).fh as uint32_t,
                strerr(EBADF),
            );
            fuse_reply_err(req, EBADF);
            return;
        }
        if fileinfo.as_ref().unwrap().inode as fuse_ino_t != ino {
            oplog_printf(
                &raw mut ctx,
                b"fsync (%lu!=%lu,%d) [handle:%08X]: %s\0".as_ptr() as *const ::core::ffi::c_char,
                fileinfo.as_ref().unwrap().inode as ::core::ffi::c_ulong,
                ino as ::core::ffi::c_ulong,
                datasync,
                (*fi).fh as uint32_t,
                strerr(EBADF),
            );
            fuse_reply_err(req, EBADF);
            return;
        }
        err = mfs_do_fsync(fileinfo.as_ref().unwrap().clone());
        if err != 0 as ::core::ffi::c_int {
            oplog_printf(
                &raw mut ctx,
                b"fsync (%lu,%d) [handle:%08X]: %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                datasync,
                (*fi).fh as uint32_t,
                strerr(err),
            );
        } else {
            oplog_printf(
                &raw mut ctx,
                b"fsync (%lu,%d) [handle:%08X]: OK\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                datasync,
                (*fi).fh as uint32_t,
            );
        }
        fuse_reply_err(req, err);
        fs_fsync_notify();
    }
}
static mut flock_reqid: uint32_t = 0 as uint32_t;
fn mfs_flock_interrupt(data: std::sync::Arc<FlockData>) {
    while std::sync::Arc::strong_count(&data) > 1 {
        unsafe {
            fs_flock(
                data.inode,
                data.reqid,
                data.owner,
                FLOCK_INTERRUPT as uint8_t,
            );
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_flock_interrupt_spawner(
    req: fuse_req_t,
    data: *mut ::core::ffi::c_void,
) {
    let ptr = data as *const FlockData;
    // SAFETY: libfuse callback data is an Arc::into_raw pointer held until
    // the request unregisters this callback.
    unsafe { std::sync::Arc::increment_strong_count(ptr) };
    let data = unsafe { std::sync::Arc::from_raw(ptr) };
    let ctx = unsafe { *fuse_req_ctx(req) };
    if unsafe { debug_mode } != 0 {
        unsafe {
            oplog_printf(
                &ctx,
                b"flock (%u,%u,%016lX,-): interrupted\0".as_ptr() as *const ::core::ffi::c_char,
                data.reqid,
                data.inode,
                data.owner,
            );
            fprintf(
                stderr,
                b"flock (%u,%u,%016lX,-): interrupted\n\0".as_ptr() as *const ::core::ffi::c_char,
                data.reqid,
                data.inode,
                data.owner,
            );
        }
    }
    plfscommon::lwthread::spawn_min("flock-interrupt", move || mfs_flock_interrupt(data))
        .unwrap_or_else(|_| std::process::abort());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_flock(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut fi: *mut fuse_file_info,
    mut op: ::core::ffi::c_int,
) {
    unsafe {
        let mut status: ::core::ffi::c_int = 0;
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        let mut reqid: uint32_t = 0;
        let mut owner: uint64_t = 0;
        let mut lock_mode: uint8_t = 0;
        let mut lmvalid: uint8_t = 0;
        let mut lock_mode_str: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut fileinfo: Option<std::sync::Arc<FileInfo>> = None;
        let mut fld: Option<std::sync::Arc<FlockData>> = None;
        if no_bsd_locks != 0 {
            fuse_reply_err(req, ENOSYS);
            return;
        }
        if op & LOCK_UN != 0 {
            lmvalid = 1 as uint8_t;
            lock_mode = FLOCK_UNLOCK as uint8_t;
            lock_mode_str =
                b"UNLOCK\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        } else if op & LOCK_SH != 0 {
            lmvalid = 1 as uint8_t;
            if op & LOCK_NB != 0 {
                lock_mode = FLOCK_TRY_SHARED as uint8_t;
                lock_mode_str =
                    b"TRYSH\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            } else {
                lock_mode = FLOCK_LOCK_SHARED as uint8_t;
                lock_mode_str =
                    b"LOCKSH\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            }
        } else if op & LOCK_EX != 0 {
            lmvalid = 1 as uint8_t;
            if op & LOCK_NB != 0 {
                lock_mode = FLOCK_TRY_EXCLUSIVE as uint8_t;
                lock_mode_str =
                    b"TRYEX\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            } else {
                lock_mode = FLOCK_LOCK_EXCLUSIVE as uint8_t;
                lock_mode_str =
                    b"LOCKEX\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            }
        } else {
            lmvalid = 0 as uint8_t;
            lock_mode = 0 as uint8_t;
            lock_mode_str =
                b"-\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        ctx = *fuse_req_ctx(req);
        mfs_stats_inc(OP_FLOCK as ::core::ffi::c_int as uint8_t);
        if ino >= MIN_SPECIAL_INODE as fuse_ino_t {
            if debug_mode != 0 {
                oplog_printf(
                    &raw mut ctx,
                    b"flock (-,%lu,-,%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    lock_mode_str,
                    strerr(EPERM),
                );
                fprintf(
                    stderr,
                    b"flock (-,%lu,-,%s)\n\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    lock_mode_str,
                );
            }
            fuse_reply_err(req, EPERM);
            return;
        }
        if lmvalid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if debug_mode != 0 {
                oplog_printf(
                    &raw mut ctx,
                    b"flock (-,%lu,-,%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    lock_mode_str,
                    strerr(EINVAL),
                );
                fprintf(
                    stderr,
                    b"flock (-,%lu,-,%s)\n\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    lock_mode_str,
                );
            }
            fuse_reply_err(req, EINVAL);
            return;
        }
        if fi.is_null() {
            if debug_mode != 0 {
                oplog_printf(
                    &raw mut ctx,
                    b"flock (-,%lu,-,%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    lock_mode_str,
                    strerr(EBADF),
                );
                fprintf(
                    stderr,
                    b"flock (-,%lu,-,%s)\n\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    lock_mode_str,
                );
            }
            fuse_reply_err(req, EBADF);
            return;
        }
        fileinfo = fileinfo_get((*fi).fh);
        if fileinfo.is_none() {
            if debug_mode != 0 {
                oplog_printf(
                    &raw mut ctx,
                    b"flock (-,%lu,-,%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    lock_mode_str,
                    strerr(EBADF),
                );
                fprintf(
                    stderr,
                    b"flock (-,%lu,-,%s)\n\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    lock_mode_str,
                );
            }
            fuse_reply_err(req, EBADF);
            return;
        }
        if fileinfo.as_ref().unwrap().inode as fuse_ino_t != ino {
            if debug_mode != 0 {
                oplog_printf(
                    &raw mut ctx,
                    b"flock (-,%lu!=%lu,-,%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
                    fileinfo.as_ref().unwrap().inode as ::core::ffi::c_ulong,
                    ino as ::core::ffi::c_ulong,
                    lock_mode_str,
                    strerr(EBADF),
                );
                fprintf(
                    stderr,
                    b"flock (-,%lu,-,%s)\n\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    lock_mode_str,
                );
            }
            fuse_reply_err(req, EBADF);
            return;
        }
        owner = (*fi).lock_owner;
        let mut state = fileinfo.as_ref().unwrap().state();
        state = fileinfo.as_ref().unwrap().wait_for_open_locked(state);
        if state.open.status != 0 as ::core::ffi::c_int {
            status = state.open.status;
            drop(state);
            oplog_printf(
                &raw mut ctx,
                b"flock (-,%lu,%016lX,%s) [handle:%08X] (this is open error): %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                owner,
                lock_mode_str,
                (*fi).fh as uint32_t,
                strerr(status),
            );
            fuse_reply_err(req, status);
            return;
        }
        if lock_mode as ::core::ffi::c_int != FLOCK_UNLOCK {
            state.remember_flock_owner(owner);
        }
        drop(state);
        loop {
            let c2rust_lhs = &raw mut flock_reqid;
            let c2rust_rhs = 1 as uint32_t;
            reqid = ::core::intrinsics::atomic_xadd::<
                _,
                _,
                { ::core::intrinsics::AtomicOrdering::SeqCst },
            >(c2rust_lhs, c2rust_rhs)
            .wrapping_add(c2rust_rhs);
            if reqid != 0 as uint32_t {
                break;
            }
        }
        {
            let mut state = fileinfo.as_ref().unwrap().state();
            state.use_locks |= 1;
        }
        if debug_mode != 0 {
            oplog_printf(
                &raw mut ctx,
                b"flock (%u,%lu,%016lX,%s) [handle:%08X] ...\0".as_ptr()
                    as *const ::core::ffi::c_char,
                reqid,
                ino as ::core::ffi::c_ulong,
                owner,
                lock_mode_str,
                (*fi).fh as uint32_t,
            );
            fprintf(
                stderr,
                b"flock (%u,%lu,%016lX,%s)\n\0".as_ptr() as *const ::core::ffi::c_char,
                reqid,
                ino as ::core::ffi::c_ulong,
                owner,
                lock_mode_str,
            );
        }
        if lock_mode as ::core::ffi::c_int == FLOCK_UNLOCK {
            mfs_do_fsync(fileinfo.as_ref().unwrap().clone());
        }
        if lock_mode as ::core::ffi::c_int == FLOCK_LOCK_SHARED
            || lock_mode as ::core::ffi::c_int == FLOCK_LOCK_EXCLUSIVE
        {
            let data = std::sync::Arc::new(FlockData {
                reqid,
                inode: ino as uint32_t,
                owner,
            });
            fuse_req_interrupt_func(
                req,
                Some(mfs_flock_interrupt_spawner),
                std::sync::Arc::as_ptr(&data) as *mut ::core::ffi::c_void,
            );
            if fuse_req_interrupted(req) == 0 as ::core::ffi::c_int {
                status = fs_flock(ino as uint32_t, reqid, owner, lock_mode) as ::core::ffi::c_int;
                status = mfs_errorconv(status);
            } else {
                status = EINTR;
            }
            fuse_req_interrupt_func(req, None, NULL);
            fld = Some(data);
        } else {
            status = fs_flock(ino as uint32_t, reqid, owner, lock_mode) as ::core::ffi::c_int;
            status = mfs_errorconv(status);
        }
        if status == 0 as ::core::ffi::c_int {
            oplog_printf(
                &raw mut ctx,
                b"flock (%u,%lu,%016lX,%s) [handle:%08X]: OK\0".as_ptr()
                    as *const ::core::ffi::c_char,
                reqid,
                ino as ::core::ffi::c_ulong,
                owner,
                lock_mode_str,
                (*fi).fh as uint32_t,
            );
        } else {
            oplog_printf(
                &raw mut ctx,
                b"flock (%u,%lu,%016lX,%s) [handle:%08X]: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                reqid,
                ino as ::core::ffi::c_ulong,
                owner,
                lock_mode_str,
                (*fi).fh as uint32_t,
                strerr(status),
            );
        }
        fuse_reply_err(req, status);
        drop(fld);
    }
}
static mut plock_reqid: uint32_t = 0 as uint32_t;
fn mfs_plock_interrupt(data: std::sync::Arc<PlockData>) {
    while std::sync::Arc::strong_count(&data) > 1 {
        unsafe {
            fs_posixlock(
                data.inode,
                data.reqid,
                data.owner,
                POSIX_LOCK_CMD_INT as uint8_t,
                POSIX_LOCK_UNLCK as uint8_t,
                0,
                0,
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_plock_interrupt_spawner(
    req: fuse_req_t,
    data: *mut ::core::ffi::c_void,
) {
    let ptr = data as *const PlockData;
    // SAFETY: libfuse callback data is an Arc::into_raw pointer held until
    // the request unregisters this callback.
    unsafe { std::sync::Arc::increment_strong_count(ptr) };
    let data = unsafe { std::sync::Arc::from_raw(ptr) };
    let ctx = unsafe { *fuse_req_ctx(req) };
    if unsafe { debug_mode } != 0 {
        unsafe {
            oplog_printf(
                &ctx,
                b"setlkw (%u,%016lX,%lu,%lu,%c): interrupted\0".as_ptr()
                    as *const ::core::ffi::c_char,
                data.inode,
                data.owner,
                data.start,
                data.end,
                data.ctype as ::core::ffi::c_int,
            );
            fprintf(
                stderr,
                b"setlkw (%u,%016lX,%lu,%lu,%c): interrupted\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                data.inode,
                data.owner,
                data.start,
                data.end,
                data.ctype as ::core::ffi::c_int,
            );
        }
    }
    plfscommon::lwthread::spawn_min("plock-interrupt", move || mfs_plock_interrupt(data))
        .unwrap_or_else(|_| std::process::abort());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_getlk(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut fi: *mut fuse_file_info,
    mut lock: *mut flock,
) {
    unsafe {
        let mut status: ::core::ffi::c_int = 0;
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        let mut rlock: flock = flock {
            l_type: 0,
            l_whence: 0,
            l_start: 0,
            l_len: 0,
            l_pid: 0,
        };
        let mut owner: uint64_t = 0;
        let mut start: uint64_t = 0;
        let mut end: uint64_t = 0;
        let mut rstart: uint64_t = 0;
        let mut rend: uint64_t = 0;
        let mut pid: uint32_t = 0;
        let mut rpid: uint32_t = 0;
        let mut r#type: uint8_t = 0;
        let mut rtype: uint8_t = 0;
        let mut invalid: uint8_t = 0;
        let mut ctype: ::core::ffi::c_char = 0;
        let mut rctype: ::core::ffi::c_char = 0;
        if no_posix_locks != 0 {
            fuse_reply_err(req, ENOSYS);
            return;
        }
        ctx = *fuse_req_ctx(req);
        mfs_stats_inc(OP_GETLK as ::core::ffi::c_int as uint8_t);
        if ino >= MIN_SPECIAL_INODE as fuse_ino_t {
            if debug_mode != 0 {
                oplog_printf(
                    &raw mut ctx,
                    b"getlk (inode:%lu owner:- start:- end:- type:-): %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    strerr(EPERM),
                );
                fprintf(
                    stderr,
                    b"getlk (inode:%lu owner:- start:- end:- type:-)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                );
            }
            fuse_reply_err(req, EPERM);
            return;
        }
        invalid = 0 as uint8_t;
        r#type = 0 as uint8_t;
        ctype = '-' as ::core::ffi::c_char;
        if (*lock).l_whence as ::core::ffi::c_int != SEEK_SET {
            invalid = 1 as uint8_t;
        } else if (*lock).l_type as ::core::ffi::c_int == F_UNLCK {
            r#type = POSIX_LOCK_UNLCK as uint8_t;
            ctype = 'U' as ::core::ffi::c_char;
        } else if (*lock).l_type as ::core::ffi::c_int == F_RDLCK {
            r#type = POSIX_LOCK_RDLCK as uint8_t;
            ctype = 'R' as ::core::ffi::c_char;
        } else if (*lock).l_type as ::core::ffi::c_int == F_WRLCK {
            r#type = POSIX_LOCK_WRLCK as uint8_t;
            ctype = 'W' as ::core::ffi::c_char;
        } else {
            invalid = 1 as uint8_t;
        }
        if invalid != 0 {
            if debug_mode != 0 {
                oplog_printf(
                    &raw mut ctx,
                    b"getlk (inode:%lu owner:- start:- end:- type:-): %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    strerr(EINVAL),
                );
                fprintf(
                    stderr,
                    b"getlk (inode:%lu owner:- start:- end:- type:-)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                );
            }
            fuse_reply_err(req, EINVAL);
            return;
        }
        if fi.is_null() || fileinfo_get((*fi).fh).is_none() {
            if debug_mode != 0 {
                oplog_printf(
                    &raw mut ctx,
                    b"getlk (inode:%lu owner:- start:- end:- type:-): %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    strerr(EBADF),
                );
                fprintf(
                    stderr,
                    b"getlk (inode:%lu owner:- start:- end:- type:-)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                );
            }
            fuse_reply_err(req, EBADF);
            return;
        }
        owner = (*fi).lock_owner;
        start = (*lock).l_start as uint64_t;
        if (*lock).l_len == 0 as __off64_t {
            end = UINT64_MAX as uint64_t;
        } else {
            end = start.wrapping_add((*lock).l_len as uint64_t);
        }
        pid = ctx.pid as uint32_t;
        if debug_mode != 0 {
            oplog_printf(
                &raw mut ctx,
                b"getlk (inode:%lu owner:%016lX start:%lu end:%lu type:%c) [handle:%08X] ...\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                owner,
                start,
                end,
                ctype as ::core::ffi::c_int,
                (*fi).fh as uint32_t,
            );
            fprintf(
                stderr,
                b"getlk (inode:%lu owner:%016lX start:%lu end:%lu type:%c)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                owner,
                start,
                end,
                ctype as ::core::ffi::c_int,
            );
        }
        status = fs_posixlock(
            ino as uint32_t,
            0 as uint32_t,
            owner,
            POSIX_LOCK_CMD_GET as uint8_t,
            r#type,
            start,
            end,
            pid,
            &raw mut rtype,
            &raw mut rstart,
            &raw mut rend,
            &raw mut rpid,
        ) as ::core::ffi::c_int;
        status = mfs_errorconv(status);
        if status != 0 as ::core::ffi::c_int {
            oplog_printf(
                &raw mut ctx,
                b"getlk (inode:%lu owner:%016lX start:%lu end:%lu type:%c): %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                owner,
                start,
                end,
                ctype as ::core::ffi::c_int,
                strerr(status),
            );
            fuse_reply_err(req, status);
            return;
        }
        memset(
            &raw mut rlock as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<flock>(),
        );
        if rtype as ::core::ffi::c_int == POSIX_LOCK_RDLCK {
            rlock.l_type = F_RDLCK as ::core::ffi::c_short;
            rctype = 'R' as ::core::ffi::c_char;
        } else if rtype as ::core::ffi::c_int == POSIX_LOCK_WRLCK {
            rlock.l_type = F_WRLCK as ::core::ffi::c_short;
            rctype = 'W' as ::core::ffi::c_char;
        } else {
            rlock.l_type = F_UNLCK as ::core::ffi::c_short;
            rctype = 'U' as ::core::ffi::c_char;
        }
        rlock.l_whence = SEEK_SET as ::core::ffi::c_short;
        rlock.l_start = rstart as __off64_t;
        if rend.wrapping_sub(rstart) > INT64_MAX as uint64_t {
            rlock.l_len = 0 as __off64_t;
        } else {
            rlock.l_len = rend.wrapping_sub(rstart) as __off64_t;
        }
        rlock.l_pid = rpid as __pid_t;
        oplog_printf(
            &raw mut ctx,
            b"getlk (inode:%lu owner:%016lX start:%lu end:%lu type:%c) [handle:%08X]: (start:%lu end:%lu type:%c pid:%u)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            ino as ::core::ffi::c_ulong,
            owner,
            start,
            end,
            ctype as ::core::ffi::c_int,
            (*fi).fh as uint32_t,
            rstart,
            rend,
            rctype as ::core::ffi::c_int,
            rpid,
        );
        fuse_reply_lock(req, &raw mut rlock);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_setlk(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut fi: *mut fuse_file_info,
    mut lock: *mut flock,
    mut sl: ::core::ffi::c_int,
) {
    unsafe {
        let mut status: ::core::ffi::c_int = 0;
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        let mut reqid: uint32_t = 0;
        let mut owner: uint64_t = 0;
        let mut start: uint64_t = 0;
        let mut end: uint64_t = 0;
        let mut pid: uint32_t = 0;
        let mut r#type: uint8_t = 0;
        let mut invalid: uint8_t = 0;
        let mut ctype: ::core::ffi::c_char = 0;
        let mut fileinfo: Option<std::sync::Arc<FileInfo>> = None;
        let mut pld: Option<std::sync::Arc<PlockData>> = None;
        let mut cmdname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if no_posix_locks != 0 {
            fuse_reply_err(req, ENOSYS);
            return;
        }
        ctx = *fuse_req_ctx(req);
        mfs_stats_inc(OP_SETLK as ::core::ffi::c_int as uint8_t);
        if sl != 0 {
            cmdname =
                b"setlkw\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        } else {
            cmdname = b"setlk\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        if ino >= MIN_SPECIAL_INODE as fuse_ino_t {
            if debug_mode != 0 {
                oplog_printf(
                    &raw mut ctx,
                    b"%s (inode:%lu owner:- start:- end:- type:-): %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    cmdname,
                    ino as ::core::ffi::c_ulong,
                    strerr(EPERM),
                );
                fprintf(
                    stderr,
                    b"%s (inode:%lu owner:- start:- end:- type:-)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    cmdname,
                    ino as ::core::ffi::c_ulong,
                );
            }
            fuse_reply_err(req, EPERM);
            return;
        }
        invalid = 0 as uint8_t;
        r#type = 0 as uint8_t;
        ctype = '-' as ::core::ffi::c_char;
        if (*lock).l_whence as ::core::ffi::c_int != SEEK_SET {
            invalid = 1 as uint8_t;
        } else if (*lock).l_type as ::core::ffi::c_int == F_UNLCK {
            r#type = POSIX_LOCK_UNLCK as uint8_t;
            ctype = 'U' as ::core::ffi::c_char;
        } else if (*lock).l_type as ::core::ffi::c_int == F_RDLCK {
            r#type = POSIX_LOCK_RDLCK as uint8_t;
            ctype = 'R' as ::core::ffi::c_char;
        } else if (*lock).l_type as ::core::ffi::c_int == F_WRLCK {
            r#type = POSIX_LOCK_WRLCK as uint8_t;
            ctype = 'W' as ::core::ffi::c_char;
        } else {
            invalid = 1 as uint8_t;
        }
        if invalid != 0 {
            if debug_mode != 0 {
                oplog_printf(
                    &raw mut ctx,
                    b"%s (inode:%lu owner:- start:- end:- type:-): %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    cmdname,
                    ino as ::core::ffi::c_ulong,
                    strerr(EINVAL),
                );
                fprintf(
                    stderr,
                    b"%s (inode:%lu owner:- start:- end:- type:-)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    cmdname,
                    ino as ::core::ffi::c_ulong,
                );
            }
            fuse_reply_err(req, EINVAL);
            return;
        }
        if fi.is_null() {
            if debug_mode != 0 {
                oplog_printf(
                    &raw mut ctx,
                    b"%s (inode:%lu owner:- start:- end:- type:-): %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    cmdname,
                    ino as ::core::ffi::c_ulong,
                    strerr(EBADF),
                );
                fprintf(
                    stderr,
                    b"%s (inode:%lu owner:- start:- end:- type:-)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    cmdname,
                    ino as ::core::ffi::c_ulong,
                );
            }
            fuse_reply_err(req, EBADF);
            return;
        }
        fileinfo = fileinfo_get((*fi).fh);
        if fileinfo.is_none() {
            if debug_mode != 0 {
                oplog_printf(
                    &raw mut ctx,
                    b"%s (inode:%lu owner:- start:- end:- type:-): %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    cmdname,
                    ino as ::core::ffi::c_ulong,
                    strerr(EBADF),
                );
                fprintf(
                    stderr,
                    b"%s (inode:%lu owner:- start:- end:- type:-)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    cmdname,
                    ino as ::core::ffi::c_ulong,
                );
            }
            fuse_reply_err(req, EBADF);
            return;
        }
        if fileinfo.as_ref().unwrap().inode as fuse_ino_t != ino {
            if debug_mode != 0 {
                oplog_printf(
                    &raw mut ctx,
                    b"%s (handle_inode:%lu != inode:%lu owner:- start:- end:- type:-): %s\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    cmdname,
                    fileinfo.as_ref().unwrap().inode as ::core::ffi::c_ulong,
                    ino as ::core::ffi::c_ulong,
                    strerr(EBADF),
                );
                fprintf(
                    stderr,
                    b"%s (inode:%lu owner:- start:- end:- type:-)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    cmdname,
                    ino as ::core::ffi::c_ulong,
                );
            }
            fuse_reply_err(req, EBADF);
            return;
        }
        owner = (*fi).lock_owner;
        start = (*lock).l_start as uint64_t;
        if (*lock).l_len == 0 as __off64_t {
            end = UINT64_MAX as uint64_t;
        } else {
            end = start.wrapping_add((*lock).l_len as uint64_t);
        }
        let mut state = fileinfo.as_ref().unwrap().state();
        state = fileinfo.as_ref().unwrap().wait_for_open_locked(state);
        if state.open.status != 0 as ::core::ffi::c_int {
            status = state.open.status;
            drop(state);
            oplog_printf(
                &raw mut ctx,
                b"%s (inode:%lu owner:%016lX start:%lu end:%lu type:%c) [handle:%08X] (this is open error): %s\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                cmdname,
                ino as ::core::ffi::c_ulong,
                owner,
                start,
                end,
                ctype as ::core::ffi::c_int,
                (*fi).fh as uint32_t,
                strerr(status),
            );
            fuse_reply_err(req, status);
            return;
        }
        if r#type as ::core::ffi::c_int != POSIX_LOCK_UNLCK {
            state.remember_posix_owner(finfo_core::LockOwner {
                owner,
                pid: ctx.pid as i32,
            });
        }
        drop(state);
        pid = ctx.pid as uint32_t;
        loop {
            let c2rust_lhs = &raw mut plock_reqid;
            let c2rust_rhs = 1 as uint32_t;
            reqid = ::core::intrinsics::atomic_xadd::<
                _,
                _,
                { ::core::intrinsics::AtomicOrdering::SeqCst },
            >(c2rust_lhs, c2rust_rhs)
            .wrapping_add(c2rust_rhs);
            if reqid != 0 as uint32_t {
                break;
            }
        }
        {
            let mut state = fileinfo.as_ref().unwrap().state();
            state.use_locks |= 2;
        }
        if debug_mode != 0 {
            oplog_printf(
                &raw mut ctx,
                b"%s (inode:%lu owner:%016lX start:%lu end:%lu type:%c) [handle:%08X] ...\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                cmdname,
                ino as ::core::ffi::c_ulong,
                owner,
                start,
                end,
                ctype as ::core::ffi::c_int,
                (*fi).fh as uint32_t,
            );
            fprintf(
                stderr,
                b"%s (inode:%lu owner:%016lX start:%lu end:%lu type:%c)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                cmdname,
                ino as ::core::ffi::c_ulong,
                owner,
                start,
                end,
                ctype as ::core::ffi::c_int,
            );
        }
        if r#type as ::core::ffi::c_int == POSIX_LOCK_UNLCK {
            mfs_do_fsync(fileinfo.as_ref().unwrap().clone());
            status = fs_posixlock(
                ino as uint32_t,
                reqid,
                owner,
                POSIX_LOCK_CMD_SET as uint8_t,
                POSIX_LOCK_UNLCK as uint8_t,
                start,
                end,
                pid,
                ::core::ptr::null_mut::<uint8_t>(),
                ::core::ptr::null_mut::<uint64_t>(),
                ::core::ptr::null_mut::<uint64_t>(),
                ::core::ptr::null_mut::<uint32_t>(),
            ) as ::core::ffi::c_int;
            status = mfs_errorconv(status);
        } else if sl == 0 as ::core::ffi::c_int {
            status = fs_posixlock(
                ino as uint32_t,
                reqid,
                owner,
                POSIX_LOCK_CMD_TRY as uint8_t,
                r#type,
                start,
                end,
                pid,
                ::core::ptr::null_mut::<uint8_t>(),
                ::core::ptr::null_mut::<uint64_t>(),
                ::core::ptr::null_mut::<uint64_t>(),
                ::core::ptr::null_mut::<uint32_t>(),
            ) as ::core::ffi::c_int;
            status = mfs_errorconv(status);
        } else {
            let data = std::sync::Arc::new(PlockData {
                reqid,
                inode: ino as uint32_t,
                owner,
                start,
                end,
                ctype,
            });
            fuse_req_interrupt_func(
                req,
                Some(mfs_plock_interrupt_spawner),
                std::sync::Arc::as_ptr(&data) as *mut ::core::ffi::c_void,
            );
            if fuse_req_interrupted(req) == 0 as ::core::ffi::c_int {
                status = fs_posixlock(
                    ino as uint32_t,
                    reqid,
                    owner,
                    POSIX_LOCK_CMD_SET as uint8_t,
                    r#type,
                    start,
                    end,
                    pid,
                    ::core::ptr::null_mut::<uint8_t>(),
                    ::core::ptr::null_mut::<uint64_t>(),
                    ::core::ptr::null_mut::<uint64_t>(),
                    ::core::ptr::null_mut::<uint32_t>(),
                ) as ::core::ffi::c_int;
                status = mfs_errorconv(status);
            } else {
                status = EINTR;
            }
            fuse_req_interrupt_func(req, None, NULL);
            pld = Some(data);
        }
        if status == 0 as ::core::ffi::c_int {
            oplog_printf(
                &raw mut ctx,
                b"%s (inode:%lu owner:%016lX start:%lu end:%lu type:%c) [handle:%08X]: OK\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                cmdname,
                ino as ::core::ffi::c_ulong,
                owner,
                start,
                end,
                ctype as ::core::ffi::c_int,
                (*fi).fh as uint32_t,
            );
        } else {
            oplog_printf(
                &raw mut ctx,
                b"%s (inode:%lu owner:%016lX start:%lu end:%lu type:%c) [handle:%08X]: %s\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                cmdname,
                ino as ::core::ffi::c_ulong,
                owner,
                start,
                end,
                ctype as ::core::ffi::c_int,
                (*fi).fh as uint32_t,
                strerr(status),
            );
        }
        fuse_reply_err(req, status);
        drop(pld);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_getfacl(
    _req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut aclxattr: uint8_t,
    mut buff: *mut *const uint8_t,
    mut leng: *mut uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut userperm: uint16_t = 0;
        let mut groupperm: uint16_t = 0;
        let mut otherperm: uint16_t = 0;
        let mut maskperm: uint16_t = 0;
        let mut namedusers: uint16_t = 0;
        let mut namedgroups: uint16_t = 0;
        let mut namedacls: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut b: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut namedaclssize: uint32_t = 0;
        let mut p: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut status: ::core::ffi::c_int = 0;
        *buff = ::core::ptr::null::<uint8_t>();
        *leng = 0 as uint32_t;
        userperm = 0 as uint16_t;
        groupperm = 0 as uint16_t;
        otherperm = 0 as uint16_t;
        maskperm = 0 as uint16_t;
        namedusers = 0 as uint16_t;
        namedgroups = 0 as uint16_t;
        status = fs_getfacl(
            ino as uint32_t,
            aclxattr,
            &raw mut userperm,
            &raw mut groupperm,
            &raw mut otherperm,
            &raw mut maskperm,
            &raw mut namedusers,
            &raw mut namedgroups,
            &raw mut namedacls,
            &raw mut namedaclssize,
        ) as ::core::ffi::c_int;
        if status != MFS_STATUS_OK {
            return status;
        }
        if ((namedusers as ::core::ffi::c_int + namedgroups as ::core::ffi::c_int) as uint32_t)
            .wrapping_mul(6 as uint32_t)
            != namedaclssize
        {
            return MFS_ERROR_EINVAL;
        }
        *leng = (4 as ::core::ffi::c_int
            + 32 as ::core::ffi::c_int
            + (namedusers as ::core::ffi::c_int + namedgroups as ::core::ffi::c_int)
                * 8 as ::core::ffi::c_int) as uint32_t;
        b = mfs_aclstorage_get(
            (4 as ::core::ffi::c_int
                + 32 as ::core::ffi::c_int
                + (namedusers as ::core::ffi::c_int + namedgroups as ::core::ffi::c_int)
                    * 8 as ::core::ffi::c_int) as uint32_t,
        ) as *mut uint8_t;
        *buff = b;
        p = namedacls;
        *b.offset(0 as isize) = 2 as uint8_t;
        *b.offset(1 as isize) = 0 as uint8_t;
        *b.offset(2 as isize) = 0 as uint8_t;
        *b.offset(3 as isize) = 0 as uint8_t;
        b = b.offset(4 as ::core::ffi::c_int as isize);
        *(b as *mut uint16_t) = 1 as uint16_t;
        *(b.offset(2 as ::core::ffi::c_int as isize) as *mut uint16_t) = userperm;
        *(b.offset(4 as ::core::ffi::c_int as isize) as *mut uint32_t) =
            0xffffffff as ::core::ffi::c_uint as uint32_t;
        b = b.offset(8 as ::core::ffi::c_int as isize);
        i = 0 as uint32_t;
        while i < namedusers as uint32_t {
            *(b.offset(4 as ::core::ffi::c_int as isize) as *mut uint32_t) = get32bit(&raw mut p);
            *(b as *mut uint16_t) = 2 as uint16_t;
            *(b.offset(2 as ::core::ffi::c_int as isize) as *mut uint16_t) = get16bit(&raw mut p);
            b = b.offset(8 as ::core::ffi::c_int as isize);
            i = i.wrapping_add(1);
        }
        *(b as *mut uint16_t) = 4 as uint16_t;
        *(b.offset(2 as ::core::ffi::c_int as isize) as *mut uint16_t) = groupperm;
        *(b.offset(4 as ::core::ffi::c_int as isize) as *mut uint32_t) =
            0xffffffff as ::core::ffi::c_uint as uint32_t;
        b = b.offset(8 as ::core::ffi::c_int as isize);
        i = 0 as uint32_t;
        while i < namedgroups as uint32_t {
            *(b.offset(4 as ::core::ffi::c_int as isize) as *mut uint32_t) = get32bit(&raw mut p);
            *(b as *mut uint16_t) = 8 as uint16_t;
            *(b.offset(2 as ::core::ffi::c_int as isize) as *mut uint16_t) = get16bit(&raw mut p);
            b = b.offset(8 as ::core::ffi::c_int as isize);
            i = i.wrapping_add(1);
        }
        *(b as *mut uint16_t) = 16 as uint16_t;
        *(b.offset(2 as ::core::ffi::c_int as isize) as *mut uint16_t) = maskperm;
        *(b.offset(4 as ::core::ffi::c_int as isize) as *mut uint32_t) =
            0xffffffff as ::core::ffi::c_uint as uint32_t;
        b = b.offset(8 as ::core::ffi::c_int as isize);
        *(b as *mut uint16_t) = 32 as uint16_t;
        *(b.offset(2 as ::core::ffi::c_int as isize) as *mut uint16_t) = otherperm;
        *(b.offset(4 as ::core::ffi::c_int as isize) as *mut uint32_t) =
            0xffffffff as ::core::ffi::c_uint as uint32_t;
        return MFS_STATUS_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_setfacl(
    _req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut uid: uint32_t,
    mut aclxattr: uint8_t,
    mut buff: *const ::core::ffi::c_char,
    mut leng: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut userperm: uint16_t = 0;
        let mut groupperm: uint16_t = 0;
        let mut otherperm: uint16_t = 0;
        let mut maskperm: uint16_t = 0;
        let mut namedusers: uint16_t = 0;
        let mut namedgroups: uint16_t = 0;
        let mut acls: uint16_t = 0;
        let mut p: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut namedacls: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut tag: uint16_t = 0;
        if leng < 4 as uint32_t || leng.wrapping_rem(8 as uint32_t) != 4 as uint32_t {
            return MFS_ERROR_EINVAL;
        }
        if *buff.offset(0 as isize) as ::core::ffi::c_int != 2 as ::core::ffi::c_int {
            return MFS_ERROR_EINVAL;
        }
        acls = leng.wrapping_sub(4 as uint32_t).wrapping_div(8 as uint32_t) as uint16_t;
        userperm = 0xffff as uint16_t;
        groupperm = 0xffff as uint16_t;
        otherperm = 0xffff as uint16_t;
        maskperm = 0xffff as uint16_t;
        namedusers = 0 as uint16_t;
        namedgroups = 0 as uint16_t;
        i = 0 as uint32_t;
        while i < acls as uint32_t {
            tag = *(buff
                .offset(4 as ::core::ffi::c_int as isize)
                .offset(i.wrapping_mul(8 as uint32_t) as isize)
                as *const uint16_t);
            if tag as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                if userperm as ::core::ffi::c_int != 0xffff as ::core::ffi::c_int {
                    return MFS_ERROR_EINVAL;
                }
                userperm = *(buff
                    .offset(6 as ::core::ffi::c_int as isize)
                    .offset(i.wrapping_mul(8 as uint32_t) as isize)
                    as *const uint16_t);
            }
            if tag as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0 {
                namedusers = namedusers.wrapping_add(1);
            }
            if tag as ::core::ffi::c_int & 4 as ::core::ffi::c_int != 0 {
                if groupperm as ::core::ffi::c_int != 0xffff as ::core::ffi::c_int {
                    return MFS_ERROR_EINVAL;
                }
                groupperm = *(buff
                    .offset(6 as ::core::ffi::c_int as isize)
                    .offset(i.wrapping_mul(8 as uint32_t) as isize)
                    as *const uint16_t);
            }
            if tag as ::core::ffi::c_int & 8 as ::core::ffi::c_int != 0 {
                namedgroups = namedgroups.wrapping_add(1);
            }
            if tag as ::core::ffi::c_int & 16 as ::core::ffi::c_int != 0 {
                if maskperm as ::core::ffi::c_int != 0xffff as ::core::ffi::c_int {
                    return MFS_ERROR_EINVAL;
                }
                maskperm = *(buff
                    .offset(6 as ::core::ffi::c_int as isize)
                    .offset(i.wrapping_mul(8 as uint32_t) as isize)
                    as *const uint16_t);
            }
            if tag as ::core::ffi::c_int & 32 as ::core::ffi::c_int != 0 {
                if otherperm as ::core::ffi::c_int != 0xffff as ::core::ffi::c_int {
                    return MFS_ERROR_EINVAL;
                }
                otherperm = *(buff
                    .offset(6 as ::core::ffi::c_int as isize)
                    .offset(i.wrapping_mul(8 as uint32_t) as isize)
                    as *const uint16_t);
            }
            i = i.wrapping_add(1);
        }
        if maskperm as ::core::ffi::c_int == 0xffff as ::core::ffi::c_int
            && namedusers as ::core::ffi::c_int | namedgroups as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int
        {
            return MFS_ERROR_EINVAL;
        }
        namedacls = mfs_aclstorage_get(
            ((namedusers as ::core::ffi::c_int + namedgroups as ::core::ffi::c_int)
                * 6 as ::core::ffi::c_int) as uint32_t,
        ) as *mut uint8_t;
        p = namedacls;
        i = 0 as uint32_t;
        while i < acls as uint32_t {
            tag = *(buff
                .offset(4 as ::core::ffi::c_int as isize)
                .offset(i.wrapping_mul(8 as uint32_t) as isize)
                as *const uint16_t);
            if tag as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0 {
                put32bit(
                    &raw mut p,
                    *(buff
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset(i.wrapping_mul(8 as uint32_t) as isize)
                        as *const uint32_t),
                );
                put16bit(
                    &raw mut p,
                    *(buff
                        .offset(6 as ::core::ffi::c_int as isize)
                        .offset(i.wrapping_mul(8 as uint32_t) as isize)
                        as *const uint16_t),
                );
            }
            i = i.wrapping_add(1);
        }
        i = 0 as uint32_t;
        while i < acls as uint32_t {
            tag = *(buff
                .offset(4 as ::core::ffi::c_int as isize)
                .offset(i.wrapping_mul(8 as uint32_t) as isize)
                as *const uint16_t);
            if tag as ::core::ffi::c_int & 8 as ::core::ffi::c_int != 0 {
                put32bit(
                    &raw mut p,
                    *(buff
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset(i.wrapping_mul(8 as uint32_t) as isize)
                        as *const uint32_t),
                );
                put16bit(
                    &raw mut p,
                    *(buff
                        .offset(6 as ::core::ffi::c_int as isize)
                        .offset(i.wrapping_mul(8 as uint32_t) as isize)
                        as *const uint16_t),
                );
            }
            i = i.wrapping_add(1);
        }
        return fs_setfacl(
            ino as uint32_t,
            uid,
            aclxattr,
            userperm,
            groupperm,
            otherperm,
            maskperm,
            namedusers,
            namedgroups,
            namedacls,
            ((namedusers as ::core::ffi::c_int + namedgroups as ::core::ffi::c_int)
                * 6 as ::core::ffi::c_int) as uint32_t,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_setxattr(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut name: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
    mut size: size_t,
    mut flags: ::core::ffi::c_int,
) {
    unsafe {
        let _position: uint32_t = 0 as uint32_t;
        let mut nleng: uint32_t = 0;
        let mut status: ::core::ffi::c_int = 0;
        let mut mode: uint8_t = 0;
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        let mut aclxattr: uint8_t = 0;
        if no_xattrs != 0 {
            fuse_reply_err(req, ENOSYS);
            return;
        }
        ctx = *fuse_req_ctx(req);
        mfs_stats_inc(OP_SETXATTR as ::core::ffi::c_int as uint8_t);
        if debug_mode != 0 {
            oplog_printf(
                &raw mut ctx,
                b"setxattr (%lu,%s,%llu,%d) ...\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
                size as ::core::ffi::c_ulonglong,
                flags,
            );
            fprintf(
                stderr,
                b"setxattr (%lu,%s,%llu,%d)\n\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
                size as ::core::ffi::c_ulonglong,
                flags,
            );
        }
        if ino >= MIN_SPECIAL_INODE as fuse_ino_t {
            oplog_printf(
                &raw mut ctx,
                b"setxattr (%lu,%s,%llu,%d): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
                size as ::core::ffi::c_ulonglong,
                flags,
                strerr(EPERM),
            );
            fuse_reply_err(req, EPERM);
            return;
        }
        if size > MFS_XATTR_SIZE_MAX as size_t {
            oplog_printf(
                &raw mut ctx,
                b"setxattr (%lu,%s,%llu,%d): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
                size as ::core::ffi::c_ulonglong,
                flags,
                strerr(ERANGE),
            );
            fuse_reply_err(req, ERANGE);
            return;
        }
        nleng = strlen(name) as uint32_t;
        if nleng > MFS_XATTR_NAME_MAX as uint32_t {
            oplog_printf(
                &raw mut ctx,
                b"setxattr (%lu,%s,%llu,%d): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
                size as ::core::ffi::c_ulonglong,
                flags,
                strerr(ERANGE),
            );
            fuse_reply_err(req, ERANGE);
            return;
        }
        if nleng == 0 as uint32_t {
            oplog_printf(
                &raw mut ctx,
                b"setxattr (%lu,%s,%llu,%d): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
                size as ::core::ffi::c_ulonglong,
                flags,
                strerr(EINVAL),
            );
            fuse_reply_err(req, EINVAL);
            return;
        }
        if flags & XATTR_CREATE as ::core::ffi::c_int != 0
            && flags & XATTR_REPLACE as ::core::ffi::c_int != 0
        {
            oplog_printf(
                &raw mut ctx,
                b"setxattr (%lu,%s,%llu,%d): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
                size as ::core::ffi::c_ulonglong,
                flags,
                strerr(EINVAL),
            );
            fuse_reply_err(req, EINVAL);
            return;
        }
        mode = (if flags == XATTR_CREATE as ::core::ffi::c_int {
            MFS_XATTR_CREATE_ONLY
        } else if flags == XATTR_REPLACE as ::core::ffi::c_int {
            MFS_XATTR_REPLACE_ONLY
        } else {
            MFS_XATTR_CREATE_OR_REPLACE
        }) as uint8_t;
        aclxattr = POSIX_ACL_NONE as uint8_t;
        if strcmp(
            name,
            b"system.posix_acl_access\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            aclxattr = POSIX_ACL_ACCESS as uint8_t;
        } else if strcmp(
            name,
            b"system.posix_acl_default\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            aclxattr = POSIX_ACL_DEFAULT as uint8_t;
        }
        if xattr_cache_on != 0 {
            xattr_cache_del(ino as uint32_t, nleng, name as *const uint8_t);
            xattr_cache_del(
                ino as uint32_t,
                0 as uint32_t,
                ::core::ptr::null::<uint8_t>(),
            );
        }
        if aclxattr as ::core::ffi::c_int != POSIX_ACL_NONE
            && xattr_acl_support == 0 as ::core::ffi::c_int
        {
            oplog_printf(
                &raw mut ctx,
                b"setxattr (%lu,%s,%llu,%d): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
                size as ::core::ffi::c_ulonglong,
                flags,
                strerr(ENOTSUP),
            );
            fuse_reply_err(req, ENOTSUP);
            return;
        }
        if aclxattr as ::core::ffi::c_int != POSIX_ACL_NONE {
            status = mfs_setfacl(
                req,
                ino,
                ctx.uid as uint32_t,
                aclxattr,
                value,
                size as uint32_t,
            );
        } else if full_permissions != 0 {
            let gids = getgroups::get_common(ctx.pid, ctx.uid, ctx.gid, false);
            status = fs_setxattr(
                ino as uint32_t,
                0 as uint8_t,
                ctx.uid as uint32_t,
                gids.len() as u32,
                gids.as_slice().as_ptr() as *mut u32,
                nleng as uint8_t,
                name as *const uint8_t,
                size as uint32_t,
                value as *const uint8_t,
                mode,
            ) as ::core::ffi::c_int;
        } else {
            let mut gidtmp: uint32_t = ctx.gid as uint32_t;
            status = fs_setxattr(
                ino as uint32_t,
                0 as uint8_t,
                ctx.uid as uint32_t,
                1 as uint32_t,
                &raw mut gidtmp,
                nleng as uint8_t,
                name as *const uint8_t,
                size as uint32_t,
                value as *const uint8_t,
                mode,
            ) as ::core::ffi::c_int;
        }
        status = mfs_errorconv(status);
        if status != 0 as ::core::ffi::c_int {
            oplog_printf(
                &raw mut ctx,
                b"setxattr (%lu,%s,%llu,%d): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
                size as ::core::ffi::c_ulonglong,
                flags,
                strerr(status),
            );
            fuse_reply_err(req, status);
            return;
        }
        oplog_printf(
            &raw mut ctx,
            b"setxattr (%lu,%s,%llu,%d): OK\0".as_ptr() as *const ::core::ffi::c_char,
            ino as ::core::ffi::c_ulong,
            name,
            size as ::core::ffi::c_ulonglong,
            flags,
        );
        xattr_cache_set(
            ino as uint32_t,
            ctx.uid as uint32_t,
            ctx.gid as uint32_t,
            nleng,
            name as *const uint8_t,
            value as *const uint8_t,
            size as uint32_t,
            MFS_STATUS_OK,
        );
        fuse_reply_err(req, 0 as ::core::ffi::c_int);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_getxattr(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut name: *const ::core::ffi::c_char,
    mut size: size_t,
) {
    unsafe {
        let _position: uint32_t = 0 as uint32_t;
        let mut nleng: uint32_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut status: ::core::ffi::c_int = 0;
        let mut mode: uint8_t = 0;
        let mut buff: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut leng: uint32_t = 0;
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        let mut xattr_value_release: *mut ::core::ffi::c_void =
            ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut aclxattr: uint8_t = 0;
        let mut use_cache: uint8_t = 0;
        if no_xattrs != 0 {
            fuse_reply_err(req, ENOSYS);
            return;
        }
        ctx = *fuse_req_ctx(req);
        mfs_stats_inc(OP_GETXATTR as ::core::ffi::c_int as uint8_t);
        if debug_mode != 0 {
            oplog_printf(
                &raw mut ctx,
                b"getxattr (%lu,%s,%llu) ...\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
                size as ::core::ffi::c_ulonglong,
            );
            fprintf(
                stderr,
                b"getxattr (%lu,%s,%llu)\n\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
                size as ::core::ffi::c_ulonglong,
            );
        }
        if ino >= MIN_SPECIAL_INODE as fuse_ino_t {
            oplog_printf(
                &raw mut ctx,
                b"getxattr (%lu,%s,%llu): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
                size as ::core::ffi::c_ulonglong,
                strerr(EPERM),
            );
            fuse_reply_err(req, EPERM);
            return;
        }
        nleng = strlen(name) as uint32_t;
        if nleng > MFS_XATTR_NAME_MAX as uint32_t {
            oplog_printf(
                &raw mut ctx,
                b"getxattr (%lu,%s,%llu): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
                size as ::core::ffi::c_ulonglong,
                strerr(ERANGE),
            );
            fuse_reply_err(req, ERANGE);
            return;
        }
        if nleng == 0 as uint32_t {
            oplog_printf(
                &raw mut ctx,
                b"getxattr (%lu,%s,%llu): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
                size as ::core::ffi::c_ulonglong,
                strerr(EINVAL),
            );
            fuse_reply_err(req, EINVAL);
            return;
        }
        if size == 0 as size_t {
            mode = MFS_XATTR_LENGTH_ONLY as uint8_t;
        } else {
            mode = MFS_XATTR_GETA_DATA as uint8_t;
        }
        aclxattr = POSIX_ACL_NONE as uint8_t;
        if strcmp(
            name,
            b"system.posix_acl_access\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            aclxattr = POSIX_ACL_ACCESS as uint8_t;
        } else if strcmp(
            name,
            b"system.posix_acl_default\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            aclxattr = POSIX_ACL_DEFAULT as uint8_t;
        }
        if aclxattr as ::core::ffi::c_int != POSIX_ACL_NONE
            && xattr_acl_support == 0 as ::core::ffi::c_int
        {
            oplog_printf(
                &raw mut ctx,
                b"getxattr (%lu,%s,%llu): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
                size as ::core::ffi::c_ulonglong,
                strerr(ENOTSUP),
            );
            fuse_reply_err(req, ENOTSUP);
            return;
        }
        if xattr_cache_on != 0 {
            xattr_value_release = xattr_cache_get(
                ino as uint32_t,
                ctx.uid as uint32_t,
                ctx.gid as uint32_t,
                nleng,
                name as *const uint8_t,
                &raw mut buff,
                &raw mut leng,
                &raw mut status,
            );
        } else {
            xattr_value_release = NULL;
        }
        let gids = if aclxattr as ::core::ffi::c_int == POSIX_ACL_NONE
            && full_permissions != 0
            && xattr_value_release.is_null()
        {
            Some(getgroups::get_common(ctx.pid, ctx.uid, ctx.gid, false))
        } else {
            None
        };
        use_cache = 0 as uint8_t;
        if xattr_cache_on != 0 {
            if xattr_value_release.is_null() {
                if usedircache != 0
                    && lookup_cached_attr(&ctx, ino as uint32_t, &mut attr)
                    && mfs_attr_get_mattr(&raw mut attr as *mut uint8_t as *const uint8_t)
                        as ::core::ffi::c_int
                        & MATTR_NOXATTR
                        != 0
                {
                    status = MFS_ERROR_ENOATTR;
                    buff = ::core::ptr::null::<uint8_t>();
                    leng = 0 as uint32_t;
                    use_cache = 2 as uint8_t;
                    if debug_mode != 0 {
                        fprintf(
                            stderr,
                            b"getxattr: sending negative answer using open dir cache\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                } else if aclxattr as ::core::ffi::c_int != POSIX_ACL_NONE {
                    status = mfs_getfacl(req, ino, aclxattr, &raw mut buff, &raw mut leng);
                } else if let Some(gids) = gids.as_ref() {
                    status = fs_getxattr(
                        ino as uint32_t,
                        0 as uint8_t,
                        ctx.uid as uint32_t,
                        gids.len() as u32,
                        gids.as_slice().as_ptr() as *mut u32,
                        nleng as uint8_t,
                        name as *const uint8_t,
                        MFS_XATTR_GETA_DATA as uint8_t,
                        &raw mut buff,
                        &raw mut leng,
                    ) as ::core::ffi::c_int;
                } else {
                    let mut gidtmp: uint32_t = ctx.gid as uint32_t;
                    status = fs_getxattr(
                        ino as uint32_t,
                        0 as uint8_t,
                        ctx.uid as uint32_t,
                        1 as uint32_t,
                        &raw mut gidtmp,
                        nleng as uint8_t,
                        name as *const uint8_t,
                        MFS_XATTR_GETA_DATA as uint8_t,
                        &raw mut buff,
                        &raw mut leng,
                    ) as ::core::ffi::c_int;
                }
                xattr_cache_set(
                    ino as uint32_t,
                    ctx.uid as uint32_t,
                    ctx.gid as uint32_t,
                    nleng,
                    name as *const uint8_t,
                    buff,
                    leng,
                    status,
                );
            } else {
                use_cache = 1 as uint8_t;
                if debug_mode != 0 {
                    fprintf(
                        stderr,
                        b"getxattr: sending data from cache\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            }
        } else if usedircache != 0
            && lookup_cached_attr(&ctx, ino as uint32_t, &mut attr)
            && mfs_attr_get_mattr(&raw mut attr as *mut uint8_t as *const uint8_t)
                as ::core::ffi::c_int
                & MATTR_NOXATTR
                != 0
        {
            status = MFS_ERROR_ENOATTR;
            buff = ::core::ptr::null::<uint8_t>();
            leng = 0 as uint32_t;
            use_cache = 2 as uint8_t;
            if debug_mode != 0 {
                fprintf(
                    stderr,
                    b"getxattr: sending negative answer using open dir cache\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        } else if aclxattr as ::core::ffi::c_int != POSIX_ACL_NONE {
            status = mfs_getfacl(req, ino, aclxattr, &raw mut buff, &raw mut leng);
        } else if let Some(gids) = gids.as_ref() {
            status = fs_getxattr(
                ino as uint32_t,
                0 as uint8_t,
                ctx.uid as uint32_t,
                gids.len() as u32,
                gids.as_slice().as_ptr() as *mut u32,
                nleng as uint8_t,
                name as *const uint8_t,
                mode,
                &raw mut buff,
                &raw mut leng,
            ) as ::core::ffi::c_int;
        } else {
            let mut gidtmp_0: uint32_t = ctx.gid as uint32_t;
            status = fs_getxattr(
                ino as uint32_t,
                0 as uint8_t,
                ctx.uid as uint32_t,
                1 as uint32_t,
                &raw mut gidtmp_0,
                nleng as uint8_t,
                name as *const uint8_t,
                mode,
                &raw mut buff,
                &raw mut leng,
            ) as ::core::ffi::c_int;
        }
        status = mfs_errorconv(status);
        if status != 0 as ::core::ffi::c_int {
            oplog_printf(
                &raw mut ctx,
                b"getxattr (%lu,%s,%llu)%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
                size as ::core::ffi::c_ulonglong,
                if use_cache as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                } else if use_cache as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    b" (using cache)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b" (using open dir cache)\0".as_ptr() as *const ::core::ffi::c_char
                },
                strerr(status),
            );
            fuse_reply_err(req, status);
            if !xattr_value_release.is_null() {
                xattr_cache_rel(xattr_value_release);
            }
            return;
        }
        if size == 0 as size_t {
            oplog_printf(
                &raw mut ctx,
                b"getxattr (%lu,%s,%llu)%s: OK (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
                size as ::core::ffi::c_ulonglong,
                if use_cache as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                } else if use_cache as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    b" (using cache)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b" (using open dir cache)\0".as_ptr() as *const ::core::ffi::c_char
                },
                leng,
            );
            fuse_reply_xattr(req, leng as size_t);
        } else if leng as size_t > size {
            oplog_printf(
                &raw mut ctx,
                b"getxattr (%lu,%s,%llu)%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
                size as ::core::ffi::c_ulonglong,
                if use_cache as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                } else if use_cache as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    b" (using cache)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b" (using open dir cache)\0".as_ptr() as *const ::core::ffi::c_char
                },
                strerr(ERANGE),
            );
            fuse_reply_err(req, ERANGE);
        } else {
            oplog_printf(
                &raw mut ctx,
                b"getxattr (%lu,%s,%llu)%s: OK (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
                size as ::core::ffi::c_ulonglong,
                if use_cache as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                } else if use_cache as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    b" (using cache)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b" (using open dir cache)\0".as_ptr() as *const ::core::ffi::c_char
                },
                leng,
            );
            fuse_reply_buf(req, buff as *const ::core::ffi::c_char, leng as size_t);
        }
        if !xattr_value_release.is_null() {
            xattr_cache_rel(xattr_value_release);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_listxattr(mut req: fuse_req_t, mut ino: fuse_ino_t, mut size: size_t) {
    unsafe {
        let mut buff: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut leng: uint32_t = 0;
        let mut aclbuff: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut aclleng: uint32_t = 0;
        let mut resbuff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut resleng: uint32_t = 0;
        let mut hasaccacl: uint8_t = 0;
        let mut hasdefacl: uint8_t = 0;
        let mut attr: [uint8_t; 36] = [0; 36];
        let mut vattr: uint8_t = 0;
        let mut use_cache: uint8_t = 0;
        let mut status: ::core::ffi::c_int = 0;
        let mut mode: uint8_t = 0;
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        let mut xattr_value_release: *mut ::core::ffi::c_void =
            ::core::ptr::null_mut::<::core::ffi::c_void>();
        if no_xattrs != 0 {
            fuse_reply_err(req, ENOSYS);
            return;
        }
        ctx = *fuse_req_ctx(req);
        mfs_stats_inc(OP_LISTXATTR as ::core::ffi::c_int as uint8_t);
        if debug_mode != 0 {
            oplog_printf(
                &raw mut ctx,
                b"listxattr (%lu,%llu) ...\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
            );
            fprintf(
                stderr,
                b"listxattr (%lu,%llu)\n\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
            );
        }
        if ino >= MIN_SPECIAL_INODE as fuse_ino_t {
            oplog_printf(
                &raw mut ctx,
                b"listxattr (%lu,%llu): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                strerr(EPERM),
            );
            fuse_reply_err(req, EPERM);
            return;
        }
        if size == 0 as size_t && xattr_cache_on == 0 as ::core::ffi::c_int {
            mode = MFS_XATTR_LENGTH_ONLY as uint8_t;
        } else {
            mode = MFS_XATTR_GETA_DATA as uint8_t;
        }
        hasdefacl = 0 as uint8_t;
        hasaccacl = 0 as uint8_t;
        use_cache = 0 as uint8_t;
        if xattr_cache_on != 0 {
            xattr_value_release = xattr_cache_get(
                ino as uint32_t,
                ctx.uid as uint32_t,
                ctx.gid as uint32_t,
                0 as uint32_t,
                ::core::ptr::null::<uint8_t>(),
                &raw mut buff,
                &raw mut leng,
                &raw mut status,
            );
        } else {
            xattr_value_release = NULL;
        }
        if xattr_value_release.is_null() {
            if usedircache != 0 {
                vattr = lookup_cached_attr(&ctx, ino as uint32_t, &mut attr) as uint8_t;
            } else {
                vattr = 0 as uint8_t;
            }
            if vattr as ::core::ffi::c_int != 0
                && mfs_attr_get_mattr(&raw mut attr as *mut uint8_t as *const uint8_t)
                    as ::core::ffi::c_int
                    & MATTR_NOXATTR
                    != 0
            {
                status = MFS_STATUS_OK;
                buff = ::core::ptr::null::<uint8_t>();
                leng = 0 as uint32_t;
                use_cache = 1 as uint8_t;
            } else {
                if xattr_acl_support != 0 {
                    if vattr != 0 {
                        if mfs_attr_get_type(&raw mut attr as *mut uint8_t as *const uint8_t)
                            as ::core::ffi::c_int
                            == TYPE_DIRECTORY
                        {
                            if mfs_getfacl(
                                req,
                                ino,
                                POSIX_ACL_DEFAULT as uint8_t,
                                &raw mut aclbuff,
                                &raw mut aclleng,
                            ) == MFS_STATUS_OK
                            {
                                hasdefacl = 1 as uint8_t;
                            }
                        }
                    } else if mfs_getfacl(
                        req,
                        ino,
                        POSIX_ACL_DEFAULT as uint8_t,
                        &raw mut aclbuff,
                        &raw mut aclleng,
                    ) == MFS_STATUS_OK
                    {
                        hasdefacl = 1 as uint8_t;
                    }
                    if mfs_getfacl(
                        req,
                        ino,
                        POSIX_ACL_ACCESS as uint8_t,
                        &raw mut aclbuff,
                        &raw mut aclleng,
                    ) == MFS_STATUS_OK
                    {
                        hasaccacl = 1 as uint8_t;
                    }
                }
                if full_permissions != 0 {
                    let gids = getgroups::get_common(ctx.pid, ctx.uid, ctx.gid, false);
                    status = fs_listxattr(
                        ino as uint32_t,
                        0 as uint8_t,
                        ctx.uid as uint32_t,
                        gids.len() as u32,
                        gids.as_slice().as_ptr() as *mut u32,
                        mode,
                        &raw mut buff,
                        &raw mut leng,
                    ) as ::core::ffi::c_int;
                } else {
                    let mut gidtmp: uint32_t = ctx.gid as uint32_t;
                    status = fs_listxattr(
                        ino as uint32_t,
                        0 as uint8_t,
                        ctx.uid as uint32_t,
                        1 as uint32_t,
                        &raw mut gidtmp,
                        mode,
                        &raw mut buff,
                        &raw mut leng,
                    ) as ::core::ffi::c_int;
                }
            }
        }
        status = mfs_errorconv(status);
        if status != 0 as ::core::ffi::c_int {
            oplog_printf(
                &raw mut ctx,
                b"listxattr (%lu,%llu): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                size as ::core::ffi::c_ulonglong,
                strerr(status),
            );
            fuse_reply_err(req, status);
            return;
        }
        if xattr_cache_on != 0 && xattr_value_release.is_null() {
            resleng = leng
                .wrapping_add(
                    (if hasdefacl as ::core::ffi::c_int != 0 {
                        25 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint32_t,
                )
                .wrapping_add(
                    (if hasaccacl as ::core::ffi::c_int != 0 {
                        24 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint32_t,
                );
            resbuff = malloc(resleng as size_t) as *mut ::core::ffi::c_char;
            memcpy(
                resbuff as *mut ::core::ffi::c_void,
                buff as *const ::core::ffi::c_void,
                leng as size_t,
            );
            if hasdefacl != 0 {
                memcpy(
                    resbuff.offset(leng as isize) as *mut ::core::ffi::c_void,
                    b"system.posix_acl_default\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    25 as size_t,
                );
                if hasaccacl != 0 {
                    memcpy(
                        resbuff
                            .offset(leng as isize)
                            .offset(25 as ::core::ffi::c_int as isize)
                            as *mut ::core::ffi::c_void,
                        b"system.posix_acl_access\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        24 as size_t,
                    );
                }
            } else if hasaccacl != 0 {
                memcpy(
                    resbuff.offset(leng as isize) as *mut ::core::ffi::c_void,
                    b"system.posix_acl_access\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    24 as size_t,
                );
            }
            xattr_cache_set(
                ino as uint32_t,
                ctx.uid as uint32_t,
                ctx.gid as uint32_t,
                0 as uint32_t,
                ::core::ptr::null::<uint8_t>(),
                resbuff as *const uint8_t,
                resleng,
                MFS_STATUS_OK,
            );
            free(resbuff as *mut ::core::ffi::c_void);
        }
        if !xattr_value_release.is_null() {
            if size == 0 as size_t {
                oplog_printf(
                    &raw mut ctx,
                    b"listxattr (%lu,%llu) (using cache): OK (%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    size as ::core::ffi::c_ulonglong,
                    leng,
                );
                fuse_reply_xattr(req, leng as size_t);
            } else if leng as size_t > size {
                oplog_printf(
                    &raw mut ctx,
                    b"listxattr (%lu,%llu) (using cache): %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    size as ::core::ffi::c_ulonglong,
                    strerr(ERANGE),
                );
                fuse_reply_err(req, ERANGE);
            } else {
                oplog_printf(
                    &raw mut ctx,
                    b"listxattr (%lu,%llu) (using cache): OK (%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    size as ::core::ffi::c_ulonglong,
                    leng,
                );
                fuse_reply_buf(req, buff as *const ::core::ffi::c_char, leng as size_t);
            }
            xattr_cache_rel(xattr_value_release);
        } else {
            resleng = leng
                .wrapping_add(
                    (if hasdefacl as ::core::ffi::c_int != 0 {
                        25 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint32_t,
                )
                .wrapping_add(
                    (if hasaccacl as ::core::ffi::c_int != 0 {
                        24 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint32_t,
                );
            if size == 0 as size_t {
                oplog_printf(
                    &raw mut ctx,
                    b"listxattr (%lu,%llu)%s: OK (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    size as ::core::ffi::c_ulonglong,
                    if use_cache as ::core::ffi::c_int != 0 {
                        b" (using open dir cache)\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    leng,
                );
                fuse_reply_xattr(req, resleng as size_t);
            } else if resleng as size_t > size {
                oplog_printf(
                    &raw mut ctx,
                    b"listxattr (%lu,%llu)%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    size as ::core::ffi::c_ulonglong,
                    if use_cache as ::core::ffi::c_int != 0 {
                        b" (using open dir cache)\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    strerr(ERANGE),
                );
                fuse_reply_err(req, ERANGE);
            } else if resleng > leng {
                resbuff = malloc(resleng as size_t) as *mut ::core::ffi::c_char;
                memcpy(
                    resbuff as *mut ::core::ffi::c_void,
                    buff as *const ::core::ffi::c_void,
                    leng as size_t,
                );
                if hasdefacl != 0 {
                    memcpy(
                        resbuff.offset(leng as isize) as *mut ::core::ffi::c_void,
                        b"system.posix_acl_default\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        25 as size_t,
                    );
                    if hasaccacl != 0 {
                        memcpy(
                            resbuff
                                .offset(leng as isize)
                                .offset(25 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_void,
                            b"system.posix_acl_access\0".as_ptr() as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            24 as size_t,
                        );
                    }
                } else if hasaccacl != 0 {
                    memcpy(
                        resbuff.offset(leng as isize) as *mut ::core::ffi::c_void,
                        b"system.posix_acl_access\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        24 as size_t,
                    );
                }
                oplog_printf(
                    &raw mut ctx,
                    b"listxattr (%lu,%llu)%s: OK (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    size as ::core::ffi::c_ulonglong,
                    if use_cache as ::core::ffi::c_int != 0 {
                        b" (using open dir cache)\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    resleng,
                );
                fuse_reply_buf(req, resbuff, resleng as size_t);
                free(resbuff as *mut ::core::ffi::c_void);
            } else {
                oplog_printf(
                    &raw mut ctx,
                    b"listxattr (%lu,%llu)%s: OK (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                    ino as ::core::ffi::c_ulong,
                    size as ::core::ffi::c_ulonglong,
                    if use_cache as ::core::ffi::c_int != 0 {
                        b" (using open dir cache\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    leng,
                );
                fuse_reply_buf(req, buff as *const ::core::ffi::c_char, leng as size_t);
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_removexattr(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut name: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut nleng: uint32_t = 0;
        let mut status: ::core::ffi::c_int = 0;
        let mut usecache: uint8_t = 0;
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        let mut aclxattr: uint8_t = 0;
        let mut xattr_value_release: *mut ::core::ffi::c_void =
            ::core::ptr::null_mut::<::core::ffi::c_void>();
        if no_xattrs != 0 {
            fuse_reply_err(req, ENOSYS);
            return;
        }
        ctx = *fuse_req_ctx(req);
        mfs_stats_inc(OP_REMOVEXATTR as ::core::ffi::c_int as uint8_t);
        if debug_mode != 0 {
            oplog_printf(
                &raw mut ctx,
                b"removexattr (%lu,%s) ...\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
            );
            fprintf(
                stderr,
                b"removexattr (%lu,%s)\n\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
            );
        }
        if ino >= MIN_SPECIAL_INODE as fuse_ino_t {
            oplog_printf(
                &raw mut ctx,
                b"removexattr (%lu,%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
                strerr(EPERM),
            );
            fuse_reply_err(req, EPERM);
            return;
        }
        aclxattr = POSIX_ACL_NONE as uint8_t;
        if strcmp(
            name,
            b"system.posix_acl_access\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            aclxattr = POSIX_ACL_ACCESS as uint8_t;
        } else if strcmp(
            name,
            b"system.posix_acl_default\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            aclxattr = POSIX_ACL_DEFAULT as uint8_t;
        }
        if aclxattr as ::core::ffi::c_int != POSIX_ACL_NONE
            && xattr_acl_support == 0 as ::core::ffi::c_int
        {
            oplog_printf(
                &raw mut ctx,
                b"removexattr (%lu,%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
                strerr(ENOTSUP),
            );
            fuse_reply_err(req, ENOTSUP);
            return;
        }
        nleng = strlen(name) as uint32_t;
        if nleng > MFS_XATTR_NAME_MAX as uint32_t {
            oplog_printf(
                &raw mut ctx,
                b"removexattr (%lu,%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
                strerr(ERANGE),
            );
            fuse_reply_err(req, ERANGE);
            return;
        }
        if nleng == 0 as uint32_t {
            oplog_printf(
                &raw mut ctx,
                b"removexattr (%lu,%s): %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
                strerr(EINVAL),
            );
            fuse_reply_err(req, EINVAL);
            return;
        }
        xattr_value_release = NULL;
        usecache = 0 as uint8_t;
        if xattr_cache_on != 0 {
            xattr_value_release = xattr_cache_get(
                ino as uint32_t,
                ctx.uid as uint32_t,
                ctx.gid as uint32_t,
                nleng,
                name as *const uint8_t,
                ::core::ptr::null_mut::<*const uint8_t>(),
                ::core::ptr::null_mut::<uint32_t>(),
                &raw mut status,
            );
            if !xattr_value_release.is_null() {
                if status == MFS_ERROR_ENOATTR {
                    usecache = 1 as uint8_t;
                }
                xattr_cache_rel(xattr_value_release);
            }
        }
        if usecache as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if aclxattr as ::core::ffi::c_int != POSIX_ACL_NONE {
                status = fs_setfacl(
                    ino as uint32_t,
                    ctx.uid as uint32_t,
                    aclxattr,
                    0xffff as uint16_t,
                    0xffff as uint16_t,
                    0xffff as uint16_t,
                    0xffff as uint16_t,
                    0 as uint16_t,
                    0 as uint16_t,
                    ::core::ptr::null_mut::<uint8_t>(),
                    0 as uint32_t,
                ) as ::core::ffi::c_int;
            } else if full_permissions != 0 {
                let gids = getgroups::get_common(ctx.pid, ctx.uid, ctx.gid, false);
                status = fs_removexattr(
                    ino as uint32_t,
                    0 as uint8_t,
                    ctx.uid as uint32_t,
                    gids.len() as u32,
                    gids.as_slice().as_ptr() as *mut u32,
                    nleng as uint8_t,
                    name as *const uint8_t,
                ) as ::core::ffi::c_int;
            } else {
                let mut gidtmp: uint32_t = ctx.gid as uint32_t;
                status = fs_removexattr(
                    ino as uint32_t,
                    0 as uint8_t,
                    ctx.uid as uint32_t,
                    1 as uint32_t,
                    &raw mut gidtmp,
                    nleng as uint8_t,
                    name as *const uint8_t,
                ) as ::core::ffi::c_int;
            }
        }
        if xattr_cache_on != 0 && (status == MFS_STATUS_OK || status == MFS_ERROR_ENOATTR) {
            xattr_cache_set(
                ino as uint32_t,
                ctx.uid as uint32_t,
                ctx.gid as uint32_t,
                nleng,
                name as *const uint8_t,
                ::core::ptr::null::<uint8_t>(),
                0 as uint32_t,
                MFS_ERROR_ENOATTR,
            );
            xattr_cache_del(
                ino as uint32_t,
                0 as uint32_t,
                ::core::ptr::null::<uint8_t>(),
            );
        }
        status = mfs_errorconv(status);
        if status != 0 as ::core::ffi::c_int {
            oplog_printf(
                &raw mut ctx,
                b"removexattr (%lu,%s)%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
                if usecache as ::core::ffi::c_int != 0 {
                    b" (using cache)\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
                strerr(status),
            );
            fuse_reply_err(req, status);
        } else {
            oplog_printf(
                &raw mut ctx,
                b"removexattr (%lu,%s): OK\0".as_ptr() as *const ::core::ffi::c_char,
                ino as ::core::ffi::c_ulong,
                name,
            );
            fuse_reply_err(req, 0 as ::core::ffi::c_int);
        }
        if usecache != 0 {
            if aclxattr as ::core::ffi::c_int != POSIX_ACL_NONE {
                status = fs_setfacl(
                    ino as uint32_t,
                    ctx.uid as uint32_t,
                    aclxattr,
                    0xffff as uint16_t,
                    0xffff as uint16_t,
                    0xffff as uint16_t,
                    0xffff as uint16_t,
                    0 as uint16_t,
                    0 as uint16_t,
                    ::core::ptr::null_mut::<uint8_t>(),
                    0 as uint32_t,
                ) as ::core::ffi::c_int;
            } else if full_permissions != 0 {
                let gids = getgroups::get_common(ctx.pid, ctx.uid, ctx.gid, false);
                status = fs_removexattr(
                    ino as uint32_t,
                    0 as uint8_t,
                    ctx.uid as uint32_t,
                    gids.len() as u32,
                    gids.as_slice().as_ptr() as *mut u32,
                    nleng as uint8_t,
                    name as *const uint8_t,
                ) as ::core::ffi::c_int;
            } else {
                let mut gidtmp_0: uint32_t = ctx.gid as uint32_t;
                status = fs_removexattr(
                    ino as uint32_t,
                    0 as uint8_t,
                    ctx.uid as uint32_t,
                    1 as uint32_t,
                    &raw mut gidtmp_0,
                    nleng as uint8_t,
                    name as *const uint8_t,
                ) as ::core::ffi::c_int;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_inode_clear_cache(
    mut inode: uint32_t,
    mut offset: uint64_t,
    mut leng: uint64_t,
) {
    unsafe {
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        ctx.uid = 0 as uid_t;
        ctx.gid = 0 as gid_t;
        ctx.pid = 0 as ::core::ffi::c_int as pid_t;
        fdcache::invalidate(inode);
        if !fuse_comm.is_null() {
            if fuse_lowlevel_notify_inval_inode(
                fuse_comm,
                inode as fuse_ino_t,
                offset as off_t,
                leng as off_t,
            ) < 0 as ::core::ffi::c_int
            {
                fs_clr_working_flags(WFLAG_INVALIDATE_CACHE as uint8_t);
                oplog_printf(
                    &raw mut ctx,
                    b"invalidate cache (%u:%lu:%lu): error\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    inode,
                    offset,
                    leng,
                );
            } else {
                oplog_printf(
                    &raw mut ctx,
                    b"invalidate cache (%u:%lu:%lu): ok\0".as_ptr() as *const ::core::ffi::c_char,
                    inode,
                    offset,
                    leng,
                );
            }
        } else {
            oplog_printf(
                &raw mut ctx,
                b"invalidate cache (%u:%lu:%lu): lost\0".as_ptr() as *const ::core::ffi::c_char,
                inode,
                offset,
                leng,
            );
            fs_clr_working_flags(WFLAG_INVALIDATE_CACHE as uint8_t);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_dentry_invalidate(
    mut parent: uint32_t,
    mut nleng: uint8_t,
    mut name: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut ctx: fuse_ctx = fuse_ctx {
            uid: 0,
            gid: 0,
            pid: 0,
            umask: 0,
        };
        ctx.uid = 0 as uid_t;
        ctx.gid = 0 as gid_t;
        ctx.pid = 0 as ::core::ffi::c_int as pid_t;
        if !fuse_comm.is_null() {
            fuse_lowlevel_notify_inval_entry(
                fuse_comm,
                parent as fuse_ino_t,
                name,
                nleng as size_t,
            );
            oplog_printf(
                &raw mut ctx,
                b"invalidate entry (%u:%s): ok\0".as_ptr() as *const ::core::ffi::c_char,
                parent,
                name,
            );
        } else {
            oplog_printf(
                &raw mut ctx,
                b"invalidate entry (%u:%s): lost\0".as_ptr() as *const ::core::ffi::c_char,
                parent,
                name,
            );
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_inode_change_fleng(mut inode: uint32_t, mut fleng: uint64_t) {
    unsafe {
        finfo_change_fleng(inode, fleng);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_term() {
    unsafe {
        sinfo_freeall();
        dirbuf::free_all();
        finfo_freeall();
        xattr_cache_term();
        if full_permissions != 0 {
            getgroups::term();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_setdisables(mut disables: uint32_t) {
    unsafe {
        mfs_disables = disables;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_setsession(mut se: *mut fuse_session) {
    unsafe {
        fuse_comm = se;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_init(
    mut debug_mode_in: ::core::ffi::c_int,
    mut keep_cache_in: ::core::ffi::c_int,
    mut readdirplus_cache_min_timeout_in: ::core::ffi::c_double,
    mut direntry_cache_timeout_in: ::core::ffi::c_double,
    mut entry_cache_timeout_in: ::core::ffi::c_double,
    mut attr_cache_timeout_in: ::core::ffi::c_double,
    mut xattr_cache_timeout_in: ::core::ffi::c_double,
    mut groups_cache_timeout: ::core::ffi::c_double,
    mut mkdir_copy_sgid_in: ::core::ffi::c_int,
    mut sugid_clear_mode_in: ::core::ffi::c_int,
    mut xattr_acl_support_in: ::core::ffi::c_int,
    mut fsync_before_close_min_time_in: ::core::ffi::c_double,
    mut no_xattrs_in: ::core::ffi::c_int,
    mut no_posix_locks_in: ::core::ffi::c_int,
    mut no_bsd_locks_in: ::core::ffi::c_int,
) {
    unsafe {
        let mut kver: uint32_t = 0;
        let mut sugid_clear_mode_strings: [*const ::core::ffi::c_char; 6] = [
            b"never\0".as_ptr() as *const ::core::ffi::c_char,
            b"always\0".as_ptr() as *const ::core::ffi::c_char,
            b"osx\0".as_ptr() as *const ::core::ffi::c_char,
            b"bsd\0".as_ptr() as *const ::core::ffi::c_char,
            b"ext\0".as_ptr() as *const ::core::ffi::c_char,
            b"xfs\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        kver = main_kernelversion();
        debug_mode = debug_mode_in;
        keep_cache = keep_cache_in;
        readdirplus_cache_min_timeout = readdirplus_cache_min_timeout_in;
        direntry_cache_timeout = direntry_cache_timeout_in;
        entry_cache_timeout = entry_cache_timeout_in;
        attr_cache_timeout = attr_cache_timeout_in;
        mkdir_copy_sgid = mkdir_copy_sgid_in;
        sugid_clear_mode = sugid_clear_mode_in;
        xattr_cache_init(xattr_cache_timeout_in);
        xattr_cache_on = if xattr_cache_timeout_in > 0.0f64 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        xattr_acl_support = xattr_acl_support_in;
        fsync_before_close_min_time = fsync_before_close_min_time_in;
        no_xattrs = no_xattrs_in;
        no_posix_locks = no_posix_locks_in;
        no_bsd_locks = no_bsd_locks_in;
        if groups_cache_timeout > 0.0f64 {
            getgroups::init(groups_cache_timeout, debug_mode);
            full_permissions = 1 as ::core::ffi::c_int;
        } else {
            full_permissions = 0 as ::core::ffi::c_int;
        }
        fdcache::init();
        mfs_aclstorage_init();
        if debug_mode != 0 {
            fprintf(
                stderr,
                b"kernel version: %u.%u\n\0".as_ptr() as *const ::core::ffi::c_char,
                kver >> 16 as ::core::ffi::c_int,
                kver & 0xffff as uint32_t,
            );
            fprintf(
                stderr,
                b"cache parameters: file_keep_cache=%s direntry_cache_timeout=%.2lf entry_cache_timeout=%.2lf attr_cache_timeout=%.2lf readdirplus_cache_min_timeout=%.6lf xattr_cache_timeout_in=%.2lf (%s)\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                if keep_cache == 1 as ::core::ffi::c_int {
                    b"always\0".as_ptr() as *const ::core::ffi::c_char
                } else if keep_cache == 2 as ::core::ffi::c_int {
                    b"never\0".as_ptr() as *const ::core::ffi::c_char
                } else if keep_cache == 3 as ::core::ffi::c_int {
                    b"direct\0".as_ptr() as *const ::core::ffi::c_char
                } else if keep_cache == 4 as ::core::ffi::c_int {
                    b"fbsdauto\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"auto\0".as_ptr() as *const ::core::ffi::c_char
                },
                direntry_cache_timeout,
                entry_cache_timeout,
                attr_cache_timeout,
                readdirplus_cache_min_timeout,
                xattr_cache_timeout_in,
                if xattr_cache_on != 0 {
                    b"on\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"off\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            fprintf(
                stderr,
                b"mkdir copy sgid=%d\nsugid clear mode=%s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mkdir_copy_sgid_in,
                if sugid_clear_mode_in < SUGID_CLEAR_MODE_OPTIONS {
                    sugid_clear_mode_strings[sugid_clear_mode_in as usize]
                } else {
                    b"???\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
        mfs_statsptr_init();
        if kver
            < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int + 19 as ::core::ffi::c_int)
                as uint32_t
        {
            dinval = 1 as ::core::ffi::c_int;
            if debug_mode != 0 {
                fprintf(
                    stderr,
                    b"turn on dentry invalidator\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            dinval_init(direntry_cache_timeout);
        } else {
            dinval = 0 as ::core::ffi::c_int;
        };
    }
}
