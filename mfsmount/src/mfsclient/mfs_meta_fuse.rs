pub enum fuse_req {}
pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn fuse_reply_err(req: fuse_req_t, err: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn fuse_reply_entry(req: fuse_req_t, e: *const fuse_entry_param) -> ::core::ffi::c_int;
    unsafe fn fuse_reply_attr(
        req: fuse_req_t,
        attr: *const stat,
        attr_timeout: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    unsafe fn fuse_reply_open(req: fuse_req_t, fi: *const fuse_file_info) -> ::core::ffi::c_int;
    unsafe fn fuse_reply_write(req: fuse_req_t, count: size_t) -> ::core::ffi::c_int;
    unsafe fn fuse_reply_buf(
        req: fuse_req_t,
        buf: *const ::core::ffi::c_char,
        size: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn fuse_reply_statfs(req: fuse_req_t, stbuf: *const statvfs) -> ::core::ffi::c_int;
    unsafe fn fuse_add_direntry(
        req: fuse_req_t,
        buf: *mut ::core::ffi::c_char,
        bufsize: size_t,
        name: *const ::core::ffi::c_char,
        stbuf: *const stat,
        off: off_t,
    ) -> size_t;
    static mut stderr: *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
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
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn fs_getmasterlocation(loc: *mut uint8_t);
    unsafe fn fs_statfs(
        totalspace: *mut uint64_t,
        availspace: *mut uint64_t,
        freespace: *mut uint64_t,
        trashspace: *mut uint64_t,
        sustainedspace: *mut uint64_t,
        inodes: *mut uint32_t,
    );
    unsafe fn fs_getsustained(dbuff: *mut *const uint8_t, dbuffsize: *mut uint32_t) -> uint8_t;
    unsafe fn fs_gettrash(
        tid: uint32_t,
        dbuff: *mut *const uint8_t,
        dbuffsize: *mut uint32_t,
    ) -> uint8_t;
    unsafe fn fs_getdetachedattr(inode: uint32_t, attr: *mut uint8_t) -> uint8_t;
    unsafe fn fs_gettrashpath(inode: uint32_t, path: *mut *const uint8_t) -> uint8_t;
    unsafe fn fs_settrashpath(inode: uint32_t, path: *const uint8_t) -> uint8_t;
    unsafe fn fs_undel(inode: uint32_t) -> uint8_t;
    unsafe fn fs_purge(inode: uint32_t) -> uint8_t;
    unsafe fn master_version() -> uint32_t;
    unsafe fn masterproxy_getlocation(masterinfo: *mut uint8_t);
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
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __fsblkcnt64_t = ::core::ffi::c_ulong;
pub type __fsfilcnt64_t = ::core::ffi::c_ulong;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type off_t = __off64_t;
pub type time_t = __time_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: ::core::ffi::c_int,
    pub __count: ::core::ffi::c_uint,
    pub __owner: ::core::ffi::c_int,
    pub __nusers: ::core::ffi::c_uint,
    pub __kind: ::core::ffi::c_int,
    pub __spins: ::core::ffi::c_short,
    pub __glibc_reserved: ::core::ffi::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dirbuf {
    pub wasread: ::core::ffi::c_int,
    pub p: *mut uint8_t,
    pub size: size_t,
    pub lock: pthread_mutex_t,
}
pub type dirbuf = _dirbuf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _pathbuf {
    pub changed: ::core::ffi::c_int,
    pub p: *mut ::core::ffi::c_char,
    pub size: size_t,
    pub lock: pthread_mutex_t,
}
pub type pathbuf = _pathbuf;
pub const __S_IFDIR: ::core::ffi::c_int = 0o40000 as ::core::ffi::c_int;
pub const __S_IFCHR: ::core::ffi::c_int = 0o20000 as ::core::ffi::c_int;
pub const __S_IFBLK: ::core::ffi::c_int = 0o60000 as ::core::ffi::c_int;
pub const __S_IFREG: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const __S_IFIFO: ::core::ffi::c_int = 0o10000 as ::core::ffi::c_int;
pub const __S_IFLNK: ::core::ffi::c_int = 0o120000 as ::core::ffi::c_int;
pub const __S_IFSOCK: ::core::ffi::c_int = 0o140000 as ::core::ffi::c_int;
pub const S_IFDIR: ::core::ffi::c_int = __S_IFDIR;
pub const S_IFCHR: ::core::ffi::c_int = __S_IFCHR;
pub const S_IFBLK: ::core::ffi::c_int = __S_IFBLK;
pub const S_IFREG: ::core::ffi::c_int = __S_IFREG;
pub const S_IFIFO: ::core::ffi::c_int = __S_IFIFO;
pub const S_IFLNK: ::core::ffi::c_int = __S_IFLNK;
pub const S_IFSOCK: ::core::ffi::c_int = __S_IFSOCK;
pub const FUSE_ROOT_ID: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EIO: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const EBADF: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const EEXIST: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const ENOTDIR: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const EROFS: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const ENOTEMPTY: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
pub const EDQUOT: ::core::ffi::c_int = 122 as ::core::ffi::c_int;
pub const MFSBLOCKSIZE: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const TRASH_BUCKETS: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0;
pub const MFS_ERROR_EPERM: ::core::ffi::c_int = 1;
pub const MFS_ERROR_ENOTDIR: ::core::ffi::c_int = 2;
pub const MFS_ERROR_ENOENT: ::core::ffi::c_int = 3;
pub const MFS_ERROR_EACCES: ::core::ffi::c_int = 4;
pub const MFS_ERROR_EEXIST: ::core::ffi::c_int = 5;
pub const MFS_ERROR_EINVAL: ::core::ffi::c_int = 6;
pub const MFS_ERROR_ENOTEMPTY: ::core::ffi::c_int = 7;
pub const MFS_ERROR_IO: ::core::ffi::c_int = 22;
pub const MFS_ERROR_EROFS: ::core::ffi::c_int = 33;
pub const MFS_ERROR_QUOTA: ::core::ffi::c_int = 34;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
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
pub const VERSMAJ: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const VERSMID: ::core::ffi::c_int = 59 as ::core::ffi::c_int;
pub const VERSMIN: ::core::ffi::c_int = 2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
pub const READDIR_BUFFSIZE: ::core::ffi::c_int = 50000 as ::core::ffi::c_int;
pub const NAME_MAX: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
pub const PATH_SIZE_LIMIT: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const META_ROOT_INODE: ::core::ffi::c_int = FUSE_ROOT_ID;
pub const META_ROOT_MODE: ::core::ffi::c_int = 0o555 as ::core::ffi::c_int;
pub const META_SUBTRASH_INODE_MIN: ::core::ffi::c_int = 0x7fff0000 as ::core::ffi::c_int;
pub const META_SUBTRASH_INODE_MAX: ::core::ffi::c_int =
    0x7fff0000 as ::core::ffi::c_int + TRASH_BUCKETS - 1 as ::core::ffi::c_int;
pub const META_SUBTRASH_MODE: ::core::ffi::c_int = 0o700 as ::core::ffi::c_int;
pub const META_TRASH_INODE: ::core::ffi::c_int = 0x7ffffff8 as ::core::ffi::c_int;
pub const META_TRASH_MODE: ::core::ffi::c_int = 0o700 as ::core::ffi::c_int;
pub const META_TRASH_NAME: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"trash\0") };
pub const META_UNDEL_INODE: ::core::ffi::c_int = 0x7ffffff9 as ::core::ffi::c_int;
pub const META_UNDEL_MODE: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const META_UNDEL_NAME: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"undel\0") };
pub const META_SUSTAINED_INODE: ::core::ffi::c_int = 0x7ffffffa as ::core::ffi::c_int;
pub const META_SUSTAINED_MODE: ::core::ffi::c_int = 0o500 as ::core::ffi::c_int;
pub const META_SUSTAINED_NAME: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"sustained\0") };
pub const MASTERINFO_NAME: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b".masterinfo\0") };
pub const MASTERINFO_INODE: ::core::ffi::c_int = 0x7fffffff as ::core::ffi::c_int;
static mut masterinfoattr: [uint8_t; 36] = [
    'f' as uint8_t,
    0x1 as uint8_t,
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
    0,
];
pub const MIN_SPECIAL_INODE: ::core::ffi::c_int = 0x7fff0000 as ::core::ffi::c_int;
pub const PKGVERSION: ::core::ffi::c_int =
    VERSMAJ * 1000000 as ::core::ffi::c_int + VERSMID * 1000 as ::core::ffi::c_int + VERSMIN;
