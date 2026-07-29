use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::core::ffi::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
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
    fn lwt_minthread_create(
        th: *mut pthread_t,
        detached: uint8_t,
        r#fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
        arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn monotonic_seconds() -> ::core::ffi::c_double;
}
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
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
pub type pthread_t = ::core::ffi::c_ulong;
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
pub struct _sinoparent {
    pub inode: uint32_t,
    pub parent: uint32_t,
    pub validtime: uint32_t,
    pub next: *mut _sinoparent,
}
pub type sinoparent = _sinoparent;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn portable_usleep(mut usec: uint64_t) {
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
pub const MAX_LIST_LENGTH: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const HASH_SIZE: ::core::ffi::c_int = 16384 as ::core::ffi::c_int;
static mut sparents_hash: [*mut sinoparent; 16384] = [::core::ptr::null_mut::<sinoparent>(); 16384];
static mut sparents_lock: [pthread_mutex_t; 16384] = [pthread_mutex_t {
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
}; 16384];
static mut clthread: pthread_t = 0;
static mut term: uint8_t = 0;
#[inline]
unsafe extern "C" fn sparents_hashfn(mut inode: uint32_t) -> uint32_t {
    return inode.wrapping_rem(HASH_SIZE as uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn sparents_add(
    mut inode: uint32_t,
    mut parent: uint32_t,
    mut timeout: uint32_t,
) {
    let mut hash: uint32_t = 0;
    let mut leng: uint32_t = 0;
    let mut oldest: *mut sinoparent = ::core::ptr::null_mut::<sinoparent>();
    let mut sip: *mut sinoparent = ::core::ptr::null_mut::<sinoparent>();
    hash = sparents_hashfn(inode);
    let mut _mfs_assert_ret: ::core::ffi::c_int =
        pthread_mutex_lock((&raw mut sparents_lock as *mut pthread_mutex_t).offset(hash as isize));
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
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                65 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(sparents_lock+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                65 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(sparents_lock+hash)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                65 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(sparents_lock+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                65 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(sparents_lock+hash)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                65 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(sparents_lock+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                65 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(sparents_lock+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    sip = sparents_hash[hash as usize];
    oldest = ::core::ptr::null_mut::<sinoparent>();
    leng = 0 as uint32_t;
    while !sip.is_null() {
        if (*sip).inode == inode {
            (*sip).parent = parent;
            (*sip).validtime = (monotonic_seconds() + timeout as ::core::ffi::c_double) as uint32_t;
            let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(
                (&raw mut sparents_lock as *mut pthread_mutex_t).offset(hash as isize),
            );
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
                        b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        73 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(sparents_lock+hash)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        73 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(sparents_lock+hash)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        73 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(sparents_lock+hash)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        73 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(sparents_lock+hash)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        73 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(sparents_lock+hash)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        73 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(sparents_lock+hash)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                }
                abort();
            }
            return;
        }
        if oldest.is_null() || (*oldest).validtime > (*sip).validtime {
            oldest = sip;
        }
        leng = leng.wrapping_add(1);
        sip = (*sip).next as *mut sinoparent;
    }
    if leng >= MAX_LIST_LENGTH as uint32_t && !oldest.is_null() {
        sip = oldest;
    } else {
        sip = malloc(::core::mem::size_of::<sinoparent>()) as *mut sinoparent;
        (*sip).next = sparents_hash[hash as usize] as *mut _sinoparent;
        sparents_hash[hash as usize] = sip;
    }
    (*sip).inode = inode;
    (*sip).parent = parent;
    (*sip).validtime = (monotonic_seconds() + timeout as ::core::ffi::c_double) as uint32_t;
    let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_unlock(
        (&raw mut sparents_lock as *mut pthread_mutex_t).offset(hash as isize),
    );
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
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                92 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(sparents_lock+hash)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                92 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(sparents_lock+hash)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                92 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(sparents_lock+hash)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                92 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(sparents_lock+hash)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                92 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(sparents_lock+hash)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                92 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(sparents_lock+hash)\0".as_ptr()
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
#[no_mangle]
pub unsafe extern "C" fn sparents_get(mut inode: uint32_t) -> uint32_t {
    let mut hash: uint32_t = 0;
    let mut parent: uint32_t = 0;
    let mut sip: *mut sinoparent = ::core::ptr::null_mut::<sinoparent>();
    parent = 0 as uint32_t;
    hash = sparents_hashfn(inode);
    let mut _mfs_assert_ret: ::core::ffi::c_int =
        pthread_mutex_lock((&raw mut sparents_lock as *mut pthread_mutex_t).offset(hash as isize));
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
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                102 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(sparents_lock+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                102 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(sparents_lock+hash)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                102 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(sparents_lock+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                102 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(sparents_lock+hash)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                102 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(sparents_lock+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                102 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(sparents_lock+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    sip = sparents_hash[hash as usize];
    while !sip.is_null() {
        if (*sip).inode == inode {
            parent = (*sip).parent;
            break;
        } else {
            sip = (*sip).next as *mut sinoparent;
        }
    }
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(
        (&raw mut sparents_lock as *mut pthread_mutex_t).offset(hash as isize),
    );
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
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                111 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(sparents_lock+hash)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                111 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(sparents_lock+hash)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                111 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(sparents_lock+hash)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                111 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(sparents_lock+hash)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                111 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(sparents_lock+hash)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                111 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(sparents_lock+hash)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
    return parent;
}
unsafe extern "C" fn sparents_cleanup(mut hash: uint32_t, mut current_time: uint32_t) {
    let mut sip: *mut sinoparent = ::core::ptr::null_mut::<sinoparent>();
    let mut sipp: *mut *mut sinoparent = ::core::ptr::null_mut::<*mut sinoparent>();
    let mut _mfs_assert_ret: ::core::ffi::c_int =
        pthread_mutex_lock((&raw mut sparents_lock as *mut pthread_mutex_t).offset(hash as isize));
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
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                118 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(sparents_lock+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                118 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(sparents_lock+hash)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                118 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(sparents_lock+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                118 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(sparents_lock+hash)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                118 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(sparents_lock+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                118 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(sparents_lock+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    sipp = (&raw mut sparents_hash as *mut *mut sinoparent).offset(hash as isize);
    loop {
        sip = *sipp;
        if sip.is_null() {
            break;
        }
        if current_time == 0xffffffff as uint32_t || (*sip).validtime < current_time {
            *sipp = (*sip).next as *mut sinoparent;
            free(sip as *mut ::core::ffi::c_void);
        } else {
            sipp = &raw mut (*sip).next as *mut *mut sinoparent;
        }
    }
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(
        (&raw mut sparents_lock as *mut pthread_mutex_t).offset(hash as isize),
    );
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
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                128 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(sparents_lock+hash)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                128 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(sparents_lock+hash)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                128 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(sparents_lock+hash)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                128 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(sparents_lock+hash)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                128 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(sparents_lock+hash)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                128 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(sparents_lock+hash)\0".as_ptr()
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
unsafe extern "C" fn sparents_cleanupthread(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut cuhashpos: uint32_t = 0;
    let mut current_time: uint32_t = 0;
    let mut i: uint32_t = 0;
    cuhashpos = 0 as uint32_t;
    loop {
        current_time = monotonic_seconds() as uint32_t;
        i = 0 as uint32_t;
        while i < (HASH_SIZE / (10 as ::core::ffi::c_int * 30 as ::core::ffi::c_int)) as uint32_t {
            sparents_cleanup(cuhashpos, current_time);
            cuhashpos = cuhashpos
                .wrapping_add(1 as uint32_t)
                .wrapping_rem(HASH_SIZE as uint32_t);
            i = i.wrapping_add(1);
        }
        portable_usleep(100000 as uint64_t);
        if ::core::intrinsics::atomic_or_seqcst(&raw mut term, 0 as uint8_t) as ::core::ffi::c_int
            == 1 as ::core::ffi::c_int
        {
            return NULL;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sparents_term() {
    let mut hash: uint32_t = 0;
    ::core::intrinsics::atomic_or_seqcst(&raw mut term, 1 as uint8_t);
    pthread_join(
        clthread,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
    );
    hash = 0 as uint32_t;
    while hash < HASH_SIZE as uint32_t {
        sparents_cleanup(hash, 0xffffffff as uint32_t);
        if sparents_hash[hash as usize].is_null() {
        } else {
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                173 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"sparents_hash[hash]==NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"structure hasn't been cleaned up\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s' : %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                173 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"sparents_hash[hash]==NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"structure hasn't been cleaned up\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_destroy(
            (&raw mut sparents_lock as *mut pthread_mutex_t).offset(hash as isize),
        );
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
                    b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(sparents_lock+hash)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(sparents_lock+hash)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(sparents_lock+hash)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(sparents_lock+hash)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(sparents_lock+hash)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(sparents_lock+hash)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        hash = hash.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sparents_init() {
    let mut hash: uint32_t = 0;
    ::core::intrinsics::atomic_and_seqcst(&raw mut term, 0 as uint8_t);
    hash = 0 as uint32_t;
    while hash < HASH_SIZE as uint32_t {
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_init(
            (&raw mut sparents_lock as *mut pthread_mutex_t).offset(hash as isize),
            ::core::ptr::null::<pthread_mutexattr_t>(),
        );
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
                    b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(sparents_lock+hash,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(sparents_lock+hash,NULL)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(sparents_lock+hash,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(sparents_lock+hash,NULL)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(sparents_lock+hash,NULL)\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/sustained_parents.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(sparents_lock+hash,NULL)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        sparents_hash[hash as usize] = ::core::ptr::null_mut::<sinoparent>();
        hash = hash.wrapping_add(1);
    }
    lwt_minthread_create(
        &raw mut clthread,
        0 as uint8_t,
        Some(
            sparents_cleanupthread
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        NULL,
    );
}
