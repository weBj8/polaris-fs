use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn pthread_detach(__th: pthread_t) -> ::core::ffi::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn main_destruct_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    fn main_canexit_register_fname(
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
    fn main_poll_register_fname(
        desc: Option<unsafe extern "C" fn(*mut pollfd, *mut uint32_t) -> ()>,
        serve: Option<unsafe extern "C" fn(*mut pollfd) -> ()>,
        dname: *const ::core::ffi::c_char,
        sname: *const ::core::ffi::c_char,
    );
    fn main_eachloop_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    fn main_time_register_fname(
        seconds: uint32_t,
        offset: uint32_t,
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    fn cfg_isdefined(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn cfg_getuint32(name: *const ::core::ffi::c_char, def: uint32_t) -> uint32_t;
    fn queue_new(size: uint32_t) -> *mut ::core::ffi::c_void;
    fn queue_delete(que: *mut ::core::ffi::c_void);
    fn queue_close(que: *mut ::core::ffi::c_void);
    fn queue_isempty(que: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn queue_elements(que: *mut ::core::ffi::c_void) -> uint32_t;
    fn queue_sizeleft(que: *mut ::core::ffi::c_void) -> uint32_t;
    fn queue_put(
        que: *mut ::core::ffi::c_void,
        id: uint32_t,
        op: uint32_t,
        data: *mut uint8_t,
        leng: uint32_t,
    );
    fn queue_get(
        que: *mut ::core::ffi::c_void,
        id: *mut uint32_t,
        op: *mut uint32_t,
        data: *mut *mut uint8_t,
        leng: *mut uint32_t,
    );
    fn ionice_medium();
    fn lwt_minthread_create(
        th: *mut pthread_t,
        detached: uint8_t,
        r#fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
        arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn monotonic_useconds() -> uint64_t;
    fn mainserv_read(sock: ::core::ffi::c_int, packet: *const uint8_t, length: uint32_t)
        -> uint8_t;
    fn mainserv_write(
        sock: ::core::ffi::c_int,
        packet: *const uint8_t,
        length: uint32_t,
    ) -> uint8_t;
    fn hdd_get_chunk_info(
        chunkid: uint64_t,
        version: uint32_t,
        requested_info: uint8_t,
        info_buff: *mut uint8_t,
    ) -> ::core::ffi::c_int;
    fn hdd_move(
        fsrcv: *mut ::core::ffi::c_void,
        fdstv: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn hdd_chunkop(
        chunkid: uint64_t,
        version: uint32_t,
        newversion: uint32_t,
        copychunkid: uint64_t,
        copyversion: uint32_t,
        length: uint32_t,
    ) -> ::core::ffi::c_int;
    fn replicate(
        rmode: repmodeenum,
        chunkid: uint64_t,
        version: uint32_t,
        partno: uint8_t,
        parts: uint8_t,
        srcip: *const uint32_t,
        srcport: *const uint16_t,
        srcchunkid: *const uint64_t,
    ) -> uint8_t;
    fn masterconn_reportload();
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type ssize_t = isize;
pub type int32_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: ::core::ffi::c_ulonglong,
    pub __value32: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub __low: ::core::ffi::c_uint,
    pub __high: ::core::ffi::c_uint,
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
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_size: [::core::ffi::c_uint; 2],
    pub __g1_orig_size: ::core::ffi::c_uint,
    pub __wrefs: ::core::ffi::c_uint,
    pub __g_signals: [::core::ffi::c_uint; 2],
    pub __unused_initialized_1: ::core::ffi::c_uint,
    pub __unused_initialized_2: ::core::ffi::c_uint,
}
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::core::ffi::c_char; 48],
    pub __align: ::core::ffi::c_longlong,
}
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
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
pub type repmodeenum = ::core::ffi::c_uint;
pub const JOIN: repmodeenum = 3;
pub const RECOVER: repmodeenum = 2;
pub const SPLIT: repmodeenum = 1;
pub const SIMPLE: repmodeenum = 0;
pub type C2Rust_Unnamed_0 = ::core::ffi::c_uint;
pub const JSTATE_INPROGRESS: C2Rust_Unnamed_0 = 2;
pub const JSTATE_ENABLED: C2Rust_Unnamed_0 = 1;
pub const JSTATE_DISABLED: C2Rust_Unnamed_0 = 0;
pub type C2Rust_Unnamed_1 = ::core::ffi::c_uint;
pub const OP_CHUNKMOVE: C2Rust_Unnamed_1 = 10;
pub const OP_GETINFO: C2Rust_Unnamed_1 = 9;
pub const OP_REPLICATE_JOIN: C2Rust_Unnamed_1 = 8;
pub const OP_REPLICATE_RECOVER: C2Rust_Unnamed_1 = 7;
pub const OP_REPLICATE_SPLIT: C2Rust_Unnamed_1 = 6;
pub const OP_REPLICATE_SIMPLE: C2Rust_Unnamed_1 = 5;
pub const OP_SERV_WRITE: C2Rust_Unnamed_1 = 4;
pub const OP_SERV_READ: C2Rust_Unnamed_1 = 3;
pub const OP_CHUNKOP: C2Rust_Unnamed_1 = 2;
pub const OP_INVAL: C2Rust_Unnamed_1 = 1;
pub const OP_EXIT: C2Rust_Unnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _chunk_op_args {
    pub chunkid: uint64_t,
    pub copychunkid: uint64_t,
    pub version: uint32_t,
    pub newversion: uint32_t,
    pub copyversion: uint32_t,
    pub length: uint32_t,
}
pub type chunk_op_args = _chunk_op_args;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _chunk_rw_args {
    pub sock: ::core::ffi::c_int,
    pub packet: *const uint8_t,
    pub length: uint32_t,
}
pub type chunk_rw_args = _chunk_rw_args;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _chunk_rp_args {
    pub chunkid: uint64_t,
    pub version: uint32_t,
    pub partno: uint8_t,
    pub parts: uint8_t,
    pub srcip: [uint32_t; 8],
    pub srcport: [uint16_t; 8],
    pub srcchunkid: [uint64_t; 8],
}
pub type chunk_rp_args = _chunk_rp_args;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _chunk_ij_args {
    pub chunkid: uint64_t,
    pub version: uint32_t,
    pub requested_info: uint8_t,
    pub pointer: *mut ::core::ffi::c_void,
}
pub type chunk_ij_args = _chunk_ij_args;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _chunk_mv_args {
    pub fsrc: *mut ::core::ffi::c_void,
    pub fdst: *mut ::core::ffi::c_void,
}
pub type chunk_mv_args = _chunk_mv_args;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _job {
    pub jobid: uint32_t,
    pub callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
    pub extra: *mut ::core::ffi::c_void,
    pub args: *mut ::core::ffi::c_void,
    pub jstate: uint8_t,
    pub tasktype: uint8_t,
    pub starttime: uint64_t,
    pub chunkid: uint64_t,
    pub next: *mut _job,
}
pub type job = _job;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _jobpool {
    pub rpipe: ::core::ffi::c_int,
    pub wpipe: ::core::ffi::c_int,
    pub fdpdescpos: int32_t,
    pub workers_max: uint32_t,
    pub workers_himark: uint32_t,
    pub workers_lomark: uint32_t,
    pub workers_max_idle: uint32_t,
    pub workers_avail: uint32_t,
    pub workers_total: uint32_t,
    pub workers_term_waiting: uint32_t,
    pub worker_term_cond: pthread_cond_t,
    pub pipelock: pthread_mutex_t,
    pub jobslock: pthread_mutex_t,
    pub worker_fn:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
    pub jobqueue: *mut ::core::ffi::c_void,
    pub statusqueue: *mut ::core::ffi::c_void,
    pub jobhash: [*mut job; 1024],
    pub jobs_time_max_glob: [uint64_t; 6],
    pub jobs_time_max_prev: [uint64_t; 6],
    pub jobs_time_prev: [uint64_t; 6],
    pub jobs_count_prev: [uint32_t; 6],
    pub jobs_time_max: [uint64_t; 6],
    pub jobs_time: [uint64_t; 6],
    pub jobs_count: [uint32_t; 6],
    pub nextjobid: uint32_t,
}
pub type jobpool = _jobpool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _worker {
    pub thread_id: pthread_t,
    pub jp: *mut jobpool,
}
pub type worker = _worker;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MFS_ERROR_EINVAL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MFS_ERROR_NOTDONE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const HLSTATUS_DEFAULT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const HLSTATUS_OK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const HLSTATUS_OVERLOADED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const JHASHSIZE: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const TASK_READ: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TASK_WRITE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TASK_REPLICATE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TASK_CHUNKOP: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const TASK_INFO: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const TASK_MOVE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const TASK_COUNT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
static mut jobtype_str: [*mut ::core::ffi::c_char; 6] = [
    b"read\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"write\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"replicate\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"chunk operation\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"chunk info\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    b"chunk move\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
];
static mut hp_pool: *mut jobpool = ::core::ptr::null_mut::<jobpool>();
static mut lp_pool: *mut jobpool = ::core::ptr::null_mut::<jobpool>();
static mut stats_maxjobscnt: uint32_t = 0 as uint32_t;
#[no_mangle]
pub unsafe extern "C" fn job_stats(mut maxjobscnt: *mut uint32_t) {
    *maxjobscnt = stats_maxjobscnt;
    stats_maxjobscnt = 0 as uint32_t;
}
#[inline]
unsafe extern "C" fn job_send_status(
    mut jp: *mut jobpool,
    mut jobid: uint32_t,
    mut status: uint8_t,
) {
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*jp).pipelock);
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                217 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                217 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                217 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                217 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
        } else {
            let mut _mfs_errorstring_err: *const ::core::ffi::c_char = strerr(*__errno_location());
            let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                217 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                217 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    if queue_isempty((*jp).statusqueue) != 0 {
        if !(write(
            (*jp).wpipe,
            &raw mut status as *const ::core::ffi::c_void,
            1 as size_t,
        ) == 1 as ssize_t)
        {
            let mut _mfs_errorstring_1: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s', error: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                219 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"write(jp->wpipe,&status,1)==1\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s', error: %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                219 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"write(jp->wpipe,&status,1)==1\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_1,
            );
            abort();
        }
    }
    queue_put(
        (*jp).statusqueue,
        jobid,
        status as uint32_t,
        ::core::ptr::null_mut::<uint8_t>(),
        1 as uint32_t,
    );
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*jp).pipelock);
    if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_2: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                222 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                222 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_2,
            );
        } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
            && *__errno_location() == 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_3: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                222 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                222 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_3,
            );
        } else {
            let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                222 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                222 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
}
#[inline]
unsafe extern "C" fn job_receive_status(
    mut jp: *mut jobpool,
    mut jobid: *mut uint32_t,
    mut status: *mut uint8_t,
) -> ::core::ffi::c_int {
    let mut qstatus: uint32_t = 0;
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*jp).pipelock);
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                228 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                228 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                228 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                228 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
        } else {
            let mut _mfs_errorstring_err: *const ::core::ffi::c_char = strerr(*__errno_location());
            let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                228 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                228 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    queue_get(
        (*jp).statusqueue,
        jobid,
        &raw mut qstatus,
        ::core::ptr::null_mut::<*mut uint8_t>(),
        ::core::ptr::null_mut::<uint32_t>(),
    );
    *status = qstatus as uint8_t;
    if queue_isempty((*jp).statusqueue) != 0 {
        if !(read(
            (*jp).rpipe,
            &raw mut qstatus as *mut ::core::ffi::c_void,
            1 as size_t,
        ) == 1 as ssize_t)
        {
            let mut _mfs_errorstring_1: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s', error: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                232 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"read(jp->rpipe,&qstatus,1)==1\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s', error: %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                232 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"read(jp->rpipe,&qstatus,1)==1\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_1,
            );
            abort();
        }
        let mut _mfs_assert_ret_0: ::core::ffi::c_int =
            pthread_mutex_unlock(&raw mut (*jp).pipelock);
        if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    233 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->pipelock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    233 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->pipelock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_2,
                );
            } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_3: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    233 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->pipelock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    233 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->pipelock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_3,
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    233 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->pipelock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    233 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->pipelock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
        return 0 as ::core::ffi::c_int;
    }
    let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*jp).pipelock);
    if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_4: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                236 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                236 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_4,
            );
        } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
            && *__errno_location() == 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_5: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                236 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_5,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                236 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_5,
            );
        } else {
            let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                236 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                236 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
        }
        abort();
    }
    return 1 as ::core::ffi::c_int;
}
static mut lastnotify: uint32_t = 0 as uint32_t;
#[inline]
unsafe extern "C" fn job_spawn_worker(mut jp: *mut jobpool) {
    let mut w: *mut worker = ::core::ptr::null_mut::<worker>();
    w = malloc(::core::mem::size_of::<worker>()) as *mut worker;
    if w.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            248 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"w\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            248 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"w\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if w
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut worker
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            248 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"w\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            248 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"w\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    (*w).jp = jp;
    if lwt_minthread_create(
        &raw mut (*w).thread_id,
        0 as uint8_t,
        (*jp).worker_fn,
        w as *mut ::core::ffi::c_void,
    ) < 0 as ::core::ffi::c_int
    {
        return;
    }
    (*jp).workers_avail = (*jp).workers_avail.wrapping_add(1);
    (*jp).workers_total = (*jp).workers_total.wrapping_add(1);
    if (*jp).workers_total.wrapping_rem(10 as uint32_t) == 0 as uint32_t
        && lastnotify != (*jp).workers_total
    {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_INFO,
            b"workers: %u+\0".as_ptr() as *const ::core::ffi::c_char,
            (*jp).workers_total,
        );
        lastnotify = (*jp).workers_total;
    }
}
#[inline]
unsafe extern "C" fn job_close_worker(mut w: *mut worker) {
    let mut jp: *mut jobpool = (*w).jp;
    (*jp).workers_avail = (*jp).workers_avail.wrapping_sub(1);
    (*jp).workers_total = (*jp).workers_total.wrapping_sub(1);
    if (*jp).workers_total == 0 as uint32_t && (*jp).workers_term_waiting != 0 {
        let mut _mfs_assert_ret: ::core::ffi::c_int =
            pthread_cond_signal(&raw mut (*jp).worker_term_cond);
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    267 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_signal(&(jp->worker_term_cond))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    267 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_signal(&(jp->worker_term_cond))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    267 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_signal(&(jp->worker_term_cond))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    267 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_signal(&(jp->worker_term_cond))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    267 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_signal(&(jp->worker_term_cond))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    267 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_signal(&(jp->worker_term_cond))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        (*jp).workers_term_waiting = (*jp).workers_term_waiting.wrapping_sub(1);
    }
    pthread_detach((*w).thread_id);
    free(w as *mut ::core::ffi::c_void);
    if (*jp).workers_total.wrapping_rem(10 as uint32_t) == 0 as uint32_t
        && lastnotify != (*jp).workers_total
    {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_INFO,
            b"workers: %u-\0".as_ptr() as *const ::core::ffi::c_char,
            (*jp).workers_total,
        );
        lastnotify = (*jp).workers_total;
    }
}
#[no_mangle]
pub unsafe extern "C" fn job_worker(mut arg: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    let mut w: *mut worker = arg as *mut worker;
    let mut jp: *mut jobpool = (*w).jp;
    let mut jptr: *mut job = ::core::ptr::null_mut::<job>();
    let mut jptrarg: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut status: uint8_t = 0;
    let mut jstate: uint8_t = 0;
    let mut jobid: uint32_t = 0;
    let mut op: uint32_t = 0;
    loop {
        queue_get(
            (*jp).jobqueue,
            &raw mut jobid,
            &raw mut op,
            &raw mut jptrarg,
            ::core::ptr::null_mut::<uint32_t>(),
        );
        jptr = jptrarg as *mut job;
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*jp).jobslock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    301 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    301 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    301 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    301 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    301 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    301 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        if jobid == 0 as uint32_t && op == 0 as uint32_t && jptrarg.is_null() {
            job_close_worker(w);
            let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                pthread_mutex_unlock(&raw mut (*jp).jobslock);
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
                        b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        304 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        304 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_0);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        304 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        304 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
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
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        304 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        304 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                }
                abort();
            }
            return NULL;
        }
        (*jp).workers_avail = (*jp).workers_avail.wrapping_sub(1);
        if (*jp).workers_avail == 0 as uint32_t && (*jp).workers_total < (*jp).workers_max {
            job_spawn_worker(jp);
        }
        if !jptr.is_null() {
            jstate = (*jptr).jstate;
            if (*jptr).jstate as ::core::ffi::c_int == JSTATE_ENABLED as ::core::ffi::c_int {
                (*jptr).jstate = JSTATE_INPROGRESS as ::core::ffi::c_int as uint8_t;
                (*jptr).starttime = monotonic_useconds();
            }
        } else {
            jstate = JSTATE_DISABLED as ::core::ffi::c_int as uint8_t;
        }
        let mut _mfs_assert_ret_1: ::core::ffi::c_int =
            pthread_mutex_unlock(&raw mut (*jp).jobslock);
        if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    320 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    320 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
            } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_4: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    320 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    320 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
            } else {
                let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_1);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    320 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    320 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
            }
            abort();
        }
        match op {
            1 => {
                status = MFS_ERROR_EINVAL as uint8_t;
            }
            2 => {
                if jstate as ::core::ffi::c_int == JSTATE_DISABLED as ::core::ffi::c_int {
                    status = MFS_ERROR_NOTDONE as uint8_t;
                } else {
                    status = hdd_chunkop(
                        (*((*jptr).args as *mut chunk_op_args)).chunkid,
                        (*((*jptr).args as *mut chunk_op_args)).version,
                        (*((*jptr).args as *mut chunk_op_args)).newversion,
                        (*((*jptr).args as *mut chunk_op_args)).copychunkid,
                        (*((*jptr).args as *mut chunk_op_args)).copyversion,
                        (*((*jptr).args as *mut chunk_op_args)).length,
                    ) as uint8_t;
                }
            }
            3 => {
                if jstate as ::core::ffi::c_int == JSTATE_DISABLED as ::core::ffi::c_int {
                    status = 0 as uint8_t;
                } else {
                    status = mainserv_read(
                        (*((*jptr).args as *mut chunk_rw_args)).sock,
                        (*((*jptr).args as *mut chunk_rw_args)).packet,
                        (*((*jptr).args as *mut chunk_rw_args)).length,
                    );
                }
            }
            4 => {
                if jstate as ::core::ffi::c_int == JSTATE_DISABLED as ::core::ffi::c_int {
                    status = 0 as uint8_t;
                } else {
                    status = mainserv_write(
                        (*((*jptr).args as *mut chunk_rw_args)).sock,
                        (*((*jptr).args as *mut chunk_rw_args)).packet,
                        (*((*jptr).args as *mut chunk_rw_args)).length,
                    );
                }
            }
            5 => {
                if jstate as ::core::ffi::c_int == JSTATE_DISABLED as ::core::ffi::c_int {
                    status = MFS_ERROR_NOTDONE as uint8_t;
                } else {
                    status = replicate(
                        SIMPLE,
                        (*((*jptr).args as *mut chunk_rp_args)).chunkid,
                        (*((*jptr).args as *mut chunk_rp_args)).version,
                        (*((*jptr).args as *mut chunk_rp_args)).partno,
                        (*((*jptr).args as *mut chunk_rp_args)).parts,
                        &raw mut (*((*jptr).args as *mut chunk_rp_args)).srcip as *mut uint32_t
                            as *const uint32_t,
                        &raw mut (*((*jptr).args as *mut chunk_rp_args)).srcport as *mut uint16_t
                            as *const uint16_t,
                        &raw mut (*((*jptr).args as *mut chunk_rp_args)).srcchunkid as *mut uint64_t
                            as *const uint64_t,
                    );
                }
            }
            6 => {
                if jstate as ::core::ffi::c_int == JSTATE_DISABLED as ::core::ffi::c_int {
                    status = MFS_ERROR_NOTDONE as uint8_t;
                } else {
                    status = replicate(
                        SPLIT,
                        (*((*jptr).args as *mut chunk_rp_args)).chunkid,
                        (*((*jptr).args as *mut chunk_rp_args)).version,
                        (*((*jptr).args as *mut chunk_rp_args)).partno,
                        (*((*jptr).args as *mut chunk_rp_args)).parts,
                        &raw mut (*((*jptr).args as *mut chunk_rp_args)).srcip as *mut uint32_t
                            as *const uint32_t,
                        &raw mut (*((*jptr).args as *mut chunk_rp_args)).srcport as *mut uint16_t
                            as *const uint16_t,
                        &raw mut (*((*jptr).args as *mut chunk_rp_args)).srcchunkid as *mut uint64_t
                            as *const uint64_t,
                    );
                }
            }
            7 => {
                if jstate as ::core::ffi::c_int == JSTATE_DISABLED as ::core::ffi::c_int {
                    status = MFS_ERROR_NOTDONE as uint8_t;
                } else {
                    status = replicate(
                        RECOVER,
                        (*((*jptr).args as *mut chunk_rp_args)).chunkid,
                        (*((*jptr).args as *mut chunk_rp_args)).version,
                        (*((*jptr).args as *mut chunk_rp_args)).partno,
                        (*((*jptr).args as *mut chunk_rp_args)).parts,
                        &raw mut (*((*jptr).args as *mut chunk_rp_args)).srcip as *mut uint32_t
                            as *const uint32_t,
                        &raw mut (*((*jptr).args as *mut chunk_rp_args)).srcport as *mut uint16_t
                            as *const uint16_t,
                        &raw mut (*((*jptr).args as *mut chunk_rp_args)).srcchunkid as *mut uint64_t
                            as *const uint64_t,
                    );
                }
            }
            8 => {
                if jstate as ::core::ffi::c_int == JSTATE_DISABLED as ::core::ffi::c_int {
                    status = MFS_ERROR_NOTDONE as uint8_t;
                } else {
                    status = replicate(
                        JOIN,
                        (*((*jptr).args as *mut chunk_rp_args)).chunkid,
                        (*((*jptr).args as *mut chunk_rp_args)).version,
                        (*((*jptr).args as *mut chunk_rp_args)).partno,
                        (*((*jptr).args as *mut chunk_rp_args)).parts,
                        &raw mut (*((*jptr).args as *mut chunk_rp_args)).srcip as *mut uint32_t
                            as *const uint32_t,
                        &raw mut (*((*jptr).args as *mut chunk_rp_args)).srcport as *mut uint16_t
                            as *const uint16_t,
                        &raw mut (*((*jptr).args as *mut chunk_rp_args)).srcchunkid as *mut uint64_t
                            as *const uint64_t,
                    );
                }
            }
            9 => {
                if jstate as ::core::ffi::c_int == JSTATE_DISABLED as ::core::ffi::c_int {
                    status = MFS_ERROR_NOTDONE as uint8_t;
                } else {
                    status = hdd_get_chunk_info(
                        (*((*jptr).args as *mut chunk_ij_args)).chunkid,
                        (*((*jptr).args as *mut chunk_ij_args)).version,
                        (*((*jptr).args as *mut chunk_ij_args)).requested_info,
                        (*((*jptr).args as *mut chunk_ij_args)).pointer as *mut uint8_t,
                    ) as uint8_t;
                }
            }
            10 => {
                if jstate as ::core::ffi::c_int == JSTATE_DISABLED as ::core::ffi::c_int {
                    status = MFS_ERROR_NOTDONE as uint8_t;
                } else {
                    status = hdd_move(
                        (*((*jptr).args as *mut chunk_mv_args)).fsrc,
                        (*((*jptr).args as *mut chunk_mv_args)).fdst,
                    ) as uint8_t;
                }
            }
            _ => {
                let mut _mfs_assert_ret_2: ::core::ffi::c_int =
                    pthread_mutex_lock(&raw mut (*jp).jobslock);
                if _mfs_assert_ret_2 != 0 as ::core::ffi::c_int {
                    if _mfs_assert_ret_2 < 0 as ::core::ffi::c_int
                        && *__errno_location() != 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_5: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            430 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_2,
                            *__errno_location(),
                            _mfs_errorstring_5,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            430 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_2,
                            *__errno_location(),
                            _mfs_errorstring_5,
                        );
                    } else if _mfs_assert_ret_2 > 0 as ::core::ffi::c_int
                        && *__errno_location() == 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_6: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_2);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            430 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_2,
                            _mfs_errorstring_6,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            430 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_2,
                            _mfs_errorstring_6,
                        );
                    } else {
                        let mut _mfs_errorstring_err_2: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        let mut _mfs_errorstring_ret_2: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_2);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            430 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_2,
                            _mfs_errorstring_ret_2,
                            *__errno_location(),
                            _mfs_errorstring_err_2,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            430 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_2,
                            _mfs_errorstring_ret_2,
                            *__errno_location(),
                            _mfs_errorstring_err_2,
                        );
                    }
                    abort();
                }
                job_close_worker(w);
                let mut _mfs_assert_ret_3: ::core::ffi::c_int =
                    pthread_mutex_unlock(&raw mut (*jp).jobslock);
                if _mfs_assert_ret_3 != 0 as ::core::ffi::c_int {
                    if _mfs_assert_ret_3 < 0 as ::core::ffi::c_int
                        && *__errno_location() != 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_7: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            432 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_3,
                            *__errno_location(),
                            _mfs_errorstring_7,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            432 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_3,
                            *__errno_location(),
                            _mfs_errorstring_7,
                        );
                    } else if _mfs_assert_ret_3 > 0 as ::core::ffi::c_int
                        && *__errno_location() == 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_8: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_3);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            432 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_3,
                            _mfs_errorstring_8,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            432 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_3,
                            _mfs_errorstring_8,
                        );
                    } else {
                        let mut _mfs_errorstring_err_3: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        let mut _mfs_errorstring_ret_3: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_3);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            432 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_3,
                            _mfs_errorstring_ret_3,
                            *__errno_location(),
                            _mfs_errorstring_err_3,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            432 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_3,
                            _mfs_errorstring_ret_3,
                            *__errno_location(),
                            _mfs_errorstring_err_3,
                        );
                    }
                    abort();
                }
                return NULL;
            }
        }
        let mut _mfs_assert_ret_4: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*jp).jobslock);
        if _mfs_assert_ret_4 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_4 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_9: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    435 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_9,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    435 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_9,
                );
            } else if _mfs_assert_ret_4 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_10: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_4);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    435 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_10,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    435 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_10,
                );
            } else {
                let mut _mfs_errorstring_err_4: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_4: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_4);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    435 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_err_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    435 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_err_4,
                );
            }
            abort();
        }
        if !jptr.is_null() {
            let mut tasktime: uint64_t = 0;
            tasktime = monotonic_useconds().wrapping_sub((*jptr).starttime);
            (*jp).jobs_count
                [((*jptr).tasktype as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int) as usize] =
                (*jp).jobs_count
                    [((*jptr).tasktype as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int) as usize]
                    .wrapping_add(1);
            (*jp).jobs_time
                [((*jptr).tasktype as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int) as usize] =
                (*jp).jobs_time
                    [((*jptr).tasktype as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int) as usize]
                    .wrapping_add(tasktime);
            if tasktime
                > (*jp).jobs_time_max
                    [((*jptr).tasktype as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int) as usize]
            {
                (*jp).jobs_time_max[((*jptr).tasktype as ::core::ffi::c_int
                    & 0x7 as ::core::ffi::c_int) as usize] = tasktime;
            }
            if tasktime
                > (*jp).jobs_time_max_glob
                    [((*jptr).tasktype as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int) as usize]
            {
                (*jp).jobs_time_max_glob[((*jptr).tasktype as ::core::ffi::c_int
                    & 0x7 as ::core::ffi::c_int)
                    as usize] = tasktime;
            }
            (*jptr).tasktype =
                ((*jptr).tasktype as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int) as uint8_t;
            (*jptr).starttime = 0 as uint64_t;
        }
        let mut _mfs_assert_ret_5: ::core::ffi::c_int =
            pthread_mutex_unlock(&raw mut (*jp).jobslock);
        if _mfs_assert_ret_5 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_5 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_11: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    450 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_5,
                    *__errno_location(),
                    _mfs_errorstring_11,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    450 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_5,
                    *__errno_location(),
                    _mfs_errorstring_11,
                );
            } else if _mfs_assert_ret_5 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_12: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_5);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    450 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_5,
                    _mfs_errorstring_12,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    450 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_5,
                    _mfs_errorstring_12,
                );
            } else {
                let mut _mfs_errorstring_err_5: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_5: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_5);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    450 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_5,
                    _mfs_errorstring_ret_5,
                    *__errno_location(),
                    _mfs_errorstring_err_5,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    450 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_5,
                    _mfs_errorstring_ret_5,
                    *__errno_location(),
                    _mfs_errorstring_err_5,
                );
            }
            abort();
        }
        job_send_status(jp, jobid, status);
        let mut _mfs_assert_ret_6: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*jp).jobslock);
        if _mfs_assert_ret_6 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_6 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_13: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    452 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_6,
                    *__errno_location(),
                    _mfs_errorstring_13,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    452 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_6,
                    *__errno_location(),
                    _mfs_errorstring_13,
                );
            } else if _mfs_assert_ret_6 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_14: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_6);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    452 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_6,
                    _mfs_errorstring_14,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    452 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_6,
                    _mfs_errorstring_14,
                );
            } else {
                let mut _mfs_errorstring_err_6: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_6: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_6);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    452 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_6,
                    _mfs_errorstring_ret_6,
                    *__errno_location(),
                    _mfs_errorstring_err_6,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    452 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_6,
                    _mfs_errorstring_ret_6,
                    *__errno_location(),
                    _mfs_errorstring_err_6,
                );
            }
            abort();
        }
        (*jp).workers_avail = (*jp).workers_avail.wrapping_add(1);
        if (*jp).workers_avail > (*jp).workers_max_idle {
            job_close_worker(w);
            let mut _mfs_assert_ret_7: ::core::ffi::c_int =
                pthread_mutex_unlock(&raw mut (*jp).jobslock);
            if _mfs_assert_ret_7 != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret_7 < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_15: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        456 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_7,
                        *__errno_location(),
                        _mfs_errorstring_15,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        456 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_7,
                        *__errno_location(),
                        _mfs_errorstring_15,
                    );
                } else if _mfs_assert_ret_7 > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_16: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_7);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        456 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_7,
                        _mfs_errorstring_16,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        456 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_7,
                        _mfs_errorstring_16,
                    );
                } else {
                    let mut _mfs_errorstring_err_7: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    let mut _mfs_errorstring_ret_7: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_7);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        456 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_7,
                        _mfs_errorstring_ret_7,
                        *__errno_location(),
                        _mfs_errorstring_err_7,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        456 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_7,
                        _mfs_errorstring_ret_7,
                        *__errno_location(),
                        _mfs_errorstring_err_7,
                    );
                }
                abort();
            }
            return NULL;
        }
        let mut _mfs_assert_ret_8: ::core::ffi::c_int =
            pthread_mutex_unlock(&raw mut (*jp).jobslock);
        if _mfs_assert_ret_8 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_8 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_17: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    459 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_8,
                    *__errno_location(),
                    _mfs_errorstring_17,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    459 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_8,
                    *__errno_location(),
                    _mfs_errorstring_17,
                );
            } else if _mfs_assert_ret_8 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_18: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_8);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    459 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_8,
                    _mfs_errorstring_18,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    459 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_8,
                    _mfs_errorstring_18,
                );
            } else {
                let mut _mfs_errorstring_err_8: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_8: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_8);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    459 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_8,
                    _mfs_errorstring_ret_8,
                    *__errno_location(),
                    _mfs_errorstring_err_8,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    459 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_8,
                    _mfs_errorstring_ret_8,
                    *__errno_location(),
                    _mfs_errorstring_err_8,
                );
            }
            abort();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn job_hp_worker(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    return job_worker(arg);
}
#[no_mangle]
pub unsafe extern "C" fn job_lp_worker(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    ionice_medium();
    return job_worker(arg);
}
#[inline]
unsafe extern "C" fn job_op_to_tasktype(mut op: uint32_t) -> uint32_t {
    match op {
        2 => return TASK_CHUNKOP as uint32_t,
        3 => return TASK_READ as uint32_t,
        4 => return TASK_WRITE as uint32_t,
        5 | 6 | 7 | 8 => return TASK_REPLICATE as uint32_t,
        10 => return TASK_MOVE as uint32_t,
        _ => return TASK_INFO as uint32_t,
    };
}
pub const JOB_MODE_ALWAYS_DO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const JOB_MODE_LIMITED_RETURN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const JOB_MODE_LIMITED_QUEUE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn job_new(
    mut jp: *mut jobpool,
    mut op: uint32_t,
    mut chunkid: uint64_t,
    mut args: *mut ::core::ffi::c_void,
    mut callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
    mut extra: *mut ::core::ffi::c_void,
    mut errstatus: uint8_t,
    mut jobmode: uint8_t,
) -> uint32_t {
    let mut jobid: uint32_t = 0;
    let mut jhpos: uint32_t = 0;
    let mut workers_busy: uint32_t = 0;
    let mut limit: uint32_t = 0;
    let mut jhandle: *mut *mut job = ::core::ptr::null_mut::<*mut job>();
    let mut jptr: *mut job = ::core::ptr::null_mut::<job>();
    jptr = malloc(::core::mem::size_of::<job>()) as *mut job;
    if jptr.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            517 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"jptr\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            517 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"jptr\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if jptr
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut job
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            517 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"jptr\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            517 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"jptr\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*jp).jobslock);
    if _mfs_assert_ret != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                519 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                519 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                519 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                519 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_1,
            );
        } else {
            let mut _mfs_errorstring_err: *const ::core::ffi::c_char = strerr(*__errno_location());
            let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                519 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                519 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    jobid = (*jp).nextjobid;
    (*jp).nextjobid = (*jp).nextjobid.wrapping_add(1);
    if (*jp).nextjobid == 0 as uint32_t {
        (*jp).nextjobid = 1 as uint32_t;
    }
    jhpos = jobid & 0x3ff as uint32_t;
    (*jptr).jobid = jobid;
    (*jptr).callback = callback;
    (*jptr).extra = extra;
    (*jptr).args = args;
    (*jptr).jstate = JSTATE_ENABLED as ::core::ffi::c_int as uint8_t;
    (*jptr).starttime = 0 as uint64_t;
    (*jptr).chunkid = chunkid;
    (*jptr).tasktype = job_op_to_tasktype(op) as uint8_t;
    (*jptr).next = (*jp).jobhash[jhpos as usize] as *mut _job;
    (*jp).jobhash[jhpos as usize] = jptr;
    workers_busy = (*jp).workers_total.wrapping_sub((*jp).workers_avail);
    limit = (*jp).workers_max;
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*jp).jobslock);
    if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_2: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                538 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                538 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_2,
            );
        } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
            && *__errno_location() == 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_3: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                538 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                538 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_3,
            );
        } else {
            let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                538 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                538 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
    if queue_elements((*jp).jobqueue).wrapping_add(workers_busy) > limit
        && jobmode as ::core::ffi::c_int != JOB_MODE_ALWAYS_DO
    {
        if jobmode as ::core::ffi::c_int == JOB_MODE_LIMITED_RETURN {
            let mut _mfs_assert_ret_1: ::core::ffi::c_int =
                pthread_mutex_lock(&raw mut (*jp).jobslock);
            if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_4: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        542 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_1,
                        *__errno_location(),
                        _mfs_errorstring_4,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        542 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_1,
                        *__errno_location(),
                        _mfs_errorstring_4,
                    );
                } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_5: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_1);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        542 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_1,
                        _mfs_errorstring_5,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        542 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_1,
                        _mfs_errorstring_5,
                    );
                } else {
                    let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_1);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        542 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_1,
                        _mfs_errorstring_ret_1,
                        *__errno_location(),
                        _mfs_errorstring_err_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        542 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_1,
                        _mfs_errorstring_ret_1,
                        *__errno_location(),
                        _mfs_errorstring_err_1,
                    );
                }
                abort();
            }
            jhandle = (&raw mut (*jp).jobhash as *mut *mut job).offset(jhpos as isize);
            loop {
                jptr = *jhandle;
                if jptr.is_null() {
                    break;
                }
                if (*jptr).jobid == jobid {
                    *jhandle = (*jptr).next as *mut job;
                    if !(*jptr).args.is_null() {
                        free((*jptr).args);
                    }
                    free(jptr as *mut ::core::ffi::c_void);
                    break;
                } else {
                    jhandle = &raw mut (*jptr).next as *mut *mut job;
                }
            }
            let mut _mfs_assert_ret_2: ::core::ffi::c_int =
                pthread_mutex_unlock(&raw mut (*jp).jobslock);
            if _mfs_assert_ret_2 != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret_2 < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_6: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        556 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        *__errno_location(),
                        _mfs_errorstring_6,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        556 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        *__errno_location(),
                        _mfs_errorstring_6,
                    );
                } else if _mfs_assert_ret_2 > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_7: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_2);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        556 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        _mfs_errorstring_7,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        556 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        _mfs_errorstring_7,
                    );
                } else {
                    let mut _mfs_errorstring_err_2: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    let mut _mfs_errorstring_ret_2: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_2);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        556 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        _mfs_errorstring_ret_2,
                        *__errno_location(),
                        _mfs_errorstring_err_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        556 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        _mfs_errorstring_ret_2,
                        *__errno_location(),
                        _mfs_errorstring_err_2,
                    );
                }
                abort();
            }
            return 0 as uint32_t;
        } else {
            job_send_status(jp, jobid, errstatus);
        }
    } else {
        queue_put(
            (*jp).jobqueue,
            jobid,
            op,
            jptr as *mut uint8_t,
            1 as uint32_t,
        );
    }
    return jobid;
}
#[no_mangle]
pub unsafe extern "C" fn job_pool_new(
    mut worker_fn: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
    >,
) -> *mut ::core::ffi::c_void {
    let mut fd: [::core::ffi::c_int; 2] = [0; 2];
    let mut i: uint32_t = 0;
    let mut jp: *mut jobpool = ::core::ptr::null_mut::<jobpool>();
    if pipe(&raw mut fd as *mut ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        return NULL;
    }
    jp = malloc(::core::mem::size_of::<jobpool>()) as *mut jobpool;
    if jp.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            580 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"jp\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            580 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"jp\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if jp
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut jobpool
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            580 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"jp\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            580 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"jp\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    (*jp).worker_fn = worker_fn;
    (*jp).rpipe = fd[0 as usize];
    (*jp).wpipe = fd[1 as usize];
    (*jp).workers_avail = 0 as uint32_t;
    (*jp).workers_total = 0 as uint32_t;
    (*jp).workers_term_waiting = 0 as uint32_t;
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_cond_init(
        &raw mut (*jp).worker_term_cond,
        ::core::ptr::null::<pthread_condattr_t>(),
    );
    if _mfs_assert_ret != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                588 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_init(&(jp->worker_term_cond),NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                588 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_init(&(jp->worker_term_cond),NULL)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                588 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_init(&(jp->worker_term_cond),NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                588 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_init(&(jp->worker_term_cond),NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_1,
            );
        } else {
            let mut _mfs_errorstring_err: *const ::core::ffi::c_char = strerr(*__errno_location());
            let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                588 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_init(&(jp->worker_term_cond),NULL)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                588 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_init(&(jp->worker_term_cond),NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_init(
        &raw mut (*jp).pipelock,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    );
    if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_2: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                589 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(jp->pipelock),NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                589 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(jp->pipelock),NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_2,
            );
        } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
            && *__errno_location() == 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_3: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                589 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(jp->pipelock),NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                589 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(jp->pipelock),NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_3,
            );
        } else {
            let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                589 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(jp->pipelock),NULL)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                589 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(jp->pipelock),NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
    let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_init(
        &raw mut (*jp).jobslock,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    );
    if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_4: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                590 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(jp->jobslock),NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                590 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(jp->jobslock),NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_4,
            );
        } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
            && *__errno_location() == 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_5: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                590 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(jp->jobslock),NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_5,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                590 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(jp->jobslock),NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_5,
            );
        } else {
            let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                590 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(jp->jobslock),NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                590 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(jp->jobslock),NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
        }
        abort();
    }
    (*jp).jobqueue = queue_new(0 as uint32_t);
    (*jp).statusqueue = queue_new(0 as uint32_t);
    let mut _mfs_assert_ret_2: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*jp).jobslock);
    if _mfs_assert_ret_2 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_2 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_6: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                594 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                *__errno_location(),
                _mfs_errorstring_6,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                594 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                *__errno_location(),
                _mfs_errorstring_6,
            );
        } else if _mfs_assert_ret_2 > 0 as ::core::ffi::c_int
            && *__errno_location() == 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_7: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_2);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                594 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_7,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                594 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_7,
            );
        } else {
            let mut _mfs_errorstring_err_2: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_2: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_2);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                594 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_ret_2,
                *__errno_location(),
                _mfs_errorstring_err_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                594 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_ret_2,
                *__errno_location(),
                _mfs_errorstring_err_2,
            );
        }
        abort();
    }
    i = 0 as uint32_t;
    while i < TASK_COUNT as uint32_t {
        (*jp).jobs_time_max_glob[i as usize] = 0 as uint64_t;
        (*jp).jobs_time_max_prev[i as usize] = 0 as uint64_t;
        (*jp).jobs_count_prev[i as usize] = 0 as uint32_t;
        (*jp).jobs_time_max[i as usize] = 0 as uint64_t;
        (*jp).jobs_time[i as usize] = 0 as uint64_t;
        (*jp).jobs_count[i as usize] = 0 as uint32_t;
        i = i.wrapping_add(1);
    }
    i = 0 as uint32_t;
    while i < JHASHSIZE as uint32_t {
        (*jp).jobhash[i as usize] = ::core::ptr::null_mut::<job>();
        i = i.wrapping_add(1);
    }
    (*jp).nextjobid = 1 as uint32_t;
    job_spawn_worker(jp);
    let mut _mfs_assert_ret_3: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*jp).jobslock);
    if _mfs_assert_ret_3 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_3 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_8: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                608 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                *__errno_location(),
                _mfs_errorstring_8,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                608 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                *__errno_location(),
                _mfs_errorstring_8,
            );
        } else if _mfs_assert_ret_3 > 0 as ::core::ffi::c_int
            && *__errno_location() == 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_9: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_3);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                608 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                _mfs_errorstring_9,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                608 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                _mfs_errorstring_9,
            );
        } else {
            let mut _mfs_errorstring_err_3: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_3: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_3);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                608 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                _mfs_errorstring_ret_3,
                *__errno_location(),
                _mfs_errorstring_err_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                608 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                _mfs_errorstring_ret_3,
                *__errno_location(),
                _mfs_errorstring_err_3,
            );
        }
        abort();
    }
    return jp as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn job_pool_jobs_count(mut jp: *mut jobpool) -> uint32_t {
    let mut res: uint32_t = 0;
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*jp).jobslock);
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                614 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                614 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                614 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                614 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
        } else {
            let mut _mfs_errorstring_err: *const ::core::ffi::c_char = strerr(*__errno_location());
            let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                614 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                614 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    res = (*jp)
        .workers_total
        .wrapping_sub((*jp).workers_avail)
        .wrapping_add(queue_elements((*jp).jobqueue));
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*jp).jobslock);
    if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_1: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                616 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                616 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                616 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                616 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
        } else {
            let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                616 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                616 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
    return res;
}
#[inline]
unsafe extern "C" fn job_pool_disable_job_in_pool(mut jp: *mut jobpool, mut jobid: uint32_t) {
    let mut jhpos: uint32_t = jobid & 0x3ff as uint32_t;
    let mut jptr: *mut job = ::core::ptr::null_mut::<job>();
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*jp).jobslock);
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                643 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                643 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                643 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                643 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
        } else {
            let mut _mfs_errorstring_err: *const ::core::ffi::c_char = strerr(*__errno_location());
            let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                643 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                643 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    jptr = (*jp).jobhash[jhpos as usize];
    while !jptr.is_null() {
        if (*jptr).jobid == jobid {
            if (*jptr).jstate as ::core::ffi::c_int == JSTATE_ENABLED as ::core::ffi::c_int {
                (*jptr).jstate = JSTATE_DISABLED as ::core::ffi::c_int as uint8_t;
            }
        }
        jptr = (*jptr).next as *mut job;
    }
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*jp).jobslock);
    if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_1: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                651 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                651 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                651 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                651 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
        } else {
            let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                651 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                651 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
}
#[no_mangle]
pub unsafe extern "C" fn job_pool_disable_job(mut jobid: uint32_t) {
    job_pool_disable_job_in_pool(hp_pool, jobid);
    job_pool_disable_job_in_pool(lp_pool, jobid);
}
#[inline]
unsafe extern "C" fn job_pool_change_callback_in_pool(
    mut jp: *mut jobpool,
    mut jobid: uint32_t,
    mut callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
    mut extra: *mut ::core::ffi::c_void,
) {
    let mut jhpos: uint32_t = jobid & 0x3ff as uint32_t;
    let mut jptr: *mut job = ::core::ptr::null_mut::<job>();
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*jp).jobslock);
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                663 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                663 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                663 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                663 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
        } else {
            let mut _mfs_errorstring_err: *const ::core::ffi::c_char = strerr(*__errno_location());
            let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                663 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                663 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    jptr = (*jp).jobhash[jhpos as usize];
    while !jptr.is_null() {
        if (*jptr).jobid == jobid {
            (*jptr).callback = callback;
            (*jptr).extra = extra;
        }
        jptr = (*jptr).next as *mut job;
    }
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*jp).jobslock);
    if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_1: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                670 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                670 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                670 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                670 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
        } else {
            let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                670 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                670 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
}
#[no_mangle]
pub unsafe extern "C" fn job_pool_change_callback(
    mut jobid: uint32_t,
    mut callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
    mut extra: *mut ::core::ffi::c_void,
) {
    job_pool_change_callback_in_pool(hp_pool, jobid, callback, extra);
    job_pool_change_callback_in_pool(lp_pool, jobid, callback, extra);
}
#[inline]
unsafe extern "C" fn job_pool_check_jobs_in_pool(mut jp: *mut jobpool, mut cb: uint8_t) {
    let mut jobid: uint32_t = 0;
    let mut jhpos: uint32_t = 0;
    let mut status: uint8_t = 0;
    let mut notlast: ::core::ffi::c_int = 0;
    let mut jhandle: *mut *mut job = ::core::ptr::null_mut::<*mut job>();
    let mut jptr: *mut job = ::core::ptr::null_mut::<job>();
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*jp).jobslock);
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                684 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                684 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                684 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                684 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
        } else {
            let mut _mfs_errorstring_err: *const ::core::ffi::c_char = strerr(*__errno_location());
            let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                684 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                684 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    loop {
        notlast = job_receive_status(jp, &raw mut jobid, &raw mut status);
        jhpos = jobid & 0x3ff as uint32_t;
        jhandle = (&raw mut (*jp).jobhash as *mut *mut job).offset(jhpos as isize);
        loop {
            jptr = *jhandle;
            if jptr.is_null() {
                break;
            }
            if (*jptr).jobid == jobid {
                if (*jptr).callback.is_some() && cb as ::core::ffi::c_int != 0 {
                    (*jptr).callback.expect("non-null function pointer")(status, (*jptr).extra);
                }
                *jhandle = (*jptr).next as *mut job;
                if !(*jptr).args.is_null() {
                    free((*jptr).args);
                }
                free(jptr as *mut ::core::ffi::c_void);
                break;
            } else {
                jhandle = &raw mut (*jptr).next as *mut *mut job;
            }
        }
        if notlast == 0 {
            break;
        }
    }
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*jp).jobslock);
    if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_1: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                705 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                705 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                705 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                705 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
        } else {
            let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                705 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                705 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
}
#[no_mangle]
pub unsafe extern "C" fn job_pool_delete(mut jp: *mut jobpool) {
    queue_close((*jp).jobqueue);
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*jp).jobslock);
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                716 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                716 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                716 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                716 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
        } else {
            let mut _mfs_errorstring_err: *const ::core::ffi::c_char = strerr(*__errno_location());
            let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                716 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                716 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    while (*jp).workers_total > 0 as uint32_t {
        (*jp).workers_term_waiting = (*jp).workers_term_waiting.wrapping_add(1);
        let mut _mfs_assert_ret_0: ::core::ffi::c_int =
            pthread_cond_wait(&raw mut (*jp).worker_term_cond, &raw mut (*jp).jobslock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    719 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(jp->worker_term_cond),&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    719 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(jp->worker_term_cond),&(jp->jobslock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    719 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(jp->worker_term_cond),&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    719 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(jp->worker_term_cond),&(jp->jobslock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    719 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(jp->worker_term_cond),&(jp->jobslock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    719 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(jp->worker_term_cond),&(jp->jobslock))\0".as_ptr()
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
    let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*jp).jobslock);
    if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_3: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                721 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                721 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_3,
            );
        } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
            && *__errno_location() == 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_4: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                721 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                721 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
        } else {
            let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                721 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                721 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
        }
        abort();
    }
    if queue_isempty((*jp).statusqueue) == 0 {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"not empty job queue !!!\0".as_ptr() as *const ::core::ffi::c_char,
        );
        job_pool_check_jobs_in_pool(jp, 0 as uint8_t);
    }
    queue_delete((*jp).jobqueue);
    queue_delete((*jp).statusqueue);
    let mut _mfs_assert_ret_2: ::core::ffi::c_int =
        pthread_cond_destroy(&raw mut (*jp).worker_term_cond);
    if _mfs_assert_ret_2 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_2 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_5: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                729 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_destroy(&(jp->worker_term_cond))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                *__errno_location(),
                _mfs_errorstring_5,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                729 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_destroy(&(jp->worker_term_cond))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                *__errno_location(),
                _mfs_errorstring_5,
            );
        } else if _mfs_assert_ret_2 > 0 as ::core::ffi::c_int
            && *__errno_location() == 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_6: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_2);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                729 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_destroy(&(jp->worker_term_cond))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_6,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                729 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_destroy(&(jp->worker_term_cond))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_6,
            );
        } else {
            let mut _mfs_errorstring_err_2: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_2: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_2);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                729 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_destroy(&(jp->worker_term_cond))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_ret_2,
                *__errno_location(),
                _mfs_errorstring_err_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                729 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_destroy(&(jp->worker_term_cond))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_ret_2,
                *__errno_location(),
                _mfs_errorstring_err_2,
            );
        }
        abort();
    }
    let mut _mfs_assert_ret_3: ::core::ffi::c_int = pthread_mutex_destroy(&raw mut (*jp).pipelock);
    if _mfs_assert_ret_3 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_3 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_7: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                730 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                *__errno_location(),
                _mfs_errorstring_7,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                730 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                *__errno_location(),
                _mfs_errorstring_7,
            );
        } else if _mfs_assert_ret_3 > 0 as ::core::ffi::c_int
            && *__errno_location() == 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_8: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_3);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                730 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                _mfs_errorstring_8,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                730 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                _mfs_errorstring_8,
            );
        } else {
            let mut _mfs_errorstring_err_3: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_3: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_3);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                730 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                _mfs_errorstring_ret_3,
                *__errno_location(),
                _mfs_errorstring_err_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                730 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&(jp->pipelock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                _mfs_errorstring_ret_3,
                *__errno_location(),
                _mfs_errorstring_err_3,
            );
        }
        abort();
    }
    let mut _mfs_assert_ret_4: ::core::ffi::c_int = pthread_mutex_destroy(&raw mut (*jp).jobslock);
    if _mfs_assert_ret_4 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_4 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_9: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                731 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_4,
                *__errno_location(),
                _mfs_errorstring_9,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                731 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_4,
                *__errno_location(),
                _mfs_errorstring_9,
            );
        } else if _mfs_assert_ret_4 > 0 as ::core::ffi::c_int
            && *__errno_location() == 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_10: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_4);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                731 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_4,
                _mfs_errorstring_10,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                731 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_4,
                _mfs_errorstring_10,
            );
        } else {
            let mut _mfs_errorstring_err_4: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_4: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_4);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                731 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_4,
                _mfs_errorstring_ret_4,
                *__errno_location(),
                _mfs_errorstring_err_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                731 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_4,
                _mfs_errorstring_ret_4,
                *__errno_location(),
                _mfs_errorstring_err_4,
            );
        }
        abort();
    }
    close((*jp).rpipe);
    close((*jp).wpipe);
    free(jp as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn job_inval(
    mut callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
    mut extra: *mut ::core::ffi::c_void,
) -> uint32_t {
    let mut jp: *mut jobpool = lp_pool;
    return job_new(
        jp,
        OP_INVAL as ::core::ffi::c_int as uint32_t,
        0 as uint64_t,
        NULL,
        callback,
        extra,
        MFS_ERROR_EINVAL as uint8_t,
        JOB_MODE_LIMITED_QUEUE as uint8_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn job_chunkop(
    mut callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
    mut extra: *mut ::core::ffi::c_void,
    mut chunkid: uint64_t,
    mut version: uint32_t,
    mut newversion: uint32_t,
    mut copychunkid: uint64_t,
    mut copyversion: uint32_t,
    mut length: uint32_t,
) -> uint32_t {
    let mut jp: *mut jobpool = hp_pool;
    let mut args: *mut chunk_op_args = ::core::ptr::null_mut::<chunk_op_args>();
    args = malloc(::core::mem::size_of::<chunk_op_args>()) as *mut chunk_op_args;
    if args.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            757 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            757 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if args
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut chunk_op_args
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            757 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            757 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    (*args).chunkid = chunkid;
    (*args).version = version;
    (*args).newversion = newversion;
    (*args).copychunkid = copychunkid;
    (*args).copyversion = copyversion;
    (*args).length = length;
    return job_new(
        jp,
        OP_CHUNKOP as ::core::ffi::c_int as uint32_t,
        chunkid,
        args as *mut ::core::ffi::c_void,
        callback,
        extra,
        MFS_ERROR_NOTDONE as uint8_t,
        JOB_MODE_LIMITED_QUEUE as uint8_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn job_serv_read(
    mut callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
    mut extra: *mut ::core::ffi::c_void,
    mut sock: ::core::ffi::c_int,
    mut packet: *const uint8_t,
    mut length: uint32_t,
) -> uint32_t {
    let mut jp: *mut jobpool = hp_pool;
    let mut args: *mut chunk_rw_args = ::core::ptr::null_mut::<chunk_rw_args>();
    args = malloc(::core::mem::size_of::<chunk_rw_args>()) as *mut chunk_rw_args;
    if args.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            822 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            822 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if args
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut chunk_rw_args
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            822 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            822 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    (*args).sock = sock;
    (*args).packet = packet;
    (*args).length = length;
    return job_new(
        jp,
        OP_SERV_READ as ::core::ffi::c_int as uint32_t,
        0 as uint64_t,
        args as *mut ::core::ffi::c_void,
        callback,
        extra,
        0 as uint8_t,
        JOB_MODE_LIMITED_RETURN as uint8_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn job_serv_write(
    mut callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
    mut extra: *mut ::core::ffi::c_void,
    mut sock: ::core::ffi::c_int,
    mut packet: *const uint8_t,
    mut length: uint32_t,
) -> uint32_t {
    let mut jp: *mut jobpool = hp_pool;
    let mut args: *mut chunk_rw_args = ::core::ptr::null_mut::<chunk_rw_args>();
    args = malloc(::core::mem::size_of::<chunk_rw_args>()) as *mut chunk_rw_args;
    if args.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            833 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            833 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if args
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut chunk_rw_args
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            833 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            833 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    (*args).sock = sock;
    (*args).packet = packet;
    (*args).length = length;
    return job_new(
        jp,
        OP_SERV_WRITE as ::core::ffi::c_int as uint32_t,
        0 as uint64_t,
        args as *mut ::core::ffi::c_void,
        callback,
        extra,
        0 as uint8_t,
        JOB_MODE_LIMITED_RETURN as uint8_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn job_replicate_simple(
    mut callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
    mut extra: *mut ::core::ffi::c_void,
    mut chunkid: uint64_t,
    mut version: uint32_t,
    mut srcip: uint32_t,
    mut srcport: uint16_t,
) -> uint32_t {
    let mut jp: *mut jobpool = lp_pool;
    let mut args: *mut chunk_rp_args = ::core::ptr::null_mut::<chunk_rp_args>();
    args = malloc(::core::mem::size_of::<chunk_rp_args>()) as *mut chunk_rp_args;
    if args.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            844 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            844 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if args
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut chunk_rp_args
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            844 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            844 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    memset(
        args as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<chunk_rp_args>(),
    );
    (*args).chunkid = chunkid;
    (*args).version = version;
    (*args).srcip[0 as usize] = srcip;
    (*args).srcport[0 as usize] = srcport;
    (*args).srcchunkid[0 as usize] = chunkid;
    return job_new(
        jp,
        OP_REPLICATE_SIMPLE as ::core::ffi::c_int as uint32_t,
        chunkid,
        args as *mut ::core::ffi::c_void,
        callback,
        extra,
        MFS_ERROR_NOTDONE as uint8_t,
        JOB_MODE_LIMITED_QUEUE as uint8_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn job_replicate_split(
    mut callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
    mut extra: *mut ::core::ffi::c_void,
    mut chunkid: uint64_t,
    mut version: uint32_t,
    mut srcip: uint32_t,
    mut srcport: uint16_t,
    mut srcchunkid: uint64_t,
    mut partno: uint8_t,
    mut parts: uint8_t,
) -> uint32_t {
    let mut jp: *mut jobpool = lp_pool;
    let mut args: *mut chunk_rp_args = ::core::ptr::null_mut::<chunk_rp_args>();
    args = malloc(::core::mem::size_of::<chunk_rp_args>()) as *mut chunk_rp_args;
    if args.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            858 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            858 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if args
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut chunk_rp_args
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            858 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            858 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    memset(
        args as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<chunk_rp_args>(),
    );
    (*args).chunkid = chunkid;
    (*args).version = version;
    (*args).srcip[0 as usize] = srcip;
    (*args).srcport[0 as usize] = srcport;
    (*args).srcchunkid[0 as usize] = srcchunkid;
    (*args).partno = partno;
    (*args).parts = parts;
    return job_new(
        jp,
        OP_REPLICATE_SPLIT as ::core::ffi::c_int as uint32_t,
        chunkid,
        args as *mut ::core::ffi::c_void,
        callback,
        extra,
        MFS_ERROR_NOTDONE as uint8_t,
        JOB_MODE_LIMITED_QUEUE as uint8_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn job_replicate_recover(
    mut callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
    mut extra: *mut ::core::ffi::c_void,
    mut chunkid: uint64_t,
    mut version: uint32_t,
    mut parts: uint8_t,
    mut srcip: *mut uint32_t,
    mut srcport: *mut uint16_t,
    mut srcchunkid: *mut uint64_t,
) -> uint32_t {
    let mut jp: *mut jobpool = lp_pool;
    let mut args: *mut chunk_rp_args = ::core::ptr::null_mut::<chunk_rp_args>();
    let mut i: uint8_t = 0;
    args = malloc(::core::mem::size_of::<chunk_rp_args>()) as *mut chunk_rp_args;
    if args.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            875 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            875 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if args
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut chunk_rp_args
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            875 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            875 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    memset(
        args as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<chunk_rp_args>(),
    );
    (*args).chunkid = chunkid;
    (*args).version = version;
    (*args).parts = parts;
    i = 0 as uint8_t;
    while (i as ::core::ffi::c_int) < parts as ::core::ffi::c_int {
        (*args).srcip[i as usize] = *srcip.offset(i as isize);
        (*args).srcport[i as usize] = *srcport.offset(i as isize);
        (*args).srcchunkid[i as usize] = *srcchunkid.offset(i as isize);
        i = i.wrapping_add(1);
    }
    return job_new(
        jp,
        OP_REPLICATE_RECOVER as ::core::ffi::c_int as uint32_t,
        chunkid,
        args as *mut ::core::ffi::c_void,
        callback,
        extra,
        MFS_ERROR_NOTDONE as uint8_t,
        JOB_MODE_LIMITED_QUEUE as uint8_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn job_replicate_join(
    mut callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
    mut extra: *mut ::core::ffi::c_void,
    mut chunkid: uint64_t,
    mut version: uint32_t,
    mut parts: uint8_t,
    mut srcip: *mut uint32_t,
    mut srcport: *mut uint16_t,
    mut srcchunkid: *mut uint64_t,
) -> uint32_t {
    let mut jp: *mut jobpool = lp_pool;
    let mut args: *mut chunk_rp_args = ::core::ptr::null_mut::<chunk_rp_args>();
    let mut i: uint8_t = 0;
    args = malloc(::core::mem::size_of::<chunk_rp_args>()) as *mut chunk_rp_args;
    if args.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            893 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            893 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if args
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut chunk_rp_args
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            893 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            893 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    memset(
        args as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<chunk_rp_args>(),
    );
    (*args).chunkid = chunkid;
    (*args).version = version;
    (*args).parts = parts;
    i = 0 as uint8_t;
    while (i as ::core::ffi::c_int) < parts as ::core::ffi::c_int {
        (*args).srcip[i as usize] = *srcip.offset(i as isize);
        (*args).srcport[i as usize] = *srcport.offset(i as isize);
        (*args).srcchunkid[i as usize] = *srcchunkid.offset(i as isize);
        i = i.wrapping_add(1);
    }
    return job_new(
        jp,
        OP_REPLICATE_JOIN as ::core::ffi::c_int as uint32_t,
        chunkid,
        args as *mut ::core::ffi::c_void,
        callback,
        extra,
        MFS_ERROR_NOTDONE as uint8_t,
        JOB_MODE_LIMITED_QUEUE as uint8_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn job_get_chunk_info(
    mut callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
    mut extra: *mut ::core::ffi::c_void,
    mut chunkid: uint64_t,
    mut version: uint32_t,
    mut requested_info: uint8_t,
    mut info_buff: *mut uint8_t,
) -> uint32_t {
    let mut jp: *mut jobpool = lp_pool;
    let mut args: *mut chunk_ij_args = ::core::ptr::null_mut::<chunk_ij_args>();
    args = malloc(::core::mem::size_of::<chunk_ij_args>()) as *mut chunk_ij_args;
    if args.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            910 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            910 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if args
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut chunk_ij_args
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            910 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            910 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    (*args).chunkid = chunkid;
    (*args).version = version;
    (*args).requested_info = requested_info;
    (*args).pointer = info_buff as *mut ::core::ffi::c_void;
    return job_new(
        jp,
        OP_GETINFO as ::core::ffi::c_int as uint32_t,
        chunkid,
        args as *mut ::core::ffi::c_void,
        callback,
        extra,
        MFS_ERROR_NOTDONE as uint8_t,
        JOB_MODE_LIMITED_QUEUE as uint8_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn job_chunk_move(
    mut callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
    mut extra: *mut ::core::ffi::c_void,
    mut fsrc: *mut ::core::ffi::c_void,
    mut fdst: *mut ::core::ffi::c_void,
) -> uint32_t {
    let mut jp: *mut jobpool = lp_pool;
    let mut args: *mut chunk_mv_args = ::core::ptr::null_mut::<chunk_mv_args>();
    args = malloc(::core::mem::size_of::<chunk_mv_args>()) as *mut chunk_mv_args;
    if args.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            922 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            922 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if args
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut chunk_mv_args
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            922 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr() as *const ::core::ffi::c_char,
            922 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"args\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    (*args).fsrc = fsrc;
    (*args).fdst = fdst;
    return job_new(
        jp,
        OP_CHUNKMOVE as ::core::ffi::c_int as uint32_t,
        0 as uint64_t,
        args as *mut ::core::ffi::c_void,
        callback,
        extra,
        MFS_ERROR_NOTDONE as uint8_t,
        JOB_MODE_LIMITED_QUEUE as uint8_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn job_desc(mut pdesc: *mut pollfd, mut ndesc: *mut uint32_t) {
    let mut pos: uint32_t = *ndesc;
    (*pdesc.offset(pos as isize)).fd = (*hp_pool).rpipe;
    (*pdesc.offset(pos as isize)).events = POLLIN as ::core::ffi::c_short;
    (*hp_pool).fdpdescpos = pos as int32_t;
    pos = pos.wrapping_add(1);
    (*pdesc.offset(pos as isize)).fd = (*lp_pool).rpipe;
    (*pdesc.offset(pos as isize)).events = POLLIN as ::core::ffi::c_short;
    (*lp_pool).fdpdescpos = pos as int32_t;
    pos = pos.wrapping_add(1);
    *ndesc = pos;
}
#[no_mangle]
pub unsafe extern "C" fn job_serve(mut pdesc: *mut pollfd) {
    let mut jp: *mut jobpool = ::core::ptr::null_mut::<jobpool>();
    let mut jobscnt: uint32_t = 0;
    jp = hp_pool;
    if (*jp).fdpdescpos >= 0 as int32_t
        && (*pdesc.offset((*jp).fdpdescpos as isize)).revents as ::core::ffi::c_int & POLLIN != 0
    {
        job_pool_check_jobs_in_pool(jp, 1 as uint8_t);
    }
    jp = lp_pool;
    if (*jp).fdpdescpos >= 0 as int32_t
        && (*pdesc.offset((*jp).fdpdescpos as isize)).revents as ::core::ffi::c_int & POLLIN != 0
    {
        job_pool_check_jobs_in_pool(jp, 1 as uint8_t);
    }
    jobscnt = job_pool_jobs_count(hp_pool).wrapping_add(job_pool_jobs_count(lp_pool));
    if jobscnt >= stats_maxjobscnt {
        stats_maxjobscnt = jobscnt;
    }
}
static mut current_hlstatus: uint8_t = HLSTATUS_OK as uint8_t;
#[no_mangle]
pub unsafe extern "C" fn job_get_load_and_hlstatus(
    mut load: *mut uint32_t,
    mut hlstatus: *mut uint8_t,
) {
    *load = job_pool_jobs_count(hp_pool).wrapping_add(job_pool_jobs_count(lp_pool));
    *hlstatus = current_hlstatus;
}
#[no_mangle]
pub unsafe extern "C" fn job_heavyload_test() {
    let mut hlstatus: uint8_t = 0;
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*hp_pool).jobslock);
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                974 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(hp_pool->jobslock))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                974 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(hp_pool->jobslock))\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                974 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(hp_pool->jobslock))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                974 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(hp_pool->jobslock))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
        } else {
            let mut _mfs_errorstring_err: *const ::core::ffi::c_char = strerr(*__errno_location());
            let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                974 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(hp_pool->jobslock))\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                974 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(hp_pool->jobslock))\0".as_ptr()
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
        pthread_mutex_lock(&raw mut (*lp_pool).jobslock);
    if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_1: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                975 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(lp_pool->jobslock))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                975 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(lp_pool->jobslock))\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                975 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(lp_pool->jobslock))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                975 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(lp_pool->jobslock))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
        } else {
            let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                975 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(lp_pool->jobslock))\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                975 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(lp_pool->jobslock))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
    hlstatus = HLSTATUS_DEFAULT as uint8_t;
    if (*hp_pool)
        .workers_total
        .wrapping_sub((*hp_pool).workers_avail)
        > (*hp_pool).workers_himark
        || (*lp_pool)
            .workers_total
            .wrapping_sub((*lp_pool).workers_avail)
            > (*lp_pool).workers_himark
    {
        hlstatus = HLSTATUS_OVERLOADED as uint8_t;
    }
    if (*hp_pool)
        .workers_total
        .wrapping_sub((*hp_pool).workers_avail)
        < (*hp_pool).workers_lomark
        && (*lp_pool)
            .workers_total
            .wrapping_sub((*lp_pool).workers_avail)
            < (*lp_pool).workers_lomark
    {
        hlstatus = HLSTATUS_OK as uint8_t;
    }
    let mut _mfs_assert_ret_1: ::core::ffi::c_int =
        pthread_mutex_unlock(&raw mut (*lp_pool).jobslock);
    if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_3: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                983 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(lp_pool->jobslock))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                983 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(lp_pool->jobslock))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_3,
            );
        } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
            && *__errno_location() == 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_4: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                983 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(lp_pool->jobslock))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                983 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(lp_pool->jobslock))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
        } else {
            let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                983 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(lp_pool->jobslock))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                983 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(lp_pool->jobslock))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
        }
        abort();
    }
    let mut _mfs_assert_ret_2: ::core::ffi::c_int =
        pthread_mutex_unlock(&raw mut (*hp_pool).jobslock);
    if _mfs_assert_ret_2 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_2 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_5: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                984 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(hp_pool->jobslock))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                *__errno_location(),
                _mfs_errorstring_5,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                984 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(hp_pool->jobslock))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                *__errno_location(),
                _mfs_errorstring_5,
            );
        } else if _mfs_assert_ret_2 > 0 as ::core::ffi::c_int
            && *__errno_location() == 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_6: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_2);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                984 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(hp_pool->jobslock))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_6,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                984 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(hp_pool->jobslock))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_6,
            );
        } else {
            let mut _mfs_errorstring_err_2: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_2: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_2);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                984 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(hp_pool->jobslock))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_ret_2,
                *__errno_location(),
                _mfs_errorstring_err_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                984 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(hp_pool->jobslock))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_ret_2,
                *__errno_location(),
                _mfs_errorstring_err_2,
            );
        }
        abort();
    }
    if hlstatus as ::core::ffi::c_int != HLSTATUS_DEFAULT
        && hlstatus as ::core::ffi::c_int != current_hlstatus as ::core::ffi::c_int
    {
        current_hlstatus = hlstatus;
        masterconn_reportload();
    }
}
#[no_mangle]
pub unsafe extern "C" fn job_info(mut fd: *mut FILE) {
    let mut jp: *mut jobpool = ::core::ptr::null_mut::<jobpool>();
    let mut wm: uint32_t = 0;
    let mut whm: uint32_t = 0;
    let mut wlm: uint32_t = 0;
    let mut wmi: uint32_t = 0;
    let mut wa: uint32_t = 0;
    let mut wt: uint32_t = 0;
    let mut qe: uint32_t = 0;
    let mut qa: uint32_t = 0;
    let mut tasks_in_queue_disabled: [uint32_t; 6] = [0; 6];
    let mut tasks_in_queue_enabled: [uint32_t; 6] = [0; 6];
    let mut tasks_in_progress: [uint32_t; 6] = [0; 6];
    let mut task_time_max_glob: [uint64_t; 6] = [0; 6];
    let mut task_time_max: [uint64_t; 6] = [0; 6];
    let mut task_time_sum: [uint64_t; 6] = [0; 6];
    let mut task_count_sum: [uint32_t; 6] = [0; 6];
    let mut i: uint32_t = 0;
    let mut h: uint32_t = 0;
    let mut j: *mut job = ::core::ptr::null_mut::<job>();
    fprintf(
        fd,
        b"[background jobs]\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    h = 0 as uint32_t;
    while h < TASK_COUNT as uint32_t {
        tasks_in_queue_disabled[h as usize] = 0 as uint32_t;
        tasks_in_queue_enabled[h as usize] = 0 as uint32_t;
        tasks_in_progress[h as usize] = 0 as uint32_t;
        task_time_max_glob[h as usize] = 0 as uint64_t;
        task_time_max[h as usize] = 0 as uint64_t;
        task_time_sum[h as usize] = 0 as uint64_t;
        task_count_sum[h as usize] = 0 as uint32_t;
        h = h.wrapping_add(1);
    }
    i = 0 as uint32_t;
    while i < 2 as uint32_t {
        jp = if i == 0 as uint32_t { hp_pool } else { lp_pool };
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*jp).jobslock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1025 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1025 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1025 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1025 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1025 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1025 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        wm = (*jp).workers_max;
        whm = (*jp).workers_himark;
        wlm = (*jp).workers_lomark;
        wmi = (*jp).workers_max_idle;
        wa = (*jp).workers_avail;
        wt = (*jp).workers_total;
        qe = queue_elements((*jp).jobqueue);
        qa = queue_sizeleft((*jp).jobqueue);
        h = 0 as uint32_t;
        while h < TASK_COUNT as uint32_t {
            task_time_sum[h as usize] =
                task_time_sum[h as usize].wrapping_add((*jp).jobs_time_prev[h as usize]);
            task_count_sum[h as usize] =
                task_count_sum[h as usize].wrapping_add((*jp).jobs_count_prev[h as usize]);
            if (*jp).jobs_time_max_prev[h as usize] > task_time_max[h as usize] {
                task_time_max[h as usize] = (*jp).jobs_time_max_prev[h as usize];
            }
            if (*jp).jobs_time_max_glob[h as usize] > task_time_max_glob[h as usize] {
                task_time_max_glob[h as usize] = (*jp).jobs_time_max_glob[h as usize];
            }
            h = h.wrapping_add(1);
        }
        h = 0 as uint32_t;
        while h < JHASHSIZE as uint32_t {
            j = (*jp).jobhash[h as usize];
            while !j.is_null() {
                if (*j).tasktype as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    match (*j).jstate as ::core::ffi::c_int {
                        2 => {
                            tasks_in_progress[((*j).tasktype as ::core::ffi::c_int
                                & 0x7 as ::core::ffi::c_int)
                                as usize] = tasks_in_progress[((*j).tasktype as ::core::ffi::c_int
                                & 0x7 as ::core::ffi::c_int)
                                as usize]
                                .wrapping_add(1);
                        }
                        1 => {
                            tasks_in_queue_enabled[((*j).tasktype as ::core::ffi::c_int
                                & 0x7 as ::core::ffi::c_int)
                                as usize] = tasks_in_queue_enabled[((*j).tasktype
                                as ::core::ffi::c_int
                                & 0x7 as ::core::ffi::c_int)
                                as usize]
                                .wrapping_add(1);
                        }
                        0 => {
                            tasks_in_queue_disabled[((*j).tasktype as ::core::ffi::c_int
                                & 0x7 as ::core::ffi::c_int)
                                as usize] = tasks_in_queue_disabled[((*j).tasktype
                                as ::core::ffi::c_int
                                & 0x7 as ::core::ffi::c_int)
                                as usize]
                                .wrapping_add(1);
                        }
                        _ => {}
                    }
                }
                j = (*j).next as *mut job;
            }
            h = h.wrapping_add(1);
        }
        let mut _mfs_assert_ret_0: ::core::ffi::c_int =
            pthread_mutex_unlock(&raw mut (*jp).jobslock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1061 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1061 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1061 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1061 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1061 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1061 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
        fprintf(
            fd,
            b"%s priority jobs params: workers_max: %u ; workers_himark: %u ; workers_lomark: %u ; workers_max_idle: %u\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            if i == 0 as uint32_t {
                b"hi\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"lo\0".as_ptr() as *const ::core::ffi::c_char
            },
            wm,
            whm,
            wlm,
            wmi,
        );
        fprintf(
            fd,
            b"%s priority jobs info: workers_total: %u ; workers_avail: %u ; queue_elements: %u ; queue_available: %u\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            if i == 0 as uint32_t {
                b"hi\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"lo\0".as_ptr() as *const ::core::ffi::c_char
            },
            wt,
            wa,
            qe,
            qa,
        );
        i = i.wrapping_add(1);
    }
    h = 0 as uint32_t;
    while h < TASK_COUNT as uint32_t {
        if task_count_sum[h as usize] > 0 as uint32_t {
            task_time_sum[h as usize] =
                task_time_sum[h as usize].wrapping_div(task_count_sum[h as usize] as uint64_t);
        } else {
            task_time_sum[h as usize] = 0 as uint64_t;
        }
        h = h.wrapping_add(1);
    }
    fprintf(
        fd,
        b"tasks in queue (enabled): reads: %u ; writes: %u ; replications: %u ; chunkops: %u ; chunkinfos: %u ; chunkmoves: %u\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        tasks_in_queue_enabled[0 as usize],
        tasks_in_queue_enabled[1 as usize],
        tasks_in_queue_enabled[2 as usize],
        tasks_in_queue_enabled[3 as usize],
        tasks_in_queue_enabled[4 as usize],
        tasks_in_queue_enabled[5 as usize],
    );
    fprintf(
        fd,
        b"tasks in queue (disabled): reads: %u ; writes: %u ; replications: %u ; chunkops: %u ; chunkinfos: %u ; chunkmoves: %u\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        tasks_in_queue_disabled[0 as usize],
        tasks_in_queue_disabled[1 as usize],
        tasks_in_queue_disabled[2 as usize],
        tasks_in_queue_disabled[3 as usize],
        tasks_in_queue_disabled[4 as usize],
        tasks_in_queue_disabled[5 as usize],
    );
    fprintf(
        fd,
        b"tasks in progress: reads: %u ; writes: %u ; replications: %u ; chunkops: %u ; chunkinfos: %u ; chunkmoves: %u\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        tasks_in_progress[0 as usize],
        tasks_in_progress[1 as usize],
        tasks_in_progress[2 as usize],
        tasks_in_progress[3 as usize],
        tasks_in_progress[4 as usize],
        tasks_in_progress[5 as usize],
    );
    fprintf(
        fd,
        b"max task times (microseconds, since start): reads: %lu ; writes: %lu ; replications: %lu ; chunkops: %lu ; chunkinfos: %lu ; chunkmoves: %lu\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        task_time_max_glob[0 as usize],
        task_time_max_glob[1 as usize],
        task_time_max_glob[2 as usize],
        task_time_max_glob[3 as usize],
        task_time_max_glob[4 as usize],
        task_time_max_glob[5 as usize],
    );
    fprintf(
        fd,
        b"max task times (microseconds, last minute): reads: %lu ; writes: %lu ; replications: %lu ; chunkops: %lu ; chunkinfos: %lu ; chunkmoves: %lu\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        task_time_max[0 as usize],
        task_time_max[1 as usize],
        task_time_max[2 as usize],
        task_time_max[3 as usize],
        task_time_max[4 as usize],
        task_time_max[5 as usize],
    );
    fprintf(
        fd,
        b"avg task times (microseconds, last minute): reads: %lu ; writes: %lu ; replications: %lu ; chunkops: %lu ; chunkinfos: %lu ; chunkmoves: %lu\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        task_time_sum[0 as usize],
        task_time_sum[1 as usize],
        task_time_sum[2 as usize],
        task_time_sum[3 as usize],
        task_time_sum[4 as usize],
        task_time_sum[5 as usize],
    );
    fprintf(
        fd,
        b"task counts (last minute): reads: %u ; writes: %u ; replications: %u ; chunkops: %u ; chunkinfos: %u ; chunkmoves: %u\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        task_count_sum[0 as usize],
        task_count_sum[1 as usize],
        task_count_sum[2 as usize],
        task_count_sum[3 as usize],
        task_count_sum[4 as usize],
        task_count_sum[5 as usize],
    );
    fprintf(fd, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn job_counters_shift() {
    let mut jp: *mut jobpool = ::core::ptr::null_mut::<jobpool>();
    let mut i: uint32_t = 0;
    let mut h: uint32_t = 0;
    i = 0 as uint32_t;
    while i < 2 as uint32_t {
        jp = if i == 0 as uint32_t { hp_pool } else { lp_pool };
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*jp).jobslock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1092 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1092 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1092 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1092 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1092 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1092 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        h = 0 as uint32_t;
        while h < TASK_COUNT as uint32_t {
            (*jp).jobs_time_max_prev[h as usize] = (*jp).jobs_time_max[h as usize];
            (*jp).jobs_time_prev[h as usize] = (*jp).jobs_time[h as usize];
            (*jp).jobs_count_prev[h as usize] = (*jp).jobs_count[h as usize];
            (*jp).jobs_time_max[h as usize] = 0 as uint64_t;
            (*jp).jobs_time[h as usize] = 0 as uint64_t;
            (*jp).jobs_count[h as usize] = 0 as uint32_t;
            h = h.wrapping_add(1);
        }
        let mut _mfs_assert_ret_0: ::core::ffi::c_int =
            pthread_mutex_unlock(&raw mut (*jp).jobslock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1101 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1101 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1101 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1101 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1101 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1101 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn job_stalled_check() {
    let mut jp: *mut jobpool = ::core::ptr::null_mut::<jobpool>();
    let mut j: *mut job = ::core::ptr::null_mut::<job>();
    let mut i: uint32_t = 0;
    let mut h: uint32_t = 0;
    let mut t: uint64_t = 0;
    i = 0 as uint32_t;
    while i < 2 as uint32_t {
        jp = if i == 0 as uint32_t { hp_pool } else { lp_pool };
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*jp).jobslock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1114 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1114 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1114 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1114 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1114 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1114 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        t = monotonic_useconds();
        h = 0 as uint32_t;
        while h < JHASHSIZE as uint32_t {
            j = (*jp).jobhash[h as usize];
            while !j.is_null() {
                if (*j).tasktype as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                    && (*j).starttime > 0 as uint64_t
                {
                    if t > (*j).starttime && t.wrapping_sub((*j).starttime) > 600000000 as uint64_t
                    {
                        if (*j).chunkid != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"stalled job '%s' on chunk %016lX detected\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                jobtype_str[((*j).tasktype as ::core::ffi::c_int
                                    & 0x7 as ::core::ffi::c_int)
                                    as usize],
                                (*j).chunkid,
                            );
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"stalled job '%s' detected\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                jobtype_str[((*j).tasktype as ::core::ffi::c_int
                                    & 0x7 as ::core::ffi::c_int)
                                    as usize],
                            );
                        }
                        (*j).tasktype = ((*j).tasktype as ::core::ffi::c_int
                            | 0x40 as ::core::ffi::c_int)
                            as uint8_t;
                    }
                }
                j = (*j).next as *mut job;
            }
            h = h.wrapping_add(1);
        }
        let mut _mfs_assert_ret_0: ::core::ffi::c_int =
            pthread_mutex_unlock(&raw mut (*jp).jobslock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1130 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1130 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1130 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1130 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1130 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1130 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn job_canexit() -> ::core::ffi::c_int {
    return if job_pool_jobs_count(hp_pool).wrapping_add(job_pool_jobs_count(lp_pool))
        > 0 as uint32_t
    {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn job_term() {
    job_pool_delete(hp_pool);
    job_pool_delete(lp_pool);
}
#[no_mangle]
pub unsafe extern "C" fn job_reload() {
    let mut jp: *mut jobpool = ::core::ptr::null_mut::<jobpool>();
    let mut wm: uint32_t = 0;
    let mut whm: uint32_t = 0;
    let mut wlm: uint32_t = 0;
    let mut wmi: uint32_t = 0;
    wm = cfg_getuint32(
        b"WORKERS_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        250 as uint32_t,
    );
    if cfg_isdefined(b"WORKERS_HLOAD_HIMARK\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
        whm = cfg_getuint32(
            b"WORKERS_HLOAD_HIMARK\0".as_ptr() as *const ::core::ffi::c_char,
            187 as uint32_t,
        );
    } else {
        whm = wm.wrapping_mul(3 as uint32_t).wrapping_div(4 as uint32_t);
    }
    if cfg_isdefined(b"WORKERS_HLOAD_LOMARK\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
        wlm = cfg_getuint32(
            b"WORKERS_HLOAD_LOMARK\0".as_ptr() as *const ::core::ffi::c_char,
            125 as uint32_t,
        );
    } else {
        wlm = wm.wrapping_mul(2 as uint32_t).wrapping_div(4 as uint32_t);
    }
    wmi = cfg_getuint32(
        b"WORKERS_MAX_IDLE\0".as_ptr() as *const ::core::ffi::c_char,
        40 as uint32_t,
    );
    if whm >= wm {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"WORKERS_HLOAD_HIMARK >= WORKERS_MAX - it doesn't make sense - setting WORKERS_HLOAD_HIMARK to WORKERS_MAX * 3/4\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        whm = wm.wrapping_mul(3 as uint32_t).wrapping_div(4 as uint32_t);
    }
    if wlm >= wm {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"WORKERS_HLOAD_LOMARK >= WORKERS_MAX - it doesn't make sense - setting WORKERS_HLOAD_LOMARK to WORKERS_MAX * 1/2\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        wlm = wm.wrapping_mul(2 as uint32_t).wrapping_div(4 as uint32_t);
    } else if wlm >= whm {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"WORKERS_HLOAD_LOMARK >= WORKERS_HLOAD_HIMARK - it doesn't make sense - setting WORKERS_HLOAD_LOMARK to WORKERS_HLOAD_HIMARK * 2/3\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        wlm = whm.wrapping_mul(2 as uint32_t).wrapping_div(3 as uint32_t);
    }
    jp = hp_pool;
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*jp).jobslock);
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
        } else {
            let mut _mfs_errorstring_err: *const ::core::ffi::c_char = strerr(*__errno_location());
            let mut _mfs_errorstring_ret: *const ::core::ffi::c_char = strerr(_mfs_assert_ret);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    (*jp).workers_max = wm;
    (*jp).workers_himark = whm;
    (*jp).workers_lomark = wlm;
    (*jp).workers_max_idle = wmi;
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*jp).jobslock);
    if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_1: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
        } else {
            let mut _mfs_errorstring_err_0: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_0: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_0);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
    jp = lp_pool;
    let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*jp).jobslock);
    if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_3: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1185 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1185 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_3,
            );
        } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
            && *__errno_location() == 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_4: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1185 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1185 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
        } else {
            let mut _mfs_errorstring_err_1: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_1: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_1);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1185 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1185 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
        }
        abort();
    }
    (*jp).workers_max = wm;
    (*jp).workers_himark = whm;
    (*jp).workers_lomark = wlm;
    (*jp).workers_max_idle = wmi;
    let mut _mfs_assert_ret_2: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*jp).jobslock);
    if _mfs_assert_ret_2 != 0 as ::core::ffi::c_int {
        if _mfs_assert_ret_2 < 0 as ::core::ffi::c_int
            && *__errno_location() != 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_5: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1192 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                *__errno_location(),
                _mfs_errorstring_5,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1192 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                *__errno_location(),
                _mfs_errorstring_5,
            );
        } else if _mfs_assert_ret_2 > 0 as ::core::ffi::c_int
            && *__errno_location() == 0 as ::core::ffi::c_int
        {
            let mut _mfs_errorstring_6: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_2);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1192 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_6,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1192 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_6,
            );
        } else {
            let mut _mfs_errorstring_err_2: *const ::core::ffi::c_char =
                strerr(*__errno_location());
            let mut _mfs_errorstring_ret_2: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_2);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1192 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_ret_2,
                *__errno_location(),
                _mfs_errorstring_err_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/bgjobs.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1192 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(jp->jobslock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_ret_2,
                *__errno_location(),
                _mfs_errorstring_err_2,
            );
        }
        abort();
    }
}
#[no_mangle]
pub unsafe extern "C" fn job_init() -> ::core::ffi::c_int {
    hp_pool = job_pool_new(Some(
        job_hp_worker as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
    )) as *mut jobpool;
    lp_pool = job_pool_new(Some(
        job_lp_worker as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
    )) as *mut jobpool;
    if hp_pool.is_null() || lp_pool.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    job_reload();
    main_destruct_register_fname(
        Some(job_term as unsafe extern "C" fn() -> ()),
        b"job_term\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_canexit_register_fname(
        Some(job_canexit as unsafe extern "C" fn() -> ::core::ffi::c_int),
        b"job_canexit\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_reload_register_fname(
        Some(job_reload as unsafe extern "C" fn() -> ()),
        b"job_reload\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_eachloop_register_fname(
        Some(job_heavyload_test as unsafe extern "C" fn() -> ()),
        b"job_heavyload_test\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_poll_register_fname(
        Some(job_desc as unsafe extern "C" fn(*mut pollfd, *mut uint32_t) -> ()),
        Some(job_serve as unsafe extern "C" fn(*mut pollfd) -> ()),
        b"job_desc\0".as_ptr() as *const ::core::ffi::c_char,
        b"job_serve\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_info_register_fname(
        Some(job_info as unsafe extern "C" fn(*mut FILE) -> ()),
        b"job_info\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_time_register_fname(
        60 as uint32_t,
        0 as uint32_t,
        Some(job_counters_shift as unsafe extern "C" fn() -> ()),
        b"job_counters_shift\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_time_register_fname(
        10 as uint32_t,
        0 as uint32_t,
        Some(job_stalled_check as unsafe extern "C" fn() -> ()),
        b"job_stalled_check\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