static mut debug_mode: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut flat_trash: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut entry_cache_timeout: ::core::ffi::c_double = 0.0f64;
static mut attr_cache_timeout: ::core::ffi::c_double = 1.0f64;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_name_to_inode(mut name: *const ::core::ffi::c_char) -> uint32_t {
    unsafe {
        let mut inode: uint32_t = 0 as uint32_t;
        let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        inode = strtoul(name, &raw mut end, 16 as ::core::ffi::c_int) as uint32_t;
        if *end as ::core::ffi::c_int == '|' as ::core::ffi::c_int
            && *end.offset(1 as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            return inode;
        } else {
            return 0 as uint32_t;
        };
    }
}
unsafe extern "C" fn mfs_errorconv(mut status: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        match status {
            MFS_STATUS_OK => return 0 as ::core::ffi::c_int,
            MFS_ERROR_EPERM => return EPERM,
            MFS_ERROR_ENOTDIR => return ENOTDIR,
            MFS_ERROR_ENOENT => return ENOENT,
            MFS_ERROR_EACCES => return EACCES,
            MFS_ERROR_EEXIST => return EEXIST,
            MFS_ERROR_EINVAL => return EINVAL,
            MFS_ERROR_ENOTEMPTY => return ENOTEMPTY,
            MFS_ERROR_IO => return EIO,
            MFS_ERROR_EROFS => return EROFS,
            MFS_ERROR_QUOTA => return EDQUOT,
            _ => return EINVAL,
        };
    }
}
#[inline]
unsafe extern "C" fn fsnodes_type_convert(mut r#type: uint8_t) -> uint8_t {
    unsafe {
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
}
unsafe extern "C" fn mfs_meta_type_to_stat(
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
unsafe extern "C" fn mfs_meta_stat(mut inode: uint32_t, mut stbuf: *mut stat) {
    unsafe {
        let mut now: ::core::ffi::c_int = 0;
        (*stbuf).st_ino = inode as __ino_t;
        (*stbuf).st_size = 0 as __off_t;
        (*stbuf).st_blksize = MFSBLOCKSIZE as __blksize_t;
        match inode {
            1 => {
                (*stbuf).st_nlink = 4 as __nlink_t;
                (*stbuf).st_mode = (S_IFDIR | META_ROOT_MODE) as __mode_t;
            }
            2147483640 => {
                (*stbuf).st_nlink = (3 as ::core::ffi::c_int + TRASH_BUCKETS) as __nlink_t;
                (*stbuf).st_mode = (S_IFDIR | META_TRASH_MODE) as __mode_t;
            }
            2147483641 => {
                (*stbuf).st_nlink = (2 as ::core::ffi::c_int + TRASH_BUCKETS) as __nlink_t;
                (*stbuf).st_mode = (S_IFDIR | META_UNDEL_MODE) as __mode_t;
            }
            2147483642 => {
                (*stbuf).st_nlink = 2 as __nlink_t;
                (*stbuf).st_mode = (S_IFDIR | META_SUSTAINED_MODE) as __mode_t;
            }
            _ => {
                if inode >= META_SUBTRASH_INODE_MIN as uint32_t
                    && inode <= META_SUBTRASH_INODE_MAX as uint32_t
                {
                    (*stbuf).st_nlink = 3 as __nlink_t;
                    (*stbuf).st_mode = (S_IFDIR | META_SUBTRASH_MODE) as __mode_t;
                }
            }
        }
        (*stbuf).st_uid = 0 as __uid_t;
        (*stbuf).st_gid = 0 as __gid_t;
        now = time(::core::ptr::null_mut::<time_t>()) as ::core::ffi::c_int;
        (*stbuf).st_atim.tv_sec = now as __time_t;
        (*stbuf).st_mtim.tv_sec = now as __time_t;
        (*stbuf).st_ctim.tv_sec = now as __time_t;
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
        attrlength = get64bit(&raw mut ptr);
        (*stbuf).st_ino = inode as __ino_t;
        (*stbuf).st_blksize = MFSBLOCKSIZE as __blksize_t;
        if attrtype as ::core::ffi::c_int == TYPE_FILE
            || attrtype as ::core::ffi::c_int == TYPE_TRASH
            || attrtype as ::core::ffi::c_int == TYPE_SUSTAINED
        {
            (*stbuf).st_mode = (S_IFREG
                | attrmode as ::core::ffi::c_int & 0o7777 as ::core::ffi::c_int)
                as __mode_t;
        } else {
            (*stbuf).st_mode = 0 as __mode_t;
        }
        (*stbuf).st_size = attrlength as __off_t;
        (*stbuf).st_blocks = attrlength
            .wrapping_add(511 as uint64_t)
            .wrapping_div(512 as uint64_t) as __blkcnt_t;
        (*stbuf).st_uid = attruid as __uid_t;
        (*stbuf).st_gid = attrgid as __gid_t;
        (*stbuf).st_atim.tv_sec = attratime as __time_t;
        (*stbuf).st_mtim.tv_sec = attrmtime as __time_t;
        (*stbuf).st_ctim.tv_sec = attrctime as __time_t;
        (*stbuf).st_nlink = attrnlink as __nlink_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_statfs(mut req: fuse_req_t, mut ino: fuse_ino_t) {
    unsafe {
        let mut totalspace: uint64_t = 0;
        let mut availspace: uint64_t = 0;
        let mut freespace: uint64_t = 0;
        let mut trashspace: uint64_t = 0;
        let mut sustainedspace: uint64_t = 0;
        let mut inodes: uint32_t = 0;
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
        fs_statfs(
            &raw mut totalspace,
            &raw mut availspace,
            &raw mut freespace,
            &raw mut trashspace,
            &raw mut sustainedspace,
            &raw mut inodes,
        );
        stfsbuf.f_namemax = NAME_MAX as ::core::ffi::c_ulong;
        stfsbuf.f_frsize = MFSBLOCKSIZE as ::core::ffi::c_ulong;
        stfsbuf.f_bsize = MFSBLOCKSIZE as ::core::ffi::c_ulong;
        stfsbuf.f_blocks = trashspace
            .wrapping_div(MFSBLOCKSIZE as uint64_t)
            .wrapping_add(sustainedspace.wrapping_div(MFSBLOCKSIZE as uint64_t))
            as __fsblkcnt64_t;
        stfsbuf.f_bfree = sustainedspace.wrapping_div(MFSBLOCKSIZE as uint64_t) as __fsblkcnt64_t;
        stfsbuf.f_bavail = sustainedspace.wrapping_div(MFSBLOCKSIZE as uint64_t) as __fsblkcnt64_t;
        stfsbuf.f_files = (1000000000 as ::core::ffi::c_int + PKGVERSION) as __fsfilcnt64_t;
        stfsbuf.f_ffree = (1000000000 as ::core::ffi::c_int + PKGVERSION) as __fsfilcnt64_t;
        stfsbuf.f_favail = (1000000000 as ::core::ffi::c_int + PKGVERSION) as __fsfilcnt64_t;
        fuse_reply_statfs(req, &raw mut stfsbuf);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_lookup(
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
        let mut inode: uint32_t = 0;
        memset(
            &raw mut e as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<fuse_entry_param>(),
        );
        inode = 0 as uint32_t;
        match parent {
            1 => {
                if strcmp(name, b".\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(name, b"..\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                {
                    inode = META_ROOT_INODE as uint32_t;
                } else if strcmp(name, META_TRASH_NAME.as_ptr()) == 0 as ::core::ffi::c_int {
                    inode = META_TRASH_INODE as uint32_t;
                } else if strcmp(name, META_SUSTAINED_NAME.as_ptr()) == 0 as ::core::ffi::c_int {
                    inode = META_SUSTAINED_INODE as uint32_t;
                } else if strcmp(name, MASTERINFO_NAME.as_ptr()) == 0 as ::core::ffi::c_int {
                    memset(
                        &raw mut e as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<fuse_entry_param>(),
                    );
                    e.ino = MASTERINFO_INODE as fuse_ino_t;
                    e.attr_timeout = 3600.0f64;
                    e.entry_timeout = 3600.0f64;
                    mfs_attr_to_stat(
                        MASTERINFO_INODE as uint32_t,
                        &raw mut masterinfoattr as *mut uint8_t as *const uint8_t,
                        &raw mut e.attr,
                    );
                    fuse_reply_entry(req, &raw mut e);
                    return;
                }
            }
            2147483640 => {
                if strcmp(name, b".\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    inode = META_TRASH_INODE as uint32_t;
                } else if strcmp(name, b"..\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    inode = META_ROOT_INODE as uint32_t;
                } else if strcmp(name, META_UNDEL_NAME.as_ptr()) == 0 as ::core::ffi::c_int {
                    inode = META_UNDEL_INODE as uint32_t;
                } else if master_version()
                    >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            64 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            64 as ::core::ffi::c_int
                        })) as uint32_t
                    && flat_trash == 0 as ::core::ffi::c_int
                {
                    inode = strtoul(
                        name,
                        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                        16 as ::core::ffi::c_int,
                    ) as uint32_t;
                    if inode < TRASH_BUCKETS as uint32_t {
                        inode = inode.wrapping_add(META_SUBTRASH_INODE_MIN as uint32_t);
                    } else {
                        inode = 0 as uint32_t;
                    }
                } else {
                    inode = mfs_meta_name_to_inode(name);
                    if inode > 0 as uint32_t {
                        let mut status: ::core::ffi::c_int = 0;
                        let mut attr: [uint8_t; 36] = [0; 36];
                        status = fs_getdetachedattr(inode, &raw mut attr as *mut uint8_t)
                            as ::core::ffi::c_int;
                        status = mfs_errorconv(status);
                        if status != 0 as ::core::ffi::c_int {
                            fuse_reply_err(req, status);
                        } else {
                            e.ino = inode as fuse_ino_t;
                            e.attr_timeout = attr_cache_timeout;
                            e.entry_timeout = entry_cache_timeout;
                            mfs_attr_to_stat(
                                inode,
                                &raw mut attr as *mut uint8_t as *const uint8_t,
                                &raw mut e.attr,
                            );
                            fuse_reply_entry(req, &raw mut e);
                        }
                        return;
                    }
                }
            }
            2147483641 => {
                if strcmp(name, b".\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    inode = META_UNDEL_INODE as uint32_t;
                } else if strcmp(name, b"..\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    inode = META_TRASH_INODE as uint32_t;
                }
            }
            2147483642 => {
                if strcmp(name, b".\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    inode = META_SUSTAINED_INODE as uint32_t;
                } else if strcmp(name, b"..\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    inode = META_ROOT_INODE as uint32_t;
                } else {
                    inode = mfs_meta_name_to_inode(name);
                    if inode > 0 as uint32_t {
                        let mut status_0: ::core::ffi::c_int = 0;
                        let mut attr_0: [uint8_t; 36] = [0; 36];
                        status_0 = fs_getdetachedattr(inode, &raw mut attr_0 as *mut uint8_t)
                            as ::core::ffi::c_int;
                        status_0 = mfs_errorconv(status_0);
                        if status_0 != 0 as ::core::ffi::c_int {
                            fuse_reply_err(req, status_0);
                        } else {
                            e.ino = inode as fuse_ino_t;
                            e.attr_timeout = attr_cache_timeout;
                            e.entry_timeout = entry_cache_timeout;
                            mfs_attr_to_stat(
                                inode,
                                &raw mut attr_0 as *mut uint8_t as *const uint8_t,
                                &raw mut e.attr,
                            );
                            fuse_reply_entry(req, &raw mut e);
                        }
                        return;
                    }
                }
            }
            _ => {
                if parent >= META_SUBTRASH_INODE_MIN as fuse_ino_t
                    && parent <= META_SUBTRASH_INODE_MAX as fuse_ino_t
                {
                    if strcmp(name, b".\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        inode = parent as uint32_t;
                    } else if strcmp(name, b"..\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        inode = META_TRASH_INODE as uint32_t;
                    } else if strcmp(name, META_UNDEL_NAME.as_ptr()) == 0 as ::core::ffi::c_int {
                        inode = META_UNDEL_INODE as uint32_t;
                    } else {
                        inode = mfs_meta_name_to_inode(name);
                        if inode > 0 as uint32_t {
                            let mut status_1: ::core::ffi::c_int = 0;
                            let mut attr_1: [uint8_t; 36] = [0; 36];
                            status_1 = fs_getdetachedattr(inode, &raw mut attr_1 as *mut uint8_t)
                                as ::core::ffi::c_int;
                            status_1 = mfs_errorconv(status_1);
                            if status_1 != 0 as ::core::ffi::c_int {
                                fuse_reply_err(req, status_1);
                            } else {
                                e.ino = inode as fuse_ino_t;
                                e.attr_timeout = attr_cache_timeout;
                                e.entry_timeout = entry_cache_timeout;
                                mfs_attr_to_stat(
                                    inode,
                                    &raw mut attr_1 as *mut uint8_t as *const uint8_t,
                                    &raw mut e.attr,
                                );
                                fuse_reply_entry(req, &raw mut e);
                            }
                            return;
                        }
                    }
                }
            }
        }
        if inode == 0 as uint32_t {
            fuse_reply_err(req, ENOENT);
        } else {
            e.ino = inode as fuse_ino_t;
            e.attr_timeout = attr_cache_timeout;
            e.entry_timeout = entry_cache_timeout;
            mfs_meta_stat(inode, &raw mut e.attr);
            fuse_reply_entry(req, &raw mut e);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_getattr(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
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
            fuse_reply_attr(req, &raw mut o_stbuf, 3600.0f64);
        } else if ino >= MIN_SPECIAL_INODE as fuse_ino_t || ino == META_ROOT_INODE as fuse_ino_t {
            memset(
                &raw mut o_stbuf as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<stat>(),
            );
            mfs_meta_stat(ino as uint32_t, &raw mut o_stbuf);
            fuse_reply_attr(req, &raw mut o_stbuf, attr_cache_timeout);
        } else {
            let mut status: ::core::ffi::c_int = 0;
            let mut attr: [uint8_t; 36] = [0; 36];
            status = fs_getdetachedattr(ino as uint32_t, &raw mut attr as *mut uint8_t)
                as ::core::ffi::c_int;
            status = mfs_errorconv(status);
            if status != 0 as ::core::ffi::c_int {
                fuse_reply_err(req, status);
            } else {
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
                fuse_reply_attr(req, &raw mut o_stbuf, attr_cache_timeout);
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_setattr(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut stbuf: *mut stat,
    mut to_set: ::core::ffi::c_int,
    mut fi: *mut fuse_file_info,
) {
    unsafe {
        mfs_meta_getattr(req, ino, fi);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_unlink(
    mut req: fuse_req_t,
    mut parent: fuse_ino_t,
    mut name: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut status: ::core::ffi::c_int = 0;
        let mut inode: uint32_t = 0;
        if !(parent == META_TRASH_INODE as fuse_ino_t
            || parent >= META_SUBTRASH_INODE_MIN as fuse_ino_t
                && parent <= META_SUBTRASH_INODE_MAX as fuse_ino_t)
        {
            fuse_reply_err(req, EACCES);
            return;
        }
        inode = mfs_meta_name_to_inode(name);
        if inode == 0 as uint32_t {
            fuse_reply_err(req, ENOENT);
            return;
        }
        status = fs_purge(inode) as ::core::ffi::c_int;
        status = mfs_errorconv(status);
        fuse_reply_err(req, status);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_rename(
    mut req: fuse_req_t,
    mut parent: fuse_ino_t,
    mut name: *const ::core::ffi::c_char,
    mut newparent: fuse_ino_t,
    mut newname: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_uint,
) {
    unsafe {
        let mut status: ::core::ffi::c_int = 0;
        let mut inode: uint32_t = 0;
        if !(parent == META_TRASH_INODE as fuse_ino_t
            || parent >= META_SUBTRASH_INODE_MIN as fuse_ino_t
                && parent <= META_SUBTRASH_INODE_MAX as fuse_ino_t)
            && newparent != META_UNDEL_INODE as fuse_ino_t
        {
            fuse_reply_err(req, EACCES);
            return;
        }
        inode = mfs_meta_name_to_inode(name);
        if inode == 0 as uint32_t {
            fuse_reply_err(req, ENOENT);
            return;
        }
        status = fs_undel(inode) as ::core::ffi::c_int;
        status = mfs_errorconv(status);
        fuse_reply_err(req, status);
    }
}
unsafe extern "C" fn dir_metaentries_size(mut ino: uint32_t) -> uint32_t {
    unsafe {
        match ino {
            1 => {
                return ((4 as ::core::ffi::c_int * 6 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as size_t)
                    .wrapping_add(strlen(META_TRASH_NAME.as_ptr()))
                    .wrapping_add(strlen(META_SUSTAINED_NAME.as_ptr()))
                    as uint32_t;
            }
            2147483640 => {
                if master_version()
                    >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            64 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            64 as ::core::ffi::c_int
                        })) as uint32_t
                    && flat_trash == 0 as ::core::ffi::c_int
                {
                    return (((3 as ::core::ffi::c_int + TRASH_BUCKETS) * 6 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int) as size_t)
                        .wrapping_add(strlen(META_UNDEL_NAME.as_ptr()))
                        .wrapping_add(
                            (TRASH_BUCKETS
                                * (if TRASH_BUCKETS <= 4096 as ::core::ffi::c_int {
                                    3 as ::core::ffi::c_int
                                } else {
                                    4 as ::core::ffi::c_int
                                })) as size_t,
                        ) as uint32_t;
                } else {
                    return ((3 as ::core::ffi::c_int * 6 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int) as size_t)
                        .wrapping_add(strlen(META_UNDEL_NAME.as_ptr()))
                        as uint32_t;
                }
            }
            2147483641 => {
                return (2 as ::core::ffi::c_int * 6 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as uint32_t;
            }
            2147483642 => {
                return (2 as ::core::ffi::c_int * 6 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as uint32_t;
            }
            _ => {
                if ino >= META_SUBTRASH_INODE_MIN as uint32_t
                    && ino <= META_SUBTRASH_INODE_MAX as uint32_t
                {
                    return ((3 as ::core::ffi::c_int * 6 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int) as size_t)
                        .wrapping_add(strlen(META_UNDEL_NAME.as_ptr()))
                        as uint32_t;
                }
            }
        }
        return 0 as uint32_t;
    }
}
unsafe extern "C" fn dir_metaentries_fill(mut buff: *mut uint8_t, mut ino: uint32_t) {
    unsafe {
        let mut l: uint8_t = 0;
        match ino {
            1 => {
                put8bit(&raw mut buff, 1 as uint8_t);
                put8bit(&raw mut buff, '.' as uint8_t);
                put32bit(&raw mut buff, META_ROOT_INODE as uint32_t);
                put8bit(&raw mut buff, TYPE_DIRECTORY as uint8_t);
                put8bit(&raw mut buff, 2 as uint8_t);
                put8bit(&raw mut buff, '.' as uint8_t);
                put8bit(&raw mut buff, '.' as uint8_t);
                put32bit(&raw mut buff, META_ROOT_INODE as uint32_t);
                put8bit(&raw mut buff, TYPE_DIRECTORY as uint8_t);
                l = strlen(META_TRASH_NAME.as_ptr()) as uint8_t;
                put8bit(&raw mut buff, l);
                memcpy(
                    buff as *mut ::core::ffi::c_void,
                    META_TRASH_NAME.as_ptr() as *const ::core::ffi::c_void,
                    l as size_t,
                );
                buff = buff.offset(l as ::core::ffi::c_int as isize);
                put32bit(&raw mut buff, META_TRASH_INODE as uint32_t);
                put8bit(&raw mut buff, TYPE_DIRECTORY as uint8_t);
                l = strlen(META_SUSTAINED_NAME.as_ptr()) as uint8_t;
                put8bit(&raw mut buff, l);
                memcpy(
                    buff as *mut ::core::ffi::c_void,
                    META_SUSTAINED_NAME.as_ptr() as *const ::core::ffi::c_void,
                    l as size_t,
                );
                buff = buff.offset(l as ::core::ffi::c_int as isize);
                put32bit(&raw mut buff, META_SUSTAINED_INODE as uint32_t);
                put8bit(&raw mut buff, TYPE_DIRECTORY as uint8_t);
                return;
            }
            2147483640 => {
                put8bit(&raw mut buff, 1 as uint8_t);
                put8bit(&raw mut buff, '.' as uint8_t);
                put32bit(&raw mut buff, META_TRASH_INODE as uint32_t);
                put8bit(&raw mut buff, TYPE_DIRECTORY as uint8_t);
                put8bit(&raw mut buff, 2 as uint8_t);
                put8bit(&raw mut buff, '.' as uint8_t);
                put8bit(&raw mut buff, '.' as uint8_t);
                put32bit(&raw mut buff, META_ROOT_INODE as uint32_t);
                put8bit(&raw mut buff, TYPE_DIRECTORY as uint8_t);
                l = strlen(META_UNDEL_NAME.as_ptr()) as uint8_t;
                put8bit(&raw mut buff, l);
                memcpy(
                    buff as *mut ::core::ffi::c_void,
                    META_UNDEL_NAME.as_ptr() as *const ::core::ffi::c_void,
                    l as size_t,
                );
                buff = buff.offset(l as ::core::ffi::c_int as isize);
                put32bit(&raw mut buff, META_UNDEL_INODE as uint32_t);
                put8bit(&raw mut buff, TYPE_DIRECTORY as uint8_t);
                if master_version()
                    >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            64 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            64 as ::core::ffi::c_int
                        })) as uint32_t
                    && flat_trash == 0 as ::core::ffi::c_int
                {
                    let mut tid: uint32_t = 0;
                    tid = 0 as uint32_t;
                    while tid < TRASH_BUCKETS as uint32_t {
                        if TRASH_BUCKETS > 4096 as ::core::ffi::c_int {
                            put8bit(&raw mut buff, 4 as uint8_t);
                            put8bit(
                                &raw mut buff,
                                ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(
                                    *b"0123456789ABCDEF\0",
                                )
                                    [(tid >> 12 as ::core::ffi::c_int & 15 as uint32_t) as usize]
                                    as uint8_t,
                            );
                        } else {
                            put8bit(&raw mut buff, 3 as uint8_t);
                        }
                        put8bit(
                            &raw mut buff,
                            ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(
                                *b"0123456789ABCDEF\0",
                            )
                                [(tid >> 8 as ::core::ffi::c_int & 15 as uint32_t) as usize]
                                as uint8_t,
                        );
                        put8bit(
                            &raw mut buff,
                            ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(
                                *b"0123456789ABCDEF\0",
                            )
                                [(tid >> 4 as ::core::ffi::c_int & 15 as uint32_t) as usize]
                                as uint8_t,
                        );
                        put8bit(
                            &raw mut buff,
                            ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(
                                *b"0123456789ABCDEF\0",
                            )[(tid & 15 as uint32_t) as usize]
                                as uint8_t,
                        );
                        put32bit(
                            &raw mut buff,
                            tid.wrapping_add(META_SUBTRASH_INODE_MIN as uint32_t),
                        );
                        put8bit(&raw mut buff, TYPE_DIRECTORY as uint8_t);
                        tid = tid.wrapping_add(1);
                    }
                }
                return;
            }
            2147483641 => {
                put8bit(&raw mut buff, 1 as uint8_t);
                put8bit(&raw mut buff, '.' as uint8_t);
                put32bit(&raw mut buff, META_UNDEL_INODE as uint32_t);
                put8bit(&raw mut buff, TYPE_DIRECTORY as uint8_t);
                put8bit(&raw mut buff, 2 as uint8_t);
                put8bit(&raw mut buff, '.' as uint8_t);
                put8bit(&raw mut buff, '.' as uint8_t);
                put32bit(&raw mut buff, META_TRASH_INODE as uint32_t);
                put8bit(&raw mut buff, TYPE_DIRECTORY as uint8_t);
                return;
            }
            2147483642 => {
                put8bit(&raw mut buff, 1 as uint8_t);
                put8bit(&raw mut buff, '.' as uint8_t);
                put32bit(&raw mut buff, META_SUSTAINED_INODE as uint32_t);
                put8bit(&raw mut buff, TYPE_DIRECTORY as uint8_t);
                put8bit(&raw mut buff, 2 as uint8_t);
                put8bit(&raw mut buff, '.' as uint8_t);
                put8bit(&raw mut buff, '.' as uint8_t);
                put32bit(&raw mut buff, META_ROOT_INODE as uint32_t);
                put8bit(&raw mut buff, TYPE_DIRECTORY as uint8_t);
                return;
            }
            _ => {
                if ino >= META_SUBTRASH_INODE_MIN as uint32_t
                    && ino <= META_SUBTRASH_INODE_MAX as uint32_t
                {
                    put8bit(&raw mut buff, 1 as uint8_t);
                    put8bit(&raw mut buff, '.' as uint8_t);
                    put32bit(&raw mut buff, META_TRASH_INODE as uint32_t);
                    put8bit(&raw mut buff, TYPE_DIRECTORY as uint8_t);
                    put8bit(&raw mut buff, 2 as uint8_t);
                    put8bit(&raw mut buff, '.' as uint8_t);
                    put8bit(&raw mut buff, '.' as uint8_t);
                    put32bit(&raw mut buff, META_ROOT_INODE as uint32_t);
                    put8bit(&raw mut buff, TYPE_DIRECTORY as uint8_t);
                    l = strlen(META_UNDEL_NAME.as_ptr()) as uint8_t;
                    put8bit(&raw mut buff, l);
                    memcpy(
                        buff as *mut ::core::ffi::c_void,
                        META_UNDEL_NAME.as_ptr() as *const ::core::ffi::c_void,
                        l as size_t,
                    );
                    buff = buff.offset(l as ::core::ffi::c_int as isize);
                    put32bit(&raw mut buff, META_UNDEL_INODE as uint32_t);
                    put8bit(&raw mut buff, TYPE_DIRECTORY as uint8_t);
                    return;
                }
            }
        };
    }
}
unsafe extern "C" fn dir_dataentries_size(
    mut dbuff: *const uint8_t,
    mut dsize: uint32_t,
) -> uint32_t {
    unsafe {
        let mut nleng: uint8_t = 0;
        let mut eleng: uint32_t = 0;
        let mut eptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        eleng = 0 as uint32_t;
        if dbuff.is_null() || dsize == 0 as uint32_t {
            return 0 as uint32_t;
        }
        eptr = dbuff.offset(dsize as isize);
        while dbuff < eptr {
            nleng = *dbuff.offset(0 as isize);
            dbuff = dbuff.offset((5 as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as isize);
            if nleng as ::core::ffi::c_int > 255 as ::core::ffi::c_int - 9 as ::core::ffi::c_int {
                eleng = eleng.wrapping_add(
                    (6 as ::core::ffi::c_int + 255 as ::core::ffi::c_int) as uint32_t,
                );
            } else {
                eleng = eleng.wrapping_add(
                    (6 as ::core::ffi::c_int
                        + nleng as ::core::ffi::c_int
                        + 9 as ::core::ffi::c_int) as uint32_t,
                );
            }
        }
        return eleng;
    }
}
unsafe extern "C" fn dir_hexgen(mut buff: *mut uint8_t, mut hex: uint32_t) {
    unsafe {
        let mut i: uint8_t = 0;
        i = 0 as uint8_t;
        while (i as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
            *buff.offset((7 as ::core::ffi::c_int - i as ::core::ffi::c_int) as isize) =
                ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(
                    *b"0123456789ABCDEF\0",
                )[(hex & 0xf as uint32_t) as usize] as uint8_t;
            hex >>= 4 as ::core::ffi::c_int;
            i = i.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn dir_dataentries_convert(
    mut buff: *mut uint8_t,
    mut dbuff: *const uint8_t,
    mut dsize: uint32_t,
) {
    unsafe {
        let mut name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut inode: uint32_t = 0;
        let mut nleng: uint8_t = 0;
        let mut inoleng: uint8_t = 0;
        let mut eptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        eptr = dbuff.offset(dsize as isize);
        while dbuff < eptr {
            nleng = *dbuff.offset(0 as isize);
            if dbuff
                .offset(nleng as ::core::ffi::c_int as isize)
                .offset(5 as ::core::ffi::c_int as isize)
                <= eptr
            {
                dbuff = dbuff.offset(1);
                if nleng as ::core::ffi::c_int > 255 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                {
                    inoleng = 255 as uint8_t;
                } else {
                    inoleng = (nleng as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as uint8_t;
                }
                put8bit(&raw mut buff, inoleng);
                name = dbuff as *const ::core::ffi::c_char;
                dbuff = dbuff.offset(nleng as ::core::ffi::c_int as isize);
                inode = get32bit(&raw mut dbuff);
                dir_hexgen(buff, inode);
                *buff.offset(8 as isize) = '|' as uint8_t;
                if nleng as ::core::ffi::c_int > 255 as ::core::ffi::c_int - 9 as ::core::ffi::c_int
                {
                    memcpy(
                        buff.offset(9 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        (255 as ::core::ffi::c_int - 9 as ::core::ffi::c_int) as size_t,
                    );
                    buff = buff.offset(255 as ::core::ffi::c_int as isize);
                } else {
                    memcpy(
                        buff.offset(9 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        nleng as size_t,
                    );
                    buff = buff
                        .offset((9 as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as isize);
                }
                put32bit(&raw mut buff, inode);
                put8bit(&raw mut buff, TYPE_FILE as uint8_t);
            } else {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"dir data malformed (trash)\0".as_ptr() as *const ::core::ffi::c_char,
                );
                dbuff = eptr;
            }
        }
    }
}
unsafe extern "C" fn dirbuf_meta_fill(mut b: *mut dirbuf, mut ino: uint32_t) {
    unsafe {
        let mut status: ::core::ffi::c_int = 0;
        let mut msize: uint32_t = 0;
        let mut dcsize: uint32_t = 0;
        let mut dbuff: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut dsize: uint32_t = 0;
        (*b).p = ::core::ptr::null_mut::<uint8_t>();
        (*b).size = 0 as size_t;
        msize = dir_metaentries_size(ino);
        dbuff = ::core::ptr::null::<uint8_t>();
        dsize = 0 as uint32_t;
        if ino == META_TRASH_INODE as uint32_t
            && (master_version()
                < (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        64 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        64 as ::core::ffi::c_int
                    })) as uint32_t
                || flat_trash != 0)
        {
            status = fs_gettrash(0xffffffff as uint32_t, &raw mut dbuff, &raw mut dsize)
                as ::core::ffi::c_int;
            if status != MFS_STATUS_OK {
                return;
            }
            dcsize = dir_dataentries_size(dbuff, dsize);
        } else if ino == META_SUSTAINED_INODE as uint32_t {
            status = fs_getsustained(&raw mut dbuff, &raw mut dsize) as ::core::ffi::c_int;
            if status != MFS_STATUS_OK {
                return;
            }
            dcsize = dir_dataentries_size(dbuff, dsize);
        } else if ino >= META_SUBTRASH_INODE_MIN as uint32_t
            && ino <= META_SUBTRASH_INODE_MAX as uint32_t
            && master_version()
                >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        64 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        64 as ::core::ffi::c_int
                    })) as uint32_t
            && flat_trash == 0 as ::core::ffi::c_int
        {
            status = fs_gettrash(
                ino.wrapping_sub(META_SUBTRASH_INODE_MIN as uint32_t),
                &raw mut dbuff,
                &raw mut dsize,
            ) as ::core::ffi::c_int;
            if status != MFS_STATUS_OK {
                return;
            }
            dcsize = dir_dataentries_size(dbuff, dsize);
        } else {
            dcsize = 0 as uint32_t;
        }
        if msize.wrapping_add(dcsize) == 0 as uint32_t {
            return;
        }
        (*b).p = malloc(msize.wrapping_add(dcsize) as size_t) as *mut uint8_t;
        if (*b).p.is_null() {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        if msize > 0 as uint32_t {
            dir_metaentries_fill((*b).p, ino);
        }
        if dcsize > 0 as uint32_t {
            dir_dataentries_convert((*b).p.offset(msize as isize), dbuff, dsize);
        }
        (*b).size = msize.wrapping_add(dcsize) as size_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_opendir(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut fi: *mut fuse_file_info,
) {
    unsafe {
        let mut dirinfo: *mut dirbuf = ::core::ptr::null_mut::<dirbuf>();
        if ino == META_ROOT_INODE as fuse_ino_t
            || ino == META_TRASH_INODE as fuse_ino_t
            || ino == META_UNDEL_INODE as fuse_ino_t
            || ino == META_SUSTAINED_INODE as fuse_ino_t
            || ino >= META_SUBTRASH_INODE_MIN as fuse_ino_t
                && ino <= META_SUBTRASH_INODE_MAX as fuse_ino_t
        {
            dirinfo = malloc(::core::mem::size_of::<dirbuf>()) as *mut dirbuf;
            pthread_mutex_init(
                &raw mut (*dirinfo).lock,
                ::core::ptr::null::<pthread_mutexattr_t>(),
            );
            (*dirinfo).p = ::core::ptr::null_mut::<uint8_t>();
            (*dirinfo).size = 0 as size_t;
            (*dirinfo).wasread = 0 as ::core::ffi::c_int;
            (*fi).fh = dirinfo.expose_provenance() as ::core::ffi::c_ulong as uint64_t;
            if fuse_reply_open(req, fi) == -ENOENT {
                (*fi).fh = 0 as uint64_t;
                pthread_mutex_destroy(&raw mut (*dirinfo).lock);
                free((*dirinfo).p as *mut ::core::ffi::c_void);
                free(dirinfo as *mut ::core::ffi::c_void);
            }
        } else {
            fuse_reply_err(req, ENOTDIR);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_readdir(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut size: size_t,
    mut off: off_t,
    mut fi: *mut fuse_file_info,
) {
    unsafe {
        let mut dirinfo: *mut dirbuf = ::core::ptr::with_exposed_provenance_mut::<dirbuf>(
            (*fi).fh as ::core::ffi::c_ulong as usize,
        );
        let mut buffer: [::core::ffi::c_char; 50000] = [0; 50000];
        let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut c: ::core::ffi::c_char = 0;
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut eptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut end: uint8_t = 0;
        let mut opos: size_t = 0;
        let mut oleng: size_t = 0;
        let mut nleng: uint8_t = 0;
        let mut inode: uint32_t = 0;
        let mut r#type: uint8_t = 0;
        let mut stbuf: stat = stat {
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
        if off < 0 as off_t {
            fuse_reply_err(req, EINVAL);
            return;
        }
        pthread_mutex_lock(&raw mut (*dirinfo).lock);
        if (*dirinfo).wasread == 0 as ::core::ffi::c_int
            || (*dirinfo).wasread == 1 as ::core::ffi::c_int && off == 0 as off_t
        {
            if !(*dirinfo).p.is_null() {
                free((*dirinfo).p as *mut ::core::ffi::c_void);
            }
            dirbuf_meta_fill(dirinfo, ino as uint32_t);
        }
        (*dirinfo).wasread = 1 as ::core::ffi::c_int;
        if off >= (*dirinfo).size as off_t {
            fuse_reply_buf(req, ::core::ptr::null::<::core::ffi::c_char>(), 0 as size_t);
        } else {
            if size > READDIR_BUFFSIZE as size_t {
                size = READDIR_BUFFSIZE as size_t;
            }
            ptr = ((*dirinfo).p as *const uint8_t).offset(off as isize);
            eptr = ((*dirinfo).p as *const uint8_t).offset((*dirinfo).size as isize);
            opos = 0 as size_t;
            end = 0 as uint8_t;
            while ptr < eptr && end as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                nleng = *ptr.offset(0 as isize);
                ptr = ptr.offset(1);
                name = ptr as *mut ::core::ffi::c_char;
                ptr = ptr.offset(nleng as ::core::ffi::c_int as isize);
                off += (nleng as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as off_t;
                if ptr.offset(5 as ::core::ffi::c_int as isize) <= eptr {
                    inode = get32bit(&raw mut ptr);
                    r#type = get8bit(&raw mut ptr);
                    mfs_meta_type_to_stat(inode, r#type, &raw mut stbuf);
                    c = *name.offset(nleng as isize);
                    *name.offset(nleng as isize) = 0 as ::core::ffi::c_char;
                    oleng = fuse_add_direntry(
                        req,
                        (&raw mut buffer as *mut ::core::ffi::c_char).offset(opos as isize),
                        size.wrapping_sub(opos),
                        name,
                        &raw mut stbuf,
                        off,
                    );
                    *name.offset(nleng as isize) = c;
                    if opos.wrapping_add(oleng) > size {
                        end = 1 as uint8_t;
                    } else {
                        opos = opos.wrapping_add(oleng);
                    }
                }
            }
            fuse_reply_buf(req, &raw mut buffer as *mut ::core::ffi::c_char, opos);
        }
        pthread_mutex_unlock(&raw mut (*dirinfo).lock);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_releasedir(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut fi: *mut fuse_file_info,
) {
    unsafe {
        let mut dirinfo: *mut dirbuf = ::core::ptr::with_exposed_provenance_mut::<dirbuf>(
            (*fi).fh as ::core::ffi::c_ulong as usize,
        );
        pthread_mutex_lock(&raw mut (*dirinfo).lock);
        pthread_mutex_unlock(&raw mut (*dirinfo).lock);
        pthread_mutex_destroy(&raw mut (*dirinfo).lock);
        free((*dirinfo).p as *mut ::core::ffi::c_void);
        free(dirinfo as *mut ::core::ffi::c_void);
        (*fi).fh = 0 as uint64_t;
        fuse_reply_err(req, 0 as ::core::ffi::c_int);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_open(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut fi: *mut fuse_file_info,
) {
    unsafe {
        let mut pathinfo: *mut pathbuf = ::core::ptr::null_mut::<pathbuf>();
        let mut path: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut status: ::core::ffi::c_int = 0;
        if ino == MASTERINFO_INODE as fuse_ino_t {
            (*fi).fh = 0 as uint64_t;
            (*fi).set_direct_io(0 as uint32_t as uint32_t);
            (*fi).set_keep_cache(1 as uint32_t as uint32_t);
            fuse_reply_open(req, fi);
            return;
        }
        if ino >= MIN_SPECIAL_INODE as fuse_ino_t || ino == META_ROOT_INODE as fuse_ino_t {
            fuse_reply_err(req, EACCES);
        } else {
            status = fs_gettrashpath(ino as uint32_t, &raw mut path) as ::core::ffi::c_int;
            status = mfs_errorconv(status);
            if status != 0 as ::core::ffi::c_int {
                fuse_reply_err(req, status);
            } else {
                pathinfo = malloc(::core::mem::size_of::<pathbuf>()) as *mut pathbuf;
                pthread_mutex_init(
                    &raw mut (*pathinfo).lock,
                    ::core::ptr::null::<pthread_mutexattr_t>(),
                );
                (*pathinfo).changed = 0 as ::core::ffi::c_int;
                (*pathinfo).size =
                    strlen(path as *mut ::core::ffi::c_char).wrapping_add(1 as size_t);
                (*pathinfo).p = malloc((*pathinfo).size) as *mut ::core::ffi::c_char;
                memcpy(
                    (*pathinfo).p as *mut ::core::ffi::c_void,
                    path as *const ::core::ffi::c_void,
                    (*pathinfo).size.wrapping_sub(1 as size_t),
                );
                *(*pathinfo)
                    .p
                    .offset((*pathinfo).size.wrapping_sub(1 as size_t) as isize) =
                    '\n' as ::core::ffi::c_char;
                (*fi).set_direct_io(1 as uint32_t as uint32_t);
                (*fi).fh = pathinfo.expose_provenance() as ::core::ffi::c_ulong as uint64_t;
                if fuse_reply_open(req, fi) == -ENOENT {
                    (*fi).fh = 0 as uint64_t;
                    pthread_mutex_destroy(&raw mut (*pathinfo).lock);
                    free((*pathinfo).p as *mut ::core::ffi::c_void);
                    free(pathinfo as *mut ::core::ffi::c_void);
                }
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_release(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut fi: *mut fuse_file_info,
) {
    unsafe {
        if ino == MASTERINFO_INODE as fuse_ino_t {
            fuse_reply_err(req, 0 as ::core::ffi::c_int);
            return;
        }
        let mut pathinfo: *mut pathbuf = ::core::ptr::with_exposed_provenance_mut::<pathbuf>(
            (*fi).fh as ::core::ffi::c_ulong as usize,
        );
        pthread_mutex_lock(&raw mut (*pathinfo).lock);
        if (*pathinfo).changed != 0 {
            if *(*pathinfo)
                .p
                .offset((*pathinfo).size.wrapping_sub(1 as size_t) as isize)
                as ::core::ffi::c_int
                == '\n' as ::core::ffi::c_int
            {
                *(*pathinfo)
                    .p
                    .offset((*pathinfo).size.wrapping_sub(1 as size_t) as isize) =
                    0 as ::core::ffi::c_char;
            } else {
                (*pathinfo).p = realloc(
                    (*pathinfo).p as *mut ::core::ffi::c_void,
                    (*pathinfo).size.wrapping_add(1 as size_t),
                ) as *mut ::core::ffi::c_char;
                *(*pathinfo).p.offset((*pathinfo).size as isize) = 0 as ::core::ffi::c_char;
            }
            fs_settrashpath(ino as uint32_t, (*pathinfo).p as *mut uint8_t);
        }
        pthread_mutex_unlock(&raw mut (*pathinfo).lock);
        pthread_mutex_destroy(&raw mut (*pathinfo).lock);
        free((*pathinfo).p as *mut ::core::ffi::c_void);
        free(pathinfo as *mut ::core::ffi::c_void);
        (*fi).fh = 0 as uint64_t;
        fuse_reply_err(req, 0 as ::core::ffi::c_int);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_read(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut size: size_t,
    mut off: off_t,
    mut fi: *mut fuse_file_info,
) {
    unsafe {
        let mut pathinfo: *mut pathbuf = ::core::ptr::with_exposed_provenance_mut::<pathbuf>(
            (*fi).fh as ::core::ffi::c_ulong as usize,
        );
        if ino == MASTERINFO_INODE as fuse_ino_t {
            let mut masterinfo: [uint8_t; 22] = [0; 22];
            fs_getmasterlocation(&raw mut masterinfo as *mut uint8_t);
            masterproxy_getlocation(&raw mut masterinfo as *mut uint8_t);
            if off >= 22 as off_t {
                fuse_reply_buf(req, ::core::ptr::null::<::core::ffi::c_char>(), 0 as size_t);
            } else if (off as size_t).wrapping_add(size) > 22 as size_t {
                fuse_reply_buf(
                    req,
                    (&raw mut masterinfo as *mut uint8_t).offset(off as isize)
                        as *mut ::core::ffi::c_char,
                    (22 as off_t - off) as size_t,
                );
            } else {
                fuse_reply_buf(
                    req,
                    (&raw mut masterinfo as *mut uint8_t).offset(off as isize)
                        as *mut ::core::ffi::c_char,
                    size,
                );
            }
            return;
        }
        if pathinfo.is_null() {
            fuse_reply_err(req, EBADF);
            return;
        }
        pthread_mutex_lock(&raw mut (*pathinfo).lock);
        if off < 0 as off_t {
            pthread_mutex_unlock(&raw mut (*pathinfo).lock);
            fuse_reply_err(req, EINVAL);
            return;
        }
        if off as size_t > (*pathinfo).size {
            fuse_reply_buf(req, ::core::ptr::null::<::core::ffi::c_char>(), 0 as size_t);
        } else if (off as size_t).wrapping_add(size) > (*pathinfo).size {
            fuse_reply_buf(
                req,
                (*pathinfo).p.offset(off as isize),
                (*pathinfo).size.wrapping_sub(off as size_t),
            );
        } else {
            fuse_reply_buf(req, (*pathinfo).p.offset(off as isize), size);
        }
        pthread_mutex_unlock(&raw mut (*pathinfo).lock);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_write(
    mut req: fuse_req_t,
    mut ino: fuse_ino_t,
    mut buf: *const ::core::ffi::c_char,
    mut size: size_t,
    mut off: off_t,
    mut fi: *mut fuse_file_info,
) {
    unsafe {
        let mut pathinfo: *mut pathbuf = ::core::ptr::with_exposed_provenance_mut::<pathbuf>(
            (*fi).fh as ::core::ffi::c_ulong as usize,
        );
        if ino == MASTERINFO_INODE as fuse_ino_t {
            fuse_reply_err(req, EACCES);
            return;
        }
        if pathinfo.is_null() {
            fuse_reply_err(req, EBADF);
            return;
        }
        if (off as size_t).wrapping_add(size) > PATH_SIZE_LIMIT as size_t {
            fuse_reply_err(req, EINVAL);
            return;
        }
        pthread_mutex_lock(&raw mut (*pathinfo).lock);
        if (*pathinfo).changed == 0 as ::core::ffi::c_int {
            (*pathinfo).size = 0 as size_t;
        }
        if (off as size_t).wrapping_add(size) > (*pathinfo).size {
            let mut s: size_t = (*pathinfo).size;
            (*pathinfo).p = realloc(
                (*pathinfo).p as *mut ::core::ffi::c_void,
                (off as size_t).wrapping_add(size),
            ) as *mut ::core::ffi::c_char;
            (*pathinfo).size = (off as size_t).wrapping_add(size);
            memset(
                (*pathinfo).p.offset(s as isize) as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (off as size_t).wrapping_add(size).wrapping_sub(s),
            );
        }
        memcpy(
            (*pathinfo).p.offset(off as isize) as *mut ::core::ffi::c_void,
            buf as *const ::core::ffi::c_void,
            size,
        );
        (*pathinfo).changed = 1 as ::core::ffi::c_int;
        pthread_mutex_unlock(&raw mut (*pathinfo).lock);
        fuse_reply_write(req, size);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_init(
    mut debug_mode_in: ::core::ffi::c_int,
    mut entry_cache_timeout_in: ::core::ffi::c_double,
    mut attr_cache_timeout_in: ::core::ffi::c_double,
    mut flat_trash_in: ::core::ffi::c_int,
) {
    unsafe {
        debug_mode = debug_mode_in;
        entry_cache_timeout = entry_cache_timeout_in;
        attr_cache_timeout = attr_cache_timeout_in;
        flat_trash = flat_trash_in;
        if debug_mode != 0 {
            fprintf(
                stderr,
                b"cache parameters: entry_cache_timeout=%.2lf attr_cache_timeout=%.2lf\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                entry_cache_timeout,
                attr_cache_timeout,
            );
            if flat_trash != 0 {
                fprintf(
                    stderr,
                    b"force using 'flat' trash\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
        }
    }
}
