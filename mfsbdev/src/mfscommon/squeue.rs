use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
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
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
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
    fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
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
pub struct _qentry {
    pub data: *mut ::core::ffi::c_void,
    pub next: *mut _qentry,
}
pub type qentry = _qentry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _queue {
    pub head: *mut qentry,
    pub tail: *mut *mut qentry,
    pub elements: uint32_t,
    pub maxelements: uint32_t,
    pub closed: uint32_t,
    pub waitfree: pthread_cond_t,
    pub waitfull: pthread_cond_t,
    pub lock: pthread_mutex_t,
}
pub type queue = _queue;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EIO: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const EBUSY: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn squeue_new(mut length: uint32_t) -> *mut ::core::ffi::c_void {
    let mut q: *mut queue = ::core::ptr::null_mut::<queue>();
    q = malloc(::core::mem::size_of::<queue>()) as *mut queue;
    if q.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            48 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"q\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            48 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"q\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if q
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut queue
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            48 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"q\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            48 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"q\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    (*q).head = ::core::ptr::null_mut::<qentry>();
    (*q).tail = &raw mut (*q).head;
    (*q).elements = 0 as uint32_t;
    (*q).maxelements = length;
    (*q).closed = 0 as uint32_t;
    if length != 0 {
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_cond_init(
            &raw mut (*q).waitfull,
            ::core::ptr::null::<pthread_condattr_t>(),
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    55 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(q->waitfull),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    55 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(q->waitfull),NULL)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    55 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(q->waitfull),NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    55 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(q->waitfull),NULL)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    55 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(q->waitfull),NULL)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    55 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&(q->waitfull),NULL)\0".as_ptr()
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
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_cond_init(
        &raw mut (*q).waitfree,
        ::core::ptr::null::<pthread_condattr_t>(),
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                57 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_init(&(q->waitfree),NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                57 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_init(&(q->waitfree),NULL)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                57 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_init(&(q->waitfree),NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                57 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_init(&(q->waitfree),NULL)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                57 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_init(&(q->waitfree),NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                57 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_init(&(q->waitfree),NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
    let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_init(
        &raw mut (*q).lock,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                58 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(q->lock),NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                58 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(q->lock),NULL)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                58 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(q->lock),NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_5,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                58 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(q->lock),NULL)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                58 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(q->lock),NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                58 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(q->lock),NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
        }
        abort();
    }
    return q as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn squeue_delete(mut que: *mut ::core::ffi::c_void) {
    let mut q: *mut queue = que as *mut queue;
    let mut qe: *mut qentry = ::core::ptr::null_mut::<qentry>();
    let mut qen: *mut qentry = ::core::ptr::null_mut::<qentry>();
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*q).lock);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                65 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                65 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                65 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                65 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                65 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                65 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    qe = (*q).head;
    while !qe.is_null() {
        qen = (*qe).next as *mut qentry;
        free((*qe).data);
        free(qe as *mut ::core::ffi::c_void);
        qe = qen;
    }
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*q).lock);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                71 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                71 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                71 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                71 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                71 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                71 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
    let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_destroy(&raw mut (*q).lock);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                72 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                72 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                72 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                72 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                72 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                72 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
        }
        abort();
    }
    let mut _mfs_assert_ret_2: ::core::ffi::c_int = pthread_cond_destroy(&raw mut (*q).waitfree);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                73 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_destroy(&(q->waitfree))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                *__errno_location(),
                _mfs_errorstring_5,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                73 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_destroy(&(q->waitfree))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                73 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_destroy(&(q->waitfree))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_6,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                73 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_destroy(&(q->waitfree))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                73 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_destroy(&(q->waitfree))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_ret_2,
                *__errno_location(),
                _mfs_errorstring_err_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                73 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_destroy(&(q->waitfree))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_ret_2,
                *__errno_location(),
                _mfs_errorstring_err_2,
            );
        }
        abort();
    }
    if (*q).maxelements != 0 {
        let mut _mfs_assert_ret_3: ::core::ffi::c_int =
            pthread_cond_destroy(&raw mut (*q).waitfull);
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    75 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&(q->waitfull))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_7,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    75 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&(q->waitfull))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    75 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&(q->waitfull))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_8,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    75 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&(q->waitfull))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    75 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&(q->waitfull))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    75 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&(q->waitfull))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_err_3,
                );
            }
            abort();
        }
    }
    free(q as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn squeue_close(mut que: *mut ::core::ffi::c_void) {
    let mut q: *mut queue = que as *mut queue;
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*q).lock);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                82 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                82 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                82 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                82 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                82 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                82 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    (*q).closed = 1 as uint32_t;
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_cond_broadcast(&raw mut (*q).waitfree);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                84 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_broadcast(&(q->waitfree))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                84 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_broadcast(&(q->waitfree))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                84 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_broadcast(&(q->waitfree))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                84 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_broadcast(&(q->waitfree))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                84 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_broadcast(&(q->waitfree))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                84 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_broadcast(&(q->waitfree))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
    if (*q).maxelements != 0 {
        let mut _mfs_assert_ret_1: ::core::ffi::c_int =
            pthread_cond_broadcast(&raw mut (*q).waitfull);
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    86 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_broadcast(&(q->waitfull))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    86 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_broadcast(&(q->waitfull))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    86 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_broadcast(&(q->waitfull))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    86 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_broadcast(&(q->waitfull))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    86 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_broadcast(&(q->waitfull))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    86 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_broadcast(&(q->waitfull))\0".as_ptr()
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
    let mut _mfs_assert_ret_2: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*q).lock);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                88 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                *__errno_location(),
                _mfs_errorstring_5,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                88 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                88 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_6,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                88 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                88 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_ret_2,
                *__errno_location(),
                _mfs_errorstring_err_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                88 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn squeue_isempty(mut que: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut q: *mut queue = que as *mut queue;
    let mut r: ::core::ffi::c_int = 0;
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*q).lock);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                94 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                94 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                94 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                94 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                94 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                94 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    r = if (*q).elements == 0 as uint32_t {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*q).lock);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn squeue_elements(mut que: *mut ::core::ffi::c_void) -> uint32_t {
    let mut q: *mut queue = que as *mut queue;
    let mut r: uint32_t = 0;
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*q).lock);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                103 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                103 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                103 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                103 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                103 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                103 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    r = (*q).elements;
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*q).lock);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                105 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                105 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                105 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                105 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                105 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                105 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn squeue_isfull(mut que: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut q: *mut queue = que as *mut queue;
    let mut r: ::core::ffi::c_int = 0;
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*q).lock);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                112 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                112 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                112 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                112 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                112 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                112 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    r = if (*q).maxelements > 0 as uint32_t && (*q).maxelements <= (*q).elements {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*q).lock);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                114 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                114 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                114 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                114 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                114 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                114 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn squeue_sizeleft(mut que: *mut ::core::ffi::c_void) -> uint32_t {
    let mut q: *mut queue = que as *mut queue;
    let mut r: uint32_t = 0;
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*q).lock);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                121 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                121 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                121 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                121 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                121 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                121 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    if (*q).maxelements > 0 as uint32_t {
        r = (*q).maxelements.wrapping_sub((*q).elements);
    } else {
        r = 0xffffffff as ::core::ffi::c_uint as uint32_t;
    }
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*q).lock);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                127 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                127 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                127 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                127 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                127 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                127 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn squeue_put(
    mut que: *mut ::core::ffi::c_void,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut q: *mut queue = que as *mut queue;
    let mut qe: *mut qentry = ::core::ptr::null_mut::<qentry>();
    qe = malloc(::core::mem::size_of::<qentry>()) as *mut qentry;
    if qe.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            135 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"qe\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            135 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"qe\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if qe
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut qentry
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            135 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"qe\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            135 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"qe\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    (*qe).data = data;
    (*qe).next = ::core::ptr::null_mut::<_qentry>();
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*q).lock);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                138 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                138 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                138 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                138 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                138 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                138 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    if (*q).maxelements != 0 {
        while (*q).elements >= (*q).maxelements && (*q).closed == 0 as uint32_t {
            let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                pthread_cond_wait(&raw mut (*q).waitfull, &raw mut (*q).lock);
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
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        141 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(q->waitfull),&(q->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        141 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(q->waitfull),&(q->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_2,
                    );
                } else if _mfs_assert_ret_0 > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret_0);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        141 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(q->waitfull),&(q->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_3,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        141 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(q->waitfull),&(q->lock))\0".as_ptr()
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
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        141 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(q->waitfull),&(q->lock))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        141 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(q->waitfull),&(q->lock))\0".as_ptr()
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
        if (*q).closed != 0 {
            let mut _mfs_assert_ret_1: ::core::ffi::c_int =
                pthread_mutex_unlock(&raw mut (*q).lock);
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
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        144 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(q->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_1,
                        *__errno_location(),
                        _mfs_errorstring_4,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        144 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(q->lock))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        144 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(q->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_1,
                        _mfs_errorstring_5,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        144 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(q->lock))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        144 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(q->lock))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        144 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(q->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_1,
                        _mfs_errorstring_ret_1,
                        *__errno_location(),
                        _mfs_errorstring_err_1,
                    );
                }
                abort();
            }
            free(qe as *mut ::core::ffi::c_void);
            *__errno_location() = EIO;
            return -1 as ::core::ffi::c_int;
        }
    }
    (*q).elements = (*q).elements.wrapping_add(1);
    *(*q).tail = qe;
    (*q).tail = &raw mut (*qe).next as *mut *mut qentry;
    let mut _mfs_assert_ret_2: ::core::ffi::c_int = pthread_cond_signal(&raw mut (*q).waitfree);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                153 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_signal(&(q->waitfree))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                *__errno_location(),
                _mfs_errorstring_6,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                153 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_signal(&(q->waitfree))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                153 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_signal(&(q->waitfree))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_7,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                153 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_signal(&(q->waitfree))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                153 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_signal(&(q->waitfree))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_ret_2,
                *__errno_location(),
                _mfs_errorstring_err_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                153 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_signal(&(q->waitfree))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_ret_2,
                *__errno_location(),
                _mfs_errorstring_err_2,
            );
        }
        abort();
    }
    let mut _mfs_assert_ret_3: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*q).lock);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                154 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                *__errno_location(),
                _mfs_errorstring_8,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                154 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                154 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                _mfs_errorstring_9,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                154 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                154 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                _mfs_errorstring_ret_3,
                *__errno_location(),
                _mfs_errorstring_err_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                154 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                _mfs_errorstring_ret_3,
                *__errno_location(),
                _mfs_errorstring_err_3,
            );
        }
        abort();
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn squeue_tryput(
    mut que: *mut ::core::ffi::c_void,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut q: *mut queue = que as *mut queue;
    let mut qe: *mut qentry = ::core::ptr::null_mut::<qentry>();
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*q).lock);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                161 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                161 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                161 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                161 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                161 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                161 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    if (*q).maxelements != 0 {
        if (*q).elements >= (*q).maxelements {
            let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                pthread_mutex_unlock(&raw mut (*q).lock);
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
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        164 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(q->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        164 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(q->lock))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        164 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(q->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        164 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(q->lock))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        164 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(q->lock))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        164 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&(q->lock))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                }
                abort();
            }
            *__errno_location() = EBUSY;
            return -1 as ::core::ffi::c_int;
        }
    }
    qe = malloc(::core::mem::size_of::<qentry>()) as *mut qentry;
    if qe.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            170 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"qe\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            170 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"qe\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if qe
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut qentry
    {
        let mut _mfs_errorstring_3: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            170 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"qe\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring_3,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            170 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"qe\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring_3,
        );
        abort();
    }
    (*qe).data = data;
    (*qe).next = ::core::ptr::null_mut::<_qentry>();
    (*q).elements = (*q).elements.wrapping_add(1);
    *(*q).tail = qe;
    (*q).tail = &raw mut (*qe).next as *mut *mut qentry;
    let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_cond_signal(&raw mut (*q).waitfree);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                176 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_signal(&(q->waitfree))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                176 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_signal(&(q->waitfree))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                176 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_signal(&(q->waitfree))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_5,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                176 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_signal(&(q->waitfree))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                176 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_signal(&(q->waitfree))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                176 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_signal(&(q->waitfree))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
        }
        abort();
    }
    let mut _mfs_assert_ret_2: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*q).lock);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                177 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                *__errno_location(),
                _mfs_errorstring_6,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                177 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                177 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_7,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                177 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                177 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_ret_2,
                *__errno_location(),
                _mfs_errorstring_err_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                177 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_ret_2,
                *__errno_location(),
                _mfs_errorstring_err_2,
            );
        }
        abort();
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn squeue_get(
    mut que: *mut ::core::ffi::c_void,
    mut data: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut q: *mut queue = que as *mut queue;
    let mut qe: *mut qentry = ::core::ptr::null_mut::<qentry>();
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*q).lock);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                184 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                184 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                184 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                184 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                184 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                184 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    while (*q).elements == 0 as uint32_t && (*q).closed == 0 as uint32_t {
        let mut _mfs_assert_ret_0: ::core::ffi::c_int =
            pthread_cond_wait(&raw mut (*q).waitfree, &raw mut (*q).lock);
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    186 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(q->waitfree),&(q->lock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    186 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(q->waitfree),&(q->lock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    186 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(q->waitfree),&(q->lock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    186 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(q->waitfree),&(q->lock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    186 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(q->waitfree),&(q->lock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    186 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(q->waitfree),&(q->lock))\0".as_ptr()
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
    if (*q).closed != 0 {
        let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*q).lock);
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    189 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    189 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    189 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    189 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    189 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    189 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
            }
            abort();
        }
        if !data.is_null() {
            *data = NULL;
        }
        *__errno_location() = EIO;
        return -1 as ::core::ffi::c_int;
    }
    qe = (*q).head;
    (*q).head = (*qe).next as *mut qentry;
    if (*q).head.is_null() {
        (*q).tail = &raw mut (*q).head;
    }
    (*q).elements = (*q).elements.wrapping_sub(1);
    if (*q).maxelements != 0 {
        let mut _mfs_assert_ret_2: ::core::ffi::c_int = pthread_cond_signal(&raw mut (*q).waitfull);
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    203 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_signal(&(q->waitfull))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_5,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    203 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_signal(&(q->waitfull))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    203 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_signal(&(q->waitfull))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_6,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    203 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_signal(&(q->waitfull))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    203 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_signal(&(q->waitfull))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    203 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_signal(&(q->waitfull))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
            }
            abort();
        }
    }
    let mut _mfs_assert_ret_3: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*q).lock);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                205 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                *__errno_location(),
                _mfs_errorstring_7,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                205 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                205 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                _mfs_errorstring_8,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                205 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                205 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                _mfs_errorstring_ret_3,
                *__errno_location(),
                _mfs_errorstring_err_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                205 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                _mfs_errorstring_ret_3,
                *__errno_location(),
                _mfs_errorstring_err_3,
            );
        }
        abort();
    }
    if !data.is_null() {
        *data = (*qe).data;
    }
    free(qe as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn squeue_tryget(
    mut que: *mut ::core::ffi::c_void,
    mut data: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut q: *mut queue = que as *mut queue;
    let mut qe: *mut qentry = ::core::ptr::null_mut::<qentry>();
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*q).lock);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                216 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                216 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                216 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                216 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                216 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                216 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    if (*q).elements == 0 as uint32_t {
        let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*q).lock);
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    218 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    218 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    218 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    218 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    218 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    218 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
        if !data.is_null() {
            *data = NULL;
        }
        *__errno_location() = EBUSY;
        return -1 as ::core::ffi::c_int;
    }
    qe = (*q).head;
    (*q).head = (*qe).next as *mut qentry;
    if (*q).head.is_null() {
        (*q).tail = &raw mut (*q).head;
    }
    (*q).elements = (*q).elements.wrapping_sub(1);
    if (*q).maxelements != 0 {
        let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_cond_signal(&raw mut (*q).waitfull);
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    232 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_signal(&(q->waitfull))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    232 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_signal(&(q->waitfull))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    232 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_signal(&(q->waitfull))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    232 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_signal(&(q->waitfull))\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    232 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_signal(&(q->waitfull))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    232 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_signal(&(q->waitfull))\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
            }
            abort();
        }
    }
    let mut _mfs_assert_ret_2: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*q).lock);
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                234 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                *__errno_location(),
                _mfs_errorstring_5,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                234 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                234 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_6,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                234 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                234 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_ret_2,
                *__errno_location(),
                _mfs_errorstring_err_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/../mfscommon/squeue.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                234 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(q->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_ret_2,
                *__errno_location(),
                _mfs_errorstring_err_2,
            );
        }
        abort();
    }
    if !data.is_null() {
        *data = (*qe).data;
    }
    free(qe as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
