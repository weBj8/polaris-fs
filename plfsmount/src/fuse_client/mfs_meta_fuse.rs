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
// ---------------------------------------------------------------------------
// Safe core (P4 rewrite): all pure conversion/serialization logic of the
// meta filesystem. FUSE reply calls and fs_* master traffic stay at the
// boundary below.
// ---------------------------------------------------------------------------

#[deny(unsafe_code)]
pub mod imp {
    pub const MIN_SPECIAL_INODE: u32 = 0x7FFF0000;
    pub const META_ROOT_INODE: u32 = 1;
    pub const META_TRASH_INODE: u32 = 0x7ffffff8;
    pub const META_UNDEL_INODE: u32 = 0x7ffffff9;
    pub const META_SUSTAINED_INODE: u32 = 0x7ffffffa;
    pub const MASTERINFO_INODE: u32 = 0x7fffffff;
    pub const META_SUBTRASH_INODE_MIN: u32 = 0x7fff0000;
    pub const TRASH_BUCKETS: u32 = 4096;
    pub const META_SUBTRASH_INODE_MAX: u32 = META_SUBTRASH_INODE_MIN + TRASH_BUCKETS - 1;
    pub const META_TRASH_NAME: &[u8] = b"trash";
    pub const META_UNDEL_NAME: &[u8] = b"undel";
    pub const META_SUSTAINED_NAME: &[u8] = b"sustained";
    pub const MASTERINFO_NAME: &[u8] = b".masterinfo";
    pub const VERSION_3_0_64: u32 = 3_000_064;
    pub const MFSBLOCKSIZE: u32 = 0x10000;

    pub const TYPE_FILE: u8 = 1;
    pub const TYPE_DIRECTORY: u8 = 2;
    pub const TYPE_SYMLINK: u8 = 3;
    pub const TYPE_FIFO: u8 = 4;
    pub const TYPE_BLOCKDEV: u8 = 5;
    pub const TYPE_CHARDEV: u8 = 6;
    pub const TYPE_SOCKET: u8 = 7;
    pub const TYPE_TRASH: u8 = 8;
    pub const TYPE_SUSTAINED: u8 = 9;

    pub const DISP_TYPE_DIRECTORY: u8 = 100; // 'd'
    pub const DISP_TYPE_FILE: u8 = 102; // 'f'
    pub const DISP_TYPE_SYMLINK: u8 = 108; // 'l'
    pub const DISP_TYPE_FIFO: u8 = 113; // 'q'
    pub const DISP_TYPE_BLOCKDEV: u8 = 98; // 'b'
    pub const DISP_TYPE_CHARDEV: u8 = 99; // 'c'
    pub const DISP_TYPE_SOCKET: u8 = 115; // 's'
    pub const DISP_TYPE_TRASH: u8 = 116; // 't'
    pub const DISP_TYPE_SUSTAINED: u8 = 114; // 'r'

    pub const S_IFIFO: u32 = 0o10000;
    pub const S_IFCHR: u32 = 0o20000;
    pub const S_IFDIR: u32 = 0o40000;
    pub const S_IFBLK: u32 = 0o60000;
    pub const S_IFREG: u32 = 0o100000;
    pub const S_IFLNK: u32 = 0o120000;
    pub const S_IFSOCK: u32 = 0o140000;

    pub fn is_special_inode(ino: u32) -> bool {
        ino >= MIN_SPECIAL_INODE || ino == META_ROOT_INODE
    }

    /// "HEX|rest" → inode; 0 when not in meta-name form (C: strtoul hex,
    /// then '|' with a non-NUL follower required)
    pub fn name_to_inode(name: &[u8]) -> u32 {
        let mut inode: u32 = 0;
        let mut i = 0usize;
        let mut any = false;
        while i < name.len() {
            let d = match name[i] {
                b'0'..=b'9' => (name[i] - b'0') as u32,
                b'a'..=b'f' => (name[i] - b'a') as u32 + 10,
                b'A'..=b'F' => (name[i] - b'A') as u32 + 10,
                _ => break,
            };
            // strtoul would wrap on overflow; names here are short
            inode = inode.wrapping_mul(16).wrapping_add(d);
            any = true;
            i += 1;
        }
        if any && i < name.len() && name[i] == b'|' && i + 1 < name.len() && name[i + 1] != 0 {
            inode
        } else {
            0
        }
    }

    /// MFS status → errno (mfs_errorconv)
    pub fn errorconv(status: i32) -> i32 {
        match status {
            0 => 0,    // MFS_STATUS_OK
            1 => 1,    // EPERM
            2 => 20,   // ENOTDIR
            3 => 2,    // ENOENT
            4 => 13,   // EACCES
            5 => 17,   // EEXIST
            6 => 22,   // EINVAL
            7 => 39,   // ENOTEMPTY
            8 => 5,    // EIO (MFS_ERROR_IO)
            33 => 30,  // EROFS
            40 => 122, // EDQUOT (MFS_ERROR_QUOTA)
            _ => 22,   // EINVAL
        }
    }

    /// DISP_TYPE_* → TYPE_* (fsnodes_type_convert)
    pub fn fsnodes_type_convert(t: u8) -> u8 {
        match t {
            DISP_TYPE_FILE => TYPE_FILE,
            DISP_TYPE_DIRECTORY => TYPE_DIRECTORY,
            DISP_TYPE_SYMLINK => TYPE_SYMLINK,
            DISP_TYPE_FIFO => TYPE_FIFO,
            DISP_TYPE_BLOCKDEV => TYPE_BLOCKDEV,
            DISP_TYPE_CHARDEV => TYPE_CHARDEV,
            DISP_TYPE_SOCKET => TYPE_SOCKET,
            DISP_TYPE_TRASH => TYPE_TRASH,
            DISP_TYPE_SUSTAINED => TYPE_SUSTAINED,
            _ => 0,
        }
    }

    /// st_mode for a meta entry type byte (mfs_meta_type_to_stat)
    pub fn type_to_mode(t: u8) -> u32 {
        match t & 0x7F {
            DISP_TYPE_DIRECTORY | TYPE_DIRECTORY => S_IFDIR,
            DISP_TYPE_SYMLINK | TYPE_SYMLINK => S_IFLNK,
            DISP_TYPE_FILE | TYPE_FILE => S_IFREG,
            DISP_TYPE_FIFO | TYPE_FIFO => S_IFIFO,
            DISP_TYPE_SOCKET | TYPE_SOCKET => S_IFSOCK,
            DISP_TYPE_BLOCKDEV | TYPE_BLOCKDEV => S_IFBLK,
            DISP_TYPE_CHARDEV | TYPE_CHARDEV => S_IFCHR,
            _ => 0,
        }
    }

    /// stat fields the FUSE boundary needs (struct stat stays C-side)
    #[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
    pub struct StatFields {
        pub ino: u32,
        pub mode: u32,
        pub size: u64,
        pub blocks: u64,
        pub blksize: u32,
        pub uid: u32,
        pub gid: u32,
        pub atime: u32,
        pub mtime: u32,
        pub ctime: u32,
        pub nlink: u32,
    }

