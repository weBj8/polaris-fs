pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    unsafe fn __sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    unsafe fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_detach(__th: pthread_t) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_init(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    unsafe fn pthread_attr_setstacksize(
        __attr: *mut pthread_attr_t,
        __stacksize: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> ::core::ffi::c_int;
    unsafe fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    unsafe fn sigaddset(__set: *mut sigset_t, __signo: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn pthread_sigmask(
        __how: ::core::ffi::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> ::core::ffi::c_int;
    unsafe fn squeue_new(length: uint32_t) -> *mut ::core::ffi::c_void;
    unsafe fn squeue_delete(que: *mut ::core::ffi::c_void);
    unsafe fn squeue_close(que: *mut ::core::ffi::c_void);
    unsafe fn squeue_put(que: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void);
    unsafe fn squeue_get(que: *mut ::core::ffi::c_void, element: *mut *mut ::core::ffi::c_void);
    static mut stderr: *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
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
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
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
pub type uint32_t = u32;
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
pub struct _workers {
    pub sustainworkers: uint32_t,
    pub maxworkers: uint32_t,
    pub jqueue: *mut ::core::ffi::c_void,
    pub name: *mut ::core::ffi::c_char,
    pub workerfn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> ()>,
    pub lock: pthread_mutex_t,
    pub term_cond: pthread_cond_t,
    pub thattr: pthread_attr_t,
    pub avail: uint32_t,
    pub total: uint32_t,
    pub lastnotify: uint32_t,
}
pub type workers = _workers;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _worker {
    pub ws: *mut workers,
    pub thread_id: pthread_t,
}
pub type worker = _worker;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __SC_THREAD_STACK_MIN_VALUE: ::core::ffi::c_int = 75 as ::core::ffi::c_int;
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SIGHUP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIGQUIT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SIG_BLOCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SIG_SETMASK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn workers_spawn_worker(mut ws: *mut workers) {
    unsafe {
        let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
        let mut newset: sigset_t = sigset_t { __val: [0; 16] };
        let mut w: *mut worker = ::core::ptr::null_mut::<worker>();
        let mut res: ::core::ffi::c_int = 0;
        w = malloc(::core::mem::size_of::<worker>()) as *mut worker;
        if w.is_null() {
            return;
        }
        (*w).ws = ws;
        sigemptyset(&raw mut newset);
        sigaddset(&raw mut newset, SIGTERM);
        sigaddset(&raw mut newset, SIGINT);
        sigaddset(&raw mut newset, SIGHUP);
        sigaddset(&raw mut newset, SIGQUIT);
        pthread_sigmask(SIG_BLOCK, &raw mut newset, &raw mut oldset);
        res = pthread_create(
            &raw mut (*w).thread_id,
            &raw mut (*ws).thattr,
            Some(
                workers_worker_thread
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
            ),
            w as *mut ::core::ffi::c_void,
        );
        pthread_sigmask(
            SIG_SETMASK,
            &raw mut oldset,
            ::core::ptr::null_mut::<__sigset_t>(),
        );
        if res < 0 as ::core::ffi::c_int {
            return;
        }
        (*ws).avail = (*ws).avail.wrapping_add(1);
        (*ws).total = (*ws).total.wrapping_add(1);
        if (*ws).total.wrapping_rem(10 as uint32_t) == 0 as uint32_t
            && (*ws).total != (*ws).lastnotify
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_INFO,
                b"%s workers: %u+\0".as_ptr() as *const ::core::ffi::c_char,
                (*ws).name,
                (*ws).total,
            );
            (*ws).lastnotify = (*ws).total;
        }
    }
}
#[inline]
unsafe extern "C" fn workers_close_worker(mut w: *mut worker) {
    unsafe {
        let mut ws: *mut workers = (*w).ws;
        (*ws).avail = (*ws).avail.wrapping_sub(1);
        (*ws).total = (*ws).total.wrapping_sub(1);
        if (*ws).total.wrapping_rem(10 as uint32_t) == 0 as uint32_t
            && (*ws).total != (*ws).lastnotify
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_INFO,
                b"%s workers: %u-\0".as_ptr() as *const ::core::ffi::c_char,
                (*ws).name,
                (*ws).total,
            );
            (*ws).lastnotify = (*ws).total;
        }
        if (*ws).total == 0 as uint32_t {
            let mut _mfs_assert_ret: ::core::ffi::c_int =
                pthread_cond_signal(&raw mut (*ws).term_cond);
            if _mfs_assert_ret != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        102 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&(ws->term_cond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        *__errno_location(),
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        102 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&(ws->term_cond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        *__errno_location(),
                        _mfs_errorstring,
                    );
                } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        102 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&(ws->term_cond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        102 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&(ws->term_cond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_0,
                    );
                } else {
                    let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    let mut _mfs_errorstring_ret: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        102 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&(ws->term_cond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_ret,
                        *__errno_location(),
                        _mfs_errorstring_err,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        102 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&(ws->term_cond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_ret,
                        *__errno_location(),
                        _mfs_errorstring_err,
                    );
                }
                abort();
            }
        }
        pthread_detach((*w).thread_id);
        free(w as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn workers_worker_thread(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut w: *mut worker = arg as *mut worker;
        let mut ws: *mut workers = (*w).ws;
        let mut current_workers: uint32_t = 0;
        let mut data: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut firstrun: uint8_t = 1 as uint8_t;
        loop {
            if firstrun as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                let mut _mfs_assert_ret: ::core::ffi::c_int =
                    pthread_mutex_lock(&raw mut (*ws).lock);
                if _mfs_assert_ret != 0 as ::core::ffi::c_int {
                    if _mfs_assert_ret < 0 as ::core::ffi::c_int
                        && *__errno_location() != 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            118 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_lock(&(ws->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret,
                            *__errno_location(),
                            _mfs_errorstring,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            118 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_lock(&(ws->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret,
                            *__errno_location(),
                            _mfs_errorstring,
                        );
                    } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                        && *__errno_location() == 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            118 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_lock(&(ws->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret,
                            _mfs_errorstring_0,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            118 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_lock(&(ws->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret,
                            _mfs_errorstring_0,
                        );
                    } else {
                        let mut _mfs_errorstring_err: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        let mut _mfs_errorstring_ret: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            118 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_lock(&(ws->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret,
                            _mfs_errorstring_ret,
                            *__errno_location(),
                            _mfs_errorstring_err,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            118 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_lock(&(ws->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret,
                            _mfs_errorstring_ret,
                            *__errno_location(),
                            _mfs_errorstring_err,
                        );
                    }
                    abort();
                }
                (*ws).avail = (*ws).avail.wrapping_add(1);
                if (*ws).avail > (*ws).sustainworkers {
                    workers_close_worker(w);
                    let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                        pthread_mutex_unlock(&raw mut (*ws).lock);
                    if _mfs_assert_ret_0 != 0 as ::core::ffi::c_int {
                        if _mfs_assert_ret_0 < 0 as ::core::ffi::c_int
                            && *__errno_location() != 0 as ::core::ffi::c_int
                        {
                            let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                122 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_0,
                                *__errno_location(),
                                _mfs_errorstring_1,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                122 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr()
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
                                b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                122 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_0,
                                _mfs_errorstring_2,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                122 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr()
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
                                b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                122 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr()
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
                                b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                122 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr()
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
                let mut _mfs_assert_ret_1: ::core::ffi::c_int =
                    pthread_mutex_unlock(&raw mut (*ws).lock);
                if _mfs_assert_ret_1 != 0 as ::core::ffi::c_int {
                    if _mfs_assert_ret_1 < 0 as ::core::ffi::c_int
                        && *__errno_location() != 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            125 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_1,
                            *__errno_location(),
                            _mfs_errorstring_3,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            125 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_1,
                            *__errno_location(),
                            _mfs_errorstring_3,
                        );
                    } else if _mfs_assert_ret_1 > 0 as ::core::ffi::c_int
                        && *__errno_location() == 0 as ::core::ffi::c_int
                    {
                        let mut _mfs_errorstring_4: *const ::core::ffi::c_char =
                            strerr(_mfs_assert_ret_1);
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            125 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_1,
                            _mfs_errorstring_4,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            125 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr()
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
                            b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            125 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            125 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_1,
                            _mfs_errorstring_ret_1,
                            *__errno_location(),
                            _mfs_errorstring_err_1,
                        );
                    }
                    abort();
                }
            }
            firstrun = 0 as uint8_t;
            squeue_get((*ws).jqueue, &raw mut data);
            let mut _mfs_assert_ret_2: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*ws).lock);
            if _mfs_assert_ret_2 != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret_2 < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_5: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        133 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        *__errno_location(),
                        _mfs_errorstring_5,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        133 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        133 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        _mfs_errorstring_6,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        133 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        133 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        _mfs_errorstring_ret_2,
                        *__errno_location(),
                        _mfs_errorstring_err_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        133 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        _mfs_errorstring_ret_2,
                        *__errno_location(),
                        _mfs_errorstring_err_2,
                    );
                }
                abort();
            }
            if data.is_null() {
                workers_close_worker(w);
                let mut _mfs_assert_ret_3: ::core::ffi::c_int =
                    pthread_mutex_unlock(&raw mut (*ws).lock);
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
                            b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            137 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_3,
                            *__errno_location(),
                            _mfs_errorstring_7,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            137 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            137 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_3,
                            _mfs_errorstring_8,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            137 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            137 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            137 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr()
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
            (*ws).avail = (*ws).avail.wrapping_sub(1);
            if (*ws).avail == 0 as uint32_t && (*ws).total < (*ws).maxworkers {
                workers_spawn_worker(ws);
            }
            current_workers = (*ws).total;
            let mut _mfs_assert_ret_4: ::core::ffi::c_int =
                pthread_mutex_unlock(&raw mut (*ws).lock);
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
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        146 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_4,
                        *__errno_location(),
                        _mfs_errorstring_9,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        146 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_4,
                        *__errno_location(),
                        _mfs_errorstring_9,
                    );
                } else if _mfs_assert_ret_4 > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_10: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_4);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        146 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_4,
                        _mfs_errorstring_10,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        146 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
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
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        146 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_4,
                        _mfs_errorstring_ret_4,
                        *__errno_location(),
                        _mfs_errorstring_err_4,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        146 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_4,
                        _mfs_errorstring_ret_4,
                        *__errno_location(),
                        _mfs_errorstring_err_4,
                    );
                }
                abort();
            }
            (*ws).workerfn.expect("non-null function pointer")(data, current_workers);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn workers_init(
    mut maxworkers: uint32_t,
    mut sustainworkers: uint32_t,
    mut qleng: uint32_t,
    mut name: *mut ::core::ffi::c_char,
    mut workerfn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> ()>,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut ws: *mut workers = ::core::ptr::null_mut::<workers>();
        let mut mystacksize: size_t = 0;
        ws = malloc(::core::mem::size_of::<workers>()) as *mut workers;
        if ws.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                157 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"ws\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                157 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"ws\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if ws
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut workers
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                157 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"ws\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                157 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"ws\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*ws).sustainworkers = sustainworkers;
        (*ws).maxworkers = maxworkers;
        (*ws).jqueue = squeue_new(qleng);
        (*ws).name = strdup(name);
        (*ws).workerfn = workerfn;
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_init(
            &raw mut (*ws).lock,
            ::core::ptr::null::<pthread_mutexattr_t>(),
        );
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    163 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&(ws->lock),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    163 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&(ws->lock),NULL)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    163 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&(ws->lock),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    163 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&(ws->lock),NULL)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    163 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&(ws->lock),NULL)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    163 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&(ws->lock),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_cond_init(
            &raw mut (*ws).term_cond,
            ::core::ptr::null::<pthread_condattr_t>(),
        );
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    164 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(ws->term_cond),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    164 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(ws->term_cond),NULL)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    164 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(ws->term_cond),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    164 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(ws->term_cond),NULL)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    164 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(ws->term_cond),NULL)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    164 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(ws->term_cond),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_attr_init(&raw mut (*ws).thattr);
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    165 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_init(&(ws->thattr))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    165 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_init(&(ws->thattr))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    165 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_init(&(ws->thattr))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_5,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    165 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_init(&(ws->thattr))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    165 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_init(&(ws->thattr))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    165 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_init(&(ws->thattr))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
            }
            abort();
        }
        mystacksize = __sysconf(__SC_THREAD_STACK_MIN_VALUE) as size_t;
        if mystacksize < 0x20000 as ::core::ffi::c_int as size_t {
            mystacksize = 0x20000 as ::core::ffi::c_int as size_t;
        }
        let mut _mfs_assert_ret_2: ::core::ffi::c_int =
            pthread_attr_setstacksize(&raw mut (*ws).thattr, mystacksize);
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_setstacksize(&(ws->thattr),mystacksize)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_6,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_setstacksize(&(ws->thattr),mystacksize)\0".as_ptr()
                        as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_setstacksize(&(ws->thattr),mystacksize)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_7,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_setstacksize(&(ws->thattr),mystacksize)\0".as_ptr()
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
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_setstacksize(&(ws->thattr),mystacksize)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_setstacksize(&(ws->thattr),mystacksize)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_3: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*ws).lock);
        if _mfs_assert_ret_3 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_3 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_8: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    175 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_8,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    175 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    175 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_9,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    175 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_9,
                );
            } else {
                let mut _mfs_errorstring_err_3: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                let mut _mfs_errorstring_ret_3: *const ::core::ffi::c_char =
                    strerr(_mfs_assert_ret_3);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    175 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_err_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    175 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_err_3,
                );
            }
            abort();
        }
        (*ws).avail = 0 as uint32_t;
        (*ws).total = 0 as uint32_t;
        (*ws).lastnotify = 0 as uint32_t;
        workers_spawn_worker(ws);
        let mut _mfs_assert_ret_4: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*ws).lock);
        if _mfs_assert_ret_4 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_4 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_10: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    180 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_10,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    180 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_10,
                );
            } else if _mfs_assert_ret_4 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_11: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_4);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    180 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_11,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    180 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_11,
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    180 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_err_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    180 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_err_4,
                );
            }
            abort();
        }
        return ws as *mut ::core::ffi::c_void;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn workers_term(mut wsv: *mut ::core::ffi::c_void) {
    unsafe {
        let mut ws: *mut workers = wsv as *mut workers;
        squeue_close((*ws).jqueue);
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*ws).lock);
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    189 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    189 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    189 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    189 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    189 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    189 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        while (*ws).total > 0 as uint32_t {
            let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                pthread_cond_wait(&raw mut (*ws).term_cond, &raw mut (*ws).lock);
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
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        191 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(ws->term_cond),&(ws->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        191 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(ws->term_cond),&(ws->lock))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        191 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(ws->term_cond),&(ws->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        191 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(ws->term_cond),&(ws->lock))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        191 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(ws->term_cond),&(ws->lock))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        191 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(ws->term_cond),&(ws->lock))\0".as_ptr()
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
        let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*ws).lock);
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
            }
            abort();
        }
        squeue_delete((*ws).jqueue);
        let mut _mfs_assert_ret_2: ::core::ffi::c_int = pthread_attr_destroy(&raw mut (*ws).thattr);
        if _mfs_assert_ret_2 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_2 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_5: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    195 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_destroy(&(ws->thattr))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_5,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    195 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_destroy(&(ws->thattr))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    195 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_destroy(&(ws->thattr))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_6,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    195 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_destroy(&(ws->thattr))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    195 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_destroy(&(ws->thattr))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    195 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_destroy(&(ws->thattr))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_3: ::core::ffi::c_int =
            pthread_cond_destroy(&raw mut (*ws).term_cond);
        if _mfs_assert_ret_3 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_3 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_7: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    196 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&(ws->term_cond))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_7,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    196 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&(ws->term_cond))\0".as_ptr()
                        as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    196 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&(ws->term_cond))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_8,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    196 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&(ws->term_cond))\0".as_ptr()
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
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    196 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&(ws->term_cond))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_err_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    196 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&(ws->term_cond))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_err_3,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_4: ::core::ffi::c_int = pthread_mutex_destroy(&raw mut (*ws).lock);
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    197 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_9,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    197 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    197 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_10,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    197 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    197 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_err_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/workers.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    197 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&(ws->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_err_4,
                );
            }
            abort();
        }
        free((*ws).name as *mut ::core::ffi::c_void);
        free(ws as *mut ::core::ffi::c_void);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn workers_newjob(
    mut wsv: *mut ::core::ffi::c_void,
    mut data: *mut ::core::ffi::c_void,
) {
    unsafe {
        let mut ws: *mut workers = wsv as *mut workers;
        squeue_put((*ws).jqueue, data);
    }
}
