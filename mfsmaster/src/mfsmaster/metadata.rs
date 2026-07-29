use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    pub type _bio;
    static mut stderr: *mut FILE;
    fn rename(
        __old: *const ::core::ffi::c_char,
        __new: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn mkstemp(__template: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn lseek(
        __fd: ::core::ffi::c_int,
        __offset: __off64_t,
        __whence: ::core::ffi::c_int,
    ) -> __off64_t;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sleep(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    fn fork() -> __pid_t;
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn lockf(
        __fd: ::core::ffi::c_int,
        __cmd: ::core::ffi::c_int,
        __len: __off64_t,
    ) -> ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    fn opendir(__name: *const ::core::ffi::c_char) -> *mut DIR;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn rewinddir(__dirp: *mut DIR);
    fn kill(__pid: __pid_t, __sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn bio_null_open(direction: uint8_t) -> *mut bio;
    fn bio_file_open(
        fname: *const ::core::ffi::c_char,
        direction: uint8_t,
        buffersize: uint32_t,
    ) -> *mut bio;
    fn bio_file_position(b: *mut bio) -> uint64_t;
    fn bio_file_size(b: *mut bio) -> uint64_t;
    fn bio_crc(b: *mut bio) -> uint32_t;
    fn bio_read(b: *mut bio, dst: *mut ::core::ffi::c_void, len: uint64_t) -> int64_t;
    fn bio_write(b: *mut bio, src: *const ::core::ffi::c_void, len: uint64_t) -> int64_t;
    fn bio_seek(b: *mut bio, offset: int64_t, whence: ::core::ffi::c_int) -> int8_t;
    fn bio_skip(b: *mut bio, len: uint64_t);
    fn bio_error(b: *mut bio) -> uint8_t;
    fn bio_lasterrno(b: *mut bio) -> ::core::ffi::c_int;
    fn bio_descriptor(b: *mut bio) -> ::core::ffi::c_int;
    fn bio_sync(b: *mut bio);
    fn bio_close(b: *mut bio);
    fn sessions_cleanup();
    fn sessions_init() -> ::core::ffi::c_int;
    fn sessions_new();
    fn sessions_store(fd: *mut bio) -> uint8_t;
    fn sessions_load(fd: *mut bio, mver: uint8_t) -> ::core::ffi::c_int;
    fn sessions_import();
    fn sessions_set_nextsessionid(nsi: uint32_t);
    fn dict_init() -> ::core::ffi::c_int;
    fn dict_cleanup();
    fn xattr_cleanup();
    fn xattr_store(fd: *mut bio) -> uint8_t;
    fn xattr_load(
        fd: *mut bio,
        mver: uint8_t,
        ignoreflag_0: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn xattr_init() -> ::core::ffi::c_int;
    fn posix_acl_cleanup();
    fn posix_acl_store(fd: *mut bio) -> uint8_t;
    fn posix_acl_load(
        fd: *mut bio,
        mver: uint8_t,
        ignoreflag_0: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn posix_acl_init() -> ::core::ffi::c_int;
    fn flock_store(fd: *mut bio) -> uint8_t;
    fn flock_load(fd: *mut bio, mver: uint8_t, ignoreflag_0: uint8_t) -> ::core::ffi::c_int;
    fn flock_cleanup();
    fn flock_init() -> ::core::ffi::c_int;
    fn posix_lock_store(fd: *mut bio) -> uint8_t;
    fn posix_lock_load(fd: *mut bio, mver: uint8_t, ignoreflag_0: uint8_t) -> ::core::ffi::c_int;
    fn posix_lock_cleanup();
    fn posix_lock_init() -> ::core::ffi::c_int;
    fn of_store(fd: *mut bio) -> uint8_t;
    fn of_load(fd: *mut bio, mver: uint8_t) -> ::core::ffi::c_int;
    fn of_cleanup();
    fn of_init() -> ::core::ffi::c_int;
    fn csdb_cleanup();
    fn csdb_store(fd: *mut bio) -> uint8_t;
    fn csdb_load(
        fd: *mut bio,
        mver: uint8_t,
        ignoreflag_0: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn csdb_init() -> ::core::ffi::c_int;
    fn sclass_store(fd: *mut bio) -> uint8_t;
    fn sclass_load(
        fd: *mut bio,
        mver: uint8_t,
        ignoreflag_0: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sclass_new();
    fn sclass_cleanup();
    fn sclass_init() -> ::core::ffi::c_int;
    fn patterns_cleanup();
    fn patterns_store(fd: *mut bio) -> uint8_t;
    fn patterns_load(
        fd: *mut bio,
        mver: uint8_t,
        ignoreflag_0: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn patterns_init() -> ::core::ffi::c_int;
    fn chunk_is_afterload_needed(mver: uint8_t) -> uint8_t;
    fn chunk_load(
        fd: *mut bio,
        mver: uint8_t,
        ignoreflag_0: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn chunk_store(fd: *mut bio) -> uint8_t;
    fn chunk_cleanup();
    fn chunk_newfs();
    fn chunk_strinit() -> ::core::ffi::c_int;
    fn fs_set_root_times(ts: uint32_t) -> ::core::ffi::c_int;
    fn fs_new();
    fn fs_cleanup();
    fn fs_printinfo();
    fn fs_afterload();
    fn fs_check_consistency(ignoreflag_0: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn fs_importnodes(
        fd: *mut bio,
        maxnodeid: uint32_t,
        ignoreflag_0: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn fs_loadnodes(
        fd: *mut bio,
        mver: uint8_t,
        ignoreflag_0: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn fs_loadedges(
        fd: *mut bio,
        mver: uint8_t,
        ignoreflag_0: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn fs_loadfree(
        fd: *mut bio,
        mver: uint8_t,
        ignoreflag_0: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn fs_loadquota(
        fd: *mut bio,
        mver: uint8_t,
        ignoreflag_0: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn fs_storenodes(fd: *mut bio) -> uint8_t;
    fn fs_storeedges(fd: *mut bio) -> uint8_t;
    fn fs_storefree(fd: *mut bio) -> uint8_t;
    fn fs_storequota(fd: *mut bio) -> uint8_t;
    fn fs_renumerate_edge_test();
    fn fs_strinit() -> ::core::ffi::c_int;
    fn rndu32() -> uint32_t;
    fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn merger_start(
        files: uint32_t,
        filenames: *mut *mut ::core::ffi::c_char,
        maxhole: uint64_t,
        minlv: uint64_t,
        maxlv: uint64_t,
    ) -> ::core::ffi::c_int;
    fn merger_loop(verblevel: uint8_t) -> ::core::ffi::c_int;
    fn changelog_rotate(rotate_flags: uint8_t);
    fn changelog(format: *const ::core::ffi::c_char, ...);
    fn changelog_findfirstversion(fname: *const ::core::ffi::c_char) -> uint64_t;
    fn changelog_findlastversion(fname: *const ::core::ffi::c_char) -> uint64_t;
    fn changelog_checkname(fname: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn monotonic_seconds() -> ::core::ffi::c_double;
    fn monotonic_useconds() -> uint64_t;
    fn matoclserv_close_lsock();
    fn matocsserv_close_lsock();
    fn matomlserv_close_lsock();
    fn mycrc32(crc: uint32_t, block: *const ::core::ffi::c_void, leng: uint32_t) -> uint32_t;
    fn processname_set(name: *mut ::core::ffi::c_char);
    fn cfg_getstr(
        name: *const ::core::ffi::c_char,
        def: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn cfg_getuint32(name: *const ::core::ffi::c_char, def: uint32_t) -> uint32_t;
    fn main_destruct_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    fn main_mayexit_register_fname(
        fun: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        fname: *const ::core::ffi::c_char,
    );
    fn main_reload_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    fn main_info_register_fname(
        fun: Option<unsafe extern "C" fn(*mut FILE) -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    fn main_chld_register_fname(
        pid: pid_t,
        fun: Option<unsafe extern "C" fn(pid_t, ::core::ffi::c_int) -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    fn main_time_register_fname(
        seconds: uint32_t,
        offset: uint32_t,
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    fn main_exit();
    fn main_time() -> uint32_t;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __ino64_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
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
pub type off_t = __off64_t;
pub type ssize_t = isize;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
pub type int8_t = i8;
pub type int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub struct dirent {
    pub d_ino: __ino64_t,
    pub d_off: __off64_t,
    pub d_reclen: ::core::ffi::c_ushort,
    pub d_type: ::core::ffi::c_uchar,
    pub d_name: [::core::ffi::c_char; 256],
}
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: ::core::ffi::c_int,
    pub tm_min: ::core::ffi::c_int,
    pub tm_hour: ::core::ffi::c_int,
    pub tm_mday: ::core::ffi::c_int,
    pub tm_mon: ::core::ffi::c_int,
    pub tm_year: ::core::ffi::c_int,
    pub tm_wday: ::core::ffi::c_int,
    pub tm_yday: ::core::ffi::c_int,
    pub tm_isdst: ::core::ffi::c_int,
    pub tm_gmtoff: ::core::ffi::c_long,
    pub tm_zone: *const ::core::ffi::c_char,
}
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type bio = _bio;
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const BIO_WRITE: C2Rust_Unnamed = 1;
pub const BIO_READ: C2Rust_Unnamed = 0;
pub type chlog_keep = _chlog_keep;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _chlog_keep {
    pub version: uint64_t,
    pub validtime: uint32_t,
    pub saverpid: pid_t,
    pub next: *mut _chlog_keep,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const F_TLOCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const F_TEST: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIGKILL: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_ENOENT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFS_ERROR_EINVAL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LASTSTORE_UNKNOWN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LASTSTORE_META_STORED_BG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LASTSTORE_DOWNLOADED: ::core::ffi::c_int = 1;
pub const LASTSTORE_META_STORED_FG: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LASTSTORE_CRC_STORED_BG: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn put64bit(mut ptr: *mut *mut uint8_t, mut val: uint64_t) {
    val = val.swap_bytes() as uint64_t;
    memcpy(
        *ptr as *mut ::core::ffi::c_void,
        &raw mut val as *const ::core::ffi::c_void,
        8 as size_t,
    );
    *ptr = (*ptr).offset(8 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn put32bit(mut ptr: *mut *mut uint8_t, mut val: uint32_t) {
    val = val.swap_bytes() as uint32_t;
    memcpy(
        *ptr as *mut ::core::ffi::c_void,
        &raw mut val as *const ::core::ffi::c_void,
        4 as size_t,
    );
    *ptr = (*ptr).offset(4 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn get64bit(mut ptr: *mut *const uint8_t) -> uint64_t {
    let mut t64: uint64_t = 0;
    memcpy(
        &raw mut t64 as *mut ::core::ffi::c_void,
        *ptr as *const ::core::ffi::c_void,
        8 as size_t,
    );
    *ptr = (*ptr).offset(8 as ::core::ffi::c_int as isize);
    return t64.swap_bytes();
}
#[inline]
unsafe extern "C" fn get32bit(mut ptr: *mut *const uint8_t) -> uint32_t {
    let mut t32: uint32_t = 0;
    memcpy(
        &raw mut t32 as *mut ::core::ffi::c_void,
        *ptr as *const ::core::ffi::c_void,
        4 as size_t,
    );
    *ptr = (*ptr).offset(4 as ::core::ffi::c_int as isize);
    return t32.swap_bytes();
}
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_ERRNO_SYSLOG_STDERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ROTATE_FLAG_BROADCAST: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ROTATE_FLAG_FOREGROUND: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const META_FILE_BUFFER_SIZE: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;
pub const MAXIDHOLE: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
pub const STORE_UNIT: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const STORE_TIMEOUT: ::core::ffi::c_int = 7200 as ::core::ffi::c_int / STORE_UNIT;
static mut metaversion: uint64_t = 0;
static mut metaid: uint64_t = 0;
static mut ignoreflag: uint8_t = 0 as uint8_t;
static mut allowautorestore: uint8_t = 0 as uint8_t;
static mut emptystart: uint8_t = 0 as uint8_t;
static mut verboselevel: uint8_t = 0 as uint8_t;
static mut lastsuccessfulstore: uint32_t = 0 as uint32_t;
static mut laststoretime: ::core::ffi::c_double = 0.0f64;
static mut laststorestatus: uint8_t = LASTSTORE_UNKNOWN as uint8_t;
static mut laststoremetaversion: uint64_t = 0 as uint64_t;
static mut laststorechecksum: uint32_t = 0 as uint32_t;
static mut BackMetaCopies: uint32_t = 0;
static mut MetaSaveFreq: uint32_t = 0;
static mut MetaCheckFreq: uint32_t = 0;
static mut MetaDownloadFreq: uint32_t = 0;
static mut MetaSaveOffset: uint32_t = 0;
static mut MetaSaveOffsetLocal: uint8_t = 0;
static mut metasaverpid: pid_t = -1 as pid_t;
static mut metasavermode: uint8_t = 0 as uint8_t;
static mut metasaverkilled: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut chlog_keep_head: *mut chlog_keep = ::core::ptr::null_mut::<chlog_keep>();
#[no_mangle]
pub unsafe extern "C" fn meta_chlog_keep_version() -> uint64_t {
    let mut ck: *mut chlog_keep = ::core::ptr::null_mut::<chlog_keep>();
    let mut ckp: *mut *mut chlog_keep = ::core::ptr::null_mut::<*mut chlog_keep>();
    let mut minversion: uint64_t = 0;
    let mut now: uint32_t = 0;
    minversion = metaversion;
    now = main_time();
    ckp = &raw mut chlog_keep_head;
    loop {
        ck = *ckp;
        if ck.is_null() {
            break;
        }
        if now > (*ck).validtime {
            *ckp = (*ck).next as *mut chlog_keep;
            free(ck as *mut ::core::ffi::c_void);
        } else {
            ckp = &raw mut (*ck).next as *mut *mut chlog_keep;
            if (*ck).version < minversion {
                minversion = (*ck).version;
            }
        }
    }
    return minversion;
}
unsafe extern "C" fn meta_chlog_keep_free() {
    let mut ck: *mut chlog_keep = ::core::ptr::null_mut::<chlog_keep>();
    let mut ckp: *mut *mut chlog_keep = ::core::ptr::null_mut::<*mut chlog_keep>();
    ckp = &raw mut chlog_keep_head;
    loop {
        ck = *ckp;
        if ck.is_null() {
            break;
        }
        *ckp = (*ck).next as *mut chlog_keep;
        free(ck as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn meta_store_chunk(
    mut fd: *mut bio,
    mut storefn: Option<unsafe extern "C" fn(*mut bio) -> uint8_t>,
    mut chunkname: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut hdr: [uint8_t; 16] = [0; 16];
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut mver: uint8_t = 0;
    let mut offbegin: uint64_t = 0 as uint64_t;
    let mut offend: uint64_t = 0;
    if storefn.is_none() {
        memcpy(
            &raw mut hdr as *mut uint8_t as *mut ::core::ffi::c_void,
            b"[MFS EOF MARKER]\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            16 as size_t,
        );
    } else {
        memcpy(
            &raw mut hdr as *mut uint8_t as *mut ::core::ffi::c_void,
            chunkname as *const ::core::ffi::c_void,
            4 as size_t,
        );
        mver = storefn.expect("non-null function pointer")(::core::ptr::null_mut::<bio>());
        hdr[4 as usize] = ' ' as uint8_t;
        hdr[5 as usize] = ('0' as ::core::ffi::c_int
            + (mver as ::core::ffi::c_int >> 4 as ::core::ffi::c_int & 0xf as ::core::ffi::c_int))
            as uint8_t;
        hdr[6 as usize] = '.' as uint8_t;
        hdr[7 as usize] = ('0' as ::core::ffi::c_int
            + (mver as ::core::ffi::c_int & 0xf as ::core::ffi::c_int))
            as uint8_t;
        offbegin = bio_file_position(fd);
        memset(
            (&raw mut hdr as *mut uint8_t).offset(8 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_void,
            0xff as ::core::ffi::c_int,
            8 as size_t,
        );
    }
    if bio_write(
        fd,
        &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
        16 as uint64_t,
    ) as size_t
        != 16 as ::core::ffi::c_int as size_t
    {
        return -1 as ::core::ffi::c_int;
    }
    if storefn.is_some() {
        storefn.expect("non-null function pointer")(fd);
        if bio_error(fd) != 0 {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"error writing section '%c%c%c%c'\0".as_ptr() as *const ::core::ffi::c_char,
                *chunkname.offset(0 as isize) as ::core::ffi::c_int,
                *chunkname.offset(1 as isize) as ::core::ffi::c_int,
                *chunkname.offset(2 as isize) as ::core::ffi::c_int,
                *chunkname.offset(3 as isize) as ::core::ffi::c_int,
            );
        }
        if offbegin != 0 as uint64_t {
            offend = bio_file_position(fd);
            ptr = (&raw mut hdr as *mut uint8_t).offset(8 as ::core::ffi::c_int as isize);
            put64bit(
                &raw mut ptr,
                offend.wrapping_sub(offbegin).wrapping_sub(16 as uint64_t),
            );
            bio_seek(
                fd,
                offbegin.wrapping_add(8 as uint64_t) as int64_t,
                SEEK_SET,
            );
            if bio_write(
                fd,
                (&raw mut hdr as *mut uint8_t).offset(8 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_void,
                8 as uint64_t,
            ) as size_t
                != 8 as ::core::ffi::c_int as size_t
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"error updating size of section '%c%c%c%c'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    *chunkname.offset(0 as isize) as ::core::ffi::c_int,
                    *chunkname.offset(1 as isize) as ::core::ffi::c_int,
                    *chunkname.offset(2 as isize) as ::core::ffi::c_int,
                    *chunkname.offset(3 as isize) as ::core::ffi::c_int,
                );
                return -1 as ::core::ffi::c_int;
            }
            bio_seek(fd, offend as int64_t, SEEK_SET);
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn meta_store(mut fd: *mut bio, mut crcfname: *const ::core::ffi::c_char) {
    let mut crcfd: *mut bio = ::core::ptr::null_mut::<bio>();
    let mut hdr: [uint8_t; 16] = [0; 16];
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    if !crcfname.is_null() {
        crcfd = bio_file_open(
            crcfname,
            BIO_WRITE as ::core::ffi::c_int as uint8_t,
            1024 as uint32_t,
        );
    } else {
        crcfd = ::core::ptr::null_mut::<bio>();
    }
    ptr = &raw mut hdr as *mut uint8_t;
    put64bit(&raw mut ptr, metaversion);
    put64bit(&raw mut ptr, metaid);
    if bio_write(
        fd,
        &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
        16 as uint64_t,
    ) as size_t
        != 16 as ::core::ffi::c_int as size_t
    {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"error writing metadata header\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    if !crcfd.is_null() {
        bio_write(
            crcfd,
            &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
            16 as uint64_t,
        );
    }
    if !crcfd.is_null() {
        ptr = &raw mut hdr as *mut uint8_t;
        memcpy(
            ptr as *mut ::core::ffi::c_void,
            b"HEAD\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        );
        ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
        put32bit(&raw mut ptr, bio_crc(fd));
        bio_write(
            crcfd,
            &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
            8 as uint64_t,
        );
    }
    if meta_store_chunk(
        fd,
        Some(sessions_store as unsafe extern "C" fn(*mut bio) -> uint8_t),
        b"SESS\0".as_ptr() as *const ::core::ffi::c_char,
    ) < 0 as ::core::ffi::c_int
    {
        return;
    }
    if !crcfd.is_null() {
        ptr = &raw mut hdr as *mut uint8_t;
        memcpy(
            ptr as *mut ::core::ffi::c_void,
            b"SESS\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        );
        ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
        put32bit(&raw mut ptr, bio_crc(fd));
        bio_write(
            crcfd,
            &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
            8 as uint64_t,
        );
    }
    if meta_store_chunk(
        fd,
        Some(sclass_store as unsafe extern "C" fn(*mut bio) -> uint8_t),
        b"SCLA\0".as_ptr() as *const ::core::ffi::c_char,
    ) < 0 as ::core::ffi::c_int
    {
        return;
    }
    if !crcfd.is_null() {
        ptr = &raw mut hdr as *mut uint8_t;
        memcpy(
            ptr as *mut ::core::ffi::c_void,
            b"SCLA\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        );
        ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
        put32bit(&raw mut ptr, bio_crc(fd));
        bio_write(
            crcfd,
            &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
            8 as uint64_t,
        );
    }
    if meta_store_chunk(
        fd,
        Some(patterns_store as unsafe extern "C" fn(*mut bio) -> uint8_t),
        b"PATT\0".as_ptr() as *const ::core::ffi::c_char,
    ) < 0 as ::core::ffi::c_int
    {
        return;
    }
    if !crcfd.is_null() {
        ptr = &raw mut hdr as *mut uint8_t;
        memcpy(
            ptr as *mut ::core::ffi::c_void,
            b"PATT\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        );
        ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
        put32bit(&raw mut ptr, bio_crc(fd));
        bio_write(
            crcfd,
            &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
            8 as uint64_t,
        );
    }
    if meta_store_chunk(
        fd,
        Some(fs_storenodes as unsafe extern "C" fn(*mut bio) -> uint8_t),
        b"NODE\0".as_ptr() as *const ::core::ffi::c_char,
    ) < 0 as ::core::ffi::c_int
    {
        return;
    }
    if !crcfd.is_null() {
        ptr = &raw mut hdr as *mut uint8_t;
        memcpy(
            ptr as *mut ::core::ffi::c_void,
            b"NODE\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        );
        ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
        put32bit(&raw mut ptr, bio_crc(fd));
        bio_write(
            crcfd,
            &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
            8 as uint64_t,
        );
    }
    if meta_store_chunk(
        fd,
        Some(fs_storeedges as unsafe extern "C" fn(*mut bio) -> uint8_t),
        b"EDGE\0".as_ptr() as *const ::core::ffi::c_char,
    ) < 0 as ::core::ffi::c_int
    {
        return;
    }
    if !crcfd.is_null() {
        ptr = &raw mut hdr as *mut uint8_t;
        memcpy(
            ptr as *mut ::core::ffi::c_void,
            b"EDGE\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        );
        ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
        put32bit(&raw mut ptr, bio_crc(fd));
        bio_write(
            crcfd,
            &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
            8 as uint64_t,
        );
    }
    if meta_store_chunk(
        fd,
        Some(fs_storefree as unsafe extern "C" fn(*mut bio) -> uint8_t),
        b"FREE\0".as_ptr() as *const ::core::ffi::c_char,
    ) < 0 as ::core::ffi::c_int
    {
        return;
    }
    if !crcfd.is_null() {
        ptr = &raw mut hdr as *mut uint8_t;
        memcpy(
            ptr as *mut ::core::ffi::c_void,
            b"FREE\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        );
        ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
        put32bit(&raw mut ptr, bio_crc(fd));
        bio_write(
            crcfd,
            &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
            8 as uint64_t,
        );
    }
    if meta_store_chunk(
        fd,
        Some(fs_storequota as unsafe extern "C" fn(*mut bio) -> uint8_t),
        b"QUOT\0".as_ptr() as *const ::core::ffi::c_char,
    ) < 0 as ::core::ffi::c_int
    {
        return;
    }
    if !crcfd.is_null() {
        ptr = &raw mut hdr as *mut uint8_t;
        memcpy(
            ptr as *mut ::core::ffi::c_void,
            b"QUOT\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        );
        ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
        put32bit(&raw mut ptr, bio_crc(fd));
        bio_write(
            crcfd,
            &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
            8 as uint64_t,
        );
    }
    if meta_store_chunk(
        fd,
        Some(xattr_store as unsafe extern "C" fn(*mut bio) -> uint8_t),
        b"XATR\0".as_ptr() as *const ::core::ffi::c_char,
    ) < 0 as ::core::ffi::c_int
    {
        return;
    }
    if !crcfd.is_null() {
        ptr = &raw mut hdr as *mut uint8_t;
        memcpy(
            ptr as *mut ::core::ffi::c_void,
            b"XATR\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        );
        ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
        put32bit(&raw mut ptr, bio_crc(fd));
        bio_write(
            crcfd,
            &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
            8 as uint64_t,
        );
    }
    if meta_store_chunk(
        fd,
        Some(posix_acl_store as unsafe extern "C" fn(*mut bio) -> uint8_t),
        b"PACL\0".as_ptr() as *const ::core::ffi::c_char,
    ) < 0 as ::core::ffi::c_int
    {
        return;
    }
    if !crcfd.is_null() {
        ptr = &raw mut hdr as *mut uint8_t;
        memcpy(
            ptr as *mut ::core::ffi::c_void,
            b"PACL\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        );
        ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
        put32bit(&raw mut ptr, bio_crc(fd));
        bio_write(
            crcfd,
            &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
            8 as uint64_t,
        );
    }
    if meta_store_chunk(
        fd,
        Some(of_store as unsafe extern "C" fn(*mut bio) -> uint8_t),
        b"OPEN\0".as_ptr() as *const ::core::ffi::c_char,
    ) < 0 as ::core::ffi::c_int
    {
        return;
    }
    if !crcfd.is_null() {
        ptr = &raw mut hdr as *mut uint8_t;
        memcpy(
            ptr as *mut ::core::ffi::c_void,
            b"OPEN\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        );
        ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
        put32bit(&raw mut ptr, bio_crc(fd));
        bio_write(
            crcfd,
            &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
            8 as uint64_t,
        );
    }
    if meta_store_chunk(
        fd,
        Some(flock_store as unsafe extern "C" fn(*mut bio) -> uint8_t),
        b"FLCK\0".as_ptr() as *const ::core::ffi::c_char,
    ) < 0 as ::core::ffi::c_int
    {
        return;
    }
    if !crcfd.is_null() {
        ptr = &raw mut hdr as *mut uint8_t;
        memcpy(
            ptr as *mut ::core::ffi::c_void,
            b"FLCK\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        );
        ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
        put32bit(&raw mut ptr, bio_crc(fd));
        bio_write(
            crcfd,
            &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
            8 as uint64_t,
        );
    }
    if meta_store_chunk(
        fd,
        Some(posix_lock_store as unsafe extern "C" fn(*mut bio) -> uint8_t),
        b"PLCK\0".as_ptr() as *const ::core::ffi::c_char,
    ) < 0 as ::core::ffi::c_int
    {
        return;
    }
    if !crcfd.is_null() {
        ptr = &raw mut hdr as *mut uint8_t;
        memcpy(
            ptr as *mut ::core::ffi::c_void,
            b"PLCK\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        );
        ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
        put32bit(&raw mut ptr, bio_crc(fd));
        bio_write(
            crcfd,
            &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
            8 as uint64_t,
        );
    }
    if meta_store_chunk(
        fd,
        Some(csdb_store as unsafe extern "C" fn(*mut bio) -> uint8_t),
        b"CSDB\0".as_ptr() as *const ::core::ffi::c_char,
    ) < 0 as ::core::ffi::c_int
    {
        return;
    }
    if !crcfd.is_null() {
        ptr = &raw mut hdr as *mut uint8_t;
        memcpy(
            ptr as *mut ::core::ffi::c_void,
            b"CSDB\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        );
        ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
        put32bit(&raw mut ptr, bio_crc(fd));
        bio_write(
            crcfd,
            &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
            8 as uint64_t,
        );
    }
    if meta_store_chunk(
        fd,
        Some(chunk_store as unsafe extern "C" fn(*mut bio) -> uint8_t),
        b"CHNK\0".as_ptr() as *const ::core::ffi::c_char,
    ) < 0 as ::core::ffi::c_int
    {
        return;
    }
    if !crcfd.is_null() {
        ptr = &raw mut hdr as *mut uint8_t;
        memcpy(
            ptr as *mut ::core::ffi::c_void,
            b"CHNK\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        );
        ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
        put32bit(&raw mut ptr, bio_crc(fd));
        bio_write(
            crcfd,
            &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
            8 as uint64_t,
        );
    }
    if meta_store_chunk(fd, None, ::core::ptr::null::<::core::ffi::c_char>())
        < 0 as ::core::ffi::c_int
    {
        return;
    }
    if !crcfd.is_null() {
        ptr = &raw mut hdr as *mut uint8_t;
        memcpy(
            ptr as *mut ::core::ffi::c_void,
            b"TAIL\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        );
        ptr = ptr.offset(4 as ::core::ffi::c_int as isize);
        put32bit(&raw mut ptr, bio_crc(fd));
        bio_write(
            crcfd,
            &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
            8 as uint64_t,
        );
    }
    if !crcfd.is_null() {
        bio_sync(crcfd);
        if bio_error(crcfd) != 0 {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"error writing metadata crc file\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        bio_close(crcfd);
    }
}
pub const META_CHECK_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const META_CHECK_NOFILE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const META_CHECK_IOERROR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const META_CHECK_BADHEADER: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const META_CHECK_BADENDING: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn meta_check_metadatafile(
    mut name: *const ::core::ffi::c_char,
    mut ver: *mut uint64_t,
    mut id: *mut uint64_t,
) -> uint8_t {
    let mut fd: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    let mut chkbuff: [uint8_t; 16] = [0; 16];
    let mut eofmark: [uint8_t; 16] = [0; 16];
    let mut fver: uint8_t = 0;
    let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
    *ver = 0 as uint64_t;
    *id = 0 as uint64_t;
    fd = open(name, O_RDONLY);
    if fd < 0 as ::core::ffi::c_int {
        if *__errno_location() == ENOENT {
            return META_CHECK_NOFILE as uint8_t;
        } else {
            return META_CHECK_IOERROR as uint8_t;
        }
    }
    if read(
        fd,
        &raw mut chkbuff as *mut uint8_t as *mut ::core::ffi::c_void,
        8 as size_t,
    ) != 8 as ssize_t
    {
        err = *__errno_location();
        close(fd);
        *__errno_location() = err;
        return META_CHECK_IOERROR as uint8_t;
    }
    if memcmp(
        &raw mut chkbuff as *mut uint8_t as *const ::core::ffi::c_void,
        b"MFSM NEW\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        8 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        close(fd);
        *ver = 1 as uint64_t;
        return META_CHECK_OK as uint8_t;
    }
    if memcmp(
        &raw mut chkbuff as *mut uint8_t as *const ::core::ffi::c_void,
        b"MFSM \0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        5 as size_t,
    ) == 0 as ::core::ffi::c_int
        && chkbuff[5 as usize] as ::core::ffi::c_int >= '1' as ::core::ffi::c_int
        && chkbuff[5 as usize] as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
        && chkbuff[6 as usize] as ::core::ffi::c_int == '.' as ::core::ffi::c_int
        && chkbuff[7 as usize] as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
        && chkbuff[7 as usize] as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
    {
        fver = (((chkbuff[5 as usize] as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int)
            + (chkbuff[7 as usize] as ::core::ffi::c_int - '0' as ::core::ffi::c_int))
            as uint8_t;
    } else {
        close(fd);
        return META_CHECK_BADHEADER as uint8_t;
    }
    if (fver as ::core::ffi::c_int) < 0x16 as ::core::ffi::c_int {
        memset(
            &raw mut eofmark as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            16 as size_t,
        );
    } else {
        memcpy(
            &raw mut eofmark as *mut uint8_t as *mut ::core::ffi::c_void,
            b"[MFS EOF MARKER]\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            16 as size_t,
        );
    }
    if read(
        fd,
        &raw mut chkbuff as *mut uint8_t as *mut ::core::ffi::c_void,
        16 as size_t,
    ) != 16 as ssize_t
    {
        err = *__errno_location();
        close(fd);
        *__errno_location() = err;
        return META_CHECK_IOERROR as uint8_t;
    }
    if (fver as ::core::ffi::c_int) < 0x20 as ::core::ffi::c_int {
        ptr = (&raw mut chkbuff as *mut uint8_t).offset(4 as ::core::ffi::c_int as isize);
        *ver = get64bit(&raw mut ptr);
    } else {
        ptr = &raw mut chkbuff as *mut uint8_t;
        *ver = get64bit(&raw mut ptr);
        *id = get64bit(&raw mut ptr);
    }
    lseek(fd, -16 as __off64_t, SEEK_END);
    if read(
        fd,
        &raw mut chkbuff as *mut uint8_t as *mut ::core::ffi::c_void,
        16 as size_t,
    ) != 16 as ssize_t
    {
        err = *__errno_location();
        close(fd);
        *__errno_location() = err;
        return META_CHECK_IOERROR as uint8_t;
    }
    close(fd);
    if memcmp(
        &raw mut chkbuff as *mut uint8_t as *const ::core::ffi::c_void,
        &raw mut eofmark as *mut uint8_t as *const ::core::ffi::c_void,
        16 as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        return META_CHECK_BADENDING as uint8_t;
    }
    return META_CHECK_OK as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn meta_load(
    mut fd: *mut bio,
    mut fver: uint8_t,
    mut afterload: *mut uint8_t,
) -> ::core::ffi::c_int {
    let mut hdr: [uint8_t; 16] = [0; 16];
    let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut offbegin: off_t = 0 as off_t;
    let mut sleng: uint64_t = 0;
    let mut maxnodeid: uint32_t = 0 as uint32_t;
    let mut nextsessionid: uint32_t = 0;
    let mut profdata: ::core::ffi::c_double = 0.;
    let mut mver: uint8_t = 0;
    if bio_read(
        fd,
        &raw mut hdr as *mut uint8_t as *mut ::core::ffi::c_void,
        16 as uint64_t,
    ) != 16 as int64_t
    {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"error loading metadata header\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    *afterload = 1 as uint8_t;
    ptr = &raw mut hdr as *mut uint8_t;
    if (fver as ::core::ffi::c_int) < 0x20 as ::core::ffi::c_int {
        sessions_import();
        maxnodeid = get32bit(&raw mut ptr);
        metaversion = get64bit(&raw mut ptr);
        nextsessionid = get32bit(&raw mut ptr);
        sessions_set_nextsessionid(nextsessionid);
        metaid = 0 as uint64_t;
    } else {
        metaversion = get64bit(&raw mut ptr);
        metaid = get64bit(&raw mut ptr);
    }
    if (fver as ::core::ffi::c_int) < 0x16 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"loading objects (files,directories,etc.) ... \0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        fflush(stderr);
        if fs_importnodes(fd, maxnodeid, ignoreflag as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        {
            fprintf(stderr, b"error\n\0".as_ptr() as *const ::core::ffi::c_char);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"error reading metadata (node)\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        fprintf(stderr, b"ok\n\0".as_ptr() as *const ::core::ffi::c_char);
        fprintf(
            stderr,
            b"loading names ... \0".as_ptr() as *const ::core::ffi::c_char,
        );
        fflush(stderr);
        if fs_loadedges(fd, 0x10 as uint8_t, ignoreflag as ::core::ffi::c_int)
            < 0 as ::core::ffi::c_int
        {
            fprintf(stderr, b"error\n\0".as_ptr() as *const ::core::ffi::c_char);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"error reading metadata (edge)\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        fprintf(stderr, b"ok\n\0".as_ptr() as *const ::core::ffi::c_char);
        fprintf(
            stderr,
            b"loading deletion timestamps ... \0".as_ptr() as *const ::core::ffi::c_char,
        );
        fflush(stderr);
        if fs_loadfree(fd, 0x10 as uint8_t, ignoreflag as ::core::ffi::c_int)
            < 0 as ::core::ffi::c_int
        {
            fprintf(stderr, b"error\n\0".as_ptr() as *const ::core::ffi::c_char);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"error reading metadata (free)\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        fprintf(stderr, b"ok\n\0".as_ptr() as *const ::core::ffi::c_char);
        fprintf(
            stderr,
            b"loading chunks data ... \0".as_ptr() as *const ::core::ffi::c_char,
        );
        fflush(stderr);
        if chunk_load(fd, 0x10 as uint8_t, ignoreflag as ::core::ffi::c_int)
            < 0 as ::core::ffi::c_int
        {
            fprintf(stderr, b"error\n\0".as_ptr() as *const ::core::ffi::c_char);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"error reading metadata (chunks)\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        fprintf(stderr, b"ok\n\0".as_ptr() as *const ::core::ffi::c_char);
    } else {
        loop {
            if bio_read(
                fd,
                &raw mut hdr as *mut uint8_t as *mut ::core::ffi::c_void,
                16 as uint64_t,
            ) != 16 as int64_t
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"error loading metadata section header\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            if memcmp(
                &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
                b"[MFS EOF MARKER]\0".as_ptr() as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                16 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                break;
            }
            ptr = (&raw mut hdr as *mut uint8_t).offset(8 as ::core::ffi::c_int as isize);
            sleng = get64bit(&raw mut ptr);
            if sleng < 0xffffffffffffffff as uint64_t {
                offbegin = bio_file_position(fd) as off_t;
            }
            profdata = monotonic_seconds();
            mver = (((hdr[5 as usize] as ::core::ffi::c_int - '0' as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int)
                + (hdr[7 as usize] as ::core::ffi::c_int - '0' as ::core::ffi::c_int
                    & 0xf as ::core::ffi::c_int)) as uint8_t;
            if memcmp(
                &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
                b"NODE\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                4 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                if (fver as ::core::ffi::c_int) < 0x20 as ::core::ffi::c_int {
                    fprintf(
                        stderr,
                        b"loading objects (files,directories,etc.) ... \0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    fflush(stderr);
                    if fs_importnodes(fd, maxnodeid, ignoreflag as ::core::ffi::c_int)
                        < 0 as ::core::ffi::c_int
                    {
                        fprintf(stderr, b"error\n\0".as_ptr() as *const ::core::ffi::c_char);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"error reading metadata (node)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                } else {
                    if mver as ::core::ffi::c_int
                        > fs_storenodes(::core::ptr::null_mut::<bio>()) as ::core::ffi::c_int
                    {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_ERR,
                            b"error reading metadata (node) - metadata in file have been stored by newer version of MFS !!!\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                    fprintf(
                        stderr,
                        b"loading objects (files,directories,etc.) ... \0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    fflush(stderr);
                    if fs_loadnodes(fd, mver, ignoreflag as ::core::ffi::c_int)
                        < 0 as ::core::ffi::c_int
                    {
                        fprintf(stderr, b"error\n\0".as_ptr() as *const ::core::ffi::c_char);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"error reading metadata (node)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                }
            } else if memcmp(
                &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
                b"EDGE\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                4 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                if mver as ::core::ffi::c_int
                    > fs_storeedges(::core::ptr::null_mut::<bio>()) as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"error reading metadata (edge) - metadata in file have been stored by newer version of MFS !!!\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                fprintf(
                    stderr,
                    b"loading names ... \0".as_ptr() as *const ::core::ffi::c_char,
                );
                fflush(stderr);
                if fs_loadedges(fd, mver, ignoreflag as ::core::ffi::c_int)
                    < 0 as ::core::ffi::c_int
                {
                    fprintf(stderr, b"error\n\0".as_ptr() as *const ::core::ffi::c_char);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"error reading metadata (edge)\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            } else if memcmp(
                &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
                b"FREE\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                4 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                if mver as ::core::ffi::c_int
                    > fs_storefree(::core::ptr::null_mut::<bio>()) as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"error reading metadata (free) - metadata in file have been stored by newer version of MFS !!!\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                fprintf(
                    stderr,
                    b"loading deletion timestamps ... \0".as_ptr() as *const ::core::ffi::c_char,
                );
                fflush(stderr);
                if fs_loadfree(fd, mver, ignoreflag as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                {
                    fprintf(stderr, b"error\n\0".as_ptr() as *const ::core::ffi::c_char);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"error reading metadata (free)\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            } else if memcmp(
                &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
                b"QUOT\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                4 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                if mver as ::core::ffi::c_int
                    > fs_storequota(::core::ptr::null_mut::<bio>()) as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"error reading metadata (quota) - metadata in file have been stored by newer version of MFS !!!\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                fprintf(
                    stderr,
                    b"loading quota definitions ... \0".as_ptr() as *const ::core::ffi::c_char,
                );
                fflush(stderr);
                if fs_loadquota(fd, mver, ignoreflag as ::core::ffi::c_int)
                    < 0 as ::core::ffi::c_int
                {
                    fprintf(stderr, b"error\n\0".as_ptr() as *const ::core::ffi::c_char);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"error reading metadata (quota)\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            } else if memcmp(
                &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
                b"XATR\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                4 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                if mver as ::core::ffi::c_int
                    > xattr_store(::core::ptr::null_mut::<bio>()) as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"error reading metadata (xattr) - metadata in file have been stored by newer version of MFS !!!\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                fprintf(
                    stderr,
                    b"loading xattr data ... \0".as_ptr() as *const ::core::ffi::c_char,
                );
                fflush(stderr);
                if xattr_load(fd, mver, ignoreflag as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                {
                    fprintf(stderr, b"error\n\0".as_ptr() as *const ::core::ffi::c_char);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"error reading metadata (xattr)\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            } else if memcmp(
                &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
                b"PACL\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                4 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                if mver as ::core::ffi::c_int
                    > posix_acl_store(::core::ptr::null_mut::<bio>()) as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"error reading metadata (posix_acl) - metadata in file have been stored by newer version of MFS !!!\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                fprintf(
                    stderr,
                    b"loading posix_acl data ... \0".as_ptr() as *const ::core::ffi::c_char,
                );
                fflush(stderr);
                if posix_acl_load(fd, mver, ignoreflag as ::core::ffi::c_int)
                    < 0 as ::core::ffi::c_int
                {
                    fprintf(stderr, b"error\n\0".as_ptr() as *const ::core::ffi::c_char);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"error reading metadata (posix_acl)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            } else if memcmp(
                &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
                b"FLCK\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                4 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                if mver as ::core::ffi::c_int
                    > flock_store(::core::ptr::null_mut::<bio>()) as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"error reading metadata (flock_locks) - metadata in file have been stored by newer version of MFS !!!\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                fprintf(
                    stderr,
                    b"loading flock_locks data ... \0".as_ptr() as *const ::core::ffi::c_char,
                );
                fflush(stderr);
                if flock_load(fd, mver, ignoreflag) < 0 as ::core::ffi::c_int {
                    fprintf(stderr, b"error\n\0".as_ptr() as *const ::core::ffi::c_char);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"error reading metadata (flock_locks)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            } else if memcmp(
                &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
                b"PLCK\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                4 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                if mver as ::core::ffi::c_int
                    > posix_lock_store(::core::ptr::null_mut::<bio>()) as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"error reading metadata (posix_locks) - metadata in file have been stored by newer version of MFS !!!\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                fprintf(
                    stderr,
                    b"loading posix_locks data ... \0".as_ptr() as *const ::core::ffi::c_char,
                );
                fflush(stderr);
                if posix_lock_load(fd, mver, ignoreflag) < 0 as ::core::ffi::c_int {
                    fprintf(stderr, b"error\n\0".as_ptr() as *const ::core::ffi::c_char);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"error reading metadata (posix_locks)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            } else if memcmp(
                &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
                b"CSDB\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                4 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                if mver as ::core::ffi::c_int
                    > csdb_store(::core::ptr::null_mut::<bio>()) as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"error reading metadata (csdb) - metadata in file have been stored by newer version of MFS !!!\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                fprintf(
                    stderr,
                    b"loading chunkservers data ... \0".as_ptr() as *const ::core::ffi::c_char,
                );
                fflush(stderr);
                if csdb_load(fd, mver, ignoreflag as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                    fprintf(stderr, b"error\n\0".as_ptr() as *const ::core::ffi::c_char);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"error reading metadata (csdb)\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            } else if memcmp(
                &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
                b"SESS\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                4 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                if mver as ::core::ffi::c_int
                    > sessions_store(::core::ptr::null_mut::<bio>()) as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"error reading metadata (sessions) - metadata in file have been stored by newer version of MFS !!!\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                fprintf(
                    stderr,
                    b"loading sessions data ... \0".as_ptr() as *const ::core::ffi::c_char,
                );
                fflush(stderr);
                if sessions_load(fd, mver) < 0 as ::core::ffi::c_int {
                    fprintf(stderr, b"error\n\0".as_ptr() as *const ::core::ffi::c_char);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"error reading metadata (sessions)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            } else if memcmp(
                &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
                b"LABS\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                4 as size_t,
            ) == 0 as ::core::ffi::c_int
                || memcmp(
                    &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
                    b"SCLA\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                    4 as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                if mver as ::core::ffi::c_int
                    > sclass_store(::core::ptr::null_mut::<bio>()) as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"error reading metadata (storage classes) - metadata in file have been stored by newer version of MFS !!!\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                fprintf(
                    stderr,
                    b"loading storage classes data ... \0".as_ptr() as *const ::core::ffi::c_char,
                );
                fflush(stderr);
                if sclass_load(fd, mver, ignoreflag as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                {
                    fprintf(stderr, b"error\n\0".as_ptr() as *const ::core::ffi::c_char);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"error reading metadata (storage classes)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            } else if memcmp(
                &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
                b"PATT\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                4 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                if mver as ::core::ffi::c_int
                    > patterns_store(::core::ptr::null_mut::<bio>()) as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"error reading metadata (patterns) - metadata in file have been stored by newer version of MFS !!!\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                fprintf(
                    stderr,
                    b"loading patterns data ... \0".as_ptr() as *const ::core::ffi::c_char,
                );
                fflush(stderr);
                if patterns_load(fd, mver, ignoreflag as ::core::ffi::c_int)
                    < 0 as ::core::ffi::c_int
                {
                    fprintf(stderr, b"error\n\0".as_ptr() as *const ::core::ffi::c_char);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"error reading metadata (patterns)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            } else if memcmp(
                &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
                b"OPEN\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                4 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                if mver as ::core::ffi::c_int
                    > of_store(::core::ptr::null_mut::<bio>()) as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"error reading metadata (open files) - metadata in file have been stored by newer version of MFS !!!\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                fprintf(
                    stderr,
                    b"loading open files data ... \0".as_ptr() as *const ::core::ffi::c_char,
                );
                fflush(stderr);
                if of_load(fd, mver) < 0 as ::core::ffi::c_int {
                    fprintf(stderr, b"error\n\0".as_ptr() as *const ::core::ffi::c_char);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"error reading metadata (open files)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            } else if memcmp(
                &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
                b"CHNK\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                4 as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                if mver as ::core::ffi::c_int
                    > chunk_store(::core::ptr::null_mut::<bio>()) as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"error reading metadata (chunks) - metadata in file have been stored by newer version of MFS !!!\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                fprintf(
                    stderr,
                    b"loading chunks data ... \0".as_ptr() as *const ::core::ffi::c_char,
                );
                fflush(stderr);
                if chunk_load(fd, mver, ignoreflag as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                {
                    fprintf(stderr, b"error\n\0".as_ptr() as *const ::core::ffi::c_char);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"error reading metadata (chunks)\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                *afterload = chunk_is_afterload_needed(mver);
            } else {
                hdr[8 as usize] = 0 as uint8_t;
                if ignoreflag != 0 {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_NOTICE,
                        b"unknown section found (leng:%lu,name:%s) - all data from this section will be lost !!!\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        sleng,
                        &raw mut hdr as *mut uint8_t,
                    );
                    bio_skip(fd, sleng);
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"error: unknown section found (leng:%lu,name:%s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        sleng,
                        &raw mut hdr as *mut uint8_t,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            }
            profdata = monotonic_seconds() - profdata;
            if sleng < 0xffffffffffffffff as uint64_t {
                if offbegin >= 0 as off_t
                    && (offbegin as uint64_t).wrapping_add(sleng) != bio_file_position(fd)
                {
                    if ignoreflag != 0 {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_NOTICE,
                            b"not all section has been read - file corrupted - ignoring\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_ERR,
                            b"not all section has been read - file corrupted\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                }
            }
            fprintf(
                stderr,
                b"ok (%.4lf)\n\0".as_ptr() as *const ::core::ffi::c_char,
                profdata,
            );
        }
    }
    return fs_check_consistency(ignoreflag as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn meta_file_storeall(
    mut fname: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut fd: *mut bio = ::core::ptr::null_mut::<bio>();
    fd = bio_file_open(
        fname,
        BIO_WRITE as ::core::ffi::c_int as uint8_t,
        META_FILE_BUFFER_SIZE as uint32_t,
    );
    if fd.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    if bio_write(
        fd,
        b"MFSM 2.0\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        8 as uint64_t,
    ) as size_t
        != 8 as ::core::ffi::c_int as size_t
    {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"error writing metadata signature in emergency mode, file name: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            fname,
        );
    } else {
        meta_store(fd, ::core::ptr::null::<::core::ffi::c_char>());
    }
    bio_sync(fd);
    if bio_error(fd) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"can't write metadata in emergency mode, file name: %s\0".as_ptr()
                as *const ::core::ffi::c_char,
            fname,
        );
        bio_close(fd);
        return -1 as ::core::ffi::c_int;
    }
    mfs_log(
        MFSLOG_SYSLOG,
        MFSLOG_WARNING,
        b"metadata file stored in emergency mode, file name: %s\0".as_ptr()
            as *const ::core::ffi::c_char,
        fname,
    );
    bio_close(fd);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub static mut meta_emergency_locations: [*const ::core::ffi::c_char; 9] = [
    b"/metadata.mfs.emergency\0".as_ptr() as *const ::core::ffi::c_char,
    b"/tmp/metadata.mfs.emergency\0".as_ptr() as *const ::core::ffi::c_char,
    b"/var/metadata.mfs.emergency\0".as_ptr() as *const ::core::ffi::c_char,
    b"/usr/metadata.mfs.emergency\0".as_ptr() as *const ::core::ffi::c_char,
    b"/usr/share/metadata.mfs.emergency\0".as_ptr() as *const ::core::ffi::c_char,
    b"/usr/local/metadata.mfs.emergency\0".as_ptr() as *const ::core::ffi::c_char,
    b"/usr/local/var/metadata.mfs.emergency\0".as_ptr() as *const ::core::ffi::c_char,
    b"/usr/local/share/metadata.mfs.emergency\0".as_ptr() as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
#[no_mangle]
pub unsafe extern "C" fn meta_create_homedir_emergency_filename() -> *mut ::core::ffi::c_char {
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn meta_emergency_saves() -> ::core::ffi::c_int {
    let mut hfname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fname: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    if meta_file_storeall(b"metadata.mfs.emergency\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    hfname = meta_create_homedir_emergency_filename();
    if !hfname.is_null() {
        if meta_file_storeall(hfname) == 0 as ::core::ffi::c_int {
            free(hfname as *mut ::core::ffi::c_void);
            return 0 as ::core::ffi::c_int;
        }
        free(hfname as *mut ::core::ffi::c_void);
    }
    i = 0 as ::core::ffi::c_int;
    loop {
        let c2rust_fresh1 = i;
        i += 1;
        fname = meta_emergency_locations[c2rust_fresh1 as usize];
        if fname.is_null() {
            break;
        }
        if meta_file_storeall(fname) == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
    }
    return -1 as ::core::ffi::c_int;
}
static mut storestarttime: ::core::ffi::c_double = 0.;
#[no_mangle]
pub unsafe extern "C" fn meta_process_crcdata() {
    let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut crcmetaversion: uint64_t = 0;
    let mut crcmetaid: uint64_t = 0;
    let mut crcdata: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut crcdatasize: int64_t = 0;
    let mut crcfd: *mut bio = ::core::ptr::null_mut::<bio>();
    crcfd = bio_file_open(
        b"metadata.crc\0".as_ptr() as *const ::core::ffi::c_char,
        BIO_READ as ::core::ffi::c_int as uint8_t,
        1024 as uint32_t,
    );
    if crcfd.is_null() {
        crcdata = ::core::ptr::null_mut::<uint8_t>();
        crcdatasize = 0 as int64_t;
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"can't process crc file\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        crcdatasize = bio_file_size(crcfd) as int64_t;
        crcdata = malloc(crcdatasize as size_t) as *mut uint8_t;
        if crcdata.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/metadata.c\0".as_ptr() as *const ::core::ffi::c_char,
                767 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"crcdata\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/metadata.c\0".as_ptr() as *const ::core::ffi::c_char,
                767 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"crcdata\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if crcdata
            == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut uint8_t
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/metadata.c\0".as_ptr() as *const ::core::ffi::c_char,
                767 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"crcdata\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/metadata.c\0".as_ptr() as *const ::core::ffi::c_char,
                767 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"crcdata\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        if bio_read(
            crcfd,
            crcdata as *mut ::core::ffi::c_void,
            crcdatasize as uint64_t,
        ) != crcdatasize
        {
            free(crcdata as *mut ::core::ffi::c_void);
            crcdata = ::core::ptr::null_mut::<uint8_t>();
            crcdatasize = 0 as int64_t;
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"can't process crc file\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        bio_close(crcfd);
    }
    if crcdatasize >= 16 as int64_t {
        rptr = crcdata;
        crcmetaversion = get64bit(&raw mut rptr);
        crcmetaid = get64bit(&raw mut rptr);
        if crcmetaid != metaid {
            laststoremetaversion = 0 as uint64_t;
            laststorechecksum = 0 as uint32_t;
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"wrong metaid - ignoring crc data\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            laststoremetaversion = crcmetaversion;
            laststorechecksum = mycrc32(
                0 as uint32_t,
                rptr as *const ::core::ffi::c_void,
                (crcdatasize - 16 as int64_t) as uint32_t,
            );
        }
    }
    free(crcdata as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn meta_storeended(mut pid: pid_t, mut status: ::core::ffi::c_int) {
    let mut chstatus: ::core::ffi::c_int = 0;
    if metasaverpid != pid {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"pid mismatch - saver pid: %d, sigchld pid: %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            metasaverpid,
            pid,
        );
    }
    if metasaverkilled != 0 {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_NOTICE,
            b"store process has been killed due to termination\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        unlink(b"metadata.mfs.back.tmp\0".as_ptr() as *const ::core::ffi::c_char);
        unlink(b"metadata.crc\0".as_ptr() as *const ::core::ffi::c_char);
        chstatus = 0 as ::core::ffi::c_int;
        storestarttime = 0.0f64;
    } else {
        if storestarttime > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            laststoretime = monotonic_seconds() - storestarttime;
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_INFO,
                b"store process has finished - store time: %.3lf seconds\0".as_ptr()
                    as *const ::core::ffi::c_char,
                laststoretime,
            );
        } else {
            laststoretime = 0.0f64;
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_NOTICE,
                b"store process has finished - unknown store time\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if status & 0x7f as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            chstatus = (status & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int;
        } else {
            chstatus = 3 as ::core::ffi::c_int;
        }
        if chstatus == 1 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"metadata stored in emergency mode (in non-standard location) - exiting\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            main_exit();
        } else if chstatus == 2 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"metadata not stored !!! (child exited) - exiting\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            main_exit();
        } else if chstatus == 3 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"metadata not stored !!! (child was signaled) - exiting\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            main_exit();
        } else if chstatus != 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"metadata not stored !!! (unknown status) - exiting\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            main_exit();
        } else {
            storestarttime = 0.0f64;
            laststorestatus = (if metasavermode as ::core::ffi::c_int != 0 {
                LASTSTORE_CRC_STORED_BG
            } else {
                LASTSTORE_META_STORED_BG
            }) as uint8_t;
            lastsuccessfulstore = main_time();
            meta_process_crcdata();
        }
    }
    metasaverpid = -1 as ::core::ffi::c_int as pid_t;
    metasavermode = 0 as uint8_t;
    metasaverkilled = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn meta_storeall(
    mut bg: ::core::ffi::c_int,
    mut dontstore: uint8_t,
) -> ::core::ffi::c_int {
    let mut fd: *mut bio = ::core::ptr::null_mut::<bio>();
    let mut i: ::core::ffi::c_int = 0;
    let mut estat: ::core::ffi::c_int = 0;
    let mut mfd: ::core::ffi::c_int = 0;
    let mut pfd: [::core::ffi::c_int; 2] = [0; 2];
    if metaversion == 0 as uint64_t {
        return 2 as ::core::ffi::c_int;
    }
    if metasaverpid >= 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"previous metadata save process hasn't finished yet - do not start another one\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if dontstore as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        mfd = open(
            b"metadata.mfs.back.tmp\0".as_ptr() as *const ::core::ffi::c_char,
            O_RDWR,
        );
        if mfd >= 0 as ::core::ffi::c_int {
            if lockf(mfd, F_TEST, 0 as __off64_t) < 0 as ::core::ffi::c_int {
                if *__errno_location() != EACCES
                    && *__errno_location() != EAGAIN
                    && *__errno_location() != EWOULDBLOCK
                {
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"metadata store lockf error\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"previous metadata save process hasn't finished yet - do not start another one\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                close(mfd);
                return -1 as ::core::ffi::c_int;
            }
            close(mfd);
        }
    }
    if bg != 0 {
        if pipe(&raw mut pfd as *mut ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            pfd[0 as usize] = -1 as ::core::ffi::c_int;
            pfd[1 as usize] = -1 as ::core::ffi::c_int;
        }
        i = fork() as ::core::ffi::c_int;
        if i < 0 as ::core::ffi::c_int {
            if dontstore as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"fork error (store data in foreground - it will block master for a while - check /proc/sys/vm/overcommit_memory and if necessary set to 1)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        } else if i == 0 as ::core::ffi::c_int {
            matocsserv_close_lsock();
            matoclserv_close_lsock();
            matomlserv_close_lsock();
            processname_set(
                b"mfsmaster (metadata saver)\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
    } else {
        i = -1 as ::core::ffi::c_int;
    }
    if i <= 0 as ::core::ffi::c_int {
        if i == 0 as ::core::ffi::c_int {
            let mut c: ::core::ffi::c_char = 0;
            if read(
                pfd[0 as usize],
                &raw mut c as *mut ::core::ffi::c_void,
                1 as size_t,
            ) != 1 as ssize_t
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"metadata store, pipe read error\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            close(pfd[0 as usize]);
            close(pfd[1 as usize]);
        } else {
            storestarttime = monotonic_seconds();
        }
        if dontstore != 0 {
            fd = bio_null_open(BIO_WRITE as ::core::ffi::c_int as uint8_t);
        } else {
            fd = bio_file_open(
                b"metadata.mfs.back.tmp\0".as_ptr() as *const ::core::ffi::c_char,
                BIO_WRITE as ::core::ffi::c_int as uint8_t,
                META_FILE_BUFFER_SIZE as uint32_t,
            );
        }
        if fd.is_null() {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"metadata store child - open error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if dontstore as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                estat = meta_emergency_saves();
                if i == 0 as ::core::ffi::c_int {
                    if estat < 0 as ::core::ffi::c_int {
                        exit(2 as ::core::ffi::c_int);
                    } else {
                        exit(1 as ::core::ffi::c_int);
                    }
                }
            }
            return 0 as ::core::ffi::c_int;
        }
        if i == 0 as ::core::ffi::c_int
            && dontstore as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            mfd = bio_descriptor(fd);
            if lockf(mfd, F_TLOCK, 0 as __off64_t) < 0 as ::core::ffi::c_int {
                if *__errno_location() != EACCES
                    && *__errno_location() != EAGAIN
                    && *__errno_location() != EWOULDBLOCK
                {
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"metadata store child - lockf error\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"metadata store child process - file is already locked !!!\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                bio_close(fd);
                estat = meta_emergency_saves();
                if estat < 0 as ::core::ffi::c_int {
                    exit(2 as ::core::ffi::c_int);
                } else {
                    exit(1 as ::core::ffi::c_int);
                }
            }
        }
        if bio_write(
            fd,
            b"MFSM 2.0\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            8 as uint64_t,
        ) as size_t
            != 8 as ::core::ffi::c_int as size_t
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_NOTICE,
                b"error writing metadata signature\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            meta_store(fd, b"metadata.crc\0".as_ptr() as *const ::core::ffi::c_char);
        }
        bio_sync(fd);
        if bio_error(fd) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"can't write metadata\0".as_ptr() as *const ::core::ffi::c_char,
            );
            bio_close(fd);
            if dontstore as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                unlink(b"metadata.mfs.back.tmp\0".as_ptr() as *const ::core::ffi::c_char);
            }
            unlink(b"metadata.crc\0".as_ptr() as *const ::core::ffi::c_char);
            if dontstore as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                estat = meta_emergency_saves();
                if i == 0 as ::core::ffi::c_int {
                    if estat < 0 as ::core::ffi::c_int {
                        exit(2 as ::core::ffi::c_int);
                    } else {
                        exit(1 as ::core::ffi::c_int);
                    }
                }
            }
            return 0 as ::core::ffi::c_int;
        } else {
            bio_close(fd);
            if dontstore as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if BackMetaCopies > 0 as uint32_t {
                    let mut metaname1: [::core::ffi::c_char; 100] = [0; 100];
                    let mut metaname2: [::core::ffi::c_char; 100] = [0; 100];
                    let mut n: ::core::ffi::c_int = 0;
                    n = BackMetaCopies.wrapping_sub(1 as uint32_t) as ::core::ffi::c_int;
                    while n > 0 as ::core::ffi::c_int {
                        snprintf(
                            &raw mut metaname1 as *mut ::core::ffi::c_char,
                            100 as size_t,
                            b"metadata.mfs.back.%u\0".as_ptr() as *const ::core::ffi::c_char,
                            n + 1 as ::core::ffi::c_int,
                        );
                        snprintf(
                            &raw mut metaname2 as *mut ::core::ffi::c_char,
                            100 as size_t,
                            b"metadata.mfs.back.%u\0".as_ptr() as *const ::core::ffi::c_char,
                            n,
                        );
                        rename(
                            &raw mut metaname2 as *mut ::core::ffi::c_char,
                            &raw mut metaname1 as *mut ::core::ffi::c_char,
                        );
                        n -= 1;
                    }
                    rename(
                        b"metadata.mfs.back\0".as_ptr() as *const ::core::ffi::c_char,
                        b"metadata.mfs.back.1\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                rename(
                    b"metadata.mfs.back.tmp\0".as_ptr() as *const ::core::ffi::c_char,
                    b"metadata.mfs.back\0".as_ptr() as *const ::core::ffi::c_char,
                );
                unlink(b"metadata.mfs\0".as_ptr() as *const ::core::ffi::c_char);
            }
        }
        if i == 0 as ::core::ffi::c_int {
            exit(0 as ::core::ffi::c_int);
        } else {
            lastsuccessfulstore = time(::core::ptr::null_mut::<time_t>()) as uint32_t;
            laststoretime = monotonic_seconds() - storestarttime;
            storestarttime = 0.0f64;
            laststorestatus = LASTSTORE_META_STORED_FG as uint8_t;
            meta_process_crcdata();
        }
    } else {
        storestarttime = monotonic_seconds();
        main_chld_register_fname(
            i as pid_t,
            Some(meta_storeended as unsafe extern "C" fn(pid_t, ::core::ffi::c_int) -> ()),
            b"meta_storeended\0".as_ptr() as *const ::core::ffi::c_char,
        );
        metasaverpid = i as pid_t;
        metasavermode = dontstore;
        metasaverkilled = 0 as ::core::ffi::c_int;
        if write(
            pfd[1 as usize],
            b"x\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            1 as size_t,
        ) != 1 as ssize_t
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"metadata store, pipe read error\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        close(pfd[0 as usize]);
        close(pfd[1 as usize]);
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn meta_download_status(mut status: uint8_t) {
    if storestarttime > 0.0f64 {
        laststoretime = monotonic_seconds() - storestarttime;
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_INFO,
            b"end of downloading metadata - download time: %.3lf\0".as_ptr()
                as *const ::core::ffi::c_char,
            laststoretime,
        );
    } else {
        laststoretime = 0.0f64;
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_NOTICE,
            b"end of downloading metadata - unknown download time\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    storestarttime = 0.0f64;
    if status as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        laststorestatus = LASTSTORE_DOWNLOADED as uint8_t;
        lastsuccessfulstore = main_time();
        laststoremetaversion = 0 as uint64_t;
        laststorechecksum = 0 as uint32_t;
    } else if meta_storeall(1 as ::core::ffi::c_int, 0 as uint8_t) <= 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"can't download metadata and can't store them - exiting\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        main_exit();
    }
}
static mut last_store_htime: uint32_t = 0 as uint32_t;
#[no_mangle]
pub unsafe extern "C" fn meta_store_task() {
    let mut curtime: uint32_t = 0;
    let mut htime: uint32_t = 0;
    let mut offset: uint32_t = 0;
    let mut rhtime: uint32_t = 0;
    let mut t: time_t = 0;
    let mut lt: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    curtime = main_time();
    rhtime = curtime.wrapping_div(STORE_UNIT as uint32_t);
    if MetaSaveOffsetLocal != 0 {
        t = curtime as time_t;
        memset(
            &raw mut lt as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<tm>(),
        );
        localtime_r(&raw mut t, &raw mut lt);
        if lt.tm_gmtoff >= 0 as ::core::ffi::c_long {
            offset = ((MetaSaveOffset
                .wrapping_add((24 as ::core::ffi::c_int * 60 as ::core::ffi::c_int) as uint32_t)
                as ::core::ffi::c_long
                - lt.tm_gmtoff / 60 as ::core::ffi::c_long)
                % (24 as ::core::ffi::c_int * 60 as ::core::ffi::c_int) as ::core::ffi::c_long)
                as uint32_t;
        } else {
            offset = ((MetaSaveOffset as ::core::ffi::c_long
                + lt.tm_gmtoff / 60 as ::core::ffi::c_long)
                % (24 as ::core::ffi::c_int * 60 as ::core::ffi::c_int) as ::core::ffi::c_long)
                as uint32_t;
        }
    } else {
        offset = MetaSaveOffset;
    }
    htime = curtime
        .wrapping_div(STORE_UNIT as uint32_t)
        .wrapping_sub(offset);
    if rhtime.wrapping_rem(60 as uint32_t) == 0 as uint32_t {
        changelog_rotate(ROTATE_FLAG_BROADCAST as uint8_t);
    }
    if htime.wrapping_rem(MetaSaveFreq) == 0 as uint32_t {
        if metasaverpid >= 0 as ::core::ffi::c_int
            && last_store_htime.wrapping_add(STORE_TIMEOUT as uint32_t) > rhtime
        {
            return;
        }
        last_store_htime = rhtime;
        if meta_storeall(1 as ::core::ffi::c_int, 0 as uint8_t) <= 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"can't store metadata - exiting\0".as_ptr() as *const ::core::ffi::c_char,
            );
            main_exit();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn meta_do_store_metadata() {
    if meta_storeall(1 as ::core::ffi::c_int, 0 as uint8_t) <= 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"can't store metadata\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        last_store_htime = main_time().wrapping_div(STORE_UNIT as uint32_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn meta_cleanup() {
    mfs_log(
        MFSLOG_SYSLOG_STDERR,
        MFSLOG_INFO,
        b"cleaning metadata ...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"cleaning fs data ...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fflush(stderr);
    fs_cleanup();
    fprintf(stderr, b"done\n\0".as_ptr() as *const ::core::ffi::c_char);
    fprintf(
        stderr,
        b"cleaning chunks data ...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fflush(stderr);
    chunk_cleanup();
    fprintf(stderr, b"done\n\0".as_ptr() as *const ::core::ffi::c_char);
    fprintf(
        stderr,
        b"cleaning xattr data ...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fflush(stderr);
    xattr_cleanup();
    fprintf(stderr, b"done\n\0".as_ptr() as *const ::core::ffi::c_char);
    fprintf(
        stderr,
        b"cleaning posix_acl data ...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fflush(stderr);
    posix_acl_cleanup();
    fprintf(stderr, b"done\n\0".as_ptr() as *const ::core::ffi::c_char);
    fprintf(
        stderr,
        b"cleaning flock locks data ...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fflush(stderr);
    flock_cleanup();
    fprintf(stderr, b"done\n\0".as_ptr() as *const ::core::ffi::c_char);
    fprintf(
        stderr,
        b"cleaning posix locks data ...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fflush(stderr);
    posix_lock_cleanup();
    fprintf(stderr, b"done\n\0".as_ptr() as *const ::core::ffi::c_char);
    fprintf(
        stderr,
        b"cleaning chunkservers data ...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fflush(stderr);
    csdb_cleanup();
    fprintf(stderr, b"done\n\0".as_ptr() as *const ::core::ffi::c_char);
    fprintf(
        stderr,
        b"cleaning open files data ...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fflush(stderr);
    of_cleanup();
    fprintf(stderr, b"done\n\0".as_ptr() as *const ::core::ffi::c_char);
    fprintf(
        stderr,
        b"cleaning sessions data ...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fflush(stderr);
    sessions_cleanup();
    fprintf(stderr, b"done\n\0".as_ptr() as *const ::core::ffi::c_char);
    fprintf(
        stderr,
        b"cleaning storage classes data ...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fflush(stderr);
    sclass_cleanup();
    fprintf(stderr, b"done\n\0".as_ptr() as *const ::core::ffi::c_char);
    fprintf(
        stderr,
        b"cleaning patterns data ...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fflush(stderr);
    patterns_cleanup();
    fprintf(stderr, b"done\n\0".as_ptr() as *const ::core::ffi::c_char);
    fprintf(
        stderr,
        b"cleaning dictionary data ...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fflush(stderr);
    dict_cleanup();
    fprintf(stderr, b"done\n\0".as_ptr() as *const ::core::ffi::c_char);
    metaversion = 0 as uint64_t;
    mfs_log(
        MFSLOG_SYSLOG_STDERR,
        MFSLOG_INFO,
        b"metadata have been cleaned\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn meta_mayexit() -> ::core::ffi::c_int {
    if metasaverpid > 0 as ::core::ffi::c_int {
        if metasaverkilled == 0 as ::core::ffi::c_int {
            if kill(metasaverpid as __pid_t, SIGKILL) < 0 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"can't kill meta saver process\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                metasaverkilled = 1 as ::core::ffi::c_int;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn meta_term() {
    changelog_rotate(ROTATE_FLAG_FOREGROUND as uint8_t);
    loop {
        let mut status: uint8_t = 0;
        status = meta_storeall(0 as ::core::ffi::c_int, 0 as uint8_t) as uint8_t;
        if status as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            if rename(
                b"metadata.mfs.back\0".as_ptr() as *const ::core::ffi::c_char,
                b"metadata.mfs\0".as_ptr() as *const ::core::ffi::c_char,
            ) < 0 as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"can't rename metadata.mfs.back -> metadata.mfs\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            meta_cleanup();
            meta_chlog_keep_free();
            return;
        } else if status as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_NOTICE,
                b"no metadata to store\0".as_ptr() as *const ::core::ffi::c_char,
            );
            meta_chlog_keep_free();
            return;
        }
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"can't store metadata - try to make more space on your hdd or change privileges - retrying after 10 seconds\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        sleep(10 as ::core::ffi::c_uint);
    }
}
#[no_mangle]
pub unsafe extern "C" fn meta_loadfile(
    mut filename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut fd: *mut bio = ::core::ptr::null_mut::<bio>();
    let mut fver: uint8_t = 0;
    let mut al: uint8_t = 0;
    let mut hdr: [uint8_t; 8] = [0; 8];
    al = 0 as uint8_t;
    fd = bio_file_open(
        filename,
        BIO_READ as ::core::ffi::c_int as uint8_t,
        META_FILE_BUFFER_SIZE as uint32_t,
    );
    if fd.is_null() {
        mfs_log(
            MFSLOG_ERRNO_SYSLOG_STDERR,
            MFSLOG_WARNING,
            b"error opening metadata\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if bio_read(
        fd,
        &raw mut hdr as *mut uint8_t as *mut ::core::ffi::c_void,
        8 as uint64_t,
    ) != 8 as int64_t
    {
        if bio_error(fd) != 0 {
            *__errno_location() = bio_lasterrno(fd);
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"error reading metadata\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        bio_close(fd);
        return -1 as ::core::ffi::c_int;
    }
    if memcmp(
        &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
        b"MFSM NEW\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        8 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        bio_close(fd);
        fs_new();
        chunk_newfs();
        sessions_new();
        sclass_new();
        metaversion = 1 as uint64_t;
        metaid = 0 as uint64_t;
        return 0 as ::core::ffi::c_int;
    }
    if memcmp(
        &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
        b"MFSM \0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        5 as size_t,
    ) == 0 as ::core::ffi::c_int
        && hdr[5 as usize] as ::core::ffi::c_int >= '1' as ::core::ffi::c_int
        && hdr[5 as usize] as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
        && hdr[6 as usize] as ::core::ffi::c_int == '.' as ::core::ffi::c_int
        && hdr[7 as usize] as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
        && hdr[7 as usize] as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
    {
        fver = (((hdr[5 as usize] as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int)
            + (hdr[7 as usize] as ::core::ffi::c_int - '0' as ::core::ffi::c_int))
            as uint8_t;
        if meta_load(fd, fver, &raw mut al) < 0 as ::core::ffi::c_int {
            if bio_error(fd) != 0 {
                *__errno_location() = bio_lasterrno(fd);
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"error reading metadata\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            meta_cleanup();
            bio_close(fd);
            return -2 as ::core::ffi::c_int;
        }
    } else {
        if bio_error(fd) != 0 {
            *__errno_location() = bio_lasterrno(fd);
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"error reading metadata\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"wrong metadata header\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        bio_close(fd);
        return -2 as ::core::ffi::c_int;
    }
    if bio_error(fd) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        *__errno_location() = bio_lasterrno(fd);
        mfs_log(
            MFSLOG_ERRNO_SYSLOG_STDERR,
            MFSLOG_WARNING,
            b"error reading metadata\0".as_ptr() as *const ::core::ffi::c_char,
        );
        meta_cleanup();
        bio_close(fd);
        return -2 as ::core::ffi::c_int;
    }
    bio_close(fd);
    if al != 0 {
        fs_afterload();
    } else {
        fs_printinfo();
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn meta_file_infos() {
    let mut dd: *mut DIR = ::core::ptr::null_mut::<DIR>();
    let mut dp: *mut dirent = ::core::ptr::null_mut::<dirent>();
    let mut ver: uint64_t = 0;
    let mut id: uint64_t = 0;
    let mut status: uint8_t = 0;
    dd = opendir(b".\0".as_ptr() as *const ::core::ffi::c_char);
    if dd.is_null() {
        mfs_log(
            MFSLOG_ERRNO_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"can't access data directory\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        loop {
            dp = readdir(dd);
            if dp.is_null() {
                break;
            }
            if strlen(&raw mut (*dp).d_name as *mut ::core::ffi::c_char) > 8 as size_t
                && memcmp(
                    &raw mut (*dp).d_name as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                    b"metadata\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    8 as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                status = meta_check_metadatafile(
                    &raw mut (*dp).d_name as *mut ::core::ffi::c_char,
                    &raw mut ver,
                    &raw mut id,
                );
                if status as ::core::ffi::c_int == META_CHECK_OK {
                    if id != 0 as uint64_t {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_INFO,
                            b" - found valid metadata file: %s (version: %lu ; id: %lX)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut (*dp).d_name as *mut ::core::ffi::c_char,
                            ver,
                            id,
                        );
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_INFO,
                            b" - found valid metadata file: %s (version: %lu)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut (*dp).d_name as *mut ::core::ffi::c_char,
                            ver,
                        );
                    }
                } else if status as ::core::ffi::c_int == META_CHECK_IOERROR {
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_NOTICE,
                        b" - error reading metadata file: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*dp).d_name as *mut ::core::ffi::c_char,
                    );
                } else if status as ::core::ffi::c_int == META_CHECK_BADHEADER {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_NOTICE,
                        b" - found invalid metadata file (wrong header): %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut (*dp).d_name as *mut ::core::ffi::c_char,
                    );
                } else if status as ::core::ffi::c_int == META_CHECK_BADENDING {
                    if id != 0 as uint64_t {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_NOTICE,
                            b" - found invalid metadata file (wrong ending): %s (version: %lu ; id: %lX)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            &raw mut (*dp).d_name as *mut ::core::ffi::c_char,
                            ver,
                            id,
                        );
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_NOTICE,
                            b" - found invalid metadata file (wrong ending): %s (version: %lu)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            &raw mut (*dp).d_name as *mut ::core::ffi::c_char,
                            ver,
                        );
                    }
                }
            }
        }
        closedir(dd);
    };
}
#[no_mangle]
pub unsafe extern "C" fn meta_loadall() -> ::core::ffi::c_int {
    let mut dd: *mut DIR = ::core::ptr::null_mut::<DIR>();
    let mut dp: *mut dirent = ::core::ptr::null_mut::<dirent>();
    let mut bestver: uint64_t = 0;
    let mut ver: uint64_t = 0;
    let mut bestid: uint64_t = 0;
    let mut id: uint64_t = 0;
    let mut maxlastlv: uint64_t = 0;
    let mut bestfname: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut files: uint32_t = 0;
    let mut pos: uint32_t = 0;
    let mut filenames: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut status: uint8_t = 0;
    let mut hfname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fname: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut st: stat = stat {
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
    let mut i: ::core::ffi::c_int = 0;
    if emptystart != 0 {
        status = meta_check_metadatafile(
            b"metadata.mfs\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut ver,
            &raw mut id,
        );
        if status as ::core::ffi::c_int == META_CHECK_OK {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_INFO,
                b"found valid file 'metadata.mfs' (version: %lu ; id: %lX) in 'empty start' mode - will try to rename it\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                ver,
                id,
            );
            if rename(
                b"metadata.mfs\0".as_ptr() as *const ::core::ffi::c_char,
                b"metadata.mfs.back_emptystarted\0".as_ptr() as *const ::core::ffi::c_char,
            ) < 0 as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"can't rename metadata.mfs -> metadata.mfs.back_emptystarted\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            } else {
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_INFO,
                    b"file metadata.mfs has been renamed to metadata.mfs.back_emptystarted\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        }
        return 0 as ::core::ffi::c_int;
    }
    if allowautorestore != 0 {
        bestver = 0 as uint64_t;
        bestid = 0 as uint64_t;
        bestfname = ::core::ptr::null::<::core::ffi::c_char>();
        dd = opendir(b".\0".as_ptr() as *const ::core::ffi::c_char);
        if dd.is_null() {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"can't access data directory\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            loop {
                dp = readdir(dd);
                if dp.is_null() {
                    break;
                }
                if strlen(&raw mut (*dp).d_name as *mut ::core::ffi::c_char) > 8 as size_t
                    && memcmp(
                        &raw mut (*dp).d_name as *mut ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        b"metadata\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        8 as size_t,
                    ) == 0 as ::core::ffi::c_int
                {
                    status = meta_check_metadatafile(
                        &raw mut (*dp).d_name as *mut ::core::ffi::c_char,
                        &raw mut ver,
                        &raw mut id,
                    );
                    if verboselevel as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        if ver > 0 as uint64_t && status as ::core::ffi::c_int == META_CHECK_OK {
                            if id != 0 as uint64_t {
                                mfs_log(
                                    MFSLOG_SYSLOG_STDERR,
                                    MFSLOG_INFO,
                                    b"found valid metadata file: %s (version: %lu ; id: %lX)\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    &raw mut (*dp).d_name as *mut ::core::ffi::c_char,
                                    ver,
                                    id,
                                );
                            } else {
                                mfs_log(
                                    MFSLOG_SYSLOG_STDERR,
                                    MFSLOG_INFO,
                                    b"found valid metadata file: %s (version: %lu)\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    &raw mut (*dp).d_name as *mut ::core::ffi::c_char,
                                    ver,
                                );
                            }
                        } else if status as ::core::ffi::c_int == META_CHECK_IOERROR {
                            mfs_log(
                                MFSLOG_ERRNO_SYSLOG_STDERR,
                                MFSLOG_NOTICE,
                                b"error reading metadata file: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut (*dp).d_name as *mut ::core::ffi::c_char,
                            );
                        } else if status as ::core::ffi::c_int == META_CHECK_BADHEADER {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_NOTICE,
                                b"found invalid metadata file (wrong header): %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut (*dp).d_name as *mut ::core::ffi::c_char,
                            );
                        } else if status as ::core::ffi::c_int == META_CHECK_BADENDING {
                            if id != 0 as uint64_t {
                                mfs_log(
                                    MFSLOG_SYSLOG_STDERR,
                                    MFSLOG_NOTICE,
                                    b"found invalid metadata file (wrong ending): %s (version: %lu ; id: %lX)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    &raw mut (*dp).d_name as *mut ::core::ffi::c_char,
                                    ver,
                                    id,
                                );
                            } else {
                                mfs_log(
                                    MFSLOG_SYSLOG_STDERR,
                                    MFSLOG_NOTICE,
                                    b"found invalid metadata file (wrong ending): %s (version: %lu)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    &raw mut (*dp).d_name as *mut ::core::ffi::c_char,
                                    ver,
                                );
                            }
                        }
                    }
                    if status as ::core::ffi::c_int == META_CHECK_OK {
                        if bestid != 0 as uint64_t && id != 0 as uint64_t && bestid != id {
                            if ignoreflag != 0 {
                                mfs_log(
                                    MFSLOG_SYSLOG_STDERR,
                                    MFSLOG_NOTICE,
                                    b"found metadata file with different id number - ignoring\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                            } else {
                                mfs_log(
                                    MFSLOG_SYSLOG_STDERR,
                                    MFSLOG_ERR,
                                    b"found metadata file with different id number - cleanup your working directory or use '-i' flag (might be dangerous without cleaning)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                );
                                closedir(dd);
                                meta_file_infos();
                                if !bestfname.is_null() {
                                    free(
                                        bestfname as *mut ::core::ffi::c_char
                                            as *mut ::core::ffi::c_void,
                                    );
                                }
                                return -1 as ::core::ffi::c_int;
                            }
                        }
                        if ver > bestver {
                            bestver = ver;
                            if !bestfname.is_null() {
                                free(
                                    bestfname as *mut ::core::ffi::c_char
                                        as *mut ::core::ffi::c_void,
                                );
                            }
                            bestfname = strdup(&raw mut (*dp).d_name as *mut ::core::ffi::c_char);
                            if id != 0 {
                                bestid = id;
                            }
                        }
                    }
                }
            }
            closedir(dd);
        }
        if bestid != 0 as uint64_t {
            hfname = meta_create_homedir_emergency_filename();
            if !hfname.is_null() {
                status = meta_check_metadatafile(hfname, &raw mut ver, &raw mut id);
                if verboselevel as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    if ver > 0 as uint64_t && status as ::core::ffi::c_int == META_CHECK_OK {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_INFO,
                            b"found valid metadata file: %s (version: %lu)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            hfname,
                            ver,
                        );
                    }
                }
                if status as ::core::ffi::c_int == META_CHECK_OK && ver > bestver && id == bestid {
                    bestver = ver;
                    if !bestfname.is_null() {
                        free(bestfname as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
                    }
                    bestfname = strdup(hfname);
                }
                free(hfname as *mut ::core::ffi::c_void);
            }
            i = 0 as ::core::ffi::c_int;
            loop {
                let c2rust_fresh2 = i;
                i += 1;
                fname = meta_emergency_locations[c2rust_fresh2 as usize];
                if fname.is_null() {
                    break;
                }
                status = meta_check_metadatafile(fname, &raw mut ver, &raw mut id);
                if verboselevel as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    if ver > 0 as uint64_t && status as ::core::ffi::c_int == META_CHECK_OK {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_INFO,
                            b"found valid metadata file: %s (version: %lu)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            fname,
                            ver,
                        );
                    }
                }
                if status as ::core::ffi::c_int == META_CHECK_OK && ver > bestver && id == bestid {
                    bestver = ver;
                    if !bestfname.is_null() {
                        free(bestfname as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
                    }
                    bestfname = strdup(fname);
                }
            }
        }
        if bestver == 0 as uint64_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"can't find valid metadata file\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if !bestfname.is_null() {
                free(bestfname as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
            }
            return -1 as ::core::ffi::c_int;
        }
        if verboselevel as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            if bestid != 0 as uint64_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_INFO,
                    b"chosen most recent metadata file: %s (version: %lu ; id: %lX)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    bestfname,
                    bestver,
                    bestid,
                );
            } else {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_INFO,
                    b"chosen most recent metadata file: %s (version: %lu)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    bestfname,
                    bestver,
                );
            }
        }
        if meta_loadfile(bestfname) < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"error loading metadata file (%s)\0".as_ptr() as *const ::core::ffi::c_char,
                bestfname,
            );
            free(bestfname as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
            return -1 as ::core::ffi::c_int;
        }
        free(bestfname as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        dd = opendir(b".\0".as_ptr() as *const ::core::ffi::c_char);
        files = 0 as uint32_t;
        maxlastlv = 0 as uint64_t;
        if !dd.is_null() {
            loop {
                dp = readdir(dd);
                if dp.is_null() {
                    break;
                }
                files = files.wrapping_add(changelog_checkname(
                    &raw mut (*dp).d_name as *mut ::core::ffi::c_char,
                ) as uint32_t);
            }
        }
        if files > 0 as uint32_t {
            filenames = malloc(
                ::core::mem::size_of::<*mut ::core::ffi::c_char>().wrapping_mul(files as size_t),
            ) as *mut *mut ::core::ffi::c_char;
            rewinddir(dd);
            pos = 0 as uint32_t;
            loop {
                dp = readdir(dd);
                if dp.is_null() {
                    break;
                }
                if changelog_checkname(&raw mut (*dp).d_name as *mut ::core::ffi::c_char) != 0 {
                    let mut firstlv: uint64_t = 0;
                    let mut lastlv: uint64_t = 0;
                    let mut skip: uint8_t = 0;
                    *filenames.offset(pos as isize) =
                        strdup(&raw mut (*dp).d_name as *mut ::core::ffi::c_char);
                    firstlv = changelog_findfirstversion(*filenames.offset(pos as isize));
                    lastlv = changelog_findlastversion(*filenames.offset(pos as isize));
                    skip = (if lastlv < metaversion || firstlv == 0 as uint64_t {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint8_t;
                    if verboselevel as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                        let mut firstlvstr: [::core::ffi::c_char; 21] = [0; 21];
                        let mut fvp: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        let mut lastlvstr: [::core::ffi::c_char; 21] = [0; 21];
                        let mut lvp: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        if firstlv > 0 as uint64_t {
                            fvp = (&raw mut firstlvstr as *mut ::core::ffi::c_char)
                                .offset(20 as ::core::ffi::c_int as isize);
                            *fvp = '\0' as ::core::ffi::c_char;
                            while firstlv > 0 as uint64_t {
                                fvp = fvp.offset(-1);
                                *fvp = ('0' as uint64_t)
                                    .wrapping_add(firstlv.wrapping_rem(10 as uint64_t))
                                    as ::core::ffi::c_char;
                                firstlv = firstlv.wrapping_div(10 as uint64_t);
                            }
                        } else {
                            fvp = &raw mut firstlvstr as *mut ::core::ffi::c_char;
                            *fvp.offset(2 as isize) = '?' as ::core::ffi::c_char;
                            *fvp.offset(1 as isize) = *fvp.offset(2 as isize);
                            *fvp.offset(0 as isize) = *fvp.offset(1 as isize);
                            *fvp.offset(3 as isize) = '\0' as ::core::ffi::c_char;
                        }
                        if lastlv > 0 as uint64_t {
                            lvp = (&raw mut lastlvstr as *mut ::core::ffi::c_char)
                                .offset(20 as ::core::ffi::c_int as isize);
                            *lvp = '\0' as ::core::ffi::c_char;
                            while lastlv > 0 as uint64_t {
                                lvp = lvp.offset(-1);
                                *lvp = ('0' as uint64_t)
                                    .wrapping_add(lastlv.wrapping_rem(10 as uint64_t))
                                    as ::core::ffi::c_char;
                                lastlv = lastlv.wrapping_div(10 as uint64_t);
                            }
                        } else {
                            lvp = &raw mut lastlvstr as *mut ::core::ffi::c_char;
                            *lvp.offset(2 as isize) = '?' as ::core::ffi::c_char;
                            *lvp.offset(1 as isize) = *lvp.offset(2 as isize);
                            *lvp.offset(0 as isize) = *lvp.offset(1 as isize);
                            *lvp.offset(3 as isize) = '\0' as ::core::ffi::c_char;
                        }
                        if skip != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_INFO,
                                b"skipping changelog file: %s (changes: %s - %s)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                *filenames.offset(pos as isize),
                                fvp,
                                lvp,
                            );
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_INFO,
                                b"using changelog file: %s (changes: %s - %s)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                *filenames.offset(pos as isize),
                                fvp,
                                lvp,
                            );
                        }
                    }
                    if skip != 0 {
                        free(*filenames.offset(pos as isize) as *mut ::core::ffi::c_void);
                        files = files.wrapping_sub(1);
                    } else {
                        pos = pos.wrapping_add(1);
                        if lastlv > maxlastlv {
                            maxlastlv = lastlv;
                        }
                    }
                }
            }
            closedir(dd);
            merger_start(files, filenames, MAXIDHOLE as uint64_t, bestver, maxlastlv);
            pos = 0 as uint32_t;
            while pos < files {
                free(*filenames.offset(pos as isize) as *mut ::core::ffi::c_void);
                pos = pos.wrapping_add(1);
            }
            free(filenames as *mut ::core::ffi::c_void);
            if merger_loop(verboselevel) != 0 as ::core::ffi::c_int {
                if ignoreflag != 0 {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_NOTICE,
                        b"error applying changelogs - ignoring (using best possible metadata version)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"error applying changelogs - fix changelogs manually or use '-i' flag\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            }
        } else if !dd.is_null() {
            closedir(dd);
        }
        if stat(
            b"metadata.mfs\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut st,
        ) == 0 as ::core::ffi::c_int
        {
            if st.st_size == 0 as __off_t {
                if unlink(b"metadata.mfs\0".as_ptr() as *const ::core::ffi::c_char)
                    < 0 as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"can't unlink metadata.mfs\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            } else if stat(
                b"metadata.mfs.back\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut st,
            ) < 0 as ::core::ffi::c_int
                && *__errno_location() == ENOENT
            {
                if rename(
                    b"metadata.mfs\0".as_ptr() as *const ::core::ffi::c_char,
                    b"metadata.mfs.back\0".as_ptr() as *const ::core::ffi::c_char,
                ) < 0 as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"can't rename metadata.mfs -> metadata.mfs.back\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            } else {
                let mut name: *mut ::core::ffi::c_char =
                    strdup(b"metadata.mfs.XXXXXX\0".as_ptr() as *const ::core::ffi::c_char);
                let mut fd: ::core::ffi::c_int = 0;
                fd = mkstemp(name);
                if fd < 0 as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"can't create temporary file %s\0".as_ptr() as *const ::core::ffi::c_char,
                        name,
                    );
                    free(name as *mut ::core::ffi::c_void);
                    return -1 as ::core::ffi::c_int;
                }
                if rename(
                    b"metadata.mfs\0".as_ptr() as *const ::core::ffi::c_char,
                    name,
                ) < 0 as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"can't rename metadata.mfs -> %s\0".as_ptr() as *const ::core::ffi::c_char,
                        name,
                    );
                    close(fd);
                    free(name as *mut ::core::ffi::c_void);
                    return -1 as ::core::ffi::c_int;
                }
                close(fd);
                free(name as *mut ::core::ffi::c_void);
            }
        }
    } else {
        match meta_check_metadatafile(
            b"metadata.mfs\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut ver,
            &raw mut id,
        ) as ::core::ffi::c_int
        {
            META_CHECK_NOFILE => {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"can't find metadata.mfs - try using option '-a'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            META_CHECK_IOERROR => {
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"error reading metadata.mfs - try using option '-a'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            META_CHECK_BADHEADER => {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"metadata.mfs has wrong header - try using option '-a'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            META_CHECK_BADENDING => {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"metadata.mfs has wrong ending - try using option '-a'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            _ => {}
        }
        if meta_check_metadatafile(
            b"metadata.mfs.back\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut bestver,
            &raw mut bestid,
        ) as ::core::ffi::c_int
            == META_CHECK_OK
            && (ver == 1 as uint64_t && id == 0 as uint64_t
                || bestver > ver && bestid == id
                || bestid != 0 as uint64_t && id != 0 as uint64_t && bestid != id)
        {
            if ver == 1 as uint64_t && id == 0 as uint64_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"backup file exists but current metadata file is empty - please check it manually - try using option '-a'\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            } else if bestver > ver {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"backup file is newer than current file - please check it manually - try using option '-a'\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"backup file has different file id - please check it manually - try using option '-a' and '-i'\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
            return -1 as ::core::ffi::c_int;
        }
        if meta_check_metadatafile(
            b"metadata_ml.mfs.back\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut bestver,
            &raw mut bestid,
        ) as ::core::ffi::c_int
            == META_CHECK_OK
            && (ver == 1 as uint64_t && id == 0 as uint64_t
                || bestver > ver && bestid == id
                || bestid != 0 as uint64_t && id != 0 as uint64_t && bestid != id)
        {
            if ver == 1 as uint64_t && id == 0 as uint64_t {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"metalogger file exists but current metadata file is empty - please check it manually - try using option '-a'\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            } else if bestver > ver {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"metalogger file is newer than current file - please check it manually - try using option '-a'\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"metalogger file has different file id - please check it manually - try using option '-a' and '-i'\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
            return -1 as ::core::ffi::c_int;
        }
        if meta_loadfile(b"metadata.mfs\0".as_ptr() as *const ::core::ffi::c_char)
            < 0 as ::core::ffi::c_int
        {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"error loading metadata.mfs - try using option '-a'\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        if rename(
            b"metadata.mfs\0".as_ptr() as *const ::core::ffi::c_char,
            b"metadata.mfs.back\0".as_ptr() as *const ::core::ffi::c_char,
        ) < 0 as ::core::ffi::c_int
        {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"can't rename metadata.mfs -> metadata.mfs.back\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
    }
    unlink(b"metadata.mfs.back.tmp\0".as_ptr() as *const ::core::ffi::c_char);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn meta_version_inc() -> uint64_t {
    let c2rust_fresh0 = metaversion;
    metaversion = metaversion.wrapping_add(1);
    return c2rust_fresh0;
}
#[no_mangle]
pub unsafe extern "C" fn meta_version() -> uint64_t {
    return metaversion;
}
#[no_mangle]
pub unsafe extern "C" fn meta_setignoreflag() {
    ignoreflag = 1 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn meta_allowautorestore() {
    allowautorestore = 1 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn meta_emptystart() {
    emptystart = 1 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn meta_incverboselevel() {
    verboselevel = verboselevel.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn meta_info(
    mut lsstore: *mut uint32_t,
    mut lstime: *mut uint32_t,
    mut lsstat: *mut uint8_t,
    mut lsmetavers: *mut uint64_t,
    mut lschecksum: *mut uint32_t,
) {
    *lsstore = lastsuccessfulstore;
    *lstime = (laststoretime * 1000 as ::core::ffi::c_int as ::core::ffi::c_double) as uint32_t;
    *lsstat = laststorestatus;
    *lsmetavers = laststoremetaversion;
    *lschecksum = laststorechecksum;
}
#[no_mangle]
pub unsafe extern "C" fn meta_get_id() -> uint64_t {
    return metaid;
}
#[no_mangle]
pub unsafe extern "C" fn meta_set_id(mut newmetaid: uint64_t) {
    metaid = newmetaid;
    if fs_set_root_times((metaid >> 32 as ::core::ffi::c_int) as uint32_t) < 0 as ::core::ffi::c_int
    {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"internal error: can't set atime/mtime/ctime on root node\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        main_exit();
    }
}
#[no_mangle]
pub unsafe extern "C" fn meta_parse_offset(mut offsetstr: *const ::core::ffi::c_char) {
    let mut hours: uint32_t = 0;
    let mut mins: uint32_t = 0;
    let mut error: uint8_t = 0;
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    hours = 0 as uint32_t;
    mins = 0 as uint32_t;
    MetaSaveOffsetLocal = 0 as uint8_t;
    p = offsetstr;
    while *p != 0 {
        error = 0 as uint8_t;
        if *p as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
            && *p as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
        {
            if MetaSaveOffsetLocal != 0 {
                error = 1 as uint8_t;
            } else {
                mins = mins.wrapping_mul(10 as uint32_t);
                mins = mins.wrapping_add(
                    (*p as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as uint32_t,
                );
            }
        } else if *p as ::core::ffi::c_int == ':' as ::core::ffi::c_int {
            if MetaSaveOffsetLocal as ::core::ffi::c_int != 0 || hours != 0 {
                error = 1 as uint8_t;
            } else {
                hours = mins;
                mins = 0 as uint32_t;
            }
        } else if *p as ::core::ffi::c_int == 'L' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == 'l' as ::core::ffi::c_int
        {
            MetaSaveOffsetLocal = 1 as uint8_t;
        } else if *p as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
            && *p as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
        {
            error = 1 as uint8_t;
        }
        if error != 0 {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"METADATA_SAVE_OFFSET - parse error - using zero in UTC\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            MetaSaveOffset = 0 as uint32_t;
            MetaSaveOffsetLocal = 0 as uint8_t;
            return;
        }
        p = p.offset(1);
    }
    MetaSaveOffset = hours.wrapping_mul(60 as uint32_t).wrapping_add(mins);
}
#[no_mangle]
pub unsafe extern "C" fn meta_reload() {
    let mut back_logs: uint32_t = 0;
    let mut MetaSaveOffsetStr: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    MetaSaveFreq = cfg_getuint32(
        b"METADATA_SAVE_FREQ\0".as_ptr() as *const ::core::ffi::c_char,
        1 as uint32_t,
    );
    MetaSaveOffsetStr = cfg_getstr(
        b"METADATA_SAVE_OFFSET\0".as_ptr() as *const ::core::ffi::c_char,
        b"0\0".as_ptr() as *const ::core::ffi::c_char,
    );
    meta_parse_offset(MetaSaveOffsetStr);
    free(MetaSaveOffsetStr as *mut ::core::ffi::c_void);
    MetaDownloadFreq = 0 as uint32_t;
    MetaCheckFreq = 0 as uint32_t;
    back_logs = cfg_getuint32(
        b"BACK_LOGS\0".as_ptr() as *const ::core::ffi::c_char,
        50 as uint32_t,
    );
    if MetaSaveFreq > back_logs.wrapping_div(2 as uint32_t) {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            b"METADATA_SAVE_FREQ is higher than half of BACK_LOGS - decreasing\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        MetaSaveFreq = back_logs.wrapping_div(2 as uint32_t);
    }
    if MetaSaveFreq == 0 as uint32_t {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            b"METADATA_SAVE_FREQ is zero - set to one\0".as_ptr() as *const ::core::ffi::c_char,
        );
        MetaSaveFreq = 1 as uint32_t;
    }
    MetaSaveFreq = MetaSaveFreq.wrapping_mul(60 as uint32_t);
    MetaDownloadFreq = MetaDownloadFreq.wrapping_mul(60 as uint32_t);
    MetaCheckFreq = MetaCheckFreq.wrapping_mul(60 as uint32_t);
    if MetaSaveOffset > (60 as ::core::ffi::c_int * 24 as ::core::ffi::c_int) as uint32_t {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            b"METADATA_SAVE_OFFSET is higher than 24 hours - using value modulo 24 hours\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        MetaSaveOffset = MetaSaveOffset
            .wrapping_rem(60 as uint32_t)
            .wrapping_mul(24 as uint32_t);
    }
    BackMetaCopies = cfg_getuint32(
        b"BACK_META_KEEP_PREVIOUS\0".as_ptr() as *const ::core::ffi::c_char,
        1 as uint32_t,
    );
    if BackMetaCopies > 99 as uint32_t {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            b"BACK_META_KEEP_PREVIOUS is too high (>99) - decreasing\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        BackMetaCopies = 99 as uint32_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn meta_check_id() {
    if metaid == 0 as uint64_t {
        let mut now: uint32_t = main_time();
        if fs_set_root_times(now) < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"internal error: can't set atime/mtime/ctime on root node\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            main_exit();
        } else {
            metaid = now as uint64_t;
            metaid <<= 32 as ::core::ffi::c_int;
            metaid |= (rndu32() as uint64_t).wrapping_add(monotonic_useconds());
            changelog(
                b"%u|SETMETAID(%lu)\0".as_ptr() as *const ::core::ffi::c_char,
                now,
                metaid,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn meta_mr_setmetaid(mut newmetaid: uint64_t) -> uint8_t {
    if metaid == 0 as uint64_t || metaid == newmetaid {
        if metaid == 0 as uint64_t {
            if fs_set_root_times((metaid >> 32 as ::core::ffi::c_int) as uint32_t)
                < 0 as ::core::ffi::c_int
            {
                return MFS_ERROR_ENOENT as uint8_t;
            }
        }
        metaversion = metaversion.wrapping_add(1);
        metaid = newmetaid;
        return MFS_STATUS_OK as uint8_t;
    } else {
        return MFS_ERROR_EINVAL as uint8_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn meta_prepare_data_structures() -> ::core::ffi::c_int {
    metaversion = 0 as uint64_t;
    metaid = 0 as uint64_t;
    if dict_init() < 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"dictionary init error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if sclass_init() < 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"storage class init error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if patterns_init() < 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"patterns init error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if fs_strinit() < 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"filesystem-tree init error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if chunk_strinit() < 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"chunk init error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if xattr_init() < 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"xattr init error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if posix_acl_init() < 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"posix_acl init error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if flock_init() < 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"flock_locks init error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if posix_lock_init() < 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"posix_locks init error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if csdb_init() < 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"csdb init error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if sessions_init() < 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"sessions init error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if of_init() < 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"open-files init error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn meta_restore() -> ::core::ffi::c_int {
    let mut status: uint8_t = 0;
    if meta_prepare_data_structures() < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    allowautorestore = 1 as uint8_t;
    mfs_log(
        MFSLOG_SYSLOG_STDERR,
        MFSLOG_INFO,
        b"loading metadata ...\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if meta_loadall() < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    status = meta_storeall(0 as ::core::ffi::c_int, 0 as uint8_t) as uint8_t;
    if status as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        if rename(
            b"metadata.mfs.back\0".as_ptr() as *const ::core::ffi::c_char,
            b"metadata.mfs\0".as_ptr() as *const ::core::ffi::c_char,
        ) < 0 as ::core::ffi::c_int
        {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"can't rename metadata.mfs.back -> metadata.mfs\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        meta_cleanup();
        return 0 as ::core::ffi::c_int;
    } else if status as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_NOTICE,
            b"no metadata to store\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    return -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn meta_store_status_str() -> *const ::core::ffi::c_char {
    match laststorestatus as ::core::ffi::c_int {
        LASTSTORE_DOWNLOADED => {
            return b"DOWNLOADED\0".as_ptr() as *const ::core::ffi::c_char;
        }
        LASTSTORE_CRC_STORED_BG => {
            return b"CRC ONLY IN BACKGROUND\0".as_ptr() as *const ::core::ffi::c_char;
        }
        LASTSTORE_META_STORED_BG => {
            return b"STORED IN BACKGROUND\0".as_ptr() as *const ::core::ffi::c_char;
        }
        LASTSTORE_META_STORED_FG => {
            return b"STORED IN FOREGROUND\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return b"???\0".as_ptr() as *const ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn meta_log_extra_info(mut fd: *mut FILE) {
    fprintf(fd, b"[meta]\n\0".as_ptr() as *const ::core::ffi::c_char);
    fprintf(
        fd,
        b"meta id: 0x%016lX\n\0".as_ptr() as *const ::core::ffi::c_char,
        metaid,
    );
    fprintf(
        fd,
        b"meta version: %lu\n\0".as_ptr() as *const ::core::ffi::c_char,
        metaversion,
    );
    if laststoremetaversion > 0 as uint64_t {
        fprintf(
            fd,
            b"last stored meta version: %lu\n\0".as_ptr() as *const ::core::ffi::c_char,
            laststoremetaversion,
        );
    }
    if lastsuccessfulstore > 0 as uint32_t {
        fprintf(
            fd,
            b"metadata has been stored %d seconds ago\n\0".as_ptr() as *const ::core::ffi::c_char,
            main_time().wrapping_sub(lastsuccessfulstore),
        );
        fprintf(
            fd,
            b"last metadata store mode: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            meta_store_status_str(),
        );
    }
    if metasaverpid >= 0 as ::core::ffi::c_int {
        fprintf(
            fd,
            b"background store in progress; process pid: %d\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            metasaverpid,
        );
    }
    if storestarttime > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        fprintf(
            fd,
            b"store started %.2lf seconds ego\n\0".as_ptr() as *const ::core::ffi::c_char,
            monotonic_seconds() - storestarttime,
        );
    }
    fprintf(fd, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn meta_init() -> ::core::ffi::c_int {
    if meta_prepare_data_structures() < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if emptystart as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_INFO,
            b"loading metadata ...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if meta_loadall() < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_INFO,
            b"metadata file has been loaded\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"can't run master without metadata\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    meta_reload();
    main_reload_register_fname(
        Some(meta_reload as unsafe extern "C" fn() -> ()),
        b"meta_reload\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_info_register_fname(
        Some(meta_log_extra_info as unsafe extern "C" fn(*mut FILE) -> ()),
        b"meta_log_extra_info\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_time_register_fname(
        60 as uint32_t,
        0 as uint32_t,
        Some(meta_store_task as unsafe extern "C" fn() -> ()),
        b"meta_store_task\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_mayexit_register_fname(
        Some(meta_mayexit as unsafe extern "C" fn() -> ::core::ffi::c_int),
        b"meta_mayexit\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_destruct_register_fname(
        Some(meta_term as unsafe extern "C" fn() -> ()),
        b"meta_term\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fs_renumerate_edge_test();
    meta_check_id();
    return 0 as ::core::ffi::c_int;
}