    fn get16(b: &[u8]) -> u16 {
        u16::from_be_bytes([b[0], b[1]])
    }
    fn get32(b: &[u8]) -> u32 {
        u32::from_be_bytes([b[0], b[1], b[2], b[3]])
    }
    fn get64(b: &[u8]) -> u64 {
        u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
    }

    /// mfs_attr_to_stat: parse a 36-byte attr record into stat fields.
    /// Layout (1.7.29+, attr[0]<64): [flags:1][mode+type:2BE][uid:4][gid:4]
    /// [atime:4][mtime:4][ctime:4][nlink:4][length:8]
    /// Legacy (attr[0]>=64): [disptype:1][mode:2BE] then same tail.
    pub fn attr_to_stat(inode: u32, attr: &[u8; 36]) -> StatFields {
        let (attrmode, attrtype);
        if attr[0] < 64 {
            let m = get16(&attr[1..]);
            attrmode = m & 0x0FFF;
            attrtype = (m >> 12) as u8;
        } else {
            attrtype = fsnodes_type_convert(attr[0] & 0x7F);
            attrmode = get16(&attr[1..]) & 0x0FFF;
        }
        let uid = get32(&attr[3..]);
        let gid = get32(&attr[7..]);
        let atime = get32(&attr[11..]);
        let mtime = get32(&attr[15..]);
        let ctime = get32(&attr[19..]);
        let nlink = get32(&attr[23..]);
        let length = get64(&attr[27..]);
        let mode = if attrtype == TYPE_FILE || attrtype == TYPE_TRASH || attrtype == TYPE_SUSTAINED
        {
            S_IFREG | (attrmode as u32 & 0o7777)
        } else {
            0
        };
        StatFields {
            ino: inode,
            mode,
            size: length,
            blocks: (length + 511) / 512,
            blksize: MFSBLOCKSIZE,
            uid,
            gid,
            atime,
            mtime,
            ctime,
            nlink,
        }
    }

    /// mfs_meta_stat: fixed stats for the virtual meta directories
    pub fn meta_stat(inode: u32, now: u32) -> StatFields {
        let (nlink, mode) = match inode {
            META_ROOT_INODE => (4, S_IFDIR | 0o555),
            META_TRASH_INODE => (3 + TRASH_BUCKETS, S_IFDIR | 0o700),
            META_UNDEL_INODE => (2 + TRASH_BUCKETS, S_IFDIR | 0o200),
            META_SUSTAINED_INODE => (2, S_IFDIR | 0o500),
            META_SUBTRASH_INODE_MIN..=META_SUBTRASH_INODE_MAX => (3, S_IFDIR | 0o700),
            _ => (0, 0),
        };
        StatFields {
            ino: inode,
            mode,
            size: 0,
            blocks: 0,
            blksize: MFSBLOCKSIZE,
            uid: 0,
            gid: 0,
            atime: now,
            mtime: now,
            ctime: now,
            nlink,
        }
    }

    /// size of the meta (static) entries blob for a directory inode
    pub fn dir_metaentries_size(ino: u32, master_ge_3064: bool, flat_trash: bool) -> u32 {
        match ino {
            META_ROOT_INODE => {
                (4 * 6 + 1 + 2 + META_TRASH_NAME.len() + META_SUSTAINED_NAME.len()) as u32
            }
            META_TRASH_INODE => {
                if master_ge_3064 && !flat_trash {
                    ((3 + TRASH_BUCKETS) * 6
                        + 1
                        + 2
                        + META_UNDEL_NAME.len() as u32
                        + TRASH_BUCKETS * if TRASH_BUCKETS <= 4096 { 3 } else { 4 })
                        as u32
                } else {
                    (3 * 6 + 1 + 2 + META_UNDEL_NAME.len()) as u32
                }
            }
            META_UNDEL_INODE | META_SUSTAINED_INODE => 2 * 6 + 1 + 2,
            META_SUBTRASH_INODE_MIN..=META_SUBTRASH_INODE_MAX => {
                (3 * 6 + 1 + 2 + META_UNDEL_NAME.len()) as u32
            }
            _ => 0,
        }
    }

    fn put_entry(out: &mut Vec<u8>, name: &[u8], ino: u32, type_: u8) {
        out.push(name.len() as u8);
        out.extend_from_slice(name);
        out.extend_from_slice(&ino.to_be_bytes());
        out.push(type_);
    }

    /// the static entries blob for a meta directory (exact C byte layout:
    /// [nleng][name][inode:4BE][type:1] per entry)
    pub fn dir_metaentries_fill(ino: u32, master_ge_3064: bool, flat_trash: bool) -> Vec<u8> {
        let mut out = Vec::new();
        match ino {
            META_ROOT_INODE => {
                put_entry(&mut out, b".", META_ROOT_INODE, TYPE_DIRECTORY);
                put_entry(&mut out, b"..", META_ROOT_INODE, TYPE_DIRECTORY);
                put_entry(&mut out, META_TRASH_NAME, META_TRASH_INODE, TYPE_DIRECTORY);
                put_entry(
                    &mut out,
                    META_SUSTAINED_NAME,
                    META_SUSTAINED_INODE,
                    TYPE_DIRECTORY,
                );
            }
            META_TRASH_INODE => {
                put_entry(&mut out, b".", META_TRASH_INODE, TYPE_DIRECTORY);
                put_entry(&mut out, b"..", META_ROOT_INODE, TYPE_DIRECTORY);
                put_entry(&mut out, META_UNDEL_NAME, META_UNDEL_INODE, TYPE_DIRECTORY);
                if master_ge_3064 && !flat_trash {
                    const HEX: &[u8; 16] = b"0123456789ABCDEF";
                    for tid in 0..TRASH_BUCKETS {
                        let mut name = Vec::with_capacity(4);
                        if TRASH_BUCKETS > 4096 {
                            name.push(HEX[((tid >> 12) & 15) as usize]);
                        }
                        name.push(HEX[((tid >> 8) & 15) as usize]);
                        name.push(HEX[((tid >> 4) & 15) as usize]);
                        name.push(HEX[(tid & 15) as usize]);
                        put_entry(
                            &mut out,
                            &name,
                            META_SUBTRASH_INODE_MIN + tid,
                            TYPE_DIRECTORY,
                        );
                    }
                }
            }
            META_UNDEL_INODE => {
                put_entry(&mut out, b".", META_UNDEL_INODE, TYPE_DIRECTORY);
                put_entry(&mut out, b"..", META_TRASH_INODE, TYPE_DIRECTORY);
            }
            META_SUSTAINED_INODE => {
                put_entry(&mut out, b".", META_SUSTAINED_INODE, TYPE_DIRECTORY);
                put_entry(&mut out, b"..", META_ROOT_INODE, TYPE_DIRECTORY);
            }
            META_SUBTRASH_INODE_MIN..=META_SUBTRASH_INODE_MAX => {
                put_entry(&mut out, b".", META_TRASH_INODE, TYPE_DIRECTORY);
                put_entry(&mut out, b"..", META_ROOT_INODE, TYPE_DIRECTORY);
                put_entry(&mut out, META_UNDEL_NAME, META_UNDEL_INODE, TYPE_DIRECTORY);
            }
            _ => {}
        }
        out
    }

