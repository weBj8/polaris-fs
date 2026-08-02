pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
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
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn gettimeofday(
        __tv: *mut timeval,
        __tz: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn getpwuid(__uid: __uid_t) -> *mut passwd;
    unsafe fn getpwuid_r(
        __uid: __uid_t,
        __resultbuf: *mut passwd,
        __buffer: *mut ::core::ffi::c_char,
        __buflen: size_t,
        __result: *mut *mut passwd,
    ) -> ::core::ffi::c_int;
    unsafe fn getgrgid(__gid: __gid_t) -> *mut group;
    unsafe fn getgrgid_r(
        __gid: __gid_t,
        __resultbuf: *mut group,
        __buffer: *mut ::core::ffi::c_char,
        __buflen: size_t,
        __result: *mut *mut group,
    ) -> ::core::ffi::c_int;
    unsafe fn univmakestrip(strip: *mut ::core::ffi::c_char, ip: uint32_t);
    unsafe fn tcpsocket() -> ::core::ffi::c_int;
    unsafe fn tcpresolve(
        hostname: *const ::core::ffi::c_char,
        service: *const ::core::ffi::c_char,
        ip: *mut uint32_t,
        port: *mut uint16_t,
        passiveflag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn tcpnodelay(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpnumbind(
        sock: ::core::ffi::c_int,
        ip: uint32_t,
        port: uint16_t,
    ) -> ::core::ffi::c_int;
    unsafe fn tcpnumtoconnect(
        sock: ::core::ffi::c_int,
        ip: uint32_t,
        port: uint16_t,
        msecto: uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn tcptoread(
        sock: ::core::ffi::c_int,
        buff: *mut ::core::ffi::c_void,
        leng: uint32_t,
        msectopart: uint32_t,
        msectoall: uint32_t,
    ) -> int32_t;
    unsafe fn tcptowrite(
        sock: ::core::ffi::c_int,
        buff: *const ::core::ffi::c_void,
        leng: uint32_t,
        msectopart: uint32_t,
        msectoall: uint32_t,
    ) -> int32_t;
    unsafe fn tcpclose(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn md5_init(ctx: *mut md5ctx);
    unsafe fn md5_update(ctx: *mut md5ctx, buff: *const uint8_t, leng: uint32_t);
    unsafe fn md5_final(digest: *mut uint8_t, ctx: *mut md5ctx);
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
    unsafe fn monotonic_useconds() -> uint64_t;
    unsafe fn heap_cleanup();
    unsafe fn heap_push(element: uint32_t);
    unsafe fn heap_pop() -> uint32_t;
    unsafe fn heap_elements() -> uint32_t;
    unsafe fn heap_term();
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn read_get_total_bytes() -> uint64_t;
    unsafe fn write_get_total_bytes() -> uint64_t;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
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
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut ::core::ffi::c_char,
    pub pw_passwd: *mut ::core::ffi::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut ::core::ffi::c_char,
    pub pw_dir: *mut ::core::ffi::c_char,
    pub pw_shell: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut ::core::ffi::c_char,
    pub gr_passwd: *mut ::core::ffi::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _md5ctx {
    pub state: [uint32_t; 4],
    pub count: [uint32_t; 2],
    pub buffer: [uint8_t; 64],
}
pub type md5ctx = _md5ctx;
// std::sync primitives replace pthread_mutex_t/pthread_cond_t: futex-based,
// no heap resources, created with Mutex::new/Condvar::new inside Box::new in
// fs_get_my_threc. Copy/Clone derive dropped (Mutex is not Copy); the struct
// is only ever handled through Box::into_raw'd pointers.
#[repr(C)]
pub struct _threc {
    pub lock: std::sync::Mutex<()>,
    pub cond: std::sync::Condvar,
    pub obuff: *mut uint8_t,
    pub obuffsize: uint32_t,
    pub odataleng: uint32_t,
    pub ibuff: *mut uint8_t,
    pub ibuffsize: uint32_t,
    pub idataleng: uint32_t,
    pub sent: uint8_t,
    pub status: uint8_t,
    pub rcvd: uint8_t,
    pub receiving: uint8_t,
    pub rcvd_cmd: uint32_t,
    pub packetid: uint32_t,
    pub next: *mut _threc,
}
pub type threc = _threc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _amtime_file {
    pub inode: uint32_t,
    pub atimeage: uint16_t,
    pub mtimeage: uint16_t,
    pub atime: uint64_t,
    pub mtime: uint64_t,
    pub next: *mut _amtime_file,
}
pub type amtime_file = _amtime_file;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _acquired_file {
    pub inode: uint32_t,
    pub cnt: uint16_t,
    pub age: uint8_t,
    pub dentry: uint8_t,
    pub next: *mut _acquired_file,
    pub lrunext: *mut _acquired_file,
    pub lruprev: *mut *mut _acquired_file,
}
pub type acquired_file = _acquired_file;
pub type C2Rust_Unnamed_0 = ::core::ffi::c_uint;
pub const STATNODES: C2Rust_Unnamed_0 = 7;
pub const MASTER_TIMEDIFF: C2Rust_Unnamed_0 = 6;
pub const MASTER_PING: C2Rust_Unnamed_0 = 5;
pub const MASTER_PACKETSRCVD: C2Rust_Unnamed_0 = 4;
pub const MASTER_PACKETSSENT: C2Rust_Unnamed_0 = 3;
pub const MASTER_BYTESRCVD: C2Rust_Unnamed_0 = 2;
pub const MASTER_BYTESSENT: C2Rust_Unnamed_0 = 1;
pub const MASTER_CONNECTS: C2Rust_Unnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct connect_args_t {
    pub bindhostname: *mut ::core::ffi::c_char,
    pub masterhostname: *mut ::core::ffi::c_char,
    pub masterportname: *mut ::core::ffi::c_char,
    pub meta: uint8_t,
    pub clearpassword: uint8_t,
    pub info: *mut ::core::ffi::c_char,
    pub subfolder: *mut ::core::ffi::c_char,
    pub passworddigest: *mut uint8_t,
    pub minversion: uint32_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ETIMEDOUT: ::core::ffi::c_int = 110 as ::core::ffi::c_int;
pub const MFSCHUNKSIZE: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;
pub const EXPORT_GROUPS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_ERROR_EINVAL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MFS_ERROR_IO: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const MFS_ERROR_ENOTSUP: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
pub const MFS_ERROR_MAX: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DISP_TYPE_FILE: ::core::ffi::c_int = 'f' as ::core::ffi::c_int;
pub const DISP_TYPE_DIRECTORY: ::core::ffi::c_int = 'd' as ::core::ffi::c_int;
pub const DISP_TYPE_SYMLINK: ::core::ffi::c_int = 'l' as ::core::ffi::c_int;
pub const DISP_TYPE_FIFO: ::core::ffi::c_int = 'q' as ::core::ffi::c_int;
pub const DISP_TYPE_BLOCKDEV: ::core::ffi::c_int = 'b' as ::core::ffi::c_int;
pub const DISP_TYPE_CHARDEV: ::core::ffi::c_int = 'c' as ::core::ffi::c_int;
pub const DISP_TYPE_SOCKET: ::core::ffi::c_int = 's' as ::core::ffi::c_int;
pub const DISP_TYPE_TRASH: ::core::ffi::c_int = 't' as ::core::ffi::c_int;
pub const DISP_TYPE_SUSTAINED: ::core::ffi::c_int = 'r' as ::core::ffi::c_int;
pub const TYPE_FILE: ::core::ffi::c_int = 1;
pub const TYPE_DIRECTORY: ::core::ffi::c_int = 2;
pub const TYPE_SYMLINK: ::core::ffi::c_int = 3;
pub const TYPE_FIFO: ::core::ffi::c_int = 4;
pub const TYPE_BLOCKDEV: ::core::ffi::c_int = 5;
pub const TYPE_CHARDEV: ::core::ffi::c_int = 6;
pub const TYPE_SOCKET: ::core::ffi::c_int = 7;
pub const TYPE_TRASH: ::core::ffi::c_int = 8;
pub const TYPE_SUSTAINED: ::core::ffi::c_int = 9;
pub const LOOKUP_CHUNK_ZERO_DATA: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const POSIX_LOCK_UNLCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const GETDIR_FLAG_WITHATTR: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const GETDIR_FLAG_ADDTOCACHE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SESFLAG_MAPALL: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const MFS_XATTR_REMOVE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFS_XATTR_GETA_DATA: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_XATTR_LENGTH_ONLY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ATTR_RECORD_SIZE: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const ANTOAN_NOP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ANTOAN_UNKNOWN_COMMAND: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ANTOAN_BAD_COMMAND_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ANTOAN_FORCE_TIMEOUT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const ANTOAN_GET_CONFIG: ::core::ffi::c_int = PROTO_BASE + 80 as ::core::ffi::c_int;
pub const ANTOAN_CONFIG_VALUE: ::core::ffi::c_int = PROTO_BASE + 81 as ::core::ffi::c_int;
pub const ANTOAN_GET_CONFIG_FILE: ::core::ffi::c_int = PROTO_BASE + 82 as ::core::ffi::c_int;
pub const ANTOAN_CONFIG_FILE_CONTENT: ::core::ffi::c_int = PROTO_BASE + 83 as ::core::ffi::c_int;
pub const CLTOMA_PATH_LOOKUP: ::core::ffi::c_int = PROTO_BASE + 390 as ::core::ffi::c_int;
pub const MATOCL_PATH_LOOKUP: ::core::ffi::c_int = PROTO_BASE + 391 as ::core::ffi::c_int;
pub const FUSE_REGISTER_BLOB_ACL: [::core::ffi::c_char; 65] = unsafe {
    ::core::mem::transmute::<[u8; 65], [::core::ffi::c_char; 65]>(
        *b"DjI1GAQDULI5d2YjA26ypc3ovkhjvhciTQVx3CS4nYgtBoUcsljiVpsErJENHaw0\0",
    )
};
pub const REGISTER_GETRANDOM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const REGISTER_NEWSESSION: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const REGISTER_RECONNECT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const REGISTER_NEWMETASESSION: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const REGISTER_CLOSESESSION: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_REGISTER: ::core::ffi::c_int = PROTO_BASE + 400 as ::core::ffi::c_int;
pub const MATOCL_FUSE_REGISTER: ::core::ffi::c_int = PROTO_BASE + 401 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_STATFS: ::core::ffi::c_int = PROTO_BASE + 402 as ::core::ffi::c_int;
pub const MATOCL_FUSE_STATFS: ::core::ffi::c_int = PROTO_BASE + 403 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_ACCESS: ::core::ffi::c_int = PROTO_BASE + 404 as ::core::ffi::c_int;
pub const MATOCL_FUSE_ACCESS: ::core::ffi::c_int = PROTO_BASE + 405 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_LOOKUP: ::core::ffi::c_int = PROTO_BASE + 406 as ::core::ffi::c_int;
pub const MATOCL_FUSE_LOOKUP: ::core::ffi::c_int = PROTO_BASE + 407 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETATTR: ::core::ffi::c_int = PROTO_BASE + 408 as ::core::ffi::c_int;
pub const MATOCL_FUSE_GETATTR: ::core::ffi::c_int = PROTO_BASE + 409 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SETATTR: ::core::ffi::c_int = PROTO_BASE + 410 as ::core::ffi::c_int;
pub const MATOCL_FUSE_SETATTR: ::core::ffi::c_int = PROTO_BASE + 411 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_READLINK: ::core::ffi::c_int = PROTO_BASE + 412 as ::core::ffi::c_int;
pub const MATOCL_FUSE_READLINK: ::core::ffi::c_int = PROTO_BASE + 413 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SYMLINK: ::core::ffi::c_int = PROTO_BASE + 414 as ::core::ffi::c_int;
pub const MATOCL_FUSE_SYMLINK: ::core::ffi::c_int = PROTO_BASE + 415 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_MKNOD: ::core::ffi::c_int = PROTO_BASE + 416 as ::core::ffi::c_int;
pub const MATOCL_FUSE_MKNOD: ::core::ffi::c_int = PROTO_BASE + 417 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_MKDIR: ::core::ffi::c_int = PROTO_BASE + 418 as ::core::ffi::c_int;
pub const MATOCL_FUSE_MKDIR: ::core::ffi::c_int = PROTO_BASE + 419 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_UNLINK: ::core::ffi::c_int = PROTO_BASE + 420 as ::core::ffi::c_int;
pub const MATOCL_FUSE_UNLINK: ::core::ffi::c_int = PROTO_BASE + 421 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_RMDIR: ::core::ffi::c_int = PROTO_BASE + 422 as ::core::ffi::c_int;
pub const MATOCL_FUSE_RMDIR: ::core::ffi::c_int = PROTO_BASE + 423 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_RENAME: ::core::ffi::c_int = PROTO_BASE + 424 as ::core::ffi::c_int;
pub const MATOCL_FUSE_RENAME: ::core::ffi::c_int = PROTO_BASE + 425 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_LINK: ::core::ffi::c_int = PROTO_BASE + 426 as ::core::ffi::c_int;
pub const MATOCL_FUSE_LINK: ::core::ffi::c_int = PROTO_BASE + 427 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_READDIR: ::core::ffi::c_int = PROTO_BASE + 428 as ::core::ffi::c_int;
pub const MATOCL_FUSE_READDIR: ::core::ffi::c_int = PROTO_BASE + 429 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_OPEN: ::core::ffi::c_int = PROTO_BASE + 430 as ::core::ffi::c_int;
pub const MATOCL_FUSE_OPEN: ::core::ffi::c_int = PROTO_BASE + 431 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_READ_CHUNK: ::core::ffi::c_int = PROTO_BASE + 432 as ::core::ffi::c_int;
pub const MATOCL_FUSE_READ_CHUNK: ::core::ffi::c_int = PROTO_BASE + 433 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_WRITE_CHUNK: ::core::ffi::c_int = PROTO_BASE + 434 as ::core::ffi::c_int;
pub const MATOCL_FUSE_WRITE_CHUNK: ::core::ffi::c_int = PROTO_BASE + 435 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_WRITE_CHUNK_END: ::core::ffi::c_int = PROTO_BASE + 436 as ::core::ffi::c_int;
pub const MATOCL_FUSE_WRITE_CHUNK_END: ::core::ffi::c_int = PROTO_BASE + 437 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETTRASH: ::core::ffi::c_int = PROTO_BASE + 450 as ::core::ffi::c_int;
pub const MATOCL_FUSE_GETTRASH: ::core::ffi::c_int = PROTO_BASE + 451 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETDETACHEDATTR: ::core::ffi::c_int = PROTO_BASE + 452 as ::core::ffi::c_int;
pub const MATOCL_FUSE_GETDETACHEDATTR: ::core::ffi::c_int = PROTO_BASE + 453 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETTRASHPATH: ::core::ffi::c_int = PROTO_BASE + 454 as ::core::ffi::c_int;
pub const MATOCL_FUSE_GETTRASHPATH: ::core::ffi::c_int = PROTO_BASE + 455 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SETTRASHPATH: ::core::ffi::c_int = PROTO_BASE + 456 as ::core::ffi::c_int;
pub const MATOCL_FUSE_SETTRASHPATH: ::core::ffi::c_int = PROTO_BASE + 457 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_UNDEL: ::core::ffi::c_int = PROTO_BASE + 458 as ::core::ffi::c_int;
pub const MATOCL_FUSE_UNDEL: ::core::ffi::c_int = PROTO_BASE + 459 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_PURGE: ::core::ffi::c_int = PROTO_BASE + 460 as ::core::ffi::c_int;
pub const MATOCL_FUSE_PURGE: ::core::ffi::c_int = PROTO_BASE + 461 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_TRUNCATE: ::core::ffi::c_int = PROTO_BASE + 464 as ::core::ffi::c_int;
pub const MATOCL_FUSE_TRUNCATE: ::core::ffi::c_int = PROTO_BASE + 465 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETSUSTAINED: ::core::ffi::c_int = PROTO_BASE + 470 as ::core::ffi::c_int;
pub const MATOCL_FUSE_GETSUSTAINED: ::core::ffi::c_int = PROTO_BASE + 471 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETXATTR: ::core::ffi::c_int = PROTO_BASE + 478 as ::core::ffi::c_int;
pub const MATOCL_FUSE_GETXATTR: ::core::ffi::c_int = PROTO_BASE + 479 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SETXATTR: ::core::ffi::c_int = PROTO_BASE + 480 as ::core::ffi::c_int;
pub const MATOCL_FUSE_SETXATTR: ::core::ffi::c_int = PROTO_BASE + 481 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_CREATE: ::core::ffi::c_int = PROTO_BASE + 482 as ::core::ffi::c_int;
pub const MATOCL_FUSE_CREATE: ::core::ffi::c_int = PROTO_BASE + 483 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETFACL: ::core::ffi::c_int = PROTO_BASE + 488 as ::core::ffi::c_int;
pub const MATOCL_FUSE_GETFACL: ::core::ffi::c_int = PROTO_BASE + 489 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SETFACL: ::core::ffi::c_int = PROTO_BASE + 490 as ::core::ffi::c_int;
pub const MATOCL_FUSE_SETFACL: ::core::ffi::c_int = PROTO_BASE + 491 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_FLOCK: ::core::ffi::c_int = PROTO_BASE + 492 as ::core::ffi::c_int;
pub const MATOCL_FUSE_FLOCK: ::core::ffi::c_int = PROTO_BASE + 493 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_POSIX_LOCK: ::core::ffi::c_int = PROTO_BASE + 494 as ::core::ffi::c_int;
pub const MATOCL_FUSE_POSIX_LOCK: ::core::ffi::c_int = PROTO_BASE + 495 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SUSTAINED_INODES_DEPRECATED: ::core::ffi::c_int =
    PROTO_BASE + 499 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SUSTAINED_INODES: ::core::ffi::c_int = PROTO_BASE + 700 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_AMTIME_INODES: ::core::ffi::c_int = PROTO_BASE + 701 as ::core::ffi::c_int;
pub const MATOCL_FUSE_CHUNK_HAS_CHANGED: ::core::ffi::c_int =
    PROTO_BASE + 702 as ::core::ffi::c_int;
pub const MATOCL_FUSE_FLENG_HAS_CHANGED: ::core::ffi::c_int =
    PROTO_BASE + 703 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_TIME_SYNC: ::core::ffi::c_int = PROTO_BASE + 704 as ::core::ffi::c_int;
pub const MATOCL_FUSE_TIME_SYNC: ::core::ffi::c_int = PROTO_BASE + 705 as ::core::ffi::c_int;
pub const MATOCL_FUSE_INVALIDATE_CHUNK_CACHE: ::core::ffi::c_int =
    PROTO_BASE + 706 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_OPDATA: ::core::ffi::c_int = PROTO_BASE + 710 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_WFLAGS: ::core::ffi::c_int = PROTO_BASE + 711 as ::core::ffi::c_int;
pub const PROTO_BASE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const VERSMAJ: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const VERSMID: ::core::ffi::c_int = 59 as ::core::ffi::c_int;
pub const VERSMIN: ::core::ffi::c_int = 2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
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
#[inline]
unsafe extern "C" fn portable_sleep(mut sec: uint64_t) {
    unsafe {
        portable_usleep(sec.wrapping_mul(1000000 as uint64_t));
    }
}
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CONNECT_TIMEOUT: ::core::ffi::c_int = 2000 as ::core::ffi::c_int;
pub const THRECHASHSIZE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
static mut threchash: [*mut threc; 256] = [::core::ptr::null_mut::<threc>(); 256];
static mut threcfree: *mut threc = ::core::ptr::null_mut::<threc>();
static mut threcnextid: uint16_t = 0 as uint16_t;
pub const AMTIME_HASH_SIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const AMTIME_MAX_AGE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
static mut amtime_hash: [*mut amtime_file; 4096] = [::core::ptr::null_mut::<amtime_file>(); 4096];
pub const ACQFILES_HASH_SIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const ACQFILES_LRU_LIMIT: ::core::ffi::c_int = 5000 as ::core::ffi::c_int;
pub const ACQFILES_MAX_AGE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
static mut af_hash: [*mut acquired_file; 4096] = [::core::ptr::null_mut::<acquired_file>(); 4096];
static mut af_lrutail: *mut *mut acquired_file = ::core::ptr::null_mut::<*mut acquired_file>();
static mut af_lruhead: *mut acquired_file = ::core::ptr::null_mut::<acquired_file>();
static mut af_lru_cnt: uint32_t = 0;
static mut timediffusec: int64_t = 0 as int64_t;
pub const DEFAULT_OUTPUT_BUFFSIZE: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const DEFAULT_INPUT_BUFFSIZE: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
static mut sock_timeout: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
static mut recv_timeout: ::core::ffi::c_int = 300 as ::core::ffi::c_int;
static mut send_timeout: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
static mut fd: ::core::ffi::c_int = 0;
static mut disconnect: ::core::ffi::c_int = 0;
static mut donotsendsustainedinodes: ::core::ffi::c_int = 0;
static mut lastwrite: ::core::ffi::c_double = 0.;
static mut sessionlost: ::core::ffi::c_int = 0;
static mut lastsyncsend: uint64_t = 0 as uint64_t;
static mut usectimeout: uint64_t = 0;
static mut maxretries: uint32_t = 0;
// ponytail: c2rust dropped the _Atomic statics inside #if HAVE_ATOMICS
// (mastercomm.c:186-187); re-add them. Upgrade: report upstream.
static mut rcnt: uint32_t = 0;
static mut wcnt: uint32_t = 0;
static mut fcnt: uint32_t = 0;
static mut rbyt: uint64_t = 0;
static mut wbyt: uint64_t = 0;
// C rpthid/npthid: pthread_create'd in fs_init_threads (1 MiB stack via
// pthread_attr stacksize 0x100000), pthread_join'd in fs_term (nop first,
// then receive). C has no pthread_detach and no signal-mask block around
// these two spawns (unlike the readdata worker pool).
static RPT_HANDLE: std::sync::Mutex<Option<std::thread::JoinHandle<()>>> =
    std::sync::Mutex::new(None);
static NPT_HANDLE: std::sync::Mutex<Option<std::thread::JoinHandle<()>>> =
    std::sync::Mutex::new(None);
// Global locks replacing the C pthread_mutex_t statics fdlock/reclock/
// aflock/amtimelock. std::sync::Mutex is RAII-only, so emulate pthread-style
// manual lock/unlock by stashing the guard in a thread_local slot (same
// pattern as readdata.rs inode_global_lock). Verified against mastercomm.c:
// no pthread_cond_wait/timedwait pairs with any of these four globals
// (the only cond waits use threc->lock), and every lock/unlock pair runs on
// the same thread. Poisoning is ignored (into_inner) for pthread parity.
// NESTING AUDIT (why four separate slots): fs_sendandreceive/
// fs_sendandreceive_any hold FD_LOCK while locking/unlocking the per-threc
// lock; fs_receive_thread holds FD_LOCK then REC_LOCK while walking
// threchash; fs_nop_thread holds FD_LOCK across fs_send_open_inodes (AF_LOCK)
// and fs_send_amtime_inodes (AMTIME_LOCK). Lock order is always
// FD_LOCK < REC_LOCK < threc.lock and FD_LOCK < AF_LOCK / AMTIME_LOCK;
// AF_LOCK and AMTIME_LOCK never nest with each other or with REC_LOCK.
// ponytail: guard-slot emulates pthread_mutex_t; upgrade path is a full RAII
// restructure of every lock region (large diff, separate wave).
static FD_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
static REC_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
static AF_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
static AMTIME_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
thread_local! {
    static FD_LOCK_GUARD: std::cell::RefCell<Option<std::sync::MutexGuard<'static, ()>>> =
        const { std::cell::RefCell::new(None) };
    static REC_LOCK_GUARD: std::cell::RefCell<Option<std::sync::MutexGuard<'static, ()>>> =
        const { std::cell::RefCell::new(None) };
    static AF_LOCK_GUARD: std::cell::RefCell<Option<std::sync::MutexGuard<'static, ()>>> =
        const { std::cell::RefCell::new(None) };
    static AMTIME_LOCK_GUARD: std::cell::RefCell<Option<std::sync::MutexGuard<'static, ()>>> =
        const { std::cell::RefCell::new(None) };
}
macro_rules! global_lock_helpers {
    ($lock_fn:ident, $unlock_fn:ident, $lock:ident, $slot:ident) => {
        fn $lock_fn() {
            let guard = $lock.lock().unwrap_or_else(|e| e.into_inner());
            $slot.with(|slot| {
                let mut slot = slot.borrow_mut();
                assert!(
                    slot.is_none(),
                    concat!(stringify!($lock_fn), ": guard already held")
                );
                *slot = Some(guard);
            });
        }
        fn $unlock_fn() {
            let guard = $slot.with(|slot| slot.borrow_mut().take()).expect(concat!(
                stringify!($unlock_fn),
                ": no guard held on this thread"
            ));
            drop(guard);
        }
    };
}
global_lock_helpers!(fd_lock, fd_unlock, FD_LOCK, FD_LOCK_GUARD);
global_lock_helpers!(rec_lock, rec_unlock, REC_LOCK, REC_LOCK_GUARD);
global_lock_helpers!(af_lock, af_unlock, AF_LOCK, AF_LOCK_GUARD);
global_lock_helpers!(amtime_lock, amtime_unlock, AMTIME_LOCK, AMTIME_LOCK_GUARD);
// Per-threc lock: C rec->mutex (pthread_mutex_t) with rec->cond waiting on
// it. Same guard-slot emulation as the globals, but the slot also carries the
// mutex address so unlock/wait with a mismatched rec panics (pthread would
// UB). HOLD-OVERLAP AUDIT: no thread ever holds two threc locks at once —
// fs_receive_thread/fs_free_threc lock one rec at a time (unlock before
// moving to the next hash entry), fs_createpacket/fs_sendandreceive*/term
// each touch a single rec. Single slot suffices. Nests inside FD_LOCK and
// REC_LOCK at the sites documented above; it is always the innermost lock.
// SAFETY (guard lifetime): the guard borrows (*rec).lock inside Box'd
// threc; transmuted to 'static. Sound because the guard is only dropped via
// threc_unlock/threc_cond_* on the same thread, and threc is freed only in
// fs_term (after all threads are joined) or recycled via threcfree under
// REC_LOCK, which guarantees no holder or waiter survives.
// Poisoning is ignored (into_inner) for pthread parity.
thread_local! {
    static THREC_LOCK_GUARD: std::cell::RefCell<
        Option<(*const std::sync::Mutex<()>, std::sync::MutexGuard<'static, ()>)>,
    > = const { std::cell::RefCell::new(None) };
}
unsafe fn threc_lock(rec: *mut threc) {
    unsafe {
        let guard = (*rec).lock.lock().unwrap_or_else(|e| e.into_inner());
        // SAFETY: see THREC_LOCK_GUARD invariant above.
        let guard: std::sync::MutexGuard<'static, ()> = std::mem::transmute(guard);
        THREC_LOCK_GUARD.with(|slot| {
            let mut slot = slot.borrow_mut();
            assert!(slot.is_none(), "threc_lock: guard already held");
            *slot = Some((&raw const (*rec).lock, guard));
        });
    }
}
unsafe fn threc_unlock(rec: *mut threc) {
    unsafe {
        let entry = THREC_LOCK_GUARD
            .with(|slot| slot.borrow_mut().take())
            .expect("threc_unlock: no guard held on this thread");
        assert!(
            entry.0 == &raw const (*rec).lock,
            "threc_unlock: rec pointer mismatch"
        );
        drop(entry.1);
    }
}
// Caller must hold rec->lock; wait releases and reacquires it, exactly like
// pthread_cond_wait(&rec->cond, &rec->lock). C uses while-predicate loops at
// every site, so spurious wakeups are already handled there.
unsafe fn threc_cond_wait(rec: *mut threc) {
    unsafe {
        let entry = THREC_LOCK_GUARD
            .with(|slot| slot.borrow_mut().take())
            .expect("threc_cond_wait: no guard held on this thread");
        assert!(
            entry.0 == &raw const (*rec).lock,
            "threc_cond_wait: rec pointer mismatch"
        );
        let guard = (*rec).cond.wait(entry.1).unwrap_or_else(|e| e.into_inner());
        THREC_LOCK_GUARD.with(|slot| {
            *slot.borrow_mut() = Some((entry.0, guard));
        });
    }
}
// Returns true on timeout (C: pthread_cond_timedwait == ETIMEDOUT).
// Divergence from C: C builds a CLOCK_REALTIME abstime (gettimeofday +
// remaining usecs); std Condvar::wait_timeout takes a relative Duration of
// the same length on a monotonic clock — identical timeout, immune to
// wall-clock jumps (same choice as readdata.rs ind_cond_timedwait).
unsafe fn threc_cond_timedwait(rec: *mut threc, dur: std::time::Duration) -> bool {
    unsafe {
        let entry = THREC_LOCK_GUARD
            .with(|slot| slot.borrow_mut().take())
            .expect("threc_cond_timedwait: no guard held on this thread");
        assert!(
            entry.0 == &raw const (*rec).lock,
            "threc_cond_timedwait: rec pointer mismatch"
        );
        let (guard, res) = (*rec)
            .cond
            .wait_timeout(entry.1, dur)
            .unwrap_or_else(|e| e.into_inner());
        THREC_LOCK_GUARD.with(|slot| {
            *slot.borrow_mut() = Some((entry.0, guard));
        });
        res.timed_out()
    }
}
// C reckey pthread_key with fs_free_threc as destructor. Drop runs at
// thread exit — after the thread closure returns, before the pthread is
// torn down — mirroring the key destructor: fs_free_threc unregisters the
// rec from threchash under REC_LOCK and frees its buffers (the struct is
// recycled via threcfree, never freed), so fs_term's later threchash walk
// cannot see a freed entry — the same protocol as C. fs_term clears this
// slot after its explicit fs_free_threc(mainrec), mirroring
// pthread_key_delete(reckey); C setspecific(NULL) likewise maps to clearing
// the slot. const-init keeps the guard-slot TLS locals reachable from this
// destructor regardless of TLS destruction order.
struct MyThrec(std::cell::Cell<*mut threc>);
impl Drop for MyThrec {
    fn drop(&mut self) {
        if !self.0.get().is_null() {
            // SAFETY: this thread's registered threc; fs_free_threc only
            // unregisters it and frees its buffers (no struct free), so the
            // fs_term threchash/threcfree walk stays consistent.
            unsafe { fs_free_threc(self.0.get() as *mut ::core::ffi::c_void) };
        }
    }
}
thread_local! {
    static MY_THREC: MyThrec = const { MyThrec(std::cell::Cell::new(::core::ptr::null_mut())) };
}
static mut mainrec: *mut threc = ::core::ptr::null_mut::<threc>();
static mut sessionid: uint32_t = 0;
static mut metaid: uint64_t = 0;
static mut masterversion: uint32_t = 0;
static mut masterprocessid: uint64_t = 0;
static mut attrsize: uint8_t = 0;
static mut masterstrip: [::core::ffi::c_char; 16] = [0; 16];
static mut masterip: uint32_t = 0 as uint32_t;
static mut masterport: uint16_t = 0 as uint16_t;
static mut srcstrip: [::core::ffi::c_char; 16] = [0; 16];
static mut srcip: uint32_t = 0 as uint32_t;
static mut fterm: uint8_t = 0;
static mut working_flags: uint8_t = 0 as uint8_t;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_getmasterlocation(mut loc: *mut uint8_t) {
    unsafe {
        fd_lock();
        put32bit(&raw mut loc, masterip);
        put16bit(&raw mut loc, masterport);
        put32bit(&raw mut loc, sessionid);
        put32bit(&raw mut loc, masterversion);
        put64bit(&raw mut loc, masterprocessid);
        fd_unlock();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_getmasterparams(
    mut mip: *mut uint32_t,
    mut mport: *mut uint16_t,
    mut sid: *mut uint32_t,
    mut mver: *mut uint32_t,
    mut mprocid: *mut uint64_t,
) {
    unsafe {
        fd_lock();
        if !mip.is_null() {
            *mip = masterip;
        }
        if !mport.is_null() {
            *mport = masterport;
        }
        if !sid.is_null() {
            *sid = sessionid;
        }
        if !mver.is_null() {
            *mver = masterversion;
        }
        if !mprocid.is_null() {
            *mprocid = masterprocessid;
        }
        fd_unlock();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn master_version() -> uint32_t {
    unsafe {
        let mut mver: uint32_t = 0;
        fd_lock();
        mver = masterversion;
        fd_unlock();
        return mver;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn master_attrsize() -> uint8_t {
    unsafe {
        let mut asize: uint8_t = 0;
        fd_lock();
        asize = attrsize;
        fd_unlock();
        return asize;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_getsrcip() -> uint32_t {
    unsafe {
        let mut sip: uint32_t = 0;
        fd_lock();
        sip = srcip;
        fd_unlock();
        return sip;
    }
}
#[inline]
unsafe extern "C" fn copy_attr(
    mut rptr: *const uint8_t,
    mut attr: *mut uint8_t,
    mut asize: uint8_t,
) {
    unsafe {
        if asize as ::core::ffi::c_int >= ATTR_RECORD_SIZE {
            memcpy(
                attr as *mut ::core::ffi::c_void,
                rptr as *const ::core::ffi::c_void,
                ATTR_RECORD_SIZE as size_t,
            );
        } else {
            memcpy(
                attr as *mut ::core::ffi::c_void,
                rptr as *const ::core::ffi::c_void,
                asize as size_t,
            );
            memset(
                attr.offset(asize as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (ATTR_RECORD_SIZE - asize as ::core::ffi::c_int) as size_t,
            );
        };
    }
}
static STATS: std::sync::OnceLock<[crate::stats::StatsHandle; 7]> = std::sync::OnceLock::new();
static mut connect_args: connect_args_t = connect_args_t {
    bindhostname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    masterhostname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    masterportname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    meta: 0,
    clearpassword: 0,
    info: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    subfolder: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    passworddigest: ::core::ptr::null_mut::<uint8_t>(),
    minversion: 0,
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn master_statsptr_init() {
    let root = crate::stats::subnode(None, "master", false, false);
    assert!(
        STATS
            .set([
                crate::stats::subnode(Some(&root), "packets_received", false, true),
                crate::stats::subnode(Some(&root), "packets_sent", false, true),
                crate::stats::subnode(Some(&root), "bytes_received", false, true),
                crate::stats::subnode(Some(&root), "bytes_sent", false, true),
                crate::stats::subnode(Some(&root), "reconnects", false, true),
                crate::stats::subnode(Some(&root), "usec_ping", true, true),
                crate::stats::subnode(Some(&root), "usec_timediff", true, true),
            ])
            .is_ok(),
        "master stats initialized twice"
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn master_stats_inc(id: uint8_t) {
    if let Some(node) = STATS.get().and_then(|stats| stats.get(id as usize)) {
        crate::stats::counter_inc(node);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn master_stats_add(id: uint8_t, value: uint64_t) {
    if let Some(node) = STATS.get().and_then(|stats| stats.get(id as usize)) {
        crate::stats::counter_add(node, value);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn master_stats_set(id: uint8_t, value: uint64_t) {
    if let Some(node) = STATS.get().and_then(|stats| stats.get(id as usize)) {
        crate::stats::counter_set(node, value);
    }
}
#[unsafe(no_mangle)]
pub static mut errtab: [*const ::core::ffi::c_char; 65] = [
    b"OK\0".as_ptr() as *const ::core::ffi::c_char,
    b"Operation not permitted\0".as_ptr() as *const ::core::ffi::c_char,
    b"Not a directory\0".as_ptr() as *const ::core::ffi::c_char,
    b"No such file or directory\0".as_ptr() as *const ::core::ffi::c_char,
    b"Permission denied\0".as_ptr() as *const ::core::ffi::c_char,
    b"File exists\0".as_ptr() as *const ::core::ffi::c_char,
    b"Invalid argument\0".as_ptr() as *const ::core::ffi::c_char,
    b"Directory not empty\0".as_ptr() as *const ::core::ffi::c_char,
    b"Chunk lost\0".as_ptr() as *const ::core::ffi::c_char,
    b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
    b"Index too big\0".as_ptr() as *const ::core::ffi::c_char,
    b"Chunk locked\0".as_ptr() as *const ::core::ffi::c_char,
    b"No chunk servers\0".as_ptr() as *const ::core::ffi::c_char,
    b"No such chunk\0".as_ptr() as *const ::core::ffi::c_char,
    b"Chunk is busy\0".as_ptr() as *const ::core::ffi::c_char,
    b"Incorrect register BLOB\0".as_ptr() as *const ::core::ffi::c_char,
    b"Operation not completed\0".as_ptr() as *const ::core::ffi::c_char,
    b"File not opened\0".as_ptr() as *const ::core::ffi::c_char,
    b"Write not started\0".as_ptr() as *const ::core::ffi::c_char,
    b"Wrong chunk version\0".as_ptr() as *const ::core::ffi::c_char,
    b"Chunk already exists\0".as_ptr() as *const ::core::ffi::c_char,
    b"No space left\0".as_ptr() as *const ::core::ffi::c_char,
    b"IO error\0".as_ptr() as *const ::core::ffi::c_char,
    b"Incorrect block number\0".as_ptr() as *const ::core::ffi::c_char,
    b"Incorrect size\0".as_ptr() as *const ::core::ffi::c_char,
    b"Incorrect offset\0".as_ptr() as *const ::core::ffi::c_char,
    b"Can't connect\0".as_ptr() as *const ::core::ffi::c_char,
    b"Incorrect chunk id\0".as_ptr() as *const ::core::ffi::c_char,
    b"Disconnected\0".as_ptr() as *const ::core::ffi::c_char,
    b"CRC error\0".as_ptr() as *const ::core::ffi::c_char,
    b"Operation delayed\0".as_ptr() as *const ::core::ffi::c_char,
    b"Can't create path\0".as_ptr() as *const ::core::ffi::c_char,
    b"Data mismatch\0".as_ptr() as *const ::core::ffi::c_char,
    b"Read-only file system\0".as_ptr() as *const ::core::ffi::c_char,
    b"Quota exceeded\0".as_ptr() as *const ::core::ffi::c_char,
    b"Bad session id\0".as_ptr() as *const ::core::ffi::c_char,
    b"Password is needed\0".as_ptr() as *const ::core::ffi::c_char,
    b"Incorrect password\0".as_ptr() as *const ::core::ffi::c_char,
    b"Attribute not found\0".as_ptr() as *const ::core::ffi::c_char,
    b"Operation not supported\0".as_ptr() as *const ::core::ffi::c_char,
    b"Result too large\0".as_ptr() as *const ::core::ffi::c_char,
    b"Entity not found\0".as_ptr() as *const ::core::ffi::c_char,
    b"Entity is active\0".as_ptr() as *const ::core::ffi::c_char,
    b"Chunkserver not present\0".as_ptr() as *const ::core::ffi::c_char,
    b"Waiting on lock\0".as_ptr() as *const ::core::ffi::c_char,
    b"Resource temporarily unavailable\0".as_ptr() as *const ::core::ffi::c_char,
    b"Interrupted system call\0".as_ptr() as *const ::core::ffi::c_char,
    b"Operation canceled\0".as_ptr() as *const ::core::ffi::c_char,
    b"No such file or directory (not cacheable)\0".as_ptr() as *const ::core::ffi::c_char,
    b"Operation not permitted (mfs admin only)\0".as_ptr() as *const ::core::ffi::c_char,
    b"Class name already in use\0".as_ptr() as *const ::core::ffi::c_char,
    b"Maximum number of classes reached\0".as_ptr() as *const ::core::ffi::c_char,
    b"No such class\0".as_ptr() as *const ::core::ffi::c_char,
    b"Class in use\0".as_ptr() as *const ::core::ffi::c_char,
    b"One of MFS instance components is too old to perform this operation\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"Pattern already defined\0".as_ptr() as *const ::core::ffi::c_char,
    b"Maximum number of patterns reached\0".as_ptr() as *const ::core::ffi::c_char,
    b"No such pattern\0".as_ptr() as *const ::core::ffi::c_char,
    b"File name too long\0".as_ptr() as *const ::core::ffi::c_char,
    b"Too many links\0".as_ptr() as *const ::core::ffi::c_char,
    b"Operation timed out\0".as_ptr() as *const ::core::ffi::c_char,
    b"Bad file descriptor\0".as_ptr() as *const ::core::ffi::c_char,
    b"File too large\0".as_ptr() as *const ::core::ffi::c_char,
    b"Is a directory\0".as_ptr() as *const ::core::ffi::c_char,
    b"Unknown MFS error\0".as_ptr() as *const ::core::ffi::c_char,
];
#[inline]
unsafe extern "C" fn mfs_strerror(mut status: uint8_t) -> *const ::core::ffi::c_char {
    unsafe {
        if status as ::core::ffi::c_int > MFS_ERROR_MAX {
            status = MFS_ERROR_MAX as uint8_t;
        }
        return errtab[status as usize];
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_atime(mut inode: uint32_t) {
    unsafe {
        let mut amfptr: *mut amtime_file = ::core::ptr::null_mut::<amtime_file>();
        let mut amhash: uint32_t = 0;
        amtime_lock();
        amhash = inode.wrapping_rem(AMTIME_HASH_SIZE as uint32_t);
        amfptr = amtime_hash[amhash as usize];
        while !amfptr.is_null() {
            if (*amfptr).inode == inode {
                (*amfptr).atime = monotonic_useconds().wrapping_add(timediffusec as uint64_t);
                (*amfptr).atimeage = 0 as uint16_t;
                amtime_unlock();
                return;
            }
            amfptr = (*amfptr).next as *mut amtime_file;
        }
        amfptr = Box::into_raw(Box::new(_amtime_file {
            inode,
            atimeage: 0 as uint16_t,
            mtimeage: 0 as uint16_t,
            atime: monotonic_useconds().wrapping_add(timediffusec as uint64_t),
            mtime: 0 as uint64_t,
            next: amtime_hash[amhash as usize] as *mut _amtime_file,
        }));
        amtime_hash[amhash as usize] = amfptr;
        amtime_unlock();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mtime(mut inode: uint32_t) {
    unsafe {
        let mut amfptr: *mut amtime_file = ::core::ptr::null_mut::<amtime_file>();
        let mut amhash: uint32_t = 0;
        amtime_lock();
        amhash = inode.wrapping_rem(AMTIME_HASH_SIZE as uint32_t);
        amfptr = amtime_hash[amhash as usize];
        while !amfptr.is_null() {
            if (*amfptr).inode == inode {
                (*amfptr).mtime = monotonic_useconds().wrapping_add(timediffusec as uint64_t);
                (*amfptr).mtimeage = 0 as uint16_t;
                amtime_unlock();
                return;
            }
            amfptr = (*amfptr).next as *mut amtime_file;
        }
        amfptr = Box::into_raw(Box::new(_amtime_file {
            inode,
            atimeage: 0 as uint16_t,
            mtimeage: 0 as uint16_t,
            atime: 0 as uint64_t,
            mtime: monotonic_useconds().wrapping_add(timediffusec as uint64_t),
            next: amtime_hash[amhash as usize] as *mut _amtime_file,
        }));
        amtime_hash[amhash as usize] = amfptr;
        amtime_unlock();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_no_atime(mut inode: uint32_t) {
    unsafe {
        let mut amfptr: *mut amtime_file = ::core::ptr::null_mut::<amtime_file>();
        let mut amhash: uint32_t = 0;
        amtime_lock();
        amhash = inode.wrapping_rem(AMTIME_HASH_SIZE as uint32_t);
        amfptr = amtime_hash[amhash as usize];
        while !amfptr.is_null() {
            if (*amfptr).inode == inode {
                (*amfptr).atimeage = 0 as uint16_t;
                (*amfptr).atime = 0 as uint64_t;
                amtime_unlock();
                return;
            }
            amfptr = (*amfptr).next as *mut amtime_file;
        }
        amtime_unlock();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_no_mtime(mut inode: uint32_t) {
    unsafe {
        let mut amfptr: *mut amtime_file = ::core::ptr::null_mut::<amtime_file>();
        let mut amhash: uint32_t = 0;
        amtime_lock();
        amhash = inode.wrapping_rem(AMTIME_HASH_SIZE as uint32_t);
        amfptr = amtime_hash[amhash as usize];
        while !amfptr.is_null() {
            if (*amfptr).inode == inode {
                (*amfptr).mtimeage = 0 as uint16_t;
                (*amfptr).mtime = 0 as uint64_t;
                amtime_unlock();
                return;
            }
            amfptr = (*amfptr).next as *mut amtime_file;
        }
        amtime_unlock();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_fix_amtime(
    mut inode: uint32_t,
    mut atime: *mut uint32_t,
    mut mtime: *mut uint32_t,
) {
    unsafe {
        let mut amfptr: *mut amtime_file = ::core::ptr::null_mut::<amtime_file>();
        let mut amhash: uint32_t = 0;
        let mut ioatime: uint32_t = 0;
        let mut iomtime: uint32_t = 0;
        amtime_lock();
        amhash = inode.wrapping_rem(AMTIME_HASH_SIZE as uint32_t);
        amfptr = amtime_hash[amhash as usize];
        while !amfptr.is_null() {
            if (*amfptr).inode == inode {
                ioatime = (*amfptr).atime.wrapping_div(1000000 as uint64_t) as uint32_t;
                iomtime = (*amfptr).mtime.wrapping_div(1000000 as uint64_t) as uint32_t;
                if ioatime > *atime {
                    *atime = ioatime;
                }
                if iomtime > *mtime {
                    *mtime = iomtime;
                }
                amtime_unlock();
                return;
            }
            amfptr = (*amfptr).next as *mut amtime_file;
        }
        amtime_unlock();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_amtime_reference_clock(
    mut localmonotonic: uint64_t,
    mut remotewall: uint64_t,
) {
    unsafe {
        amtime_lock();
        timediffusec = remotewall.wrapping_sub(localmonotonic) as int64_t;
        amtime_unlock();
    }
}
unsafe extern "C" fn fs_af_remove_from_lru(mut afptr: *mut acquired_file) {
    unsafe {
        if !(*afptr).lrunext.is_null() {
            (*(*afptr).lrunext).lruprev = (*afptr).lruprev;
        } else {
            af_lrutail = (*afptr).lruprev as *mut *mut acquired_file;
        }
        *(*afptr).lruprev = (*afptr).lrunext;
        af_lru_cnt = af_lru_cnt.wrapping_sub(1);
        (*afptr).lrunext = ::core::ptr::null_mut::<_acquired_file>();
        (*afptr).lruprev = ::core::ptr::null_mut::<*mut _acquired_file>();
    }
}
unsafe extern "C" fn fs_af_add_to_lru(mut afptr: *mut acquired_file) {
    unsafe {
        let mut iafptr: *mut acquired_file = ::core::ptr::null_mut::<acquired_file>();
        let mut afpptr: *mut *mut acquired_file = ::core::ptr::null_mut::<*mut acquired_file>();
        let mut hash: uint32_t = 0;
        if af_lru_cnt > ACQFILES_LRU_LIMIT as uint32_t {
            hash = (*af_lruhead)
                .inode
                .wrapping_rem(ACQFILES_HASH_SIZE as uint32_t);
            afpptr = (&raw mut af_hash as *mut *mut acquired_file).offset(hash as isize);
            loop {
                iafptr = *afpptr;
                if iafptr.is_null() {
                    break;
                }
                if iafptr == af_lruhead {
                    *afpptr = (*iafptr).next as *mut acquired_file;
                    crate::chunksdatacache::clear_inode((*iafptr).inode, 0 as uint32_t);
                    fs_af_remove_from_lru(iafptr);
                    // C: free(iafptr) — pairs with Box::into_raw in
                    // fs_add_entry/fs_inc_acnt.
                    drop(Box::from_raw(iafptr));
                } else {
                    afpptr = &raw mut (*iafptr).next as *mut *mut acquired_file;
                }
            }
        }
        if af_lru_cnt <= 5000 as uint32_t {
        } else {
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mastercomm.c\0".as_ptr() as *const ::core::ffi::c_char,
                510 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"af_lru_cnt<=ACQFILES_LRU_LIMIT\0".as_ptr() as *const ::core::ffi::c_char,
                b"open files lru data mismatch !!!\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s' : %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mastercomm.c\0".as_ptr() as *const ::core::ffi::c_char,
                510 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"af_lru_cnt<=ACQFILES_LRU_LIMIT\0".as_ptr() as *const ::core::ffi::c_char,
                b"open files lru data mismatch !!!\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        (*afptr).lruprev = af_lrutail as *mut *mut _acquired_file;
        *af_lrutail = afptr;
        (*afptr).lrunext = ::core::ptr::null_mut::<_acquired_file>();
        af_lrutail = &raw mut (*afptr).lrunext as *mut *mut acquired_file;
        af_lru_cnt = af_lru_cnt.wrapping_add(1);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_add_entry(mut inode: uint32_t) {
    unsafe {
        let mut afhash: uint32_t = 0;
        let mut afptr: *mut acquired_file = ::core::ptr::null_mut::<acquired_file>();
        af_lock();
        afhash = inode.wrapping_rem(ACQFILES_HASH_SIZE as uint32_t);
        afptr = af_hash[afhash as usize];
        while !afptr.is_null() {
            if (*afptr).inode == inode {
                (*afptr).dentry = 1 as uint8_t;
                if !(*afptr).lruprev.is_null() {
                    fs_af_remove_from_lru(afptr);
                }
                (*afptr).age = 0 as uint8_t;
                af_unlock();
                return;
            }
            afptr = (*afptr).next as *mut acquired_file;
        }
        afptr = Box::into_raw(Box::new(_acquired_file {
            inode,
            cnt: 0 as uint16_t,
            age: 0 as uint8_t,
            dentry: 1 as uint8_t,
            next: af_hash[afhash as usize] as *mut _acquired_file,
            lrunext: ::core::ptr::null_mut::<_acquired_file>(),
            lruprev: ::core::ptr::null_mut::<*mut _acquired_file>(),
        }));
        af_hash[afhash as usize] = afptr;
        af_unlock();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_forget_entry(mut inode: uint32_t) {
    unsafe {
        let mut afhash: uint32_t = 0;
        let mut afptr: *mut acquired_file = ::core::ptr::null_mut::<acquired_file>();
        af_lock();
        afhash = inode.wrapping_rem(ACQFILES_HASH_SIZE as uint32_t);
        afptr = af_hash[afhash as usize];
        while !afptr.is_null() {
            if (*afptr).inode == inode {
                (*afptr).dentry = 0 as uint8_t;
                if (*afptr).cnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && (*afptr).lruprev.is_null()
                {
                    fs_af_add_to_lru(afptr);
                }
                (*afptr).age = 0 as uint8_t;
                af_unlock();
                return;
            }
            afptr = (*afptr).next as *mut acquired_file;
        }
        af_unlock();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_isopen(mut inode: uint32_t) -> ::core::ffi::c_int {
    unsafe {
        let mut afhash: uint32_t = 0;
        let mut afptr: *mut acquired_file = ::core::ptr::null_mut::<acquired_file>();
        af_lock();
        afhash = inode.wrapping_rem(ACQFILES_HASH_SIZE as uint32_t);
        afptr = af_hash[afhash as usize];
        while !afptr.is_null() {
            if (*afptr).inode == inode {
                if (*afptr).dentry as ::core::ffi::c_int != 0
                    || (*afptr).cnt as ::core::ffi::c_int != 0
                {
                    af_unlock();
                    return 1 as ::core::ffi::c_int;
                } else {
                    af_unlock();
                    return 0 as ::core::ffi::c_int;
                }
            }
            afptr = (*afptr).next as *mut acquired_file;
        }
        af_unlock();
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_inc_acnt(mut inode: uint32_t) {
    unsafe {
        let mut afhash: uint32_t = 0;
        let mut afptr: *mut acquired_file = ::core::ptr::null_mut::<acquired_file>();
        af_lock();
        afhash = inode.wrapping_rem(ACQFILES_HASH_SIZE as uint32_t);
        afptr = af_hash[afhash as usize];
        while !afptr.is_null() {
            if (*afptr).inode == inode {
                (*afptr).cnt = (*afptr).cnt.wrapping_add(1);
                if !(*afptr).lruprev.is_null() {
                    fs_af_remove_from_lru(afptr);
                }
                (*afptr).age = 0 as uint8_t;
                af_unlock();
                return;
            }
            afptr = (*afptr).next as *mut acquired_file;
        }
        afptr = Box::into_raw(Box::new(_acquired_file {
            inode,
            cnt: 1 as uint16_t,
            age: 0 as uint8_t,
            dentry: 0 as uint8_t,
            next: af_hash[afhash as usize] as *mut _acquired_file,
            lrunext: ::core::ptr::null_mut::<_acquired_file>(),
            lruprev: ::core::ptr::null_mut::<*mut _acquired_file>(),
        }));
        af_hash[afhash as usize] = afptr;
        af_unlock();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_dec_acnt(mut inode: uint32_t) {
    unsafe {
        let mut afhash: uint32_t = 0;
        let mut afptr: *mut acquired_file = ::core::ptr::null_mut::<acquired_file>();
        af_lock();
        afhash = inode.wrapping_rem(ACQFILES_HASH_SIZE as uint32_t);
        afptr = af_hash[afhash as usize];
        while !afptr.is_null() {
            if (*afptr).inode == inode {
                if (*afptr).cnt as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    (*afptr).cnt = (*afptr).cnt.wrapping_sub(1);
                }
                if (*afptr).cnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && (*afptr).dentry as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && (*afptr).lruprev.is_null()
                {
                    fs_af_add_to_lru(afptr);
                }
                (*afptr).age = 0 as uint8_t;
                af_unlock();
                return;
            }
            afptr = (*afptr).next as *mut acquired_file;
        }
        af_unlock();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_read_notify(mut bytes: uint64_t) {
    unsafe {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut rbyt,
            bytes,
        );
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut rcnt,
            1 as uint32_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_write_notify(mut bytes: uint64_t) {
    unsafe {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut wbyt,
            bytes,
        );
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut wcnt,
            1 as uint32_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_fsync_notify() {
    unsafe {
        ::core::intrinsics::atomic_xadd::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut fcnt,
            1 as uint32_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_init_counters() {
    unsafe {
        ::core::intrinsics::atomic_and::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut rbyt,
            0 as uint64_t,
        );
        ::core::intrinsics::atomic_and::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut rcnt,
            0 as uint32_t,
        );
        ::core::intrinsics::atomic_and::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut wbyt,
            0 as uint64_t,
        );
        ::core::intrinsics::atomic_and::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut wcnt,
            0 as uint32_t,
        );
        ::core::intrinsics::atomic_and::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut fcnt,
            0 as uint32_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_free_threc(mut vrec: *mut ::core::ffi::c_void) {
    unsafe {
        let mut drec: *mut threc = vrec as *mut threc;
        let mut rec: *mut threc = ::core::ptr::null_mut::<threc>();
        let mut recp: *mut *mut threc = ::core::ptr::null_mut::<*mut threc>();
        let mut rechash: uint32_t = 0;
        rec_lock();
        rechash = (*drec).packetid.wrapping_rem(THRECHASHSIZE as uint32_t);
        recp = (&raw mut threchash as *mut *mut threc).offset(rechash as isize);
        loop {
            rec = *recp;
            if rec.is_null() {
                break;
            }
            if rec == drec {
                *recp = (*rec).next as *mut threc;
                (*rec).next = threcfree as *mut _threc;
                threcfree = rec;
                threc_lock(rec);
                if !(*rec).obuff.is_null() {
                    // C: free(rec->obuff) — Box<[u8]> from
                    // fs_output_buffer_init, length from obuffsize.
                    drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                        (*rec).obuff,
                        (*rec).obuffsize as usize,
                    )));
                    (*rec).obuff = ::core::ptr::null_mut::<uint8_t>();
                    (*rec).obuffsize = 0 as uint32_t;
                }
                if !(*rec).ibuff.is_null() {
                    // C: free(rec->ibuff) — Box<[u8]> from fs_input_buffer_init.
                    drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                        (*rec).ibuff,
                        (*rec).ibuffsize as usize,
                    )));
                    (*rec).ibuff = ::core::ptr::null_mut::<uint8_t>();
                    (*rec).ibuffsize = 0 as uint32_t;
                }
                threc_unlock(rec);
                rec_unlock();
                return;
            } else {
                recp = &raw mut (*rec).next as *mut *mut threc;
            }
        }
        rec_unlock();
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"threc not found in data structures !!!\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_get_my_threc() -> *mut threc {
    unsafe {
        let mut rec: *mut threc = ::core::ptr::null_mut::<threc>();
        let mut rechash: uint32_t = 0;
        rec = MY_THREC.with(|slot| slot.0.get());
        if !rec.is_null() {
            return rec;
        }
        rec_lock();
        if !threcfree.is_null() {
            rec = threcfree;
            threcfree = (*rec).next as *mut threc;
        } else {
            // C: malloc(sizeof(threc)) + pthread_mutex/cond_init. Box literal
            // sets every field; freed with Box::from_raw in fs_term (Mutex/
            // Condvar Drop runs there, matching C's process-end teardown).
            threcnextid = threcnextid.wrapping_add(1);
            rec = Box::into_raw(Box::new(_threc {
                lock: std::sync::Mutex::new(()),
                cond: std::sync::Condvar::new(),
                obuff: ::core::ptr::null_mut::<uint8_t>(),
                obuffsize: 0 as uint32_t,
                odataleng: 0 as uint32_t,
                ibuff: ::core::ptr::null_mut::<uint8_t>(),
                ibuffsize: 0 as uint32_t,
                idataleng: 0 as uint32_t,
                sent: 0 as uint8_t,
                status: 0 as uint8_t,
                rcvd: 0 as uint8_t,
                receiving: 0 as uint8_t,
                rcvd_cmd: 0 as uint32_t,
                packetid: threcnextid as uint32_t,
                next: ::core::ptr::null_mut::<_threc>(),
            }));
        }
        rechash = (*rec).packetid.wrapping_rem(THRECHASHSIZE as uint32_t);
        (*rec).next = threchash[rechash as usize] as *mut _threc;
        threchash[rechash as usize] = rec;
        (*rec).obuff = ::core::ptr::null_mut::<uint8_t>();
        (*rec).ibuff = ::core::ptr::null_mut::<uint8_t>();
        (*rec).obuffsize = 0 as uint32_t;
        (*rec).ibuffsize = 0 as uint32_t;
        (*rec).odataleng = 0 as uint32_t;
        (*rec).idataleng = 0 as uint32_t;
        (*rec).sent = 0 as uint8_t;
        (*rec).status = 0 as uint8_t;
        (*rec).rcvd = 0 as uint8_t;
        (*rec).receiving = 0 as uint8_t;
        (*rec).rcvd_cmd = 0 as uint32_t;
        rec_unlock();
        MY_THREC.with(|slot| slot.0.set(rec));
        return rec;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_get_threc_by_id(mut packetid: uint32_t) -> *mut threc {
    unsafe {
        let mut rec: *mut threc = ::core::ptr::null_mut::<threc>();
        let mut rechash: uint32_t = 0;
        rechash = packetid.wrapping_rem(THRECHASHSIZE as uint32_t);
        rec_lock();
        rec = threchash[rechash as usize];
        while !rec.is_null() {
            if (*rec).packetid == packetid {
                rec_unlock();
                return rec;
            }
            rec = (*rec).next as *mut threc;
        }
        rec_unlock();
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"packet: %u - record not found !!!\0".as_ptr() as *const ::core::ffi::c_char,
            packetid,
        );
        return ::core::ptr::null_mut::<threc>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_output_buffer_init(mut rec: *mut threc, mut size: uint32_t) {
    unsafe {
        if size > DEFAULT_OUTPUT_BUFFSIZE as uint32_t {
            if !(*rec).obuff.is_null() {
                // C: free(rec->obuff) — Box<[u8]>, length from obuffsize.
                drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                    (*rec).obuff,
                    (*rec).obuffsize as usize,
                )));
            }
            // C: malloc(size) + passert. Box aborts on OOM (passert branch
            // dead); uninit payload matches C — caller writes the packet
            // before any read.
            (*rec).obuff = Box::into_raw(Box::<[u8]>::new_uninit_slice(size as usize).assume_init())
                as *mut uint8_t;
            (*rec).obuffsize = size;
        } else if (*rec).obuffsize != DEFAULT_OUTPUT_BUFFSIZE as uint32_t {
            if !(*rec).obuff.is_null() {
                // C: free(rec->obuff) — Box<[u8]>, length from obuffsize.
                drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                    (*rec).obuff,
                    (*rec).obuffsize as usize,
                )));
            }
            // C: malloc(DEFAULT_OUTPUT_BUFFSIZE) + passert (dead, Box aborts
            // on OOM).
            (*rec).obuff = Box::into_raw(
                Box::<[u8]>::new_uninit_slice(DEFAULT_OUTPUT_BUFFSIZE as usize).assume_init(),
            ) as *mut uint8_t;
            (*rec).obuffsize = DEFAULT_OUTPUT_BUFFSIZE as uint32_t;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_input_buffer_init(mut rec: *mut threc, mut size: uint32_t) {
    unsafe {
        if size > DEFAULT_INPUT_BUFFSIZE as uint32_t {
            if !(*rec).ibuff.is_null() {
                // C: free(rec->ibuff) — Box<[u8]>, length from ibuffsize.
                drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                    (*rec).ibuff,
                    (*rec).ibuffsize as usize,
                )));
            }
            // C: malloc(size) + passert (dead, Box aborts on OOM); uninit
            // payload matches C — receiver fills it from the socket.
            (*rec).ibuff = Box::into_raw(Box::<[u8]>::new_uninit_slice(size as usize).assume_init())
                as *mut uint8_t;
            (*rec).ibuffsize = size;
        } else if (*rec).ibuffsize != DEFAULT_INPUT_BUFFSIZE as uint32_t {
            if !(*rec).ibuff.is_null() {
                // C: free(rec->ibuff) — Box<[u8]>, length from ibuffsize.
                drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                    (*rec).ibuff,
                    (*rec).ibuffsize as usize,
                )));
            }
            // C: malloc(DEFAULT_INPUT_BUFFSIZE) + passert (dead, Box aborts
            // on OOM).
            (*rec).ibuff = Box::into_raw(
                Box::<[u8]>::new_uninit_slice(DEFAULT_INPUT_BUFFSIZE as usize).assume_init(),
            ) as *mut uint8_t;
            (*rec).ibuffsize = DEFAULT_INPUT_BUFFSIZE as uint32_t;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_createpacket(
    mut rec: *mut threc,
    mut cmd: uint32_t,
    mut size: uint32_t,
) -> *mut uint8_t {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut hdrsize: uint32_t = size.wrapping_add(4 as uint32_t);
        threc_lock(rec);
        fs_output_buffer_init(rec, size.wrapping_add(12 as uint32_t));
        if (*rec).obuff.is_null() {
            return ::core::ptr::null_mut::<uint8_t>();
        }
        ptr = (*rec).obuff;
        put32bit(&raw mut ptr, cmd);
        put32bit(&raw mut ptr, hdrsize);
        put32bit(&raw mut ptr, (*rec).packetid);
        (*rec).odataleng = size.wrapping_add(12 as uint32_t);
        threc_unlock(rec);
        return ptr;
    }
}
#[inline]
unsafe extern "C" fn fs_disconnect() {
    unsafe {
        ::core::intrinsics::atomic_or::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut disconnect,
            1 as ::core::ffi::c_int,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_sendandreceive(
    mut rec: *mut threc,
    mut expected_cmd: uint32_t,
    mut answer_leng: *mut uint32_t,
) -> *const uint8_t {
    unsafe {
        let mut cnt: uint32_t = 0;
        static mut notsup: uint8_t = MFS_ERROR_ENOTSUP as uint8_t;
        let mut start: uint64_t = 0;
        let mut period: uint64_t = 0;
        let mut usecto: uint64_t = 0;
        start = 0 as uint64_t;
        if usectimeout > 0 as uint64_t {
            start = monotonic_useconds();
        }
        cnt = 1 as uint32_t;
        while cnt <= maxretries {
            fd_lock();
            if sessionlost == 1 as ::core::ffi::c_int {
                fd_unlock();
                return ::core::ptr::null::<uint8_t>();
            }
            if fd == -1 as ::core::ffi::c_int {
                fd_unlock();
                usecto = (1000 as uint32_t).wrapping_add(if cnt < 30 as uint32_t {
                    cnt.wrapping_sub(1 as uint32_t)
                        .wrapping_mul(300000 as uint32_t)
                } else {
                    10000000 as uint32_t
                }) as uint64_t;
                if usectimeout > 0 as uint64_t {
                    period = monotonic_useconds().wrapping_sub(start);
                    if period >= usectimeout {
                        return ::core::ptr::null::<uint8_t>();
                    }
                    if usecto > usectimeout.wrapping_sub(period) {
                        usecto = usectimeout.wrapping_sub(period);
                    }
                }
                portable_usleep(usecto);
            } else {
                threc_lock(rec);
                if tcptowrite(
                    fd,
                    (*rec).obuff as *const ::core::ffi::c_void,
                    (*rec).odataleng,
                    1000 as uint32_t,
                    (send_timeout * 10 as ::core::ffi::c_int) as uint32_t,
                ) != (*rec).odataleng as int32_t
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"tcp send error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        strerr(*__errno_location()),
                    );
                    ::core::intrinsics::atomic_or::<
                        _,
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(&raw mut disconnect, 1 as ::core::ffi::c_int);
                    threc_unlock(rec);
                    fd_unlock();
                    usecto = (1000 as uint32_t).wrapping_add(if cnt < 30 as uint32_t {
                        cnt.wrapping_sub(1 as uint32_t)
                            .wrapping_mul(300000 as uint32_t)
                    } else {
                        10000000 as uint32_t
                    }) as uint64_t;
                    if usectimeout > 0 as uint64_t {
                        period = monotonic_useconds().wrapping_sub(start);
                        if period >= usectimeout {
                            return ::core::ptr::null::<uint8_t>();
                        }
                        if usecto > usectimeout.wrapping_sub(period) {
                            usecto = usectimeout.wrapping_sub(period);
                        }
                    }
                    portable_usleep(usecto);
                } else {
                    (*rec).rcvd = 0 as uint8_t;
                    (*rec).sent = 1 as uint8_t;
                    threc_unlock(rec);
                    master_stats_add(
                        MASTER_BYTESSENT as ::core::ffi::c_int as uint8_t,
                        (*rec).odataleng as uint64_t,
                    );
                    master_stats_inc(MASTER_PACKETSSENT as ::core::ffi::c_int as uint8_t);
                    lastwrite = monotonic_seconds();
                    fd_unlock();
                    threc_lock(rec);
                    while (*rec).rcvd as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        if usectimeout > 0 as uint64_t {
                            period = monotonic_useconds().wrapping_sub(start);
                            if period >= usectimeout {
                                threc_unlock(rec);
                                return ::core::ptr::null::<uint8_t>();
                            }
                            period = usectimeout.wrapping_sub(period);
                            // C builds a CLOCK_REALTIME abstime (gettimeofday +
                            // period) here; a relative Duration of the same
                            // length is the identical timeout (see
                            // threc_cond_timedwait).
                            if threc_cond_timedwait(rec, std::time::Duration::from_micros(period)) {
                                threc_unlock(rec);
                                return ::core::ptr::null::<uint8_t>();
                            }
                        } else {
                            threc_cond_wait(rec);
                        }
                    }
                    *answer_leng = (*rec).idataleng;
                    if (*rec).status as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        threc_unlock(rec);
                        usecto = (1000 as uint32_t).wrapping_add(if cnt < 30 as uint32_t {
                            cnt.wrapping_sub(1 as uint32_t)
                                .wrapping_mul(300000 as uint32_t)
                        } else {
                            10000000 as uint32_t
                        }) as uint64_t;
                        if usectimeout > 0 as uint64_t {
                            period = monotonic_useconds().wrapping_sub(start);
                            if period >= usectimeout {
                                return ::core::ptr::null::<uint8_t>();
                            }
                            if usecto > usectimeout.wrapping_sub(period) {
                                usecto = usectimeout.wrapping_sub(period);
                            }
                        }
                        portable_usleep(usecto);
                    } else {
                        if (*rec).rcvd_cmd == ANTOAN_UNKNOWN_COMMAND as uint32_t
                            || (*rec).rcvd_cmd == ANTOAN_BAD_COMMAND_SIZE as uint32_t
                        {
                            threc_unlock(rec);
                            *answer_leng = 1 as uint32_t;
                            return &raw mut notsup;
                        }
                        if (*rec).rcvd_cmd != expected_cmd {
                            threc_unlock(rec);
                            fs_disconnect();
                            usecto = (1000 as uint32_t).wrapping_add(if cnt < 30 as uint32_t {
                                cnt.wrapping_sub(1 as uint32_t)
                                    .wrapping_mul(300000 as uint32_t)
                            } else {
                                10000000 as uint32_t
                            }) as uint64_t;
                            if usectimeout > 0 as uint64_t {
                                period = monotonic_useconds().wrapping_sub(start);
                                if period >= usectimeout {
                                    return ::core::ptr::null::<uint8_t>();
                                }
                                if usecto > usectimeout.wrapping_sub(period) {
                                    usecto = usectimeout.wrapping_sub(period);
                                }
                            }
                            portable_usleep(usecto);
                        } else {
                            threc_unlock(rec);
                            return (*rec).ibuff;
                        }
                    }
                }
            }
            cnt = cnt.wrapping_add(1);
        }
        return ::core::ptr::null::<uint8_t>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_sendandreceive_any(
    mut rec: *mut threc,
    mut received_cmd: *mut uint32_t,
    mut answer_leng: *mut uint32_t,
) -> *const uint8_t {
    unsafe {
        let mut cnt: uint32_t = 0;
        let mut start: uint64_t = 0;
        let mut period: uint64_t = 0;
        let mut usecto: uint64_t = 0;
        start = 0 as uint64_t;
        if usectimeout > 0 as uint64_t {
            start = monotonic_useconds();
        }
        cnt = 1 as uint32_t;
        while cnt <= maxretries {
            fd_lock();
            if sessionlost == 1 as ::core::ffi::c_int {
                fd_unlock();
                return ::core::ptr::null::<uint8_t>();
            }
            if fd == -1 as ::core::ffi::c_int {
                fd_unlock();
                usecto = (1000 as uint32_t).wrapping_add(if cnt < 30 as uint32_t {
                    cnt.wrapping_sub(1 as uint32_t)
                        .wrapping_mul(300000 as uint32_t)
                } else {
                    10000000 as uint32_t
                }) as uint64_t;
                if usectimeout > 0 as uint64_t {
                    period = monotonic_useconds().wrapping_sub(start);
                    if period >= usectimeout {
                        return ::core::ptr::null::<uint8_t>();
                    }
                    if usecto > usectimeout.wrapping_sub(period) {
                        usecto = usectimeout.wrapping_sub(period);
                    }
                }
                portable_usleep(usecto);
            } else {
                threc_lock(rec);
                if tcptowrite(
                    fd,
                    (*rec).obuff as *const ::core::ffi::c_void,
                    (*rec).odataleng,
                    1000 as uint32_t,
                    (send_timeout * 1000 as ::core::ffi::c_int) as uint32_t,
                ) != (*rec).odataleng as int32_t
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"tcp send error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        strerr(*__errno_location()),
                    );
                    ::core::intrinsics::atomic_or::<
                        _,
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(&raw mut disconnect, 1 as ::core::ffi::c_int);
                    threc_unlock(rec);
                    fd_unlock();
                    usecto = (1000 as uint32_t).wrapping_add(if cnt < 30 as uint32_t {
                        cnt.wrapping_sub(1 as uint32_t)
                            .wrapping_mul(300000 as uint32_t)
                    } else {
                        10000000 as uint32_t
                    }) as uint64_t;
                    if usectimeout > 0 as uint64_t {
                        period = monotonic_useconds().wrapping_sub(start);
                        if period >= usectimeout {
                            return ::core::ptr::null::<uint8_t>();
                        }
                        if usecto > usectimeout.wrapping_sub(period) {
                            usecto = usectimeout.wrapping_sub(period);
                        }
                    }
                    portable_usleep(usecto);
                } else {
                    (*rec).rcvd = 0 as uint8_t;
                    (*rec).sent = 1 as uint8_t;
                    threc_unlock(rec);
                    master_stats_add(
                        MASTER_BYTESSENT as ::core::ffi::c_int as uint8_t,
                        (*rec).odataleng as uint64_t,
                    );
                    master_stats_inc(MASTER_PACKETSSENT as ::core::ffi::c_int as uint8_t);
                    lastwrite = monotonic_seconds();
                    fd_unlock();
                    threc_lock(rec);
                    while (*rec).rcvd as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        if usectimeout > 0 as uint64_t {
                            period = monotonic_useconds().wrapping_sub(start);
                            if period >= usectimeout {
                                threc_unlock(rec);
                                return ::core::ptr::null::<uint8_t>();
                            }
                            period = usectimeout.wrapping_sub(period);
                            // C builds a CLOCK_REALTIME abstime (gettimeofday +
                            // period) here; a relative Duration of the same
                            // length is the identical timeout (see
                            // threc_cond_timedwait).
                            if threc_cond_timedwait(rec, std::time::Duration::from_micros(period)) {
                                threc_unlock(rec);
                                return ::core::ptr::null::<uint8_t>();
                            }
                        } else {
                            threc_cond_wait(rec);
                        }
                    }
                    *answer_leng = (*rec).idataleng;
                    if (*rec).status as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        threc_unlock(rec);
                        usecto = (1000 as uint32_t).wrapping_add(if cnt < 30 as uint32_t {
                            cnt.wrapping_sub(1 as uint32_t)
                                .wrapping_mul(300000 as uint32_t)
                        } else {
                            10000000 as uint32_t
                        }) as uint64_t;
                        if usectimeout > 0 as uint64_t {
                            period = monotonic_useconds().wrapping_sub(start);
                            if period >= usectimeout {
                                return ::core::ptr::null::<uint8_t>();
                            }
                            if usecto > usectimeout.wrapping_sub(period) {
                                usecto = usectimeout.wrapping_sub(period);
                            }
                        }
                        portable_usleep(usecto);
                    } else {
                        *received_cmd = (*rec).rcvd_cmd;
                        threc_unlock(rec);
                        return (*rec).ibuff;
                    }
                }
            }
            cnt = cnt.wrapping_add(1);
        }
        return ::core::ptr::null::<uint8_t>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_resolve(
    mut oninit: uint8_t,
    mut bindhostname: *const ::core::ffi::c_char,
    mut masterhostname: *const ::core::ffi::c_char,
    mut masterportname: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if !bindhostname.is_null() {
            if tcpresolve(
                bindhostname,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut srcip,
                ::core::ptr::null_mut::<uint16_t>(),
                1 as ::core::ffi::c_int,
            ) < 0 as ::core::ffi::c_int
            {
                if oninit != 0 {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"can't resolve source hostname (%s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        bindhostname,
                    );
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"can't resolve source hostname (%s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        bindhostname,
                    );
                }
                return -1 as ::core::ffi::c_int;
            }
        } else {
            srcip = 0 as uint32_t;
        }
        univmakestrip(&raw mut srcstrip as *mut ::core::ffi::c_char, srcip);
        if tcpresolve(
            masterhostname,
            masterportname,
            &raw mut masterip,
            &raw mut masterport,
            0 as ::core::ffi::c_int,
        ) < 0 as ::core::ffi::c_int
        {
            if oninit != 0 {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"can't resolve master hostname and/or portname (%s:%s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    masterhostname,
                    masterportname,
                );
            } else {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"can't resolve master hostname and/or portname (%s:%s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    masterhostname,
                    masterportname,
                );
            }
            return -1 as ::core::ffi::c_int;
        }
        univmakestrip(&raw mut masterstrip as *mut ::core::ffi::c_char, masterip);
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_get_current_srcstrip() -> *const ::core::ffi::c_char {
    return &raw mut srcstrip as *mut ::core::ffi::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_get_current_masterstrip() -> *const ::core::ffi::c_char {
    return &raw mut masterstrip as *mut ::core::ffi::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_get_current_masterport() -> uint16_t {
    unsafe {
        return masterport;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_connect(
    mut oninit: uint8_t,
    mut cargs: *mut connect_args_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: uint32_t = 0;
        let mut j: uint32_t = 0;
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut regbuff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut ctx: md5ctx = md5ctx {
            state: [0; 4],
            count: [0; 2],
            buffer: [0; 64],
        };
        let mut digest: [uint8_t; 16] = [0; 16];
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut newmasterip: uint32_t = 0;
        let mut havepassword: uint8_t = 0;
        let mut pleng: uint32_t = 0;
        let mut ileng: uint32_t = 0;
        let mut sesflags: uint8_t = 0;
        let mut umaskval: uint16_t = 0;
        let mut rootuid: uint32_t = 0;
        let mut rootgid: uint32_t = 0;
        let mut mapalluid: uint32_t = 0;
        let mut mapallgid: uint32_t = 0;
        let mut mingoal: uint8_t = 0;
        let mut maxgoal: uint8_t = 0;
        let mut sclassgroups: int32_t = 0;
        let mut mintrashretention: uint32_t = 0;
        let mut maxtrashretention: uint32_t = 0;
        let mut disables: uint32_t = 0;
        let mut rleng: int32_t = 0;
        let mut disablestr: [*const ::core::ffi::c_char; 26] = [
            b"chown\0".as_ptr() as *const ::core::ffi::c_char,
            b"chmod\0".as_ptr() as *const ::core::ffi::c_char,
            b"symlink\0".as_ptr() as *const ::core::ffi::c_char,
            b"mkfifo\0".as_ptr() as *const ::core::ffi::c_char,
            b"mkdev\0".as_ptr() as *const ::core::ffi::c_char,
            b"mksock\0".as_ptr() as *const ::core::ffi::c_char,
            b"mkdir\0".as_ptr() as *const ::core::ffi::c_char,
            b"unlink\0".as_ptr() as *const ::core::ffi::c_char,
            b"rmdir\0".as_ptr() as *const ::core::ffi::c_char,
            b"rename\0".as_ptr() as *const ::core::ffi::c_char,
            b"move\0".as_ptr() as *const ::core::ffi::c_char,
            b"link\0".as_ptr() as *const ::core::ffi::c_char,
            b"create\0".as_ptr() as *const ::core::ffi::c_char,
            b"readdir\0".as_ptr() as *const ::core::ffi::c_char,
            b"read\0".as_ptr() as *const ::core::ffi::c_char,
            b"write\0".as_ptr() as *const ::core::ffi::c_char,
            b"truncate\0".as_ptr() as *const ::core::ffi::c_char,
            b"setlength\0".as_ptr() as *const ::core::ffi::c_char,
            b"appendchunks\0".as_ptr() as *const ::core::ffi::c_char,
            b"snapshot\0".as_ptr() as *const ::core::ffi::c_char,
            b"settrash\0".as_ptr() as *const ::core::ffi::c_char,
            b"setsclass\0".as_ptr() as *const ::core::ffi::c_char,
            b"seteattr\0".as_ptr() as *const ::core::ffi::c_char,
            b"setxattr\0".as_ptr() as *const ::core::ffi::c_char,
            b"setfacl\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ];
        let mut sesflagposstrtab: [*const ::core::ffi::c_char; 8] = [
            b"read-only\0".as_ptr() as *const ::core::ffi::c_char,
            b"not_restricted_ip\0".as_ptr() as *const ::core::ffi::c_char,
            b"ignore_gid\0".as_ptr() as *const ::core::ffi::c_char,
            b"admin\0".as_ptr() as *const ::core::ffi::c_char,
            b"map_all\0".as_ptr() as *const ::core::ffi::c_char,
            b"undefined_flag_5\0".as_ptr() as *const ::core::ffi::c_char,
            b"reserved (attr bit)\0".as_ptr() as *const ::core::ffi::c_char,
            b"reserved (metarestore)\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        let mut sesflagnegstrtab: [*const ::core::ffi::c_char; 8] = [
            b"read-write\0".as_ptr() as *const ::core::ffi::c_char,
            b"restricted_ip\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ];
        let mut infobuff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut ibleng: uint32_t = 0;
        let mut pwd: passwd = passwd {
            pw_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pw_passwd: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pw_uid: 0,
            pw_gid: 0,
            pw_gecos: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pw_dir: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pw_shell: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        let mut pw: *mut passwd = ::core::ptr::null_mut::<passwd>();
        let mut grp: group = group {
            gr_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            gr_passwd: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            gr_gid: 0,
            gr_mem: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        };
        let mut gr: *mut group = ::core::ptr::null_mut::<group>();
        let mut pwdgrpbuff: [::core::ffi::c_char; 16384] = [0; 16384];
        static mut trycnt: uint32_t = 0 as uint32_t;
        if fs_resolve(
            oninit,
            (*cargs).bindhostname,
            (*cargs).masterhostname,
            (*cargs).masterportname,
        ) < 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        havepassword = (if (*cargs).passworddigest.is_null() {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as uint8_t;
        ileng = strlen((*cargs).info).wrapping_add(1 as size_t) as uint32_t;
        if (*cargs).meta != 0 {
            pleng = 0 as uint32_t;
            rleng = 9 as ::core::ffi::c_int as int32_t;
        } else {
            pleng = strlen((*cargs).subfolder).wrapping_add(1 as size_t) as uint32_t;
            rleng = 13 as ::core::ffi::c_int as int32_t;
        }
        rleng = (rleng as uint32_t).wrapping_add(
            ((8 as ::core::ffi::c_int + 64 as ::core::ffi::c_int) as uint32_t)
                .wrapping_add(pleng)
                .wrapping_add(ileng),
        ) as int32_t;
        if havepassword != 0 {
            rleng = (rleng as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as int32_t;
        }
        if sessionlost == 2 as ::core::ffi::c_int {
            rleng = (rleng as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as int32_t;
            if metaid != 0 as uint64_t {
                rleng = (rleng as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as int32_t;
            }
        }
        // C: regbuff = malloc(rleng). Box<[u8]> leaked as raw ptr; uninit
        // matches C (put32bit/memcpy fill it before any read). Every C
        // free(regbuff) site below is on a return path; the retry loop's
        // `continue` paths keep it alive, same as C.
        regbuff = Box::into_raw(Box::<[u8]>::new_uninit_slice(rleng as usize).assume_init())
            as *mut uint8_t;
        loop {
            fd = tcpsocket();
            if fd < 0 as ::core::ffi::c_int {
                drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                    regbuff,
                    rleng as usize,
                )));
                return -1 as ::core::ffi::c_int;
            }
            if tcpnodelay(fd) < 0 as ::core::ffi::c_int {
                if oninit != 0 {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_NOTICE,
                        b"can't set TCP_NODELAY\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_NOTICE,
                        b"can't set TCP_NODELAY\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            }
            if srcip > 0 as uint32_t {
                if tcpnumbind(fd, srcip, 0 as uint16_t) < 0 as ::core::ffi::c_int {
                    if oninit != 0 {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"can't bind socket to given ip (\"%s\")\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut srcstrip as *mut ::core::ffi::c_char,
                        );
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"can't bind socket to given ip (\"%s\")\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut srcstrip as *mut ::core::ffi::c_char,
                        );
                    }
                    tcpclose(fd);
                    fd = -1 as ::core::ffi::c_int;
                    drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                        regbuff,
                        rleng as usize,
                    )));
                    return -1 as ::core::ffi::c_int;
                }
            }
            if tcpnumtoconnect(fd, masterip, masterport, CONNECT_TIMEOUT as uint32_t)
                < 0 as ::core::ffi::c_int
            {
                tcpclose(fd);
                fd = -1 as ::core::ffi::c_int;
                if oninit != 0 {
                    if trycnt < 10 as uint32_t {
                        trycnt = trycnt.wrapping_add(1);
                        if fs_resolve(
                            oninit,
                            (*cargs).bindhostname,
                            (*cargs).masterhostname,
                            (*cargs).masterportname,
                        ) < 0 as ::core::ffi::c_int
                        {
                            drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                                regbuff,
                                rleng as usize,
                            )));
                            return -1 as ::core::ffi::c_int;
                        }
                        i = 4 as uint32_t;
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"can't connect to mfsmaster (\"%s\":\"%hu\")\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut masterstrip as *mut ::core::ffi::c_char,
                            masterport as ::core::ffi::c_int,
                        );
                        drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                            regbuff,
                            rleng as usize,
                        )));
                        return -1 as ::core::ffi::c_int;
                    }
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"can't connect to mfsmaster (\"%s\":\"%hu\")\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut masterstrip as *mut ::core::ffi::c_char,
                        masterport as ::core::ffi::c_int,
                    );
                    drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                        regbuff,
                        rleng as usize,
                    )));
                    return -1 as ::core::ffi::c_int;
                }
            } else {
                if havepassword != 0 {
                    wptr = regbuff;
                    put32bit(&raw mut wptr, CLTOMA_FUSE_REGISTER as uint32_t);
                    put32bit(&raw mut wptr, 65 as uint32_t);
                    memcpy(
                        wptr as *mut ::core::ffi::c_void,
                        FUSE_REGISTER_BLOB_ACL.as_ptr() as *const ::core::ffi::c_void,
                        64 as size_t,
                    );
                    wptr = wptr.offset(64 as ::core::ffi::c_int as isize);
                    put8bit(&raw mut wptr, REGISTER_GETRANDOM as uint8_t);
                    if tcptowrite(
                        fd,
                        regbuff as *const ::core::ffi::c_void,
                        (8 as ::core::ffi::c_int + 65 as ::core::ffi::c_int) as uint32_t,
                        1000 as uint32_t,
                        (send_timeout * 1000 as ::core::ffi::c_int) as uint32_t,
                    ) != 8 as int32_t + 65 as int32_t
                    {
                        if oninit != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"error sending data to mfsmaster\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"error sending data to mfsmaster\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                        tcpclose(fd);
                        fd = -1 as ::core::ffi::c_int;
                        drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                            regbuff,
                            rleng as usize,
                        )));
                        return -1 as ::core::ffi::c_int;
                    }
                    if tcptoread(
                        fd,
                        regbuff as *mut ::core::ffi::c_void,
                        8 as uint32_t,
                        1000 as uint32_t,
                        (recv_timeout * 1000 as ::core::ffi::c_int) as uint32_t,
                    ) != 8 as int32_t
                    {
                        if oninit != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"error receiving data from mfsmaster\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"error receiving data from mfsmaster\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                        tcpclose(fd);
                        fd = -1 as ::core::ffi::c_int;
                        drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                            regbuff,
                            rleng as usize,
                        )));
                        return -1 as ::core::ffi::c_int;
                    }
                    rptr = regbuff;
                    i = get32bit(&raw mut rptr);
                    if i != MATOCL_FUSE_REGISTER as uint32_t {
                        if oninit != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"got incorrect answer from mfsmaster\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"got incorrect answer from mfsmaster\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                        tcpclose(fd);
                        fd = -1 as ::core::ffi::c_int;
                        drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                            regbuff,
                            rleng as usize,
                        )));
                        return -1 as ::core::ffi::c_int;
                    }
                    i = get32bit(&raw mut rptr);
                    if i != 32 as uint32_t {
                        if oninit != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"got incorrect answer from mfsmaster\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"got incorrect answer from mfsmaster\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                        tcpclose(fd);
                        fd = -1 as ::core::ffi::c_int;
                        drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                            regbuff,
                            rleng as usize,
                        )));
                        return -1 as ::core::ffi::c_int;
                    }
                    if tcptoread(
                        fd,
                        regbuff as *mut ::core::ffi::c_void,
                        32 as uint32_t,
                        1000 as uint32_t,
                        (recv_timeout * 1000 as ::core::ffi::c_int) as uint32_t,
                    ) != 32 as int32_t
                    {
                        if oninit != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"error receiving data from mfsmaster\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"error receiving data from mfsmaster\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                        tcpclose(fd);
                        fd = -1 as ::core::ffi::c_int;
                        drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                            regbuff,
                            rleng as usize,
                        )));
                        return -1 as ::core::ffi::c_int;
                    }
                    md5_init(&raw mut ctx);
                    md5_update(&raw mut ctx, regbuff, 16 as uint32_t);
                    md5_update(&raw mut ctx, (*cargs).passworddigest, 16 as uint32_t);
                    md5_update(
                        &raw mut ctx,
                        regbuff.offset(16 as ::core::ffi::c_int as isize),
                        16 as uint32_t,
                    );
                    md5_final(&raw mut digest as *mut uint8_t, &raw mut ctx);
                }
                wptr = regbuff;
                put32bit(&raw mut wptr, CLTOMA_FUSE_REGISTER as uint32_t);
                put32bit(&raw mut wptr, (rleng - 8 as int32_t) as uint32_t);
                memcpy(
                    wptr as *mut ::core::ffi::c_void,
                    FUSE_REGISTER_BLOB_ACL.as_ptr() as *const ::core::ffi::c_void,
                    64 as size_t,
                );
                wptr = wptr.offset(64 as ::core::ffi::c_int as isize);
                put8bit(
                    &raw mut wptr,
                    (if (*cargs).meta as ::core::ffi::c_int != 0 {
                        REGISTER_NEWMETASESSION
                    } else {
                        REGISTER_NEWSESSION
                    }) as uint8_t,
                );
                put16bit(&raw mut wptr, VERSMAJ as uint16_t);
                put8bit(&raw mut wptr, VERSMID as uint8_t);
                put8bit(&raw mut wptr, VERSMIN as uint8_t);
                put32bit(&raw mut wptr, ileng);
                memcpy(
                    wptr as *mut ::core::ffi::c_void,
                    (*cargs).info as *const ::core::ffi::c_void,
                    ileng as size_t,
                );
                wptr = wptr.offset(ileng as isize);
                if (*cargs).meta == 0 {
                    put32bit(&raw mut wptr, pleng);
                    memcpy(
                        wptr as *mut ::core::ffi::c_void,
                        (*cargs).subfolder as *const ::core::ffi::c_void,
                        pleng as size_t,
                    );
                    wptr = wptr.offset(pleng as isize);
                }
                if sessionlost == 2 as ::core::ffi::c_int {
                    put32bit(&raw mut wptr, sessionid);
                    if metaid != 0 as uint64_t {
                        put64bit(&raw mut wptr, metaid);
                    }
                }
                if havepassword != 0 {
                    memcpy(
                        wptr as *mut ::core::ffi::c_void,
                        &raw mut digest as *mut uint8_t as *const ::core::ffi::c_void,
                        16 as size_t,
                    );
                }
                if tcptowrite(
                    fd,
                    regbuff as *const ::core::ffi::c_void,
                    rleng as uint32_t,
                    1000 as uint32_t,
                    (send_timeout * 1000 as ::core::ffi::c_int) as uint32_t,
                ) != rleng
                {
                    if oninit != 0 {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"error sending data to mfsmaster: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            strerr(*__errno_location()),
                        );
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"error sending data to mfsmaster: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            strerr(*__errno_location()),
                        );
                    }
                    tcpclose(fd);
                    fd = -1 as ::core::ffi::c_int;
                    drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                        regbuff,
                        rleng as usize,
                    )));
                    return -1 as ::core::ffi::c_int;
                }
                if tcptoread(
                    fd,
                    regbuff as *mut ::core::ffi::c_void,
                    8 as uint32_t,
                    1000 as uint32_t,
                    (recv_timeout * 1000 as ::core::ffi::c_int) as uint32_t,
                ) != 8 as int32_t
                {
                    if oninit != 0 {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"error receiving data from mfsmaster: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            strerr(*__errno_location()),
                        );
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"error receiving data from mfsmaster: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            strerr(*__errno_location()),
                        );
                    }
                    tcpclose(fd);
                    fd = -1 as ::core::ffi::c_int;
                    drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                        regbuff,
                        rleng as usize,
                    )));
                    return -1 as ::core::ffi::c_int;
                }
                rptr = regbuff;
                i = get32bit(&raw mut rptr);
                if i != MATOCL_FUSE_REGISTER as uint32_t {
                    if oninit != 0 {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"got incorrect answer from mfsmaster\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"got incorrect answer from mfsmaster\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                    tcpclose(fd);
                    fd = -1 as ::core::ffi::c_int;
                    drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                        regbuff,
                        rleng as usize,
                    )));
                    return -1 as ::core::ffi::c_int;
                }
                i = get32bit(&raw mut rptr);
                if !(i == 1 as uint32_t
                    || i == 4 as uint32_t
                    || (*cargs).meta as ::core::ffi::c_int != 0
                        && (i == 19 as uint32_t || i == 27 as uint32_t || i == 35 as uint32_t)
                    || (*cargs).meta as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        && (i == 35 as uint32_t
                            || i == 43 as uint32_t
                            || i == 45 as uint32_t
                            || i == 49 as uint32_t
                            || i == 57 as uint32_t))
                {
                    if oninit != 0 {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"got incorrect answer from mfsmaster\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"got incorrect answer from mfsmaster\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                    tcpclose(fd);
                    fd = -1 as ::core::ffi::c_int;
                    drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                        regbuff,
                        rleng as usize,
                    )));
                    return -1 as ::core::ffi::c_int;
                }
                if tcptoread(
                    fd,
                    regbuff as *mut ::core::ffi::c_void,
                    i,
                    1000 as uint32_t,
                    (recv_timeout * 1000 as ::core::ffi::c_int) as uint32_t,
                ) != i as int32_t
                {
                    if oninit != 0 {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"error receiving data from mfsmaster: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            strerr(*__errno_location()),
                        );
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"error receiving data from mfsmaster: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            strerr(*__errno_location()),
                        );
                    }
                    tcpclose(fd);
                    fd = -1 as ::core::ffi::c_int;
                    drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                        regbuff,
                        rleng as usize,
                    )));
                    return -1 as ::core::ffi::c_int;
                }
                rptr = regbuff;
                if i == 1 as uint32_t {
                    if oninit != 0 {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"mfsmaster register error: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            mfs_strerror(*rptr.offset(0 as isize)),
                        );
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"mfsmaster register error: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            mfs_strerror(*rptr.offset(0 as isize)),
                        );
                    }
                    tcpclose(fd);
                    fd = -1 as ::core::ffi::c_int;
                    drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                        regbuff,
                        rleng as usize,
                    )));
                    return -1 as ::core::ffi::c_int;
                }
                if i == 4 as uint32_t {
                    newmasterip = get32bit(&raw mut rptr);
                    if newmasterip == 0 as uint32_t || newmasterip == masterip {
                        if oninit != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_NOTICE,
                                b"mfsmaster %s - got empty redirect - retrying\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut masterstrip as *mut ::core::ffi::c_char,
                            );
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_NOTICE,
                                b"mfsmaster %s - got empty redirect - retrying\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut masterstrip as *mut ::core::ffi::c_char,
                            );
                        }
                        tcpclose(fd);
                        fd = -1 as ::core::ffi::c_int;
                        if oninit != 0 {
                            portable_sleep(2 as uint64_t);
                        } else {
                            drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                                regbuff,
                                rleng as usize,
                            )));
                            return -1 as ::core::ffi::c_int;
                        }
                    } else {
                        let mut newmasterstrip: [::core::ffi::c_char; 16] = [0; 16];
                        univmakestrip(
                            &raw mut newmasterstrip as *mut ::core::ffi::c_char,
                            newmasterip,
                        );
                        if oninit != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_NOTICE,
                                b"mfsmaster %s - got redirect to %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut masterstrip as *mut ::core::ffi::c_char,
                                &raw mut newmasterstrip as *mut ::core::ffi::c_char,
                            );
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_NOTICE,
                                b"mfsmaster %s - got redirect to %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut masterstrip as *mut ::core::ffi::c_char,
                                &raw mut newmasterstrip as *mut ::core::ffi::c_char,
                            );
                        }
                        masterip = newmasterip;
                        strcpy(
                            &raw mut masterstrip as *mut ::core::ffi::c_char,
                            &raw mut newmasterstrip as *mut ::core::ffi::c_char,
                        );
                        tcpclose(fd);
                        fd = -1 as ::core::ffi::c_int;
                    }
                }
            }
            if i != 4 as uint32_t {
                break;
            }
        }
        masterversion = get32bit(&raw mut rptr);
        if masterversion
            < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    7 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    7 as ::core::ffi::c_int
                })) as uint32_t
            || masterversion < (*cargs).minversion
        {
            if oninit != 0 {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"incompatible mfsmaster version\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"incompatible mfsmaster version\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            tcpclose(fd);
            fd = -1 as ::core::ffi::c_int;
            drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                regbuff,
                rleng as usize,
            )));
            return -1 as ::core::ffi::c_int;
        }
        attrsize = (if masterversion
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    93 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    93 as ::core::ffi::c_int
                })) as uint32_t
            && masterversion
                != (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
            && masterversion
                != (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    })) as uint32_t
        {
            ATTR_RECORD_SIZE
        } else {
            35 as ::core::ffi::c_int
        }) as uint8_t;
        sessionid = get32bit(&raw mut rptr);
        if (*cargs).meta as ::core::ffi::c_int != 0 && (i == 27 as uint32_t || i == 35 as uint32_t)
            || (*cargs).meta as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && (i == 43 as uint32_t
                    || i == 45 as uint32_t
                    || i == 49 as uint32_t
                    || i == 57 as uint32_t)
        {
            metaid = get64bit(&raw mut rptr);
        }
        sesflags = get8bit(&raw mut rptr);
        if (*cargs).meta == 0 {
            if i == 45 as uint32_t || i == 49 as uint32_t || i == 57 as uint32_t {
                umaskval = get16bit(&raw mut rptr);
            } else {
                umaskval = 0 as uint16_t;
            }
            rootuid = get32bit(&raw mut rptr);
            rootgid = get32bit(&raw mut rptr);
            mapalluid = get32bit(&raw mut rptr);
            mapallgid = get32bit(&raw mut rptr);
        } else {
            umaskval = 0 as uint16_t;
            rootuid = 0 as uint32_t;
            rootgid = 0 as uint32_t;
            mapalluid = 0 as uint32_t;
            mapallgid = 0 as uint32_t;
        }
        if masterversion
            < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 57 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            mingoal = get8bit(&raw mut rptr);
            maxgoal = get8bit(&raw mut rptr);
            sclassgroups = -1 as ::core::ffi::c_int as int32_t;
        } else {
            mingoal = 1 as uint8_t;
            maxgoal = 9 as uint8_t;
            sclassgroups = get16bit(&raw mut rptr) as int32_t;
        }
        mintrashretention = get32bit(&raw mut rptr);
        maxtrashretention = get32bit(&raw mut rptr);
        if (*cargs).meta == 0 {
            if i == 49 as uint32_t || i == 57 as uint32_t {
                disables = get32bit(&raw mut rptr);
            } else {
                disables = 0 as uint32_t;
            }
        } else {
            disables = 0 as uint32_t;
        }
        if (*cargs).meta as ::core::ffi::c_int != 0 && i == 35 as uint32_t
            || (*cargs).meta as ::core::ffi::c_int == 0 as ::core::ffi::c_int && i == 57 as uint32_t
        {
            masterprocessid = get64bit(&raw mut rptr);
        } else {
            masterprocessid = metaid;
        }
        drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
            regbuff,
            rleng as usize,
        )));
        lastwrite = monotonic_seconds();
        if oninit as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if sessionlost == 2 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_INFO,
                    b"registered to master using previous session\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            } else {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_INFO,
                    b"registered to master with new session\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        }
        if (*cargs).clearpassword as ::core::ffi::c_int != 0 && !(*cargs).passworddigest.is_null() {
            memset(
                (*cargs).passworddigest as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                16 as size_t,
            );
            // C: free(cargs->passworddigest) — Box<[u8; 16]> from
            // fs_init_master_connection.
            drop(Box::from_raw((*cargs).passworddigest as *mut [uint8_t; 16]));
            (*cargs).passworddigest = ::core::ptr::null_mut::<uint8_t>();
        }
        // C: infobuff = malloc(INFOBUFF_SIZE). Zeroed (unlike C's malloc)
        // because the mfs_log %s below reads it as a C string even when no
        // sesflag bit set a byte — malloc garbage there was latent C UB.
        infobuff = Box::into_raw(vec![0 as uint8_t; INFOBUFF_SIZE as usize].into_boxed_slice())
            as *mut ::core::ffi::c_char;
        ibleng = 0 as uint32_t;
        j = 0 as uint32_t;
        i = 0 as uint32_t;
        while i < 8 as uint32_t {
            if sesflags as ::core::ffi::c_int & (1 as ::core::ffi::c_int) << i != 0 {
                if ibleng < INFOBUFF_SIZE as uint32_t {
                    ibleng = ibleng.wrapping_add(snprintf(
                        infobuff.offset(ibleng as isize),
                        (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                        b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
                        if j != 0 {
                            b",\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"\0".as_ptr() as *const ::core::ffi::c_char
                        },
                        sesflagposstrtab[i as usize],
                    ) as uint32_t);
                }
                j = 1 as uint32_t;
            } else if !sesflagnegstrtab[i as usize].is_null() {
                if ibleng < INFOBUFF_SIZE as uint32_t {
                    ibleng = ibleng.wrapping_add(snprintf(
                        infobuff.offset(ibleng as isize),
                        (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                        b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
                        if j != 0 {
                            b",\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"\0".as_ptr() as *const ::core::ffi::c_char
                        },
                        sesflagnegstrtab[i as usize],
                    ) as uint32_t);
                }
                j = 1 as uint32_t;
            }
            i = i.wrapping_add(1);
        }
        if j == 0 as uint32_t {
            if ibleng < INFOBUFF_SIZE as uint32_t {
                ibleng = ibleng.wrapping_add(snprintf(
                    infobuff.offset(ibleng as isize),
                    (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                    b"-\0".as_ptr() as *const ::core::ffi::c_char,
                ) as uint32_t);
            }
        }
        if (*cargs).meta == 0 {
            if umaskval as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                if ibleng < INFOBUFF_SIZE as uint32_t {
                    ibleng = ibleng.wrapping_add(snprintf(
                        infobuff.offset(ibleng as isize),
                        (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                        b" ; global umask set to 0%03ho\0".as_ptr() as *const ::core::ffi::c_char,
                        umaskval as ::core::ffi::c_int,
                    ) as uint32_t);
                }
            }
            if ibleng < INFOBUFF_SIZE as uint32_t {
                ibleng = ibleng.wrapping_add(snprintf(
                    infobuff.offset(ibleng as isize),
                    (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                    b" ; root mapped to \0".as_ptr() as *const ::core::ffi::c_char,
                ) as uint32_t);
            }
            getpwuid_r(
                rootuid as __uid_t,
                &raw mut pwd,
                &raw mut pwdgrpbuff as *mut ::core::ffi::c_char,
                16384 as size_t,
                &raw mut pw,
            );
            if !pw.is_null() {
                if ibleng < INFOBUFF_SIZE as uint32_t {
                    ibleng = ibleng.wrapping_add(snprintf(
                        infobuff.offset(ibleng as isize),
                        (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                        b"%s:\0".as_ptr() as *const ::core::ffi::c_char,
                        (*pw).pw_name,
                    ) as uint32_t);
                }
            } else if ibleng < INFOBUFF_SIZE as uint32_t {
                ibleng = ibleng.wrapping_add(snprintf(
                    infobuff.offset(ibleng as isize),
                    (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                    b"%u:\0".as_ptr() as *const ::core::ffi::c_char,
                    rootuid,
                ) as uint32_t);
            }
            getgrgid_r(
                rootgid as __gid_t,
                &raw mut grp,
                &raw mut pwdgrpbuff as *mut ::core::ffi::c_char,
                16384 as size_t,
                &raw mut gr,
            );
            if !gr.is_null() {
                if ibleng < INFOBUFF_SIZE as uint32_t {
                    ibleng = ibleng.wrapping_add(snprintf(
                        infobuff.offset(ibleng as isize),
                        (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        (*gr).gr_name,
                    ) as uint32_t);
                }
            } else if ibleng < INFOBUFF_SIZE as uint32_t {
                ibleng = ibleng.wrapping_add(snprintf(
                    infobuff.offset(ibleng as isize),
                    (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                    b"%u\0".as_ptr() as *const ::core::ffi::c_char,
                    rootgid,
                ) as uint32_t);
            }
            if sesflags as ::core::ffi::c_int & SESFLAG_MAPALL != 0 {
                if ibleng < INFOBUFF_SIZE as uint32_t {
                    ibleng = ibleng.wrapping_add(snprintf(
                        infobuff.offset(ibleng as isize),
                        (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                        b" ; users mapped to \0".as_ptr() as *const ::core::ffi::c_char,
                    ) as uint32_t);
                }
                pw = getpwuid(mapalluid as __uid_t);
                if !pw.is_null() {
                    if ibleng < INFOBUFF_SIZE as uint32_t {
                        ibleng = ibleng.wrapping_add(snprintf(
                            infobuff.offset(ibleng as isize),
                            (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                            b"%s:\0".as_ptr() as *const ::core::ffi::c_char,
                            (*pw).pw_name,
                        ) as uint32_t);
                    }
                } else if ibleng < INFOBUFF_SIZE as uint32_t {
                    ibleng = ibleng.wrapping_add(snprintf(
                        infobuff.offset(ibleng as isize),
                        (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                        b"%u:\0".as_ptr() as *const ::core::ffi::c_char,
                        mapalluid,
                    ) as uint32_t);
                }
                gr = getgrgid(mapallgid as __gid_t);
                if !gr.is_null() {
                    if ibleng < INFOBUFF_SIZE as uint32_t {
                        ibleng = ibleng.wrapping_add(snprintf(
                            infobuff.offset(ibleng as isize),
                            (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            (*gr).gr_name,
                        ) as uint32_t);
                    }
                } else if ibleng < INFOBUFF_SIZE as uint32_t {
                    ibleng = ibleng.wrapping_add(snprintf(
                        infobuff.offset(ibleng as isize),
                        (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                        b"%u\0".as_ptr() as *const ::core::ffi::c_char,
                        mapallgid,
                    ) as uint32_t);
                }
            }
        }
        if mingoal as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && maxgoal as ::core::ffi::c_int > 0 as ::core::ffi::c_int
        {
            if mingoal as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                || (maxgoal as ::core::ffi::c_int) < 9 as ::core::ffi::c_int
            {
                if ibleng < INFOBUFF_SIZE as uint32_t {
                    ibleng = ibleng.wrapping_add(snprintf(
                        infobuff.offset(ibleng as isize),
                        (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                        b" ; setgoal limited to (%u:%u)\0".as_ptr() as *const ::core::ffi::c_char,
                        mingoal as ::core::ffi::c_int,
                        maxgoal as ::core::ffi::c_int,
                    ) as uint32_t);
                }
            }
            if sclassgroups >= 0 as int32_t {
                if ibleng < INFOBUFF_SIZE as uint32_t {
                    ibleng = ibleng.wrapping_add(snprintf(
                        infobuff.offset(ibleng as isize),
                        (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                        b" ; sclass groups allowed: \0".as_ptr() as *const ::core::ffi::c_char,
                    ) as uint32_t);
                }
                if sclassgroups == 0xffff as int32_t {
                    if ibleng < INFOBUFF_SIZE as uint32_t {
                        ibleng = ibleng.wrapping_add(snprintf(
                            infobuff.offset(ibleng as isize),
                            (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                            b"ALL\0".as_ptr() as *const ::core::ffi::c_char,
                        ) as uint32_t);
                    }
                } else {
                    j = 0 as uint32_t;
                    i = 0 as uint32_t;
                    while i < EXPORT_GROUPS as uint32_t {
                        if (1 as int32_t) << i & sclassgroups != 0 {
                            if j != 0 {
                                if ibleng < INFOBUFF_SIZE as uint32_t {
                                    ibleng = ibleng.wrapping_add(snprintf(
                                        infobuff.offset(ibleng as isize),
                                        (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                                        b",%u\0".as_ptr() as *const ::core::ffi::c_char,
                                        i,
                                    )
                                        as uint32_t);
                                }
                            } else {
                                if ibleng < INFOBUFF_SIZE as uint32_t {
                                    ibleng = ibleng.wrapping_add(snprintf(
                                        infobuff.offset(ibleng as isize),
                                        (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                                        b"%u\0".as_ptr() as *const ::core::ffi::c_char,
                                        i,
                                    )
                                        as uint32_t);
                                }
                                j = 1 as uint32_t;
                            }
                        }
                        i = i.wrapping_add(1);
                    }
                    if j == 0 as uint32_t {
                        if ibleng < INFOBUFF_SIZE as uint32_t {
                            ibleng = ibleng.wrapping_add(snprintf(
                                infobuff.offset(ibleng as isize),
                                (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                                b"-\0".as_ptr() as *const ::core::ffi::c_char,
                            ) as uint32_t);
                        }
                    }
                }
            }
            if mintrashretention > 0 as uint32_t || maxtrashretention < 0xffffffff as uint32_t {
                if ibleng < INFOBUFF_SIZE as uint32_t {
                    ibleng = ibleng.wrapping_add(snprintf(
                        infobuff.offset(ibleng as isize),
                        (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                        b" ; settrashretention limited to (\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    ) as uint32_t);
                }
                if mintrashretention > 0 as uint32_t {
                    if mintrashretention > 604800 as uint32_t {
                        if ibleng < INFOBUFF_SIZE as uint32_t {
                            ibleng = ibleng.wrapping_add(snprintf(
                                infobuff.offset(ibleng as isize),
                                (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                                b"%uw\0".as_ptr() as *const ::core::ffi::c_char,
                                mintrashretention.wrapping_div(604800 as uint32_t),
                            ) as uint32_t);
                        }
                        mintrashretention = mintrashretention.wrapping_rem(604800 as uint32_t);
                    }
                    if mintrashretention > 86400 as uint32_t {
                        if ibleng < INFOBUFF_SIZE as uint32_t {
                            ibleng = ibleng.wrapping_add(snprintf(
                                infobuff.offset(ibleng as isize),
                                (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                                b"%ud\0".as_ptr() as *const ::core::ffi::c_char,
                                mintrashretention.wrapping_div(86400 as uint32_t),
                            ) as uint32_t);
                        }
                        mintrashretention = mintrashretention.wrapping_rem(86400 as uint32_t);
                    }
                    if mintrashretention > 3600 as uint32_t {
                        if ibleng < INFOBUFF_SIZE as uint32_t {
                            ibleng = ibleng.wrapping_add(snprintf(
                                infobuff.offset(ibleng as isize),
                                (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                                b"%uh\0".as_ptr() as *const ::core::ffi::c_char,
                                mintrashretention.wrapping_div(3600 as uint32_t),
                            ) as uint32_t);
                        }
                        mintrashretention = mintrashretention.wrapping_rem(3600 as uint32_t);
                    }
                    if mintrashretention > 60 as uint32_t {
                        if ibleng < INFOBUFF_SIZE as uint32_t {
                            ibleng = ibleng.wrapping_add(snprintf(
                                infobuff.offset(ibleng as isize),
                                (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                                b"%um\0".as_ptr() as *const ::core::ffi::c_char,
                                mintrashretention.wrapping_div(60 as uint32_t),
                            ) as uint32_t);
                        }
                        mintrashretention = mintrashretention.wrapping_rem(60 as uint32_t);
                    }
                    if mintrashretention > 0 as uint32_t {
                        if ibleng < INFOBUFF_SIZE as uint32_t {
                            ibleng = ibleng.wrapping_add(snprintf(
                                infobuff.offset(ibleng as isize),
                                (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                                b"%us\0".as_ptr() as *const ::core::ffi::c_char,
                                mintrashretention,
                            ) as uint32_t);
                        }
                    }
                } else if ibleng < INFOBUFF_SIZE as uint32_t {
                    ibleng = ibleng.wrapping_add(snprintf(
                        infobuff.offset(ibleng as isize),
                        (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                        b"0s\0".as_ptr() as *const ::core::ffi::c_char,
                    ) as uint32_t);
                }
                if ibleng < INFOBUFF_SIZE as uint32_t {
                    ibleng = ibleng.wrapping_add(snprintf(
                        infobuff.offset(ibleng as isize),
                        (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                        b":\0".as_ptr() as *const ::core::ffi::c_char,
                    ) as uint32_t);
                }
                if maxtrashretention > 0 as uint32_t {
                    if maxtrashretention > 604800 as uint32_t {
                        if ibleng < INFOBUFF_SIZE as uint32_t {
                            ibleng = ibleng.wrapping_add(snprintf(
                                infobuff.offset(ibleng as isize),
                                (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                                b"%uw\0".as_ptr() as *const ::core::ffi::c_char,
                                maxtrashretention.wrapping_div(604800 as uint32_t),
                            ) as uint32_t);
                        }
                        maxtrashretention = maxtrashretention.wrapping_rem(604800 as uint32_t);
                    }
                    if maxtrashretention > 86400 as uint32_t {
                        if ibleng < INFOBUFF_SIZE as uint32_t {
                            ibleng = ibleng.wrapping_add(snprintf(
                                infobuff.offset(ibleng as isize),
                                (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                                b"%ud\0".as_ptr() as *const ::core::ffi::c_char,
                                maxtrashretention.wrapping_div(86400 as uint32_t),
                            ) as uint32_t);
                        }
                        maxtrashretention = maxtrashretention.wrapping_rem(86400 as uint32_t);
                    }
                    if maxtrashretention > 3600 as uint32_t {
                        if ibleng < INFOBUFF_SIZE as uint32_t {
                            ibleng = ibleng.wrapping_add(snprintf(
                                infobuff.offset(ibleng as isize),
                                (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                                b"%uh\0".as_ptr() as *const ::core::ffi::c_char,
                                maxtrashretention.wrapping_div(3600 as uint32_t),
                            ) as uint32_t);
                        }
                        maxtrashretention = maxtrashretention.wrapping_rem(3600 as uint32_t);
                    }
                    if maxtrashretention > 60 as uint32_t {
                        if ibleng < INFOBUFF_SIZE as uint32_t {
                            ibleng = ibleng.wrapping_add(snprintf(
                                infobuff.offset(ibleng as isize),
                                (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                                b"%um\0".as_ptr() as *const ::core::ffi::c_char,
                                maxtrashretention.wrapping_div(60 as uint32_t),
                            ) as uint32_t);
                        }
                        maxtrashretention = maxtrashretention.wrapping_rem(60 as uint32_t);
                    }
                    if maxtrashretention > 0 as uint32_t {
                        if ibleng < INFOBUFF_SIZE as uint32_t {
                            ibleng = ibleng.wrapping_add(snprintf(
                                infobuff.offset(ibleng as isize),
                                (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                                b"%us\0".as_ptr() as *const ::core::ffi::c_char,
                                maxtrashretention,
                            ) as uint32_t);
                        }
                    }
                } else if ibleng < INFOBUFF_SIZE as uint32_t {
                    ibleng = ibleng.wrapping_add(snprintf(
                        infobuff.offset(ibleng as isize),
                        (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                        b"0s\0".as_ptr() as *const ::core::ffi::c_char,
                    ) as uint32_t);
                }
                if ibleng < INFOBUFF_SIZE as uint32_t {
                    ibleng = ibleng.wrapping_add(snprintf(
                        infobuff.offset(ibleng as isize),
                        (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                        b")\0".as_ptr() as *const ::core::ffi::c_char,
                    ) as uint32_t);
                }
            }
        }
        if disables > 0 as uint32_t {
            let mut s: ::core::ffi::c_int = 0;
            if ibleng < INFOBUFF_SIZE as uint32_t {
                ibleng = ibleng.wrapping_add(snprintf(
                    infobuff.offset(ibleng as isize),
                    (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                    b" ; disabled commands: \0".as_ptr() as *const ::core::ffi::c_char,
                ) as uint32_t);
            }
            s = 0 as ::core::ffi::c_int;
            i = 0 as uint32_t;
            j = 1 as uint32_t;
            while !disablestr[i as usize].is_null() {
                if disables & j != 0 {
                    if ibleng < INFOBUFF_SIZE as uint32_t {
                        ibleng = ibleng.wrapping_add(snprintf(
                            infobuff.offset(ibleng as isize),
                            (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                            b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
                            if s != 0 {
                                b",\0".as_ptr() as *const ::core::ffi::c_char
                            } else {
                                b"\0".as_ptr() as *const ::core::ffi::c_char
                            },
                            disablestr[i as usize],
                        ) as uint32_t);
                    }
                    s = 1 as ::core::ffi::c_int;
                }
                i = i.wrapping_add(1);
                j <<= 1 as ::core::ffi::c_int;
            }
        }
        if ibleng < INFOBUFF_SIZE as uint32_t {
            ibleng = ibleng.wrapping_add(snprintf(
                infobuff.offset(ibleng as isize),
                (INFOBUFF_SIZE as uint32_t).wrapping_sub(ibleng) as size_t,
                b"\n\0".as_ptr() as *const ::core::ffi::c_char,
            ) as uint32_t);
        }
        if ibleng < INFOBUFF_SIZE as uint32_t {
            *infobuff.offset(ibleng as isize) = '\0' as ::core::ffi::c_char;
        } else {
            *infobuff.offset((INFOBUFF_SIZE - 1 as ::core::ffi::c_int) as isize) =
                '\0' as ::core::ffi::c_char;
        }
        if oninit != 0 {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_INFO,
                b"mfsmaster accepted connection with parameters: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                infobuff,
            );
        } else {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_INFO,
                b"mfsmaster accepted connection with parameters: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                infobuff,
            );
        }
        drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
            infobuff as *mut uint8_t,
            INFOBUFF_SIZE as usize,
        )));
        return 0 as ::core::ffi::c_int;
    }
}
pub const INFOBUFF_SIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_reconnect(mut minversion: uint32_t) {
    unsafe {
        let mut newmasterip: uint32_t = 0;
        let mut i: uint32_t = 0;
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut regbuff: [uint8_t; 89] = [0; 89];
        let mut rleng: int32_t = 0;
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        if sessionid == 0 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"can't register using previous sessionid\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        univmakestrip(&raw mut masterstrip as *mut ::core::ffi::c_char, masterip);
        loop {
            fd = tcpsocket();
            if fd < 0 as ::core::ffi::c_int {
                return;
            }
            if tcpnodelay(fd) < 0 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_NOTICE,
                    b"can't set TCP_NODELAY: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    strerr(*__errno_location()),
                );
            }
            if srcip > 0 as uint32_t {
                if tcpnumbind(fd, srcip, 0 as uint16_t) < 0 as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"can't bind socket to given ip (\"%s\")\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut srcstrip as *mut ::core::ffi::c_char,
                    );
                    tcpclose(fd);
                    fd = -1 as ::core::ffi::c_int;
                    return;
                }
            }
            if tcpnumtoconnect(fd, masterip, masterport, CONNECT_TIMEOUT as uint32_t)
                < 0 as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"can't connect to master (\"%s\":\"%hu\")\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut masterstrip as *mut ::core::ffi::c_char,
                    masterport as ::core::ffi::c_int,
                );
                tcpclose(fd);
                fd = -1 as ::core::ffi::c_int;
                return;
            }
            master_stats_inc(MASTER_CONNECTS as ::core::ffi::c_int as uint8_t);
            wptr = &raw mut regbuff as *mut uint8_t;
            put32bit(&raw mut wptr, CLTOMA_FUSE_REGISTER as uint32_t);
            if masterversion
                >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        11 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        11 as ::core::ffi::c_int
                    })) as uint32_t
                && metaid != 0 as uint64_t
            {
                put32bit(&raw mut wptr, 81 as uint32_t);
                rleng = (8 as ::core::ffi::c_int + 81 as ::core::ffi::c_int) as int32_t;
            } else {
                put32bit(&raw mut wptr, 73 as uint32_t);
                rleng = (8 as ::core::ffi::c_int + 73 as ::core::ffi::c_int) as int32_t;
            }
            memcpy(
                wptr as *mut ::core::ffi::c_void,
                FUSE_REGISTER_BLOB_ACL.as_ptr() as *const ::core::ffi::c_void,
                64 as size_t,
            );
            wptr = wptr.offset(64 as ::core::ffi::c_int as isize);
            put8bit(&raw mut wptr, REGISTER_RECONNECT as uint8_t);
            put32bit(&raw mut wptr, sessionid);
            put16bit(&raw mut wptr, VERSMAJ as uint16_t);
            put8bit(&raw mut wptr, VERSMID as uint8_t);
            put8bit(&raw mut wptr, VERSMIN as uint8_t);
            put64bit(&raw mut wptr, metaid);
            if tcptowrite(
                fd,
                &raw mut regbuff as *mut uint8_t as *const ::core::ffi::c_void,
                rleng as uint32_t,
                1000 as uint32_t,
                (send_timeout * 1000 as ::core::ffi::c_int) as uint32_t,
            ) != rleng
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"master: register error (write: %s)\0".as_ptr() as *const ::core::ffi::c_char,
                    strerr(*__errno_location()),
                );
                tcpclose(fd);
                fd = -1 as ::core::ffi::c_int;
                return;
            }
            master_stats_add(
                MASTER_BYTESSENT as ::core::ffi::c_int as uint8_t,
                rleng as uint64_t,
            );
            master_stats_inc(MASTER_PACKETSSENT as ::core::ffi::c_int as uint8_t);
            if tcptoread(
                fd,
                &raw mut regbuff as *mut uint8_t as *mut ::core::ffi::c_void,
                8 as uint32_t,
                1000 as uint32_t,
                (recv_timeout * 1000 as ::core::ffi::c_int) as uint32_t,
            ) != 8 as int32_t
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"master: register error (read header: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    strerr(*__errno_location()),
                );
                tcpclose(fd);
                fd = -1 as ::core::ffi::c_int;
                return;
            }
            master_stats_add(
                MASTER_BYTESRCVD as ::core::ffi::c_int as uint8_t,
                8 as uint64_t,
            );
            rptr = &raw mut regbuff as *mut uint8_t;
            i = get32bit(&raw mut rptr);
            if i != MATOCL_FUSE_REGISTER as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"master: register error (bad answer: %u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    i,
                );
                tcpclose(fd);
                fd = -1 as ::core::ffi::c_int;
                return;
            }
            i = get32bit(&raw mut rptr);
            if i != 1 as uint32_t && i != 4 as uint32_t && i != 5 as uint32_t && i != 13 as uint32_t
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"master: register error (bad length: %u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    i,
                );
                tcpclose(fd);
                fd = -1 as ::core::ffi::c_int;
                return;
            }
            if tcptoread(
                fd,
                &raw mut regbuff as *mut uint8_t as *mut ::core::ffi::c_void,
                i,
                1000 as uint32_t,
                (recv_timeout * 1000 as ::core::ffi::c_int) as uint32_t,
            ) != i as int32_t
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"master: register error (read data: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    strerr(*__errno_location()),
                );
                tcpclose(fd);
                fd = -1 as ::core::ffi::c_int;
                return;
            }
            master_stats_add(
                MASTER_BYTESRCVD as ::core::ffi::c_int as uint8_t,
                i as uint64_t,
            );
            master_stats_inc(MASTER_PACKETSRCVD as ::core::ffi::c_int as uint8_t);
            rptr = &raw mut regbuff as *mut uint8_t;
            if i == 4 as uint32_t {
                newmasterip = get32bit(&raw mut rptr);
                if newmasterip == 0 as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_NOTICE,
                        b"mfsmaster %s - got empty redirect\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut masterstrip as *mut ::core::ffi::c_char,
                    );
                    tcpclose(fd);
                    fd = -1 as ::core::ffi::c_int;
                    return;
                } else if newmasterip == masterip {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_NOTICE,
                        b"mfsmaster %s - got self redirect\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        &raw mut masterstrip as *mut ::core::ffi::c_char,
                    );
                    tcpclose(fd);
                    fd = -1 as ::core::ffi::c_int;
                    return;
                } else {
                    masterip = newmasterip;
                    univmakestrip(&raw mut masterstrip as *mut ::core::ffi::c_char, masterip);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_NOTICE,
                        b"mfsmaster sent redirect to: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut masterstrip as *mut ::core::ffi::c_char,
                    );
                    tcpclose(fd);
                    fd = -1 as ::core::ffi::c_int;
                }
            }
            if i != 4 as uint32_t {
                break;
            }
        }
        if i >= 5 as uint32_t {
            masterversion = get32bit(&raw mut rptr);
            if masterversion
                < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        7 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        7 as ::core::ffi::c_int
                    })) as uint32_t
                || masterversion < minversion
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"incompatible mfsmaster version\0".as_ptr() as *const ::core::ffi::c_char,
                );
                tcpclose(fd);
                fd = -1 as ::core::ffi::c_int;
                return;
            }
            attrsize = (if masterversion
                >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        93 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        93 as ::core::ffi::c_int
                    })) as uint32_t
                && masterversion
                    != (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })) as uint32_t
                && masterversion
                    != (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            1 as ::core::ffi::c_int
                        })) as uint32_t
            {
                ATTR_RECORD_SIZE
            } else {
                35 as ::core::ffi::c_int
            }) as uint8_t;
            if i >= 13 as uint32_t {
                masterprocessid = get64bit(&raw mut rptr);
            } else {
                masterprocessid = metaid;
            }
        }
        if *rptr.offset(0 as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            sessionlost = if *rptr.offset(0 as isize) as ::core::ffi::c_int == MFS_ERROR_EPERM {
                2 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            };
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"master: register status: %s\0".as_ptr() as *const ::core::ffi::c_char,
                mfs_strerror(*rptr.offset(0 as isize)),
            );
            tcpclose(fd);
            fd = -1 as ::core::ffi::c_int;
            return;
        }
        lastwrite = monotonic_seconds();
        lastsyncsend = 0 as uint64_t;
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_INFO,
            b"registered to master\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_close_session() {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut regbuff: [uint8_t; 85] = [0; 85];
        let mut rleng: int32_t = 0;
        if sessionid == 0 as uint32_t {
            return;
        }
        wptr = &raw mut regbuff as *mut uint8_t;
        put32bit(&raw mut wptr, CLTOMA_FUSE_REGISTER as uint32_t);
        if masterversion
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    11 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    11 as ::core::ffi::c_int
                })) as uint32_t
            && metaid != 0 as uint64_t
        {
            put32bit(&raw mut wptr, 77 as uint32_t);
            rleng = (8 as ::core::ffi::c_int + 77 as ::core::ffi::c_int) as int32_t;
        } else {
            put32bit(&raw mut wptr, 69 as uint32_t);
            rleng = (8 as ::core::ffi::c_int + 69 as ::core::ffi::c_int) as int32_t;
        }
        memcpy(
            wptr as *mut ::core::ffi::c_void,
            FUSE_REGISTER_BLOB_ACL.as_ptr() as *const ::core::ffi::c_void,
            64 as size_t,
        );
        wptr = wptr.offset(64 as ::core::ffi::c_int as isize);
        put8bit(&raw mut wptr, REGISTER_CLOSESESSION as uint8_t);
        put32bit(&raw mut wptr, sessionid);
        put64bit(&raw mut wptr, metaid);
        if tcptowrite(
            fd,
            &raw mut regbuff as *mut uint8_t as *const ::core::ffi::c_void,
            rleng as uint32_t,
            1000 as uint32_t,
            1000 as uint32_t,
        ) != rleng
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"master: close session error (write: %s)\0".as_ptr() as *const ::core::ffi::c_char,
                strerr(*__errno_location()),
            );
        }
        if masterversion
            >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    29 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    29 as ::core::ffi::c_int
                })) as uint32_t
        {
            if tcptoread(
                fd,
                &raw mut regbuff as *mut uint8_t as *mut ::core::ffi::c_void,
                9 as uint32_t,
                500 as uint32_t,
                500 as uint32_t,
            ) != 9 as int32_t
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"master: close session error (read: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    strerr(*__errno_location()),
                );
            } else if regbuff[8 as usize] as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"master: closes session error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    mfs_strerror(regbuff[8 as usize]),
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_send_amtime_inodes() {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut inodespacket: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut inodesleng: int32_t = 0;
        let mut amfptr: *mut amtime_file = ::core::ptr::null_mut::<amtime_file>();
        let mut amfpptr: *mut *mut amtime_file = ::core::ptr::null_mut::<*mut amtime_file>();
        let mut amhash: uint32_t = 0;
        amtime_lock();
        if masterversion
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    74 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    74 as ::core::ffi::c_int
                })) as uint32_t
        {
            inodesleng = 0 as ::core::ffi::c_int as int32_t;
            amhash = 0 as uint32_t;
            while amhash < AMTIME_HASH_SIZE as uint32_t {
                amfptr = amtime_hash[amhash as usize];
                while !amfptr.is_null() {
                    if (*amfptr).atime > 0 as uint64_t || (*amfptr).mtime > 0 as uint64_t {
                        inodesleng = (inodesleng as ::core::ffi::c_int + 12 as ::core::ffi::c_int)
                            as int32_t;
                    }
                    amfptr = (*amfptr).next as *mut amtime_file;
                }
                amhash = amhash.wrapping_add(1);
            }
            if inodesleng > 0 as int32_t {
                inodesleng =
                    (inodesleng as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as int32_t;
                // C: inodespacket = malloc(inodesleng). Box<[u8]> leaked as
                // raw ptr; every entry written before the send below.
                inodespacket =
                    Box::into_raw(Box::<[u8]>::new_uninit_slice(inodesleng as usize).assume_init())
                        as *mut uint8_t;
                ptr = inodespacket;
                put32bit(&raw mut ptr, CLTOMA_FUSE_AMTIME_INODES as uint32_t);
                put32bit(&raw mut ptr, (inodesleng - 8 as int32_t) as uint32_t);
                amhash = 0 as uint32_t;
                while amhash < AMTIME_HASH_SIZE as uint32_t {
                    amfpptr =
                        (&raw mut amtime_hash as *mut *mut amtime_file).offset(amhash as isize);
                    loop {
                        amfptr = *amfpptr;
                        if amfptr.is_null() {
                            break;
                        }
                        if (*amfptr).atime > 0 as uint64_t || (*amfptr).mtime > 0 as uint64_t {
                            put32bit(&raw mut ptr, (*amfptr).inode);
                            put32bit(
                                &raw mut ptr,
                                (*amfptr).atime.wrapping_div(1000000 as uint64_t) as uint32_t,
                            );
                            put32bit(
                                &raw mut ptr,
                                (*amfptr).mtime.wrapping_div(1000000 as uint64_t) as uint32_t,
                            );
                        }
                        if (*amfptr).atimeage as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
                            (*amfptr).atime = 0 as uint64_t;
                        }
                        if (*amfptr).mtimeage as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
                            (*amfptr).mtime = 0 as uint64_t;
                        }
                        if ((*amfptr).atimeage as ::core::ffi::c_int) < AMTIME_MAX_AGE
                            || ((*amfptr).mtimeage as ::core::ffi::c_int) < AMTIME_MAX_AGE
                        {
                            if ((*amfptr).atimeage as ::core::ffi::c_int) < AMTIME_MAX_AGE {
                                (*amfptr).atimeage = (*amfptr).atimeage.wrapping_add(1);
                            }
                            if ((*amfptr).mtimeage as ::core::ffi::c_int) < AMTIME_MAX_AGE {
                                (*amfptr).mtimeage = (*amfptr).mtimeage.wrapping_add(1);
                            }
                            amfpptr = &raw mut (*amfptr).next as *mut *mut amtime_file;
                        } else {
                            *amfpptr = (*amfptr).next as *mut amtime_file;
                            // C: free(amfptr) — pairs with Box::into_raw in
                            // fs_atime/fs_mtime.
                            drop(Box::from_raw(amfptr));
                        }
                    }
                    amhash = amhash.wrapping_add(1);
                }
                amtime_unlock();
                if tcptowrite(
                    fd,
                    inodespacket as *const ::core::ffi::c_void,
                    inodesleng as uint32_t,
                    1000 as uint32_t,
                    (send_timeout * 1000 as ::core::ffi::c_int) as uint32_t,
                ) != inodesleng
                {
                    ::core::intrinsics::atomic_or::<
                        _,
                        _,
                        { ::core::intrinsics::AtomicOrdering::SeqCst },
                    >(&raw mut disconnect, 1 as ::core::ffi::c_int);
                } else {
                    master_stats_add(
                        MASTER_BYTESSENT as ::core::ffi::c_int as uint8_t,
                        inodesleng as uint64_t,
                    );
                    master_stats_inc(MASTER_PACKETSSENT as ::core::ffi::c_int as uint8_t);
                }
                drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                    inodespacket,
                    inodesleng as usize,
                )));
                return;
            }
        }
        amhash = 0 as uint32_t;
        while amhash < AMTIME_HASH_SIZE as uint32_t {
            amfpptr = (&raw mut amtime_hash as *mut *mut amtime_file).offset(amhash as isize);
            loop {
                amfptr = *amfpptr;
                if amfptr.is_null() {
                    break;
                }
                if ((*amfptr).atimeage as ::core::ffi::c_int) < AMTIME_MAX_AGE
                    || ((*amfptr).mtimeage as ::core::ffi::c_int) < AMTIME_MAX_AGE
                {
                    if ((*amfptr).atimeage as ::core::ffi::c_int) < AMTIME_MAX_AGE {
                        (*amfptr).atimeage = (*amfptr).atimeage.wrapping_add(1);
                    }
                    if ((*amfptr).mtimeage as ::core::ffi::c_int) < AMTIME_MAX_AGE {
                        (*amfptr).mtimeage = (*amfptr).mtimeage.wrapping_add(1);
                    }
                    amfpptr = &raw mut (*amfptr).next as *mut *mut amtime_file;
                } else {
                    *amfpptr = (*amfptr).next as *mut amtime_file;
                    // C: free(amfptr) — pairs with Box::into_raw in
                    // fs_atime/fs_mtime.
                    drop(Box::from_raw(amfptr));
                }
            }
            amhash = amhash.wrapping_add(1);
        }
        amtime_unlock();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_send_open_inodes() {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut inodespacket: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut inodes: uint32_t = 0;
        let mut hash: uint32_t = 0;
        let mut afptr: *mut acquired_file = ::core::ptr::null_mut::<acquired_file>();
        let mut afpptr: *mut *mut acquired_file = ::core::ptr::null_mut::<*mut acquired_file>();
        af_lock();
        heap_cleanup();
        hash = 0 as uint32_t;
        while hash < ACQFILES_HASH_SIZE as uint32_t {
            afpptr = (&raw mut af_hash as *mut *mut acquired_file).offset(hash as isize);
            loop {
                afptr = *afpptr;
                if afptr.is_null() {
                    break;
                }
                if (*afptr).cnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && (*afptr).dentry as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    (*afptr).age = (*afptr).age.wrapping_add(1);
                    if (*afptr).age as ::core::ffi::c_int > ACQFILES_MAX_AGE {
                        *afpptr = (*afptr).next as *mut acquired_file;
                        crate::chunksdatacache::clear_inode((*afptr).inode, 0 as uint32_t);
                        fs_af_remove_from_lru(afptr);
                        // C: free(afptr) — pairs with Box::into_raw in
                        // fs_add_entry/fs_inc_acnt.
                        drop(Box::from_raw(afptr));
                        continue;
                    }
                }
                afpptr = &raw mut (*afptr).next as *mut *mut acquired_file;
                heap_push((*afptr).inode);
            }
            hash = hash.wrapping_add(1);
        }
        inodes = heap_elements();
        // C: inodespacket = malloc(inodes*4+8). Box<[u8]> leaked as raw ptr;
        // header + every entry written before the send below.
        inodespacket = Box::into_raw(
            Box::<[u8]>::new_uninit_slice(
                inodes
                    .wrapping_mul(4 as uint32_t)
                    .wrapping_add(8 as uint32_t) as usize,
            )
            .assume_init(),
        ) as *mut uint8_t;
        ptr = inodespacket;
        if masterversion
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    74 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    74 as ::core::ffi::c_int
                })) as uint32_t
        {
            put32bit(&raw mut ptr, CLTOMA_FUSE_SUSTAINED_INODES as uint32_t);
        } else {
            put32bit(
                &raw mut ptr,
                CLTOMA_FUSE_SUSTAINED_INODES_DEPRECATED as uint32_t,
            );
        }
        put32bit(&raw mut ptr, inodes.wrapping_mul(4 as uint32_t));
        i = 0 as uint32_t;
        while i < inodes {
            put32bit(&raw mut ptr, heap_pop());
            i = i.wrapping_add(1);
        }
        af_unlock();
        i = inodes
            .wrapping_mul(4 as uint32_t)
            .wrapping_add(8 as uint32_t);
        if tcptowrite(
            fd,
            inodespacket as *const ::core::ffi::c_void,
            i,
            1000 as uint32_t,
            (send_timeout * 1000 as ::core::ffi::c_int) as uint32_t,
        ) != i as int32_t
        {
            ::core::intrinsics::atomic_or::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut disconnect,
                1 as ::core::ffi::c_int,
            );
        } else {
            master_stats_add(
                MASTER_BYTESSENT as ::core::ffi::c_int as uint8_t,
                i as uint64_t,
            );
            master_stats_inc(MASTER_PACKETSSENT as ::core::ffi::c_int as uint8_t);
        }
        // C: free(inodespacket) — i still holds inodes*4+8, the alloc len.
        drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
            inodespacket,
            i as usize,
        )));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_send_opdata() {
    unsafe {
        let mut packetdata: [uint8_t; 52] = [0; 52];
        let mut senddata: int32_t = 0;
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rbyt_copy: uint64_t = 0;
        let mut wbyt_copy: uint64_t = 0;
        let mut sentbyt_copy: uint64_t = 0;
        let mut rcvdbyt_copy: uint64_t = 0;
        let mut rcnt_copy: uint32_t = 0;
        let mut wcnt_copy: uint32_t = 0;
        let mut fcnt_copy: uint32_t = 0;
        senddata = 0 as ::core::ffi::c_int as int32_t;
        rbyt_copy = ::core::intrinsics::atomic_and::<
            _,
            _,
            { ::core::intrinsics::AtomicOrdering::SeqCst },
        >(&raw mut rbyt, 0 as uint64_t);
        rcnt_copy = ::core::intrinsics::atomic_and::<
            _,
            _,
            { ::core::intrinsics::AtomicOrdering::SeqCst },
        >(&raw mut rcnt, 0 as uint32_t);
        wbyt_copy = ::core::intrinsics::atomic_and::<
            _,
            _,
            { ::core::intrinsics::AtomicOrdering::SeqCst },
        >(&raw mut wbyt, 0 as uint64_t);
        wcnt_copy = ::core::intrinsics::atomic_and::<
            _,
            _,
            { ::core::intrinsics::AtomicOrdering::SeqCst },
        >(&raw mut wcnt, 0 as uint32_t);
        fcnt_copy = ::core::intrinsics::atomic_and::<
            _,
            _,
            { ::core::intrinsics::AtomicOrdering::SeqCst },
        >(&raw mut fcnt, 0 as uint32_t);
        sentbyt_copy = write_get_total_bytes();
        rcvdbyt_copy = read_get_total_bytes();
        if masterversion
            >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 57 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
            && rbyt_copy
                | wbyt_copy
                | sentbyt_copy
                | rcvdbyt_copy
                | rcnt_copy as uint64_t
                | wcnt_copy as uint64_t
                | fcnt_copy as uint64_t
                != 0 as uint64_t
        {
            wptr = &raw mut packetdata as *mut uint8_t;
            put32bit(&raw mut wptr, CLTOMA_FUSE_OPDATA as uint32_t);
            put32bit(&raw mut wptr, 44 as uint32_t);
            put64bit(&raw mut wptr, rbyt_copy);
            put64bit(&raw mut wptr, wbyt_copy);
            put32bit(&raw mut wptr, rcnt_copy);
            put32bit(&raw mut wptr, wcnt_copy);
            put32bit(&raw mut wptr, fcnt_copy);
            put64bit(&raw mut wptr, rcvdbyt_copy);
            put64bit(&raw mut wptr, sentbyt_copy);
            senddata = (44 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as int32_t;
        } else if masterversion
            >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 27 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
            && rbyt_copy
                | wbyt_copy
                | rcnt_copy as uint64_t
                | wcnt_copy as uint64_t
                | fcnt_copy as uint64_t
                != 0 as uint64_t
        {
            wptr = &raw mut packetdata as *mut uint8_t;
            put32bit(&raw mut wptr, CLTOMA_FUSE_OPDATA as uint32_t);
            put32bit(&raw mut wptr, 28 as uint32_t);
            put64bit(&raw mut wptr, rbyt_copy);
            put64bit(&raw mut wptr, wbyt_copy);
            put32bit(&raw mut wptr, rcnt_copy);
            put32bit(&raw mut wptr, wcnt_copy);
            put32bit(&raw mut wptr, fcnt_copy);
            senddata = (28 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as int32_t;
        }
        if senddata != 0 {
            if tcptowrite(
                fd,
                &raw mut packetdata as *mut uint8_t as *const ::core::ffi::c_void,
                senddata as uint32_t,
                1000 as uint32_t,
                (send_timeout * 1000 as ::core::ffi::c_int) as uint32_t,
            ) != senddata
            {
                ::core::intrinsics::atomic_or::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut disconnect,
                    1 as ::core::ffi::c_int,
                );
            } else {
                master_stats_add(
                    MASTER_BYTESSENT as ::core::ffi::c_int as uint8_t,
                    senddata as uint64_t,
                );
                master_stats_inc(MASTER_PACKETSSENT as ::core::ffi::c_int as uint8_t);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_set_working_flags(mut sflags: uint8_t) {
    unsafe {
        ::core::intrinsics::atomic_or::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut working_flags,
            sflags,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_clr_working_flags(mut cflags: uint8_t) {
    unsafe {
        ::core::intrinsics::atomic_and::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut working_flags,
            !(cflags as ::core::ffi::c_int) as uint8_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_send_working_flags() {
    unsafe {
        let mut packetdata: [uint8_t; 9] = [0; 9];
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if masterversion
            >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 40 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = &raw mut packetdata as *mut uint8_t;
            put32bit(&raw mut wptr, CLTOMA_FUSE_WFLAGS as uint32_t);
            put32bit(&raw mut wptr, 1 as uint32_t);
            put8bit(&raw mut wptr, working_flags);
            if tcptowrite(
                fd,
                &raw mut packetdata as *mut uint8_t as *const ::core::ffi::c_void,
                (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t,
                1000 as uint32_t,
                (send_timeout * 1000 as ::core::ffi::c_int) as uint32_t,
            ) != 8 as int32_t + 1 as int32_t
            {
                ::core::intrinsics::atomic_or::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                    &raw mut disconnect,
                    1 as ::core::ffi::c_int,
                );
            } else {
                master_stats_add(
                    MASTER_BYTESSENT as ::core::ffi::c_int as uint8_t,
                    (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint64_t,
                );
                master_stats_inc(MASTER_PACKETSSENT as ::core::ffi::c_int as uint8_t);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_nop_thread(_arg: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut hdr: [uint8_t; 12] = [0; 12];
        let mut usec: uint64_t = 0;
        let mut now: ::core::ffi::c_int = 0;
        let mut inodeswritecnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            fd_lock();
            if fterm as ::core::ffi::c_int == 2 as ::core::ffi::c_int
                && donotsendsustainedinodes == 0 as ::core::ffi::c_int
            {
                if fd >= 0 as ::core::ffi::c_int {
                    fs_send_opdata();
                    fs_send_amtime_inodes();
                    fs_send_open_inodes();
                    fs_close_session();
                    tcpclose(fd);
                    fd = -1 as ::core::ffi::c_int;
                }
                fd_unlock();
                return NULL;
            }
            if ::core::intrinsics::atomic_or::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut disconnect,
                0 as ::core::ffi::c_int,
            ) == 0 as ::core::ffi::c_int
                && fd >= 0 as ::core::ffi::c_int
            {
                now = monotonic_seconds() as ::core::ffi::c_int;
                if lastwrite + 2.0f64 < now as ::core::ffi::c_double {
                    ptr = &raw mut hdr as *mut uint8_t;
                    put32bit(&raw mut ptr, ANTOAN_NOP as uint32_t);
                    put32bit(&raw mut ptr, 4 as uint32_t);
                    put32bit(&raw mut ptr, 0 as uint32_t);
                    if tcptowrite(
                        fd,
                        &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
                        12 as uint32_t,
                        1000 as uint32_t,
                        (send_timeout * 1000 as ::core::ffi::c_int) as uint32_t,
                    ) != 12 as int32_t
                    {
                        ::core::intrinsics::atomic_or::<
                            _,
                            _,
                            { ::core::intrinsics::AtomicOrdering::SeqCst },
                        >(&raw mut disconnect, 1 as ::core::ffi::c_int);
                    } else {
                        master_stats_add(
                            MASTER_BYTESSENT as ::core::ffi::c_int as uint8_t,
                            12 as uint64_t,
                        );
                        master_stats_inc(MASTER_PACKETSSENT as ::core::ffi::c_int as uint8_t);
                    }
                    lastwrite = now as ::core::ffi::c_double;
                }
                usec = monotonic_useconds();
                if masterversion
                    >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            74 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            74 as ::core::ffi::c_int
                        })) as uint32_t
                    && (lastsyncsend == 0 as uint64_t
                        || lastsyncsend.wrapping_add(60000000 as uint64_t) < usec)
                {
                    ptr = &raw mut hdr as *mut uint8_t;
                    put32bit(&raw mut ptr, CLTOMA_FUSE_TIME_SYNC as uint32_t);
                    put32bit(&raw mut ptr, 4 as uint32_t);
                    put32bit(&raw mut ptr, 0 as uint32_t);
                    if tcptowrite(
                        fd,
                        &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
                        12 as uint32_t,
                        1000 as uint32_t,
                        (send_timeout * 1000 as ::core::ffi::c_int) as uint32_t,
                    ) != 12 as int32_t
                    {
                        ::core::intrinsics::atomic_or::<
                            _,
                            _,
                            { ::core::intrinsics::AtomicOrdering::SeqCst },
                        >(&raw mut disconnect, 1 as ::core::ffi::c_int);
                    } else {
                        master_stats_add(
                            MASTER_BYTESSENT as ::core::ffi::c_int as uint8_t,
                            12 as uint64_t,
                        );
                        master_stats_inc(MASTER_PACKETSSENT as ::core::ffi::c_int as uint8_t);
                    }
                    lastsyncsend = usec;
                }
                if inodeswritecnt <= 0 as ::core::ffi::c_int
                    || inodeswritecnt > 60 as ::core::ffi::c_int
                {
                    inodeswritecnt = 60 as ::core::ffi::c_int;
                } else {
                    inodeswritecnt -= 1;
                }
                if inodeswritecnt == 0 as ::core::ffi::c_int {
                    if donotsendsustainedinodes != 0 {
                        inodeswritecnt = 1 as ::core::ffi::c_int;
                    } else {
                        fs_send_open_inodes();
                    }
                }
                fs_send_opdata();
                fs_send_amtime_inodes();
                fs_send_working_flags();
            }
            fd_unlock();
            portable_sleep(1 as uint64_t);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_receive_thread(
    _arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut hdr: [uint8_t; 12] = [0; 12];
        let mut msgbuff: [uint8_t; 37] = [0; 37];
        let mut internal: uint8_t = 0;
        let mut rec: *mut threc = ::core::ptr::null_mut::<threc>();
        let mut cmd: uint32_t = 0;
        let mut size: uint32_t = 0;
        let mut packetid: uint32_t = 0;
        let mut rcvd: uint32_t = 0;
        let mut toread: uint32_t = 0;
        let mut rechash: uint32_t = 0;
        let mut r: int32_t = 0;
        loop {
            fd_lock();
            if fterm != 0 {
                fterm = 2 as uint8_t;
                fd_unlock();
                return NULL;
            }
            if ::core::intrinsics::atomic_and::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut disconnect,
                0 as ::core::ffi::c_int,
            ) != 0
            {
                crate::chunksdatacache::cleanup();
                tcpclose(fd);
                fd = -1 as ::core::ffi::c_int;
                rec_lock();
                rechash = 0 as uint32_t;
                while rechash < THRECHASHSIZE as uint32_t {
                    rec = threchash[rechash as usize];
                    while !rec.is_null() {
                        threc_lock(rec);
                        if (*rec).sent != 0 {
                            (*rec).status = 1 as uint8_t;
                            (*rec).rcvd = 1 as uint8_t;
                            (*rec).cond.notify_one();
                        }
                        threc_unlock(rec);
                        rec = (*rec).next as *mut threc;
                    }
                    rechash = rechash.wrapping_add(1);
                }
                rec_unlock();
            }
            if fd == -1 as ::core::ffi::c_int && sessionid != 0 as uint32_t {
                fs_reconnect(connect_args.minversion);
            }
            if fd == -1 as ::core::ffi::c_int {
                if sessionlost != 0 || sessionid == 0 as uint32_t {
                    if fs_connect(0 as uint8_t, &raw mut connect_args) == 0 as ::core::ffi::c_int {
                        sessionlost = 0 as ::core::ffi::c_int;
                    }
                } else if fs_resolve(
                    0 as uint8_t,
                    connect_args.bindhostname,
                    connect_args.masterhostname,
                    connect_args.masterportname,
                ) == 0 as ::core::ffi::c_int
                {
                    fs_reconnect(connect_args.minversion);
                }
            }
            if fd == -1 as ::core::ffi::c_int {
                fd_unlock();
                portable_sleep(2 as uint64_t);
            } else {
                fd_unlock();
                r = tcptoread(
                    fd,
                    &raw mut hdr as *mut uint8_t as *mut ::core::ffi::c_void,
                    12 as uint32_t,
                    (sock_timeout * 1000 as ::core::ffi::c_int) as uint32_t,
                    (recv_timeout * 1000 as ::core::ffi::c_int) as uint32_t,
                );
                if r == 0 as int32_t {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"master: connection lost (header)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    fs_disconnect();
                } else if r != 12 as int32_t {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"master: tcp recv error: %s (header)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        strerr(*__errno_location()),
                    );
                    fs_disconnect();
                } else {
                    master_stats_add(
                        MASTER_BYTESRCVD as ::core::ffi::c_int as uint8_t,
                        12 as uint64_t,
                    );
                    master_stats_inc(MASTER_PACKETSRCVD as ::core::ffi::c_int as uint8_t);
                    ptr = &raw mut hdr as *mut uint8_t;
                    cmd = get32bit(&raw mut ptr);
                    size = get32bit(&raw mut ptr);
                    packetid = get32bit(&raw mut ptr);
                    if size < 4 as uint32_t {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"master: packet too small\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        fs_disconnect();
                    } else {
                        size = size.wrapping_sub(4 as uint32_t);
                        if packetid == 0 as uint32_t {
                            if cmd == ANTOAN_NOP as uint32_t && size == 0 as uint32_t {
                                continue;
                            } else {
                                if cmd == ANTOAN_UNKNOWN_COMMAND as uint32_t
                                    || cmd == ANTOAN_BAD_COMMAND_SIZE as uint32_t
                                {
                                    continue;
                                }
                                internal = 0 as uint8_t;
                                if cmd == MATOCL_FUSE_CHUNK_HAS_CHANGED as uint32_t {
                                    if size == 29 as uint32_t || size == 37 as uint32_t {
                                        internal = 1 as uint8_t;
                                    } else {
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_WARNING,
                                            b"master: unexpected msg size (msg:MATOCL_FUSE_CHUNK_HAS_CHANGED ; size:%u/33|41)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            size.wrapping_add(4 as uint32_t),
                                        );
                                        fs_disconnect();
                                        continue;
                                    }
                                }
                                if cmd == MATOCL_FUSE_FLENG_HAS_CHANGED as uint32_t {
                                    if size == 12 as uint32_t {
                                        internal = 1 as uint8_t;
                                    } else {
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_WARNING,
                                            b"master: unexpected msg size (msg:MATOCL_FUSE_FLENG_HAS_CHANGED ; size:%u/16)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            size.wrapping_add(4 as uint32_t),
                                        );
                                        fs_disconnect();
                                        continue;
                                    }
                                }
                                if cmd == MATOCL_FUSE_TIME_SYNC as uint32_t {
                                    if size == 8 as uint32_t {
                                        internal = 1 as uint8_t;
                                    } else {
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_WARNING,
                                            b"master: unexpected msg size (msg:MATOCL_FUSE_TIME_SYNC ; size:%u/12)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            size.wrapping_add(4 as uint32_t),
                                        );
                                        fs_disconnect();
                                        continue;
                                    }
                                }
                                if cmd == MATOCL_FUSE_INVALIDATE_CHUNK_CACHE as uint32_t {
                                    if size == 0 as uint32_t {
                                        internal = 1 as uint8_t;
                                    } else {
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_WARNING,
                                            b"master: unexpected msg size (msg:MATOCL_FUSE_INVALIDATE_CHUNK_CACHE ; size:%u/4)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            size.wrapping_add(4 as uint32_t),
                                        );
                                        fs_disconnect();
                                        continue;
                                    }
                                }
                                if cmd == ANTOAN_FORCE_TIMEOUT as uint32_t {
                                    if size == 2 as uint32_t {
                                        internal = 1 as uint8_t;
                                    } else {
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_WARNING,
                                            b"master: unexpected msg size (msg:ANTOAN_FORCE_TIMEOUT ; size:%u/6)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            size.wrapping_add(4 as uint32_t),
                                        );
                                        fs_disconnect();
                                        continue;
                                    }
                                }
                                if internal != 0 {
                                    if size > 0 as uint32_t {
                                        r = tcptoread(
                                            fd,
                                            &raw mut msgbuff as *mut uint8_t
                                                as *mut ::core::ffi::c_void,
                                            size,
                                            (sock_timeout * 1000 as ::core::ffi::c_int) as uint32_t,
                                            (recv_timeout * 1000 as ::core::ffi::c_int) as uint32_t,
                                        );
                                        if r == 0 as int32_t {
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_WARNING,
                                                b"master: connection lost (data/internal ; cmd:%u ; size:%u)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                cmd,
                                                size,
                                            );
                                            fs_disconnect();
                                            continue;
                                        } else if r != size as int32_t {
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_WARNING,
                                                b"master: tcp recv error: %s (data/internal ; cmd:%u ; size:%u)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                strerr(*__errno_location()),
                                                cmd,
                                                size,
                                            );
                                            fs_disconnect();
                                            continue;
                                        } else {
                                            master_stats_add(
                                                MASTER_BYTESRCVD as ::core::ffi::c_int as uint8_t,
                                                size as uint64_t,
                                            );
                                        }
                                    }
                                    ptr = &raw mut msgbuff as *mut uint8_t;
                                    if cmd == MATOCL_FUSE_FLENG_HAS_CHANGED as uint32_t {
                                        let mut inode: uint32_t = 0;
                                        let mut fleng: uint64_t = 0;
                                        inode = get32bit(&raw mut ptr);
                                        fleng = get64bit(&raw mut ptr);
                                        crate::extrapackets::file_length_changed(inode, fleng);
                                        continue;
                                    } else if cmd == MATOCL_FUSE_CHUNK_HAS_CHANGED as uint32_t {
                                        let mut inode_0: uint32_t = 0;
                                        let mut chindx: uint32_t = 0;
                                        let mut chunkid: uint64_t = 0;
                                        let mut version: uint32_t = 0;
                                        let mut fleng_0: uint64_t = 0;
                                        let mut truncflag: uint8_t = 0;
                                        let mut choffset: uint32_t = 0;
                                        let mut chsize: uint32_t = 0;
                                        inode_0 = get32bit(&raw mut ptr);
                                        chindx = get32bit(&raw mut ptr);
                                        chunkid = get64bit(&raw mut ptr);
                                        version = get32bit(&raw mut ptr);
                                        fleng_0 = get64bit(&raw mut ptr);
                                        truncflag = get8bit(&raw mut ptr);
                                        if size == 37 as uint32_t {
                                            choffset = get32bit(&raw mut ptr);
                                            chsize = get32bit(&raw mut ptr);
                                        } else {
                                            choffset = 0 as uint32_t;
                                            chsize = MFSCHUNKSIZE as uint32_t;
                                        }
                                        crate::extrapackets::chunk_changed(
                                            inode_0,
                                            chindx,
                                            chunkid,
                                            version,
                                            fleng_0,
                                            truncflag != 0,
                                            choffset,
                                            chsize,
                                        );
                                        continue;
                                    } else if cmd == MATOCL_FUSE_TIME_SYNC as uint32_t {
                                        let mut lusectime: uint64_t = 0;
                                        let mut rusectime: uint64_t = 0;
                                        let mut usec: uint64_t = 0;
                                        let mut usecping: uint64_t = 0;
                                        let mut tv: timeval = timeval {
                                            tv_sec: 0,
                                            tv_usec: 0,
                                        };
                                        usec = monotonic_useconds();
                                        fd_lock();
                                        if usec >= lastsyncsend {
                                            usecping = usec.wrapping_sub(lastsyncsend);
                                            fd_unlock();
                                            master_stats_set(
                                                MASTER_PING as ::core::ffi::c_int as uint8_t,
                                                usecping,
                                            );
                                        } else {
                                            fd_unlock();
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_WARNING,
                                                b"negative packet travel time between client and master - ignoring in time sync\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                            );
                                            usecping = 0 as uint64_t;
                                        }
                                        if usecping > 100000 as uint64_t {
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_NOTICE,
                                                b"high packet travel time between client and master (%u.%06us) - ignoring in time sync\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                usecping.wrapping_div(1000000 as uint64_t)
                                                    as ::core::ffi::c_uint,
                                                usecping.wrapping_rem(1000000 as uint64_t)
                                                    as ::core::ffi::c_uint,
                                            );
                                            usecping = 0 as uint64_t;
                                        }
                                        rusectime = get64bit(&raw mut ptr);
                                        rusectime = rusectime
                                            .wrapping_add(usecping.wrapping_div(2 as uint64_t));
                                        fs_amtime_reference_clock(usec, rusectime);
                                        gettimeofday(&raw mut tv, NULL);
                                        lusectime = tv.tv_sec as uint64_t;
                                        lusectime = lusectime.wrapping_mul(1000000 as uint64_t);
                                        lusectime = lusectime.wrapping_add(tv.tv_usec as uint64_t);
                                        if rusectime.wrapping_add(1000000 as uint64_t) < lusectime
                                            || lusectime.wrapping_add(1000000 as uint64_t)
                                                < rusectime
                                        {
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_WARNING,
                                                b"time desync between client and master is higher than a second - it might lead to strange atime/mtime behaviour - consider time synchronization in your moosefs cluster\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                            );
                                        }
                                        if rusectime > lusectime {
                                            master_stats_set(
                                                MASTER_TIMEDIFF as ::core::ffi::c_int as uint8_t,
                                                rusectime.wrapping_sub(lusectime),
                                            );
                                        } else {
                                            master_stats_set(
                                                MASTER_TIMEDIFF as ::core::ffi::c_int as uint8_t,
                                                lusectime.wrapping_sub(rusectime),
                                            );
                                        }
                                        continue;
                                    } else if cmd == MATOCL_FUSE_INVALIDATE_CHUNK_CACHE as uint32_t
                                    {
                                        crate::chunksdatacache::cleanup();
                                        continue;
                                    } else if cmd == ANTOAN_FORCE_TIMEOUT as uint32_t {
                                        sock_timeout = get16bit(&raw mut ptr) as ::core::ffi::c_int;
                                        if sock_timeout < 10 as ::core::ffi::c_int {
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_WARNING,
                                                b"forced timeout too small (<10s) - set to 10s\0"
                                                    .as_ptr()
                                                    as *const ::core::ffi::c_char,
                                            );
                                            sock_timeout = 10 as ::core::ffi::c_int;
                                        }
                                        if sock_timeout < 100 as ::core::ffi::c_int {
                                            recv_timeout = 300 as ::core::ffi::c_int;
                                        } else {
                                            recv_timeout = sock_timeout * 3 as ::core::ffi::c_int;
                                        }
                                        continue;
                                    }
                                }
                            }
                        }
                        rec = fs_get_threc_by_id(packetid);
                        if rec.is_null() {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"master: got unexpected queryid (%u ; cmd:%u ; size:%u)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                packetid,
                                cmd,
                                size.wrapping_add(4 as uint32_t),
                            );
                            fs_disconnect();
                        } else {
                            threc_lock(rec);
                            if (*rec).receiving != 0 {
                                threc_unlock(rec);
                                fs_disconnect();
                            } else {
                                fs_input_buffer_init(rec, size);
                                if (*rec).ibuff.is_null() {
                                    threc_unlock(rec);
                                    fs_disconnect();
                                } else {
                                    (*rec).receiving = 1 as uint8_t;
                                    threc_unlock(rec);
                                    rcvd = 0 as uint32_t;
                                    while size.wrapping_sub(rcvd) > 0 as uint32_t {
                                        toread = size.wrapping_sub(rcvd);
                                        if toread > 65536 as uint32_t {
                                            toread = 65536 as uint32_t;
                                        }
                                        r = tcptoread(
                                            fd,
                                            (*rec).ibuff.offset(rcvd as isize)
                                                as *mut ::core::ffi::c_void,
                                            toread,
                                            (sock_timeout * 1000 as ::core::ffi::c_int) as uint32_t,
                                            (recv_timeout * 1000 as ::core::ffi::c_int) as uint32_t,
                                        );
                                        if r == 0 as int32_t {
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_WARNING,
                                                b"master: connection lost (data ; cmd:%u ; size:%u)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                cmd,
                                                size,
                                            );
                                            break;
                                        } else if r != toread as int32_t {
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_WARNING,
                                                b"master: tcp recv error: %s (data ; cmd:%u ; size:%u)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                strerr(*__errno_location()),
                                                cmd,
                                                size,
                                            );
                                            break;
                                        } else {
                                            master_stats_add(
                                                MASTER_BYTESRCVD as ::core::ffi::c_int as uint8_t,
                                                toread as uint64_t,
                                            );
                                            rcvd = rcvd.wrapping_add(toread);
                                        }
                                    }
                                    if size.wrapping_sub(rcvd) > 0 as uint32_t {
                                        threc_lock(rec);
                                        (*rec).receiving = 0 as uint8_t;
                                        threc_unlock(rec);
                                        fs_disconnect();
                                    } else {
                                        threc_lock(rec);
                                        (*rec).sent = 0 as uint8_t;
                                        (*rec).status = 0 as uint8_t;
                                        (*rec).idataleng = size;
                                        (*rec).rcvd_cmd = cmd;
                                        (*rec).receiving = 0 as uint8_t;
                                        (*rec).rcvd = 1 as uint8_t;
                                        (*rec).cond.notify_one();
                                        threc_unlock(rec);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_init_master_connection(
    mut bindhostname: *const ::core::ffi::c_char,
    mut masterhostname: *const ::core::ffi::c_char,
    mut masterportname: *const ::core::ffi::c_char,
    mut meta: uint8_t,
    mut info: *const ::core::ffi::c_char,
    mut subfolder: *const ::core::ffi::c_char,
    mut passworddigest: *const uint8_t,
    mut donotrememberpassword: uint8_t,
    mut bgregister: uint8_t,
    mut minversion: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        master_statsptr_init();
        fd = -1 as ::core::ffi::c_int;
        sessionlost = if bgregister as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        lastsyncsend = 0 as uint64_t;
        sessionid = 0 as uint32_t;
        metaid = 0 as uint64_t;
        masterversion = 0 as uint32_t;
        masterprocessid = 0 as uint64_t;
        ::core::intrinsics::atomic_and::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut disconnect,
            0 as ::core::ffi::c_int,
        );
        donotsendsustainedinodes = 0 as ::core::ffi::c_int;
        // C: strdup(...) for each string arg. CString::into_raw is the Box
        // equivalent; freed with CString::from_raw in fs_term. unwrap is
        // unreachable: CStr sources contain no interior NUL by definition.
        if !bindhostname.is_null() {
            connect_args.bindhostname =
                std::ffi::CString::new(std::ffi::CStr::from_ptr(bindhostname).to_bytes())
                    .unwrap()
                    .into_raw();
        } else {
            connect_args.bindhostname = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        connect_args.masterhostname =
            std::ffi::CString::new(std::ffi::CStr::from_ptr(masterhostname).to_bytes())
                .unwrap()
                .into_raw();
        connect_args.masterportname =
            std::ffi::CString::new(std::ffi::CStr::from_ptr(masterportname).to_bytes())
                .unwrap()
                .into_raw();
        connect_args.meta = meta;
        connect_args.clearpassword = donotrememberpassword;
        connect_args.info = std::ffi::CString::new(std::ffi::CStr::from_ptr(info).to_bytes())
            .unwrap()
            .into_raw();
        connect_args.subfolder =
            std::ffi::CString::new(std::ffi::CStr::from_ptr(subfolder).to_bytes())
                .unwrap()
                .into_raw();
        if passworddigest.is_null() {
            connect_args.passworddigest = ::core::ptr::null_mut::<uint8_t>();
        } else {
            // C: malloc(16) + memcpy. Box<[u8; 16]>, freed with Box::from_raw
            // in fs_connect (clearpassword) and fs_term.
            connect_args.passworddigest =
                Box::into_raw(Box::new([0 as uint8_t; 16])) as *mut uint8_t;
            memcpy(
                connect_args.passworddigest as *mut ::core::ffi::c_void,
                passworddigest as *const ::core::ffi::c_void,
                16 as size_t,
            );
        }
        connect_args.minversion = minversion;
        if bgregister != 0 {
            return 1 as ::core::ffi::c_int;
        }
        return fs_connect(1 as uint8_t, &raw mut connect_args);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_init_threads(mut retries: uint32_t, mut timeout: uint32_t) {
    unsafe {
        let mut i: uint32_t = 0;
        let mut tv: timeval = timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        let mut usectime: uint64_t = 0;
        maxretries = retries;
        usectimeout = timeout as uint64_t;
        usectimeout = usectimeout.wrapping_mul(1000000 as uint64_t);
        fterm = 0 as uint8_t;
        crate::extrapackets::init();
        i = 0 as uint32_t;
        while i < AMTIME_HASH_SIZE as uint32_t {
            amtime_hash[i as usize] = ::core::ptr::null_mut::<amtime_file>();
            i = i.wrapping_add(1);
        }
        i = 0 as uint32_t;
        while i < ACQFILES_HASH_SIZE as uint32_t {
            af_hash[i as usize] = ::core::ptr::null_mut::<acquired_file>();
            i = i.wrapping_add(1);
        }
        af_lruhead = ::core::ptr::null_mut::<acquired_file>();
        af_lrutail = &raw mut af_lruhead;
        af_lru_cnt = 0 as uint32_t;
        gettimeofday(&raw mut tv, NULL);
        usectime = tv.tv_sec as uint64_t;
        usectime = usectime.wrapping_mul(1000000 as uint64_t);
        usectime = usectime.wrapping_add(tv.tv_usec as uint64_t);
        timediffusec = usectime.wrapping_sub(monotonic_useconds()) as int64_t;
        // C: pthread_key_create(&reckey, fs_free_threc) — replaced by the
        // MY_THREC thread_local (see its Drop impl). pthread_attr with
        // stacksize 0x100000 + pthread_create x2 -> std::thread::Builder
        // with the same 1 MiB stack. zassert aborts on pthread_create
        // failure; mirror that.
        let rpt = match std::thread::Builder::new()
            .stack_size(0x100000 as usize)
            .spawn(|| {
                fs_receive_thread(::core::ptr::null_mut::<::core::ffi::c_void>());
            }) {
            Ok(handle) => handle,
            Err(_) => {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"pthread_create(&rpthid,&thattr,fs_receive_thread,NULL) failed ".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                abort();
            }
        };
        let npt = match std::thread::Builder::new()
            .stack_size(0x100000 as usize)
            .spawn(|| {
                fs_nop_thread(::core::ptr::null_mut::<::core::ffi::c_void>());
            }) {
            Ok(handle) => handle,
            Err(_) => {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"pthread_create(&npthid,&thattr,fs_nop_thread,NULL) failed ".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                abort();
            }
        };
        *RPT_HANDLE.lock().unwrap_or_else(|e| e.into_inner()) = Some(rpt);
        *NPT_HANDLE.lock().unwrap_or_else(|e| e.into_inner()) = Some(npt);
        mainrec = fs_get_my_threc();
        fs_init_counters();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_term() {
    unsafe {
        let mut rec: *mut threc = ::core::ptr::null_mut::<threc>();
        let mut recn: *mut threc = ::core::ptr::null_mut::<threc>();
        let mut i: uint32_t = 0;
        let mut rechash: uint32_t = 0;
        let mut amf: *mut amtime_file = ::core::ptr::null_mut::<amtime_file>();
        let mut amfn: *mut amtime_file = ::core::ptr::null_mut::<amtime_file>();
        let mut af: *mut acquired_file = ::core::ptr::null_mut::<acquired_file>();
        let mut afn: *mut acquired_file = ::core::ptr::null_mut::<acquired_file>();
        fd_lock();
        fterm = 1 as uint8_t;
        fd_unlock();
        // C: pthread_join(npthid,NULL); pthread_join(rpthid,NULL) — same
        // order. C ignored the thread return value; JoinHandle::join Err
        // only means the thread panicked, which pthread_join could not
        // observe either.
        if let Some(handle) = NPT_HANDLE.lock().unwrap_or_else(|e| e.into_inner()).take() {
            let _ = handle.join();
        }
        if let Some(handle) = RPT_HANDLE.lock().unwrap_or_else(|e| e.into_inner()).take() {
            let _ = handle.join();
        }
        fs_free_threc(mainrec as *mut ::core::ffi::c_void);
        rec_lock();
        rechash = 0 as uint32_t;
        while rechash < THRECHASHSIZE as uint32_t {
            rec = threchash[rechash as usize];
            while !rec.is_null() {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"thread specific memory (id:%u) hasn't been freed\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*rec).packetid,
                );
                recn = (*rec).next as *mut threc;
                if !(*rec).obuff.is_null() {
                    drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                        (*rec).obuff,
                        (*rec).obuffsize as usize,
                    )));
                }
                if !(*rec).ibuff.is_null() {
                    drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                        (*rec).ibuff,
                        (*rec).ibuffsize as usize,
                    )));
                }
                drop(Box::from_raw(rec));
                rec = recn;
            }
            rechash = rechash.wrapping_add(1);
        }
        rec = threcfree;
        while !rec.is_null() {
            recn = (*rec).next as *mut threc;
            if !(*rec).obuff.is_null() {
                drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                    (*rec).obuff,
                    (*rec).obuffsize as usize,
                )));
            }
            if !(*rec).ibuff.is_null() {
                drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                    (*rec).ibuff,
                    (*rec).ibuffsize as usize,
                )));
            }
            drop(Box::from_raw(rec));
            rec = recn;
        }
        rec_unlock();
        // C: pthread_key_delete(reckey) — mainrec was freed explicitly
        // above; clearing the slot keeps the MY_THREC Drop from freeing it
        // again at thread/process exit.
        MY_THREC.with(|slot| slot.0.set(::core::ptr::null_mut::<threc>()));
        i = 0 as uint32_t;
        while i < ACQFILES_HASH_SIZE as uint32_t {
            af = af_hash[i as usize];
            while !af.is_null() {
                afn = (*af).next as *mut acquired_file;
                drop(Box::from_raw(af));
                af = afn;
            }
            i = i.wrapping_add(1);
        }
        i = 0 as uint32_t;
        while i < AMTIME_HASH_SIZE as uint32_t {
            amf = amtime_hash[i as usize];
            while !amf.is_null() {
                amfn = (*amf).next as *mut amtime_file;
                drop(Box::from_raw(amf));
                amf = amfn;
            }
            i = i.wrapping_add(1);
        }
        if fd >= 0 as ::core::ffi::c_int {
            tcpclose(fd);
        }
        // C: free() on each strdup'd string. free(NULL) is a no-op in C;
        // CString::from_raw(null) would be UB, so every site is null-guarded
        // (identical behavior).
        if !connect_args.bindhostname.is_null() {
            drop(std::ffi::CString::from_raw(connect_args.bindhostname));
        }
        if !connect_args.masterhostname.is_null() {
            drop(std::ffi::CString::from_raw(connect_args.masterhostname));
        }
        if !connect_args.masterportname.is_null() {
            drop(std::ffi::CString::from_raw(connect_args.masterportname));
        }
        if !connect_args.info.is_null() {
            drop(std::ffi::CString::from_raw(connect_args.info));
        }
        if !connect_args.subfolder.is_null() {
            drop(std::ffi::CString::from_raw(connect_args.subfolder));
        }
        if !connect_args.passworddigest.is_null() {
            drop(Box::from_raw(
                connect_args.passworddigest as *mut [uint8_t; 16],
            ));
        }
        heap_term();
        crate::extrapackets::term();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_get_cfg(
    mut opt_name: *const ::core::ffi::c_char,
    mut oleng: *mut uint8_t,
    mut odata: *mut *const uint8_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut nleng: uint32_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        nleng = strlen(opt_name) as uint32_t;
        if nleng > 255 as uint32_t {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        wptr = fs_createpacket(
            rec,
            ANTOAN_GET_CONFIG as uint32_t,
            (1 as uint32_t).wrapping_add(nleng),
        );
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put8bit(&raw mut wptr, nleng as uint8_t);
        memcpy(
            wptr as *mut ::core::ffi::c_void,
            opt_name as *const ::core::ffi::c_void,
            nleng as size_t,
        );
        rptr = fs_sendandreceive(rec, ANTOAN_CONFIG_VALUE as uint32_t, &raw mut i);
        if rptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        } else if i == 0 as uint32_t || i > 255 as uint32_t {
            fs_disconnect();
            return MFS_ERROR_IO as uint8_t;
        }
        nleng = get8bit(&raw mut rptr) as uint32_t;
        if i != (1 as uint32_t).wrapping_add(nleng) {
            fs_disconnect();
            return MFS_ERROR_IO as uint8_t;
        }
        *oleng = nleng as uint8_t;
        *odata = rptr;
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_get_cfg_file(
    mut opt_name: *const ::core::ffi::c_char,
    mut oleng: *mut uint16_t,
    mut odata: *mut *const uint8_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut nleng: uint32_t = 0;
        let mut cleng: uint16_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        if master_version()
            < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 42 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            return MFS_ERROR_ENOTSUP as uint8_t;
        }
        nleng = strlen(opt_name) as uint32_t;
        if nleng > 255 as uint32_t {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        wptr = fs_createpacket(
            rec,
            ANTOAN_GET_CONFIG_FILE as uint32_t,
            (1 as uint32_t).wrapping_add(nleng),
        );
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put8bit(&raw mut wptr, nleng as uint8_t);
        memcpy(
            wptr as *mut ::core::ffi::c_void,
            opt_name as *const ::core::ffi::c_void,
            nleng as size_t,
        );
        rptr = fs_sendandreceive(rec, ANTOAN_CONFIG_FILE_CONTENT as uint32_t, &raw mut i);
        if rptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            return *rptr.offset(0 as isize);
        } else if i == 0 as uint32_t {
            fs_disconnect();
            return MFS_ERROR_IO as uint8_t;
        }
        cleng = get16bit(&raw mut rptr);
        if i != (2 as uint32_t).wrapping_add(cleng as uint32_t) {
            fs_disconnect();
            return MFS_ERROR_IO as uint8_t;
        }
        *oleng = cleng;
        *odata = rptr;
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_statfs(
    mut totalspace: *mut uint64_t,
    mut availspace: *mut uint64_t,
    mut freespace: *mut uint64_t,
    mut trashspace: *mut uint64_t,
    mut sustainedspace: *mut uint64_t,
    mut inodes: *mut uint32_t,
) {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        wptr = fs_createpacket(rec, CLTOMA_FUSE_STATFS as uint32_t, 0 as uint32_t);
        if wptr.is_null() {
            *totalspace = 0 as uint64_t;
            *availspace = 0 as uint64_t;
            *freespace = 0 as uint64_t;
            *trashspace = 0 as uint64_t;
            *sustainedspace = 0 as uint64_t;
            *inodes = 0 as uint32_t;
            return;
        }
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_STATFS as uint32_t, &raw mut i);
        if rptr.is_null() || i != 36 as uint32_t && i != 44 as uint32_t {
            *totalspace = 0 as uint64_t;
            *availspace = 0 as uint64_t;
            *freespace = 0 as uint64_t;
            *trashspace = 0 as uint64_t;
            *sustainedspace = 0 as uint64_t;
            *inodes = 0 as uint32_t;
        } else {
            *totalspace = get64bit(&raw mut rptr);
            *availspace = get64bit(&raw mut rptr);
            if i == 44 as uint32_t {
                *freespace = get64bit(&raw mut rptr);
            } else {
                *freespace = *availspace;
            }
            *trashspace = get64bit(&raw mut rptr);
            *sustainedspace = get64bit(&raw mut rptr);
            *inodes = get32bit(&raw mut rptr);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_access(
    mut inode: uint32_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut modemask: uint16_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        if master_version()
            < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
            || gids == 0 as uint32_t
        {
            wptr = fs_createpacket(rec, CLTOMA_FUSE_ACCESS as uint32_t, 13 as uint32_t);
            if wptr.is_null() {
                return MFS_ERROR_IO as uint8_t;
            }
            put32bit(&raw mut wptr, inode);
            put32bit(&raw mut wptr, uid);
            if gids > 0 as uint32_t {
                put32bit(&raw mut wptr, *gid.offset(0 as isize));
            } else {
                put32bit(&raw mut wptr, 0xffffffff as uint32_t);
            }
            put8bit(&raw mut wptr, modemask as uint8_t);
        } else {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_ACCESS as uint32_t,
                (14 as uint32_t).wrapping_add((4 as uint32_t).wrapping_mul(gids)),
            );
            if wptr.is_null() {
                return MFS_ERROR_IO as uint8_t;
            }
            put32bit(&raw mut wptr, inode);
            put32bit(&raw mut wptr, uid);
            put32bit(&raw mut wptr, gids);
            i = 0 as uint32_t;
            while i < gids {
                put32bit(&raw mut wptr, *gid.offset(i as isize));
                i = i.wrapping_add(1);
            }
            put16bit(&raw mut wptr, modemask);
        }
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_ACCESS as uint32_t, &raw mut i);
        if rptr.is_null() || i != 1 as uint32_t {
            ret = MFS_ERROR_IO as uint8_t;
        } else {
            ret = *rptr.offset(0 as isize);
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_path_lookup(
    mut base_inode: uint32_t,
    mut pleng: uint32_t,
    mut path: *const uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut parent_inode: *mut uint32_t,
    mut last_inode: *mut uint32_t,
    mut nleng: *mut uint8_t,
    mut name: *mut uint8_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut nlaux: uint8_t = 0;
        let mut ret: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        let mut asize: uint8_t = master_attrsize();
        if master_version()
            < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 14 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            return MFS_ERROR_ENOTSUP as uint8_t;
        } else {
            wptr = fs_createpacket(
                rec,
                CLTOMA_PATH_LOOKUP as uint32_t,
                (16 as uint32_t)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids))
                    .wrapping_add(pleng),
            );
        }
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, base_inode);
        put32bit(&raw mut wptr, pleng);
        memcpy(
            wptr as *mut ::core::ffi::c_void,
            path as *const ::core::ffi::c_void,
            pleng as size_t,
        );
        wptr = wptr.offset(pleng as isize);
        put32bit(&raw mut wptr, uid);
        if gids > 0 as uint32_t {
            put32bit(&raw mut wptr, gids);
            i = 0 as uint32_t;
            while i < gids {
                put32bit(&raw mut wptr, *gid.offset(i as isize));
                i = i.wrapping_add(1);
            }
        } else {
            put32bit(&raw mut wptr, 0xffffffff as uint32_t);
        }
        rptr = fs_sendandreceive(rec, MATOCL_PATH_LOOKUP as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else if i
            < (9 as ::core::ffi::c_uint).wrapping_add(asize as ::core::ffi::c_uint) as uint32_t
        {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        } else {
            *parent_inode = get32bit(&raw mut rptr);
            nlaux = get8bit(&raw mut rptr);
            if i != (9 as ::core::ffi::c_uint)
                .wrapping_add(nlaux as ::core::ffi::c_uint)
                .wrapping_add(asize as ::core::ffi::c_uint) as uint32_t
            {
                fs_disconnect();
                ret = MFS_ERROR_IO as uint8_t;
            } else {
                if nlaux as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    memcpy(
                        name as *mut ::core::ffi::c_void,
                        rptr as *const ::core::ffi::c_void,
                        nlaux as size_t,
                    );
                    rptr = rptr.offset(nlaux as ::core::ffi::c_int as isize);
                }
                *name.offset(nlaux as isize) = '\0' as uint8_t;
                *nleng = nlaux;
                *last_inode = get32bit(&raw mut rptr);
                copy_attr(rptr, attr, asize);
                ret = MFS_STATUS_OK as uint8_t;
            }
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_simple_lookup(
    mut parent: uint32_t,
    mut nleng: uint8_t,
    mut name: *const uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut inode: *mut uint32_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut packetver: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        let mut asize: uint8_t = master_attrsize();
        if master_version()
            < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_LOOKUP as uint32_t,
                (13 as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as uint32_t,
            );
            packetver = 0 as uint8_t;
        } else {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_LOOKUP as uint32_t,
                (13 as uint32_t)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids))
                    .wrapping_add(nleng as uint32_t),
            );
            packetver = 1 as uint8_t;
        }
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, parent);
        put8bit(&raw mut wptr, nleng);
        memcpy(
            wptr as *mut ::core::ffi::c_void,
            name as *const ::core::ffi::c_void,
            nleng as size_t,
        );
        wptr = wptr.offset(nleng as ::core::ffi::c_int as isize);
        put32bit(&raw mut wptr, uid);
        if packetver as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if gids > 0 as uint32_t {
                put32bit(&raw mut wptr, *gid.offset(0 as isize));
            } else {
                put32bit(&raw mut wptr, 0xffffffff as uint32_t);
            }
        } else if gids > 0 as uint32_t {
            put32bit(&raw mut wptr, gids);
            i = 0 as uint32_t;
            while i < gids {
                put32bit(&raw mut wptr, *gid.offset(i as isize));
                i = i.wrapping_add(1);
            }
        } else {
            put32bit(&raw mut wptr, 0xffffffff as uint32_t);
        }
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_LOOKUP as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else if i == (4 as ::core::ffi::c_int + asize as ::core::ffi::c_int) as uint32_t
            || i >= (6 as ::core::ffi::c_int + asize as ::core::ffi::c_int) as uint32_t
        {
            *inode = get32bit(&raw mut rptr);
            copy_attr(rptr, attr, asize);
            ret = MFS_STATUS_OK as uint8_t;
        } else {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_lookup(
    mut parent: uint32_t,
    mut nleng: uint8_t,
    mut name: *const uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut inode: *mut uint32_t,
    mut attr: *mut uint8_t,
    mut lflags: *mut uint16_t,
    mut csdataver: *mut uint8_t,
    mut chunkid: *mut uint64_t,
    mut version: *mut uint32_t,
    mut csdata: *mut *const uint8_t,
    mut csdatasize: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut packetver: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        let mut asize: uint8_t = master_attrsize();
        if master_version()
            < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_LOOKUP as uint32_t,
                (13 as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as uint32_t,
            );
            packetver = 0 as uint8_t;
        } else {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_LOOKUP as uint32_t,
                (13 as uint32_t)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids))
                    .wrapping_add(nleng as uint32_t),
            );
            packetver = 1 as uint8_t;
        }
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, parent);
        put8bit(&raw mut wptr, nleng);
        memcpy(
            wptr as *mut ::core::ffi::c_void,
            name as *const ::core::ffi::c_void,
            nleng as size_t,
        );
        wptr = wptr.offset(nleng as ::core::ffi::c_int as isize);
        put32bit(&raw mut wptr, uid);
        if packetver as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if gids > 0 as uint32_t {
                put32bit(&raw mut wptr, *gid.offset(0 as isize));
            } else {
                put32bit(&raw mut wptr, 0xffffffff as uint32_t);
            }
        } else if gids > 0 as uint32_t {
            put32bit(&raw mut wptr, gids);
            i = 0 as uint32_t;
            while i < gids {
                put32bit(&raw mut wptr, *gid.offset(i as isize));
                i = i.wrapping_add(1);
            }
        } else {
            put32bit(&raw mut wptr, 0xffffffff as uint32_t);
        }
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_LOOKUP as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else if i == (4 as ::core::ffi::c_int + asize as ::core::ffi::c_int) as uint32_t {
            *inode = get32bit(&raw mut rptr);
            copy_attr(rptr, attr, asize);
            *lflags = 0xffff as uint16_t;
            ret = MFS_STATUS_OK as uint8_t;
        } else if i >= (6 as ::core::ffi::c_int + asize as ::core::ffi::c_int) as uint32_t {
            *inode = get32bit(&raw mut rptr);
            copy_attr(rptr, attr, asize);
            rptr = rptr.offset(asize as ::core::ffi::c_int as isize);
            *lflags = get16bit(&raw mut rptr);
            ret = MFS_STATUS_OK as uint8_t;
            if *lflags as ::core::ffi::c_int & LOOKUP_CHUNK_ZERO_DATA != 0 {
                if i >= (19 as ::core::ffi::c_int + asize as ::core::ffi::c_int) as uint32_t {
                    *csdataver = get8bit(&raw mut rptr);
                    *chunkid = get64bit(&raw mut rptr);
                    *version = get32bit(&raw mut rptr);
                    *csdata = rptr;
                    *csdatasize = i.wrapping_sub(
                        (19 as ::core::ffi::c_int + asize as ::core::ffi::c_int) as uint32_t,
                    );
                    if *csdataver as ::core::ffi::c_int != 2 as ::core::ffi::c_int
                        && *csdataver as ::core::ffi::c_int != 3 as ::core::ffi::c_int
                    {
                        ret = MFS_ERROR_IO as uint8_t;
                    } else if *csdataver as ::core::ffi::c_int == 2 as ::core::ffi::c_int
                        && i.wrapping_sub(
                            (19 as ::core::ffi::c_int + asize as ::core::ffi::c_int) as uint32_t,
                        )
                        .wrapping_rem(14 as uint32_t)
                            != 0 as uint32_t
                    {
                        ret = MFS_ERROR_IO as uint8_t;
                    } else if *csdataver as ::core::ffi::c_int == 3 as ::core::ffi::c_int
                        && i != ((19 as ::core::ffi::c_int + asize as ::core::ffi::c_int)
                            as uint32_t)
                            .wrapping_add(
                                (8 as ::core::ffi::c_int * 14 as ::core::ffi::c_int) as uint32_t,
                            )
                        && i != ((19 as ::core::ffi::c_int + asize as ::core::ffi::c_int)
                            as uint32_t)
                            .wrapping_add(
                                (4 as ::core::ffi::c_int * 14 as ::core::ffi::c_int) as uint32_t,
                            )
                    {
                        ret = MFS_ERROR_IO as uint8_t;
                    }
                } else {
                    ret = MFS_ERROR_IO as uint8_t;
                }
            } else {
                *csdataver = 0 as uint8_t;
                *chunkid = 0 as uint64_t;
                *version = 0 as uint32_t;
                *csdata = ::core::ptr::null::<uint8_t>();
                *csdatasize = 0 as uint32_t;
            }
            if ret as ::core::ffi::c_int == MFS_ERROR_IO {
                fs_disconnect();
            }
        } else {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_getattr(
    mut inode: uint32_t,
    mut opened: uint8_t,
    mut uid: uint32_t,
    mut gid: uint32_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut packetver: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        let mut asize: uint8_t = master_attrsize();
        if master_version()
            < (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    28 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    28 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = fs_createpacket(rec, CLTOMA_FUSE_GETATTR as uint32_t, 12 as uint32_t);
            packetver = 0 as uint8_t;
        } else {
            wptr = fs_createpacket(rec, CLTOMA_FUSE_GETATTR as uint32_t, 13 as uint32_t);
            packetver = 1 as uint8_t;
        }
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, inode);
        if packetver as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
            put8bit(&raw mut wptr, opened);
        }
        put32bit(&raw mut wptr, uid);
        put32bit(&raw mut wptr, gid);
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_GETATTR as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else if i != asize as uint32_t {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        } else {
            copy_attr(rptr, attr, asize);
            ret = MFS_STATUS_OK as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_setattr(
    mut inode: uint32_t,
    mut opened: uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut setmask: uint8_t,
    mut attrmode: uint16_t,
    mut attruid: uint32_t,
    mut attrgid: uint32_t,
    mut attratime: uint32_t,
    mut attrmtime: uint32_t,
    mut winattr: uint8_t,
    mut sugidclearmode: uint8_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut packetver: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        let mut asize: uint8_t = master_attrsize();
        let mut mv: uint32_t = master_version();
        if mv
            < (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    25 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    25 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = fs_createpacket(rec, CLTOMA_FUSE_SETATTR as uint32_t, 31 as uint32_t);
            packetver = 0 as uint8_t;
        } else if mv
            < (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    28 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    28 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = fs_createpacket(rec, CLTOMA_FUSE_SETATTR as uint32_t, 32 as uint32_t);
            packetver = 1 as uint8_t;
        } else if mv
            < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = fs_createpacket(rec, CLTOMA_FUSE_SETATTR as uint32_t, 33 as uint32_t);
            packetver = 2 as uint8_t;
        } else if mv
            < (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    93 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    93 as ::core::ffi::c_int
                })) as uint32_t
            || mv
                == (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
            || mv
                == (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    })) as uint32_t
        {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_SETATTR as uint32_t,
                (33 as uint32_t).wrapping_add(gids.wrapping_mul(4 as uint32_t)),
            );
            packetver = 3 as uint8_t;
        } else {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_SETATTR as uint32_t,
                (34 as uint32_t).wrapping_add(gids.wrapping_mul(4 as uint32_t)),
            );
            packetver = 4 as uint8_t;
        }
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, inode);
        if packetver as ::core::ffi::c_int >= 2 as ::core::ffi::c_int {
            put8bit(&raw mut wptr, opened);
        }
        put32bit(&raw mut wptr, uid);
        if packetver as ::core::ffi::c_int <= 2 as ::core::ffi::c_int {
            if gids > 0 as uint32_t {
                put32bit(&raw mut wptr, *gid.offset(0 as isize));
            } else {
                put32bit(&raw mut wptr, 0xffffffff as uint32_t);
            }
        } else if gids > 0 as uint32_t {
            put32bit(&raw mut wptr, gids);
            i = 0 as uint32_t;
            while i < gids {
                put32bit(&raw mut wptr, *gid.offset(i as isize));
                i = i.wrapping_add(1);
            }
        } else {
            put32bit(&raw mut wptr, 0xffffffff as uint32_t);
        }
        put8bit(&raw mut wptr, setmask);
        put16bit(&raw mut wptr, attrmode);
        put32bit(&raw mut wptr, attruid);
        put32bit(&raw mut wptr, attrgid);
        put32bit(&raw mut wptr, attratime);
        put32bit(&raw mut wptr, attrmtime);
        if packetver as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
            put8bit(&raw mut wptr, winattr);
        }
        if packetver as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
            put8bit(&raw mut wptr, sugidclearmode);
        }
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_SETATTR as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else if i != asize as uint32_t {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        } else {
            copy_attr(rptr, attr, asize);
            ret = MFS_STATUS_OK as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_truncate(
    mut inode: uint32_t,
    mut flags: uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut attrlength: uint64_t,
    mut attr: *mut uint8_t,
    mut prevlength: *mut uint64_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut packetver: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        let mut asize: uint8_t = master_attrsize();
        if master_version()
            < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = fs_createpacket(rec, CLTOMA_FUSE_TRUNCATE as uint32_t, 21 as uint32_t);
            packetver = 0 as uint8_t;
        } else {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_TRUNCATE as uint32_t,
                (21 as uint32_t).wrapping_add((4 as uint32_t).wrapping_mul(gids)),
            );
            packetver = 1 as uint8_t;
        }
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, inode);
        put8bit(&raw mut wptr, flags);
        put32bit(&raw mut wptr, uid);
        if packetver as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if gids > 0 as uint32_t {
                put32bit(&raw mut wptr, *gid.offset(0 as isize));
            } else {
                put32bit(&raw mut wptr, 0xffffffff as uint32_t);
            }
        } else if gids > 0 as uint32_t {
            put32bit(&raw mut wptr, gids);
            i = 0 as uint32_t;
            while i < gids {
                put32bit(&raw mut wptr, *gid.offset(i as isize));
                i = i.wrapping_add(1);
            }
        } else {
            put32bit(&raw mut wptr, 0xffffffff as uint32_t);
        }
        put64bit(&raw mut wptr, attrlength);
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_TRUNCATE as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else if i == asize as uint32_t {
            if !attr.is_null() {
                copy_attr(rptr, attr, asize);
            }
            ret = MFS_STATUS_OK as uint8_t;
        } else if i == (asize as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as uint32_t {
            if !prevlength.is_null() {
                *prevlength = get64bit(&raw mut rptr);
            } else {
                rptr = rptr.offset(8 as ::core::ffi::c_int as isize);
            }
            if !attr.is_null() {
                copy_attr(rptr, attr, asize);
            }
            ret = MFS_STATUS_OK as uint8_t;
        } else {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_readlink(
    mut inode: uint32_t,
    mut path: *mut *const uint8_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut pleng: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        wptr = fs_createpacket(rec, CLTOMA_FUSE_READLINK as uint32_t, 4 as uint32_t);
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, inode);
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_READLINK as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else if i < 4 as uint32_t {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        } else {
            pleng = get32bit(&raw mut rptr);
            if i != (4 as uint32_t).wrapping_add(pleng)
                || pleng == 0 as uint32_t
                || *rptr.offset(pleng.wrapping_sub(1 as uint32_t) as isize) as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
            {
                fs_disconnect();
                ret = MFS_ERROR_IO as uint8_t;
            } else {
                *path = rptr;
                ret = MFS_STATUS_OK as uint8_t;
            }
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_symlink(
    mut parent: uint32_t,
    mut nleng: uint8_t,
    mut name: *const uint8_t,
    mut path: *const uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut inode: *mut uint32_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut pleng: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut packetver: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        let mut asize: uint8_t = master_attrsize();
        pleng = strlen(path as *const ::core::ffi::c_char).wrapping_add(1 as size_t) as uint32_t;
        if master_version()
            < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_SYMLINK as uint32_t,
                pleng
                    .wrapping_add(nleng as uint32_t)
                    .wrapping_add(17 as uint32_t),
            );
            packetver = 0 as uint8_t;
        } else {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_SYMLINK as uint32_t,
                pleng
                    .wrapping_add(nleng as uint32_t)
                    .wrapping_add(17 as uint32_t)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids)),
            );
            packetver = 1 as uint8_t;
        }
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, parent);
        put8bit(&raw mut wptr, nleng);
        memcpy(
            wptr as *mut ::core::ffi::c_void,
            name as *const ::core::ffi::c_void,
            nleng as size_t,
        );
        wptr = wptr.offset(nleng as ::core::ffi::c_int as isize);
        put32bit(&raw mut wptr, pleng);
        memcpy(
            wptr as *mut ::core::ffi::c_void,
            path as *const ::core::ffi::c_void,
            pleng as size_t,
        );
        wptr = wptr.offset(pleng as isize);
        put32bit(&raw mut wptr, uid);
        if packetver as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if gids > 0 as uint32_t {
                put32bit(&raw mut wptr, *gid.offset(0 as isize));
            } else {
                put32bit(&raw mut wptr, 0xffffffff as uint32_t);
            }
        } else if gids > 0 as uint32_t {
            put32bit(&raw mut wptr, gids);
            i = 0 as uint32_t;
            while i < gids {
                put32bit(&raw mut wptr, *gid.offset(i as isize));
                i = i.wrapping_add(1);
            }
        } else {
            put32bit(&raw mut wptr, 0xffffffff as uint32_t);
        }
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_SYMLINK as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else if i != (4 as ::core::ffi::c_int + asize as ::core::ffi::c_int) as uint32_t {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        } else {
            *inode = get32bit(&raw mut rptr);
            copy_attr(rptr, attr, asize);
            ret = MFS_STATUS_OK as uint8_t;
        }
        return ret;
    }
}
#[inline]
unsafe extern "C" fn fsnodes_type_back_convert(mut r#type: uint8_t) -> uint8_t {
    match r#type as ::core::ffi::c_int {
        TYPE_FILE => return DISP_TYPE_FILE as uint8_t,
        TYPE_DIRECTORY => return DISP_TYPE_DIRECTORY as uint8_t,
        TYPE_SYMLINK => return DISP_TYPE_SYMLINK as uint8_t,
        TYPE_FIFO => return DISP_TYPE_FIFO as uint8_t,
        TYPE_BLOCKDEV => return DISP_TYPE_BLOCKDEV as uint8_t,
        TYPE_CHARDEV => return DISP_TYPE_CHARDEV as uint8_t,
        TYPE_SOCKET => return DISP_TYPE_SOCKET as uint8_t,
        TYPE_TRASH => return DISP_TYPE_TRASH as uint8_t,
        TYPE_SUSTAINED => return DISP_TYPE_SUSTAINED as uint8_t,
        _ => {}
    }
    return r#type;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mknod(
    mut parent: uint32_t,
    mut nleng: uint8_t,
    mut name: *const uint8_t,
    mut r#type: uint8_t,
    mut mode: uint16_t,
    mut cumask: uint16_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut rdev: uint32_t,
    mut inode: *mut uint32_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut packetver: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        let mut asize: uint8_t = master_attrsize();
        if master_version()
            < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            mode = (mode as ::core::ffi::c_int & !(cumask as ::core::ffi::c_int)) as uint16_t;
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_MKNOD as uint32_t,
                (20 as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as uint32_t,
            );
            packetver = 0 as uint8_t;
        } else {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_MKNOD as uint32_t,
                ((22 as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as uint32_t)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids)),
            );
            packetver = 1 as uint8_t;
        }
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, parent);
        put8bit(&raw mut wptr, nleng);
        memcpy(
            wptr as *mut ::core::ffi::c_void,
            name as *const ::core::ffi::c_void,
            nleng as size_t,
        );
        wptr = wptr.offset(nleng as ::core::ffi::c_int as isize);
        if master_version()
            < (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    32 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    32 as ::core::ffi::c_int
                })) as uint32_t
        {
            r#type = fsnodes_type_back_convert(r#type);
        }
        put8bit(&raw mut wptr, r#type);
        put16bit(&raw mut wptr, mode);
        if packetver as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
            put16bit(&raw mut wptr, cumask);
        }
        put32bit(&raw mut wptr, uid);
        if packetver as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if gids > 0 as uint32_t {
                put32bit(&raw mut wptr, *gid.offset(0 as isize));
            } else {
                put32bit(&raw mut wptr, 0xffffffff as uint32_t);
            }
        } else if gids > 0 as uint32_t {
            put32bit(&raw mut wptr, gids);
            i = 0 as uint32_t;
            while i < gids {
                put32bit(&raw mut wptr, *gid.offset(i as isize));
                i = i.wrapping_add(1);
            }
        } else {
            put32bit(&raw mut wptr, 0xffffffff as uint32_t);
        }
        put32bit(&raw mut wptr, rdev);
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_MKNOD as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else if i != (4 as ::core::ffi::c_int + asize as ::core::ffi::c_int) as uint32_t {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        } else {
            *inode = get32bit(&raw mut rptr);
            copy_attr(rptr, attr, asize);
            ret = MFS_STATUS_OK as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_mkdir(
    mut parent: uint32_t,
    mut nleng: uint8_t,
    mut name: *const uint8_t,
    mut mode: uint16_t,
    mut cumask: uint16_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut copysgid: uint8_t,
    mut inode: *mut uint32_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut packetver: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        let mut asize: uint8_t = master_attrsize();
        if master_version()
            < (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    25 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    25 as ::core::ffi::c_int
                })) as uint32_t
        {
            mode = (mode as ::core::ffi::c_int & !(cumask as ::core::ffi::c_int)) as uint16_t;
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_MKDIR as uint32_t,
                (15 as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as uint32_t,
            );
            packetver = 0 as uint8_t;
        } else if master_version()
            < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            mode = (mode as ::core::ffi::c_int & !(cumask as ::core::ffi::c_int)) as uint16_t;
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_MKDIR as uint32_t,
                (16 as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as uint32_t,
            );
            packetver = 1 as uint8_t;
        } else {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_MKDIR as uint32_t,
                ((18 as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as uint32_t)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids)),
            );
            packetver = 2 as uint8_t;
        }
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, parent);
        put8bit(&raw mut wptr, nleng);
        memcpy(
            wptr as *mut ::core::ffi::c_void,
            name as *const ::core::ffi::c_void,
            nleng as size_t,
        );
        wptr = wptr.offset(nleng as ::core::ffi::c_int as isize);
        put16bit(&raw mut wptr, mode);
        if packetver as ::core::ffi::c_int >= 2 as ::core::ffi::c_int {
            put16bit(&raw mut wptr, cumask);
        }
        put32bit(&raw mut wptr, uid);
        if (packetver as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
            if gids > 0 as uint32_t {
                put32bit(&raw mut wptr, *gid.offset(0 as isize));
            } else {
                put32bit(&raw mut wptr, 0xffffffff as uint32_t);
            }
        } else if gids > 0 as uint32_t {
            put32bit(&raw mut wptr, gids);
            i = 0 as uint32_t;
            while i < gids {
                put32bit(&raw mut wptr, *gid.offset(i as isize));
                i = i.wrapping_add(1);
            }
        } else {
            put32bit(&raw mut wptr, 0xffffffff as uint32_t);
        }
        if packetver as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
            put8bit(&raw mut wptr, copysgid);
        }
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_MKDIR as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else if i != (4 as ::core::ffi::c_int + asize as ::core::ffi::c_int) as uint32_t {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        } else {
            *inode = get32bit(&raw mut rptr);
            copy_attr(rptr, attr, asize);
            ret = MFS_STATUS_OK as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_unlink(
    mut parent: uint32_t,
    mut nleng: uint8_t,
    mut name: *const uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut inode: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut packetver: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        if master_version()
            < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_UNLINK as uint32_t,
                (13 as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as uint32_t,
            );
            packetver = 0 as uint8_t;
        } else {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_UNLINK as uint32_t,
                ((13 as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as uint32_t)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids)),
            );
            packetver = 1 as uint8_t;
        }
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, parent);
        put8bit(&raw mut wptr, nleng);
        memcpy(
            wptr as *mut ::core::ffi::c_void,
            name as *const ::core::ffi::c_void,
            nleng as size_t,
        );
        wptr = wptr.offset(nleng as ::core::ffi::c_int as isize);
        put32bit(&raw mut wptr, uid);
        if packetver as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if gids > 0 as uint32_t {
                put32bit(&raw mut wptr, *gid.offset(0 as isize));
            } else {
                put32bit(&raw mut wptr, 0xffffffff as uint32_t);
            }
        } else if gids > 0 as uint32_t {
            put32bit(&raw mut wptr, gids);
            i = 0 as uint32_t;
            while i < gids {
                put32bit(&raw mut wptr, *gid.offset(i as isize));
                i = i.wrapping_add(1);
            }
        } else {
            put32bit(&raw mut wptr, 0xffffffff as uint32_t);
        }
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_UNLINK as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
            *inode = 0 as uint32_t;
        } else if i == 4 as uint32_t {
            ret = MFS_STATUS_OK as uint8_t;
            *inode = get32bit(&raw mut rptr);
        } else {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_rmdir(
    mut parent: uint32_t,
    mut nleng: uint8_t,
    mut name: *const uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut inode: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut packetver: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        if master_version()
            < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_RMDIR as uint32_t,
                (13 as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as uint32_t,
            );
            packetver = 0 as uint8_t;
        } else {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_RMDIR as uint32_t,
                ((13 as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as uint32_t)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids)),
            );
            packetver = 1 as uint8_t;
        }
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, parent);
        put8bit(&raw mut wptr, nleng);
        memcpy(
            wptr as *mut ::core::ffi::c_void,
            name as *const ::core::ffi::c_void,
            nleng as size_t,
        );
        wptr = wptr.offset(nleng as ::core::ffi::c_int as isize);
        put32bit(&raw mut wptr, uid);
        if packetver as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if gids > 0 as uint32_t {
                put32bit(&raw mut wptr, *gid.offset(0 as isize));
            } else {
                put32bit(&raw mut wptr, 0xffffffff as uint32_t);
            }
        } else if gids > 0 as uint32_t {
            put32bit(&raw mut wptr, gids);
            i = 0 as uint32_t;
            while i < gids {
                put32bit(&raw mut wptr, *gid.offset(i as isize));
                i = i.wrapping_add(1);
            }
        } else {
            put32bit(&raw mut wptr, 0xffffffff as uint32_t);
        }
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_RMDIR as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
            *inode = 0 as uint32_t;
        } else if i == 4 as uint32_t {
            ret = MFS_STATUS_OK as uint8_t;
            *inode = get32bit(&raw mut rptr);
        } else {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_rename(
    mut parent_src: uint32_t,
    mut nleng_src: uint8_t,
    mut name_src: *const uint8_t,
    mut parent_dst: uint32_t,
    mut nleng_dst: uint8_t,
    mut name_dst: *const uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut mfsflags: uint8_t,
    mut inode: *mut uint32_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut packetver: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        let mut asize: uint8_t = master_attrsize();
        if master_version()
            < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_RENAME as uint32_t,
                (18 as ::core::ffi::c_int
                    + nleng_src as ::core::ffi::c_int
                    + nleng_dst as ::core::ffi::c_int) as uint32_t,
            );
            packetver = 0 as uint8_t;
        } else if master_version()
            < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 18 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_RENAME as uint32_t,
                ((18 as ::core::ffi::c_int
                    + nleng_src as ::core::ffi::c_int
                    + nleng_dst as ::core::ffi::c_int) as uint32_t)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids)),
            );
            packetver = 1 as uint8_t;
        } else {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_RENAME as uint32_t,
                ((19 as ::core::ffi::c_int
                    + nleng_src as ::core::ffi::c_int
                    + nleng_dst as ::core::ffi::c_int) as uint32_t)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids)),
            );
            packetver = 2 as uint8_t;
        }
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, parent_src);
        put8bit(&raw mut wptr, nleng_src);
        memcpy(
            wptr as *mut ::core::ffi::c_void,
            name_src as *const ::core::ffi::c_void,
            nleng_src as size_t,
        );
        wptr = wptr.offset(nleng_src as ::core::ffi::c_int as isize);
        put32bit(&raw mut wptr, parent_dst);
        put8bit(&raw mut wptr, nleng_dst);
        memcpy(
            wptr as *mut ::core::ffi::c_void,
            name_dst as *const ::core::ffi::c_void,
            nleng_dst as size_t,
        );
        wptr = wptr.offset(nleng_dst as ::core::ffi::c_int as isize);
        put32bit(&raw mut wptr, uid);
        if packetver as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if gids > 0 as uint32_t {
                put32bit(&raw mut wptr, *gid.offset(0 as isize));
            } else {
                put32bit(&raw mut wptr, 0xffffffff as uint32_t);
            }
        } else {
            if gids > 0 as uint32_t {
                put32bit(&raw mut wptr, gids);
                i = 0 as uint32_t;
                while i < gids {
                    put32bit(&raw mut wptr, *gid.offset(i as isize));
                    i = i.wrapping_add(1);
                }
            } else {
                put32bit(&raw mut wptr, 0xffffffff as uint32_t);
            }
            if packetver as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                put8bit(&raw mut wptr, mfsflags);
            }
        }
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_RENAME as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
            *inode = 0 as uint32_t;
            memset(
                attr as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ATTR_RECORD_SIZE as size_t,
            );
        } else if i != (4 as ::core::ffi::c_int + asize as ::core::ffi::c_int) as uint32_t {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        } else {
            *inode = get32bit(&raw mut rptr);
            copy_attr(rptr, attr, asize);
            ret = MFS_STATUS_OK as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_link(
    mut inode_src: uint32_t,
    mut parent_dst: uint32_t,
    mut nleng_dst: uint8_t,
    mut name_dst: *const uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut inode: *mut uint32_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut packetver: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        let mut asize: uint8_t = master_attrsize();
        if master_version()
            < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_LINK as uint32_t,
                (17 as ::core::ffi::c_int + nleng_dst as ::core::ffi::c_int) as uint32_t,
            );
            packetver = 0 as uint8_t;
        } else {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_LINK as uint32_t,
                ((17 as ::core::ffi::c_int + nleng_dst as ::core::ffi::c_int) as uint32_t)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids)),
            );
            packetver = 1 as uint8_t;
        }
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, inode_src);
        put32bit(&raw mut wptr, parent_dst);
        put8bit(&raw mut wptr, nleng_dst);
        memcpy(
            wptr as *mut ::core::ffi::c_void,
            name_dst as *const ::core::ffi::c_void,
            nleng_dst as size_t,
        );
        wptr = wptr.offset(nleng_dst as ::core::ffi::c_int as isize);
        put32bit(&raw mut wptr, uid);
        if packetver as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if gids > 0 as uint32_t {
                put32bit(&raw mut wptr, *gid.offset(0 as isize));
            } else {
                put32bit(&raw mut wptr, 0xffffffff as uint32_t);
            }
        } else if gids > 0 as uint32_t {
            put32bit(&raw mut wptr, gids);
            i = 0 as uint32_t;
            while i < gids {
                put32bit(&raw mut wptr, *gid.offset(i as isize));
                i = i.wrapping_add(1);
            }
        } else {
            put32bit(&raw mut wptr, 0xffffffff as uint32_t);
        }
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_LINK as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else if i != (4 as ::core::ffi::c_int + asize as ::core::ffi::c_int) as uint32_t {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        } else {
            *inode = get32bit(&raw mut rptr);
            copy_attr(rptr, attr, asize);
            ret = MFS_STATUS_OK as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_readdir(
    mut inode: uint32_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut edgeid: *mut uint64_t,
    mut edgelimit: uint32_t,
    mut wantattr: uint8_t,
    mut addtocache: uint8_t,
    mut dbuff: *mut *const uint8_t,
    mut dbuffsize: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut flags: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        if master_version()
            < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            return MFS_ERROR_ENOTSUP as uint8_t;
        }
        if master_version()
            < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 58 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            *edgeid = 0 as uint64_t;
            edgelimit = 0xffffffff as ::core::ffi::c_uint as uint32_t;
        }
        wptr = fs_createpacket(
            rec,
            CLTOMA_FUSE_READDIR as uint32_t,
            (25 as uint32_t).wrapping_add((4 as uint32_t).wrapping_mul(gids)),
        );
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, inode);
        put32bit(&raw mut wptr, uid);
        if gids > 0 as uint32_t {
            put32bit(&raw mut wptr, gids);
            i = 0 as uint32_t;
            while i < gids {
                put32bit(&raw mut wptr, *gid.offset(i as isize));
                i = i.wrapping_add(1);
            }
        } else {
            put32bit(&raw mut wptr, 0xffffffff as uint32_t);
        }
        flags = 0 as uint8_t;
        if wantattr != 0 {
            flags = (flags as ::core::ffi::c_int | GETDIR_FLAG_WITHATTR) as uint8_t;
        }
        if addtocache != 0 {
            flags = (flags as ::core::ffi::c_int | GETDIR_FLAG_ADDTOCACHE) as uint8_t;
        }
        put8bit(&raw mut wptr, flags);
        put32bit(&raw mut wptr, edgelimit);
        if !edgeid.is_null() {
            put64bit(&raw mut wptr, *edgeid);
        } else {
            put64bit(&raw mut wptr, 0 as uint64_t);
        }
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_READDIR as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else {
            if !edgeid.is_null() {
                *edgeid = get64bit(&raw mut rptr);
                if master_version()
                    < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 58 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })) as uint32_t
                {
                    *edgeid = 0x7fffffffffffffff as ::core::ffi::c_ulong as uint64_t;
                }
            }
            i = i.wrapping_sub(8 as uint32_t);
            *dbuff = rptr;
            *dbuffsize = i;
            ret = MFS_STATUS_OK as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_create(
    mut parent: uint32_t,
    mut nleng: uint8_t,
    mut name: *const uint8_t,
    mut mode: uint16_t,
    mut cumask: uint16_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut inode: *mut uint32_t,
    mut attr: *mut uint8_t,
    mut oflags: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut packetver: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        let mut asize: uint8_t = master_attrsize();
        if master_version()
            < (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    25 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    25 as ::core::ffi::c_int
                })) as uint32_t
        {
            return MFS_ERROR_ENOTSUP as uint8_t;
        }
        if master_version()
            < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            mode = (mode as ::core::ffi::c_int & !(cumask as ::core::ffi::c_int)) as uint16_t;
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_CREATE as uint32_t,
                (15 as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as uint32_t,
            );
            packetver = 0 as uint8_t;
        } else {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_CREATE as uint32_t,
                ((17 as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as uint32_t)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids)),
            );
            packetver = 1 as uint8_t;
        }
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, parent);
        put8bit(&raw mut wptr, nleng);
        memcpy(
            wptr as *mut ::core::ffi::c_void,
            name as *const ::core::ffi::c_void,
            nleng as size_t,
        );
        wptr = wptr.offset(nleng as ::core::ffi::c_int as isize);
        put16bit(&raw mut wptr, mode);
        if packetver as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
            put16bit(&raw mut wptr, cumask);
        }
        put32bit(&raw mut wptr, uid);
        if packetver as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if gids > 0 as uint32_t {
                put32bit(&raw mut wptr, *gid.offset(0 as isize));
            } else {
                put32bit(&raw mut wptr, 0xffffffff as uint32_t);
            }
        } else if gids > 0 as uint32_t {
            put32bit(&raw mut wptr, gids);
            i = 0 as uint32_t;
            while i < gids {
                put32bit(&raw mut wptr, *gid.offset(i as isize));
                i = i.wrapping_add(1);
            }
        } else {
            put32bit(&raw mut wptr, 0xffffffff as uint32_t);
        }
        fd_lock();
        donotsendsustainedinodes = 1 as ::core::ffi::c_int;
        fd_unlock();
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_CREATE as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else if i == (4 as ::core::ffi::c_int + asize as ::core::ffi::c_int) as uint32_t {
            *oflags = 0xff as uint8_t;
            *inode = get32bit(&raw mut rptr);
            copy_attr(rptr, attr, asize);
            ret = MFS_STATUS_OK as uint8_t;
        } else if i == (5 as ::core::ffi::c_int + asize as ::core::ffi::c_int) as uint32_t {
            *oflags = get8bit(&raw mut rptr);
            *inode = get32bit(&raw mut rptr);
            copy_attr(rptr, attr, asize);
            ret = MFS_STATUS_OK as uint8_t;
        } else {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        }
        fd_lock();
        donotsendsustainedinodes = 0 as ::core::ffi::c_int;
        fd_unlock();
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_opencheck(
    mut inode: uint32_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut flags: uint8_t,
    mut attr: *mut uint8_t,
    mut oflags: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut packetver: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        let mut asize: uint8_t = master_attrsize();
        if master_version()
            < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
            || gids == 0 as uint32_t
        {
            wptr = fs_createpacket(rec, CLTOMA_FUSE_OPEN as uint32_t, 13 as uint32_t);
            packetver = 0 as uint8_t;
        } else {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_OPEN as uint32_t,
                (13 as uint32_t).wrapping_add((4 as uint32_t).wrapping_mul(gids)),
            );
            packetver = 1 as uint8_t;
        }
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, inode);
        put32bit(&raw mut wptr, uid);
        if packetver as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if gids > 0 as uint32_t {
                put32bit(&raw mut wptr, *gid.offset(0 as isize));
            } else {
                put32bit(&raw mut wptr, 0xffffffff as uint32_t);
            }
        } else {
            put32bit(&raw mut wptr, gids);
            i = 0 as uint32_t;
            while i < gids {
                put32bit(&raw mut wptr, *gid.offset(i as isize));
                i = i.wrapping_add(1);
            }
        }
        put8bit(&raw mut wptr, flags);
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_OPEN as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            if !attr.is_null() {
                memset(
                    attr as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ATTR_RECORD_SIZE as size_t,
                );
            }
            if !oflags.is_null() {
                *oflags = 0xff as uint8_t;
            }
            ret = *rptr.offset(0 as isize);
        } else if i == asize as uint32_t {
            if !attr.is_null() {
                copy_attr(rptr, attr, asize);
            }
            if !oflags.is_null() {
                *oflags = 0xff as uint8_t;
            }
            ret = MFS_STATUS_OK as uint8_t;
        } else if i == (1 as ::core::ffi::c_int + asize as ::core::ffi::c_int) as uint32_t {
            if !oflags.is_null() {
                *oflags = *rptr.offset(0 as isize);
            }
            rptr = rptr.offset(1);
            if !attr.is_null() {
                copy_attr(rptr, attr, asize);
            }
            ret = MFS_STATUS_OK as uint8_t;
        } else {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_readchunk(
    mut inode: uint32_t,
    mut indx: uint32_t,
    mut chunkopflags: uint8_t,
    mut csdataver: *mut uint8_t,
    mut length: *mut uint64_t,
    mut chunkid: *mut uint64_t,
    mut version: *mut uint32_t,
    mut csdata: *mut *const uint8_t,
    mut csdatasize: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut packetver: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        *csdata = ::core::ptr::null::<uint8_t>();
        *csdatasize = 0 as uint32_t;
        if master_version()
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    4 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = fs_createpacket(rec, CLTOMA_FUSE_READ_CHUNK as uint32_t, 9 as uint32_t);
            packetver = 1 as uint8_t;
        } else {
            wptr = fs_createpacket(rec, CLTOMA_FUSE_READ_CHUNK as uint32_t, 8 as uint32_t);
            packetver = 0 as uint8_t;
        }
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, inode);
        put32bit(&raw mut wptr, indx);
        if packetver as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
            put8bit(&raw mut wptr, chunkopflags);
        }
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_READ_CHUNK as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else {
            if i & 1 as uint32_t != 0 {
                *csdataver = get8bit(&raw mut rptr);
                if i < 21 as uint32_t
                    || *csdataver as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                        && i.wrapping_sub(21 as uint32_t).wrapping_rem(10 as uint32_t)
                            != 0 as uint32_t
                    || *csdataver as ::core::ffi::c_int == 2 as ::core::ffi::c_int
                        && i.wrapping_sub(21 as uint32_t).wrapping_rem(14 as uint32_t)
                            != 0 as uint32_t
                    || *csdataver as ::core::ffi::c_int == 3 as ::core::ffi::c_int
                        && i != (21 as ::core::ffi::c_int
                            + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as uint32_t
                        && i != (21 as ::core::ffi::c_int
                            + 14 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                            as uint32_t
                {
                    ret = MFS_ERROR_IO as uint8_t;
                } else {
                    *csdatasize = i.wrapping_sub(21 as uint32_t);
                    ret = MFS_STATUS_OK as uint8_t;
                }
            } else {
                *csdataver = 0 as uint8_t;
                if i < 20 as uint32_t
                    || i.wrapping_sub(20 as uint32_t).wrapping_rem(6 as uint32_t) != 0 as uint32_t
                {
                    ret = MFS_ERROR_IO as uint8_t;
                } else {
                    *csdatasize = i.wrapping_sub(20 as uint32_t);
                    ret = MFS_STATUS_OK as uint8_t;
                }
            }
            if ret as ::core::ffi::c_int != MFS_STATUS_OK {
                fs_disconnect();
            } else {
                *length = get64bit(&raw mut rptr);
                *chunkid = get64bit(&raw mut rptr);
                *version = get32bit(&raw mut rptr);
                *csdata = rptr;
            }
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_writechunk(
    mut inode: uint32_t,
    mut indx: uint32_t,
    mut chunkopflags: uint8_t,
    mut csdataver: *mut uint8_t,
    mut length: *mut uint64_t,
    mut chunkid: *mut uint64_t,
    mut version: *mut uint32_t,
    mut csdata: *mut *const uint8_t,
    mut csdatasize: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        *csdata = ::core::ptr::null::<uint8_t>();
        *csdatasize = 0 as uint32_t;
        if master_version()
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    4 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = fs_createpacket(rec, CLTOMA_FUSE_WRITE_CHUNK as uint32_t, 9 as uint32_t);
        } else {
            wptr = fs_createpacket(rec, CLTOMA_FUSE_WRITE_CHUNK as uint32_t, 8 as uint32_t);
        }
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, inode);
        put32bit(&raw mut wptr, indx);
        if master_version()
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    4 as ::core::ffi::c_int
                })) as uint32_t
        {
            put8bit(&raw mut wptr, chunkopflags);
        }
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_WRITE_CHUNK as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else {
            if i & 1 as uint32_t != 0 {
                *csdataver = get8bit(&raw mut rptr);
                if i < 21 as uint32_t
                    || *csdataver as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                        && i.wrapping_sub(21 as uint32_t).wrapping_rem(10 as uint32_t)
                            != 0 as uint32_t
                    || *csdataver as ::core::ffi::c_int == 2 as ::core::ffi::c_int
                        && i.wrapping_sub(21 as uint32_t).wrapping_rem(14 as uint32_t)
                            != 0 as uint32_t
                {
                    ret = MFS_ERROR_IO as uint8_t;
                } else {
                    *csdatasize = i.wrapping_sub(21 as uint32_t);
                    ret = MFS_STATUS_OK as uint8_t;
                }
            } else {
                *csdataver = 0 as uint8_t;
                if i < 20 as uint32_t
                    || i.wrapping_sub(20 as uint32_t).wrapping_rem(6 as uint32_t) != 0 as uint32_t
                {
                    ret = MFS_ERROR_IO as uint8_t;
                } else {
                    *csdatasize = i.wrapping_sub(20 as uint32_t);
                    ret = MFS_STATUS_OK as uint8_t;
                }
            }
            if ret as ::core::ffi::c_int != MFS_STATUS_OK {
                fs_disconnect();
            } else {
                *length = get64bit(&raw mut rptr);
                *chunkid = get64bit(&raw mut rptr);
                *version = get32bit(&raw mut rptr);
                *csdata = rptr;
            }
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_writeend(
    mut chunkid: uint64_t,
    mut inode: uint32_t,
    mut indx: uint32_t,
    mut length: uint64_t,
    mut chunkopflags: uint8_t,
    mut offset: uint32_t,
    mut size: uint32_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        if master_version()
            >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 40 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = fs_createpacket(rec, CLTOMA_FUSE_WRITE_CHUNK_END as uint32_t, 33 as uint32_t);
        } else if master_version()
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    74 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    74 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = fs_createpacket(rec, CLTOMA_FUSE_WRITE_CHUNK_END as uint32_t, 25 as uint32_t);
        } else if master_version()
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    4 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = fs_createpacket(rec, CLTOMA_FUSE_WRITE_CHUNK_END as uint32_t, 21 as uint32_t);
        } else {
            wptr = fs_createpacket(rec, CLTOMA_FUSE_WRITE_CHUNK_END as uint32_t, 20 as uint32_t);
        }
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put64bit(&raw mut wptr, chunkid);
        put32bit(&raw mut wptr, inode);
        if master_version()
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    74 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    74 as ::core::ffi::c_int
                })) as uint32_t
        {
            put32bit(&raw mut wptr, indx);
        }
        put64bit(&raw mut wptr, length);
        if master_version()
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    4 as ::core::ffi::c_int
                })) as uint32_t
        {
            put8bit(&raw mut wptr, chunkopflags);
        }
        if master_version()
            >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 40 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            put32bit(&raw mut wptr, offset);
            put32bit(&raw mut wptr, size);
        }
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_WRITE_CHUNK_END as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_flock(
    mut inode: uint32_t,
    mut reqid: uint32_t,
    mut owner: uint64_t,
    mut cmd: uint8_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        wptr = fs_createpacket(rec, CLTOMA_FUSE_FLOCK as uint32_t, 17 as uint32_t);
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, inode);
        put32bit(&raw mut wptr, reqid);
        put64bit(&raw mut wptr, owner);
        put8bit(&raw mut wptr, cmd);
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_FLOCK as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_posixlock(
    mut inode: uint32_t,
    mut reqid: uint32_t,
    mut owner: uint64_t,
    mut cmd: uint8_t,
    mut r#type: uint8_t,
    mut start: uint64_t,
    mut end: uint64_t,
    mut pid: uint32_t,
    mut rtype: *mut uint8_t,
    mut rstart: *mut uint64_t,
    mut rend: *mut uint64_t,
    mut rpid: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        wptr = fs_createpacket(rec, CLTOMA_FUSE_POSIX_LOCK as uint32_t, 38 as uint32_t);
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, inode);
        put32bit(&raw mut wptr, reqid);
        put64bit(&raw mut wptr, owner);
        put32bit(&raw mut wptr, pid);
        put8bit(&raw mut wptr, cmd);
        put8bit(&raw mut wptr, r#type);
        put64bit(&raw mut wptr, start);
        put64bit(&raw mut wptr, end);
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_POSIX_LOCK as uint32_t, &raw mut i);
        if !rtype.is_null() {
            *rtype = POSIX_LOCK_UNLCK as uint8_t;
        }
        if !rstart.is_null() {
            *rstart = 0 as uint64_t;
        }
        if !rend.is_null() {
            *rend = 0 as uint64_t;
        }
        if !rpid.is_null() {
            *rpid = 0 as uint32_t;
        }
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else if i == 21 as uint32_t {
            if !rpid.is_null() {
                *rpid = get32bit(&raw mut rptr);
            } else {
                rptr = rptr.offset(4 as ::core::ffi::c_int as isize);
            }
            if !rtype.is_null() {
                *rtype = get8bit(&raw mut rptr);
            } else {
                rptr = rptr.offset(1);
            }
            if !rstart.is_null() {
                *rstart = get64bit(&raw mut rptr);
            } else {
                rptr = rptr.offset(8 as ::core::ffi::c_int as isize);
            }
            if !rend.is_null() {
                *rend = get64bit(&raw mut rptr);
            } else {
                rptr = rptr.offset(8 as ::core::ffi::c_int as isize);
            }
            ret = MFS_STATUS_OK as uint8_t;
        } else {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_getsustained(
    mut dbuff: *mut *const uint8_t,
    mut dbuffsize: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        wptr = fs_createpacket(rec, CLTOMA_FUSE_GETSUSTAINED as uint32_t, 0 as uint32_t);
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_GETSUSTAINED as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else {
            *dbuff = rptr;
            *dbuffsize = i;
            ret = MFS_STATUS_OK as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_gettrash(
    mut tid: uint32_t,
    mut dbuff: *mut *const uint8_t,
    mut dbuffsize: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        if master_version()
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    64 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    64 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = fs_createpacket(rec, CLTOMA_FUSE_GETTRASH as uint32_t, 4 as uint32_t);
        } else {
            wptr = fs_createpacket(rec, CLTOMA_FUSE_GETTRASH as uint32_t, 0 as uint32_t);
        }
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        if master_version()
            >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    64 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    64 as ::core::ffi::c_int
                })) as uint32_t
        {
            put32bit(&raw mut wptr, tid);
        }
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_GETTRASH as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else {
            *dbuff = rptr;
            *dbuffsize = i;
            ret = MFS_STATUS_OK as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_getdetachedattr(
    mut inode: uint32_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        let mut asize: uint8_t = master_attrsize();
        wptr = fs_createpacket(rec, CLTOMA_FUSE_GETDETACHEDATTR as uint32_t, 4 as uint32_t);
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, inode);
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_GETDETACHEDATTR as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else if i != asize as uint32_t {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        } else {
            copy_attr(rptr, attr, asize);
            ret = MFS_STATUS_OK as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_gettrashpath(
    mut inode: uint32_t,
    mut path: *mut *const uint8_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut pleng: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        wptr = fs_createpacket(rec, CLTOMA_FUSE_GETTRASHPATH as uint32_t, 4 as uint32_t);
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, inode);
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_GETTRASHPATH as uint32_t, &raw mut i);
        *path = ::core::ptr::null::<uint8_t>();
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else if i < 4 as uint32_t {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        } else {
            pleng = get32bit(&raw mut rptr);
            if i != (4 as uint32_t).wrapping_add(pleng)
                || pleng == 0 as uint32_t
                || *rptr.offset(pleng.wrapping_sub(1 as uint32_t) as isize) as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
            {
                fs_disconnect();
                ret = MFS_ERROR_IO as uint8_t;
            } else {
                *path = rptr;
                ret = MFS_STATUS_OK as uint8_t;
            }
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_settrashpath(mut inode: uint32_t, mut path: *const uint8_t) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut pleng: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        pleng = strlen(path as *const ::core::ffi::c_char).wrapping_add(1 as size_t) as uint32_t;
        wptr = fs_createpacket(
            rec,
            CLTOMA_FUSE_SETTRASHPATH as uint32_t,
            pleng.wrapping_add(8 as uint32_t),
        );
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, inode);
        put32bit(&raw mut wptr, pleng);
        memcpy(
            wptr as *mut ::core::ffi::c_void,
            path as *const ::core::ffi::c_void,
            pleng as size_t,
        );
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_SETTRASHPATH as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_undel(mut inode: uint32_t) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        wptr = fs_createpacket(rec, CLTOMA_FUSE_UNDEL as uint32_t, 4 as uint32_t);
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, inode);
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_UNDEL as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_purge(mut inode: uint32_t) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        wptr = fs_createpacket(rec, CLTOMA_FUSE_PURGE as uint32_t, 4 as uint32_t);
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, inode);
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_PURGE as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_getfacl(
    mut inode: uint32_t,
    mut acltype: uint8_t,
    mut userperm: *mut uint16_t,
    mut groupperm: *mut uint16_t,
    mut otherperm: *mut uint16_t,
    mut maskperm: *mut uint16_t,
    mut namedusers: *mut uint16_t,
    mut namedgroups: *mut uint16_t,
    mut namedacls: *mut *const uint8_t,
    mut namedaclssize: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        *namedacls = ::core::ptr::null::<uint8_t>();
        *namedaclssize = 0 as uint32_t;
        if master_version()
            < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            return MFS_ERROR_ENOTSUP as uint8_t;
        }
        if master_version()
            < (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    91 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    91 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = fs_createpacket(rec, CLTOMA_FUSE_GETFACL as uint32_t, 14 as uint32_t);
            if wptr.is_null() {
                return MFS_ERROR_IO as uint8_t;
            }
            put32bit(&raw mut wptr, inode);
            put8bit(&raw mut wptr, acltype);
            put8bit(&raw mut wptr, 1 as uint8_t);
            put32bit(&raw mut wptr, 0 as uint32_t);
            put32bit(&raw mut wptr, 0 as uint32_t);
        } else {
            wptr = fs_createpacket(rec, CLTOMA_FUSE_GETFACL as uint32_t, 5 as uint32_t);
            if wptr.is_null() {
                return MFS_ERROR_IO as uint8_t;
            }
            put32bit(&raw mut wptr, inode);
            put8bit(&raw mut wptr, acltype);
        }
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_GETFACL as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else if i < 12 as uint32_t {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        } else {
            *userperm = get16bit(&raw mut rptr);
            *groupperm = get16bit(&raw mut rptr);
            *otherperm = get16bit(&raw mut rptr);
            *maskperm = get16bit(&raw mut rptr);
            *namedusers = get16bit(&raw mut rptr);
            *namedgroups = get16bit(&raw mut rptr);
            *namedacls = rptr;
            *namedaclssize = i.wrapping_sub(12 as uint32_t);
            ret = MFS_STATUS_OK as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_setfacl(
    mut inode: uint32_t,
    mut uid: uint32_t,
    mut acltype: uint8_t,
    mut userperm: uint16_t,
    mut groupperm: uint16_t,
    mut otherperm: uint16_t,
    mut maskperm: uint16_t,
    mut namedusers: uint16_t,
    mut namedgroups: uint16_t,
    mut namedacls: *mut uint8_t,
    mut namedaclssize: uint32_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        if master_version()
            < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            return MFS_ERROR_ENOTSUP as uint8_t;
        }
        wptr = fs_createpacket(
            rec,
            CLTOMA_FUSE_SETFACL as uint32_t,
            (21 as uint32_t).wrapping_add(namedaclssize),
        );
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, inode);
        put32bit(&raw mut wptr, uid);
        put8bit(&raw mut wptr, acltype);
        put16bit(&raw mut wptr, userperm);
        put16bit(&raw mut wptr, groupperm);
        put16bit(&raw mut wptr, otherperm);
        put16bit(&raw mut wptr, maskperm);
        put16bit(&raw mut wptr, namedusers);
        put16bit(&raw mut wptr, namedgroups);
        memcpy(
            wptr as *mut ::core::ffi::c_void,
            namedacls as *const ::core::ffi::c_void,
            namedaclssize as size_t,
        );
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_SETFACL as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_getxattr(
    mut inode: uint32_t,
    mut opened: uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut nleng: uint8_t,
    mut name: *const uint8_t,
    mut mode: uint8_t,
    mut vbuff: *mut *const uint8_t,
    mut vleng: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut packetver: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        *vbuff = ::core::ptr::null::<uint8_t>();
        *vleng = 0 as uint32_t;
        if master_version()
            < (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            return MFS_ERROR_ENOTSUP as uint8_t;
        }
        if master_version()
            < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_GETXATTR as uint32_t,
                (15 as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as uint32_t,
            );
            packetver = 0 as uint8_t;
        } else {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_GETXATTR as uint32_t,
                ((15 as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as uint32_t)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids)),
            );
            packetver = 1 as uint8_t;
        }
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, inode);
        if packetver as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            put8bit(&raw mut wptr, opened);
            put32bit(&raw mut wptr, uid);
            if gids > 0 as uint32_t {
                put32bit(&raw mut wptr, *gid.offset(0 as isize));
            } else {
                put32bit(&raw mut wptr, 0xffffffff as uint32_t);
            }
        }
        put8bit(&raw mut wptr, nleng);
        memcpy(
            wptr as *mut ::core::ffi::c_void,
            name as *const ::core::ffi::c_void,
            nleng as size_t,
        );
        wptr = wptr.offset(nleng as ::core::ffi::c_int as isize);
        put8bit(&raw mut wptr, mode);
        if packetver as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
            put8bit(&raw mut wptr, opened);
            put32bit(&raw mut wptr, uid);
            if gids > 0 as uint32_t {
                put32bit(&raw mut wptr, gids);
                i = 0 as uint32_t;
                while i < gids {
                    put32bit(&raw mut wptr, *gid.offset(i as isize));
                    i = i.wrapping_add(1);
                }
            } else {
                put32bit(&raw mut wptr, 0xffffffff as uint32_t);
            }
        }
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_GETXATTR as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else if i < 4 as uint32_t {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        } else {
            *vleng = get32bit(&raw mut rptr);
            *vbuff = if mode as ::core::ffi::c_int == MFS_XATTR_GETA_DATA {
                rptr
            } else {
                ::core::ptr::null::<uint8_t>()
            };
            if mode as ::core::ffi::c_int == MFS_XATTR_GETA_DATA
                && i != (*vleng).wrapping_add(4 as uint32_t)
                || mode as ::core::ffi::c_int == MFS_XATTR_LENGTH_ONLY && i != 4 as uint32_t
            {
                fs_disconnect();
                ret = MFS_ERROR_IO as uint8_t;
            } else {
                ret = MFS_STATUS_OK as uint8_t;
            }
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_listxattr(
    mut inode: uint32_t,
    mut opened: uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut mode: uint8_t,
    mut dbuff: *mut *const uint8_t,
    mut dleng: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut packetver: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        if master_version()
            < (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            return MFS_ERROR_ENOTSUP as uint8_t;
        }
        if master_version()
            < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = fs_createpacket(rec, CLTOMA_FUSE_GETXATTR as uint32_t, 15 as uint32_t);
            packetver = 0 as uint8_t;
        } else {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_GETXATTR as uint32_t,
                (15 as uint32_t).wrapping_add((4 as uint32_t).wrapping_mul(gids)),
            );
            packetver = 1 as uint8_t;
        }
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, inode);
        if packetver as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            put8bit(&raw mut wptr, opened);
            put32bit(&raw mut wptr, uid);
            if gids > 0 as uint32_t {
                put32bit(&raw mut wptr, *gid.offset(0 as isize));
            } else {
                put32bit(&raw mut wptr, 0xffffffff as uint32_t);
            }
        }
        put8bit(&raw mut wptr, 0 as uint8_t);
        put8bit(&raw mut wptr, mode);
        if packetver as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
            put8bit(&raw mut wptr, opened);
            put32bit(&raw mut wptr, uid);
            if gids > 0 as uint32_t {
                put32bit(&raw mut wptr, gids);
                i = 0 as uint32_t;
                while i < gids {
                    put32bit(&raw mut wptr, *gid.offset(i as isize));
                    i = i.wrapping_add(1);
                }
            } else {
                put32bit(&raw mut wptr, 0xffffffff as uint32_t);
            }
        }
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_GETXATTR as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else if i < 4 as uint32_t {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        } else {
            *dleng = get32bit(&raw mut rptr);
            *dbuff = if mode as ::core::ffi::c_int == MFS_XATTR_GETA_DATA {
                rptr
            } else {
                ::core::ptr::null::<uint8_t>()
            };
            if mode as ::core::ffi::c_int == MFS_XATTR_GETA_DATA
                && i != (*dleng).wrapping_add(4 as uint32_t)
                || mode as ::core::ffi::c_int == MFS_XATTR_LENGTH_ONLY && i != 4 as uint32_t
            {
                fs_disconnect();
                ret = MFS_ERROR_IO as uint8_t;
            } else {
                ret = MFS_STATUS_OK as uint8_t;
            }
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_setxattr(
    mut inode: uint32_t,
    mut opened: uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut nleng: uint8_t,
    mut name: *const uint8_t,
    mut vleng: uint32_t,
    mut value: *const uint8_t,
    mut mode: uint8_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut packetver: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        if master_version()
            < (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            return MFS_ERROR_ENOTSUP as uint8_t;
        }
        if mode as ::core::ffi::c_int >= MFS_XATTR_REMOVE {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if master_version()
            < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_SETXATTR as uint32_t,
                ((19 as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as uint32_t)
                    .wrapping_add(vleng),
            );
            packetver = 0 as uint8_t;
        } else {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_SETXATTR as uint32_t,
                ((19 as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as uint32_t)
                    .wrapping_add(vleng)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids)),
            );
            packetver = 1 as uint8_t;
        }
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, inode);
        if packetver as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            put8bit(&raw mut wptr, opened);
            put32bit(&raw mut wptr, uid);
            if gids > 0 as uint32_t {
                put32bit(&raw mut wptr, *gid.offset(0 as isize));
            } else {
                put32bit(&raw mut wptr, 0xffffffff as uint32_t);
            }
        }
        put8bit(&raw mut wptr, nleng);
        memcpy(
            wptr as *mut ::core::ffi::c_void,
            name as *const ::core::ffi::c_void,
            nleng as size_t,
        );
        wptr = wptr.offset(nleng as ::core::ffi::c_int as isize);
        put32bit(&raw mut wptr, vleng);
        memcpy(
            wptr as *mut ::core::ffi::c_void,
            value as *const ::core::ffi::c_void,
            vleng as size_t,
        );
        wptr = wptr.offset(vleng as isize);
        put8bit(&raw mut wptr, mode);
        if packetver as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
            put8bit(&raw mut wptr, opened);
            put32bit(&raw mut wptr, uid);
            if gids > 0 as uint32_t {
                put32bit(&raw mut wptr, gids);
                i = 0 as uint32_t;
                while i < gids {
                    put32bit(&raw mut wptr, *gid.offset(i as isize));
                    i = i.wrapping_add(1);
                }
            } else {
                put32bit(&raw mut wptr, 0xffffffff as uint32_t);
            }
        }
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_SETXATTR as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_removexattr(
    mut inode: uint32_t,
    mut opened: uint8_t,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut nleng: uint8_t,
    mut name: *const uint8_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut packetver: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        if master_version()
            < (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            return MFS_ERROR_ENOTSUP as uint8_t;
        }
        if master_version()
            < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t
        {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_SETXATTR as uint32_t,
                (19 as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as uint32_t,
            );
            packetver = 0 as uint8_t;
        } else {
            wptr = fs_createpacket(
                rec,
                CLTOMA_FUSE_SETXATTR as uint32_t,
                ((19 as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as uint32_t)
                    .wrapping_add((4 as uint32_t).wrapping_mul(gids)),
            );
            packetver = 1 as uint8_t;
        }
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        put32bit(&raw mut wptr, inode);
        if packetver as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            put8bit(&raw mut wptr, opened);
            put32bit(&raw mut wptr, uid);
            if gids > 0 as uint32_t {
                put32bit(&raw mut wptr, *gid.offset(0 as isize));
            } else {
                put32bit(&raw mut wptr, 0xffffffff as uint32_t);
            }
        }
        put8bit(&raw mut wptr, nleng);
        memcpy(
            wptr as *mut ::core::ffi::c_void,
            name as *const ::core::ffi::c_void,
            nleng as size_t,
        );
        wptr = wptr.offset(nleng as ::core::ffi::c_int as isize);
        put32bit(&raw mut wptr, 0 as uint32_t);
        put8bit(&raw mut wptr, MFS_XATTR_REMOVE as uint8_t);
        if packetver as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
            put8bit(&raw mut wptr, opened);
            put32bit(&raw mut wptr, uid);
            if gids > 0 as uint32_t {
                put32bit(&raw mut wptr, gids);
                i = 0 as uint32_t;
                while i < gids {
                    put32bit(&raw mut wptr, *gid.offset(i as isize));
                    i = i.wrapping_add(1);
                }
            } else {
                put32bit(&raw mut wptr, 0xffffffff as uint32_t);
            }
        }
        rptr = fs_sendandreceive(rec, MATOCL_FUSE_SETXATTR as uint32_t, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else if i == 1 as uint32_t {
            ret = *rptr.offset(0 as isize);
        } else {
            fs_disconnect();
            ret = MFS_ERROR_IO as uint8_t;
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fs_custom(
    mut qcmd: uint32_t,
    mut query: *const uint8_t,
    mut queryleng: uint32_t,
    mut acmd: *mut uint32_t,
    mut answer: *mut *const uint8_t,
    mut answerleng: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut ret: uint8_t = 0;
        let mut rec: *mut threc = fs_get_my_threc();
        wptr = fs_createpacket(rec, qcmd, queryleng);
        if wptr.is_null() {
            return MFS_ERROR_IO as uint8_t;
        }
        memcpy(
            wptr as *mut ::core::ffi::c_void,
            query as *const ::core::ffi::c_void,
            queryleng as size_t,
        );
        rptr = fs_sendandreceive_any(rec, acmd, &raw mut i);
        if rptr.is_null() {
            ret = MFS_ERROR_IO as uint8_t;
        } else {
            *answerleng = i;
            *answer = rptr;
            ret = MFS_STATUS_OK as uint8_t;
        }
        return ret;
    }
}
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
