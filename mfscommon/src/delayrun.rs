pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn gettimeofday(
        __tv: *mut timeval,
        __tz: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    unsafe fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    unsafe fn sigaddset(__set: *mut sigset_t, __signo: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn pthread_sigmask(
        __how: ::core::ffi::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
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
    unsafe fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::core::ffi::c_int;
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
    unsafe fn monotonic_useconds() -> uint64_t;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
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
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
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
pub struct _heapelem {
    pub r#fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub udata: *mut ::core::ffi::c_void,
    pub firetime: uint64_t,
}
pub type heapelem = _heapelem;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SIGHUP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIGQUIT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut heap: *mut heapelem = ::core::ptr::null_mut::<heapelem>();
static mut heapelements: uint32_t = 0;
static mut heapsize: uint32_t = 0;
static mut exitflag: uint8_t = 0;
static mut waiting: uint8_t = 0;
static mut dlock: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0,
        __count: 0,
        __owner: 0,
        __nusers: 0,
        __kind: 0,
        __spins: 0,
        __glibc_reserved: 0,
        __list: __pthread_list_t {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
static mut dcond: pthread_cond_t = pthread_cond_t {
    __data: __pthread_cond_s {
        __wseq: __atomic_wide_counter { __value64: 0 },
        __g1_start: __atomic_wide_counter { __value64: 0 },
        __g_size: [0; 2],
        __g1_orig_size: 0,
        __wrefs: 0,
        __g_signals: [0; 2],
        __unused_initialized_1: 0,
        __unused_initialized_2: 0,
    },
};
static mut delay_th: pthread_t = 0;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delay_heap_sort_down() {
    unsafe {
        let mut l: uint32_t = 0;
        let mut r: uint32_t = 0;
        let mut m: uint32_t = 0;
        let mut pos: uint32_t = 0 as uint32_t;
        let mut x: heapelem = heapelem {
            r#fn: None,
            udata: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            firetime: 0,
        };
        while pos < heapelements {
            l = pos.wrapping_mul(2 as uint32_t).wrapping_add(1 as uint32_t);
            r = l.wrapping_add(1 as uint32_t);
            if l >= heapelements {
                return;
            }
            m = l;
            if r < heapelements
                && (*heap.offset(r as isize)).firetime < (*heap.offset(l as isize)).firetime
            {
                m = r;
            }
            if (*heap.offset(pos as isize)).firetime <= (*heap.offset(m as isize)).firetime {
                return;
            }
            x = *heap.offset(pos as isize);
            *heap.offset(pos as isize) = *heap.offset(m as isize);
            *heap.offset(m as isize) = x;
            pos = m;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delay_heap_sort_up() -> uint8_t {
    unsafe {
        let mut pos: uint32_t = heapelements.wrapping_sub(1 as uint32_t);
        let mut p: uint32_t = 0;
        let mut x: heapelem = heapelem {
            r#fn: None,
            udata: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            firetime: 0,
        };
        while pos > 0 as uint32_t {
            p = pos.wrapping_sub(1 as uint32_t).wrapping_div(2 as uint32_t);
            if (*heap.offset(pos as isize)).firetime >= (*heap.offset(p as isize)).firetime {
                return 0 as uint8_t;
            }
            x = *heap.offset(pos as isize);
            *heap.offset(pos as isize) = *heap.offset(p as isize);
            *heap.offset(p as isize) = x;
            pos = p;
        }
        return 1 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delay_scheduler(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut now: uint64_t = 0;
        let mut ts: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let mut tv: timeval = timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        let mut r#fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()> = None;
        let mut udata: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut dlock);
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    82 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    82 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    82 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    82 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    82 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    82 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        loop {
            if exitflag != 0 {
                let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                    pthread_mutex_unlock(&raw mut dlock);
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
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            85 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&dlock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_1,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            85 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&dlock)\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            85 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&dlock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_2,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            85 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&dlock)\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            85 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&dlock)\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            85 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&dlock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_err_0,
                        );
                    }
                    abort();
                }
                return arg;
            }
            if heapelements > 0 as uint32_t {
                now = monotonic_useconds();
                if now < (*heap.offset(0 as isize)).firetime {
                    gettimeofday(&raw mut tv, NULL);
                    ts.tv_sec = (tv.tv_sec as uint64_t).wrapping_add(
                        (*heap.offset(0 as isize))
                            .firetime
                            .wrapping_sub(now)
                            .wrapping_div(1000000 as uint64_t),
                    ) as __time_t;
                    ts.tv_nsec = (tv.tv_usec as uint64_t)
                        .wrapping_add(
                            (*heap.offset(0 as isize))
                                .firetime
                                .wrapping_sub(now)
                                .wrapping_rem(1000000 as uint64_t),
                        )
                        .wrapping_mul(1000 as uint64_t)
                        as __syscall_slong_t;
                    while ts.tv_nsec >= 1000000000 as __syscall_slong_t {
                        ts.tv_sec += 1;
                        ts.tv_nsec -= 1000000000 as __syscall_slong_t;
                    }
                    waiting = 1 as uint8_t;
                    pthread_cond_timedwait(&raw mut dcond, &raw mut dlock, &raw mut ts);
                    waiting = 0 as uint8_t;
                } else {
                    r#fn = (*heap.offset(0 as isize)).r#fn;
                    udata = (*heap.offset(0 as isize)).udata;
                    heapelements = heapelements.wrapping_sub(1);
                    if heapelements > 0 as uint32_t {
                        *heap.offset(0 as isize) = *heap.offset(heapelements as isize);
                        delay_heap_sort_down();
                    }
                    let mut _mfs_assert_ret_1: ::core::ffi::c_int =
                        pthread_mutex_unlock(&raw mut dlock);
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
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                110 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&dlock)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_1,
                                *__errno_location(),
                                _mfs_errorstring_3,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                110 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&dlock)\0".as_ptr()
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
                                b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                110 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&dlock)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_1,
                                _mfs_errorstring_4,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                110 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&dlock)\0".as_ptr()
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
                                b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                110 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&dlock)\0".as_ptr()
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
                                b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                110 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&dlock)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_1,
                                _mfs_errorstring_ret_1,
                                *__errno_location(),
                                _mfs_errorstring_err_1,
                            );
                        }
                        abort();
                    }
                    Some(r#fn.expect("non-null function pointer"))
                        .expect("non-null function pointer")(udata);
                    let mut _mfs_assert_ret_2: ::core::ffi::c_int =
                        pthread_mutex_lock(&raw mut dlock);
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
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                112 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_lock(&dlock)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_2,
                                *__errno_location(),
                                _mfs_errorstring_5,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                112 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_lock(&dlock)\0".as_ptr()
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
                                b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                112 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_lock(&dlock)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_2,
                                _mfs_errorstring_6,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                112 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_lock(&dlock)\0".as_ptr()
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
                                b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                112 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_lock(&dlock)\0".as_ptr()
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
                                b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                112 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_lock(&dlock)\0".as_ptr()
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
            } else {
                waiting = 1 as uint8_t;
                let mut _mfs_assert_ret_3: ::core::ffi::c_int =
                    pthread_cond_wait(&raw mut dcond, &raw mut dlock);
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
                            b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            116 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_wait(&dcond,&dlock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_3,
                            *__errno_location(),
                            _mfs_errorstring_7,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            116 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_wait(&dcond,&dlock)\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            116 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_wait(&dcond,&dlock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_3,
                            _mfs_errorstring_8,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            116 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_wait(&dcond,&dlock)\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            116 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_wait(&dcond,&dlock)\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            116 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_wait(&dcond,&dlock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_3,
                            _mfs_errorstring_ret_3,
                            *__errno_location(),
                            _mfs_errorstring_err_3,
                        );
                    }
                    abort();
                }
                waiting = 0 as uint8_t;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delay_run(
    mut r#fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    mut udata: *mut ::core::ffi::c_void,
    mut useconds: uint64_t,
) {
    unsafe {
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut dlock);
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    125 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    125 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    125 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    125 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    125 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    125 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        if heapelements >= heapsize {
            heapsize = heapsize.wrapping_mul(2 as uint32_t);
            heap = realloc(
                heap as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<heapelem>().wrapping_mul(heapsize as size_t),
            ) as *mut heapelem;
            if heap.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    129 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"heap\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    129 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"heap\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if heap
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut heapelem
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    129 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"heap\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    129 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"heap\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_1,
                );
                abort();
            }
        }
        (*heap.offset(heapelements as isize)).r#fn = r#fn;
        (*heap.offset(heapelements as isize)).udata = udata;
        (*heap.offset(heapelements as isize)).firetime =
            monotonic_useconds().wrapping_add(useconds);
        heapelements = heapelements.wrapping_add(1);
        if delay_heap_sort_up() as ::core::ffi::c_int != 0 && waiting as ::core::ffi::c_int != 0 {
            let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_cond_signal(&raw mut dcond);
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
                        b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        137 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&dcond)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        137 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&dcond)\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        137 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&dcond)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_3,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        137 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&dcond)\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        137 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&dcond)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        137 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&dcond)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                }
                abort();
            }
        }
        let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut dlock);
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    139 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    139 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    139 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_5,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    139 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    139 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    139 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
            }
            abort();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delay_term() {
    unsafe {
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut dlock);
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    143 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    143 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    143 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    143 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    143 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    143 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        exitflag = 1 as uint8_t;
        if waiting != 0 {
            let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_cond_signal(&raw mut dcond);
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
                        b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        146 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&dcond)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        146 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&dcond)\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        146 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&dcond)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        146 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&dcond)\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        146 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&dcond)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        146 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&dcond)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                }
                abort();
            }
        }
        let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut dlock);
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    148 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    148 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    148 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    148 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    148 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    148 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_2: ::core::ffi::c_int = pthread_join(
            delay_th,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        );
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    149 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_join(delay_th,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_5,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    149 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_join(delay_th,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    149 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_join(delay_th,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_6,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    149 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_join(delay_th,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    149 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_join(delay_th,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    149 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_join(delay_th,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_3: ::core::ffi::c_int = pthread_cond_destroy(&raw mut dcond);
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    150 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&dcond)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_7,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    150 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&dcond)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    150 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&dcond)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_8,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    150 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&dcond)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    150 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&dcond)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_err_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    150 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_destroy(&dcond)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_err_3,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_4: ::core::ffi::c_int = pthread_mutex_destroy(&raw mut dlock);
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    151 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_9,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    151 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    151 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_10,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    151 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    151 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_err_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    151 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&dlock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_err_4,
                );
            }
            abort();
        }
        free(heap as *mut ::core::ffi::c_void);
        heap = ::core::ptr::null_mut::<heapelem>();
        heapsize = 0 as uint32_t;
        heapelements = 0 as uint32_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn delay_init() {
    unsafe {
        let mut thattr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
        let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
        let mut newset: sigset_t = sigset_t { __val: [0; 16] };
        exitflag = 0 as uint8_t;
        waiting = 0 as uint8_t;
        heap = malloc(::core::mem::size_of::<heapelem>().wrapping_mul(1024 as size_t))
            as *mut heapelem;
        if heap.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                168 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"heap\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                168 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"heap\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if heap
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut heapelem
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                168 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"heap\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                168 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"heap\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        heapelements = 0 as uint32_t;
        heapsize = 1024 as uint32_t;
        let mut _mfs_assert_ret: ::core::ffi::c_int =
            pthread_mutex_init(&raw mut dlock, ::core::ptr::null::<pthread_mutexattr_t>());
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    171 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&dlock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    171 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&dlock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    171 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&dlock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    171 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&dlock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    171 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&dlock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    171 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(&dlock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_0: ::core::ffi::c_int =
            pthread_cond_init(&raw mut dcond, ::core::ptr::null::<pthread_condattr_t>());
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    172 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&dcond,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    172 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&dcond,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    172 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&dcond,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    172 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&dcond,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    172 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&dcond,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    172 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_init(&dcond,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_attr_init(&raw mut thattr);
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_init(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_init(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_init(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_5,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_init(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_init(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_init(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_2: ::core::ffi::c_int =
            pthread_attr_setstacksize(&raw mut thattr, 0x100000 as ::core::ffi::c_int as size_t);
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    175 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_setstacksize(&thattr,0x100000)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_6,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    175 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_setstacksize(&thattr,0x100000)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    175 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_setstacksize(&thattr,0x100000)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_7,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    175 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_setstacksize(&thattr,0x100000)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    175 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_setstacksize(&thattr,0x100000)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    175 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_setstacksize(&thattr,0x100000)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
            }
            abort();
        }
        sigemptyset(&raw mut newset);
        sigaddset(&raw mut newset, SIGTERM);
        sigaddset(&raw mut newset, SIGINT);
        sigaddset(&raw mut newset, SIGHUP);
        sigaddset(&raw mut newset, SIGQUIT);
        let mut _mfs_assert_ret_3: ::core::ffi::c_int =
            pthread_sigmask(0 as ::core::ffi::c_int, &raw mut newset, &raw mut oldset);
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    182 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_sigmask(SIG_BLOCK, &newset, &oldset)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_8,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    182 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_sigmask(SIG_BLOCK, &newset, &oldset)\0".as_ptr()
                        as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    182 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_sigmask(SIG_BLOCK, &newset, &oldset)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_9,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    182 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_sigmask(SIG_BLOCK, &newset, &oldset)\0".as_ptr()
                        as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    182 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_sigmask(SIG_BLOCK, &newset, &oldset)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    182 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_sigmask(SIG_BLOCK, &newset, &oldset)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_err_3,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_4: ::core::ffi::c_int = pthread_create(
            &raw mut delay_th,
            &raw mut thattr,
            Some(
                delay_scheduler
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
            ),
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    184 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_create(&delay_th,&thattr,delay_scheduler,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_10,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    184 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_create(&delay_th,&thattr,delay_scheduler,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    184 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_create(&delay_th,&thattr,delay_scheduler,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_11,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    184 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_create(&delay_th,&thattr,delay_scheduler,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    184 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_create(&delay_th,&thattr,delay_scheduler,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_err_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    184 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_create(&delay_th,&thattr,delay_scheduler,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_4,
                    _mfs_errorstring_ret_4,
                    *__errno_location(),
                    _mfs_errorstring_err_4,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_5: ::core::ffi::c_int = pthread_sigmask(
            2 as ::core::ffi::c_int,
            &raw mut oldset,
            ::core::ptr::null_mut::<__sigset_t>(),
        );
        if _mfs_assert_ret_5 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_5 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_12: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    186 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_sigmask(SIG_SETMASK, &oldset, NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_5,
                    *__errno_location(),
                    _mfs_errorstring_12,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    186 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_sigmask(SIG_SETMASK, &oldset, NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_5,
                    *__errno_location(),
                    _mfs_errorstring_12,
                );
            } else if _mfs_assert_ret_5 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_13: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_5);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    186 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_sigmask(SIG_SETMASK, &oldset, NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_5,
                    _mfs_errorstring_13,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    186 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_sigmask(SIG_SETMASK, &oldset, NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_5,
                    _mfs_errorstring_13,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    186 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_sigmask(SIG_SETMASK, &oldset, NULL)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    186 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_sigmask(SIG_SETMASK, &oldset, NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_5,
                    _mfs_errorstring_ret_5,
                    *__errno_location(),
                    _mfs_errorstring_err_5,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_6: ::core::ffi::c_int = pthread_attr_destroy(&raw mut thattr);
        if _mfs_assert_ret_6 != 0 as ::core::ffi::c_int {
            if _mfs_assert_ret_6 < 0 as ::core::ffi::c_int
                && *__errno_location() != 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_14: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    188 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_destroy(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_6,
                    *__errno_location(),
                    _mfs_errorstring_14,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    188 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_destroy(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_6,
                    *__errno_location(),
                    _mfs_errorstring_14,
                );
            } else if _mfs_assert_ret_6 > 0 as ::core::ffi::c_int
                && *__errno_location() == 0 as ::core::ffi::c_int
            {
                let mut _mfs_errorstring_15: *const ::core::ffi::c_char = strerr(_mfs_assert_ret_6);
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    188 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_destroy(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_6,
                    _mfs_errorstring_15,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    188 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_destroy(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_6,
                    _mfs_errorstring_15,
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
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    188 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_destroy(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_6,
                    _mfs_errorstring_ret_6,
                    *__errno_location(),
                    _mfs_errorstring_err_6,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfstests/../mfscommon/delayrun.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    188 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_attr_destroy(&thattr)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_6,
                    _mfs_errorstring_ret_6,
                    *__errno_location(),
                    _mfs_errorstring_err_6,
                );
            }
            abort();
        }
    }
}