    /// size of the converted data-entries blob (master's dbuff → meta dir
    /// entries with "HEX|name" filenames)
    pub fn dir_dataentries_size(dbuff: &[u8]) -> u32 {
        let mut eleng = 0u32;
        let mut pos = 0usize;
        while pos < dbuff.len() {
            let nleng = dbuff[pos] as usize;
            pos += 5 + nleng;
            if nleng > 255 - 9 {
                eleng += 6 + 255;
            } else {
                eleng += (6 + nleng + 9) as u32;
            }
        }
        eleng
    }

    /// 8 uppercase hex digits, big-endian nibble order
    pub fn dir_hexgen(out: &mut [u8; 8], hex: u32) {
        const HEX: &[u8; 16] = b"0123456789ABCDEF";
        for i in 0..8 {
            out[7 - i] = HEX[(hex >> (4 * i)) as usize & 15];
        }
    }

    /// dbuff → meta entries; long names truncated to 255-9 after the
    /// "HEX|" prefix. Second return = malformed-data flag (C logged and
    /// stopped the walk).
    pub fn dir_dataentries_convert(dbuff: &[u8]) -> (Vec<u8>, bool) {
        let mut out = Vec::new();
        let mut malformed = false;
        let mut pos = 0usize;
        while pos < dbuff.len() {
            let nleng = dbuff[pos] as usize;
            if pos + nleng + 5 <= dbuff.len() {
                let name = &dbuff[pos + 1..pos + 1 + nleng];
                let inode = get32(&dbuff[pos + 1 + nleng..]);
                let inoleng = if nleng > 255 - 9 { 255 } else { nleng + 9 };
                out.push(inoleng as u8);
                let mut hexbuf = [0u8; 8];
                dir_hexgen(&mut hexbuf, inode);
                out.extend_from_slice(&hexbuf);
                out.push(b'|');
                if nleng > 255 - 9 {
                    out.extend_from_slice(&name[..255 - 9]);
                } else {
                    out.extend_from_slice(name);
                }
                out.extend_from_slice(&inode.to_be_bytes());
                out.push(TYPE_FILE);
                pos += 1 + nleng + 4;
            } else {
                malformed = true;
                break;
            }
        }
        (out, malformed)
    }

    /// lookup name resolution within the meta tree
    pub enum Lookup {
        /// static meta inode (reply with meta_stat)
        MetaInode(u32),
        /// "HEX|name" entry: needs fs_getdetachedattr
        DetachedAttr(u32),
        /// the .masterinfo file
        MasterInfo,
        NotFound,
    }

    pub fn resolve_lookup(
        parent: u32,
        name: &[u8],
        master_ge_3064: bool,
        flat_trash: bool,
    ) -> Lookup {
        match parent {
            META_ROOT_INODE => {
                if name == b"." || name == b".." {
                    Lookup::MetaInode(META_ROOT_INODE)
                } else if name == META_TRASH_NAME {
                    Lookup::MetaInode(META_TRASH_INODE)
                } else if name == META_SUSTAINED_NAME {
                    Lookup::MetaInode(META_SUSTAINED_INODE)
                } else if name == MASTERINFO_NAME {
                    Lookup::MasterInfo
                } else {
                    Lookup::NotFound
                }
            }
            META_TRASH_INODE => {
                if name == b"." {
                    Lookup::MetaInode(META_TRASH_INODE)
                } else if name == b".." {
                    Lookup::MetaInode(META_ROOT_INODE)
                } else if name == META_UNDEL_NAME {
                    Lookup::MetaInode(META_UNDEL_INODE)
                } else if master_ge_3064 && !flat_trash {
                    // subtrash hex name (strtoul, no '|' required here)
                    let mut inode: u32 = 0;
                    let mut any = false;
                    for &b in name {
                        let d = match b {
                            b'0'..=b'9' => (b - b'0') as u32,
                            b'a'..=b'f' => (b - b'a') as u32 + 10,
                            b'A'..=b'F' => (b - b'A') as u32 + 10,
                            _ => {
                                any = false;
                                break;
                            }
                        };
                        inode = inode.wrapping_mul(16).wrapping_add(d);
                        any = true;
                    }
                    if any && inode < TRASH_BUCKETS {
                        Lookup::MetaInode(META_SUBTRASH_INODE_MIN + inode)
                    } else {
                        Lookup::NotFound
                    }
                } else {
                    let inode = name_to_inode(name);
                    if inode > 0 {
                        Lookup::DetachedAttr(inode)
                    } else {
                        Lookup::NotFound
                    }
                }
            }
            META_UNDEL_INODE => {
                if name == b"." {
                    Lookup::MetaInode(META_UNDEL_INODE)
                } else if name == b".." {
                    Lookup::MetaInode(META_TRASH_INODE)
                } else {
                    Lookup::NotFound
                }
            }
            META_SUSTAINED_INODE => {
                if name == b"." {
                    Lookup::MetaInode(META_SUSTAINED_INODE)
                } else if name == b".." {
                    Lookup::MetaInode(META_ROOT_INODE)
                } else {
                    let inode = name_to_inode(name);
                    if inode > 0 {
                        Lookup::DetachedAttr(inode)
                    } else {
                        Lookup::NotFound
                    }
                }
            }
            META_SUBTRASH_INODE_MIN..=META_SUBTRASH_INODE_MAX => {
                if name == b"." {
                    Lookup::MetaInode(parent)
                } else if name == b".." {
                    Lookup::MetaInode(META_TRASH_INODE)
                } else if name == META_UNDEL_NAME {
                    Lookup::MetaInode(META_UNDEL_INODE)
                } else {
                    let inode = name_to_inode(name);
                    if inode > 0 {
                        Lookup::DetachedAttr(inode)
                    } else {
                        Lookup::NotFound
                    }
                }
            }
            _ => Lookup::NotFound,
        }
    }

    /// read slicing shared by meta_read and the masterinfo read:
    /// Some((start,len)) or None for EOF (off<0 pre-checked by caller)
    pub fn slice_range(off: u64, size: u64, total: u64) -> Option<(u64, u64)> {
        if off >= total {
            None
        } else if off + size > total {
            Some((off, total - off))
        } else {
            Some((off, size))
        }
    }
}

// ---------------------------------------------------------------------------
// Boundary: FUSE ops (fuse_reply_* / fs_* master calls).
// ---------------------------------------------------------------------------

use imp::Lookup;
use std::sync::Mutex as StdMutex;

