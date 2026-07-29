use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn pthread_sigmask(
        __how: ::core::ffi::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn abort() -> !;
    fn __sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_attr_init(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    fn pthread_attr_setdetachstate(
        __attr: *mut pthread_attr_t,
        __detachstate: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn pthread_attr_setstacksize(
        __attr: *mut pthread_attr_t,
        __stacksize: size_t,
    ) -> ::core::ffi::c_int;
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
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
}
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
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
pub type size_t = usize;
pub type pthread_t = ::core::ffi::c_ulong;
pub type uint8_t = u8;
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const PTHREAD_CREATE_DETACHED: C2Rust_Unnamed = 1;
pub const PTHREAD_CREATE_JOINABLE: C2Rust_Unnamed = 0;
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
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SIGHUP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIGQUIT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SIGPIPE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SIGALRM: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const SIGTSTP: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const SIGTTIN: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const SIGTTOU: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const SIGVTALRM: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const SIGPROF: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const SIGUSR1: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SIGUSR2: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const SIG_BLOCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SIG_SETMASK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __SC_THREAD_STACK_MIN_VALUE: ::core::ffi::c_int = 75 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn lwt_thread_create(
    mut th: *mut pthread_t,
    mut attr: *const pthread_attr_t,
    mut r#fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
    mut arg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
    let mut newset: sigset_t = sigset_t { __val: [0; 16] };
    let mut res: ::core::ffi::c_int = 0;
    sigemptyset(&raw mut newset);
    sigaddset(&raw mut newset, SIGTERM);
    sigaddset(&raw mut newset, SIGINT);
    sigaddset(&raw mut newset, SIGHUP);
    sigaddset(&raw mut newset, SIGQUIT);
    sigaddset(&raw mut newset, SIGPIPE);
    sigaddset(&raw mut newset, SIGTSTP);
    sigaddset(&raw mut newset, SIGTTIN);
    sigaddset(&raw mut newset, SIGTTOU);
    sigaddset(&raw mut newset, SIGUSR1);
    sigaddset(&raw mut newset, SIGUSR2);
    sigaddset(&raw mut newset, SIGALRM);
    sigaddset(&raw mut newset, SIGVTALRM);
    sigaddset(&raw mut newset, SIGPROF);
    pthread_sigmask(SIG_BLOCK, &raw mut newset, &raw mut oldset);
    res = pthread_create(th, attr, r#fn, arg);
    pthread_sigmask(
        SIG_SETMASK,
        &raw mut oldset,
        ::core::ptr::null_mut::<__sigset_t>(),
    );
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn lwt_minthread_create(
    mut th: *mut pthread_t,
    mut detached: uint8_t,
    mut r#fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
    mut arg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    static mut thattr: *mut pthread_attr_t = ::core::ptr::null_mut::<pthread_attr_t>();
    static mut thattr_detached: uint8_t = 0;
    if thattr.is_null() {
        let mut mystacksize: size_t = 0;
        thattr = malloc(::core::mem::size_of::<pthread_attr_t>()) as *mut pthread_attr_t;
        if thattr.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                88 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"thattr\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                88 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"thattr\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if thattr
            == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut pthread_attr_t
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                88 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"thattr\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                88 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"thattr\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_attr_init(thattr);
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    89 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_init(thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    89 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_init(thattr)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    89 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_init(thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    89 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_init(thattr)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    89 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_init(thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    89 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_init(thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        mystacksize = __sysconf(__SC_THREAD_STACK_MIN_VALUE) as size_t;
        if mystacksize < 0x20000 as ::core::ffi::c_int as size_t {
            mystacksize = 0x20000 as ::core::ffi::c_int as size_t;
        }
        let mut _mfs_assert_ret_0: ::core::ffi::c_int =
            pthread_attr_setstacksize(thattr, mystacksize);
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    98 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_setstacksize(thattr,mystacksize)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    98 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_setstacksize(thattr,mystacksize)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    98 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_setstacksize(thattr,mystacksize)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    98 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_setstacksize(thattr,mystacksize)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    98 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_setstacksize(thattr,mystacksize)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    98 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_setstacksize(thattr,mystacksize)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
        thattr_detached = (detached as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t;
    }
    if detached as ::core::ffi::c_int != thattr_detached as ::core::ffi::c_int {
        if detached != 0 {
            let mut _mfs_assert_ret_1: ::core::ffi::c_int =
                pthread_attr_setdetachstate(thattr, PTHREAD_CREATE_DETACHED as ::core::ffi::c_int);
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
                        b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        103 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_attr_setdetachstate(thattr,PTHREAD_CREATE_DETACHED)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_1,
                        *__errno_location(),
                        _mfs_errorstring_4,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        103 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_attr_setdetachstate(thattr,PTHREAD_CREATE_DETACHED)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        103 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_attr_setdetachstate(thattr,PTHREAD_CREATE_DETACHED)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_1,
                        _mfs_errorstring_5,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        103 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_attr_setdetachstate(thattr,PTHREAD_CREATE_DETACHED)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        103 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_attr_setdetachstate(thattr,PTHREAD_CREATE_DETACHED)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        103 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_attr_setdetachstate(thattr,PTHREAD_CREATE_DETACHED)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_1,
                        _mfs_errorstring_ret_1,
                        *__errno_location(),
                        _mfs_errorstring_err_1,
                    );
                }
                abort();
            }
        } else {
            let mut _mfs_assert_ret_2: ::core::ffi::c_int =
                pthread_attr_setdetachstate(thattr, PTHREAD_CREATE_JOINABLE as ::core::ffi::c_int);
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
                        b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        105 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_attr_setdetachstate(thattr,PTHREAD_CREATE_JOINABLE)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        *__errno_location(),
                        _mfs_errorstring_6,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        105 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_attr_setdetachstate(thattr,PTHREAD_CREATE_JOINABLE)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        105 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_attr_setdetachstate(thattr,PTHREAD_CREATE_JOINABLE)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        _mfs_errorstring_7,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        105 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_attr_setdetachstate(thattr,PTHREAD_CREATE_JOINABLE)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        105 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_attr_setdetachstate(thattr,PTHREAD_CREATE_JOINABLE)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/lwthread.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        105 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_attr_setdetachstate(thattr,PTHREAD_CREATE_JOINABLE)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_2,
                        _mfs_errorstring_ret_2,
                        *__errno_location(),
                        _mfs_errorstring_err_2,
                    );
                }
                abort();
            }
        }
        thattr_detached = detached;
    }
    return lwt_thread_create(th, thattr, r#fn, arg);
}
