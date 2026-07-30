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
use ::mfsmetalogger;
unsafe extern "C" {
    unsafe fn mlockall(__flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn getrlimit(
        __resource: __rlimit_resource_t,
        __rlimits: *mut rlimit,
    ) -> ::core::ffi::c_int;
    unsafe fn setrlimit(
        __resource: __rlimit_resource_t,
        __rlimits: *const rlimit,
    ) -> ::core::ffi::c_int;
    unsafe fn setpriority(
        __which: __priority_which_t,
        __who: id_t,
        __prio: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    unsafe fn fputc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    unsafe fn prctl(__option: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    unsafe fn syslog(__pri: ::core::ffi::c_int, __fmt: *const ::core::ffi::c_char, ...);
    unsafe fn umask(__mask: __mode_t) -> __mode_t;
    unsafe fn mkdir(__path: *const ::core::ffi::c_char, __mode: __mode_t) -> ::core::ffi::c_int;
    unsafe fn kill(__pid: __pid_t, __sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    unsafe fn sigaction(
        __sig: ::core::ffi::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> ::core::ffi::c_int;
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
    unsafe fn chdir(__path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn dup(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn getpid() -> __pid_t;
    unsafe fn setpgid(__pid: __pid_t, __pgid: __pid_t) -> ::core::ffi::c_int;
    unsafe fn setsid() -> __pid_t;
    unsafe fn geteuid() -> __uid_t;
    unsafe fn setuid(__uid: __uid_t) -> ::core::ffi::c_int;
    unsafe fn setgid(__gid: __gid_t) -> ::core::ffi::c_int;
    unsafe fn fork() -> __pid_t;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    unsafe fn getopt(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn ftruncate(__fd: ::core::ffi::c_int, __length: __off64_t) -> ::core::ffi::c_int;
    unsafe fn wait(__stat_loc: *mut ::core::ffi::c_int) -> __pid_t;
    unsafe fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut ::core::ffi::c_int,
        __options: ::core::ffi::c_int,
    ) -> __pid_t;
    unsafe fn gettimeofday(
        __tv: *mut timeval,
        __tz: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...)
    -> ::core::ffi::c_int;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    unsafe fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    unsafe fn exit(__status: ::core::ffi::c_int) -> !;
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::core::ffi::c_int;
    unsafe fn getgrnam_r(
        __name: *const ::core::ffi::c_char,
        __resultbuf: *mut group,
        __buffer: *mut ::core::ffi::c_char,
        __buflen: size_t,
        __result: *mut *mut group,
    ) -> ::core::ffi::c_int;
    unsafe fn setgroups(__n: size_t, __groups: *const __gid_t) -> ::core::ffi::c_int;
    unsafe fn getpwuid_r(
        __uid: __uid_t,
        __resultbuf: *mut passwd,
        __buffer: *mut ::core::ffi::c_char,
        __buflen: size_t,
        __result: *mut *mut passwd,
    ) -> ::core::ffi::c_int;
    unsafe fn getpwnam_r(
        __name: *const ::core::ffi::c_char,
        __resultbuf: *mut passwd,
        __buffer: *mut ::core::ffi::c_char,
        __buflen: size_t,
        __result: *mut *mut passwd,
    ) -> ::core::ffi::c_int;
    unsafe fn poll(
        __fds: *mut pollfd,
        __nfds: nfds_t,
        __timeout: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
    unsafe fn monotonic_method() -> *const ::core::ffi::c_char;
    unsafe fn monotonic_speed() -> uint32_t;
    unsafe fn cfg_load(
        fname: *const ::core::ffi::c_char,
        logundefined: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn cfg_dangerous_options() -> ::core::ffi::c_int;
    unsafe fn cfg_reload() -> ::core::ffi::c_int;
    unsafe fn cfg_info(fd: *mut FILE);
    unsafe fn cfg_term();
    unsafe fn cfg_getstr(
        name: *const ::core::ffi::c_char,
        def: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn cfg_getnum(
        name: *const ::core::ffi::c_char,
        def: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn cfg_getuint8(name: *const ::core::ffi::c_char, def: uint8_t) -> uint8_t;
    unsafe fn cfg_getuint32(name: *const ::core::ffi::c_char, def: uint32_t) -> uint32_t;
    unsafe fn cfg_getint32(name: *const ::core::ffi::c_char, def: int32_t) -> int32_t;
    unsafe fn cfg_getdouble(
        name: *const ::core::ffi::c_char,
        def: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn strerr_init();
    unsafe fn strerr_term();
    unsafe fn mycrc32_init();
    unsafe fn masterconn_init() -> ::core::ffi::c_int;
    unsafe fn mfs_log_str_to_pri(pristr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn mfs_log_set_min_level(minlevel: ::core::ffi::c_int);
    unsafe fn mfs_log_set_elevate_to(elevateto: ::core::ffi::c_int);
    unsafe fn mfs_log_detach_stderr();
    unsafe fn mfs_log_term();
    unsafe fn mfs_log_init(
        ident: *const ::core::ffi::c_char,
        daemon: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn processname_init(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char);
}
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __mode_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __clock_t = ::core::ffi::c_long;
pub type __rlim64_t = ::core::ffi::c_ulong;
pub type __id_t = ::core::ffi::c_uint;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type size_t = usize;
pub type mode_t = __mode_t;
pub type __rlimit_resource = ::core::ffi::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
pub type rlim_t = __rlim64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __priority_which = ::core::ffi::c_uint;
pub const PRIO_USER: __priority_which = 2;
pub const PRIO_PGRP: __priority_which = 1;
pub const PRIO_PROCESS: __priority_which = 0;
pub type id_t = __id_t;
pub type __rlimit_resource_t = __rlimit_resource;
pub type __priority_which_t = __priority_which;
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
pub type ssize_t = isize;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
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
pub type int32_t = i32;
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
pub struct group {
    pub gr_name: *mut ::core::ffi::c_char,
    pub gr_passwd: *mut ::core::ffi::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut ::core::ffi::c_char,
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
pub type nfds_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
}
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct deentry {
    pub fun: Option<unsafe extern "C" fn() -> ()>,
    pub fname: *mut ::core::ffi::c_char,
    pub next: *mut deentry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct meentry {
    pub fun: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub fname: *mut ::core::ffi::c_char,
    pub next: *mut meentry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct weentry {
    pub fun: Option<unsafe extern "C" fn() -> ()>,
    pub fname: *mut ::core::ffi::c_char,
    pub next: *mut weentry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ceentry {
    pub fun: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub fname: *mut ::core::ffi::c_char,
    pub next: *mut ceentry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlentry {
    pub fun: Option<unsafe extern "C" fn() -> ()>,
    pub fname: *mut ::core::ffi::c_char,
    pub next: *mut rlentry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inentry {
    pub fun: Option<unsafe extern "C" fn(*mut FILE) -> ()>,
    pub fname: *mut ::core::ffi::c_char,
    pub next: *mut inentry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chldentry {
    pub pid: pid_t,
    pub fun: Option<unsafe extern "C" fn(pid_t, ::core::ffi::c_int) -> ()>,
    pub fname: *mut ::core::ffi::c_char,
    pub next: *mut chldentry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kaentry {
    pub fun: Option<unsafe extern "C" fn() -> ()>,
    pub fname: *mut ::core::ffi::c_char,
    pub next: *mut kaentry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollentry {
    pub desc: Option<unsafe extern "C" fn(*mut pollfd, *mut uint32_t) -> ()>,
    pub serve: Option<unsafe extern "C" fn(*mut pollfd) -> ()>,
    pub dname: *mut ::core::ffi::c_char,
    pub sname: *mut ::core::ffi::c_char,
    pub next: *mut pollentry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eloopentry {
    pub fun: Option<unsafe extern "C" fn() -> ()>,
    pub fname: *mut ::core::ffi::c_char,
    pub next: *mut eloopentry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeentry {
    pub nextevent: uint64_t,
    pub useconds: uint64_t,
    pub usecoffset: uint64_t,
    pub fun: Option<unsafe extern "C" fn() -> ()>,
    pub fname: *mut ::core::ffi::c_char,
    pub next: *mut timeentry,
}
pub type runfn = Option<unsafe extern "C" fn() -> ::core::ffi::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_10 {
    pub r#fn: runfn,
    pub name: *mut ::core::ffi::c_char,
}
pub const MFSMAXFILES: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const MCL_CURRENT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MCL_FUTURE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const RLIM_INFINITY: ::core::ffi::c_ulonglong = 0xffffffffffffffff as ::core::ffi::c_ulonglong;
pub const OOM_SCORE_ADJ_MIN: ::core::ffi::c_int = -1000 as ::core::ffi::c_int;
pub const OOM_DISABLE: ::core::ffi::c_int = -17 as ::core::ffi::c_int;
pub const PR_SET_DUMPABLE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SIGHUP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIGQUIT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SIGKILL: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SIGPIPE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SIGALRM: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const SIGTSTP: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const SIGCHLD: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const SIGTTIN: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const SIGTTOU: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const SIGVTALRM: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const SIGPROF: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const SIGUSR1: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SIGUSR2: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SIGCLD: ::core::ffi::c_int = SIGCHLD;
pub const SA_RESTART: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const STDIN_FILENO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STDERR_FILENO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const WNOHANG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const F_GETLK64: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const F_SETLK64: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const F_GETLK: ::core::ffi::c_int = F_GETLK64;
pub const F_SETLK: ::core::ffi::c_int = F_SETLK64;
pub const F_WRLCK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const F_UNLCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const EEXIST: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub static mut id: [::core::ffi::c_char; 72] = unsafe {
    ::core::mem::transmute::<[u8; 72], [::core::ffi::c_char; 72]>(
        *b"@(#) version: 4.59.2-1, build: 2106, written by Jakub Kruszona-Zawadzki\0",
    )
};
#[unsafe(no_mangle)]
pub static mut RunTab: [C2Rust_Unnamed_10; 2] = [
    C2Rust_Unnamed_10 {
        r#fn: Some(masterconn_init as unsafe extern "C" fn() -> ::core::ffi::c_int),
        name: b"connection with master\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    C2Rust_Unnamed_10 {
        r#fn: None,
        name: b"****\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
];
#[unsafe(no_mangle)]
pub static mut LateRunTab: [C2Rust_Unnamed_10; 1] = [C2Rust_Unnamed_10 {
    r#fn: None,
    name: b"****\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
}];
#[unsafe(no_mangle)]
pub static mut RestoreRunTab: [C2Rust_Unnamed_10; 1] = [C2Rust_Unnamed_10 {
    r#fn: None,
    name: b"****\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
}];
pub const DATA_PATH: [::core::ffi::c_char; 82] = unsafe {
    ::core::mem::transmute::<[u8; 82], [::core::ffi::c_char; 82]>(
        *b"/home/ston/Workspaces/polaris-fs/target/mfs-port/mfs-build/../mfs-install/var/mfs\0",
    )
};
pub const DEFAULT_GROUP: [::core::ffi::c_char; 1] =
    unsafe { ::core::mem::transmute::<[u8; 1], [::core::ffi::c_char; 1]>(*b"\0") };
pub const DEFAULT_USER: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"nobody\0") };
pub const VERSSTR: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"4.59.2-1\0") };
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_ERRNO_SYSLOG_STDERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
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
static mut lcall_end: ::core::ffi::c_double = 0.;
static mut lcall_start: ::core::ffi::c_double = 0.;
static mut lcall_trigger: ::core::ffi::c_double = 0.0f64;
static mut loop_usleep: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
pub const RM_RESTART: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const RM_START: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const RM_STOP: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const RM_RELOAD: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const RM_INFO: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const RM_TEST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const RM_KILL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const RM_TRY_RESTART: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const RM_RESTORE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
static mut dehead: *mut deentry = ::core::ptr::null_mut::<deentry>();
static mut mehead: *mut meentry = ::core::ptr::null_mut::<meentry>();
static mut wehead: *mut weentry = ::core::ptr::null_mut::<weentry>();
static mut cehead: *mut ceentry = ::core::ptr::null_mut::<ceentry>();
static mut rlhead: *mut rlentry = ::core::ptr::null_mut::<rlentry>();
static mut inhead: *mut inentry = ::core::ptr::null_mut::<inentry>();
static mut kahead: *mut kaentry = ::core::ptr::null_mut::<kaentry>();
static mut pollhead: *mut pollentry = ::core::ptr::null_mut::<pollentry>();
static mut eloophead: *mut eloopentry = ::core::ptr::null_mut::<eloopentry>();
static mut chldhead: *mut chldentry = ::core::ptr::null_mut::<chldentry>();
static mut timehead: *mut timeentry = ::core::ptr::null_mut::<timeentry>();
static mut now: uint32_t = 0;
static mut start_time: uint32_t = 0;
static mut usecnow: uint64_t = 0;
static mut loop_start: ::core::ffi::c_double = 0.;
static mut signalpipe: [::core::ffi::c_int; 2] = [0; 2];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_destruct_register_fname(
    mut fun: Option<unsafe extern "C" fn() -> ()>,
    mut fname: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut aux: *mut deentry = malloc(::core::mem::size_of::<deentry>()) as *mut deentry;
        if aux.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                247 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                247 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if aux
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut deentry
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                247 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                247 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*aux).fun = fun;
        (*aux).fname = strdup(fname);
        (*aux).next = dehead as *mut deentry;
        dehead = aux;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_mayexit_register_fname(
    mut fun: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    mut fname: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut aux: *mut meentry = malloc(::core::mem::size_of::<meentry>()) as *mut meentry;
        if aux.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                256 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                256 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if aux
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut meentry
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                256 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                256 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*aux).fun = fun;
        (*aux).fname = strdup(fname);
        (*aux).next = mehead as *mut meentry;
        mehead = aux;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_wantexit_register_fname(
    mut fun: Option<unsafe extern "C" fn() -> ()>,
    mut fname: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut aux: *mut weentry = malloc(::core::mem::size_of::<weentry>()) as *mut weentry;
        if aux.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                265 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                265 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if aux
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut weentry
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                265 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                265 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*aux).fun = fun;
        (*aux).fname = strdup(fname);
        (*aux).next = wehead as *mut weentry;
        wehead = aux;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_canexit_register_fname(
    mut fun: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    mut fname: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut aux: *mut ceentry = malloc(::core::mem::size_of::<ceentry>()) as *mut ceentry;
        if aux.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                274 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                274 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if aux
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut ceentry
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                274 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                274 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*aux).fun = fun;
        (*aux).fname = strdup(fname);
        (*aux).next = cehead as *mut ceentry;
        cehead = aux;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_reload_register_fname(
    mut fun: Option<unsafe extern "C" fn() -> ()>,
    mut fname: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut aux: *mut rlentry = malloc(::core::mem::size_of::<rlentry>()) as *mut rlentry;
        if aux.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                283 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                283 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if aux
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut rlentry
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                283 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                283 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*aux).fun = fun;
        (*aux).fname = strdup(fname);
        (*aux).next = rlhead as *mut rlentry;
        rlhead = aux;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_info_register_fname(
    mut fun: Option<unsafe extern "C" fn(*mut FILE) -> ()>,
    mut fname: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut aux: *mut inentry = malloc(::core::mem::size_of::<inentry>()) as *mut inentry;
        if aux.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                292 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                292 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if aux
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut inentry
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                292 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                292 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*aux).fun = fun;
        (*aux).fname = strdup(fname);
        (*aux).next = inhead as *mut inentry;
        inhead = aux;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_keepalive_register_fname(
    mut fun: Option<unsafe extern "C" fn() -> ()>,
    mut fname: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut aux: *mut kaentry = malloc(::core::mem::size_of::<kaentry>()) as *mut kaentry;
        if aux.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                301 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                301 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if aux
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut kaentry
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                301 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                301 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*aux).fun = fun;
        (*aux).fname = strdup(fname);
        (*aux).next = kahead as *mut kaentry;
        kahead = aux;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_poll_register_fname(
    mut desc: Option<unsafe extern "C" fn(*mut pollfd, *mut uint32_t) -> ()>,
    mut serve: Option<unsafe extern "C" fn(*mut pollfd) -> ()>,
    mut dname: *const ::core::ffi::c_char,
    mut sname: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut aux: *mut pollentry = malloc(::core::mem::size_of::<pollentry>()) as *mut pollentry;
        if aux.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                310 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                310 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if aux
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut pollentry
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                310 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                310 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*aux).desc = desc;
        (*aux).serve = serve;
        (*aux).dname = strdup(dname);
        (*aux).sname = strdup(sname);
        (*aux).next = pollhead as *mut pollentry;
        pollhead = aux;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_eachloop_register_fname(
    mut fun: Option<unsafe extern "C" fn() -> ()>,
    mut fname: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut aux: *mut eloopentry =
            malloc(::core::mem::size_of::<eloopentry>()) as *mut eloopentry;
        if aux.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                321 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                321 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if aux
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut eloopentry
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                321 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                321 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*aux).fun = fun;
        (*aux).fname = strdup(fname);
        (*aux).next = eloophead as *mut eloopentry;
        eloophead = aux;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_chld_register_fname(
    mut pid: pid_t,
    mut fun: Option<unsafe extern "C" fn(pid_t, ::core::ffi::c_int) -> ()>,
    mut fname: *const ::core::ffi::c_char,
) {
    unsafe {
        let mut aux: *mut chldentry = malloc(::core::mem::size_of::<chldentry>()) as *mut chldentry;
        if aux.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                330 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                330 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if aux
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut chldentry
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                330 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                330 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*aux).pid = pid;
        (*aux).fun = fun;
        (*aux).fname = strdup(fname);
        (*aux).next = chldhead as *mut chldentry;
        chldhead = aux;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_msectime_register_fname(
    mut mseconds: uint32_t,
    mut offset: uint32_t,
    mut fun: Option<unsafe extern "C" fn() -> ()>,
    mut fname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut aux: *mut timeentry = ::core::ptr::null_mut::<timeentry>();
        let mut useconds: uint64_t = (1000 as uint64_t).wrapping_mul(mseconds as uint64_t);
        let mut usecoffset: uint64_t = (1000 as uint64_t).wrapping_mul(offset as uint64_t);
        if useconds == 0 as uint64_t || usecoffset >= useconds {
            return NULL;
        }
        aux = malloc(::core::mem::size_of::<timeentry>()) as *mut timeentry;
        if aux.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                346 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                346 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if aux
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut timeentry
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                346 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                346 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"aux\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*aux).nextevent = usecnow
            .wrapping_div(useconds)
            .wrapping_mul(useconds)
            .wrapping_add(usecoffset);
        while (*aux).nextevent < usecnow {
            (*aux).nextevent = (*aux).nextevent.wrapping_add(useconds);
        }
        (*aux).useconds = useconds;
        (*aux).usecoffset = usecoffset;
        (*aux).fun = fun;
        (*aux).fname = strdup(fname);
        (*aux).next = timehead as *mut timeentry;
        timehead = aux;
        return aux as *mut ::core::ffi::c_void;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_msectime_change(
    mut x: *mut ::core::ffi::c_void,
    mut mseconds: uint32_t,
    mut offset: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aux: *mut timeentry = x as *mut timeentry;
        let mut useconds: uint64_t = (1000 as uint64_t).wrapping_mul(mseconds as uint64_t);
        let mut usecoffset: uint64_t = (1000 as uint64_t).wrapping_mul(offset as uint64_t);
        if useconds == 0 as uint64_t || usecoffset >= useconds {
            return -1 as ::core::ffi::c_int;
        }
        (*aux).nextevent = usecnow
            .wrapping_div(useconds)
            .wrapping_mul(useconds)
            .wrapping_add(usecoffset);
        while (*aux).nextevent < usecnow {
            (*aux).nextevent = (*aux).nextevent.wrapping_add(useconds);
        }
        (*aux).useconds = useconds;
        (*aux).usecoffset = usecoffset;
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_time_register_fname(
    mut seconds: uint32_t,
    mut offset: uint32_t,
    mut fun: Option<unsafe extern "C" fn() -> ()>,
    mut fname: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    unsafe {
        return main_msectime_register_fname(
            (1000 as uint32_t).wrapping_mul(seconds),
            (1000 as uint32_t).wrapping_mul(offset),
            fun,
            fname,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_time_change(
    mut x: *mut ::core::ffi::c_void,
    mut seconds: uint32_t,
    mut offset: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        return main_msectime_change(
            x,
            (1000 as uint32_t).wrapping_mul(seconds),
            (1000 as uint32_t).wrapping_mul(offset),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_all_registered_entries() {
    unsafe {
        let mut de: *mut deentry = ::core::ptr::null_mut::<deentry>();
        let mut den: *mut deentry = ::core::ptr::null_mut::<deentry>();
        let mut ce: *mut ceentry = ::core::ptr::null_mut::<ceentry>();
        let mut cen: *mut ceentry = ::core::ptr::null_mut::<ceentry>();
        let mut we: *mut weentry = ::core::ptr::null_mut::<weentry>();
        let mut wen: *mut weentry = ::core::ptr::null_mut::<weentry>();
        let mut re: *mut rlentry = ::core::ptr::null_mut::<rlentry>();
        let mut ren: *mut rlentry = ::core::ptr::null_mut::<rlentry>();
        let mut ie: *mut inentry = ::core::ptr::null_mut::<inentry>();
        let mut ien: *mut inentry = ::core::ptr::null_mut::<inentry>();
        let mut pe: *mut pollentry = ::core::ptr::null_mut::<pollentry>();
        let mut pen: *mut pollentry = ::core::ptr::null_mut::<pollentry>();
        let mut ee: *mut eloopentry = ::core::ptr::null_mut::<eloopentry>();
        let mut een: *mut eloopentry = ::core::ptr::null_mut::<eloopentry>();
        let mut te: *mut timeentry = ::core::ptr::null_mut::<timeentry>();
        let mut ten: *mut timeentry = ::core::ptr::null_mut::<timeentry>();
        de = dehead;
        while !de.is_null() {
            den = (*de).next as *mut deentry;
            free((*de).fname as *mut ::core::ffi::c_void);
            free(de as *mut ::core::ffi::c_void);
            de = den;
        }
        ce = cehead;
        while !ce.is_null() {
            cen = (*ce).next as *mut ceentry;
            free((*ce).fname as *mut ::core::ffi::c_void);
            free(ce as *mut ::core::ffi::c_void);
            ce = cen;
        }
        we = wehead;
        while !we.is_null() {
            wen = (*we).next as *mut weentry;
            free((*we).fname as *mut ::core::ffi::c_void);
            free(we as *mut ::core::ffi::c_void);
            we = wen;
        }
        re = rlhead;
        while !re.is_null() {
            ren = (*re).next as *mut rlentry;
            free((*re).fname as *mut ::core::ffi::c_void);
            free(re as *mut ::core::ffi::c_void);
            re = ren;
        }
        ie = inhead;
        while !ie.is_null() {
            ien = (*ie).next as *mut inentry;
            free((*ie).fname as *mut ::core::ffi::c_void);
            free(ie as *mut ::core::ffi::c_void);
            ie = ien;
        }
        pe = pollhead;
        while !pe.is_null() {
            pen = (*pe).next as *mut pollentry;
            free((*pe).dname as *mut ::core::ffi::c_void);
            free((*pe).sname as *mut ::core::ffi::c_void);
            free(pe as *mut ::core::ffi::c_void);
            pe = pen;
        }
        ee = eloophead;
        while !ee.is_null() {
            een = (*ee).next as *mut eloopentry;
            free((*ee).fname as *mut ::core::ffi::c_void);
            free(ee as *mut ::core::ffi::c_void);
            ee = een;
        }
        te = timehead;
        while !te.is_null() {
            ten = (*te).next as *mut timeentry;
            free((*te).fname as *mut ::core::ffi::c_void);
            free(te as *mut ::core::ffi::c_void);
            te = ten;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn canexit() -> ::core::ffi::c_int {
    unsafe {
        let mut r: ::core::ffi::c_int = 0;
        let mut aux: *mut ceentry = ::core::ptr::null_mut::<ceentry>();
        aux = cehead;
        while !aux.is_null() {
            if lcall_trigger > 0.0f64 {
                lcall_start = monotonic_seconds();
            }
            r = (*aux).fun.expect("non-null function pointer")();
            if lcall_trigger > 0.0f64 {
                lcall_end = monotonic_seconds();
                if lcall_end - lcall_start > lcall_trigger {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"long call detected: %s : %.2lfms\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*aux).fname,
                        (lcall_end - lcall_start) * 1000.0f64,
                    );
                }
            }
            if r == 0 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
            aux = (*aux).next as *mut ceentry;
        }
        return 1 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_time_refresh() -> uint32_t {
    unsafe {
        let mut tv: timeval = timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        let mut res: uint32_t = 0;
        gettimeofday(&raw mut tv, NULL);
        now = tv.tv_sec as uint32_t;
        res = now;
        return res;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_time() -> uint32_t {
    unsafe {
        return now;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_utime() -> uint64_t {
    unsafe {
        let mut tv: timeval = timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        let mut usec: uint64_t = 0;
        gettimeofday(&raw mut tv, NULL);
        usec = tv.tv_sec as uint64_t;
        usec = usec.wrapping_mul(1000000 as uint64_t);
        usec = usec.wrapping_add(tv.tv_usec as uint64_t);
        return usec;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_start_time() -> uint32_t {
    unsafe {
        return start_time;
    }
}
#[inline]
unsafe extern "C" fn destruct() {
    unsafe {
        let mut deit: *mut deentry = ::core::ptr::null_mut::<deentry>();
        deit = dehead;
        while !deit.is_null() {
            if lcall_trigger > 0.0f64 {
                lcall_start = monotonic_seconds();
            }
            (*deit).fun.expect("non-null function pointer")();
            if lcall_trigger > 0.0f64 {
                lcall_end = monotonic_seconds();
                if lcall_end - lcall_start > lcall_trigger {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"long call detected: %s : %.2lfms\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*deit).fname,
                        (lcall_end - lcall_start) * 1000.0f64,
                    );
                }
            }
            deit = (*deit).next as *mut deentry;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_keep_alive() {
    unsafe {
        let mut loop_end: ::core::ffi::c_double = 0.;
        let mut tv: timeval = timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        let mut kait: *mut kaentry = ::core::ptr::null_mut::<kaentry>();
        loop_end = monotonic_seconds();
        if loop_end - loop_start > 5.0f64 {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"long loop detected (%.3lfs)\0".as_ptr() as *const ::core::ffi::c_char,
                loop_end - loop_start,
            );
        }
        loop_start = loop_end;
        gettimeofday(&raw mut tv, NULL);
        usecnow = tv.tv_sec as uint64_t;
        usecnow = usecnow.wrapping_mul(1000000 as uint64_t);
        usecnow = usecnow.wrapping_add(tv.tv_usec as uint64_t);
        now = tv.tv_sec as uint32_t;
        kait = kahead;
        while !kait.is_null() {
            if lcall_trigger > 0.0f64 {
                lcall_start = monotonic_seconds();
            }
            (*kait).fun.expect("non-null function pointer")();
            if lcall_trigger > 0.0f64 {
                lcall_end = monotonic_seconds();
                if lcall_end - lcall_start > lcall_trigger {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"long call detected: %s : %.2lfms\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*kait).fname,
                        (lcall_end - lcall_start) * 1000.0f64,
                    );
                }
            }
            kait = (*kait).next as *mut kaentry;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_reload() {
    unsafe {
        static mut firstflag: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut logminlevelstr: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut logelevatetostr: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut tmp: ::core::ffi::c_int = 0;
        lcall_trigger = cfg_getdouble(
            b"LONG_CALL_TRIGGER_MS\0".as_ptr() as *const ::core::ffi::c_char,
            0.0f64,
        ) / 1000.0f64;
        loop_usleep = cfg_getuint32(
            b"DEBUG_LOOP_SLEEP_MS\0".as_ptr() as *const ::core::ffi::c_char,
            0 as uint32_t,
        )
        .wrapping_mul(1000 as uint32_t) as ::core::ffi::c_double;
        logminlevelstr = cfg_getstr(
            b"SYSLOG_MIN_LEVEL\0".as_ptr() as *const ::core::ffi::c_char,
            b"INFO\0".as_ptr() as *const ::core::ffi::c_char,
        );
        logelevatetostr = cfg_getstr(
            b"SYSLOG_ELEVATE_TO\0".as_ptr() as *const ::core::ffi::c_char,
            b"NOTICE\0".as_ptr() as *const ::core::ffi::c_char,
        );
        tmp = mfs_log_str_to_pri(logminlevelstr);
        if tmp >= 0 as ::core::ffi::c_int {
            mfs_log_set_min_level(tmp);
        } else if firstflag == 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"error parsing SYSLOG_MIN_LEVEL option - using INFO\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            mfs_log_set_min_level(MFSLOG_INFO);
        } else {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"error parsing SYSLOG_MIN_LEVEL option - left unchanged\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        free(logminlevelstr as *mut ::core::ffi::c_void);
        tmp = mfs_log_str_to_pri(logelevatetostr);
        if tmp >= 0 as ::core::ffi::c_int {
            mfs_log_set_elevate_to(tmp);
        } else if firstflag == 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"error parsing SYSLOG_ELEVATE_TO option - using NOTICE\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            mfs_log_set_elevate_to(MFSLOG_NOTICE);
        } else {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"error parsing SYSLOG_ELEVATE_TO option - left unchanged\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        free(logelevatetostr as *mut ::core::ffi::c_void);
        firstflag = 1 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mainloop() -> ::core::ffi::c_int {
    unsafe {
        let mut prevtime: uint64_t = 0 as uint64_t;
        let mut tv: timeval = timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        let mut pollit: *mut pollentry = ::core::ptr::null_mut::<pollentry>();
        let mut eloopit: *mut eloopentry = ::core::ptr::null_mut::<eloopentry>();
        let mut timeit: *mut timeentry = ::core::ptr::null_mut::<timeentry>();
        let mut meit: *mut meentry = ::core::ptr::null_mut::<meentry>();
        let mut weit: *mut weentry = ::core::ptr::null_mut::<weentry>();
        let mut ceit: *mut ceentry = ::core::ptr::null_mut::<ceentry>();
        let mut rlit: *mut rlentry = ::core::ptr::null_mut::<rlentry>();
        let mut init: *mut inentry = ::core::ptr::null_mut::<inentry>();
        let mut infile: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut pdesc: *mut pollfd = ::core::ptr::null_mut::<pollfd>();
        let mut ndesc: uint32_t = 0;
        let mut loop_end: ::core::ffi::c_double = 0.;
        let mut i: ::core::ffi::c_int = 0;
        let mut t: ::core::ffi::c_int = 0;
        let mut r: ::core::ffi::c_int = 0;
        let mut status: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        t = 0 as ::core::ffi::c_int;
        r = 0 as ::core::ffi::c_int;
        pdesc = malloc(::core::mem::size_of::<pollfd>().wrapping_mul(MFSMAXFILES as size_t))
            as *mut pollfd;
        loop_start = monotonic_seconds();
        while t != 3 as ::core::ffi::c_int {
            if loop_usleep != 0. {
                portable_usleep(loop_usleep as uint64_t);
            }
            ndesc = 1 as uint32_t;
            (*pdesc.offset(0 as isize)).fd = signalpipe[0 as usize];
            (*pdesc.offset(0 as isize)).events = POLLIN as ::core::ffi::c_short;
            (*pdesc.offset(0 as isize)).revents = 0 as ::core::ffi::c_short;
            pollit = pollhead;
            while !pollit.is_null() {
                if lcall_trigger > 0.0f64 {
                    lcall_start = monotonic_seconds();
                }
                (*pollit).desc.expect("non-null function pointer")(pdesc, &raw mut ndesc);
                if lcall_trigger > 0.0f64 {
                    lcall_end = monotonic_seconds();
                    if lcall_end - lcall_start > lcall_trigger {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"long call detected: %s : %.2lfms\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*pollit).dname,
                            (lcall_end - lcall_start) * 1000.0f64,
                        );
                    }
                }
                pollit = (*pollit).next as *mut pollentry;
            }
            loop_end = monotonic_seconds();
            if loop_end - loop_start > 5.0f64 {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"long loop detected (%.3lfs)\0".as_ptr() as *const ::core::ffi::c_char,
                    loop_end - loop_start,
                );
            }
            i = poll(pdesc, ndesc as nfds_t, 10 as ::core::ffi::c_int);
            loop_start = monotonic_seconds();
            gettimeofday(&raw mut tv, NULL);
            usecnow = tv.tv_sec as uint64_t;
            usecnow = usecnow.wrapping_mul(1000000 as uint64_t);
            usecnow = usecnow.wrapping_add(tv.tv_usec as uint64_t);
            now = tv.tv_sec as uint32_t;
            if i < 0 as ::core::ffi::c_int {
                if !(*__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK) {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"poll returned EAGAIN\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    portable_usleep(10000 as uint64_t);
                    continue;
                } else if *__errno_location() != EINTR {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"poll error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        strerr(*__errno_location()),
                    );
                    break;
                }
            } else if i > 0 as ::core::ffi::c_int {
                if (*pdesc.offset(0 as isize)).revents as ::core::ffi::c_int & POLLIN != 0 {
                    let mut sigid: uint8_t = 0;
                    if read(
                        signalpipe[0 as usize],
                        &raw mut sigid as *mut ::core::ffi::c_void,
                        1 as size_t,
                    ) == 1 as ssize_t
                    {
                        if sigid as ::core::ffi::c_int == '\u{1}' as ::core::ffi::c_int
                            && t == 0 as ::core::ffi::c_int
                        {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_NOTICE,
                                b"terminate signal received\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            t = 1 as ::core::ffi::c_int;
                        } else if sigid as ::core::ffi::c_int == '\u{2}' as ::core::ffi::c_int {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_INFO,
                                b"reloading config files\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            r = 1 as ::core::ffi::c_int;
                        } else if sigid as ::core::ffi::c_int == '\u{3}' as ::core::ffi::c_int {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_INFO,
                                b"child finished\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            r = 2 as ::core::ffi::c_int;
                        } else if sigid as ::core::ffi::c_int == '\u{4}' as ::core::ffi::c_int {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_INFO,
                                b"log extra info\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            r = 3 as ::core::ffi::c_int;
                        } else if sigid as ::core::ffi::c_int == '\u{5}' as ::core::ffi::c_int {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_NOTICE,
                                b"unexpected alarm/prof signal received - ignoring\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        } else if sigid as ::core::ffi::c_int == '\u{6}' as ::core::ffi::c_int {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_NOTICE,
                                b"internal terminate request\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            t = 1 as ::core::ffi::c_int;
                            status = 1 as ::core::ffi::c_int;
                        }
                    }
                }
                pollit = pollhead;
                while !pollit.is_null() {
                    if lcall_trigger > 0.0f64 {
                        lcall_start = monotonic_seconds();
                    }
                    (*pollit).serve.expect("non-null function pointer")(pdesc);
                    if lcall_trigger > 0.0f64 {
                        lcall_end = monotonic_seconds();
                        if lcall_end - lcall_start > lcall_trigger {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"long call detected: %s : %.2lfms\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                (*pollit).sname,
                                (lcall_end - lcall_start) * 1000.0f64,
                            );
                        }
                    }
                    pollit = (*pollit).next as *mut pollentry;
                }
            }
            eloopit = eloophead;
            while !eloopit.is_null() {
                if lcall_trigger > 0.0f64 {
                    lcall_start = monotonic_seconds();
                }
                (*eloopit).fun.expect("non-null function pointer")();
                if lcall_trigger > 0.0f64 {
                    lcall_end = monotonic_seconds();
                    if lcall_end - lcall_start > lcall_trigger {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"long call detected: %s : %.2lfms\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*eloopit).fname,
                            (lcall_end - lcall_start) * 1000.0f64,
                        );
                    }
                }
                eloopit = (*eloopit).next as *mut eloopentry;
            }
            if usecnow < prevtime {
                timeit = timehead;
                while !timeit.is_null() {
                    let mut previous_time_to_run: uint64_t =
                        (*timeit).nextevent.wrapping_sub(prevtime);
                    if previous_time_to_run > (*timeit).useconds {
                        previous_time_to_run = (*timeit).useconds;
                    }
                    (*timeit).nextevent = usecnow
                        .wrapping_div((*timeit).useconds)
                        .wrapping_mul((*timeit).useconds)
                        .wrapping_add((*timeit).usecoffset);
                    while (*timeit).nextevent <= usecnow.wrapping_add(previous_time_to_run) {
                        (*timeit).nextevent = (*timeit).nextevent.wrapping_add((*timeit).useconds);
                    }
                    timeit = (*timeit).next as *mut timeentry;
                }
            } else if usecnow > prevtime.wrapping_add(5000000 as uint64_t) {
                timeit = timehead;
                while !timeit.is_null() {
                    (*timeit).nextevent = usecnow
                        .wrapping_div((*timeit).useconds)
                        .wrapping_mul((*timeit).useconds)
                        .wrapping_add((*timeit).usecoffset);
                    while usecnow >= (*timeit).nextevent {
                        (*timeit).nextevent = (*timeit).nextevent.wrapping_add((*timeit).useconds);
                    }
                    timeit = (*timeit).next as *mut timeentry;
                }
            }
            timeit = timehead;
            while !timeit.is_null() {
                if usecnow >= (*timeit).nextevent {
                    let mut eventcounter: uint32_t = 0 as uint32_t;
                    while usecnow >= (*timeit).nextevent && eventcounter < 10 as uint32_t {
                        if lcall_trigger > 0.0f64 {
                            lcall_start = monotonic_seconds();
                        }
                        (*timeit).fun.expect("non-null function pointer")();
                        if lcall_trigger > 0.0f64 {
                            lcall_end = monotonic_seconds();
                            if lcall_end - lcall_start > lcall_trigger {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_WARNING,
                                    b"long call detected: %s : %.2lfms\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    (*timeit).fname,
                                    (lcall_end - lcall_start) * 1000.0f64,
                                );
                            }
                        }
                        (*timeit).nextevent = (*timeit).nextevent.wrapping_add((*timeit).useconds);
                        eventcounter = eventcounter.wrapping_add(1);
                    }
                    if usecnow >= (*timeit).nextevent {
                        (*timeit).nextevent = usecnow
                            .wrapping_div((*timeit).useconds)
                            .wrapping_mul((*timeit).useconds)
                            .wrapping_add((*timeit).usecoffset);
                        while usecnow >= (*timeit).nextevent {
                            (*timeit).nextevent =
                                (*timeit).nextevent.wrapping_add((*timeit).useconds);
                        }
                    }
                }
                timeit = (*timeit).next as *mut timeentry;
            }
            prevtime = usecnow;
            if r == 1 as ::core::ffi::c_int {
                cfg_reload();
                main_reload();
                rlit = rlhead;
                while !rlit.is_null() {
                    if lcall_trigger > 0.0f64 {
                        lcall_start = monotonic_seconds();
                    }
                    (*rlit).fun.expect("non-null function pointer")();
                    if lcall_trigger > 0.0f64 {
                        lcall_end = monotonic_seconds();
                        if lcall_end - lcall_start > lcall_trigger {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"long call detected: %s : %.2lfms\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                (*rlit).fname,
                                (lcall_end - lcall_start) * 1000.0f64,
                            );
                        }
                    }
                    rlit = (*rlit).next as *mut rlentry;
                }
                r = 0 as ::core::ffi::c_int;
            } else if r == 2 as ::core::ffi::c_int {
                let mut chldit: *mut chldentry = ::core::ptr::null_mut::<chldentry>();
                let mut chldptr: *mut *mut chldentry = ::core::ptr::null_mut::<*mut chldentry>();
                let mut pid: pid_t = 0;
                let mut st: ::core::ffi::c_int = 0;
                loop {
                    pid = waitpid(-1 as __pid_t, &raw mut st, WNOHANG) as pid_t;
                    if pid <= 0 as ::core::ffi::c_int {
                        break;
                    }
                    chldptr = &raw mut chldhead;
                    loop {
                        chldit = *chldptr;
                        if chldit.is_null() {
                            break;
                        }
                        if (*chldit).pid == pid {
                            if lcall_trigger > 0.0f64 {
                                lcall_start = monotonic_seconds();
                            }
                            (*chldit).fun.expect("non-null function pointer")(pid, st);
                            if lcall_trigger > 0.0f64 {
                                lcall_end = monotonic_seconds();
                                if lcall_end - lcall_start > lcall_trigger {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_WARNING,
                                        b"long call detected: %s : %.2lfms\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        (*chldit).fname,
                                        (lcall_end - lcall_start) * 1000.0f64,
                                    );
                                }
                            }
                            *chldptr = (*chldit).next as *mut chldentry;
                            free(chldit as *mut ::core::ffi::c_void);
                        } else {
                            chldptr = &raw mut (*chldit).next as *mut *mut chldentry;
                        }
                    }
                }
                r = 0 as ::core::ffi::c_int;
            } else if r == 3 as ::core::ffi::c_int {
                infile = fopen(
                    b".mfsmetalogger_info.txt\0".as_ptr() as *const ::core::ffi::c_char,
                    b"w\0".as_ptr() as *const ::core::ffi::c_char,
                ) as *mut FILE;
                if infile.is_null() {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"can't create info file\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                } else {
                    fprintf(
                        infile,
                        b"[general]\n\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    fprintf(
                        infile,
                        b"version: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                        VERSSTR.as_ptr(),
                    );
                    fprintf(
                        infile,
                        b"build: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                        b"2106\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    fprintf(
                        infile,
                        b"timestamp: %ld\n\0".as_ptr() as *const ::core::ffi::c_char,
                        time(::core::ptr::null_mut::<time_t>()),
                    );
                    fprintf(infile, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
                    cfg_info(infile);
                    init = inhead;
                    while !init.is_null() {
                        if lcall_trigger > 0.0f64 {
                            lcall_start = monotonic_seconds();
                        }
                        (*init).fun.expect("non-null function pointer")(infile);
                        if lcall_trigger > 0.0f64 {
                            lcall_end = monotonic_seconds();
                            if lcall_end - lcall_start > lcall_trigger {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_WARNING,
                                    b"long call detected: %s : %.2lfms\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    (*init).fname,
                                    (lcall_end - lcall_start) * 1000.0f64,
                                );
                            }
                        }
                        init = (*init).next as *mut inentry;
                    }
                    fclose(infile);
                }
                r = 0 as ::core::ffi::c_int;
            }
            if t == 1 as ::core::ffi::c_int {
                i = 1 as ::core::ffi::c_int;
                meit = mehead;
                while !meit.is_null() && i != 0 {
                    if lcall_trigger > 0.0f64 {
                        lcall_start = monotonic_seconds();
                    }
                    if (*meit).fun.expect("non-null function pointer")() == 0 as ::core::ffi::c_int
                    {
                        i = 0 as ::core::ffi::c_int;
                    }
                    if lcall_trigger > 0.0f64 {
                        lcall_end = monotonic_seconds();
                        if lcall_end - lcall_start > lcall_trigger {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"long call detected: %s : %.2lfms\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                (*meit).fname,
                                (lcall_end - lcall_start) * 1000.0f64,
                            );
                        }
                    }
                    meit = (*meit).next as *mut meentry;
                }
                if i == 1 as ::core::ffi::c_int {
                    weit = wehead;
                    while !weit.is_null() {
                        if lcall_trigger > 0.0f64 {
                            lcall_start = monotonic_seconds();
                        }
                        (*weit).fun.expect("non-null function pointer")();
                        if lcall_trigger > 0.0f64 {
                            lcall_end = monotonic_seconds();
                            if lcall_end - lcall_start > lcall_trigger {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_WARNING,
                                    b"long call detected: %s : %.2lfms\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    (*weit).fname,
                                    (lcall_end - lcall_start) * 1000.0f64,
                                );
                            }
                        }
                        weit = (*weit).next as *mut weentry;
                    }
                    t = 2 as ::core::ffi::c_int;
                }
            }
            if t == 2 as ::core::ffi::c_int {
                i = 1 as ::core::ffi::c_int;
                ceit = cehead;
                while !ceit.is_null() && i != 0 {
                    if lcall_trigger > 0.0f64 {
                        lcall_start = monotonic_seconds();
                    }
                    if (*ceit).fun.expect("non-null function pointer")() == 0 as ::core::ffi::c_int
                    {
                        i = 0 as ::core::ffi::c_int;
                    }
                    if lcall_trigger > 0.0f64 {
                        lcall_end = monotonic_seconds();
                        if lcall_end - lcall_start > lcall_trigger {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"long call detected: %s : %.2lfms\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                (*ceit).fname,
                                (lcall_end - lcall_start) * 1000.0f64,
                            );
                        }
                    }
                    ceit = (*ceit).next as *mut ceentry;
                }
                if i != 0 {
                    t = 3 as ::core::ffi::c_int;
                }
            }
        }
        free(pdesc as *mut ::core::ffi::c_void);
        return status;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn initialize() -> ::core::ffi::c_int {
    unsafe {
        let mut i: uint32_t = 0;
        let mut ok: ::core::ffi::c_int = 0;
        ok = 1 as ::core::ffi::c_int;
        i = 0 as uint32_t;
        while ::core::mem::transmute::<runfn, ::core::ffi::c_long>(RunTab[i as usize].r#fn)
            != 0 as ::core::ffi::c_long
            && ok != 0
        {
            now = time(::core::ptr::null_mut::<time_t>()) as uint32_t;
            if RunTab[i as usize].r#fn.expect("non-null function pointer")()
                < 0 as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"init: %s failed !!!\0".as_ptr() as *const ::core::ffi::c_char,
                    RunTab[i as usize].name,
                );
                ok = 0 as ::core::ffi::c_int;
            }
            i = i.wrapping_add(1);
        }
        return ok;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn restore() -> ::core::ffi::c_int {
    unsafe {
        let mut i: uint32_t = 0;
        let mut ok: ::core::ffi::c_int = 0;
        ok = 1 as ::core::ffi::c_int;
        i = 0 as uint32_t;
        while ::core::mem::transmute::<runfn, ::core::ffi::c_long>(RestoreRunTab[i as usize].r#fn)
            != 0 as ::core::ffi::c_long
            && ok != 0
        {
            now = time(::core::ptr::null_mut::<time_t>()) as uint32_t;
            if RestoreRunTab[i as usize]
                .r#fn
                .expect("non-null function pointer")()
                < 0 as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"restore: %s failed !!!\0".as_ptr() as *const ::core::ffi::c_char,
                    RestoreRunTab[i as usize].name,
                );
                ok = 0 as ::core::ffi::c_int;
            }
            i = i.wrapping_add(1);
        }
        return ok;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn initialize_late() -> ::core::ffi::c_int {
    unsafe {
        let mut i: uint32_t = 0;
        let mut ok: ::core::ffi::c_int = 0;
        ok = 1 as ::core::ffi::c_int;
        i = 0 as uint32_t;
        while ::core::mem::transmute::<runfn, ::core::ffi::c_long>(LateRunTab[i as usize].r#fn)
            != 0 as ::core::ffi::c_long
            && ok != 0
        {
            now = time(::core::ptr::null_mut::<time_t>()) as uint32_t;
            if LateRunTab[i as usize]
                .r#fn
                .expect("non-null function pointer")()
                < 0 as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"init: %s failed !!!\0".as_ptr() as *const ::core::ffi::c_char,
                    LateRunTab[i as usize].name,
                );
                ok = 0 as ::core::ffi::c_int;
            }
            i = i.wrapping_add(1);
        }
        now = time(::core::ptr::null_mut::<time_t>()) as uint32_t;
        start_time = now;
        return ok;
    }
}
static mut termsignal: [::core::ffi::c_int; 2] = [SIGTERM, -1 as ::core::ffi::c_int];
static mut reloadsignal: [::core::ffi::c_int; 2] = [SIGHUP, -1 as ::core::ffi::c_int];
static mut infosignal: [::core::ffi::c_int; 2] = [SIGUSR1, -1 as ::core::ffi::c_int];
static mut chldsignal: [::core::ffi::c_int; 3] = [SIGCHLD, SIGCLD, -1 as ::core::ffi::c_int];
static mut ignoresignal: [::core::ffi::c_int; 7] = [
    SIGQUIT,
    SIGPIPE,
    SIGTSTP,
    SIGTTIN,
    SIGTTOU,
    SIGUSR2,
    -1 as ::core::ffi::c_int,
];
static mut alarmsignal: [::core::ffi::c_int; 4] =
    [SIGALRM, SIGVTALRM, SIGPROF, -1 as ::core::ffi::c_int];
static mut daemonignoresignal: [::core::ffi::c_int; 2] = [SIGINT, -1 as ::core::ffi::c_int];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn termhandle(mut signo: ::core::ffi::c_int) {
    unsafe {
        signo = write(
            signalpipe[1 as usize],
            b"\x01\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            1 as size_t,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn reloadhandle(mut signo: ::core::ffi::c_int) {
    unsafe {
        signo = write(
            signalpipe[1 as usize],
            b"\x02\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            1 as size_t,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chldhandle(mut signo: ::core::ffi::c_int) {
    unsafe {
        signo = write(
            signalpipe[1 as usize],
            b"\x03\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            1 as size_t,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn infohandle(mut signo: ::core::ffi::c_int) {
    unsafe {
        signo = write(
            signalpipe[1 as usize],
            b"\x04\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            1 as size_t,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn alarmhandle(mut signo: ::core::ffi::c_int) {
    unsafe {
        signo = write(
            signalpipe[1 as usize],
            b"\x05\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            1 as size_t,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_signal_handlers(mut daemonflag: ::core::ffi::c_int) {
    unsafe {
        let mut sa: sigaction = sigaction {
            __sigaction_handler: C2Rust_Unnamed_9 { sa_handler: None },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        let mut i: uint32_t = 0;
        let mut _mfs_assert_ret: ::core::ffi::c_int =
            pipe(&raw mut signalpipe as *mut ::core::ffi::c_int);
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
                    b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    987 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pipe(signalpipe)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    987 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pipe(signalpipe)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    987 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pipe(signalpipe)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    987 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pipe(signalpipe)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    987 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pipe(signalpipe)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    987 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pipe(signalpipe)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        sa.sa_flags = SA_RESTART;
        sigemptyset(&raw mut sa.sa_mask);
        sa.__sigaction_handler.sa_handler =
            Some(termhandle as unsafe extern "C" fn(::core::ffi::c_int) -> ()) as __sighandler_t;
        i = 0 as uint32_t;
        while termsignal[i as usize] > 0 as ::core::ffi::c_int {
            sigaction(
                termsignal[i as usize],
                &raw mut sa,
                ::core::ptr::null_mut::<sigaction>(),
            );
            i = i.wrapping_add(1);
        }
        sa.__sigaction_handler.sa_handler =
            Some(reloadhandle as unsafe extern "C" fn(::core::ffi::c_int) -> ()) as __sighandler_t;
        i = 0 as uint32_t;
        while reloadsignal[i as usize] > 0 as ::core::ffi::c_int {
            sigaction(
                reloadsignal[i as usize],
                &raw mut sa,
                ::core::ptr::null_mut::<sigaction>(),
            );
            i = i.wrapping_add(1);
        }
        sa.__sigaction_handler.sa_handler =
            Some(infohandle as unsafe extern "C" fn(::core::ffi::c_int) -> ()) as __sighandler_t;
        i = 0 as uint32_t;
        while infosignal[i as usize] > 0 as ::core::ffi::c_int {
            sigaction(
                infosignal[i as usize],
                &raw mut sa,
                ::core::ptr::null_mut::<sigaction>(),
            );
            i = i.wrapping_add(1);
        }
        sa.__sigaction_handler.sa_handler =
            Some(alarmhandle as unsafe extern "C" fn(::core::ffi::c_int) -> ()) as __sighandler_t;
        i = 0 as uint32_t;
        while alarmsignal[i as usize] > 0 as ::core::ffi::c_int {
            sigaction(
                alarmsignal[i as usize],
                &raw mut sa,
                ::core::ptr::null_mut::<sigaction>(),
            );
            i = i.wrapping_add(1);
        }
        sa.__sigaction_handler.sa_handler =
            Some(chldhandle as unsafe extern "C" fn(::core::ffi::c_int) -> ()) as __sighandler_t;
        i = 0 as uint32_t;
        while chldsignal[i as usize] > 0 as ::core::ffi::c_int {
            sigaction(
                chldsignal[i as usize],
                &raw mut sa,
                ::core::ptr::null_mut::<sigaction>(),
            );
            i = i.wrapping_add(1);
        }
        sa.__sigaction_handler.sa_handler = ::core::mem::transmute::<
            ::libc::intptr_t,
            __sighandler_t,
        >(1 as ::core::ffi::c_int as ::libc::intptr_t);
        i = 0 as uint32_t;
        while ignoresignal[i as usize] > 0 as ::core::ffi::c_int {
            sigaction(
                ignoresignal[i as usize],
                &raw mut sa,
                ::core::ptr::null_mut::<sigaction>(),
            );
            i = i.wrapping_add(1);
        }
        sa.__sigaction_handler.sa_handler = (if daemonflag != 0 {
            ::core::mem::transmute::<
                ::libc::intptr_t,
                Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
            >(1 as ::core::ffi::c_int as ::libc::intptr_t)
        } else {
            Some(termhandle as unsafe extern "C" fn(::core::ffi::c_int) -> ())
        }) as __sighandler_t;
        i = 0 as uint32_t;
        while daemonignoresignal[i as usize] > 0 as ::core::ffi::c_int {
            sigaction(
                daemonignoresignal[i as usize],
                &raw mut sa,
                ::core::ptr::null_mut::<sigaction>(),
            );
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_syslog_wrapper(
    mut priority: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) {
    unsafe {
        syslog(
            priority,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            msg,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_exit() {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = write(
            signalpipe[1 as usize],
            b"\x06\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            1 as size_t,
        ) as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn signal_cleanup() {
    unsafe {
        close(signalpipe[0 as usize]);
        close(signalpipe[1 as usize]);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn changeugid() {
    unsafe {
        let mut pwdgrpbuff: [::core::ffi::c_char; 16384] = [0; 16384];
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
        let mut wuser: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut wgroup: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut wrk_uid: uid_t = 0;
        let mut wrk_gid: gid_t = 0;
        let mut gidok: ::core::ffi::c_int = 0;
        if geteuid() == 0 as __uid_t {
            wuser = cfg_getstr(
                b"WORKING_USER\0".as_ptr() as *const ::core::ffi::c_char,
                DEFAULT_USER.as_ptr(),
            );
            wgroup = cfg_getstr(
                b"WORKING_GROUP\0".as_ptr() as *const ::core::ffi::c_char,
                DEFAULT_GROUP.as_ptr(),
            );
            gidok = 0 as ::core::ffi::c_int;
            wrk_gid = -1 as ::core::ffi::c_int as gid_t;
            if *wgroup.offset(0 as isize) as ::core::ffi::c_int == '#' as ::core::ffi::c_int {
                wrk_gid = strtol(
                    wgroup.offset(1 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                    10 as ::core::ffi::c_int,
                ) as gid_t;
                gidok = 1 as ::core::ffi::c_int;
            } else if *wgroup.offset(0 as isize) != 0 {
                if getgrnam_r(
                    wgroup,
                    &raw mut grp,
                    &raw mut pwdgrpbuff as *mut ::core::ffi::c_char,
                    16384 as size_t,
                    &raw mut gr,
                ) != 0 as ::core::ffi::c_int
                {
                    gr = ::core::ptr::null_mut::<group>();
                }
                if gr.is_null() {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"%s: no such group !!!\0".as_ptr() as *const ::core::ffi::c_char,
                        wgroup,
                    );
                    exit(1 as ::core::ffi::c_int);
                }
                wrk_gid = (*gr).gr_gid as gid_t;
                gidok = 1 as ::core::ffi::c_int;
            }
            if *wuser.offset(0 as isize) as ::core::ffi::c_int == '#' as ::core::ffi::c_int {
                wrk_uid = strtol(
                    wuser.offset(1 as ::core::ffi::c_int as isize),
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                    10 as ::core::ffi::c_int,
                ) as uid_t;
                if gidok == 0 as ::core::ffi::c_int {
                    getpwuid_r(
                        wrk_uid as __uid_t,
                        &raw mut pwd,
                        &raw mut pwdgrpbuff as *mut ::core::ffi::c_char,
                        16384 as size_t,
                        &raw mut pw,
                    );
                    if pw.is_null() {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_ERR,
                            b"%s: no such user id - can't obtain group id\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            wuser.offset(1 as ::core::ffi::c_int as isize),
                        );
                        exit(1 as ::core::ffi::c_int);
                    }
                    wrk_gid = (*pw).pw_gid as gid_t;
                }
            } else {
                if getpwnam_r(
                    wuser,
                    &raw mut pwd,
                    &raw mut pwdgrpbuff as *mut ::core::ffi::c_char,
                    16384 as size_t,
                    &raw mut pw,
                ) != 0 as ::core::ffi::c_int
                {
                    pw = ::core::ptr::null_mut::<passwd>();
                }
                if pw.is_null() {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"%s: no such user !!!\0".as_ptr() as *const ::core::ffi::c_char,
                        wuser,
                    );
                    exit(1 as ::core::ffi::c_int);
                }
                wrk_uid = (*pw).pw_uid as uid_t;
                if gidok == 0 as ::core::ffi::c_int {
                    wrk_gid = (*pw).pw_gid as gid_t;
                }
            }
            free(wuser as *mut ::core::ffi::c_void);
            free(wgroup as *mut ::core::ffi::c_void);
            if setgid(wrk_gid as __gid_t) < 0 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"can't set gid to %d\0".as_ptr() as *const ::core::ffi::c_char,
                    wrk_gid as ::core::ffi::c_int,
                );
                exit(1 as ::core::ffi::c_int);
            } else {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_INFO,
                    b"set gid to %d\0".as_ptr() as *const ::core::ffi::c_char,
                    wrk_gid as ::core::ffi::c_int,
                );
            }
            if setgroups(0 as size_t, ::core::ptr::null::<__gid_t>()) < 0 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"can't clear auxiliary groups\0".as_ptr() as *const ::core::ffi::c_char,
                );
                exit(1 as ::core::ffi::c_int);
            } else {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_INFO,
                    b"cleared auxiliary groups\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            if setuid(wrk_uid as __uid_t) < 0 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"can't set uid to %d\0".as_ptr() as *const ::core::ffi::c_char,
                    wrk_uid as ::core::ffi::c_int,
                );
                exit(1 as ::core::ffi::c_int);
            } else {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_INFO,
                    b"set uid to %d\0".as_ptr() as *const ::core::ffi::c_char,
                    wrk_uid as ::core::ffi::c_int,
                );
            }
        }
    }
}
static mut lfd: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mylock(mut fd: ::core::ffi::c_int) -> pid_t {
    unsafe {
        let mut fl: flock = flock {
            l_type: 0,
            l_whence: 0,
            l_start: 0,
            l_len: 0,
            l_pid: 0,
        };
        fl.l_start = 0 as __off64_t;
        fl.l_len = 0 as __off64_t;
        fl.l_pid = getpid();
        fl.l_type = F_WRLCK as ::core::ffi::c_short;
        fl.l_whence = SEEK_SET as ::core::ffi::c_short;
        loop {
            if fcntl(fd, F_SETLK, &raw mut fl) >= 0 as ::core::ffi::c_int {
                return 0 as pid_t;
            }
            if *__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK {
                return -1 as pid_t;
            }
            if fcntl(fd, F_GETLK, &raw mut fl) < 0 as ::core::ffi::c_int {
                return -1 as pid_t;
            }
            if fl.l_type as ::core::ffi::c_int != F_UNLCK {
                return fl.l_pid as pid_t;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wdunlock() {
    unsafe {
        if lfd >= 0 as ::core::ffi::c_int {
            close(lfd);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wdlock(mut runmode: uint8_t, mut timeout: uint32_t) -> uint8_t {
    unsafe {
        let mut ownerpid: pid_t = 0;
        let mut newownerpid: pid_t = 0;
        let mut l: uint32_t = 0;
        lfd = open(
            b".mfsmetalogger.lock\0".as_ptr() as *const ::core::ffi::c_char,
            O_WRONLY | O_CREAT,
            0o666 as ::core::ffi::c_int,
        );
        if lfd < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"can't create lockfile in working directory\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return 1 as uint8_t;
        }
        ownerpid = mylock(lfd);
        if ownerpid < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"fcntl error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return 1 as uint8_t;
        }
        if ownerpid > 0 as ::core::ffi::c_int {
            if runmode as ::core::ffi::c_int == RM_TEST {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_INFO,
                    b"mfsmetalogger pid: %ld\0".as_ptr() as *const ::core::ffi::c_char,
                    ownerpid as ::core::ffi::c_long,
                );
                return 0 as uint8_t;
            }
            if runmode as ::core::ffi::c_int == RM_START {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"can't start: lockfile is already locked by another process\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return 1 as uint8_t;
            }
            if runmode as ::core::ffi::c_int == RM_RELOAD {
                if kill(ownerpid as __pid_t, SIGHUP) < 0 as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"can't send reload signal to lock owner\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return 1 as uint8_t;
                }
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_INFO,
                    b"reload signal has been sent\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return 0 as uint8_t;
            }
            if runmode as ::core::ffi::c_int == RM_INFO {
                if kill(ownerpid as __pid_t, SIGUSR1) < 0 as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"can't send info signal to lock owner\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return 1 as uint8_t;
                }
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_INFO,
                    b"info signal has been sent\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return 0 as uint8_t;
            }
            if runmode as ::core::ffi::c_int == RM_KILL {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_INFO,
                    b"sending SIGKILL to lock owner (pid:%ld)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ownerpid as ::core::ffi::c_long,
                );
                if kill(ownerpid as __pid_t, SIGKILL) < 0 as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"can't kill lock owner\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return 1 as uint8_t;
                }
            } else {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_INFO,
                    b"sending SIGTERM to lock owner (pid:%ld)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ownerpid as ::core::ffi::c_long,
                );
                if kill(ownerpid as __pid_t, SIGTERM) < 0 as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"can't kill lock owner\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return 1 as uint8_t;
                }
            }
            l = 0 as uint32_t;
            fprintf(
                stderr,
                b"waiting for termination ...\0".as_ptr() as *const ::core::ffi::c_char,
            );
            fflush(stderr);
            loop {
                newownerpid = mylock(lfd);
                if newownerpid < 0 as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"fcntl error\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return 1 as uint8_t;
                }
                if newownerpid > 0 as ::core::ffi::c_int {
                    l = l.wrapping_add(1);
                    if l >= timeout {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"about %u seconds passed and lockfile is still locked - giving up\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            l,
                        );
                        fprintf(
                            stderr,
                            b":giving up\n\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        return 1 as uint8_t;
                    }
                    if l.wrapping_rem(10 as uint32_t) == 0 as uint32_t {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"about %u seconds passed and lock still exists\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            l,
                        );
                        fprintf(stderr, b".\0".as_ptr() as *const ::core::ffi::c_char);
                        fflush(stderr);
                    }
                    if newownerpid != ownerpid {
                        fprintf(
                            stderr,
                            b"\nnew lock owner detected\n\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if runmode as ::core::ffi::c_int == RM_KILL {
                            fprintf(
                                stderr,
                                b":sending SIGKILL to lock owner (pid:%ld):\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                newownerpid as ::core::ffi::c_long,
                            );
                            fflush(stderr);
                            if kill(newownerpid as __pid_t, SIGKILL) < 0 as ::core::ffi::c_int {
                                mfs_log(
                                    MFSLOG_ERRNO_SYSLOG_STDERR,
                                    MFSLOG_ERR,
                                    b"can't kill lock owner\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                return 1 as uint8_t;
                            }
                        } else {
                            fprintf(
                                stderr,
                                b":sending SIGTERM to lock owner (pid:%ld):\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                newownerpid as ::core::ffi::c_long,
                            );
                            fflush(stderr);
                            if kill(newownerpid as __pid_t, SIGTERM) < 0 as ::core::ffi::c_int {
                                mfs_log(
                                    MFSLOG_ERRNO_SYSLOG_STDERR,
                                    MFSLOG_ERR,
                                    b"can't kill lock owner\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                return 1 as uint8_t;
                            }
                        }
                        ownerpid = newownerpid;
                    }
                }
                sleep(1 as ::core::ffi::c_uint);
                if newownerpid == 0 as ::core::ffi::c_int {
                    break;
                }
            }
            fprintf(
                stderr,
                b" terminated\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return 0 as uint8_t;
        }
        if runmode as ::core::ffi::c_int == RM_START || runmode as ::core::ffi::c_int == RM_RESTART
        {
            let mut pidstr: [::core::ffi::c_char; 20] = [0; 20];
            l = snprintf(
                &raw mut pidstr as *mut ::core::ffi::c_char,
                20 as size_t,
                b"%ld\n\0".as_ptr() as *const ::core::ffi::c_char,
                getpid() as ::core::ffi::c_long,
            ) as uint32_t;
            if ftruncate(lfd, 0 as __off64_t) < 0 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"can't truncate pidfile\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            if write(
                lfd,
                &raw mut pidstr as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                l as size_t,
            ) != l as ssize_t
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"can't write pid to pidfile\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_INFO,
                b"lockfile created and locked\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if runmode as ::core::ffi::c_int == RM_TRY_RESTART {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"can't find process to restart\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return 1 as uint8_t;
        } else if runmode as ::core::ffi::c_int == RM_STOP
            || runmode as ::core::ffi::c_int == RM_KILL
        {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"can't find process to terminate\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return 0 as uint8_t;
        } else if runmode as ::core::ffi::c_int == RM_RELOAD {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"can't find process to send reload signal\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return 1 as uint8_t;
        } else if runmode as ::core::ffi::c_int == RM_INFO {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"can't find process to send info signal\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return 1 as uint8_t;
        } else if runmode as ::core::ffi::c_int == RM_TEST {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_NOTICE,
                b"mfsmetalogger is not running\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return 1 as uint8_t;
        }
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn makedaemon() {
    unsafe {
        let mut f: ::core::ffi::c_int = 0;
        let mut pipebuff: [uint8_t; 1000] = [0; 1000];
        let mut r: ssize_t = 0;
        let mut happy: size_t = 0;
        let mut piped: [::core::ffi::c_int; 2] = [0; 2];
        fflush(stdout);
        fflush(stderr);
        if pipe(&raw mut piped as *mut ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"pipe error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            exit(1 as ::core::ffi::c_int);
        }
        f = fork() as ::core::ffi::c_int;
        if f < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"first fork error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                strerr(*__errno_location()),
            );
            exit(1 as ::core::ffi::c_int);
        }
        if f > 0 as ::core::ffi::c_int {
            wait(&raw mut f);
            if f != 0 {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"Child status: %d\0".as_ptr() as *const ::core::ffi::c_char,
                    f,
                );
                exit(1 as ::core::ffi::c_int);
            }
            close(piped[1 as usize]);
            loop {
                r = read(
                    piped[0 as usize],
                    &raw mut pipebuff as *mut uint8_t as *mut ::core::ffi::c_void,
                    1000 as size_t,
                );
                if r == 0 {
                    break;
                }
                if r > 0 as ssize_t {
                    if pipebuff[(r - 1 as ssize_t) as usize] as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        if r > 1 as ssize_t {
                            happy = fwrite(
                                &raw mut pipebuff as *mut uint8_t as *const ::core::ffi::c_void,
                                1 as size_t,
                                (r - 1 as ssize_t) as size_t,
                                stderr,
                            ) as size_t;
                        }
                        exit(1 as ::core::ffi::c_int);
                    }
                    happy = fwrite(
                        &raw mut pipebuff as *mut uint8_t as *const ::core::ffi::c_void,
                        1 as size_t,
                        r as size_t,
                        stderr,
                    ) as size_t;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"Error reading pipe: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        strerr(*__errno_location()),
                    );
                    exit(1 as ::core::ffi::c_int);
                }
            }
            exit(0 as ::core::ffi::c_int);
        }
        setsid();
        setpgid(0 as __pid_t, getpid());
        f = fork() as ::core::ffi::c_int;
        if f < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"second fork error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                strerr(*__errno_location()),
            );
            if write(
                piped[1 as usize],
                b"fork error\n\0".as_ptr() as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                11 as size_t,
            ) != 11 as ssize_t
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"pipe write error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    strerr(*__errno_location()),
                );
            }
            close(piped[1 as usize]);
            exit(1 as ::core::ffi::c_int);
        }
        if f > 0 as ::core::ffi::c_int {
            exit(0 as ::core::ffi::c_int);
        }
        set_signal_handlers(1 as ::core::ffi::c_int);
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
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1347 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dup(f)==STDIN_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1347 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1349 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dup(f)==STDOUT_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1349 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dup(f)==STDOUT_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        close(STDERR_FILENO);
        if dup(piped[1 as usize]) == 2 as ::core::ffi::c_int {
        } else {
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1351 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dup(piped[1])==STDERR_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1351 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dup(piped[1])==STDERR_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        close(piped[1 as usize]);
        close(f);
        mfs_log_detach_stderr();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn close_msg_channel() {
    unsafe {
        let mut f: ::core::ffi::c_int = 0;
        fflush(stderr);
        f = open(
            b"/dev/null\0".as_ptr() as *const ::core::ffi::c_char,
            O_RDWR,
            0 as ::core::ffi::c_int,
        );
        close(STDERR_FILENO);
        if dup(f) == 2 as ::core::ffi::c_int {
        } else {
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1363 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dup(f)==STDERR_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1363 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dup(f)==STDERR_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        close(f);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn createpath(mut filename: *const ::core::ffi::c_char) {
    unsafe {
        let mut pathbuff: [::core::ffi::c_char; 1024] = [0; 1024];
        let mut src: *const ::core::ffi::c_char = filename;
        let mut dst: *mut ::core::ffi::c_char = &raw mut pathbuff as *mut ::core::ffi::c_char;
        if *src as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
            let c2rust_fresh0 = src;
            src = src.offset(1);
            let c2rust_fresh1 = dst;
            dst = dst.offset(1);
            *c2rust_fresh1 = *c2rust_fresh0;
        }
        while *src != 0 {
            while *src as ::core::ffi::c_int != '/' as ::core::ffi::c_int
                && *src as ::core::ffi::c_int != 0
            {
                let c2rust_fresh2 = src;
                src = src.offset(1);
                let c2rust_fresh3 = dst;
                dst = dst.offset(1);
                *c2rust_fresh3 = *c2rust_fresh2;
            }
            if *src as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
                *dst = '\0' as ::core::ffi::c_char;
                if mkdir(
                    &raw mut pathbuff as *mut ::core::ffi::c_char,
                    0o777 as ::core::ffi::c_int as __mode_t,
                ) < 0 as ::core::ffi::c_int
                {
                    if *__errno_location() != EEXIST {
                        mfs_log(
                            MFSLOG_ERRNO_SYSLOG_STDERR,
                            MFSLOG_INFO,
                            b"creating directory %s\0".as_ptr() as *const ::core::ffi::c_char,
                            &raw mut pathbuff as *mut ::core::ffi::c_char,
                        );
                    }
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_INFO,
                        b"directory %s has been created\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut pathbuff as *mut ::core::ffi::c_char,
                    );
                }
                let c2rust_fresh4 = src;
                src = src.offset(1);
                let c2rust_fresh5 = dst;
                dst = dst.offset(1);
                *c2rust_fresh5 = *c2rust_fresh4;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn usage(mut appname: *const ::core::ffi::c_char) {
    unsafe {
        printf(
            b"usage: %s [-vhfdun] [-t locktimeout] [-c cfgfile] [start|stop|restart|reload|info|test|kill|restore]\n\n-v : print version number and exit\n-h : print this info and exit\n-f : run in foreground\n-d : run with dangerous options (names: DANGEROUS_*)\n-u : log undefined config variables\n-n : do not attempt to increase limit of core dump size\n-t locktimeout : how long wait for lockfile\n-c cfgfile : use given config file\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            appname,
        );
        exit(1 as ::core::ffi::c_int);
    }
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut logappname: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut wrkdir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut cfgfile: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut ocfgfile: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut appname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut ch: ::core::ffi::c_int = 0;
        let mut runmode: uint8_t = 0;
        let mut allowdangerousoptions: uint8_t = 0;
        let mut rundaemon: ::core::ffi::c_int = 0;
        let mut logundefined: ::core::ffi::c_int = 0;
        let mut lockmemory: ::core::ffi::c_int = 0;
        let mut forcecoredump: ::core::ffi::c_int = 0;
        let mut nicelevel: int32_t = 0;
        let mut locktimeout: uint32_t = 0;
        let mut fd: ::core::ffi::c_int = 0;
        let mut movewarning: uint8_t = 0;
        let mut userconfig: uint8_t = 0;
        let mut rls: rlimit = rlimit {
            rlim_cur: 0,
            rlim_max: 0,
        };
        let mut argc_back: ::core::ffi::c_int = 0;
        let mut argv_back: *mut *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        argc_back = argc;
        argv_back = argv;
        strerr_init();
        mycrc32_init();
        movewarning = 0 as uint8_t;
        cfgfile = strdup(
            b"/home/ston/Workspaces/polaris-fs/target/mfs-port/mfs-build/../mfs-install/etc/mfs/mfsmetalogger.cfg\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        if cfgfile.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1441 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"cfgfile\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1441 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"cfgfile\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if cfgfile
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut ::core::ffi::c_char
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1441 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"cfgfile\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1441 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"cfgfile\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        fd = open(cfgfile, O_RDONLY);
        if fd < 0 as ::core::ffi::c_int && *__errno_location() == ENOENT {
            ocfgfile = strdup(
                b"/home/ston/Workspaces/polaris-fs/target/mfs-port/mfs-build/../mfs-install/etc/mfsmetalogger.cfg\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            if ocfgfile.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1444 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ocfgfile\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1444 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ocfgfile\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if ocfgfile
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
                    b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1444 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ocfgfile\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1444 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ocfgfile\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
            fd = open(ocfgfile, O_RDONLY);
            if fd >= 0 as ::core::ffi::c_int {
                free(cfgfile as *mut ::core::ffi::c_void);
                cfgfile = ocfgfile;
                movewarning = 1 as uint8_t;
            } else {
                free(ocfgfile as *mut ::core::ffi::c_void);
            }
        }
        if fd >= 0 as ::core::ffi::c_int {
            close(fd);
        }
        locktimeout = 1800 as uint32_t;
        rundaemon = 1 as ::core::ffi::c_int;
        allowdangerousoptions = 0 as uint8_t;
        runmode = RM_START as uint8_t;
        logundefined = 0 as ::core::ffi::c_int;
        lockmemory = 0 as ::core::ffi::c_int;
        forcecoredump = 1 as ::core::ffi::c_int;
        userconfig = 0 as uint8_t;
        appname = *argv.offset(0 as isize);
        loop {
            ch = getopt(
                argc,
                argv,
                b"nuvfdc:t:h?\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if ch == -1 as ::core::ffi::c_int {
                break;
            }
            match ch {
                118 => {
                    printf(
                        b"version: %s ; build: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                        VERSSTR.as_ptr(),
                        b"2106\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    free(cfgfile as *mut ::core::ffi::c_void);
                    return 0 as ::core::ffi::c_int;
                }
                102 => {
                    rundaemon = 0 as ::core::ffi::c_int;
                }
                100 => {
                    allowdangerousoptions = 1 as uint8_t;
                }
                116 => {
                    locktimeout = strtoul(
                        optarg,
                        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                        10 as ::core::ffi::c_int,
                    ) as uint32_t;
                }
                99 => {
                    free(cfgfile as *mut ::core::ffi::c_void);
                    cfgfile = strdup(optarg);
                    if cfgfile.is_null() {
                        fprintf(
                            stderr,
                            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1484 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"cfgfile\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1484 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"cfgfile\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        abort();
                    } else if cfgfile
                        == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                            -1 as ::core::ffi::c_int as usize,
                        ) as *mut ::core::ffi::c_char
                    {
                        let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1484 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"cfgfile\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring_1,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/main.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1484 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"cfgfile\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring_1,
                        );
                        abort();
                    }
                    movewarning = 0 as uint8_t;
                    userconfig = 1 as uint8_t;
                }
                117 => {
                    logundefined = 1 as ::core::ffi::c_int;
                }
                110 => {
                    forcecoredump = 0 as ::core::ffi::c_int;
                }
                _ => {
                    usage(appname);
                    free(cfgfile as *mut ::core::ffi::c_void);
                    return 1 as ::core::ffi::c_int;
                }
            }
        }
        argc -= optind;
        argv = argv.offset(optind as isize);
        if argc == 1 as ::core::ffi::c_int {
            if strcasecmp(
                *argv.offset(0 as isize),
                b"start\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                runmode = RM_START as uint8_t;
            } else if strcasecmp(
                *argv.offset(0 as isize),
                b"stop\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                runmode = RM_STOP as uint8_t;
            } else if strcasecmp(
                *argv.offset(0 as isize),
                b"restart\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                runmode = RM_RESTART as uint8_t;
            } else if strcasecmp(
                *argv.offset(0 as isize),
                b"try-restart\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                runmode = RM_TRY_RESTART as uint8_t;
            } else if strcasecmp(
                *argv.offset(0 as isize),
                b"reload\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                runmode = RM_RELOAD as uint8_t;
            } else if strcasecmp(
                *argv.offset(0 as isize),
                b"info\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                runmode = RM_INFO as uint8_t;
            } else if strcasecmp(
                *argv.offset(0 as isize),
                b"test\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                || strcasecmp(
                    *argv.offset(0 as isize),
                    b"status\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                runmode = RM_TEST as uint8_t;
            } else if strcasecmp(
                *argv.offset(0 as isize),
                b"kill\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                runmode = RM_KILL as uint8_t;
            } else if strcasecmp(
                *argv.offset(0 as isize),
                b"restore\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                runmode = RM_RESTORE as uint8_t;
            } else {
                usage(appname);
                free(cfgfile as *mut ::core::ffi::c_void);
                return 1 as ::core::ffi::c_int;
            }
        } else if argc != 0 as ::core::ffi::c_int {
            usage(appname);
            free(cfgfile as *mut ::core::ffi::c_void);
            return 1 as ::core::ffi::c_int;
        }
        if movewarning != 0 {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"default sysconf path has changed - please move mfsmetalogger.cfg from /home/ston/Workspaces/polaris-fs/target/mfs-port/mfs-build/../mfs-install/etc/ to /home/ston/Workspaces/polaris-fs/target/mfs-port/mfs-build/../mfs-install/etc/mfs/\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if cfg_load(cfgfile, logundefined) == 0 as ::core::ffi::c_int {
            if userconfig != 0 {
                if rundaemon != 0 {
                    fputc(0 as ::core::ffi::c_int, stderr);
                    close_msg_channel();
                }
                free(cfgfile as *mut ::core::ffi::c_void);
                return 1 as ::core::ffi::c_int;
            }
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"can't load config file: %s - using defaults\0".as_ptr()
                    as *const ::core::ffi::c_char,
                cfgfile,
            );
        }
        if cfg_dangerous_options() != 0
            && allowdangerousoptions as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (runmode as ::core::ffi::c_int == RM_RELOAD
                || runmode as ::core::ffi::c_int == RM_START
                || runmode as ::core::ffi::c_int == RM_RESTART
                || runmode as ::core::ffi::c_int == RM_TRY_RESTART)
        {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"Dangerous option(s) detected in config file: %s - use '-d' option to force %s\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                cfgfile,
                if runmode as ::core::ffi::c_int == RM_RELOAD {
                    b"reload\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"start\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
            free(cfgfile as *mut ::core::ffi::c_void);
            return 1 as ::core::ffi::c_int;
        }
        free(cfgfile as *mut ::core::ffi::c_void);
        if runmode as ::core::ffi::c_int == RM_START
            || runmode as ::core::ffi::c_int == RM_RESTART
            || runmode as ::core::ffi::c_int == RM_TRY_RESTART
        {
            if rundaemon != 0 {
                makedaemon();
            } else {
                set_signal_handlers(0 as ::core::ffi::c_int);
            }
        }
        processname_init(argc_back, argv_back as *mut *mut ::core::ffi::c_char);
        logappname = cfg_getstr(
            b"SYSLOG_IDENT\0".as_ptr() as *const ::core::ffi::c_char,
            b"mfsmetalogger\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if *logappname.offset(0 as isize) != 0 {
            mfs_log_init(logappname, rundaemon);
        } else {
            mfs_log_init(
                b"mfsmetalogger\0".as_ptr() as *const ::core::ffi::c_char,
                rundaemon,
            );
        }
        main_reload();
        if runmode as ::core::ffi::c_int == RM_START
            || runmode as ::core::ffi::c_int == RM_RESTART
            || runmode as ::core::ffi::c_int == RM_TRY_RESTART
        {
            rls.rlim_cur = MFSMAXFILES as rlim_t;
            rls.rlim_max = MFSMAXFILES as rlim_t;
            if setrlimit(RLIMIT_NOFILE, &raw mut rls) < 0 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_NOTICE,
                    b"can't change open files limit to: %u (trying to set smaller value)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    MFSMAXFILES as ::core::ffi::c_uint,
                );
                if getrlimit(RLIMIT_NOFILE, &raw mut rls) >= 0 as ::core::ffi::c_int {
                    let mut limit: uint32_t = 0;
                    if rls.rlim_max > MFSMAXFILES as rlim_t {
                        limit = MFSMAXFILES as uint32_t;
                    } else {
                        limit = rls.rlim_max as uint32_t;
                    }
                    while limit > 1024 as uint32_t {
                        rls.rlim_cur = limit as rlim_t;
                        if setrlimit(RLIMIT_NOFILE, &raw mut rls) >= 0 as ::core::ffi::c_int {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_INFO,
                                b"open files limit has been set to: %u\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                limit,
                            );
                            break;
                        } else {
                            limit = limit.wrapping_mul(3 as uint32_t);
                            limit = limit.wrapping_div(4 as uint32_t);
                        }
                    }
                }
            } else {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_INFO,
                    b"open files limit has been set to: %u\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    MFSMAXFILES as ::core::ffi::c_uint,
                );
            }
            lockmemory = cfg_getnum(
                b"LOCK_MEMORY\0".as_ptr() as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            if lockmemory != 0 {
                rls.rlim_cur = RLIM_INFINITY as rlim_t;
                rls.rlim_max = RLIM_INFINITY as rlim_t;
                setrlimit(__RLIMIT_MEMLOCK, &raw mut rls);
            }
            nicelevel = cfg_getint32(
                b"NICE_LEVEL\0".as_ptr() as *const ::core::ffi::c_char,
                -19 as int32_t,
            );
            setpriority(
                PRIO_PROCESS,
                getpid() as id_t,
                nicelevel as ::core::ffi::c_int,
            );
            if cfg_getuint8(
                b"DISABLE_OOM_KILLER\0".as_ptr() as *const ::core::ffi::c_char,
                1 as uint8_t,
            ) as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                let mut oomfd: *mut FILE = ::core::ptr::null_mut::<FILE>();
                let mut oomdis: ::core::ffi::c_int = 0;
                oomdis = 0 as ::core::ffi::c_int;
                oomfd = fopen(
                    b"/proc/self/oom_score_adj\0".as_ptr() as *const ::core::ffi::c_char,
                    b"w\0".as_ptr() as *const ::core::ffi::c_char,
                ) as *mut FILE;
                if !oomfd.is_null() {
                    fprintf(
                        oomfd,
                        b"%d\n\0".as_ptr() as *const ::core::ffi::c_char,
                        OOM_SCORE_ADJ_MIN,
                    );
                    if fclose(oomfd) >= 0 as ::core::ffi::c_int {
                        oomdis = 1 as ::core::ffi::c_int;
                    }
                } else {
                    oomfd = fopen(
                        b"/proc/self/oom_adj\0".as_ptr() as *const ::core::ffi::c_char,
                        b"w\0".as_ptr() as *const ::core::ffi::c_char,
                    ) as *mut FILE;
                    if !oomfd.is_null() {
                        fprintf(
                            oomfd,
                            b"%d\n\0".as_ptr() as *const ::core::ffi::c_char,
                            OOM_DISABLE,
                        );
                        if fclose(oomfd) >= 0 as ::core::ffi::c_int {
                            oomdis = 1 as ::core::ffi::c_int;
                        }
                    }
                }
                if oomdis != 0 {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_INFO,
                        b"out of memory killer disabled\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"can't disable out of memory killer\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            }
        }
        changeugid();
        wrkdir = cfg_getstr(
            b"DATA_PATH\0".as_ptr() as *const ::core::ffi::c_char,
            DATA_PATH.as_ptr(),
        );
        if runmode as ::core::ffi::c_int == RM_START
            || runmode as ::core::ffi::c_int == RM_RESTART
            || runmode as ::core::ffi::c_int == RM_TRY_RESTART
        {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_INFO,
                b"working directory: %s\0".as_ptr() as *const ::core::ffi::c_char,
                wrkdir,
            );
        }
        if chdir(wrkdir) < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"can't set working directory to %s\0".as_ptr() as *const ::core::ffi::c_char,
                wrkdir,
            );
            if rundaemon != 0 {
                fputc(0 as ::core::ffi::c_int, stderr);
                close_msg_channel();
            }
            mfs_log_term();
            free(logappname as *mut ::core::ffi::c_void);
            return 1 as ::core::ffi::c_int;
        }
        free(wrkdir as *mut ::core::ffi::c_void);
        umask(
            cfg_getuint32(
                b"FILE_UMASK\0".as_ptr() as *const ::core::ffi::c_char,
                0o27 as uint32_t,
            ) as __mode_t
                & 0o77 as __mode_t,
        );
        ch = wdlock(runmode, locktimeout) as ::core::ffi::c_int;
        if ch != 0 {
            if rundaemon != 0 {
                fputc(0 as ::core::ffi::c_int, stderr);
                close_msg_channel();
            }
            signal_cleanup();
            cfg_term();
            strerr_term();
            wdunlock();
            mfs_log_term();
            free(logappname as *mut ::core::ffi::c_void);
            return ch;
        }
        ch = 0 as ::core::ffi::c_int;
        if runmode as ::core::ffi::c_int == RM_RESTORE {
            if restore() == 0 as ::core::ffi::c_int {
                ch = 1 as ::core::ffi::c_int;
            }
        }
        if runmode as ::core::ffi::c_int == RM_STOP
            || runmode as ::core::ffi::c_int == RM_KILL
            || runmode as ::core::ffi::c_int == RM_RELOAD
            || runmode as ::core::ffi::c_int == RM_INFO
            || runmode as ::core::ffi::c_int == RM_TEST
            || runmode as ::core::ffi::c_int == RM_RESTORE
        {
            if rundaemon != 0 {
                close_msg_channel();
            }
            signal_cleanup();
            cfg_term();
            strerr_term();
            wdunlock();
            mfs_log_term();
            free(logappname as *mut ::core::ffi::c_void);
            return ch;
        }
        if lockmemory != 0 {
            if getrlimit(__RLIMIT_MEMLOCK, &raw mut rls) < 0 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"error getting memory lock limits\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                if rls.rlim_cur as ::core::ffi::c_ulonglong != RLIM_INFINITY
                    && rls.rlim_max as ::core::ffi::c_ulonglong == RLIM_INFINITY
                {
                    rls.rlim_cur = RLIM_INFINITY as rlim_t;
                    rls.rlim_max = RLIM_INFINITY as rlim_t;
                    if setrlimit(__RLIMIT_MEMLOCK, &raw mut rls) < 0 as ::core::ffi::c_int {
                        mfs_log(
                            MFSLOG_ERRNO_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"error setting memory lock limit to unlimited\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                }
                if getrlimit(__RLIMIT_MEMLOCK, &raw mut rls) < 0 as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"error getting memory lock limits\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                } else if rls.rlim_cur as ::core::ffi::c_ulonglong != RLIM_INFINITY {
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"can't set memory lock limit to unlimited\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                } else if mlockall(MCL_CURRENT | MCL_FUTURE) < 0 as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"memory lock error\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_INFO,
                        b"process memory was successfully locked in RAM\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            }
        }
        if forcecoredump != 0 {
            rls.rlim_cur = RLIM_INFINITY as rlim_t;
            rls.rlim_max = RLIM_INFINITY as rlim_t;
            setrlimit(RLIMIT_CORE, &raw mut rls);
            prctl(PR_SET_DUMPABLE, 1 as ::core::ffi::c_int);
        }
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_INFO,
            b"monotonic clock function: %s\0".as_ptr() as *const ::core::ffi::c_char,
            monotonic_method(),
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_INFO,
            b"monotonic clock speed: %u ops / 10 mili seconds\0".as_ptr()
                as *const ::core::ffi::c_char,
            monotonic_speed(),
        );
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_INFO,
            b"initializing %s modules ...\0".as_ptr() as *const ::core::ffi::c_char,
            logappname,
        );
        if initialize() != 0 {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_INFO,
                b"%s daemon initialized properly\0".as_ptr() as *const ::core::ffi::c_char,
                logappname,
            );
            if rundaemon != 0 {
                close_msg_channel();
            }
            if initialize_late() != 0 {
                ch = mainloop();
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_INFO,
                    b"exited from main loop\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                ch = 1 as ::core::ffi::c_int;
            }
        } else {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"error occurred during initialization - exiting\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            if rundaemon != 0 {
                fputc(0 as ::core::ffi::c_int, stderr);
                close_msg_channel();
            }
            ch = 1 as ::core::ffi::c_int;
        }
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_INFO,
            b"exititng ...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        destruct();
        free_all_registered_entries();
        signal_cleanup();
        cfg_term();
        strerr_term();
        wdunlock();
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_INFO,
            b"process exited successfully (status:%d)\0".as_ptr() as *const ::core::ffi::c_char,
            ch,
        );
        mfs_log_term();
        free(logappname as *mut ::core::ffi::c_void);
        return ch;
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
