#![allow(
    clippy::missing_safety_doc,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
// (macro_use removed)
extern crate c2rust_bitfields;
#[allow(unused_imports)]
use ::plfsbdev;
use ::plfsbdev::src::mfscommon::squeue::{MallocPtr, SQueue};
unsafe extern "C" {
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn putc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn __getdelim(
        __lineptr: *mut *mut ::core::ffi::c_char,
        __n: *mut size_t,
        __delimiter: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    unsafe fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    unsafe fn exit(__status: ::core::ffi::c_int) -> !;
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
    unsafe fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    unsafe fn strsep(
        __stringp: *mut *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn read(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    unsafe fn write(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ssize_t;
    unsafe fn pipe(__pipedes: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn sleep(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    unsafe fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    unsafe fn chdir(__path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn dup(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn dup2(__fd: ::core::ffi::c_int, __fd2: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn getpid() -> __pid_t;
    unsafe fn setpgid(__pid: __pid_t, __pgid: __pid_t) -> ::core::ffi::c_int;
    unsafe fn setsid() -> __pid_t;
    unsafe fn fork() -> __pid_t;
    unsafe fn symlink(
        __from: *const ::core::ffi::c_char,
        __to: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    static mut optarg: *mut ::core::ffi::c_char;
    unsafe fn getopt(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn socketpair(
        __domain: ::core::ffi::c_int,
        __type: ::core::ffi::c_int,
        __protocol: ::core::ffi::c_int,
        __fds: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    unsafe fn sigaction(
        __sig: ::core::ffi::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> ::core::ffi::c_int;
    unsafe fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut ::core::ffi::c_int,
        __options: ::core::ffi::c_int,
    ) -> __pid_t;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn mkdir(__path: *const ::core::ffi::c_char, __mode: __mode_t) -> ::core::ffi::c_int;
    unsafe fn ioctl(
        __fd: ::core::ffi::c_int,
        __request: ::core::ffi::c_ulong,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn mfs_fstat(fildes: ::core::ffi::c_int, buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn mfs_open(
        path: *const ::core::ffi::c_char,
        oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn mfs_pread(
        fildes: ::core::ffi::c_int,
        buf: *mut ::core::ffi::c_void,
        nbyte: size_t,
        offset: off_t,
    ) -> ssize_t;
    unsafe fn mfs_pwrite(
        fildes: ::core::ffi::c_int,
        buf: *const ::core::ffi::c_void,
        nbyte: size_t,
        offset: off_t,
    ) -> ssize_t;
    unsafe fn mfs_fsync(fildes: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn mfs_close(fildes: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn mfs_flock(fildes: ::core::ffi::c_int, op: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn mfs_set_defaults(mcfg_0: *mut mfscfg);
    unsafe fn mfs_init(mcfg_0: *mut mfscfg, stage: uint8_t) -> ::core::ffi::c_int;
    unsafe fn mfs_term();
    unsafe fn workers_init(
        maxworkers: uint32_t,
        sustainworkers: uint32_t,
        qleng: uint32_t,
        name: *mut ::core::ffi::c_char,
        workerfn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> ()>,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn workers_term(w: *mut ::core::ffi::c_void);
    unsafe fn workers_newjob(w: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void);
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn strerr_init();
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn unixsocket() -> ::core::ffi::c_int;
    unsafe fn unixconnect(
        sock: ::core::ffi::c_int,
        path: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn unixtoconnect(
        sock: ::core::ffi::c_int,
        path: *const ::core::ffi::c_char,
        msecto: uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn unixlisten(
        sock: ::core::ffi::c_int,
        path: *const ::core::ffi::c_char,
        queue: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn unixtoread(
        sock: ::core::ffi::c_int,
        buff: *mut ::core::ffi::c_void,
        leng: uint32_t,
        msectopart: uint32_t,
        msectoall: uint32_t,
    ) -> int32_t;
    unsafe fn unixtowrite(
        sock: ::core::ffi::c_int,
        buff: *const ::core::ffi::c_void,
        leng: uint32_t,
        msectopart: uint32_t,
        msectoall: uint32_t,
    ) -> int32_t;
    unsafe fn unixtoaccept(sock: ::core::ffi::c_int, msecto: uint32_t) -> ::core::ffi::c_int;
    unsafe fn processname_init(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char);
    unsafe fn processname_set(name: *mut ::core::ffi::c_char);
}
pub type size_t = usize;
pub type __uint32_t = u32;
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
pub type __clock_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __useconds_t = ::core::ffi::c_uint;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
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
pub type int32_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type __socket_type = ::core::ffi::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: ::core::ffi::c_int,
    pub sival_ptr: *mut ::core::ffi::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: ::core::ffi::c_int,
    pub si_errno: ::core::ffi::c_int,
    pub si_code: ::core::ffi::c_int,
    pub __pad0: ::core::ffi::c_int,
    pub _sifields: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub _pad: [::core::ffi::c_int; 28],
    pub _kill: C2Rust_Unnamed_8,
    pub _timer: C2Rust_Unnamed_7,
    pub _rt: C2Rust_Unnamed_6,
    pub _sigchld: C2Rust_Unnamed_5,
    pub _sigfault: C2Rust_Unnamed_2,
    pub _sigpoll: C2Rust_Unnamed_1,
    pub _sigsys: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub _call_addr: *mut ::core::ffi::c_void,
    pub _syscall: ::core::ffi::c_int,
    pub _arch: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub si_band: ::core::ffi::c_long,
    pub si_fd: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub si_addr: *mut ::core::ffi::c_void,
    pub si_addr_lsb: ::core::ffi::c_short,
    pub _bounds: C2Rust_Unnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub _addr_bnd: C2Rust_Unnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub _lower: *mut ::core::ffi::c_void,
    pub _upper: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: ::core::ffi::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
    pub si_tid: ::core::ffi::c_int,
    pub si_overrun: ::core::ffi::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2Rust_Unnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: ::core::ffi::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<
        unsafe extern "C" fn(::core::ffi::c_int, *mut siginfo_t, *mut ::core::ffi::c_void) -> (),
    >,
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
pub type C2Rust_Unnamed_10 = ::core::ffi::c_uint;
pub const NBD_CMD_WRITE_ZEROES: C2Rust_Unnamed_10 = 6;
pub const NBD_CMD_TRIM: C2Rust_Unnamed_10 = 4;
pub const NBD_CMD_FLUSH: C2Rust_Unnamed_10 = 3;
pub const NBD_CMD_DISC: C2Rust_Unnamed_10 = 2;
pub const NBD_CMD_WRITE: C2Rust_Unnamed_10 = 1;
pub const NBD_CMD_READ: C2Rust_Unnamed_10 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _mfscfg {
    pub masterhost: *mut ::core::ffi::c_char,
    pub masterport: *mut ::core::ffi::c_char,
    pub masterbind: *mut ::core::ffi::c_char,
    pub masterpassword: *mut ::core::ffi::c_char,
    pub mastermd5pass: *mut ::core::ffi::c_char,
    pub mountpoint: *mut ::core::ffi::c_char,
    pub masterpath: *mut ::core::ffi::c_char,
    pub preferedlabels: *mut ::core::ffi::c_char,
    pub read_cache_mb: ::core::ffi::c_int,
    pub write_cache_mb: ::core::ffi::c_int,
    pub io_try_cnt: ::core::ffi::c_int,
    pub io_timeout: ::core::ffi::c_int,
    pub min_log_entry: ::core::ffi::c_int,
    pub readahead_leng: ::core::ffi::c_int,
    pub readahead_trigger: ::core::ffi::c_int,
    pub error_on_lost_chunk: ::core::ffi::c_int,
    pub error_on_no_space: ::core::ffi::c_int,
    pub sugid_clear_mode: ::core::ffi::c_int,
    pub mkdir_copy_sgid: ::core::ffi::c_int,
    pub lcache_retention: ::core::ffi::c_double,
    pub logident: *mut ::core::ffi::c_char,
    pub logdaemon: ::core::ffi::c_int,
    pub logminlevel: ::core::ffi::c_int,
    pub logelevateto: ::core::ffi::c_int,
    pub master_min_version_maj: uint16_t,
    pub master_min_version_mid: uint16_t,
}
pub type mfscfg = _mfscfg;
pub type C2Rust_Unnamed_11 = ::core::ffi::c_uint;
pub const MFSNBD_RESIZE: C2Rust_Unnamed_11 = 5;
pub const MFSNBD_LIST: C2Rust_Unnamed_11 = 4;
pub const MFSNBD_REMOVE: C2Rust_Unnamed_11 = 3;
pub const MFSNBD_ADD: C2Rust_Unnamed_11 = 2;
pub const MFSNBD_STOP: C2Rust_Unnamed_11 = 1;
pub const MFSNBD_NOP: C2Rust_Unnamed_11 = 0;
pub type C2Rust_Unnamed_12 = ::core::ffi::c_uint;
pub const MFSNBD_ERROR: C2Rust_Unnamed_12 = 1;
pub const MFSNBD_OK: C2Rust_Unnamed_12 = 0;
#[repr(C)]
pub struct nbdcommon {
    pub linkname: *mut ::core::ffi::c_char,
    pub nbddevice: *mut ::core::ffi::c_char,
    pub mfsfile: *mut ::core::ffi::c_char,
    pub fsize: uint64_t,
    pub bsize: uint32_t,
    pub flags: uint32_t,
    pub sp: [::core::ffi::c_int; 2],
    pub mfsfd: ::core::ffi::c_int,
    pub nbdfd: ::core::ffi::c_int,
    pub ctrl_thread: Option<std::thread::JoinHandle<()>>,
    pub active: ::core::ffi::c_int,
    pub aqueue: *mut SQueue<MallocPtr<nbdrequest>>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nbdrequest {
    pub nbdcp: *mut nbdcommon,
    pub handle: [uint8_t; 8],
    pub offset: uint64_t,
    pub length: uint32_t,
    pub cmd: uint16_t,
    pub cmdflags: uint16_t,
    pub status: uint32_t,
    pub data: [uint8_t; 1],
}
/// Sendable raw nbdcommon pointer for thread spawns (NBD threads share the
/// session struct with the controller; lifetime managed by nbd_start/nbd_stop).
#[derive(Clone, Copy)]
struct NbdcPtr(*mut nbdcommon);
// SAFETY: threads only access the shared session under its existing
// synchronization (aqueue SQueue, active flag protocol), exactly as pthreads did.
unsafe impl Send for NbdcPtr {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _bdlist {
    pub nbdcp: *mut nbdcommon,
    pub next: *mut _bdlist,
}
pub type bdlist = _bdlist;
#[inline]
unsafe extern "C" fn putchar(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        return putc(__c, stdout);
    }
}
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut ::core::ffi::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    unsafe {
        return __getdelim(__lineptr, __n, '\n' as ::core::ffi::c_int, __stream);
    }
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const STDIN_FILENO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STDERR_FILENO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_LOCAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PF_UNIX: ::core::ffi::c_int = PF_LOCAL;
pub const AF_UNIX: ::core::ffi::c_int = PF_UNIX;
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SIGPIPE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SA_RESTART: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const _IOC_NRBITS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const _IOC_TYPEBITS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const _IOC_SIZEBITS: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const _IOC_NRSHIFT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const _IOC_TYPESHIFT: ::core::ffi::c_int = _IOC_NRSHIFT + _IOC_NRBITS;
pub const _IOC_SIZESHIFT: ::core::ffi::c_int = _IOC_TYPESHIFT + _IOC_TYPEBITS;
pub const _IOC_DIRSHIFT: ::core::ffi::c_int = _IOC_SIZESHIFT + _IOC_SIZEBITS;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const LOCK_SH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LOCK_EX: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LOCK_UN: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const LOCK_NB: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const EROFS: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const EOVERFLOW: ::core::ffi::c_int = 75 as ::core::ffi::c_int;
pub const EADDRINUSE: ::core::ffi::c_int = 98 as ::core::ffi::c_int;
pub const ETIMEDOUT: ::core::ffi::c_int = 110 as ::core::ffi::c_int;
pub const ECONNREFUSED: ::core::ffi::c_int = 111 as ::core::ffi::c_int;
pub const NBD_SET_SOCK: ::core::ffi::c_uint = (0 as ::core::ffi::c_uint) << _IOC_DIRSHIFT
    | ((0xab as ::core::ffi::c_int) << _IOC_TYPESHIFT) as ::core::ffi::c_uint
    | ((0 as ::core::ffi::c_int) << _IOC_NRSHIFT) as ::core::ffi::c_uint
    | ((0 as ::core::ffi::c_int) << _IOC_SIZESHIFT) as ::core::ffi::c_uint;
pub const NBD_SET_BLKSIZE: ::core::ffi::c_uint = (0 as ::core::ffi::c_uint) << _IOC_DIRSHIFT
    | ((0xab as ::core::ffi::c_int) << _IOC_TYPESHIFT) as ::core::ffi::c_uint
    | ((1 as ::core::ffi::c_int) << _IOC_NRSHIFT) as ::core::ffi::c_uint
    | ((0 as ::core::ffi::c_int) << _IOC_SIZESHIFT) as ::core::ffi::c_uint;
pub const NBD_DO_IT: ::core::ffi::c_uint = (0 as ::core::ffi::c_uint) << _IOC_DIRSHIFT
    | ((0xab as ::core::ffi::c_int) << _IOC_TYPESHIFT) as ::core::ffi::c_uint
    | ((3 as ::core::ffi::c_int) << _IOC_NRSHIFT) as ::core::ffi::c_uint
    | ((0 as ::core::ffi::c_int) << _IOC_SIZESHIFT) as ::core::ffi::c_uint;
pub const NBD_CLEAR_SOCK: ::core::ffi::c_uint = (0 as ::core::ffi::c_uint) << _IOC_DIRSHIFT
    | ((0xab as ::core::ffi::c_int) << _IOC_TYPESHIFT) as ::core::ffi::c_uint
    | ((4 as ::core::ffi::c_int) << _IOC_NRSHIFT) as ::core::ffi::c_uint
    | ((0 as ::core::ffi::c_int) << _IOC_SIZESHIFT) as ::core::ffi::c_uint;
pub const NBD_CLEAR_QUE: ::core::ffi::c_uint = (0 as ::core::ffi::c_uint) << _IOC_DIRSHIFT
    | ((0xab as ::core::ffi::c_int) << _IOC_TYPESHIFT) as ::core::ffi::c_uint
    | ((5 as ::core::ffi::c_int) << _IOC_NRSHIFT) as ::core::ffi::c_uint
    | ((0 as ::core::ffi::c_int) << _IOC_SIZESHIFT) as ::core::ffi::c_uint;
pub const NBD_SET_SIZE_BLOCKS: ::core::ffi::c_uint = (0 as ::core::ffi::c_uint) << _IOC_DIRSHIFT
    | ((0xab as ::core::ffi::c_int) << _IOC_TYPESHIFT) as ::core::ffi::c_uint
    | ((7 as ::core::ffi::c_int) << _IOC_NRSHIFT) as ::core::ffi::c_uint
    | ((0 as ::core::ffi::c_int) << _IOC_SIZESHIFT) as ::core::ffi::c_uint;
pub const NBD_DISCONNECT: ::core::ffi::c_uint = (0 as ::core::ffi::c_uint) << _IOC_DIRSHIFT
    | ((0xab as ::core::ffi::c_int) << _IOC_TYPESHIFT) as ::core::ffi::c_uint
    | ((8 as ::core::ffi::c_int) << _IOC_NRSHIFT) as ::core::ffi::c_uint
    | ((0 as ::core::ffi::c_int) << _IOC_SIZESHIFT) as ::core::ffi::c_uint;
pub const NBD_SET_TIMEOUT: ::core::ffi::c_uint = (0 as ::core::ffi::c_uint) << _IOC_DIRSHIFT
    | ((0xab as ::core::ffi::c_int) << _IOC_TYPESHIFT) as ::core::ffi::c_uint
    | ((9 as ::core::ffi::c_int) << _IOC_NRSHIFT) as ::core::ffi::c_uint
    | ((0 as ::core::ffi::c_int) << _IOC_SIZESHIFT) as ::core::ffi::c_uint;
pub const NBD_REQUEST_MAGIC: ::core::ffi::c_int = 0x25609513 as ::core::ffi::c_int;
pub const NBD_REPLY_MAGIC: ::core::ffi::c_int = 0x67446698 as ::core::ffi::c_int;
pub const BLKGETSIZE64: usize = ((2 as ::core::ffi::c_uint) << _IOC_DIRSHIFT
    | ((0x12 as ::core::ffi::c_int) << _IOC_TYPESHIFT) as ::core::ffi::c_uint
    | ((114 as ::core::ffi::c_int) << _IOC_NRSHIFT) as ::core::ffi::c_uint)
    as usize
    | ::core::mem::size_of::<size_t>() << _IOC_SIZESHIFT;
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
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn sizestrtod(
    mut str: *const ::core::ffi::c_char,
    mut endptr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_double {
    unsafe {
        let mut val: ::core::ffi::c_double = 0.;
        let mut frac: ::core::ffi::c_double = 0.;
        let mut f: ::core::ffi::c_int = 0;
        val = 0.0f64;
        f = 0 as ::core::ffi::c_int;
        while *str as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
            && *str as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
        {
            val *= 10.0f64;
            val +=
                (*str as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as ::core::ffi::c_double;
            str = str.offset(1);
            f = 1 as ::core::ffi::c_int;
        }
        if *str as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            && *str.offset(1 as isize) as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
            && *str.offset(1 as isize) as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
        {
            f = 1 as ::core::ffi::c_int;
            str = str.offset(1);
            frac = 1.0f64;
            while *str as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                && *str as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
            {
                frac /= 10.0f64;
                val += (*str as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                    as ::core::ffi::c_double
                    * frac;
                str = str.offset(1);
            }
        }
        if f != 0 {
            match *str as ::core::ffi::c_int {
                107 => {
                    str = str.offset(1);
                    val *= 1e3f64;
                }
                75 => {
                    if *str.offset(1 as isize) as ::core::ffi::c_int == 'i' as ::core::ffi::c_int {
                        str = str.offset(2 as ::core::ffi::c_int as isize);
                        val *= 1024.0f64;
                    }
                }
                77 => {
                    str = str.offset(1);
                    if *str as ::core::ffi::c_int == 'i' as ::core::ffi::c_int {
                        str = str.offset(1);
                        val *= 1048576.0f64;
                    } else {
                        val *= 1e6f64;
                    }
                }
                71 => {
                    str = str.offset(1);
                    if *str as ::core::ffi::c_int == 'i' as ::core::ffi::c_int {
                        str = str.offset(1);
                        val *= 1073741824.0f64;
                    } else {
                        val *= 1e9f64;
                    }
                }
                84 => {
                    str = str.offset(1);
                    if *str as ::core::ffi::c_int == 'i' as ::core::ffi::c_int {
                        str = str.offset(1);
                        val *= 1099511627776.0f64;
                    } else {
                        val *= 1e12f64;
                    }
                }
                80 => {
                    str = str.offset(1);
                    if *str as ::core::ffi::c_int == 'i' as ::core::ffi::c_int {
                        str = str.offset(1);
                        val *= 1125899906842624.0f64;
                    } else {
                        val *= 1e15f64;
                    }
                }
                69 => {
                    str = str.offset(1);
                    if *str as ::core::ffi::c_int == 'i' as ::core::ffi::c_int {
                        str = str.offset(1);
                        val *= 1152921504606846976.0f64;
                    } else {
                        val *= 1e18f64;
                    }
                }
                _ => {}
            }
        }
        if !endptr.is_null() {
            *endptr = str;
        }
        return val;
    }
}
pub const READ_TOMS: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const NBD_LINK_PREFIX: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"/dev/mfs/\0") };
pub const NBD_LINK_PREFIX_LENG: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const NBD_ERR_SIZE: ::core::ffi::c_int = 200 as ::core::ffi::c_int;
static mut NbdTimeout: ::core::ffi::c_int = 1800 as ::core::ffi::c_int;
pub const FLAG_READONLY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FLAG_IGNORELOCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut workers_set: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut bdhead: *mut bdlist = ::core::ptr::null_mut::<bdlist>();
static mut mcfg: mfscfg = mfscfg {
    masterhost: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    masterport: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    masterbind: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    masterpassword: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    mastermd5pass: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    mountpoint: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    masterpath: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    preferedlabels: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    read_cache_mb: 0,
    write_cache_mb: 0,
    io_try_cnt: 0,
    io_timeout: 0,
    min_log_entry: 0,
    readahead_leng: 0,
    readahead_trigger: 0,
    error_on_lost_chunk: 0,
    error_on_no_space: 0,
    sugid_clear_mode: 0,
    mkdir_copy_sgid: 0,
    lcache_retention: 0.,
    logident: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    logdaemon: 0,
    logminlevel: 0,
    logelevateto: 0,
    master_min_version_maj: 0,
    master_min_version_mid: 0,
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn writeall(
    mut sock: ::core::ffi::c_int,
    mut buff: *mut uint8_t,
    mut leng: uint32_t,
) -> int32_t {
    unsafe {
        let mut bsent: uint32_t = 0;
        let mut res: ::core::ffi::c_int = 0;
        bsent = 0 as uint32_t;
        while bsent < leng {
            res = write(
                sock,
                buff.offset(bsent as isize) as *const ::core::ffi::c_void,
                leng.wrapping_sub(bsent) as size_t,
            ) as ::core::ffi::c_int;
            if res < 0 as ::core::ffi::c_int {
                return -1 as int32_t;
            }
            bsent = bsent.wrapping_add(res as uint32_t);
        }
        return bsent as int32_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn readall(
    mut sock: ::core::ffi::c_int,
    mut buf: *mut uint8_t,
    mut leng: uint32_t,
) -> int32_t {
    unsafe {
        let mut brecv: uint32_t = 0;
        let mut res: ::core::ffi::c_int = 0;
        brecv = 0 as uint32_t;
        while brecv < leng {
            res = read(
                sock,
                buf.offset(brecv as isize) as *mut ::core::ffi::c_void,
                leng.wrapping_sub(brecv) as size_t,
            ) as ::core::ffi::c_int;
            if res <= 0 as ::core::ffi::c_int {
                return -1 as int32_t;
            }
            brecv = brecv.wrapping_add(res as uint32_t);
        }
        return brecv as int32_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn skipall(mut sock: ::core::ffi::c_int, mut leng: uint32_t) -> int32_t {
    unsafe {
        static mut skipbuff: [uint8_t; 16384] = [0; 16384];
        let mut brecv: uint32_t = 0;
        let mut res: ::core::ffi::c_int = 0;
        brecv = 0 as uint32_t;
        while brecv < leng {
            if leng.wrapping_sub(brecv) > 16384 as uint32_t {
                res = read(
                    sock,
                    &raw mut skipbuff as *mut uint8_t as *mut ::core::ffi::c_void,
                    16384 as size_t,
                ) as ::core::ffi::c_int;
            } else {
                res = read(
                    sock,
                    &raw mut skipbuff as *mut uint8_t as *mut ::core::ffi::c_void,
                    leng.wrapping_sub(brecv) as size_t,
                ) as ::core::ffi::c_int;
            }
            if res <= 0 as ::core::ffi::c_int {
                return -1 as int32_t;
            }
            brecv = brecv.wrapping_add(res as uint32_t);
        }
        return brecv as int32_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nbd_worker_fn(mut data: *mut ::core::ffi::c_void, _workerscnt: uint32_t) {
    unsafe {
        let mut r: *mut nbdrequest = data as *mut nbdrequest;
        let mut nbdcp: *mut nbdcommon = (*r).nbdcp;
        match (*r).cmd as ::core::ffi::c_int {
            0 => {
                if (*r).offset.wrapping_add((*r).length as uint64_t) > (*nbdcp).fsize {
                    (*r).status = EOVERFLOW as uint32_t;
                } else if mfs_pread(
                    (*nbdcp).mfsfd,
                    &raw mut (*r).data as *mut uint8_t as *mut ::core::ffi::c_void,
                    (*r).length as size_t,
                    (*r).offset as off_t,
                ) < 0 as ssize_t
                {
                    (*r).status = *__errno_location() as uint32_t;
                } else {
                    (*r).status = *__errno_location() as uint32_t;
                }
            }
            1 => {
                if (*nbdcp).flags & FLAG_READONLY as uint32_t != 0 {
                    (*r).status = EROFS as uint32_t;
                } else if (*r).offset.wrapping_add((*r).length as uint64_t) > (*nbdcp).fsize {
                    (*r).status = EOVERFLOW as uint32_t;
                } else if mfs_pwrite(
                    (*nbdcp).mfsfd,
                    &raw mut (*r).data as *mut uint8_t as *const ::core::ffi::c_void,
                    (*r).length as size_t,
                    (*r).offset as off_t,
                ) < 0 as ssize_t
                {
                    (*r).status = *__errno_location() as uint32_t;
                } else {
                    (*r).status = 0 as uint32_t;
                }
            }
            2 => {
                mfs_fsync((*nbdcp).mfsfd);
                (*r).status = 0 as uint32_t;
            }
            _ => {
                (*r).status = 0 as uint32_t;
            }
        }
        // aqueue is unbounded (created with max 0), so put never fails.
        let _ = (*(*nbdcp).aqueue).put(MallocPtr::from_raw(r));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn receive_thread(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut commbuff: [uint8_t; 28] = [0; 28];
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut handleptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut bytesread: uint32_t = 0;
        let mut magic: uint32_t = 0;
        let mut cmdflags: uint16_t = 0;
        let mut cmd: uint32_t = 0;
        let mut offset: uint64_t = 0;
        let mut length: uint32_t = 0;
        let mut res: ::core::ffi::c_int = 0;
        let mut r: *mut nbdrequest = ::core::ptr::null_mut::<nbdrequest>();
        let mut nbdcp: *mut nbdcommon = arg as *mut nbdcommon;
        bytesread = 0 as uint32_t;
        loop {
            res = read(
                (*nbdcp).sp[0 as usize],
                (&raw mut commbuff as *mut uint8_t).offset(bytesread as isize)
                    as *mut ::core::ffi::c_void,
                (28 as uint32_t).wrapping_sub(bytesread) as size_t,
            ) as ::core::ffi::c_int;
            if res <= 0 as ::core::ffi::c_int {
                wptr = &raw mut commbuff as *mut uint8_t;
                put32bit(&raw mut wptr, NBD_REQUEST_MAGIC as uint32_t);
                put32bit(
                    &raw mut wptr,
                    NBD_CMD_DISC as ::core::ffi::c_int as uint32_t,
                );
                memset(
                    wptr as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    20 as size_t,
                );
                bytesread = 28 as uint32_t;
            } else {
                bytesread = bytesread.wrapping_add(res as uint32_t);
            }
            if bytesread < 28 as uint32_t {
                continue;
            }
            bytesread = 0 as uint32_t;
            rptr = &raw mut commbuff as *mut uint8_t;
            magic = get32bit(&raw mut rptr);
            cmdflags = get16bit(&raw mut rptr);
            cmd = get16bit(&raw mut rptr) as uint32_t;
            handleptr = rptr;
            rptr = rptr.offset(8 as ::core::ffi::c_int as isize);
            offset = get64bit(&raw mut rptr);
            length = get32bit(&raw mut rptr);
            if magic != NBD_REQUEST_MAGIC as uint32_t {
                cmd = NBD_CMD_DISC as ::core::ffi::c_int as uint32_t;
            }
            if cmd == NBD_CMD_WRITE as ::core::ffi::c_int as uint32_t
                || cmd == NBD_CMD_READ as ::core::ffi::c_int as uint32_t
            {
                r = malloc((36 as size_t).wrapping_add(length as size_t)) as *mut nbdrequest;
            } else {
                r = malloc(::core::mem::size_of::<nbdrequest>()) as *mut nbdrequest;
            }
            if r.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    348 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"r\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    348 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"r\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if r
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut nbdrequest
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    348 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"r\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    348 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"r\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            (*r).nbdcp = nbdcp;
            memcpy(
                &raw mut (*r).handle as *mut uint8_t as *mut ::core::ffi::c_void,
                handleptr as *const ::core::ffi::c_void,
                8 as size_t,
            );
            (*r).offset = offset;
            (*r).length = length;
            (*r).cmd = cmd as uint16_t;
            (*r).cmdflags = cmdflags;
            if cmd == NBD_CMD_WRITE as ::core::ffi::c_int as uint32_t {
                readall(
                    (*nbdcp).sp[0 as usize],
                    &raw mut (*r).data as *mut uint8_t,
                    length,
                );
            }
            workers_newjob(workers_set, r as *mut ::core::ffi::c_void);
            if cmd == NBD_CMD_DISC as ::core::ffi::c_int as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_INFO,
                    b"receive thread for %s ending (cmd:DISC)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*nbdcp).nbddevice,
                );
                return NULL;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn send_thread(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut commbuff: [uint8_t; 16] = [0; 16];
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut r: *mut nbdrequest = ::core::ptr::null_mut::<nbdrequest>();
        let mut data: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut nbdcp: *mut nbdcommon = arg as *mut nbdcommon;
        wptr = &raw mut commbuff as *mut uint8_t;
        put32bit(&raw mut wptr, NBD_REPLY_MAGIC as uint32_t);
        loop {
            data = (*(*nbdcp).aqueue)
                .get()
                .map_or(::core::ptr::null_mut(), MallocPtr::into_raw)
                as *mut ::core::ffi::c_void;
            if data.is_null() {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"send thread for %s ending (data==NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*nbdcp).nbddevice,
                );
                return NULL;
            }
            r = data as *mut nbdrequest;
            wptr = (&raw mut commbuff as *mut uint8_t).offset(4 as ::core::ffi::c_int as isize);
            put32bit(&raw mut wptr, (*r).status);
            memcpy(
                wptr as *mut ::core::ffi::c_void,
                &raw mut (*r).handle as *mut uint8_t as *const ::core::ffi::c_void,
                8 as size_t,
            );
            writeall(
                (*nbdcp).sp[0 as usize],
                &raw mut commbuff as *mut uint8_t,
                16 as uint32_t,
            );
            if (*r).cmd as ::core::ffi::c_int == NBD_CMD_READ as ::core::ffi::c_int
                && (*r).status == 0 as uint32_t
            {
                writeall(
                    (*nbdcp).sp[0 as usize],
                    &raw mut (*r).data as *mut uint8_t,
                    (*r).length,
                );
            }
            if (*r).cmd as ::core::ffi::c_int == NBD_CMD_DISC as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_INFO,
                    b"send thread for %s ending (cmd:DISC)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*nbdcp).nbddevice,
                );
                return NULL;
            }
            free(r as *mut ::core::ffi::c_void);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nbd_open_device(
    mut nbdcp: *mut nbdcommon,
    mut errmsg: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut size: uint64_t = 0;
        let mut err: ::core::ffi::c_int = 0;
        (*nbdcp).nbdfd = open((*nbdcp).nbddevice, O_RDWR);
        if (*nbdcp).nbdfd < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"error opening %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                (*nbdcp).nbddevice,
                strerror(*__errno_location()),
            );
            if !errmsg.is_null() {
                snprintf(
                    errmsg as *mut ::core::ffi::c_char,
                    NBD_ERR_SIZE as size_t,
                    b"error opening %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    (*nbdcp).nbddevice,
                    strerror(*__errno_location()),
                );
            }
            return -1 as ::core::ffi::c_int;
        }
        err = ioctl(
            (*nbdcp).nbdfd,
            BLKGETSIZE64 as ::core::ffi::c_ulong,
            &raw mut size,
        );
        if err < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"can't obtain size of block device (%s): %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*nbdcp).nbddevice,
                strerror(*__errno_location()),
            );
            if !errmsg.is_null() {
                snprintf(
                    errmsg as *mut ::core::ffi::c_char,
                    NBD_ERR_SIZE as size_t,
                    b"can't obtain size of block device (%s): %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*nbdcp).nbddevice,
                    strerror(*__errno_location()),
                );
            }
            close((*nbdcp).nbdfd);
            return -1 as ::core::ffi::c_int;
        }
        if size > 0 as uint64_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"it seems that block device (%s) is already mapped\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*nbdcp).nbddevice,
            );
            if !errmsg.is_null() {
                snprintf(
                    errmsg as *mut ::core::ffi::c_char,
                    NBD_ERR_SIZE as size_t,
                    b"it seems that block device (%s) is already mapped\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*nbdcp).nbddevice,
                );
            }
            close((*nbdcp).nbdfd);
            return -1 as ::core::ffi::c_int;
        }
        err = ioctl(
            (*nbdcp).nbdfd,
            NBD_SET_BLKSIZE as ::core::ffi::c_ulong,
            (*nbdcp).bsize,
        );
        if err < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"error setting block device block size (%s): %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*nbdcp).nbddevice,
                strerror(*__errno_location()),
            );
            if !errmsg.is_null() {
                snprintf(
                    errmsg as *mut ::core::ffi::c_char,
                    NBD_ERR_SIZE as size_t,
                    b"error setting block device block size (%s): %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*nbdcp).nbddevice,
                    strerror(*__errno_location()),
                );
            }
            close((*nbdcp).nbdfd);
            return -1 as ::core::ffi::c_int;
        }
        err = ioctl(
            (*nbdcp).nbdfd,
            NBD_SET_SIZE_BLOCKS as ::core::ffi::c_ulong,
            (*nbdcp).fsize.wrapping_div((*nbdcp).bsize as uint64_t),
        );
        if err < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"error setting block device number of blocks (%s): %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*nbdcp).nbddevice,
                strerror(*__errno_location()),
            );
            if !errmsg.is_null() {
                snprintf(
                    errmsg as *mut ::core::ffi::c_char,
                    NBD_ERR_SIZE as size_t,
                    b"error setting block device number of blocks (%s): %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*nbdcp).nbddevice,
                    strerror(*__errno_location()),
                );
            }
            close((*nbdcp).nbdfd);
            return -1 as ::core::ffi::c_int;
        }
        err = ioctl((*nbdcp).nbdfd, NBD_CLEAR_SOCK as ::core::ffi::c_ulong);
        if err < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"error clearing socket for NBD device (%s): %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*nbdcp).nbddevice,
                strerror(*__errno_location()),
            );
            if !errmsg.is_null() {
                snprintf(
                    errmsg as *mut ::core::ffi::c_char,
                    NBD_ERR_SIZE as size_t,
                    b"error clearing socket for NBD device (%s): %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*nbdcp).nbddevice,
                    strerror(*__errno_location()),
                );
            }
            close((*nbdcp).nbdfd);
            return -1 as ::core::ffi::c_int;
        }
        err = ioctl(
            (*nbdcp).nbdfd,
            NBD_SET_TIMEOUT as ::core::ffi::c_ulong,
            NbdTimeout,
        );
        if err < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"error setting timeout for NBD device (%s): %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*nbdcp).nbddevice,
                strerror(*__errno_location()),
            );
        }
        err = socketpair(
            AF_UNIX,
            SOCK_STREAM as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw mut (*nbdcp).sp as *mut ::core::ffi::c_int,
        );
        if err < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"can't create socket pair: %s\0".as_ptr() as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            if !errmsg.is_null() {
                snprintf(
                    errmsg as *mut ::core::ffi::c_char,
                    NBD_ERR_SIZE as size_t,
                    b"can't create socket pair: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    strerror(*__errno_location()),
                );
            }
            close((*nbdcp).nbdfd);
            return -1 as ::core::ffi::c_int;
        }
        err = ioctl(
            (*nbdcp).nbdfd,
            NBD_SET_SOCK as ::core::ffi::c_ulong,
            (*nbdcp).sp[1 as usize],
        );
        if err < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"can't connect socket pair to nbd (%s): %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*nbdcp).nbddevice,
                strerror(*__errno_location()),
            );
            if !errmsg.is_null() {
                snprintf(
                    errmsg as *mut ::core::ffi::c_char,
                    NBD_ERR_SIZE as size_t,
                    b"can't connect socket pair to nbd (%s): %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*nbdcp).nbddevice,
                    strerror(*__errno_location()),
                );
            }
            close((*nbdcp).nbdfd);
            close((*nbdcp).sp[0 as usize]);
            close((*nbdcp).sp[1 as usize]);
            return -1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nbd_controller_thread(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut nbdcp: *mut nbdcommon = arg as *mut nbdcommon;
        let mut recv_th: Option<std::thread::JoinHandle<()>> = None;
        let mut send_th: Option<std::thread::JoinHandle<()>> = None;
        let mut thflags: uint8_t = 0;
        thflags = 0 as uint8_t;
        while (*nbdcp).active != 0 {
            while (*nbdcp).active != 0 {
                // C: lwt_minthread_create(joinable, send_thread); spawn_min
                // keeps the same stack clamp + blocked daemon signal mask.
                let ptr = NbdcPtr(nbdcp);
                match plfscommon::lwthread::spawn_min("nbdsend", move || {
                    // Bind whole wrapper: edition-2024 closures otherwise
                    // capture only the ptr.0 field (not Send).
                    let ptr = ptr;
                    unsafe {
                        send_thread(ptr.0 as *mut ::core::ffi::c_void);
                    }
                }) {
                    Err(_) => {
                        sleep(1 as ::core::ffi::c_uint);
                    }
                    Ok(handle) => {
                        send_th = Some(handle);
                        thflags =
                            (thflags as ::core::ffi::c_int | 1 as ::core::ffi::c_int) as uint8_t;
                        break;
                    }
                }
            }
            while (*nbdcp).active != 0 {
                let ptr = NbdcPtr(nbdcp);
                match plfscommon::lwthread::spawn_min("nbdrecv", move || {
                    let ptr = ptr;
                    unsafe {
                        receive_thread(ptr.0 as *mut ::core::ffi::c_void);
                    }
                }) {
                    Err(_) => {
                        sleep(1 as ::core::ffi::c_uint);
                    }
                    Ok(handle) => {
                        recv_th = Some(handle);
                        thflags =
                            (thflags as ::core::ffi::c_int | 2 as ::core::ffi::c_int) as uint8_t;
                        break;
                    }
                }
            }
            if (*nbdcp).active != 0 {
                ioctl((*nbdcp).nbdfd, NBD_DO_IT as ::core::ffi::c_ulong);
                if (*nbdcp).active != 0 {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_INFO,
                        b"controller thread for %s: disconnected\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*nbdcp).nbddevice,
                    );
                }
            }
            ioctl((*nbdcp).nbdfd, NBD_CLEAR_QUE as ::core::ffi::c_ulong);
            ioctl((*nbdcp).nbdfd, NBD_DISCONNECT as ::core::ffi::c_ulong);
            ioctl((*nbdcp).nbdfd, NBD_CLEAR_SOCK as ::core::ffi::c_ulong);
            close((*nbdcp).sp[0 as usize]);
            close((*nbdcp).sp[1 as usize]);
            if thflags as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0 {
                if let Some(handle) = recv_th.take() {
                    let _ = handle.join();
                }
            }
            if thflags as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                if let Some(handle) = send_th.take() {
                    let _ = handle.join();
                }
            }
            thflags = 0 as uint8_t;
            close((*nbdcp).nbdfd);
            while (*nbdcp).active != 0 {
                if nbd_open_device(nbdcp, ::core::ptr::null_mut::<::core::ffi::c_char>())
                    < 0 as ::core::ffi::c_int
                {
                    sleep(1 as ::core::ffi::c_uint);
                } else {
                    thflags = (thflags as ::core::ffi::c_int | 4 as ::core::ffi::c_int) as uint8_t;
                    break;
                }
            }
        }
        if thflags as ::core::ffi::c_int & 4 as ::core::ffi::c_int != 0 {
            ioctl((*nbdcp).nbdfd, NBD_CLEAR_QUE as ::core::ffi::c_ulong);
            ioctl((*nbdcp).nbdfd, NBD_CLEAR_SOCK as ::core::ffi::c_ulong);
            close((*nbdcp).sp[0 as usize]);
            close((*nbdcp).sp[1 as usize]);
            close((*nbdcp).nbdfd);
        }
        return NULL;
    }
}
static mut term: uint8_t = 0;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn termhandle(_signo: ::core::ffi::c_int) {
    unsafe {
        term = 1 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_signals() {
    unsafe {
        let mut sa: sigaction = sigaction {
            __sigaction_handler: C2Rust_Unnamed_9 { sa_handler: None },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        sa.sa_flags = SA_RESTART;
        sigemptyset(&raw mut sa.sa_mask);
        sa.__sigaction_handler.sa_handler =
            Some(termhandle as unsafe extern "C" fn(::core::ffi::c_int) -> ()) as __sighandler_t;
        sigaction(SIGTERM, &raw mut sa, ::core::ptr::null_mut::<sigaction>());
        sigaction(SIGINT, &raw mut sa, ::core::ptr::null_mut::<sigaction>());
        sa.__sigaction_handler.sa_handler = ::core::mem::transmute::<
            ::libc::intptr_t,
            __sighandler_t,
        >(1 as ::core::ffi::c_int as ::libc::intptr_t);
        sigaction(SIGPIPE, &raw mut sa, ::core::ptr::null_mut::<sigaction>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_daemon() {
    unsafe {
        let mut f: ::core::ffi::c_int = 0;
        let mut pipefd: [::core::ffi::c_int; 2] = [0; 2];
        fflush(stdout);
        fflush(stderr);
        if pipe(&raw mut pipefd as *mut ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"daemonize, pipe error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            exit(1 as ::core::ffi::c_int);
        }
        f = fork() as ::core::ffi::c_int;
        if f < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"daemonize, first fork error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            exit(1 as ::core::ffi::c_int);
        }
        if f > 0 as ::core::ffi::c_int {
            let mut status: ::core::ffi::c_int = 0;
            let mut buf: ::core::ffi::c_char = 0;
            close(pipefd[1 as usize]);
            while read(
                pipefd[0 as usize],
                &raw mut buf as *mut ::core::ffi::c_void,
                1 as size_t,
            ) > 0 as ssize_t
            {
                status = write(
                    STDOUT_FILENO,
                    &raw mut buf as *const ::core::ffi::c_void,
                    1 as size_t,
                ) as ::core::ffi::c_int;
            }
            waitpid(f as __pid_t, &raw mut status, 0 as ::core::ffi::c_int);
            exit(0 as ::core::ffi::c_int);
        }
        if chdir(b"/\0".as_ptr() as *const ::core::ffi::c_char) < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"can't change working directory to '/': %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
        }
        setsid();
        setpgid(0 as __pid_t, getpid());
        f = fork() as ::core::ffi::c_int;
        if f < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"daemonize, second fork error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
            exit(1 as ::core::ffi::c_int);
        }
        if f > 0 as ::core::ffi::c_int {
            close(pipefd[0 as usize]);
            close(pipefd[1 as usize]);
            exit(0 as ::core::ffi::c_int);
        }
        set_signals();
        close(pipefd[0 as usize]);
        f = open(
            b"/dev/null\0".as_ptr() as *const ::core::ffi::c_char,
            O_RDWR,
            0 as ::core::ffi::c_int,
        );
        close(STDIN_FILENO);
        if dup(f) == 0 as ::core::ffi::c_int {
        } else {
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr() as *const ::core::ffi::c_char,
                633 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dup(f)==STDIN_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr() as *const ::core::ffi::c_char,
                633 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dup(f)==STDIN_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        close(STDOUT_FILENO);
        if dup(f) == 1 as ::core::ffi::c_int {
        } else {
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr() as *const ::core::ffi::c_char,
                635 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dup(f)==STDOUT_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr() as *const ::core::ffi::c_char,
                635 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dup(f)==STDOUT_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        close(STDERR_FILENO);
        if dup(pipefd[1 as usize]) == 2 as ::core::ffi::c_int {
        } else {
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr() as *const ::core::ffi::c_char,
                637 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dup(pipefd[1])==STDERR_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr() as *const ::core::ffi::c_char,
                637 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dup(pipefd[1])==STDERR_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        close(f);
        close(pipefd[1 as usize]);
    }
}
#[inline]
unsafe extern "C" fn charconv(mut c: ::core::ffi::c_char) -> ::core::ffi::c_char {
    if c as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
        && c as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
        || c as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
            && c as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
        || c as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
            && c as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int
        || c as ::core::ffi::c_int == '.' as ::core::ffi::c_int
        || c as ::core::ffi::c_int == '-' as ::core::ffi::c_int
    {
        return c;
    } else {
        return '_' as ::core::ffi::c_char;
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn linkname_generate(
    mut linkname: *mut ::core::ffi::c_char,
    mut masterhost: *const ::core::ffi::c_char,
    mut masterport: *const ::core::ffi::c_char,
    mut filename: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut mhl: uint32_t = 0;
        let mut mpl: uint32_t = 0;
        let mut fnl: uint32_t = 0;
        let mut l: uint32_t = 0;
        let mut i: uint32_t = 0;
        let mut ln: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if !linkname.is_null() {
            fnl = strlen(linkname) as uint32_t;
            ln = malloc(
                fnl.wrapping_add(NBD_LINK_PREFIX_LENG as uint32_t)
                    .wrapping_add(1 as uint32_t) as size_t,
            ) as *mut ::core::ffi::c_char;
            if ln.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    658 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ln\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    658 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ln\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if ln
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut ::core::ffi::c_char
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    658 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ln\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    658 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ln\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            memcpy(
                ln as *mut ::core::ffi::c_void,
                NBD_LINK_PREFIX.as_ptr() as *const ::core::ffi::c_void,
                NBD_LINK_PREFIX_LENG as size_t,
            );
            l = 9 as uint32_t;
            i = 0 as uint32_t;
            while i < fnl {
                let c2rust_fresh0 = l;
                l = l.wrapping_add(1);
                *ln.offset(c2rust_fresh0 as isize) = charconv(*linkname.offset(i as isize));
                i = i.wrapping_add(1);
            }
            free(linkname as *mut ::core::ffi::c_void);
        } else {
            mhl = strlen(masterhost) as uint32_t;
            mpl = strlen(masterport) as uint32_t;
            fnl = strlen(filename) as uint32_t;
            ln = malloc(
                mhl.wrapping_add(mpl)
                    .wrapping_add(fnl)
                    .wrapping_add(3 as uint32_t)
                    .wrapping_add(1 as uint32_t)
                    .wrapping_add(NBD_LINK_PREFIX_LENG as uint32_t) as size_t,
            ) as *mut ::core::ffi::c_char;
            if ln.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    672 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ln\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    672 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ln\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if ln
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut ::core::ffi::c_char
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    672 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ln\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    672 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ln\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
            memcpy(
                ln as *mut ::core::ffi::c_void,
                NBD_LINK_PREFIX.as_ptr() as *const ::core::ffi::c_void,
                NBD_LINK_PREFIX_LENG as size_t,
            );
            l = 9 as uint32_t;
            i = 0 as uint32_t;
            while i < mhl {
                let c2rust_fresh1 = l;
                l = l.wrapping_add(1);
                *ln.offset(c2rust_fresh1 as isize) = charconv(*masterhost.offset(i as isize));
                i = i.wrapping_add(1);
            }
            let c2rust_fresh2 = l;
            l = l.wrapping_add(1);
            *ln.offset(c2rust_fresh2 as isize) = '_' as ::core::ffi::c_char;
            i = 0 as uint32_t;
            while i < mpl {
                let c2rust_fresh3 = l;
                l = l.wrapping_add(1);
                *ln.offset(c2rust_fresh3 as isize) = charconv(*masterport.offset(i as isize));
                i = i.wrapping_add(1);
            }
            let c2rust_fresh4 = l;
            l = l.wrapping_add(1);
            *ln.offset(c2rust_fresh4 as isize) = '_' as ::core::ffi::c_char;
            i = 0 as uint32_t;
            while i < fnl {
                let c2rust_fresh5 = l;
                l = l.wrapping_add(1);
                *ln.offset(c2rust_fresh5 as isize) = charconv(*filename.offset(i as isize));
                i = i.wrapping_add(1);
            }
        }
        let c2rust_fresh6 = l;
        l = l.wrapping_add(1);
        *ln.offset(c2rust_fresh6 as isize) = '\0' as ::core::ffi::c_char;
        if l > (256 as ::core::ffi::c_int + NBD_LINK_PREFIX_LENG) as uint32_t {
            *ln.offset((256 as ::core::ffi::c_int + NBD_LINK_PREFIX_LENG) as isize) =
                '\0' as ::core::ffi::c_char;
        }
        return ln;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn find_free_nbddevice() -> *mut ::core::ffi::c_char {
    unsafe {
        let mut nbdfd: ::core::ffi::c_int = 0;
        let mut devicename: [::core::ffi::c_char; 50] = [0; 50];
        let mut size: uint64_t = 0;
        let mut i: uint32_t = 0;
        let mut err: ::core::ffi::c_int = 0;
        i = 0 as uint32_t;
        loop {
            snprintf(
                &raw mut devicename as *mut ::core::ffi::c_char,
                50 as size_t,
                b"/dev/nbd%u\0".as_ptr() as *const ::core::ffi::c_char,
                i,
            );
            devicename[49 as usize] = 0 as ::core::ffi::c_char;
            nbdfd = open(&raw mut devicename as *mut ::core::ffi::c_char, O_RDWR);
            if nbdfd < 0 as ::core::ffi::c_int {
                if *__errno_location() == ENOENT || *__errno_location() == EACCES {
                    return ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
            } else {
                err = ioctl(nbdfd, BLKGETSIZE64 as ::core::ffi::c_ulong, &raw mut size);
                close(nbdfd);
                if err < 0 as ::core::ffi::c_int {
                    return ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                if size == 0 as uint64_t {
                    return strdup(&raw mut devicename as *mut ::core::ffi::c_char);
                }
            }
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nbd_linktest(mut nbdcp: *mut nbdcommon) -> ::core::ffi::c_int {
    unsafe {
        let mut size: uint64_t = 0;
        let mut err: ::core::ffi::c_int = 0;
        let mut fd: ::core::ffi::c_int = 0;
        fd = open((*nbdcp).linkname, O_RDWR);
        if fd >= 0 as ::core::ffi::c_int {
            err = ioctl(fd, BLKGETSIZE64 as ::core::ffi::c_ulong, &raw mut size);
            close(fd);
            if err >= 0 as ::core::ffi::c_int && size > 0 as uint64_t {
                return -1 as ::core::ffi::c_int;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nbd_packet_to_str(
    mut pstr: *const uint8_t,
    mut pleng: uint32_t,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut r: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if pleng == 0 as uint32_t {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        r = malloc(pleng.wrapping_add(1 as uint32_t) as size_t) as *mut ::core::ffi::c_char;
        if r.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr() as *const ::core::ffi::c_char,
                753 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"r\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr() as *const ::core::ffi::c_char,
                753 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"r\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if r
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut ::core::ffi::c_char
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr() as *const ::core::ffi::c_char,
                753 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"r\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr() as *const ::core::ffi::c_char,
                753 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"r\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        memcpy(
            r as *mut ::core::ffi::c_void,
            pstr as *const ::core::ffi::c_void,
            pleng as size_t,
        );
        *r.offset(pleng as isize) = 0 as ::core::ffi::c_char;
        return r;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nbd_force_partition_reread(mut nbdcp: *mut nbdcommon) {
    unsafe {
        let mut err: ::core::ffi::c_int = 0;
        let mut status: ::core::ffi::c_int = 0;
        err = fork() as ::core::ffi::c_int;
        if err < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"fork error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
        } else if err == 0 as ::core::ffi::c_int {
            err = open((*nbdcp).nbddevice, O_RDONLY);
            if err < 0 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"error opening %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    (*nbdcp).nbddevice,
                    strerror(*__errno_location()),
                );
            } else {
                close(err);
            }
            exit(0 as ::core::ffi::c_int);
        } else {
            waitpid(err as __pid_t, &raw mut status, 0 as ::core::ffi::c_int);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nbd_start(
    mut nbdcp: *mut nbdcommon,
    mut errmsg: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut omode: ::core::ffi::c_int = 0;
        let mut lmode: ::core::ffi::c_int = 0;
        let mut err: ::core::ffi::c_int = 0;
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
        *errmsg.offset(0 as isize) = 0 as ::core::ffi::c_char;
        if (*nbdcp).nbddevice.is_null() {
            (*nbdcp).nbddevice = find_free_nbddevice();
        }
        if (*nbdcp).nbddevice.is_null() {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                b"can't find free NBD device\0".as_ptr() as *const ::core::ffi::c_char,
            );
            snprintf(
                errmsg as *mut ::core::ffi::c_char,
                NBD_ERR_SIZE as size_t,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                b"can't find free NBD device\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            if (*nbdcp).flags & FLAG_READONLY as uint32_t != 0 {
                omode = O_RDONLY;
                lmode = LOCK_SH;
            } else {
                omode = O_RDWR;
                lmode = LOCK_EX;
            }
            if (*nbdcp).fsize != 0 as uint64_t {
                omode |= O_CREAT;
            }
            (*nbdcp).mfsfd = mfs_open((*nbdcp).mfsfile, omode, 0o666 as ::core::ffi::c_int);
            if (*nbdcp).mfsfd < 0 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"error opening MFS file %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    (*nbdcp).mfsfile,
                    strerror(*__errno_location()),
                );
                snprintf(
                    errmsg as *mut ::core::ffi::c_char,
                    NBD_ERR_SIZE as size_t,
                    b"error opening MFS file %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    (*nbdcp).mfsfile,
                    strerror(*__errno_location()),
                );
            } else {
                '_err2: {
                    if mfs_flock((*nbdcp).mfsfd, lmode | LOCK_NB) < 0 as ::core::ffi::c_int {
                        if (*nbdcp).flags & FLAG_IGNORELOCK as uint32_t == 0 as uint32_t {
                            if *__errno_location() == EROFS {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_ERR,
                                    b"MFS file %s is on a read-only filesystem - only read-only devices can be used and lock ignoring must be on (options -r and -i)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    (*nbdcp).mfsfile,
                                );
                                snprintf(
                                    errmsg as *mut ::core::ffi::c_char,
                                    NBD_ERR_SIZE as size_t,
                                    b"MFS file %s is on a read-only filesystem - only read-only devices can be used and lock ignoring must be on (options -r and -i)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    (*nbdcp).mfsfile,
                                );
                            } else {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_ERR,
                                    b"MFS file %s is locked (likely mapped elsewhere)\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    (*nbdcp).mfsfile,
                                );
                                snprintf(
                                    errmsg as *mut ::core::ffi::c_char,
                                    NBD_ERR_SIZE as size_t,
                                    b"MFS file %s is locked (likely mapped elsewhere)\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    (*nbdcp).mfsfile,
                                );
                            }
                            break '_err2;
                        }
                    }
                    '_err3: {
                        if (*nbdcp).fsize == 0 as uint64_t {
                            if mfs_fstat((*nbdcp).mfsfd, &raw mut stbuf) < 0 as ::core::ffi::c_int {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_ERR,
                                    b"can't stat MFS file '%s': %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    (*nbdcp).mfsfile,
                                    strerror(*__errno_location()),
                                );
                                snprintf(
                                    errmsg as *mut ::core::ffi::c_char,
                                    NBD_ERR_SIZE as size_t,
                                    b"can't stat MFS file '%s': %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    (*nbdcp).mfsfile,
                                    strerror(*__errno_location()),
                                );
                                break '_err3;
                            } else {
                                (*nbdcp).fsize = stbuf.st_size as uint64_t;
                            }
                        }
                        (*nbdcp).fsize = ((*nbdcp).bsize as uint64_t)
                            .wrapping_mul((*nbdcp).fsize.wrapping_div((*nbdcp).bsize as uint64_t));
                        if (*nbdcp).fsize == 0 as uint64_t {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                b"file size too low (less than one 4k block)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            snprintf(
                                errmsg as *mut ::core::ffi::c_char,
                                NBD_ERR_SIZE as size_t,
                                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                b"file size too low (less than one 4k block)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        } else if nbd_open_device(nbdcp, errmsg) >= 0 as ::core::ffi::c_int {
                            (*nbdcp).aqueue = Box::into_raw(Box::new(SQueue::new(0 as uint32_t)));
                            if (*nbdcp).aqueue.is_null() {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_ERR,
                                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                    b"can't create queue\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                snprintf(
                                    errmsg as *mut ::core::ffi::c_char,
                                    NBD_ERR_SIZE as size_t,
                                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                    b"can't create queue\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                            } else {
                                (*nbdcp).active = 1 as ::core::ffi::c_int;
                                // C: lwt_minthread_create(joinable,
                                // nbd_controller_thread). spawn_min keeps the
                                // same stack clamp + blocked daemon signals.
                                let ptr = NbdcPtr(nbdcp);
                                match plfscommon::lwthread::spawn_min("nbdctrl", move || {
                                    let ptr = ptr;
                                    unsafe {
                                        nbd_controller_thread(ptr.0 as *mut ::core::ffi::c_void);
                                    }
                                }) {
                                    Ok(handle) => {
                                        (*nbdcp).ctrl_thread = Some(handle);
                                        err = 0 as ::core::ffi::c_int;
                                    }
                                    Err(_) => {
                                        err = -(1 as ::core::ffi::c_int);
                                    }
                                }
                                if err < 0 as ::core::ffi::c_int {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"can't create controller thread: %s\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        strerror(*__errno_location()),
                                    );
                                    snprintf(
                                        errmsg as *mut ::core::ffi::c_char,
                                        NBD_ERR_SIZE as size_t,
                                        b"can't create controller thread: %s\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        strerror(*__errno_location()),
                                    );
                                    drop(Box::from_raw((*nbdcp).aqueue));
                                } else {
                                    err = mkdir(NBD_LINK_PREFIX.as_ptr(), 0o777 as __mode_t);
                                    err = unlink((*nbdcp).linkname);
                                    err = symlink((*nbdcp).nbddevice, (*nbdcp).linkname);
                                    if err < 0 as ::core::ffi::c_int {
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_WARNING,
                                            b"can't create nbd device symlink %s->%s: %s\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            (*nbdcp).linkname,
                                            (*nbdcp).nbddevice,
                                            strerror(*__errno_location()),
                                        );
                                    }
                                    return 0 as ::core::ffi::c_int;
                                }
                            }
                            ioctl((*nbdcp).nbdfd, NBD_CLEAR_QUE as ::core::ffi::c_ulong);
                            ioctl((*nbdcp).nbdfd, NBD_CLEAR_SOCK as ::core::ffi::c_ulong);
                            close((*nbdcp).sp[0 as usize]);
                            close((*nbdcp).sp[1 as usize]);
                            close((*nbdcp).nbdfd);
                        }
                    }
                    mfs_flock((*nbdcp).mfsfd, LOCK_UN);
                }
                mfs_close((*nbdcp).mfsfd);
            }
        }
        return -1 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nbd_stop(mut nbdcp: *mut nbdcommon) {
    unsafe {
        let mut err: ::core::ffi::c_int = 0;
        (*nbdcp).active = 0 as ::core::ffi::c_int;
        err = ioctl((*nbdcp).nbdfd, NBD_CLEAR_QUE as ::core::ffi::c_ulong);
        if err < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"%s: ioctl (NBD_CLEAR_QUE) failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
                (*nbdcp).nbddevice,
                strerror(*__errno_location()),
            );
        }
        err = ioctl((*nbdcp).nbdfd, NBD_DISCONNECT as ::core::ffi::c_ulong);
        if err < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"%s: ioctl (NBD_DISCONNECT) failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
                (*nbdcp).nbddevice,
                strerror(*__errno_location()),
            );
        }
        if let Some(handle) = (*nbdcp).ctrl_thread.take() {
            let _ = handle.join();
        }
        drop(Box::from_raw((*nbdcp).aqueue));
        mfs_flock((*nbdcp).mfsfd, LOCK_UN);
        mfs_close((*nbdcp).mfsfd);
        err = unlink((*nbdcp).linkname);
        if err < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"can't remove nbd device symlink %s->%s: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*nbdcp).linkname,
                (*nbdcp).nbddevice,
                strerror(*__errno_location()),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nbd_free(mut nbdcp: *mut nbdcommon) {
    unsafe {
        if !(*nbdcp).linkname.is_null() {
            free((*nbdcp).linkname as *mut ::core::ffi::c_void);
        }
        if !(*nbdcp).nbddevice.is_null() {
            free((*nbdcp).nbddevice as *mut ::core::ffi::c_void);
        }
        if !(*nbdcp).mfsfile.is_null() {
            free((*nbdcp).mfsfile as *mut ::core::ffi::c_void);
        }
        free(nbdcp as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn nbd_match(
    mut nbdcp: *mut nbdcommon,
    mut pleng: uint32_t,
    mut path: *const uint8_t,
    mut dleng: uint32_t,
    mut device: *const uint8_t,
    mut nleng: uint32_t,
    mut name: *const uint8_t,
) -> uint8_t {
    unsafe {
        if pleng > 0 as uint32_t
            && (pleng as size_t != strlen((*nbdcp).mfsfile)
                || memcmp(
                    (*nbdcp).mfsfile as *const ::core::ffi::c_void,
                    path as *const ::core::ffi::c_void,
                    pleng as size_t,
                ) != 0)
        {
            return 0 as uint8_t;
        } else if dleng > 0 as uint32_t
            && (dleng as size_t != strlen((*nbdcp).nbddevice)
                || memcmp(
                    (*nbdcp).nbddevice as *const ::core::ffi::c_void,
                    device as *const ::core::ffi::c_void,
                    dleng as size_t,
                ) != 0)
        {
            return 0 as uint8_t;
        } else if nleng > 0 as uint32_t
            && (nleng as size_t != strlen((*nbdcp).linkname.offset(NBD_LINK_PREFIX_LENG as isize))
                || memcmp(
                    (*nbdcp).linkname.offset(NBD_LINK_PREFIX_LENG as isize)
                        as *const ::core::ffi::c_void,
                    name as *const ::core::ffi::c_void,
                    nleng as size_t,
                ) != 0)
        {
            return 0 as uint8_t;
        }
        return 1 as uint8_t;
    }
}
unsafe extern "C" fn nbd_auto_maps(mut cfgfname: *const ::core::ffi::c_char) -> uint8_t {
    unsafe {
        let mut path: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut device: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut name: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut pleng: uint16_t = 0;
        let mut dleng: uint8_t = 0;
        let mut nleng: uint8_t = 0;
        let mut size: uint64_t = 0;
        let mut bsize: uint32_t = 0;
        let mut flags: uint32_t = 0;
        static mut bdl: *mut bdlist = ::core::ptr::null_mut::<bdlist>();
        let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut eptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut flagstr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut lbuff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut lbsize: size_t = 0;
        let mut ans: [::core::ffi::c_char; 200] = [0; 200];
        let mut cfd: *mut FILE = ::core::ptr::null_mut::<FILE>();
        cfd = fopen(cfgfname, b"r\0".as_ptr() as *const ::core::ffi::c_char) as *mut FILE;
        if cfd.is_null() {
            fprintf(
                stderr,
                b"can't open auto mappings file: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                cfgfname,
            );
            return 0 as uint8_t;
        }
        lbsize = 10000 as size_t;
        lbuff = malloc(lbsize) as *mut ::core::ffi::c_char;
        while getline(&raw mut lbuff, &raw mut lbsize, cfd) != -1 as __ssize_t {
            ptr = lbuff;
            while *ptr.offset(0 as isize) as ::core::ffi::c_int != '\r' as ::core::ffi::c_int
                && *ptr.offset(0 as isize) as ::core::ffi::c_int != '\n' as ::core::ffi::c_int
                && *ptr.offset(0 as isize) as ::core::ffi::c_int != 0
            {
                ptr = ptr.offset(1);
            }
            *ptr.offset(0 as isize) = 0 as ::core::ffi::c_char;
            if *lbuff.offset(0 as isize) as ::core::ffi::c_int == ';' as ::core::ffi::c_int
                || *lbuff.offset(0 as isize) as ::core::ffi::c_int == '#' as ::core::ffi::c_int
                || *lbuff.offset(0 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                continue;
            }
            ptr = lbuff;
            size = 0 as uint64_t;
            bsize = 0 as uint32_t;
            flags = 0 as uint32_t;
            path = ptr as *mut uint8_t;
            pleng = 0 as uint16_t;
            while *ptr as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
                && *ptr as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
                && *ptr as ::core::ffi::c_int != 0
            {
                ptr = ptr.offset(1);
                pleng = pleng.wrapping_add(1);
            }
            while *ptr as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                || *ptr as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
            {
                ptr = ptr.offset(1);
            }
            device = ptr as *mut uint8_t;
            dleng = 0 as uint8_t;
            while *ptr as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
                && *ptr as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
                && *ptr as ::core::ffi::c_int != 0
            {
                ptr = ptr.offset(1);
                dleng = dleng.wrapping_add(1);
            }
            while *ptr as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                || *ptr as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
            {
                ptr = ptr.offset(1);
            }
            if dleng as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                && *device.offset(0 as isize) as ::core::ffi::c_int == '*' as ::core::ffi::c_int
            {
                dleng = 0 as uint8_t;
            }
            name = ptr as *mut uint8_t;
            nleng = 0 as uint8_t;
            while *ptr as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
                && *ptr as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
                && *ptr as ::core::ffi::c_int != 0
            {
                ptr = ptr.offset(1);
                nleng = nleng.wrapping_add(1);
            }
            while *ptr as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                || *ptr as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
            {
                ptr = ptr.offset(1);
            }
            if nleng as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                && *name.offset(0 as isize) as ::core::ffi::c_int == '*' as ::core::ffi::c_int
            {
                nleng = 0 as uint8_t;
            }
            if *ptr != 0 {
                if *ptr as ::core::ffi::c_int == '*' as ::core::ffi::c_int
                    && (*ptr.offset(1 as isize) as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                        || *ptr.offset(1 as isize) as ::core::ffi::c_int
                            == '\t' as ::core::ffi::c_int
                        || *ptr.offset(1 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
                {
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                } else {
                    size = sizestrtod(ptr, &raw mut eptr) as uint64_t;
                    ptr = eptr as *mut ::core::ffi::c_char;
                    if *ptr as ::core::ffi::c_int == 'B' as ::core::ffi::c_int {
                        ptr = ptr.offset(1);
                    }
                }
                if *ptr as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
                    && *ptr as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
                    && *ptr as ::core::ffi::c_int != 0
                {
                    fprintf(
                        stderr,
                        b"error parsing device size\n\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    free(lbuff as *mut ::core::ffi::c_void);
                    fclose(cfd);
                    return 0 as uint8_t;
                }
                while *ptr as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                    || *ptr as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
                {
                    ptr = ptr.offset(1);
                }
            }
            if *ptr != 0 {
                if *ptr as ::core::ffi::c_int == '*' as ::core::ffi::c_int
                    && (*ptr.offset(1 as isize) as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                        || *ptr.offset(1 as isize) as ::core::ffi::c_int
                            == '\t' as ::core::ffi::c_int
                        || *ptr.offset(1 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
                {
                    ptr = ptr.offset(1 as ::core::ffi::c_int as isize);
                } else {
                    bsize = sizestrtod(ptr, &raw mut eptr) as uint32_t;
                    ptr = eptr as *mut ::core::ffi::c_char;
                    if *ptr as ::core::ffi::c_int == 'B' as ::core::ffi::c_int {
                        ptr = ptr.offset(1);
                    }
                }
                if *ptr as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
                    && *ptr as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
                    && *ptr as ::core::ffi::c_int != 0
                {
                    fprintf(
                        stderr,
                        b"error parsing block size\n\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    free(lbuff as *mut ::core::ffi::c_void);
                    fclose(cfd);
                    return 0 as uint8_t;
                }
                while *ptr as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                    || *ptr as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
                {
                    ptr = ptr.offset(1);
                }
            }
            loop {
                flagstr = strsep(
                    &raw mut ptr,
                    b", \t\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if flagstr.is_null() {
                    break;
                }
                if *flagstr.offset(0 as isize) != 0 {
                    if strcmp(flagstr, b"ro\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        flags |= FLAG_READONLY as uint32_t;
                    } else if strcmp(flagstr, b"rw\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        flags &= !FLAG_READONLY as uint32_t;
                    } else if strcmp(flagstr, b"nolock\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        flags |= FLAG_IGNORELOCK as uint32_t;
                    } else if strcmp(flagstr, b"lock\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        flags &= !FLAG_IGNORELOCK as uint32_t;
                    } else {
                        fprintf(
                            stderr,
                            b"unknown flag: %s - ignored\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            flagstr,
                        );
                    }
                }
            }
            bdl = malloc(::core::mem::size_of::<bdlist>()) as *mut bdlist;
            if bdl.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1079 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"bdl\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1079 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"bdl\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if bdl
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut bdlist
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1079 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"bdl\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1079 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"bdl\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            (*bdl).nbdcp = malloc(::core::mem::size_of::<nbdcommon>()) as *mut nbdcommon;
            if (*bdl).nbdcp.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1081 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"bdl->nbdcp\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1081 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"bdl->nbdcp\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if (*bdl).nbdcp
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut nbdcommon
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1081 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"bdl->nbdcp\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1081 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"bdl->nbdcp\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
            // Rust JoinHandle slot (replaces pthread_t): init before any
            // nbd_stop path can take() it; malloc leaves it uninit otherwise.
            std::ptr::write(&raw mut (*(*bdl).nbdcp).ctrl_thread, None);
            (*(*bdl).nbdcp).mfsfile = nbd_packet_to_str(path, pleng as uint32_t);
            (*(*bdl).nbdcp).nbddevice = nbd_packet_to_str(device, dleng as uint32_t);
            (*(*bdl).nbdcp).linkname = nbd_packet_to_str(name, nleng as uint32_t);
            (*(*bdl).nbdcp).fsize = size;
            (*(*bdl).nbdcp).bsize = bsize;
            (*(*bdl).nbdcp).flags = flags;
            (*(*bdl).nbdcp).linkname = linkname_generate(
                (*(*bdl).nbdcp).linkname,
                mcfg.masterhost,
                mcfg.masterport,
                (*(*bdl).nbdcp).mfsfile,
            );
            if nbd_linktest((*bdl).nbdcp) < 0 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"link exists\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
                nbd_free((*bdl).nbdcp);
                free(bdl as *mut ::core::ffi::c_void);
                free(lbuff as *mut ::core::ffi::c_void);
                fclose(cfd);
                return 0 as uint8_t;
            } else if nbd_start((*bdl).nbdcp, &raw mut ans as *mut ::core::ffi::c_char)
                < 0 as ::core::ffi::c_int
            {
                ans[(NBD_ERR_SIZE - 1 as ::core::ffi::c_int) as usize] = 0 as ::core::ffi::c_char;
                fprintf(
                    stderr,
                    b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut ans as *mut ::core::ffi::c_char,
                );
                nbd_free((*bdl).nbdcp);
                free(bdl as *mut ::core::ffi::c_void);
                free(lbuff as *mut ::core::ffi::c_void);
                fclose(cfd);
                return 0 as uint8_t;
            } else {
                fprintf(
                    stderr,
                    b"started block device: (%s->%s : MFS:/%s : %.3lfGiB)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*(*bdl).nbdcp).linkname,
                    (*(*bdl).nbdcp).nbddevice,
                    (*(*bdl).nbdcp).mfsfile,
                    (*(*bdl).nbdcp).fsize as ::core::ffi::c_double
                        / (1024.0f64 * 1024.0f64 * 1024.0f64),
                );
                (*bdl).next = bdhead as *mut _bdlist;
                bdhead = bdl;
            }
        }
        free(lbuff as *mut ::core::ffi::c_void);
        fclose(cfd);
        bdl = bdhead;
        while !bdl.is_null() {
            nbd_force_partition_reread((*bdl).nbdcp);
            bdl = (*bdl).next as *mut bdlist;
        }
        return 1 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nbd_handle_nop(
    mut sock: ::core::ffi::c_int,
    _buff: *const uint8_t,
    _leng: uint32_t,
) {
    unsafe {
        let mut ans: [uint8_t; 8] = [0; 8];
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        wptr = &raw mut ans as *mut uint8_t;
        put32bit(&raw mut wptr, MFSNBD_NOP as ::core::ffi::c_int as uint32_t);
        put32bit(&raw mut wptr, 0 as uint32_t);
        unixtowrite(
            sock,
            &raw mut ans as *mut uint8_t as *const ::core::ffi::c_void,
            8 as uint32_t,
            1000 as uint32_t,
            1000 as uint32_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nbd_handle_stop_daemon(
    mut sock: ::core::ffi::c_int,
    _buff: *const uint8_t,
    mut leng: uint32_t,
) {
    unsafe {
        let mut ans: [uint8_t; 9] = [0; 9];
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        term = 1 as uint8_t;
        wptr = &raw mut ans as *mut uint8_t;
        put32bit(&raw mut wptr, MFSNBD_STOP as ::core::ffi::c_int as uint32_t);
        put32bit(&raw mut wptr, 1 as uint32_t);
        put8bit(
            &raw mut wptr,
            (if leng == 0 as uint32_t {
                MFSNBD_OK as ::core::ffi::c_int
            } else {
                MFSNBD_ERROR as ::core::ffi::c_int
            }) as uint8_t,
        );
        unixtowrite(
            sock,
            &raw mut ans as *mut uint8_t as *const ::core::ffi::c_void,
            9 as uint32_t,
            1000 as uint32_t,
            1000 as uint32_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nbd_handle_add_device(
    mut sock: ::core::ffi::c_int,
    mut buff: *const uint8_t,
    mut leng: uint32_t,
) {
    unsafe {
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut ans: [uint8_t; 210] = [0; 210];
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut msglen: uint32_t = 0;
        let mut status: uint8_t = 0;
        let mut path: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut device: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut name: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut pleng: uint16_t = 0;
        let mut dleng: uint8_t = 0;
        let mut nleng: uint8_t = 0;
        let mut size: uint64_t = 0;
        let mut bsize: uint32_t = 0;
        let mut flags: uint32_t = 0;
        static mut bdl: *mut bdlist = ::core::ptr::null_mut::<bdlist>();
        wptr = &raw mut ans as *mut uint8_t;
        put32bit(&raw mut wptr, MFSNBD_ADD as ::core::ffi::c_int as uint32_t);
        put32bit(&raw mut wptr, 0 as uint32_t);
        rptr = buff;
        if leng < 20 as uint32_t {
            unixtowrite(
                sock,
                &raw mut ans as *mut uint8_t as *const ::core::ffi::c_void,
                8 as uint32_t,
                1000 as uint32_t,
                1000 as uint32_t,
            );
            return;
        }
        pleng = get16bit(&raw mut rptr);
        path = rptr;
        rptr = rptr.offset(pleng as ::core::ffi::c_int as isize);
        if leng < (20 as uint32_t).wrapping_add(pleng as uint32_t) {
            unixtowrite(
                sock,
                &raw mut ans as *mut uint8_t as *const ::core::ffi::c_void,
                8 as uint32_t,
                1000 as uint32_t,
                1000 as uint32_t,
            );
            return;
        }
        dleng = get8bit(&raw mut rptr);
        device = rptr;
        rptr = rptr.offset(dleng as ::core::ffi::c_int as isize);
        if leng
            < (20 as uint32_t)
                .wrapping_add(pleng as uint32_t)
                .wrapping_add(dleng as uint32_t)
        {
            unixtowrite(
                sock,
                &raw mut ans as *mut uint8_t as *const ::core::ffi::c_void,
                8 as uint32_t,
                1000 as uint32_t,
                1000 as uint32_t,
            );
            return;
        }
        nleng = get8bit(&raw mut rptr);
        name = rptr;
        rptr = rptr.offset(nleng as ::core::ffi::c_int as isize);
        if leng
            != (20 as uint32_t)
                .wrapping_add(pleng as uint32_t)
                .wrapping_add(dleng as uint32_t)
                .wrapping_add(nleng as uint32_t)
        {
            unixtowrite(
                sock,
                &raw mut ans as *mut uint8_t as *const ::core::ffi::c_void,
                8 as uint32_t,
                1000 as uint32_t,
                1000 as uint32_t,
            );
            return;
        }
        size = get64bit(&raw mut rptr);
        bsize = get32bit(&raw mut rptr);
        flags = get32bit(&raw mut rptr);
        if pleng as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            msglen = snprintf(
                (&raw mut ans as *mut uint8_t).offset(10 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char,
                NBD_ERR_SIZE as size_t,
                b"empty filename\0".as_ptr() as *const ::core::ffi::c_char,
            ) as uint32_t;
            status = MFSNBD_ERROR as ::core::ffi::c_int as uint8_t;
        } else {
            bdl = malloc(::core::mem::size_of::<bdlist>()) as *mut bdlist;
            if bdl.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1201 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"bdl\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1201 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"bdl\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if bdl
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut bdlist
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1201 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"bdl\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1201 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"bdl\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            (*bdl).nbdcp = malloc(::core::mem::size_of::<nbdcommon>()) as *mut nbdcommon;
            if (*bdl).nbdcp.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1203 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"bdl->nbdcp\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1203 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"bdl->nbdcp\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if (*bdl).nbdcp
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut nbdcommon
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1203 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"bdl->nbdcp\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1203 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"bdl->nbdcp\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
            // Rust JoinHandle slot (replaces pthread_t): init before any
            // nbd_stop path can take() it; malloc leaves it uninit otherwise.
            std::ptr::write(&raw mut (*(*bdl).nbdcp).ctrl_thread, None);
            (*(*bdl).nbdcp).mfsfile = nbd_packet_to_str(path, pleng as uint32_t);
            (*(*bdl).nbdcp).nbddevice = nbd_packet_to_str(device, dleng as uint32_t);
            (*(*bdl).nbdcp).linkname = nbd_packet_to_str(name, nleng as uint32_t);
            (*(*bdl).nbdcp).fsize = size;
            (*(*bdl).nbdcp).bsize = bsize;
            (*(*bdl).nbdcp).flags = flags;
            (*(*bdl).nbdcp).linkname = linkname_generate(
                (*(*bdl).nbdcp).linkname,
                mcfg.masterhost,
                mcfg.masterport,
                (*(*bdl).nbdcp).mfsfile,
            );
            if nbd_linktest((*bdl).nbdcp) < 0 as ::core::ffi::c_int {
                msglen = snprintf(
                    (&raw mut ans as *mut uint8_t).offset(10 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                    NBD_ERR_SIZE as size_t,
                    b"link exists\0".as_ptr() as *const ::core::ffi::c_char,
                ) as uint32_t;
                status = MFSNBD_ERROR as ::core::ffi::c_int as uint8_t;
                nbd_free((*bdl).nbdcp);
                free(bdl as *mut ::core::ffi::c_void);
            } else if nbd_start(
                (*bdl).nbdcp,
                (&raw mut ans as *mut uint8_t).offset(10 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char,
            ) < 0 as ::core::ffi::c_int
            {
                ans[(9 as ::core::ffi::c_int + NBD_ERR_SIZE) as usize] = 0 as uint8_t;
                msglen = strlen(
                    (&raw mut ans as *mut uint8_t).offset(10 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                ) as uint32_t;
                status = MFSNBD_ERROR as ::core::ffi::c_int as uint8_t;
                nbd_free((*bdl).nbdcp);
                free(bdl as *mut ::core::ffi::c_void);
            } else {
                nbd_force_partition_reread((*bdl).nbdcp);
                msglen = snprintf(
                    (&raw mut ans as *mut uint8_t).offset(10 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                    NBD_ERR_SIZE as size_t,
                    b"started block device: (%s->%s : MFS:/%s : %.3lfGiB)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*(*bdl).nbdcp).linkname,
                    (*(*bdl).nbdcp).nbddevice,
                    (*(*bdl).nbdcp).mfsfile,
                    (*(*bdl).nbdcp).fsize as ::core::ffi::c_double
                        / (1024.0f64 * 1024.0f64 * 1024.0f64),
                ) as uint32_t;
                status = MFSNBD_OK as ::core::ffi::c_int as uint8_t;
                (*bdl).next = bdhead as *mut _bdlist;
                bdhead = bdl;
            }
        }
        wptr = (&raw mut ans as *mut uint8_t).offset(4 as ::core::ffi::c_int as isize);
        if msglen > NBD_ERR_SIZE as uint32_t {
            msglen = NBD_ERR_SIZE as uint32_t;
        }
        put32bit(&raw mut wptr, msglen.wrapping_add(2 as uint32_t));
        put8bit(&raw mut wptr, status);
        put8bit(&raw mut wptr, msglen as uint8_t);
        unixtowrite(
            sock,
            &raw mut ans as *mut uint8_t as *const ::core::ffi::c_void,
            (10 as uint32_t).wrapping_add(msglen),
            1000 as uint32_t,
            1000 as uint32_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nbd_handle_remove_device(
    mut sock: ::core::ffi::c_int,
    mut buff: *const uint8_t,
    mut leng: uint32_t,
) {
    unsafe {
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut ans: [uint8_t; 210] = [0; 210];
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut msglen: uint32_t = 0;
        let mut path: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut device: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut name: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut pleng: uint16_t = 0;
        let mut dleng: uint8_t = 0;
        let mut nleng: uint8_t = 0;
        static mut bdl: *mut bdlist = ::core::ptr::null_mut::<bdlist>();
        static mut bdlp: *mut *mut bdlist = ::core::ptr::null_mut::<*mut bdlist>();
        let mut found: uint8_t = 0;
        wptr = &raw mut ans as *mut uint8_t;
        put32bit(
            &raw mut wptr,
            MFSNBD_REMOVE as ::core::ffi::c_int as uint32_t,
        );
        put32bit(&raw mut wptr, 0 as uint32_t);
        rptr = buff;
        if leng < 4 as uint32_t {
            unixtowrite(
                sock,
                &raw mut ans as *mut uint8_t as *const ::core::ffi::c_void,
                8 as uint32_t,
                1000 as uint32_t,
                1000 as uint32_t,
            );
            return;
        }
        pleng = get16bit(&raw mut rptr);
        path = rptr;
        rptr = rptr.offset(pleng as ::core::ffi::c_int as isize);
        if leng < (4 as uint32_t).wrapping_add(pleng as uint32_t) {
            unixtowrite(
                sock,
                &raw mut ans as *mut uint8_t as *const ::core::ffi::c_void,
                8 as uint32_t,
                1000 as uint32_t,
                1000 as uint32_t,
            );
            return;
        }
        dleng = get8bit(&raw mut rptr);
        device = rptr;
        rptr = rptr.offset(dleng as ::core::ffi::c_int as isize);
        if leng
            < (4 as uint32_t)
                .wrapping_add(pleng as uint32_t)
                .wrapping_add(dleng as uint32_t)
        {
            unixtowrite(
                sock,
                &raw mut ans as *mut uint8_t as *const ::core::ffi::c_void,
                8 as uint32_t,
                1000 as uint32_t,
                1000 as uint32_t,
            );
            return;
        }
        nleng = get8bit(&raw mut rptr);
        name = rptr;
        rptr = rptr.offset(nleng as ::core::ffi::c_int as isize);
        if leng
            != (4 as uint32_t)
                .wrapping_add(pleng as uint32_t)
                .wrapping_add(dleng as uint32_t)
                .wrapping_add(nleng as uint32_t)
        {
            unixtowrite(
                sock,
                &raw mut ans as *mut uint8_t as *const ::core::ffi::c_void,
                8 as uint32_t,
                1000 as uint32_t,
                1000 as uint32_t,
            );
            return;
        }
        found = 0 as uint8_t;
        msglen = 0 as uint32_t;
        bdlp = &raw mut bdhead;
        while found as ::core::ffi::c_int == 0 as ::core::ffi::c_int && {
            bdl = *bdlp;
            !bdl.is_null()
        } {
            if nbd_match(
                (*bdl).nbdcp,
                pleng as uint32_t,
                path,
                dleng as uint32_t,
                device,
                nleng as uint32_t,
                name,
            ) != 0
            {
                msglen = snprintf(
                    (&raw mut ans as *mut uint8_t).offset(10 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_char,
                    NBD_ERR_SIZE as size_t,
                    b"stop block device: (%s->%s : MFS:/%s : %.3lfGiB)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*(*bdl).nbdcp).linkname,
                    (*(*bdl).nbdcp).nbddevice,
                    (*(*bdl).nbdcp).mfsfile,
                    (*(*bdl).nbdcp).fsize as ::core::ffi::c_double
                        / (1024.0f64 * 1024.0f64 * 1024.0f64),
                ) as uint32_t;
                nbd_stop((*bdl).nbdcp);
                nbd_free((*bdl).nbdcp);
                *bdlp = (*bdl).next as *mut bdlist;
                free(bdl as *mut ::core::ffi::c_void);
                found = 1 as uint8_t;
            } else {
                bdlp = &raw mut (*bdl).next as *mut *mut bdlist;
            }
        }
        if found as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            msglen = snprintf(
                (&raw mut ans as *mut uint8_t).offset(10 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char,
                NBD_ERR_SIZE as size_t,
                b"device not found\0".as_ptr() as *const ::core::ffi::c_char,
            ) as uint32_t;
        }
        wptr = (&raw mut ans as *mut uint8_t).offset(4 as ::core::ffi::c_int as isize);
        if msglen > NBD_ERR_SIZE as uint32_t {
            msglen = NBD_ERR_SIZE as uint32_t;
        }
        put32bit(&raw mut wptr, msglen.wrapping_add(2 as uint32_t));
        put8bit(
            &raw mut wptr,
            (if found as ::core::ffi::c_int != 0 {
                MFSNBD_OK as ::core::ffi::c_int
            } else {
                MFSNBD_ERROR as ::core::ffi::c_int
            }) as uint8_t,
        );
        put8bit(&raw mut wptr, msglen as uint8_t);
        unixtowrite(
            sock,
            &raw mut ans as *mut uint8_t as *const ::core::ffi::c_void,
            (10 as uint32_t).wrapping_add(msglen),
            1000 as uint32_t,
            1000 as uint32_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nbd_handle_list_devices(
    mut sock: ::core::ffi::c_int,
    _buff: *const uint8_t,
    mut leng: uint32_t,
) {
    unsafe {
        let mut dcnt: uint8_t = 0;
        let mut dsize: uint32_t = 0;
        let mut pleng: uint32_t = 0;
        let mut dleng: uint32_t = 0;
        let mut nleng: uint32_t = 0;
        static mut bdl: *mut bdlist = ::core::ptr::null_mut::<bdlist>();
        let mut ans: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if leng != 0 as uint32_t {
            ans = malloc(8 as size_t) as *mut uint8_t;
            if ans.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1321 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ans\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1321 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ans\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if ans
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint8_t
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1321 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ans\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1321 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ans\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            wptr = ans;
            put32bit(&raw mut wptr, MFSNBD_LIST as ::core::ffi::c_int as uint32_t);
            put32bit(&raw mut wptr, 0 as uint32_t);
            unixtowrite(
                sock,
                ans as *const ::core::ffi::c_void,
                8 as uint32_t,
                1000 as uint32_t,
                1000 as uint32_t,
            );
            free(ans as *mut ::core::ffi::c_void);
            return;
        }
        dcnt = 0 as uint8_t;
        dsize = 1 as uint32_t;
        bdl = bdhead;
        while !bdl.is_null() {
            pleng = strlen((*(*bdl).nbdcp).mfsfile) as uint32_t;
            dleng = strlen((*(*bdl).nbdcp).nbddevice) as uint32_t;
            nleng = strlen(
                (*(*bdl).nbdcp)
                    .linkname
                    .offset(NBD_LINK_PREFIX_LENG as isize),
            ) as uint32_t;
            if pleng > 65535 as uint32_t {
                pleng = 65535 as uint32_t;
            }
            if dleng > 255 as uint32_t {
                dleng = 255 as uint32_t;
            }
            if nleng > 255 as uint32_t {
                nleng = 255 as uint32_t;
            }
            dsize = dsize.wrapping_add(
                pleng
                    .wrapping_add(dleng)
                    .wrapping_add(nleng)
                    .wrapping_add(20 as uint32_t),
            );
            dcnt = dcnt.wrapping_add(1);
            bdl = (*bdl).next as *mut bdlist;
        }
        ans = malloc((8 as uint32_t).wrapping_add(dsize) as size_t) as *mut uint8_t;
        if ans.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr() as *const ::core::ffi::c_char,
                1348 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"ans\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr() as *const ::core::ffi::c_char,
                1348 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"ans\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if ans
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut uint8_t
        {
            let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr() as *const ::core::ffi::c_char,
                1348 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"ans\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr() as *const ::core::ffi::c_char,
                1348 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"ans\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            abort();
        }
        wptr = ans;
        put32bit(&raw mut wptr, MFSNBD_LIST as ::core::ffi::c_int as uint32_t);
        put32bit(&raw mut wptr, dsize);
        put8bit(&raw mut wptr, dcnt);
        bdl = bdhead;
        while !bdl.is_null() {
            pleng = strlen((*(*bdl).nbdcp).mfsfile) as uint32_t;
            dleng = strlen((*(*bdl).nbdcp).nbddevice) as uint32_t;
            nleng = strlen(
                (*(*bdl).nbdcp)
                    .linkname
                    .offset(NBD_LINK_PREFIX_LENG as isize),
            ) as uint32_t;
            if pleng > 65535 as uint32_t {
                pleng = 65535 as uint32_t;
            }
            if dleng > 255 as uint32_t {
                dleng = 255 as uint32_t;
            }
            if nleng > 255 as uint32_t {
                nleng = 255 as uint32_t;
            }
            put16bit(&raw mut wptr, pleng as uint16_t);
            memcpy(
                wptr as *mut ::core::ffi::c_void,
                (*(*bdl).nbdcp).mfsfile as *const ::core::ffi::c_void,
                pleng as size_t,
            );
            wptr = wptr.offset(pleng as isize);
            put8bit(&raw mut wptr, dleng as uint8_t);
            memcpy(
                wptr as *mut ::core::ffi::c_void,
                (*(*bdl).nbdcp).nbddevice as *const ::core::ffi::c_void,
                dleng as size_t,
            );
            wptr = wptr.offset(dleng as isize);
            put8bit(&raw mut wptr, nleng as uint8_t);
            memcpy(
                wptr as *mut ::core::ffi::c_void,
                (*(*bdl).nbdcp)
                    .linkname
                    .offset(NBD_LINK_PREFIX_LENG as isize)
                    as *const ::core::ffi::c_void,
                nleng as size_t,
            );
            wptr = wptr.offset(nleng as isize);
            put64bit(&raw mut wptr, (*(*bdl).nbdcp).fsize);
            put32bit(&raw mut wptr, (*(*bdl).nbdcp).bsize);
            put32bit(&raw mut wptr, (*(*bdl).nbdcp).flags);
            bdl = (*bdl).next as *mut bdlist;
        }
        unixtowrite(
            sock,
            ans as *const ::core::ffi::c_void,
            (8 as uint32_t).wrapping_add(dsize),
            1000 as uint32_t,
            1000 as uint32_t,
        );
        free(ans as *mut ::core::ffi::c_void);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nbd_handle_resize_device(
    mut sock: ::core::ffi::c_int,
    mut buff: *const uint8_t,
    mut leng: uint32_t,
) {
    unsafe {
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut ans: [uint8_t; 210] = [0; 210];
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut msglen: uint32_t = 0;
        let mut path: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut device: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut name: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut pleng: uint16_t = 0;
        let mut dleng: uint8_t = 0;
        let mut nleng: uint8_t = 0;
        let mut size: uint64_t = 0;
        let mut tsize: uint64_t = 0;
        static mut bdl: *mut bdlist = ::core::ptr::null_mut::<bdlist>();
        let mut found: uint8_t = 0;
        wptr = &raw mut ans as *mut uint8_t;
        put32bit(
            &raw mut wptr,
            MFSNBD_RESIZE as ::core::ffi::c_int as uint32_t,
        );
        put32bit(&raw mut wptr, 0 as uint32_t);
        rptr = buff;
        if leng < 12 as uint32_t {
            unixtowrite(
                sock,
                &raw mut ans as *mut uint8_t as *const ::core::ffi::c_void,
                8 as uint32_t,
                1000 as uint32_t,
                1000 as uint32_t,
            );
            return;
        }
        pleng = get16bit(&raw mut rptr);
        path = rptr;
        rptr = rptr.offset(pleng as ::core::ffi::c_int as isize);
        if leng < (12 as uint32_t).wrapping_add(pleng as uint32_t) {
            unixtowrite(
                sock,
                &raw mut ans as *mut uint8_t as *const ::core::ffi::c_void,
                8 as uint32_t,
                1000 as uint32_t,
                1000 as uint32_t,
            );
            return;
        }
        dleng = get8bit(&raw mut rptr);
        device = rptr;
        rptr = rptr.offset(dleng as ::core::ffi::c_int as isize);
        if leng
            < (12 as uint32_t)
                .wrapping_add(pleng as uint32_t)
                .wrapping_add(dleng as uint32_t)
        {
            unixtowrite(
                sock,
                &raw mut ans as *mut uint8_t as *const ::core::ffi::c_void,
                8 as uint32_t,
                1000 as uint32_t,
                1000 as uint32_t,
            );
            return;
        }
        nleng = get8bit(&raw mut rptr);
        name = rptr;
        rptr = rptr.offset(nleng as ::core::ffi::c_int as isize);
        if leng
            != (12 as uint32_t)
                .wrapping_add(pleng as uint32_t)
                .wrapping_add(dleng as uint32_t)
                .wrapping_add(nleng as uint32_t)
        {
            unixtowrite(
                sock,
                &raw mut ans as *mut uint8_t as *const ::core::ffi::c_void,
                8 as uint32_t,
                1000 as uint32_t,
                1000 as uint32_t,
            );
            return;
        }
        size = get64bit(&raw mut rptr);
        found = 0 as uint8_t;
        msglen = 0 as uint32_t;
        bdl = bdhead;
        while found as ::core::ffi::c_int == 0 as ::core::ffi::c_int && !bdl.is_null() {
            if nbd_match(
                (*bdl).nbdcp,
                pleng as uint32_t,
                path,
                dleng as uint32_t,
                device,
                nleng as uint32_t,
                name,
            ) != 0
            {
                if size == 0 as uint64_t {
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
                    if mfs_fstat((*(*bdl).nbdcp).mfsfd, &raw mut stbuf) < 0 as ::core::ffi::c_int {
                        msglen = snprintf(
                            (&raw mut ans as *mut uint8_t).offset(10 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_char,
                            NBD_ERR_SIZE as size_t,
                            b"can't stat MFS file '%s': %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*(*bdl).nbdcp).mfsfile,
                            strerror(*__errno_location()),
                        ) as uint32_t;
                    } else {
                        size = stbuf.st_size as uint64_t;
                    }
                }
                if size > 0 as uint64_t {
                    size = ((*(*bdl).nbdcp).bsize as uint64_t)
                        .wrapping_mul(size.wrapping_div((*(*bdl).nbdcp).bsize as uint64_t));
                    if size == 0 as uint64_t {
                        msglen = snprintf(
                            (&raw mut ans as *mut uint8_t).offset(10 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_char,
                            NBD_ERR_SIZE as size_t,
                            b"file size too low (less than block size (%uB))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*(*bdl).nbdcp).bsize,
                        ) as uint32_t;
                    }
                }
                if size > 0 as uint64_t {
                    if ioctl(
                        (*(*bdl).nbdcp).nbdfd,
                        NBD_SET_SIZE_BLOCKS as ::core::ffi::c_ulong,
                        size.wrapping_div((*(*bdl).nbdcp).bsize as uint64_t),
                    ) < 0 as ::core::ffi::c_int
                    {
                        msglen = snprintf(
                            (&raw mut ans as *mut uint8_t).offset(10 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_char,
                            NBD_ERR_SIZE as size_t,
                            b"error setting block device number of blocks (%s): %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*(*bdl).nbdcp).nbddevice,
                            strerror(*__errno_location()),
                        ) as uint32_t;
                    } else if ioctl(
                        (*(*bdl).nbdcp).nbdfd,
                        BLKGETSIZE64 as ::core::ffi::c_ulong,
                        &raw mut tsize,
                    ) < 0 as ::core::ffi::c_int
                    {
                        msglen = snprintf(
                            (&raw mut ans as *mut uint8_t).offset(10 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_char,
                            NBD_ERR_SIZE as size_t,
                            b"error testing block device size (%s): %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*(*bdl).nbdcp).nbddevice,
                            strerror(*__errno_location()),
                        ) as uint32_t;
                    } else if tsize != size {
                        msglen = snprintf(
                            (&raw mut ans as *mut uint8_t).offset(10 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_char,
                            NBD_ERR_SIZE as size_t,
                            b"can't resize block device - kernel 4.18+ is needed\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        ) as uint32_t;
                    } else {
                        msglen = snprintf(
                            (&raw mut ans as *mut uint8_t)
                                .offset(10 as ::core::ffi::c_int as isize)
                                as *mut ::core::ffi::c_char,
                            NBD_ERR_SIZE as size_t,
                            b"change size of block device: (%s->%s : MFS:/%s : %.3lfGiB) -> %.3lfGiB\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            (*(*bdl).nbdcp).linkname,
                            (*(*bdl).nbdcp).nbddevice,
                            (*(*bdl).nbdcp).mfsfile,
                            (*(*bdl).nbdcp).fsize as ::core::ffi::c_double
                                / (1024.0f64 * 1024.0f64 * 1024.0f64),
                            size as ::core::ffi::c_double
                                / (1024.0f64 * 1024.0f64 * 1024.0f64),
                        ) as uint32_t;
                        (*(*bdl).nbdcp).fsize = size;
                    }
                }
                found = 1 as uint8_t;
            }
            bdl = (*bdl).next as *mut bdlist;
        }
        if found as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            msglen = snprintf(
                (&raw mut ans as *mut uint8_t).offset(10 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char,
                NBD_ERR_SIZE as size_t,
                b"device not found\0".as_ptr() as *const ::core::ffi::c_char,
            ) as uint32_t;
        }
        wptr = (&raw mut ans as *mut uint8_t).offset(4 as ::core::ffi::c_int as isize);
        if msglen > NBD_ERR_SIZE as uint32_t {
            msglen = NBD_ERR_SIZE as uint32_t;
        }
        put32bit(&raw mut wptr, msglen.wrapping_add(2 as uint32_t));
        put8bit(
            &raw mut wptr,
            (if found as ::core::ffi::c_int != 0 {
                MFSNBD_OK as ::core::ffi::c_int
            } else {
                MFSNBD_ERROR as ::core::ffi::c_int
            }) as uint8_t,
        );
        put8bit(&raw mut wptr, msglen as uint8_t);
        unixtowrite(
            sock,
            &raw mut ans as *mut uint8_t as *const ::core::ffi::c_void,
            (10 as uint32_t).wrapping_add(msglen),
            1000 as uint32_t,
            1000 as uint32_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nbd_handle_request(mut sock: ::core::ffi::c_int) {
    unsafe {
        let mut cmd: uint32_t = 0;
        let mut leng: uint32_t = 0;
        let mut hdr: [uint8_t; 8] = [0; 8];
        let mut buff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        buff = ::core::ptr::null_mut::<uint8_t>();
        '_err: {
            if unixtoread(
                sock,
                &raw mut hdr as *mut uint8_t as *mut ::core::ffi::c_void,
                8 as uint32_t,
                READ_TOMS as uint32_t,
                READ_TOMS as uint32_t,
            ) == 8 as int32_t
            {
                rptr = &raw mut hdr as *mut uint8_t;
                cmd = get32bit(&raw mut rptr);
                leng = get32bit(&raw mut rptr);
                if leng <= 100000 as uint32_t {
                    if leng > 0 as uint32_t {
                        buff = malloc(leng as size_t) as *mut uint8_t;
                        if buff.is_null() {
                            break '_err;
                        }
                    } else {
                        buff = ::core::ptr::null_mut::<uint8_t>();
                    }
                    if unixtoread(
                        sock,
                        buff as *mut ::core::ffi::c_void,
                        leng,
                        READ_TOMS as uint32_t,
                        READ_TOMS as uint32_t,
                    ) == leng as int32_t
                    {
                        match cmd {
                            0 => {
                                nbd_handle_nop(sock, buff, leng);
                            }
                            1 => {
                                nbd_handle_stop_daemon(sock, buff, leng);
                            }
                            2 => {
                                nbd_handle_add_device(sock, buff, leng);
                            }
                            3 => {
                                nbd_handle_remove_device(sock, buff, leng);
                            }
                            4 => {
                                nbd_handle_list_devices(sock, buff, leng);
                            }
                            5 => {
                                nbd_handle_resize_device(sock, buff, leng);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        if !buff.is_null() {
            free(buff as *mut ::core::ffi::c_void);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nbd_stop_all_devices() {
    unsafe {
        static mut bdl: *mut bdlist = ::core::ptr::null_mut::<bdlist>();
        static mut bdlp: *mut *mut bdlist = ::core::ptr::null_mut::<*mut bdlist>();
        bdlp = &raw mut bdhead;
        loop {
            bdl = *bdlp;
            if bdl.is_null() {
                break;
            }
            nbd_stop((*bdl).nbdcp);
            nbd_free((*bdl).nbdcp);
            *bdlp = (*bdl).next as *mut bdlist;
            free(bdl as *mut ::core::ffi::c_void);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn password_read(
    mut filename: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut fd: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut passwordbuff: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut pbsize: size_t = 0;
        let mut i: ::core::ffi::c_int = 0;
        fd = fopen(filename, b"r\0".as_ptr() as *const ::core::ffi::c_char) as *mut FILE;
        if fd.is_null() {
            fprintf(
                stderr,
                b"error opening password file: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
            );
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        passwordbuff = ::core::ptr::null_mut::<::core::ffi::c_char>();
        pbsize = 0 as size_t;
        if getline(&raw mut passwordbuff, &raw mut pbsize, fd) == -1 as __ssize_t {
            fprintf(
                stderr,
                b"password file (%s) is empty\n\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
            );
            if !passwordbuff.is_null() {
                free(passwordbuff as *mut ::core::ffi::c_void);
            }
            fclose(fd);
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        fclose(fd);
        i = strlen(passwordbuff) as ::core::ffi::c_int;
        while i > 0 as ::core::ffi::c_int {
            i -= 1;
            if !(*passwordbuff.offset(i as isize) as ::core::ffi::c_int
                == '\n' as ::core::ffi::c_int
                || *passwordbuff.offset(i as isize) as ::core::ffi::c_int
                    == '\r' as ::core::ffi::c_int)
            {
                break;
            }
            *passwordbuff.offset(i as isize) = 0 as ::core::ffi::c_char;
        }
        if i == 0 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"first line in password file (%s) is empty\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                filename,
            );
            free(passwordbuff as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        return passwordbuff;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_option(
    mut oname: *const ::core::ffi::c_char,
    mut ovalue: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        if strcmp(oname, b"mfsmaster\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            if !mcfg.masterport.is_null() {
                free(mcfg.masterhost as *mut ::core::ffi::c_void);
            }
            mcfg.masterhost = strdup(ovalue);
        } else if strcmp(oname, b"mfsport\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            if !mcfg.masterport.is_null() {
                free(mcfg.masterport as *mut ::core::ffi::c_void);
            }
            mcfg.masterport = strdup(ovalue);
        } else if strcmp(oname, b"mfsbind\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            if !mcfg.masterbind.is_null() {
                free(mcfg.masterbind as *mut ::core::ffi::c_void);
            }
            mcfg.masterbind = strdup(ovalue);
        } else if strcmp(
            oname,
            b"mfspassword\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if !mcfg.masterpassword.is_null() {
                free(mcfg.masterpassword as *mut ::core::ffi::c_void);
            }
            mcfg.masterpassword = strdup(ovalue);
        } else if strcmp(
            oname,
            b"mfssubfolder\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if !mcfg.masterpath.is_null() {
                free(mcfg.masterpath as *mut ::core::ffi::c_void);
            }
            mcfg.masterpath = strdup(ovalue);
        } else if strcmp(
            oname,
            b"mfsioretries\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            mcfg.io_try_cnt = strtoul(
                ovalue,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int;
        } else if strcmp(
            oname,
            b"mfstimeout\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            mcfg.io_timeout = strtoul(
                ovalue,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int;
        } else if strcmp(
            oname,
            b"mfslogretry\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            mcfg.min_log_entry = strtoul(
                ovalue,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int;
        } else if strcmp(
            oname,
            b"mfswritecachesize\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            mcfg.write_cache_mb = strtoul(
                ovalue,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int;
        } else if strcmp(
            oname,
            b"mfsreadaheadsize\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            mcfg.read_cache_mb = strtoul(
                ovalue,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int;
        } else if strcmp(
            oname,
            b"mfsreadaheadleng\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            mcfg.readahead_leng = strtoul(
                ovalue,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int;
        } else if strcmp(
            oname,
            b"mfsreadaheadtrigger\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            mcfg.readahead_trigger = strtoul(
                ovalue,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int;
        } else if strcmp(
            oname,
            b"mfserroronlostchunk\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if *ovalue != 0 {
                printf(
                    b"value %s not used in option mfserroronlostchunk\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ovalue,
                );
            }
            mcfg.error_on_lost_chunk = 1 as ::core::ffi::c_int;
        } else if strcmp(
            oname,
            b"mfserroronnospace\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if *ovalue != 0 {
                printf(
                    b"value %s not used in option mfserroronnospace\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ovalue,
                );
            }
            mcfg.error_on_no_space = 1 as ::core::ffi::c_int;
        } else if strcmp(
            oname,
            b"mfspreflabels\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if !mcfg.preferedlabels.is_null() {
                free(mcfg.preferedlabels as *mut ::core::ffi::c_void);
            }
            mcfg.preferedlabels = strdup(ovalue);
        } else if strcmp(
            oname,
            b"mfsnbdtimeout\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            NbdTimeout = strtoul(
                ovalue,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int;
        } else {
            fprintf(
                stderr,
                b"unrecognized option: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                oname,
            );
            return -1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_options(
    mut optstring: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ostrcopy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut option: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut osptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut optr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut oname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut ovalue: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut onend: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut ovend: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        ostrcopy = strdup(optstring);
        osptr = ostrcopy;
        loop {
            option = strsep(
                &raw mut osptr,
                b",\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if option.is_null() {
                break;
            }
            optr = option;
            while *optr as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                || *optr as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
            {
                optr = optr.offset(1);
            }
            oname = optr;
            while *optr as ::core::ffi::c_int != 0
                && *optr as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
                && *optr as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
                && *optr as ::core::ffi::c_int != '=' as ::core::ffi::c_int
            {
                optr = optr.offset(1);
            }
            onend = optr;
            while *optr as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                || *optr as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
            {
                optr = optr.offset(1);
            }
            if *optr as ::core::ffi::c_int == '=' as ::core::ffi::c_int {
                optr = optr.offset(1);
                while *optr as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                    || *optr as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
                {
                    optr = optr.offset(1);
                }
                ovalue = optr;
                while *optr as ::core::ffi::c_int != 0
                    && *optr as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
                    && *optr as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
                {
                    optr = optr.offset(1);
                }
                ovend = optr;
                while *optr as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                    || *optr as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
                {
                    optr = optr.offset(1);
                }
            } else {
                ovalue = optr;
                ovend = optr;
            }
            if *optr as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                printf(
                    b"option malformed: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    option,
                );
                free(ostrcopy as *mut ::core::ffi::c_void);
                return -1 as ::core::ffi::c_int;
            }
            if oname == onend {
                continue;
            }
            *onend = 0 as ::core::ffi::c_char;
            *ovend = 0 as ::core::ffi::c_char;
            if parse_option(oname, ovalue) < 0 as ::core::ffi::c_int {
                free(ostrcopy as *mut ::core::ffi::c_void);
                return -1 as ::core::ffi::c_int;
            }
        }
        free(ostrcopy as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn usage(mut appname: *const ::core::ffi::c_char) {
    unsafe {
        fprintf(stderr, b"usage:\n\0".as_ptr() as *const ::core::ffi::c_char);
        fprintf(
            stderr,
            b"\tstart daemon:   %s start [ -H masterhost ] [ -P masterport ] [ -B masterbind ] [ -S masterpath ] [ -p masterpassword | -x passwordfile ] [ -l link_socket_name ] [ -i init_mappings_file ] [ -o options ]\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            appname,
        );
        fprintf(
            stderr,
            b"\tstop daemon:    %s stop [ -l link_socket_name ]\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            appname,
        );
        fprintf(
            stderr,
            b"\tadd mapping:    %s map [ -l link_socket_name ] -f mfsfile [ -d /dev/nbdX ] [ -n linkname ] [ -s bdevsize ] [ -b blocksize | -5 | -1 | -2 | -4 ] [-r] [-i]\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            appname,
        );
        fprintf(
            stderr,
            b"\tdelete mapping: %s unmap [ -l link_socket_name ] [ -f mfsfile ] [ -d /dev/nbdX ] [ -n linkname ]\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            appname,
        );
        fprintf(
            stderr,
            b"\tlist mappings:  %s list [ -l link_socket_name ] [ -t m|u|i ]\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            appname,
        );
        fprintf(
            stderr,
            b"\tchange size:    %s resize [ -l link_socket_name ] ( -f mfsfile | -d /dev/nbdX ) [ -s bdevsize ]\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            appname,
        );
        fprintf(
            stderr,
            b"\noptions:\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"\tmfswritecachesize=N      define size of write cache in MiB (default: 128)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"\tmfsreadaheadsize=N       define size of all read ahead buffers in MiB (default: 128)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"\tmfsreadaheadleng=N       define amount of bytes to be additionally read (default: 2097152 = 2MiB)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"\tmfsreadaheadtrigger=N    define amount of bytes read sequentially that turns on read ahead (default: 20971520)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"\tmfserroronlostchunk      when all known chunkservers are connected to the master and the required chunk is missing then immediately finish I/O and return an error\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"\tmfserroronnospace        when all known chunkservers are connected to the master and there is no free space then immediately finish I/O and return an error\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"\tmfsioretries=N           define number of retries before I/O error is returned (default: 30)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"\tmfstimeout=N             define maximum timeout in seconds before I/O error is returned (default: 0 - which means no timeout)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"\tmfslogretry=N            define minimal retry counter on which system will start log I/O messages (default: 5)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"\tmfsmaster=HOST           define mfsmaster location (default: mfsmaster)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"\tmfsport=PORT             define mfsmaster port number (default: 9421)\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"\tmfsbind=IP               define source ip address for connections (default: NOT DEFINED - chosen automatically by OS)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"\tmfssubfolder=PATH        define subfolder to mount as root (default: /)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"\tmfspassword=PASSWORD     authenticate to mfsmaster with given password\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"\tmfspreflabels=LABELEXPR  specify preferred labels for choosing chunkservers during I/O\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            stderr,
            b"\tmfsnbdtimeout=N          define maximum timeout in seconds before kernel gives up (default: 1800)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nbd_start_daemon(
    mut appname: *const ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut passfile: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut lsockname: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut initfname: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut fg: ::core::ffi::c_int = 0;
        let mut ch: ::core::ffi::c_int = 0;
        let mut lsock: ::core::ffi::c_int = 0;
        let mut argc_back: ::core::ffi::c_int = 0;
        let mut argv_back: *mut *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        argc_back = argc;
        argv_back = argv as *mut *mut ::core::ffi::c_char;
        argc -= 1;
        argv = argv.offset(1);
        mfs_set_defaults(&raw mut mcfg);
        passfile = ::core::ptr::null_mut::<::core::ffi::c_char>();
        lsockname = ::core::ptr::null_mut::<::core::ffi::c_char>();
        initfname = ::core::ptr::null_mut::<::core::ffi::c_char>();
        fg = 0 as ::core::ffi::c_int;
        bdhead = ::core::ptr::null_mut::<bdlist>();
        loop {
            ch = getopt(
                argc,
                argv as *const *mut ::core::ffi::c_char,
                b"H:P:B:S:p:x:l:i:o:Fh?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if ch == -1 as ::core::ffi::c_int {
                break;
            }
            match ch {
                72 => {
                    if !mcfg.masterhost.is_null() {
                        free(mcfg.masterhost as *mut ::core::ffi::c_void);
                    }
                    mcfg.masterhost = strdup(optarg);
                }
                80 => {
                    if !mcfg.masterport.is_null() {
                        free(mcfg.masterport as *mut ::core::ffi::c_void);
                    }
                    mcfg.masterport = strdup(optarg);
                }
                66 => {
                    if !mcfg.masterbind.is_null() {
                        free(mcfg.masterbind as *mut ::core::ffi::c_void);
                    }
                    mcfg.masterbind = strdup(optarg);
                }
                83 => {
                    if !mcfg.masterpath.is_null() {
                        free(mcfg.masterpath as *mut ::core::ffi::c_void);
                    }
                    mcfg.masterpath = strdup(optarg);
                }
                112 => {
                    if !mcfg.masterpassword.is_null() {
                        free(mcfg.masterpassword as *mut ::core::ffi::c_void);
                    }
                    mcfg.masterpassword = strdup(optarg);
                }
                120 => {
                    if !passfile.is_null() {
                        free(passfile as *mut ::core::ffi::c_void);
                    }
                    passfile = strdup(optarg);
                }
                108 => {
                    if !lsockname.is_null() {
                        free(lsockname as *mut ::core::ffi::c_void);
                    }
                    lsockname = strdup(optarg);
                }
                105 => {
                    if !initfname.is_null() {
                        free(initfname as *mut ::core::ffi::c_void);
                    }
                    initfname = strdup(optarg);
                }
                111 => {
                    if parse_options(optarg) < 0 as ::core::ffi::c_int {
                        usage(appname);
                        return 1 as ::core::ffi::c_int;
                    }
                }
                70 => {
                    fg = 1 as ::core::ffi::c_int;
                }
                104 | _ => {
                    usage(appname);
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
        if find_free_nbddevice().is_null() {
            if *__errno_location() == ENOENT {
                fprintf(
                    stderr,
                    b"no /dev/nbdX devices present - likely nbd kernel module should be loaded (try: modprobe nbd)\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            } else if *__errno_location() == EACCES {
                fprintf(
                    stderr,
                    b"permission denied accessing /dev/nbdX - try to start with root privileges\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                fprintf(
                    stderr,
                    b"can't find free NBD device\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            return 1 as ::core::ffi::c_int;
        }
        if !passfile.is_null() {
            if !mcfg.masterpassword.is_null() {
                fprintf(
                    stderr,
                    b"options '-p' and '-x' are mutually exclusive\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return 1 as ::core::ffi::c_int;
            }
            mcfg.masterpassword = password_read(passfile);
            if mcfg.masterpassword.is_null() {
                return 1 as ::core::ffi::c_int;
            }
        }
        if mcfg.write_cache_mb < 16 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"write cache size too low (%u MiB) - increased to 16 MiB\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mcfg.write_cache_mb,
            );
            mcfg.write_cache_mb = 16 as ::core::ffi::c_int;
        }
        if mcfg.write_cache_mb > 2048 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"write cache size too big (%u MiB) - decresed to 2048 MiB\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mcfg.write_cache_mb,
            );
            mcfg.write_cache_mb = 2048 as ::core::ffi::c_int;
        }
        if mcfg.read_cache_mb < 16 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"read ahead size too low (%u MiB) - increased to 16 MiB\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mcfg.read_cache_mb,
            );
            mcfg.read_cache_mb = 16 as ::core::ffi::c_int;
        }
        if mcfg.read_cache_mb > 2048 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"read ahead size too big (%u MiB) - decresed to 2048 MiB\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mcfg.read_cache_mb,
            );
            mcfg.read_cache_mb = 2048 as ::core::ffi::c_int;
        }
        if mcfg.readahead_leng < 0x20000 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"read ahead length too low (%u B) - increased to 128 KiB\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mcfg.readahead_leng,
            );
            mcfg.readahead_leng = 0x20000 as ::core::ffi::c_int;
        }
        if mcfg.readahead_leng > 0x200000 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"read ahead length too big (%u B) - decresed to 2 MiB\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                mcfg.readahead_leng,
            );
            mcfg.readahead_leng = 0x200000 as ::core::ffi::c_int;
        }
        processname_init(argc_back, argv_back as *mut *mut ::core::ffi::c_char);
        if !mcfg.logident.is_null() {
            free(mcfg.logident as *mut ::core::ffi::c_void);
        }
        mcfg.logident = strdup(b"mfsblockdev\0".as_ptr() as *const ::core::ffi::c_char);
        mcfg.logdaemon = 1 as ::core::ffi::c_int;
        if !mcfg.mountpoint.is_null() {
            free(mcfg.mountpoint as *mut ::core::ffi::c_void);
        }
        mcfg.mountpoint = strdup(b"[NBD]\0".as_ptr() as *const ::core::ffi::c_char);
        if lsockname.is_null() {
            lsockname = strdup(b"/dev/mfs/nbdsock\0".as_ptr() as *const ::core::ffi::c_char);
            mkdir(NBD_LINK_PREFIX.as_ptr(), 0o777 as __mode_t);
        }
        lsock = unixsocket();
        if unixlisten(lsock, lsockname, 5 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            if *__errno_location() == EADDRINUSE {
                let mut csock: ::core::ffi::c_int = 0;
                csock = unixsocket();
                if unixconnect(csock, lsockname) < 0 as ::core::ffi::c_int {
                    if *__errno_location() == ECONNREFUSED {
                        unlink(lsockname);
                    }
                } else {
                    close(csock);
                }
            }
            if unixlisten(lsock, lsockname, 5 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"error creating unix socket '%s': %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    lsockname,
                    strerror(*__errno_location()),
                );
                free(lsockname as *mut ::core::ffi::c_void);
                return 1 as ::core::ffi::c_int;
            }
        }
        if mfs_init(&raw mut mcfg, 1 as uint8_t) < 0 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"can't connect to master\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            close(lsock);
            unlink(lsockname);
            return 1 as ::core::ffi::c_int;
        }
        term = 0 as uint8_t;
        if fg == 0 as ::core::ffi::c_int {
            make_daemon();
        } else {
            set_signals();
        }
        if mfs_init(&raw mut mcfg, 2 as uint8_t) < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"can't initialize MFS\0".as_ptr() as *const ::core::ffi::c_char,
            );
            close(lsock);
            unlink(lsockname);
            free(lsockname as *mut ::core::ffi::c_void);
            return 1 as ::core::ffi::c_int;
        }
        workers_set = workers_init(
            150 as uint32_t,
            30 as uint32_t,
            0 as uint32_t,
            b"nbd\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            Some(nbd_worker_fn as unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> ()),
        );
        if !initfname.is_null() {
            if nbd_auto_maps(initfname) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                term = 1 as uint8_t;
            }
        }
        if fg == 0 as ::core::ffi::c_int {
            let mut f: ::core::ffi::c_int = 0;
            f = open(
                b"/dev/null\0".as_ptr() as *const ::core::ffi::c_char,
                O_RDWR,
                0 as ::core::ffi::c_int,
            );
            if dup2(f, STDERR_FILENO) < 0 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"dup2 error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    strerror(*__errno_location()),
                );
                term = 1 as uint8_t;
            }
            close(f);
        }
        if term as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut pname: [::core::ffi::c_char; 256] = [0; 256];
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_INFO,
                b"main loop start\0".as_ptr() as *const ::core::ffi::c_char,
            );
            snprintf(
                &raw mut pname as *mut ::core::ffi::c_char,
                256 as size_t,
                b"mfsbdev (daemon cmdlink:%s)\0".as_ptr() as *const ::core::ffi::c_char,
                lsockname,
            );
            pname[255 as usize] = 0 as ::core::ffi::c_char;
            processname_set(&raw mut pname as *mut ::core::ffi::c_char);
        }
        while term as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut csock_0: ::core::ffi::c_int = 0;
            csock_0 = unixtoaccept(lsock, 100 as uint32_t);
            if csock_0 >= 0 as ::core::ffi::c_int {
                nbd_handle_request(csock_0);
                close(csock_0);
            } else if *__errno_location() != ETIMEDOUT {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"accept returned: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    strerror(*__errno_location()),
                );
            }
        }
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_INFO,
            b"got term signal - closing nbd devices\0".as_ptr() as *const ::core::ffi::c_char,
        );
        nbd_stop_all_devices();
        if bdhead.is_null() {
        } else {
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr() as *const ::core::ffi::c_char,
                1977 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"bdhead==NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"structures not cleared properly\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s' : %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr() as *const ::core::ffi::c_char,
                1977 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"bdhead==NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"structures not cleared properly\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        workers_term(workers_set);
        mfs_term();
        close(lsock);
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_INFO,
            b"removing socket file '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            lsockname,
        );
        if unlink(lsockname) < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"can't unlink socket '%s': %s\0".as_ptr() as *const ::core::ffi::c_char,
                lsockname,
                strerror(*__errno_location()),
            );
        }
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_INFO,
            b"socket file '%s' removed\0".as_ptr() as *const ::core::ffi::c_char,
            lsockname,
        );
        free(lsockname as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nbd_stop_daemon(
    mut appname: *const ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut buff: [uint8_t; 8] = [0; 8];
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut lsockname: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut ch: ::core::ffi::c_int = 0;
        let mut csock: ::core::ffi::c_int = 0;
        let mut cmd: uint32_t = 0;
        let mut leng: uint32_t = 0;
        let mut res: ::core::ffi::c_int = 0;
        let mut cnt: ::core::ffi::c_int = 0;
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
        csock = -1 as ::core::ffi::c_int;
        lsockname = ::core::ptr::null_mut::<::core::ffi::c_char>();
        res = 1 as ::core::ffi::c_int;
        loop {
            ch = getopt(
                argc,
                argv as *const *mut ::core::ffi::c_char,
                b"l:?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if ch == -1 as ::core::ffi::c_int {
                break;
            }
            match ch {
                108 => {
                    if !lsockname.is_null() {
                        free(lsockname as *mut ::core::ffi::c_void);
                    }
                    lsockname = strdup(optarg);
                }
                104 | _ => {
                    usage(appname);
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
        if lsockname.is_null() {
            lsockname = strdup(b"/dev/mfs/nbdsock\0".as_ptr() as *const ::core::ffi::c_char);
        }
        csock = unixsocket();
        '_err: {
            if unixtoconnect(csock, lsockname, 1000 as uint32_t) < 0 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"can't connect to socket '%s': %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    lsockname,
                    strerror(*__errno_location()),
                );
            } else {
                wptr = &raw mut buff as *mut uint8_t;
                put32bit(&raw mut wptr, MFSNBD_STOP as ::core::ffi::c_int as uint32_t);
                put32bit(&raw mut wptr, 0 as uint32_t);
                if unixtowrite(
                    csock,
                    &raw mut buff as *mut uint8_t as *const ::core::ffi::c_void,
                    8 as uint32_t,
                    1000 as uint32_t,
                    1000 as uint32_t,
                ) != 8 as int32_t
                {
                    fprintf(
                        stderr,
                        b"unable to send data to '%s': %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        lsockname,
                        strerror(*__errno_location()),
                    );
                } else {
                    memset(
                        &raw mut buff as *mut uint8_t as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        8 as size_t,
                    );
                    if unixtoread(
                        csock,
                        &raw mut buff as *mut uint8_t as *mut ::core::ffi::c_void,
                        8 as uint32_t,
                        5000 as uint32_t,
                        5000 as uint32_t,
                    ) != 8 as int32_t
                    {
                        fprintf(
                            stderr,
                            b"error receiving data from '%s': %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            lsockname,
                            strerror(*__errno_location()),
                        );
                    } else {
                        rptr = &raw mut buff as *mut uint8_t;
                        cmd = get32bit(&raw mut rptr);
                        leng = get32bit(&raw mut rptr);
                        if cmd != MFSNBD_STOP as ::core::ffi::c_int as uint32_t {
                            fprintf(
                                stderr,
                                b"got wrong answer from '%s': Bad Command\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                lsockname,
                            );
                        } else if leng != 1 as uint32_t {
                            fprintf(
                                stderr,
                                b"got wrong answer from '%s': Wrong Size\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                lsockname,
                            );
                        } else if unixtoread(
                            csock,
                            &raw mut buff as *mut uint8_t as *mut ::core::ffi::c_void,
                            1 as uint32_t,
                            1000 as uint32_t,
                            1000 as uint32_t,
                        ) != 1 as int32_t
                        {
                            fprintf(
                                stderr,
                                b"error receiving data from '%s': %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                lsockname,
                                strerror(*__errno_location()),
                            );
                        } else {
                            close(csock);
                            csock = -1 as ::core::ffi::c_int;
                            if buff[0 as usize] as ::core::ffi::c_int
                                != MFSNBD_OK as ::core::ffi::c_int
                            {
                                fprintf(
                                    stderr,
                                    b"error stopping daemon on '%s'\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    lsockname,
                                );
                            } else {
                                printf(b"daemon received STOP command\n\0".as_ptr()
                                    as *const ::core::ffi::c_char);
                                printf(b"waiting for daemon ...\0".as_ptr()
                                    as *const ::core::ffi::c_char);
                                fflush(stdout);
                                cnt = 0 as ::core::ffi::c_int;
                                while stat(lsockname, &raw mut st) == 0 as ::core::ffi::c_int {
                                    usleep(10000 as __useconds_t);
                                    cnt += 1;
                                    if cnt % 100 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                                        printf(b".\0".as_ptr() as *const ::core::ffi::c_char);
                                        fflush(stdout);
                                    }
                                    if cnt <= 1000 as ::core::ffi::c_int {
                                        continue;
                                    }
                                    printf(b" giving up\n\0".as_ptr() as *const ::core::ffi::c_char);
                                    break '_err;
                                }
                                printf(b" ok\n\0".as_ptr() as *const ::core::ffi::c_char);
                                res = 0 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
        }
        if csock >= 0 as ::core::ffi::c_int {
            close(csock);
        }
        if !lsockname.is_null() {
            free(lsockname as *mut ::core::ffi::c_void);
        }
        return res;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nbd_add_mapping(
    mut appname: *const ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut buff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut device: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut linkname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut size: uint64_t = 0;
        let mut flags: uint32_t = 0;
        let mut bsize: uint32_t = 0;
        let mut dsize: uint32_t = 0;
        let mut pleng: uint32_t = 0;
        let mut dleng: uint32_t = 0;
        let mut nleng: uint32_t = 0;
        let mut aleng: uint8_t = 0;
        let mut status: uint8_t = 0;
        let mut answer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut lsockname: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut ch: ::core::ffi::c_int = 0;
        let mut csock: ::core::ffi::c_int = 0;
        let mut res: ::core::ffi::c_int = 0;
        let mut cmd: uint32_t = 0;
        let mut leng: uint32_t = 0;
        csock = -1 as ::core::ffi::c_int;
        buff = ::core::ptr::null_mut::<uint8_t>();
        lsockname = ::core::ptr::null_mut::<::core::ffi::c_char>();
        filename = ::core::ptr::null_mut::<::core::ffi::c_char>();
        device = ::core::ptr::null_mut::<::core::ffi::c_char>();
        linkname = ::core::ptr::null_mut::<::core::ffi::c_char>();
        answer = ::core::ptr::null_mut::<::core::ffi::c_char>();
        size = 0 as uint64_t;
        bsize = 4096 as uint32_t;
        flags = 0 as uint32_t;
        res = 1 as ::core::ffi::c_int;
        loop {
            ch = getopt(
                argc,
                argv as *const *mut ::core::ffi::c_char,
                b"l:f:d:n:s:b:5124ri?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if ch == -1 as ::core::ffi::c_int {
                break;
            }
            match ch {
                108 => {
                    if !lsockname.is_null() {
                        free(lsockname as *mut ::core::ffi::c_void);
                    }
                    lsockname = strdup(optarg);
                }
                102 => {
                    if !filename.is_null() {
                        free(filename as *mut ::core::ffi::c_void);
                    }
                    filename = strdup(optarg);
                }
                100 => {
                    if !device.is_null() {
                        free(device as *mut ::core::ffi::c_void);
                    }
                    device = strdup(optarg);
                }
                110 => {
                    if !linkname.is_null() {
                        free(linkname as *mut ::core::ffi::c_void);
                    }
                    if strlen(optarg) > NBD_LINK_PREFIX_LENG as size_t
                        && memcmp(
                            optarg as *const ::core::ffi::c_void,
                            NBD_LINK_PREFIX.as_ptr() as *const ::core::ffi::c_void,
                            NBD_LINK_PREFIX_LENG as size_t,
                        ) == 0 as ::core::ffi::c_int
                    {
                        linkname = strdup(optarg.offset(NBD_LINK_PREFIX_LENG as isize));
                    } else {
                        linkname = strdup(optarg);
                    }
                }
                115 => {
                    size = sizestrtod(
                        optarg,
                        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                    ) as uint64_t;
                }
                98 => {
                    bsize = sizestrtod(
                        optarg,
                        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                    ) as uint32_t;
                }
                53 => {
                    bsize = 512 as uint32_t;
                }
                49 => {
                    bsize = 1024 as uint32_t;
                }
                50 => {
                    bsize = 2048 as uint32_t;
                }
                52 => {
                    bsize = 4096 as uint32_t;
                }
                114 => {
                    flags |= FLAG_READONLY as uint32_t;
                }
                105 => {
                    flags |= FLAG_IGNORELOCK as uint32_t;
                }
                104 | _ => {
                    usage(appname);
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
        '_err: {
            if filename.is_null() {
                fprintf(
                    stderr,
                    b"MFS file name (option -f) not specified\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            } else {
                if lsockname.is_null() {
                    lsockname =
                        strdup(b"/dev/mfs/nbdsock\0".as_ptr() as *const ::core::ffi::c_char);
                }
                csock = unixsocket();
                if unixtoconnect(csock, lsockname, 1000 as uint32_t) < 0 as ::core::ffi::c_int {
                    fprintf(
                        stderr,
                        b"can't connect to socket '%s': %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        lsockname,
                        strerror(*__errno_location()),
                    );
                } else {
                    pleng = strlen(filename) as uint32_t;
                    if pleng > 65535 as uint32_t {
                        fprintf(
                            stderr,
                            b"MFS file name too long\n\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                    } else {
                        if !device.is_null() {
                            dleng = strlen(device) as uint32_t;
                            if dleng > 255 as uint32_t {
                                fprintf(
                                    stderr,
                                    b"device name too long\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                break '_err;
                            }
                        } else {
                            dleng = 0 as uint32_t;
                        }
                        if !linkname.is_null() {
                            nleng = strlen(linkname) as uint32_t;
                            if nleng > 255 as uint32_t {
                                fprintf(
                                    stderr,
                                    b"link name too long\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                break '_err;
                            }
                        } else {
                            nleng = 0 as uint32_t;
                        }
                        dsize = (20 as uint32_t)
                            .wrapping_add(pleng)
                            .wrapping_add(dleng)
                            .wrapping_add(nleng);
                        buff =
                            malloc((8 as uint32_t).wrapping_add(dsize) as size_t) as *mut uint8_t;
                        if buff.is_null() {
                            fprintf(
                                stderr,
                                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                2238 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                2238 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            abort();
                        } else if buff
                            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                                -1 as ::core::ffi::c_int as usize,
                            ) as *mut uint8_t
                        {
                            let mut _mfs_errorstring: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                2238 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                                _mfs_errorstring,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                2238 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                                _mfs_errorstring,
                            );
                            abort();
                        }
                        wptr = buff;
                        put32bit(&raw mut wptr, MFSNBD_ADD as ::core::ffi::c_int as uint32_t);
                        put32bit(&raw mut wptr, dsize);
                        put16bit(&raw mut wptr, pleng as uint16_t);
                        memcpy(
                            wptr as *mut ::core::ffi::c_void,
                            filename as *const ::core::ffi::c_void,
                            pleng as size_t,
                        );
                        wptr = wptr.offset(pleng as isize);
                        put8bit(&raw mut wptr, dleng as uint8_t);
                        if dleng > 0 as uint32_t {
                            memcpy(
                                wptr as *mut ::core::ffi::c_void,
                                device as *const ::core::ffi::c_void,
                                dleng as size_t,
                            );
                            wptr = wptr.offset(dleng as isize);
                        }
                        put8bit(&raw mut wptr, nleng as uint8_t);
                        if nleng > 0 as uint32_t {
                            memcpy(
                                wptr as *mut ::core::ffi::c_void,
                                linkname as *const ::core::ffi::c_void,
                                nleng as size_t,
                            );
                            wptr = wptr.offset(nleng as isize);
                        }
                        put64bit(&raw mut wptr, size);
                        put32bit(&raw mut wptr, bsize);
                        put32bit(&raw mut wptr, flags);
                        if unixtowrite(
                            csock,
                            buff as *const ::core::ffi::c_void,
                            (8 as uint32_t).wrapping_add(dsize),
                            1000 as uint32_t,
                            1000 as uint32_t,
                        ) as ssize_t
                            != (8 as uint32_t).wrapping_add(dsize) as ssize_t
                        {
                            fprintf(
                                stderr,
                                b"unable to send data to '%s': %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                lsockname,
                                strerror(*__errno_location()),
                            );
                        } else {
                            memset(
                                buff as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                8 as size_t,
                            );
                            if unixtoread(
                                csock,
                                buff as *mut ::core::ffi::c_void,
                                8 as uint32_t,
                                5000 as uint32_t,
                                5000 as uint32_t,
                            ) != 8 as int32_t
                            {
                                fprintf(
                                    stderr,
                                    b"error receiving data from '%s': %s\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    lsockname,
                                    strerror(*__errno_location()),
                                );
                            } else {
                                rptr = buff;
                                cmd = get32bit(&raw mut rptr);
                                leng = get32bit(&raw mut rptr);
                                if cmd != MFSNBD_ADD as ::core::ffi::c_int as uint32_t {
                                    fprintf(
                                        stderr,
                                        b"got wrong answer from '%s': Bad Command\n\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        lsockname,
                                    );
                                } else if leng < 2 as uint32_t
                                    || leng > (2 as ::core::ffi::c_int + NBD_ERR_SIZE) as uint32_t
                                {
                                    fprintf(
                                        stderr,
                                        b"got wrong answer from '%s': Wrong Size\n\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        lsockname,
                                    );
                                } else {
                                    free(buff as *mut ::core::ffi::c_void);
                                    buff = malloc(leng as size_t) as *mut uint8_t;
                                    if buff.is_null() {
                                        fprintf(
                                            stderr,
                                            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            2288 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                                        );
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_ERR,
                                            b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            2288 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                                        );
                                        abort();
                                    } else if buff
                                        == ::core::ptr::with_exposed_provenance_mut::<
                                            ::core::ffi::c_void,
                                        >(
                                            -1 as ::core::ffi::c_int as usize
                                        ) as *mut uint8_t
                                    {
                                        let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                                            strerr(*__errno_location());
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_ERR,
                                            b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            2288 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                                            _mfs_errorstring_0,
                                        );
                                        fprintf(
                                            stderr,
                                            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            2288 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                            b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                                            _mfs_errorstring_0,
                                        );
                                        abort();
                                    }
                                    if unixtoread(
                                        csock,
                                        buff as *mut ::core::ffi::c_void,
                                        leng,
                                        1000 as uint32_t,
                                        1000 as uint32_t,
                                    ) as ssize_t
                                        != leng as ssize_t
                                    {
                                        fprintf(
                                            stderr,
                                            b"error receiving data from '%s': %s\n\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            lsockname,
                                            strerror(*__errno_location()),
                                        );
                                    } else {
                                        close(csock);
                                        csock = -1 as ::core::ffi::c_int;
                                        rptr = buff;
                                        status = get8bit(&raw mut rptr);
                                        aleng = get8bit(&raw mut rptr);
                                        answer = nbd_packet_to_str(rptr, aleng as uint32_t);
                                        if !answer.is_null() {
                                            if status as ::core::ffi::c_int
                                                == MFSNBD_OK as ::core::ffi::c_int
                                            {
                                                printf(
                                                    b"%s\n\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    answer,
                                                );
                                            } else {
                                                fprintf(
                                                    stderr,
                                                    b"%s\n\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    answer,
                                                );
                                                break '_err;
                                            }
                                        }
                                        res = 0 as ::core::ffi::c_int;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if csock >= 0 as ::core::ffi::c_int {
            close(csock);
        }
        if !lsockname.is_null() {
            free(lsockname as *mut ::core::ffi::c_void);
        }
        if !filename.is_null() {
            free(filename as *mut ::core::ffi::c_void);
        }
        if !device.is_null() {
            free(device as *mut ::core::ffi::c_void);
        }
        if !linkname.is_null() {
            free(linkname as *mut ::core::ffi::c_void);
        }
        if !answer.is_null() {
            free(answer as *mut ::core::ffi::c_void);
        }
        if !buff.is_null() {
            free(buff as *mut ::core::ffi::c_void);
        }
        return res;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nbd_remove_mapping(
    mut appname: *const ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut buff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut device: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut linkname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut dsize: uint32_t = 0;
        let mut pleng: uint32_t = 0;
        let mut dleng: uint32_t = 0;
        let mut nleng: uint32_t = 0;
        let mut aleng: uint8_t = 0;
        let mut status: uint8_t = 0;
        let mut answer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut lsockname: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut ch: ::core::ffi::c_int = 0;
        let mut csock: ::core::ffi::c_int = 0;
        let mut res: ::core::ffi::c_int = 0;
        let mut cmd: uint32_t = 0;
        let mut leng: uint32_t = 0;
        csock = -1 as ::core::ffi::c_int;
        buff = ::core::ptr::null_mut::<uint8_t>();
        lsockname = ::core::ptr::null_mut::<::core::ffi::c_char>();
        filename = ::core::ptr::null_mut::<::core::ffi::c_char>();
        device = ::core::ptr::null_mut::<::core::ffi::c_char>();
        linkname = ::core::ptr::null_mut::<::core::ffi::c_char>();
        answer = ::core::ptr::null_mut::<::core::ffi::c_char>();
        res = 1 as ::core::ffi::c_int;
        loop {
            ch = getopt(
                argc,
                argv as *const *mut ::core::ffi::c_char,
                b"l:f:d:n:?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if ch == -1 as ::core::ffi::c_int {
                break;
            }
            match ch {
                108 => {
                    if !lsockname.is_null() {
                        free(lsockname as *mut ::core::ffi::c_void);
                    }
                    lsockname = strdup(optarg);
                }
                102 => {
                    if !filename.is_null() {
                        free(filename as *mut ::core::ffi::c_void);
                    }
                    filename = strdup(optarg);
                }
                100 => {
                    if !device.is_null() {
                        free(device as *mut ::core::ffi::c_void);
                    }
                    device = strdup(optarg);
                }
                110 => {
                    if !linkname.is_null() {
                        free(linkname as *mut ::core::ffi::c_void);
                    }
                    if strlen(optarg) > NBD_LINK_PREFIX_LENG as size_t
                        && memcmp(
                            optarg as *const ::core::ffi::c_void,
                            NBD_LINK_PREFIX.as_ptr() as *const ::core::ffi::c_void,
                            NBD_LINK_PREFIX_LENG as size_t,
                        ) == 0 as ::core::ffi::c_int
                    {
                        linkname = strdup(optarg.offset(NBD_LINK_PREFIX_LENG as isize));
                    } else {
                        linkname = strdup(optarg);
                    }
                }
                104 | _ => {
                    usage(appname);
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
        '_err: {
            if filename.is_null() && device.is_null() && linkname.is_null() {
                fprintf(
                    stderr,
                    b"device not specified\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                if lsockname.is_null() {
                    lsockname =
                        strdup(b"/dev/mfs/nbdsock\0".as_ptr() as *const ::core::ffi::c_char);
                }
                csock = unixsocket();
                if unixtoconnect(csock, lsockname, 1000 as uint32_t) < 0 as ::core::ffi::c_int {
                    fprintf(
                        stderr,
                        b"can't connect to socket '%s': %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        lsockname,
                        strerror(*__errno_location()),
                    );
                } else {
                    if !filename.is_null() {
                        pleng = strlen(filename) as uint32_t;
                        if pleng > 65535 as uint32_t {
                            fprintf(
                                stderr,
                                b"MFS file name too long\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            break '_err;
                        }
                    } else {
                        pleng = 0 as uint32_t;
                    }
                    if !device.is_null() {
                        dleng = strlen(device) as uint32_t;
                        if dleng > 255 as uint32_t {
                            fprintf(
                                stderr,
                                b"device name too long\n\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            break '_err;
                        }
                    } else {
                        dleng = 0 as uint32_t;
                    }
                    if !linkname.is_null() {
                        nleng = strlen(linkname) as uint32_t;
                        if nleng > 255 as uint32_t {
                            fprintf(
                                stderr,
                                b"link name too long\n\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            break '_err;
                        }
                    } else {
                        nleng = 0 as uint32_t;
                    }
                    dsize = (4 as uint32_t)
                        .wrapping_add(pleng)
                        .wrapping_add(dleng)
                        .wrapping_add(nleng);
                    buff = malloc((8 as uint32_t).wrapping_add(dsize) as size_t) as *mut uint8_t;
                    if buff.is_null() {
                        fprintf(
                            stderr,
                            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            2441 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            2441 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        abort();
                    } else if buff
                        == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                            -1 as ::core::ffi::c_int as usize,
                        ) as *mut uint8_t
                    {
                        let mut _mfs_errorstring: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            2441 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            2441 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring,
                        );
                        abort();
                    }
                    wptr = buff;
                    put32bit(
                        &raw mut wptr,
                        MFSNBD_REMOVE as ::core::ffi::c_int as uint32_t,
                    );
                    put32bit(&raw mut wptr, dsize);
                    put16bit(&raw mut wptr, pleng as uint16_t);
                    if pleng > 0 as uint32_t {
                        memcpy(
                            wptr as *mut ::core::ffi::c_void,
                            filename as *const ::core::ffi::c_void,
                            pleng as size_t,
                        );
                        wptr = wptr.offset(pleng as isize);
                    }
                    put8bit(&raw mut wptr, dleng as uint8_t);
                    if dleng > 0 as uint32_t {
                        memcpy(
                            wptr as *mut ::core::ffi::c_void,
                            device as *const ::core::ffi::c_void,
                            dleng as size_t,
                        );
                        wptr = wptr.offset(dleng as isize);
                    }
                    put8bit(&raw mut wptr, nleng as uint8_t);
                    if nleng > 0 as uint32_t {
                        memcpy(
                            wptr as *mut ::core::ffi::c_void,
                            linkname as *const ::core::ffi::c_void,
                            nleng as size_t,
                        );
                        wptr = wptr.offset(nleng as isize);
                    }
                    if unixtowrite(
                        csock,
                        buff as *const ::core::ffi::c_void,
                        (8 as uint32_t).wrapping_add(dsize),
                        1000 as uint32_t,
                        1000 as uint32_t,
                    ) as ssize_t
                        != (8 as uint32_t).wrapping_add(dsize) as ssize_t
                    {
                        fprintf(
                            stderr,
                            b"unable to send data to '%s': %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            lsockname,
                            strerror(*__errno_location()),
                        );
                    } else {
                        memset(
                            buff as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            8 as size_t,
                        );
                        if unixtoread(
                            csock,
                            buff as *mut ::core::ffi::c_void,
                            8 as uint32_t,
                            5000 as uint32_t,
                            5000 as uint32_t,
                        ) != 8 as int32_t
                        {
                            fprintf(
                                stderr,
                                b"error receiving data from '%s': %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                lsockname,
                                strerror(*__errno_location()),
                            );
                        } else {
                            rptr = buff;
                            cmd = get32bit(&raw mut rptr);
                            leng = get32bit(&raw mut rptr);
                            if cmd != MFSNBD_REMOVE as ::core::ffi::c_int as uint32_t {
                                fprintf(
                                    stderr,
                                    b"got wrong answer from '%s': Bad Command\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    lsockname,
                                );
                            } else if leng < 2 as uint32_t
                                || leng > (2 as ::core::ffi::c_int + NBD_ERR_SIZE) as uint32_t
                            {
                                fprintf(
                                    stderr,
                                    b"got wrong answer from '%s': Wrong Size\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    lsockname,
                                );
                            } else {
                                free(buff as *mut ::core::ffi::c_void);
                                buff = malloc(leng as size_t) as *mut uint8_t;
                                if buff.is_null() {
                                    fprintf(
                                        stderr,
                                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        2490 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                                    );
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        2490 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                                    );
                                    abort();
                                } else if buff
                                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                                        -1 as ::core::ffi::c_int as usize,
                                    ) as *mut uint8_t
                                {
                                    let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                                        strerr(*__errno_location());
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        2490 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                                        _mfs_errorstring_0,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        2490 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                                        _mfs_errorstring_0,
                                    );
                                    abort();
                                }
                                if unixtoread(
                                    csock,
                                    buff as *mut ::core::ffi::c_void,
                                    leng,
                                    1000 as uint32_t,
                                    1000 as uint32_t,
                                ) as ssize_t
                                    != leng as ssize_t
                                {
                                    fprintf(
                                        stderr,
                                        b"error receiving data from '%s': %s\n\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        lsockname,
                                        strerror(*__errno_location()),
                                    );
                                } else {
                                    close(csock);
                                    csock = -1 as ::core::ffi::c_int;
                                    rptr = buff;
                                    status = get8bit(&raw mut rptr);
                                    aleng = get8bit(&raw mut rptr);
                                    answer = nbd_packet_to_str(rptr, aleng as uint32_t);
                                    if !answer.is_null() {
                                        if status as ::core::ffi::c_int
                                            == MFSNBD_OK as ::core::ffi::c_int
                                        {
                                            printf(
                                                b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                                                answer,
                                            );
                                        } else {
                                            fprintf(
                                                stderr,
                                                b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                                                answer,
                                            );
                                            break '_err;
                                        }
                                    }
                                    res = 0 as ::core::ffi::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
        if csock >= 0 as ::core::ffi::c_int {
            close(csock);
        }
        if !lsockname.is_null() {
            free(lsockname as *mut ::core::ffi::c_void);
        }
        if !filename.is_null() {
            free(filename as *mut ::core::ffi::c_void);
        }
        if !device.is_null() {
            free(device as *mut ::core::ffi::c_void);
        }
        if !linkname.is_null() {
            free(linkname as *mut ::core::ffi::c_void);
        }
        if !answer.is_null() {
            free(answer as *mut ::core::ffi::c_void);
        }
        if !buff.is_null() {
            free(buff as *mut ::core::ffi::c_void);
        }
        return res;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nbd_resize_bdev(
    mut appname: *const ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut buff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut device: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut linkname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut size: uint64_t = 0;
        let mut dsize: uint32_t = 0;
        let mut pleng: uint32_t = 0;
        let mut dleng: uint32_t = 0;
        let mut nleng: uint32_t = 0;
        let mut aleng: uint8_t = 0;
        let mut status: uint8_t = 0;
        let mut answer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut lsockname: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut ch: ::core::ffi::c_int = 0;
        let mut csock: ::core::ffi::c_int = 0;
        let mut res: ::core::ffi::c_int = 0;
        let mut cmd: uint32_t = 0;
        let mut leng: uint32_t = 0;
        csock = -1 as ::core::ffi::c_int;
        buff = ::core::ptr::null_mut::<uint8_t>();
        lsockname = ::core::ptr::null_mut::<::core::ffi::c_char>();
        filename = ::core::ptr::null_mut::<::core::ffi::c_char>();
        device = ::core::ptr::null_mut::<::core::ffi::c_char>();
        linkname = ::core::ptr::null_mut::<::core::ffi::c_char>();
        answer = ::core::ptr::null_mut::<::core::ffi::c_char>();
        size = 0 as uint64_t;
        res = 1 as ::core::ffi::c_int;
        loop {
            ch = getopt(
                argc,
                argv as *const *mut ::core::ffi::c_char,
                b"l:f:d:n:s:?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if ch == -1 as ::core::ffi::c_int {
                break;
            }
            match ch {
                108 => {
                    if !lsockname.is_null() {
                        free(lsockname as *mut ::core::ffi::c_void);
                    }
                    lsockname = strdup(optarg);
                }
                102 => {
                    if !filename.is_null() {
                        free(filename as *mut ::core::ffi::c_void);
                    }
                    filename = strdup(optarg);
                }
                100 => {
                    if !device.is_null() {
                        free(device as *mut ::core::ffi::c_void);
                    }
                    device = strdup(optarg);
                }
                110 => {
                    if !linkname.is_null() {
                        free(linkname as *mut ::core::ffi::c_void);
                    }
                    if strlen(optarg) > NBD_LINK_PREFIX_LENG as size_t
                        && memcmp(
                            optarg as *const ::core::ffi::c_void,
                            NBD_LINK_PREFIX.as_ptr() as *const ::core::ffi::c_void,
                            NBD_LINK_PREFIX_LENG as size_t,
                        ) == 0 as ::core::ffi::c_int
                    {
                        linkname = strdup(optarg.offset(NBD_LINK_PREFIX_LENG as isize));
                    } else {
                        linkname = strdup(optarg);
                    }
                }
                115 => {
                    size = sizestrtod(
                        optarg,
                        ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                    ) as uint64_t;
                }
                104 | _ => {
                    usage(appname);
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
        '_err: {
            if filename.is_null() && device.is_null() && linkname.is_null() {
                fprintf(
                    stderr,
                    b"device not specified\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                if lsockname.is_null() {
                    lsockname =
                        strdup(b"/dev/mfs/nbdsock\0".as_ptr() as *const ::core::ffi::c_char);
                }
                csock = unixsocket();
                if unixtoconnect(csock, lsockname, 1000 as uint32_t) < 0 as ::core::ffi::c_int {
                    fprintf(
                        stderr,
                        b"can't connect to socket '%s': %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        lsockname,
                        strerror(*__errno_location()),
                    );
                } else {
                    if !filename.is_null() {
                        pleng = strlen(filename) as uint32_t;
                        if pleng > 65535 as uint32_t {
                            fprintf(
                                stderr,
                                b"MFS file name too long\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            break '_err;
                        }
                    } else {
                        pleng = 0 as uint32_t;
                    }
                    if !device.is_null() {
                        dleng = strlen(device) as uint32_t;
                        if dleng > 255 as uint32_t {
                            fprintf(
                                stderr,
                                b"device name too long\n\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            break '_err;
                        }
                    } else {
                        dleng = 0 as uint32_t;
                    }
                    if !linkname.is_null() {
                        nleng = strlen(linkname) as uint32_t;
                        if nleng > 255 as uint32_t {
                            fprintf(
                                stderr,
                                b"link name too long\n\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            break '_err;
                        }
                    } else {
                        nleng = 0 as uint32_t;
                    }
                    dsize = (12 as uint32_t)
                        .wrapping_add(pleng)
                        .wrapping_add(dleng)
                        .wrapping_add(nleng);
                    buff = malloc((8 as uint32_t).wrapping_add(dsize) as size_t) as *mut uint8_t;
                    if buff.is_null() {
                        fprintf(
                            stderr,
                            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            2648 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            2648 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        abort();
                    } else if buff
                        == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                            -1 as ::core::ffi::c_int as usize,
                        ) as *mut uint8_t
                    {
                        let mut _mfs_errorstring: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            2648 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            2648 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring,
                        );
                        abort();
                    }
                    wptr = buff;
                    put32bit(
                        &raw mut wptr,
                        MFSNBD_RESIZE as ::core::ffi::c_int as uint32_t,
                    );
                    put32bit(&raw mut wptr, dsize);
                    put16bit(&raw mut wptr, pleng as uint16_t);
                    if pleng > 0 as uint32_t {
                        memcpy(
                            wptr as *mut ::core::ffi::c_void,
                            filename as *const ::core::ffi::c_void,
                            pleng as size_t,
                        );
                        wptr = wptr.offset(pleng as isize);
                    }
                    put8bit(&raw mut wptr, dleng as uint8_t);
                    if dleng > 0 as uint32_t {
                        memcpy(
                            wptr as *mut ::core::ffi::c_void,
                            device as *const ::core::ffi::c_void,
                            dleng as size_t,
                        );
                        wptr = wptr.offset(dleng as isize);
                    }
                    put8bit(&raw mut wptr, nleng as uint8_t);
                    if nleng > 0 as uint32_t {
                        memcpy(
                            wptr as *mut ::core::ffi::c_void,
                            linkname as *const ::core::ffi::c_void,
                            nleng as size_t,
                        );
                        wptr = wptr.offset(nleng as isize);
                    }
                    put64bit(&raw mut wptr, size);
                    if unixtowrite(
                        csock,
                        buff as *const ::core::ffi::c_void,
                        (8 as uint32_t).wrapping_add(dsize),
                        1000 as uint32_t,
                        1000 as uint32_t,
                    ) as ssize_t
                        != (8 as uint32_t).wrapping_add(dsize) as ssize_t
                    {
                        fprintf(
                            stderr,
                            b"unable to send data to '%s': %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            lsockname,
                            strerror(*__errno_location()),
                        );
                    } else {
                        memset(
                            buff as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            8 as size_t,
                        );
                        if unixtoread(
                            csock,
                            buff as *mut ::core::ffi::c_void,
                            8 as uint32_t,
                            5000 as uint32_t,
                            5000 as uint32_t,
                        ) != 8 as int32_t
                        {
                            fprintf(
                                stderr,
                                b"error receiving data from '%s': %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                lsockname,
                                strerror(*__errno_location()),
                            );
                        } else {
                            rptr = buff;
                            cmd = get32bit(&raw mut rptr);
                            leng = get32bit(&raw mut rptr);
                            if cmd != MFSNBD_RESIZE as ::core::ffi::c_int as uint32_t {
                                fprintf(
                                    stderr,
                                    b"got wrong answer from '%s': Bad Command\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    lsockname,
                                );
                            } else if leng < 2 as uint32_t
                                || leng > (2 as ::core::ffi::c_int + NBD_ERR_SIZE) as uint32_t
                            {
                                fprintf(
                                    stderr,
                                    b"got wrong answer from '%s': Wrong Size\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    lsockname,
                                );
                            } else {
                                free(buff as *mut ::core::ffi::c_void);
                                buff = malloc(leng as size_t) as *mut uint8_t;
                                if buff.is_null() {
                                    fprintf(
                                        stderr,
                                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        2698 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                                    );
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        2698 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                                    );
                                    abort();
                                } else if buff
                                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                                        -1 as ::core::ffi::c_int as usize,
                                    ) as *mut uint8_t
                                {
                                    let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                                        strerr(*__errno_location());
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        2698 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                                        _mfs_errorstring_0,
                                    );
                                    fprintf(
                                        stderr,
                                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        2698 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                        b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                                        _mfs_errorstring_0,
                                    );
                                    abort();
                                }
                                if unixtoread(
                                    csock,
                                    buff as *mut ::core::ffi::c_void,
                                    leng,
                                    1000 as uint32_t,
                                    1000 as uint32_t,
                                ) as ssize_t
                                    != leng as ssize_t
                                {
                                    fprintf(
                                        stderr,
                                        b"error receiving data from '%s': %s\n\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        lsockname,
                                        strerror(*__errno_location()),
                                    );
                                } else {
                                    close(csock);
                                    csock = -1 as ::core::ffi::c_int;
                                    rptr = buff;
                                    status = get8bit(&raw mut rptr);
                                    aleng = get8bit(&raw mut rptr);
                                    answer = nbd_packet_to_str(rptr, aleng as uint32_t);
                                    if !answer.is_null() {
                                        if status as ::core::ffi::c_int
                                            == MFSNBD_OK as ::core::ffi::c_int
                                        {
                                            printf(
                                                b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                                                answer,
                                            );
                                        } else {
                                            fprintf(
                                                stderr,
                                                b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                                                answer,
                                            );
                                            break '_err;
                                        }
                                    }
                                    res = 0 as ::core::ffi::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
        if csock >= 0 as ::core::ffi::c_int {
            close(csock);
        }
        if !lsockname.is_null() {
            free(lsockname as *mut ::core::ffi::c_void);
        }
        if !filename.is_null() {
            free(filename as *mut ::core::ffi::c_void);
        }
        if !device.is_null() {
            free(device as *mut ::core::ffi::c_void);
        }
        if !linkname.is_null() {
            free(linkname as *mut ::core::ffi::c_void);
        }
        if !answer.is_null() {
            free(answer as *mut ::core::ffi::c_void);
        }
        if !buff.is_null() {
            free(buff as *mut ::core::ffi::c_void);
        }
        return res;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn nbd_list_mappings(
    mut appname: *const ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut cbuff: [uint8_t; 8] = [0; 8];
        let mut buff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut dcnt: uint8_t = 0;
        let mut pleng: uint16_t = 0;
        let mut dleng: uint8_t = 0;
        let mut nleng: uint8_t = 0;
        let mut maxpleng: uint16_t = 0;
        let mut maxdleng: uint8_t = 0;
        let mut maxnleng: uint8_t = 0;
        let mut size: uint64_t = 0;
        let mut maxsize: uint64_t = 0;
        let mut bsize: uint32_t = 0;
        let mut maxbsize: uint32_t = 0;
        let mut flags: uint32_t = 0;
        let mut displaymode: uint8_t = 0;
        let mut formatbuff: [::core::ffi::c_char; 100] = [0; 100];
        let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut device: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut linkname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut lsockname: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut lsockcustom: uint8_t = 0;
        let mut ch: ::core::ffi::c_int = 0;
        let mut csock: ::core::ffi::c_int = 0;
        let mut cmd: uint32_t = 0;
        let mut leng: uint32_t = 0;
        let mut res: ::core::ffi::c_int = 0;
        csock = -1 as ::core::ffi::c_int;
        lsockname = ::core::ptr::null_mut::<::core::ffi::c_char>();
        lsockcustom = 0 as uint8_t;
        displaymode = 0 as uint8_t;
        buff = ::core::ptr::null_mut::<uint8_t>();
        res = 1 as ::core::ffi::c_int;
        loop {
            ch = getopt(
                argc,
                argv as *const *mut ::core::ffi::c_char,
                b"l:t:?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if ch == -1 as ::core::ffi::c_int {
                break;
            }
            match ch {
                108 => {
                    if !lsockname.is_null() {
                        free(lsockname as *mut ::core::ffi::c_void);
                    }
                    lsockname = strdup(optarg);
                    lsockcustom = 1 as uint8_t;
                }
                116 => {
                    if *optarg.offset(0 as isize) as ::core::ffi::c_int == 'A' as ::core::ffi::c_int
                        || *optarg.offset(0 as isize) as ::core::ffi::c_int
                            == 'a' as ::core::ffi::c_int
                        || *optarg.offset(0 as isize) as ::core::ffi::c_int
                            == 'M' as ::core::ffi::c_int
                        || *optarg.offset(0 as isize) as ::core::ffi::c_int
                            == 'm' as ::core::ffi::c_int
                    {
                        displaymode = 1 as uint8_t;
                    } else if *optarg.offset(0 as isize) as ::core::ffi::c_int
                        == 'R' as ::core::ffi::c_int
                        || *optarg.offset(0 as isize) as ::core::ffi::c_int
                            == 'r' as ::core::ffi::c_int
                        || *optarg.offset(0 as isize) as ::core::ffi::c_int
                            == 'D' as ::core::ffi::c_int
                        || *optarg.offset(0 as isize) as ::core::ffi::c_int
                            == 'd' as ::core::ffi::c_int
                        || *optarg.offset(0 as isize) as ::core::ffi::c_int
                            == 'U' as ::core::ffi::c_int
                        || *optarg.offset(0 as isize) as ::core::ffi::c_int
                            == 'u' as ::core::ffi::c_int
                    {
                        displaymode = 2 as uint8_t;
                    } else if *optarg.offset(0 as isize) as ::core::ffi::c_int
                        == 'C' as ::core::ffi::c_int
                        || *optarg.offset(0 as isize) as ::core::ffi::c_int
                            == 'c' as ::core::ffi::c_int
                        || *optarg.offset(0 as isize) as ::core::ffi::c_int
                            == 'I' as ::core::ffi::c_int
                        || *optarg.offset(0 as isize) as ::core::ffi::c_int
                            == 'i' as ::core::ffi::c_int
                    {
                        displaymode = 3 as uint8_t;
                    } else {
                        fprintf(
                            stderr,
                            b"unknown display mode: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                            optarg,
                        );
                        usage(appname);
                        return 0 as ::core::ffi::c_int;
                    }
                }
                104 | _ => {
                    usage(appname);
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
        if lsockname.is_null() {
            lsockname = strdup(b"/dev/mfs/nbdsock\0".as_ptr() as *const ::core::ffi::c_char);
        }
        csock = unixsocket();
        if unixtoconnect(csock, lsockname, 1000 as uint32_t) < 0 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"can't connect to socket '%s': %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                lsockname,
                strerror(*__errno_location()),
            );
        } else {
            wptr = &raw mut cbuff as *mut uint8_t;
            put32bit(&raw mut wptr, MFSNBD_LIST as ::core::ffi::c_int as uint32_t);
            put32bit(&raw mut wptr, 0 as uint32_t);
            if unixtowrite(
                csock,
                &raw mut cbuff as *mut uint8_t as *const ::core::ffi::c_void,
                8 as uint32_t,
                1000 as uint32_t,
                1000 as uint32_t,
            ) != 8 as int32_t
            {
                fprintf(
                    stderr,
                    b"unable to send data to '%s': %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    lsockname,
                    strerror(*__errno_location()),
                );
            } else {
                memset(
                    &raw mut cbuff as *mut uint8_t as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    8 as size_t,
                );
                if unixtoread(
                    csock,
                    &raw mut cbuff as *mut uint8_t as *mut ::core::ffi::c_void,
                    8 as uint32_t,
                    5000 as uint32_t,
                    5000 as uint32_t,
                ) != 8 as int32_t
                {
                    fprintf(
                        stderr,
                        b"error receiving data from '%s': %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        lsockname,
                        strerror(*__errno_location()),
                    );
                } else {
                    rptr = &raw mut cbuff as *mut uint8_t;
                    cmd = get32bit(&raw mut rptr);
                    leng = get32bit(&raw mut rptr);
                    if cmd != MFSNBD_LIST as ::core::ffi::c_int as uint32_t {
                        fprintf(
                            stderr,
                            b"got wrong answer from '%s': Bad Command\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            lsockname,
                        );
                    } else {
                        buff = malloc(leng as size_t) as *mut uint8_t;
                        if buff.is_null() {
                            fprintf(
                                stderr,
                                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                2842 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                2842 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            abort();
                        } else if buff
                            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                                -1 as ::core::ffi::c_int as usize,
                            ) as *mut uint8_t
                        {
                            let mut _mfs_errorstring: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                2842 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                                _mfs_errorstring,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/mfsbdev.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                2842 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"buff\0".as_ptr() as *const ::core::ffi::c_char,
                                _mfs_errorstring,
                            );
                            abort();
                        }
                        if unixtoread(
                            csock,
                            buff as *mut ::core::ffi::c_void,
                            leng,
                            1000 as uint32_t,
                            1000 as uint32_t,
                        ) as ssize_t
                            != leng as ssize_t
                        {
                            fprintf(
                                stderr,
                                b"error receiving data from '%s': %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                lsockname,
                                strerror(*__errno_location()),
                            );
                        } else {
                            close(csock);
                            csock = -1 as ::core::ffi::c_int;
                            maxpleng = 0 as uint16_t;
                            maxdleng = 0 as uint8_t;
                            maxnleng = 0 as uint8_t;
                            maxsize = 0 as uint64_t;
                            maxbsize = 0 as uint32_t;
                            formatbuff[0 as usize] = '\n' as ::core::ffi::c_char;
                            formatbuff[1 as usize] = 0 as ::core::ffi::c_char;
                            if displaymode as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
                                rptr = buff;
                                dcnt = get8bit(&raw mut rptr);
                                while dcnt as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                                    pleng = get16bit(&raw mut rptr);
                                    rptr = rptr.offset(pleng as ::core::ffi::c_int as isize);
                                    dleng = get8bit(&raw mut rptr);
                                    rptr = rptr.offset(dleng as ::core::ffi::c_int as isize);
                                    nleng = get8bit(&raw mut rptr);
                                    rptr = rptr.offset(nleng as ::core::ffi::c_int as isize);
                                    size = get64bit(&raw mut rptr);
                                    bsize = get32bit(&raw mut rptr);
                                    rptr = rptr.offset(4 as ::core::ffi::c_int as isize);
                                    dcnt = dcnt.wrapping_sub(1);
                                    if pleng as ::core::ffi::c_int > maxpleng as ::core::ffi::c_int
                                    {
                                        maxpleng = pleng;
                                    }
                                    if dleng as ::core::ffi::c_int > maxdleng as ::core::ffi::c_int
                                    {
                                        maxdleng = dleng;
                                    }
                                    if nleng as ::core::ffi::c_int > maxnleng as ::core::ffi::c_int
                                    {
                                        maxnleng = nleng;
                                    }
                                    if size > maxsize {
                                        maxsize = size;
                                    }
                                    if bsize > maxbsize {
                                        maxbsize = bsize;
                                    }
                                }
                                snprintf(
                                    &raw mut formatbuff as *mut ::core::ffi::c_char,
                                    100 as size_t,
                                    b"%lu\0".as_ptr() as *const ::core::ffi::c_char,
                                    maxsize,
                                );
                                formatbuff[99 as usize] = 0 as ::core::ffi::c_char;
                                maxsize = strlen(&raw mut formatbuff as *mut ::core::ffi::c_char)
                                    as uint64_t;
                                snprintf(
                                    &raw mut formatbuff as *mut ::core::ffi::c_char,
                                    100 as size_t,
                                    b"%u\0".as_ptr() as *const ::core::ffi::c_char,
                                    maxbsize,
                                );
                                formatbuff[99 as usize] = 0 as ::core::ffi::c_char;
                                maxbsize = strlen(&raw mut formatbuff as *mut ::core::ffi::c_char)
                                    as uint32_t;
                                snprintf(
                                    &raw mut formatbuff as *mut ::core::ffi::c_char,
                                    100 as size_t,
                                    b"%%%ulu %%%uu %%s,%%s\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    maxsize as ::core::ffi::c_uint,
                                    maxbsize as ::core::ffi::c_uint,
                                );
                            }
                            rptr = buff;
                            dcnt = get8bit(&raw mut rptr);
                            while dcnt as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                                pleng = get16bit(&raw mut rptr);
                                path = nbd_packet_to_str(rptr, pleng as uint32_t);
                                rptr = rptr.offset(pleng as ::core::ffi::c_int as isize);
                                dleng = get8bit(&raw mut rptr);
                                device = nbd_packet_to_str(rptr, dleng as uint32_t);
                                rptr = rptr.offset(dleng as ::core::ffi::c_int as isize);
                                nleng = get8bit(&raw mut rptr);
                                linkname = nbd_packet_to_str(rptr, nleng as uint32_t);
                                rptr = rptr.offset(nleng as ::core::ffi::c_int as isize);
                                size = get64bit(&raw mut rptr);
                                bsize = get32bit(&raw mut rptr);
                                flags = get32bit(&raw mut rptr);
                                if displaymode as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                                    if displaymode as ::core::ffi::c_int == 3 as ::core::ffi::c_int
                                    {
                                        printf(
                                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                            path,
                                        );
                                        while pleng as ::core::ffi::c_int
                                            <= maxpleng as ::core::ffi::c_int
                                        {
                                            putchar(' ' as ::core::ffi::c_int);
                                            pleng = pleng.wrapping_add(1);
                                        }
                                        printf(
                                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                            device,
                                        );
                                        while dleng as ::core::ffi::c_int
                                            <= maxdleng as ::core::ffi::c_int
                                        {
                                            putchar(' ' as ::core::ffi::c_int);
                                            dleng = dleng.wrapping_add(1);
                                        }
                                        printf(
                                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                            linkname,
                                        );
                                        while nleng as ::core::ffi::c_int
                                            <= maxnleng as ::core::ffi::c_int
                                        {
                                            putchar(' ' as ::core::ffi::c_int);
                                            nleng = nleng.wrapping_add(1);
                                        }
                                        printf(
                                            &raw mut formatbuff as *mut ::core::ffi::c_char,
                                            size,
                                            bsize,
                                            if flags & FLAG_READONLY as uint32_t != 0 {
                                                b"ro\0".as_ptr() as *const ::core::ffi::c_char
                                            } else {
                                                b"rw\0".as_ptr() as *const ::core::ffi::c_char
                                            },
                                            if flags & FLAG_IGNORELOCK as uint32_t != 0 {
                                                b"nolock\0".as_ptr() as *const ::core::ffi::c_char
                                            } else {
                                                b"lock\0".as_ptr() as *const ::core::ffi::c_char
                                            },
                                        );
                                    } else {
                                        if displaymode as ::core::ffi::c_int
                                            == 1 as ::core::ffi::c_int
                                        {
                                            printf(
                                                b"%s map\0".as_ptr() as *const ::core::ffi::c_char,
                                                appname,
                                            );
                                        } else {
                                            printf(
                                                b"%s unmap\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                appname,
                                            );
                                        }
                                        if lsockcustom != 0 {
                                            printf(
                                                b" -l '%s'\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                lsockname,
                                            );
                                        }
                                        if !path.is_null() {
                                            printf(
                                                b" -f '%s'\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                path,
                                            );
                                        }
                                        if !device.is_null() {
                                            printf(
                                                b" -d '%s'\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                device,
                                            );
                                        }
                                        if !linkname.is_null() {
                                            printf(
                                                b" -n '%s'\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                linkname,
                                            );
                                        }
                                        if displaymode as ::core::ffi::c_int
                                            == 1 as ::core::ffi::c_int
                                        {
                                            printf(
                                                b" -s %lu\0".as_ptr() as *const ::core::ffi::c_char,
                                                size,
                                            );
                                            printf(
                                                b" -b %u\0".as_ptr() as *const ::core::ffi::c_char,
                                                bsize,
                                            );
                                            if flags & FLAG_READONLY as uint32_t != 0 {
                                                printf(
                                                    b" -r\0".as_ptr() as *const ::core::ffi::c_char
                                                );
                                            }
                                            if flags & FLAG_IGNORELOCK as uint32_t != 0 {
                                                printf(
                                                    b" -i\0".as_ptr() as *const ::core::ffi::c_char
                                                );
                                            }
                                        }
                                        printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
                                    }
                                } else {
                                    printf(
                                        b"file: %s ; device: %s ; link: %s ; size: %lu (%.3lfGiB) ; blocksize: %u ; rwmode: %s ; ignore_locks:%s\n\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        if path.is_null() {
                                            b"\0".as_ptr() as *const ::core::ffi::c_char
                                        } else {
                                            path as *const ::core::ffi::c_char
                                        },
                                        if device.is_null() {
                                            b"\0".as_ptr() as *const ::core::ffi::c_char
                                        } else {
                                            device as *const ::core::ffi::c_char
                                        },
                                        if linkname.is_null() {
                                            b"\0".as_ptr() as *const ::core::ffi::c_char
                                        } else {
                                            linkname as *const ::core::ffi::c_char
                                        },
                                        size,
                                        size as ::core::ffi::c_double
                                            / (1024.0f64 * 1024.0f64 * 1024.0f64),
                                        bsize,
                                        if flags & FLAG_READONLY as uint32_t != 0 {
                                            b"ro\0".as_ptr() as *const ::core::ffi::c_char
                                        } else {
                                            b"rw\0".as_ptr() as *const ::core::ffi::c_char
                                        },
                                        if flags & FLAG_IGNORELOCK as uint32_t != 0 {
                                            b"yes\0".as_ptr() as *const ::core::ffi::c_char
                                        } else {
                                            b"no\0".as_ptr() as *const ::core::ffi::c_char
                                        },
                                    );
                                }
                                if !path.is_null() {
                                    free(path as *mut ::core::ffi::c_void);
                                }
                                if !device.is_null() {
                                    free(device as *mut ::core::ffi::c_void);
                                }
                                if !linkname.is_null() {
                                    free(linkname as *mut ::core::ffi::c_void);
                                }
                                dcnt = dcnt.wrapping_sub(1);
                            }
                            res = 0 as ::core::ffi::c_int;
                        }
                    }
                }
            }
        }
        if csock >= 0 as ::core::ffi::c_int {
            close(csock);
        }
        if !lsockname.is_null() {
            free(lsockname as *mut ::core::ffi::c_void);
        }
        if !buff.is_null() {
            free(buff as *mut ::core::ffi::c_void);
        }
        return res;
    }
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut appname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut res: ::core::ffi::c_int = 0;
        appname = strdup(*argv.offset(0 as isize));
        strerr_init();
        if argc < 2 as ::core::ffi::c_int {
            usage(appname);
            res = 1 as ::core::ffi::c_int;
        } else if strcmp(
            *argv.offset(1 as isize),
            b"start\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            res = nbd_start_daemon(appname, argc, argv);
        } else if strcmp(
            *argv.offset(1 as isize),
            b"stop\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            res = nbd_stop_daemon(
                appname,
                argc - 1 as ::core::ffi::c_int,
                argv.offset(1 as ::core::ffi::c_int as isize),
            );
        } else if strcmp(
            *argv.offset(1 as isize),
            b"map\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                *argv.offset(1 as isize),
                b"add\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            res = nbd_add_mapping(
                appname,
                argc - 1 as ::core::ffi::c_int,
                argv.offset(1 as ::core::ffi::c_int as isize),
            );
        } else if strcmp(
            *argv.offset(1 as isize),
            b"unmap\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                *argv.offset(1 as isize),
                b"remove\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                *argv.offset(1 as isize),
                b"delete\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                *argv.offset(1 as isize),
                b"del\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            || strcmp(
                *argv.offset(1 as isize),
                b"rm\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            res = nbd_remove_mapping(
                appname,
                argc - 1 as ::core::ffi::c_int,
                argv.offset(1 as ::core::ffi::c_int as isize),
            );
        } else if strcmp(
            *argv.offset(1 as isize),
            b"resize\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            res = nbd_resize_bdev(
                appname,
                argc - 1 as ::core::ffi::c_int,
                argv.offset(1 as ::core::ffi::c_int as isize),
            );
        } else if strcmp(
            *argv.offset(1 as isize),
            b"list\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            res = nbd_list_mappings(
                appname,
                argc - 1 as ::core::ffi::c_int,
                argv.offset(1 as ::core::ffi::c_int as isize),
            );
        } else {
            usage(appname);
            res = 1 as ::core::ffi::c_int;
        }
        free(appname as *mut ::core::ffi::c_void);
        return res;
    }
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ) as i32)
    }
}