pub const PKGVERSION: ::core::ffi::c_int = 4 * 1000000 + 59 * 1000 + 4; // VERSMAJ/MID/MIN (transpiled value, frozen)
static mut debug_mode: ::core::ffi::c_int = 0;
static mut flat_trash: ::core::ffi::c_int = 0;
static mut entry_cache_timeout: ::core::ffi::c_double = 0.0;
static mut attr_cache_timeout: ::core::ffi::c_double = 1.0;

/// .masterinfo file content attr (masterinfoattr in C)
static MASTERINFOATTR: [uint8_t; 36] = [
    b'f', 0x01, 0x24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
    0, 0, 0, 0, 0, 22, 0,
];

/// SAFETY: config statics set once in mfs_meta_init before FUSE threads.
unsafe fn flat_trash_on() -> bool {
    unsafe { flat_trash != 0 }
}

/// SAFETY: as above.
unsafe fn entry_to() -> ::core::ffi::c_double {
    unsafe { entry_cache_timeout }
}

/// SAFETY: as above.
unsafe fn attr_to() -> ::core::ffi::c_double {
    unsafe { attr_cache_timeout }
}

fn master_ge_3064() -> bool {
    unsafe { master_version() >= imp::VERSION_3_0_64 }
}

/// fill a C struct stat from StatFields (zero first, as C memset)
/// SAFETY: stbuf valid for writes.
unsafe fn fill_stat(inode: u32, f: &imp::StatFields, stbuf: *mut stat) {
    unsafe {
        ::core::ptr::write_bytes(stbuf as *mut uint8_t, 0, 1);
        (*stbuf).st_ino = inode as __ino_t;
        (*stbuf).st_mode = f.mode;
        (*stbuf).st_size = f.size as __off_t;
        (*stbuf).st_blocks = f.blocks as __blkcnt_t;
        (*stbuf).st_blksize = f.blksize as __blksize_t;
        (*stbuf).st_uid = f.uid;
        (*stbuf).st_gid = f.gid;
        (*stbuf).st_atim.tv_sec = f.atime as __time_t;
        (*stbuf).st_mtim.tv_sec = f.mtime as __time_t;
        (*stbuf).st_ctim.tv_sec = f.ctime as __time_t;
        (*stbuf).st_nlink = f.nlink as __nlink_t;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn mfs_meta_name_to_inode(name: *const ::core::ffi::c_char) -> uint32_t {
    // SAFETY: name is a NUL-terminated C string per C contract.
    let bytes = unsafe { ::core::ffi::CStr::from_ptr(name) }.to_bytes();
    imp::name_to_inode(bytes)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_statfs(req: fuse_req_t, _ino: fuse_ino_t) {
    unsafe {
        let mut totalspace: uint64_t = 0;
        let mut availspace: uint64_t = 0;
        let mut freespace: uint64_t = 0;
        let mut trashspace: uint64_t = 0;
        let mut sustainedspace: uint64_t = 0;
        let mut inodes: uint32_t = 0;
        let mut stfsbuf: statvfs = ::core::mem::zeroed();
        fs_statfs(
            &raw mut totalspace,
            &raw mut availspace,
            &raw mut freespace,
            &raw mut trashspace,
            &raw mut sustainedspace,
            &raw mut inodes,
        );
        stfsbuf.f_namemax = NAME_MAX as ::core::ffi::c_ulong;
        stfsbuf.f_frsize = imp::MFSBLOCKSIZE as ::core::ffi::c_ulong;
        stfsbuf.f_bsize = imp::MFSBLOCKSIZE as ::core::ffi::c_ulong;
        stfsbuf.f_blocks = (trashspace + sustainedspace) / imp::MFSBLOCKSIZE as u64;
        stfsbuf.f_bfree = sustainedspace / imp::MFSBLOCKSIZE as u64;
        stfsbuf.f_bavail = stfsbuf.f_bfree;
        stfsbuf.f_files = (1000000000 as __fsfilcnt64_t) + (PKGVERSION as __fsfilcnt64_t);
        stfsbuf.f_ffree = stfsbuf.f_files;
        stfsbuf.f_favail = stfsbuf.f_files;
        fuse_reply_statfs(req, &raw const stfsbuf);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_lookup(
    req: fuse_req_t,
    parent: fuse_ino_t,
    name: *const ::core::ffi::c_char,
) {
    unsafe {
        let namelen = libc::strlen(name);
        let nameb = ::core::slice::from_raw_parts(name as *const uint8_t, namelen);
        let mut e: fuse_entry_param = ::core::mem::zeroed();
        match imp::resolve_lookup(parent as uint32_t, nameb, master_ge_3064(), flat_trash_on()) {
            Lookup::MetaInode(inode) => {
                e.ino = inode as fuse_ino_t;
                e.attr_timeout = attr_to();
                e.entry_timeout = entry_to();
                let now = time(::core::ptr::null_mut()) as u32;
                let f = imp::meta_stat(inode, now);
                fill_stat(inode, &f, &raw mut e.attr);
                fuse_reply_entry(req, &raw const e);
            }
            Lookup::MasterInfo => {
                e.ino = imp::MASTERINFO_INODE as fuse_ino_t;
                e.attr_timeout = 3600.0;
                e.entry_timeout = 3600.0;
                let f = imp::attr_to_stat(imp::MASTERINFO_INODE, &MASTERINFOATTR);
                fill_stat(imp::MASTERINFO_INODE, &f, &raw mut e.attr);
                fuse_reply_entry(req, &raw const e);
            }
            Lookup::DetachedAttr(inode) => {
                let mut attr = [0u8; 36];
                let status = imp::errorconv(fs_getdetachedattr(inode, attr.as_mut_ptr()) as i32);
                if status != 0 {
                    fuse_reply_err(req, status);
                } else {
                    e.ino = inode as fuse_ino_t;
                    e.attr_timeout = attr_to();
                    e.entry_timeout = entry_to();
                    let f = imp::attr_to_stat(inode, &attr);
                    fill_stat(inode, &f, &raw mut e.attr);
                    fuse_reply_entry(req, &raw const e);
                }
            }
            Lookup::NotFound => {
                fuse_reply_err(req, ENOENT);
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_getattr(
    req: fuse_req_t,
    ino: fuse_ino_t,
    _fi: *mut fuse_file_info,
) {
    unsafe {
        let inode = ino as uint32_t;
        let mut stbuf: stat = ::core::mem::zeroed();
        if inode == imp::MASTERINFO_INODE {
            let f = imp::attr_to_stat(inode, &MASTERINFOATTR);
            fill_stat(inode, &f, &raw mut stbuf);
            fuse_reply_attr(req, &raw const stbuf, 3600.0);
        } else if imp::is_special_inode(inode) {
            let now = time(::core::ptr::null_mut()) as u32;
            let f = imp::meta_stat(inode, now);
            fill_stat(inode, &f, &raw mut stbuf);
            fuse_reply_attr(req, &raw const stbuf, attr_to());
        } else {
            let mut attr = [0u8; 36];
            let status = imp::errorconv(fs_getdetachedattr(inode, attr.as_mut_ptr()) as i32);
            if status != 0 {
                fuse_reply_err(req, status);
            } else {
                let f = imp::attr_to_stat(inode, &attr);
                fill_stat(inode, &f, &raw mut stbuf);
                fuse_reply_attr(req, &raw const stbuf, attr_to());
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_setattr(
    req: fuse_req_t,
    ino: fuse_ino_t,
    _stbuf: *mut stat,
    _to_set: ::core::ffi::c_int,
    fi: *mut fuse_file_info,
) {
    unsafe {
        mfs_meta_getattr(req, ino, fi);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_unlink(
    req: fuse_req_t,
    parent: fuse_ino_t,
    name: *const ::core::ffi::c_char,
) {
    unsafe {
        let p = parent as uint32_t;
        if !(p == imp::META_TRASH_INODE
            || (imp::META_SUBTRASH_INODE_MIN..=imp::META_SUBTRASH_INODE_MAX).contains(&p))
        {
            fuse_reply_err(req, EACCES);
            return;
        }
        let namelen = libc::strlen(name);
        let inode = imp::name_to_inode(::core::slice::from_raw_parts(
            name as *const uint8_t,
            namelen,
        ));
        if inode == 0 {
            fuse_reply_err(req, ENOENT);
            return;
        }
        let status = imp::errorconv(fs_purge(inode) as i32);
        fuse_reply_err(req, status);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_rename(
    req: fuse_req_t,
    parent: fuse_ino_t,
    name: *const ::core::ffi::c_char,
    newparent: fuse_ino_t,
    _newname: *const ::core::ffi::c_char,
    _flags: ::core::ffi::c_uint,
) {
    unsafe {
        let p = parent as uint32_t;
        let np = newparent as uint32_t;
        if !(p == imp::META_TRASH_INODE
            || (imp::META_SUBTRASH_INODE_MIN..=imp::META_SUBTRASH_INODE_MAX).contains(&p))
            && np != imp::META_UNDEL_INODE
        {
            fuse_reply_err(req, EACCES);
            return;
        }
        let namelen = libc::strlen(name);
        let inode = imp::name_to_inode(::core::slice::from_raw_parts(
            name as *const uint8_t,
            namelen,
        ));
        if inode == 0 {
            fuse_reply_err(req, ENOENT);
            return;
        }
        let status = imp::errorconv(fs_undel(inode) as i32);
        fuse_reply_err(req, status);
    }
}

struct DirBuf {
    data: Vec<u8>,
    wasread: bool,
    lock: StdMutex<()>,
}

/// build the directory content blob for `ino` (dirbuf_meta_fill);
/// returns None on master-error (C left b->p NULL/size 0)
unsafe fn dirbuf_meta_fill(ino: uint32_t) -> Option<Vec<u8>> {
    unsafe {
        let msize = imp::dir_metaentries_size(ino, master_ge_3064(), flat_trash_on());
        let mut dbuff: *const uint8_t = ::core::ptr::null();
        let mut dsize: uint32_t = 0;
        let mut data: Option<Vec<u8>> = None;
        if ino == imp::META_TRASH_INODE && (!master_ge_3064() || flat_trash_on()) {
            if fs_gettrash(0xFFFFFFFF, &raw mut dbuff, &raw mut dsize) == MFS_STATUS_OK as uint8_t {
                let blob = ::core::slice::from_raw_parts(dbuff, dsize as usize);
                let (conv, malformed) = imp::dir_dataentries_convert(blob);
                if malformed {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"dir data malformed (trash)\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                data = Some(conv);
            }
        } else if ino == imp::META_SUSTAINED_INODE {
            if fs_getsustained(&raw mut dbuff, &raw mut dsize) == MFS_STATUS_OK as uint8_t {
                let blob = ::core::slice::from_raw_parts(dbuff, dsize as usize);
                let (conv, malformed) = imp::dir_dataentries_convert(blob);
                if malformed {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"dir data malformed (sustained)\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                data = Some(conv);
            }
        } else if (imp::META_SUBTRASH_INODE_MIN..=imp::META_SUBTRASH_INODE_MAX).contains(&ino)
            && master_ge_3064()
            && !flat_trash_on()
        {
            if fs_gettrash(
                ino - imp::META_SUBTRASH_INODE_MIN,
                &raw mut dbuff,
                &raw mut dsize,
            ) == MFS_STATUS_OK as uint8_t
            {
                let blob = ::core::slice::from_raw_parts(dbuff, dsize as usize);
                let (conv, _) = imp::dir_dataentries_convert(blob);
                data = Some(conv);
            }
        }
        if msize == 0 && data.is_none() {
            return None;
        }
        let mut out = Vec::with_capacity(msize as usize + data.as_ref().map_or(0, |d| d.len()));
        if msize > 0 {
            out.extend_from_slice(&imp::dir_metaentries_fill(
                ino,
                master_ge_3064(),
                flat_trash_on(),
            ));
        }
        if let Some(d) = data {
            out.extend_from_slice(&d);
        }
        Some(out)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_opendir(
    req: fuse_req_t,
    ino: fuse_ino_t,
    fi: *mut fuse_file_info,
) {
    unsafe {
        let inode = ino as uint32_t;
        if inode == imp::META_ROOT_INODE
            || inode == imp::META_TRASH_INODE
            || inode == imp::META_UNDEL_INODE
            || inode == imp::META_SUSTAINED_INODE
            || (imp::META_SUBTRASH_INODE_MIN..=imp::META_SUBTRASH_INODE_MAX).contains(&inode)
        {
            let dirinfo = Box::new(DirBuf {
                data: Vec::new(),
                wasread: false,
                lock: StdMutex::new(()),
            });
            let raw = Box::into_raw(dirinfo);
            (*fi).fh = raw as ::core::ffi::c_ulong;
            if fuse_reply_open(req, fi) == -ENOENT {
                (*fi).fh = 0;
                drop(Box::from_raw(raw));
            }
        } else {
            fuse_reply_err(req, ENOTDIR);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_readdir(
    req: fuse_req_t,
    ino: fuse_ino_t,
    size: size_t,
    off: off_t,
    fi: *mut fuse_file_info,
) {
    unsafe {
        if off < 0 {
            fuse_reply_err(req, EINVAL);
            return;
        }
        // SAFETY: fh from mfs_meta_opendir.
        let dirinfo = &mut *((*fi).fh as *mut DirBuf);
        let _g = dirinfo.lock.lock().unwrap();
        if !dirinfo.wasread || off == 0 {
            dirinfo.data = dirbuf_meta_fill(ino as uint32_t).unwrap_or_default();
        }
        dirinfo.wasread = true;
        if off as usize >= dirinfo.data.len() {
            fuse_reply_buf(req, ::core::ptr::null(), 0);
            return;
        }
        let size = size.min(READDIR_BUFFSIZE as size_t);
        let mut buffer = vec![0u8; size];
        let mut opos: size_t = 0;
        let mut off = off as usize;
        let data = &dirinfo.data;
        let mut pos = off;
        while pos < data.len() {
            let nleng = data[pos] as usize;
            let name_start = pos + 1;
            let entry_end = name_start + nleng + 5;
            if entry_end > data.len() {
                break;
            }
            let inode = u32::from_be_bytes([
                data[name_start + nleng],
                data[name_start + nleng + 1],
                data[name_start + nleng + 2],
                data[name_start + nleng + 3],
            ]);
            let type_ = data[name_start + nleng + 4];
            off += nleng + 6;
            let mut stbuf: stat = ::core::mem::zeroed();
            stbuf.st_ino = inode as __ino_t;
            stbuf.st_mode = imp::type_to_mode(type_);
            // fuse_add_direntry wants a NUL-terminated name
            let mut cname = Vec::with_capacity(nleng + 1);
            cname.extend_from_slice(&data[name_start..name_start + nleng]);
            cname.push(0);
            let oleng = fuse_add_direntry(
                req,
                buffer.as_mut_ptr().add(opos) as *mut ::core::ffi::c_char,
                size - opos,
                cname.as_ptr() as *const ::core::ffi::c_char,
                &raw const stbuf,
                off as off_t,
            ) as size_t;
            if opos + oleng > size {
                break;
            }
            opos += oleng;
            pos = entry_end;
        }
        fuse_reply_buf(req, buffer.as_ptr() as *const ::core::ffi::c_char, opos);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_releasedir(
    req: fuse_req_t,
    _ino: fuse_ino_t,
    fi: *mut fuse_file_info,
) {
    unsafe {
        let raw = (*fi).fh as *mut DirBuf;
        if !raw.is_null() {
            // Match C teardown: wait for any in-flight readdir before drop.
            {
                let dirinfo = &*raw;
                let _g = dirinfo.lock.lock().unwrap();
            }
            // SAFETY: fh from mfs_meta_opendir, released exactly once.
            drop(Box::from_raw(raw));
            (*fi).fh = 0;
        }
        fuse_reply_err(req, 0);
    }
}

struct PathBuf {
    data: Vec<u8>,
    changed: bool,
    lock: StdMutex<()>,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_open(req: fuse_req_t, ino: fuse_ino_t, fi: *mut fuse_file_info) {
    unsafe {
        let inode = ino as uint32_t;
        if inode == imp::MASTERINFO_INODE {
            (*fi).fh = 0;
            (*fi).set_direct_io(0);
            (*fi).set_keep_cache(1);
            fuse_reply_open(req, fi);
            return;
        }
        if imp::is_special_inode(inode) {
            fuse_reply_err(req, EACCES);
            return;
        }
        let mut path: *const uint8_t = ::core::ptr::null();
        let status = imp::errorconv(fs_gettrashpath(inode, &raw mut path) as i32);
        if status != 0 {
            fuse_reply_err(req, status);
            return;
        }
        // C: size = strlen(path)+1, copy size-1 bytes, last byte = '\n'
        let plen = libc::strlen(path as *const ::core::ffi::c_char);
        let mut data = Vec::with_capacity(plen + 1);
        data.extend_from_slice(::core::slice::from_raw_parts(path, plen));
        data.push(b'\n');
        let pathinfo = Box::new(PathBuf {
            data,
            changed: false,
            lock: StdMutex::new(()),
        });
        (*fi).set_direct_io(1);
        let raw = Box::into_raw(pathinfo);
        (*fi).fh = raw as ::core::ffi::c_ulong;
        if fuse_reply_open(req, fi) == -ENOENT {
            (*fi).fh = 0;
            drop(Box::from_raw(raw));
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_release(
    req: fuse_req_t,
    ino: fuse_ino_t,
    fi: *mut fuse_file_info,
) {
    unsafe {
        let inode = ino as uint32_t;
        if inode == imp::MASTERINFO_INODE {
            fuse_reply_err(req, 0);
            return;
        }
        let raw = (*fi).fh as *mut PathBuf;
        if raw.is_null() {
            fuse_reply_err(req, EBADF);
            return;
        }
        // SAFETY: fh from mfs_meta_open, released exactly once.
        let pathinfo = Box::from_raw(raw);
        {
            let _g = pathinfo.lock.lock().unwrap();
            let mut data = pathinfo.data.clone();
            if pathinfo.changed {
                if data.last() == Some(&b'\n') {
                    *data.last_mut().unwrap() = 0;
                } else {
                    data.push(0);
                }
                fs_settrashpath(inode, data.as_ptr());
            }
        }
        (*fi).fh = 0;
        fuse_reply_err(req, 0);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_read(
    req: fuse_req_t,
    ino: fuse_ino_t,
    size: size_t,
    off: off_t,
    fi: *mut fuse_file_info,
) {
    unsafe {
        let inode = ino as uint32_t;
        if inode == imp::MASTERINFO_INODE {
            let mut masterinfo = [0u8; 22];
            fs_getmasterlocation(masterinfo.as_mut_ptr());
            masterproxy_getlocation(masterinfo.as_mut_ptr());
            if off < 0 {
                fuse_reply_err(req, EINVAL);
                return;
            }
            match imp::slice_range(off as u64, size as u64, 22) {
                None => {
                    fuse_reply_buf(req, ::core::ptr::null(), 0);
                }
                Some((start, len)) => {
                    fuse_reply_buf(
                        req,
                        masterinfo.as_ptr().add(start as usize) as *const ::core::ffi::c_char,
                        len as size_t,
                    );
                }
            }
            return;
        }
        if (*fi).fh == 0 {
            fuse_reply_err(req, EBADF);
            return;
        }
        // SAFETY: fh from mfs_meta_open.
        let pathinfo = &mut *((*fi).fh as *mut PathBuf);
        let _g = pathinfo.lock.lock().unwrap();
        if off < 0 {
            drop(_g);
            fuse_reply_err(req, EINVAL);
            return;
        }
        match imp::slice_range(off as u64, size as u64, pathinfo.data.len() as u64) {
            None => {
                fuse_reply_buf(req, ::core::ptr::null(), 0);
            }
            Some((start, len)) => {
                fuse_reply_buf(
                    req,
                    pathinfo.data.as_ptr().add(start as usize) as *const ::core::ffi::c_char,
                    len as size_t,
                );
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_write(
    req: fuse_req_t,
    ino: fuse_ino_t,
    buf: *const ::core::ffi::c_char,
    size: size_t,
    off: off_t,
    fi: *mut fuse_file_info,
) {
    unsafe {
        let inode = ino as uint32_t;
        if inode == imp::MASTERINFO_INODE {
            fuse_reply_err(req, EACCES);
            return;
        }
        if (*fi).fh == 0 {
            fuse_reply_err(req, EBADF);
            return;
        }
        if off < 0 || off as u64 + size as u64 > PATH_SIZE_LIMIT as u64 {
            fuse_reply_err(req, EINVAL);
            return;
        }
        // SAFETY: fh from mfs_meta_open; buf valid for size bytes.
        let pathinfo = &mut *((*fi).fh as *mut PathBuf);
        let _g = pathinfo.lock.lock().unwrap();
        if !pathinfo.changed {
            pathinfo.data.clear();
        }
        let end = off as usize + size;
        if end > pathinfo.data.len() {
            pathinfo.data.resize(end, 0);
        }
        ::core::ptr::copy_nonoverlapping(
            buf as *const uint8_t,
            pathinfo.data.as_mut_ptr().add(off as usize),
            size,
        );
        pathinfo.changed = true;
        drop(_g);
        fuse_reply_write(req, size);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfs_meta_init(
    debug_mode_in: ::core::ffi::c_int,
    entry_cache_timeout_in: ::core::ffi::c_double,
    attr_cache_timeout_in: ::core::ffi::c_double,
    flat_trash_in: ::core::ffi::c_int,
) {
    unsafe {
        debug_mode = debug_mode_in;
        entry_cache_timeout = entry_cache_timeout_in;
        attr_cache_timeout = attr_cache_timeout_in;
        flat_trash = flat_trash_in;
        if debug_mode != 0 {
            let et = entry_cache_timeout;
            let at = attr_cache_timeout;
            eprintln!("cache parameters: entry_cache_timeout={et:.2} attr_cache_timeout={at:.2}");
            if flat_trash != 0 {
                eprintln!("force using 'flat' trash");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::imp::*;
    use std::format;
    use std::vec;
    use std::vec::Vec;

    #[test]
    fn name_to_inode_reference() {
        assert_eq!(name_to_inode(b"0000001A|file.txt"), 0x1A);
        assert_eq!(name_to_inode(b"deadbeef|x"), 0xdeadbeef);
        assert_eq!(name_to_inode(b"ABC|n"), 0xABC); // uppercase hex
        assert_eq!(name_to_inode(b"123"), 0); // no '|'
        assert_eq!(name_to_inode(b"123|"), 0); // '|' must have follower
        assert_eq!(name_to_inode(b"|x"), 0); // no digits
        assert_eq!(name_to_inode(b"xyz|n"), 0);
        assert_eq!(name_to_inode(b"12aF|some longer name"), 0x12AF);
    }

    #[test]
    fn errorconv_table() {
        assert_eq!(errorconv(0), 0);
        assert_eq!(errorconv(3), 2); // ENOENT
        assert_eq!(errorconv(8), 5); // IO → EIO
        assert_eq!(errorconv(33), 30); // EROFS
        assert_eq!(errorconv(40), 122); // QUOTA → EDQUOT
        assert_eq!(errorconv(99), 22); // unknown → EINVAL
    }

    #[test]
    fn type_conversions() {
        assert_eq!(fsnodes_type_convert(DISP_TYPE_FILE), TYPE_FILE);
        assert_eq!(fsnodes_type_convert(DISP_TYPE_SUSTAINED), TYPE_SUSTAINED);
        assert_eq!(fsnodes_type_convert(0), 0);
        assert_eq!(type_to_mode(DISP_TYPE_DIRECTORY), S_IFDIR);
        assert_eq!(type_to_mode(TYPE_SYMLINK), S_IFLNK);
        assert_eq!(type_to_mode(103), 0); // 'g' unmapped
        // masked high bit preserved: type&0x7F
        assert_eq!(type_to_mode(DISP_TYPE_FILE | 0x80), S_IFREG);
    }

    #[test]
    fn attr_to_stat_modern_record() {
        let mut attr = [0u8; 36];
        attr[0] = 1; // flags < 64 → modern
        attr[1..3].copy_from_slice(&0x81A4u16.to_be_bytes()); // type 8 (TRASH) | 0o644
        attr[3..7].copy_from_slice(&1000u32.to_be_bytes());
        attr[7..11].copy_from_slice(&1001u32.to_be_bytes());
        attr[11..15].copy_from_slice(&11u32.to_be_bytes());
        attr[15..19].copy_from_slice(&22u32.to_be_bytes());
        attr[19..23].copy_from_slice(&33u32.to_be_bytes());
        attr[23..27].copy_from_slice(&1u32.to_be_bytes());
        attr[27..35].copy_from_slice(&12345u64.to_be_bytes());
        let f = attr_to_stat(555, &attr);
        assert_eq!(f.ino, 555);
        assert_eq!(f.mode, S_IFREG | 0o644);
        assert_eq!(f.size, 12345);
        assert_eq!(f.blocks, (12345 + 511) / 512);
        assert_eq!(f.uid, 1000);
        assert_eq!(f.gid, 1001);
        assert_eq!(f.atime, 11);
        assert_eq!(f.mtime, 22);
        assert_eq!(f.ctime, 33);
        assert_eq!(f.nlink, 1);
        assert_eq!(f.blksize, MFSBLOCKSIZE);
    }

    #[test]
    fn attr_to_stat_legacy_record_and_dir() {
        let mut attr = [0u8; 36];
        attr[0] = DISP_TYPE_DIRECTORY; // >= 64 → legacy
        attr[1..3].copy_from_slice(&0o755u16.to_be_bytes());
        let f = attr_to_stat(1, &attr);
        assert_eq!(f.mode, 0); // only FILE/TRASH/SUSTAINED get S_IFREG
        attr[0] = DISP_TYPE_FILE;
        let f = attr_to_stat(1, &attr);
        assert_eq!(f.mode, S_IFREG | 0o755);
    }

    #[test]
    fn meta_stat_table() {
        let f = meta_stat(META_ROOT_INODE, 100);
        assert_eq!((f.nlink, f.mode), (4, S_IFDIR | 0o555));
        let f = meta_stat(META_TRASH_INODE, 100);
        assert_eq!((f.nlink, f.mode), (3 + TRASH_BUCKETS, S_IFDIR | 0o700));
        let f = meta_stat(META_UNDEL_INODE, 100);
        assert_eq!((f.nlink, f.mode), (2 + TRASH_BUCKETS, S_IFDIR | 0o200));
        let f = meta_stat(META_SUSTAINED_INODE, 100);
        assert_eq!((f.nlink, f.mode), (2, S_IFDIR | 0o500));
        let f = meta_stat(META_SUBTRASH_INODE_MIN + 7, 100);
        assert_eq!((f.nlink, f.mode), (3, S_IFDIR | 0o700));
        assert_eq!(f.atime, 100);
    }

    #[test]
    fn metaentries_root_blob_exact_bytes() {
        let blob = dir_metaentries_fill(META_ROOT_INODE, true, false);
        let mut want = Vec::new();
        for (name, ino) in [
            (&b"."[..], META_ROOT_INODE),
            (b"..", META_ROOT_INODE),
            (b"trash", META_TRASH_INODE),
            (b"sustained", META_SUSTAINED_INODE),
        ] {
            want.push(name.len() as u8);
            want.extend_from_slice(name);
            want.extend_from_slice(&ino.to_be_bytes());
            want.push(TYPE_DIRECTORY);
        }
        assert_eq!(blob, want);
        assert_eq!(
            blob.len() as u32,
            dir_metaentries_size(META_ROOT_INODE, true, false)
        );
    }

    #[test]
    fn metaentries_trash_bucket_names() {
        // bucketed trash: names are 3 hex digits (TRASH_BUCKETS=4096)
        let blob = dir_metaentries_fill(META_TRASH_INODE, true, false);
        let size = dir_metaentries_size(META_TRASH_INODE, true, false);
        assert_eq!(blob.len() as u32, size);
        // first bucket entry after ".", "..", "undel": "000"
        let prefix_len = (6 + 1) + (6 + 2) + (6 + 5); // . + .. + undel
        assert_eq!(&blob[prefix_len..prefix_len + 4], &[3, b'0', b'0', b'0']);
        // bucket 0xABC → name "ABC"
        let b0_size = 3 + 6;
        let abc_off = prefix_len + 0xABC * b0_size;
        assert_eq!(&blob[abc_off..abc_off + 4], &[3, b'A', b'B', b'C']);
        assert_eq!(
            u32::from_be_bytes([
                blob[abc_off + 4],
                blob[abc_off + 5],
                blob[abc_off + 6],
                blob[abc_off + 7]
            ]),
            META_SUBTRASH_INODE_MIN + 0xABC
        );
        // flat trash: no buckets
        let flat = dir_metaentries_fill(META_TRASH_INODE, false, false);
        assert_eq!(
            flat.len() as u32,
            dir_metaentries_size(META_TRASH_INODE, false, false)
        );
        assert_eq!(flat.len(), prefix_len);
    }

    #[test]
    fn dataentries_size_and_convert() {
        // two entries: "aa"(inode 1), "bbb"(inode 0x1234)
        let mut dbuff = Vec::new();
        dbuff.push(2);
        dbuff.extend_from_slice(b"aa");
        dbuff.extend_from_slice(&1u32.to_be_bytes());
        dbuff.push(3);
        dbuff.extend_from_slice(b"bbb");
        dbuff.extend_from_slice(&0x1234u32.to_be_bytes());
        assert_eq!(dir_dataentries_size(&dbuff), (6 + 2 + 9 + 6 + 3 + 9) as u32);
        let (conv, malformed) = dir_dataentries_convert(&dbuff);
        assert!(!malformed);
        assert_eq!(conv.len() as u32, dir_dataentries_size(&dbuff));
        // first entry: len=11, "00000001|aa", inode 1, TYPE_FILE
        assert_eq!(conv[0], 11);
        assert_eq!(&conv[1..9], b"00000001");
        assert_eq!(conv[9], b'|');
        assert_eq!(&conv[10..12], b"aa");
        assert_eq!(
            u32::from_be_bytes([conv[12], conv[13], conv[14], conv[15]]),
            1
        );
        assert_eq!(conv[16], TYPE_FILE);
        // malformed tail → flagged, stops
        let mut bad = dbuff.clone();
        bad.extend_from_slice(&[9, b'x']); // claims 9+5 bytes, truncated
        let (_, malformed) = dir_dataentries_convert(&bad);
        assert!(malformed);
    }

    #[test]
    fn hexgen_reference() {
        let mut out = [0u8; 8];
        dir_hexgen(&mut out, 0x00000001);
        assert_eq!(&out, b"00000001");
        dir_hexgen(&mut out, 0xDEADBEEF);
        assert_eq!(&out, b"DEADBEEF");
        dir_hexgen(&mut out, 0);
        assert_eq!(&out, b"00000000");
    }

    #[test]
    fn long_name_truncation() {
        // name of 250 chars → inoleng 255, name truncated to 246
        let name = vec![b'x'; 250];
        let mut dbuff = Vec::new();
        dbuff.push(250);
        dbuff.extend_from_slice(&name);
        dbuff.extend_from_slice(&7u32.to_be_bytes());
        let (conv, _) = dir_dataentries_convert(&dbuff);
        assert_eq!(conv[0], 255);
        assert_eq!(&conv[1..9], b"00000007");
        assert_eq!(conv.len(), 1 + 255 + 4 + 1);
    }

    #[test]
    fn resolve_lookup_tree() {
        assert!(matches!(
            resolve_lookup(META_ROOT_INODE, b"trash", true, false),
            Lookup::MetaInode(META_TRASH_INODE)
        ));
        assert!(matches!(
            resolve_lookup(META_ROOT_INODE, b"sustained", true, false),
            Lookup::MetaInode(META_SUSTAINED_INODE)
        ));
        assert!(matches!(
            resolve_lookup(META_ROOT_INODE, b".masterinfo", true, false),
            Lookup::MasterInfo
        ));
        assert!(matches!(
            resolve_lookup(META_ROOT_INODE, b"nope", true, false),
            Lookup::NotFound
        ));
        // bucketed trash: hex name resolves to subtrash inode
        assert!(matches!(
            resolve_lookup(META_TRASH_INODE, b"0aF", true, false),
            Lookup::MetaInode(i) if i == META_SUBTRASH_INODE_MIN + 0xAF
        ));
        assert!(matches!(
            resolve_lookup(META_TRASH_INODE, b"FFFF", true, false), // >= buckets
            Lookup::NotFound
        ));
        // flat trash: HEX|name → detached attr
        assert!(matches!(
            resolve_lookup(META_TRASH_INODE, b"00000001|f", false, false),
            Lookup::DetachedAttr(1)
        ));
        // sustained: HEX|name → detached attr
        assert!(matches!(
            resolve_lookup(META_SUSTAINED_INODE, b"00000002|g", true, false),
            Lookup::DetachedAttr(2)
        ));
        // undel parent: only . and ..
        assert!(matches!(
            resolve_lookup(META_UNDEL_INODE, b"..", true, false),
            Lookup::MetaInode(META_TRASH_INODE)
        ));
        assert!(matches!(
            resolve_lookup(META_UNDEL_INODE, b"undel", true, false),
            Lookup::NotFound
        ));
        // subtrash parent
        assert!(matches!(
            resolve_lookup(META_SUBTRASH_INODE_MIN + 5, b"undel", true, false),
            Lookup::MetaInode(META_UNDEL_INODE)
        ));
        assert!(matches!(
            resolve_lookup(META_SUBTRASH_INODE_MIN + 5, b".", true, false),
            Lookup::MetaInode(i) if i == META_SUBTRASH_INODE_MIN + 5
        ));
        assert!(matches!(
            resolve_lookup(999, b".", true, false),
            Lookup::NotFound
        ));
    }

    #[test]
    fn slice_range_cases() {
        assert_eq!(slice_range(0, 10, 22), Some((0, 10)));
        assert_eq!(slice_range(20, 10, 22), Some((20, 2)));
        assert_eq!(slice_range(22, 10, 22), None);
        assert_eq!(slice_range(0, 22, 22), Some((0, 22)));
        let _ = format!(""); // keep format import
    }
}
