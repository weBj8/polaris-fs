pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn lwt_minthread_create(
        th: *mut pthread_t,
        detached: uint8_t,
        r#fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
        arg: *mut ::core::ffi::c_void,
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
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
}
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type size_t = usize;
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
pub struct _inode_stats {
    pub inode: uint32_t,
    pub attr: [uint8_t; 35],
    pub lastrefresh: ::core::ffi::c_double,
    pub next: *mut _inode_stats,
}
pub type inode_stats = _inode_stats;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_ENOENT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const TYPE_DIRECTORY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
static mut term: uint8_t = 0;
static mut clthread: pthread_t = 0;
pub const HASHSIZE: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
pub const SSTATS_TIMEOUT: ::core::ffi::c_double = 60.0f64;
static mut default_attr: [uint8_t; 35] = [
    0 as uint8_t,
    (0x1 as ::core::ffi::c_int | TYPE_DIRECTORY << 4 as ::core::ffi::c_int) as uint8_t,
    0xff as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
];
static mut ishash: *mut *mut inode_stats = ::core::ptr::null_mut::<*mut inode_stats>();
static mut locktab: *mut pthread_mutex_t = ::core::ptr::null_mut::<pthread_mutex_t>();
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sstats_get(
    mut inode: uint32_t,
    mut attr: *mut uint8_t,
    mut forceok: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut hash: uint32_t = 0;
        let mut isc: *mut inode_stats = ::core::ptr::null_mut::<inode_stats>();
        hash = inode.wrapping_rem(HASHSIZE as uint32_t);
        let mut _mfs_assert_ret: ::core::ffi::c_int =
            pthread_mutex_lock(locktab.offset(hash as isize));
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
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    130 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(locktab+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    130 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(locktab+hash)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    130 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(locktab+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    130 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(locktab+hash)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    130 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(locktab+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    130 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(locktab+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        isc = *ishash.offset(hash as isize);
        while !isc.is_null() {
            if (*isc).inode == inode {
                memcpy(
                    attr as *mut ::core::ffi::c_void,
                    &raw mut (*isc).attr as *mut uint8_t as *const ::core::ffi::c_void,
                    35 as size_t,
                );
                (*isc).lastrefresh = monotonic_seconds();
                let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                    pthread_mutex_unlock(locktab.offset(hash as isize));
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
                            b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            136 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(locktab+hash)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_1,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            136 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(locktab+hash)\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            136 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(locktab+hash)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_2,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            136 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(locktab+hash)\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            136 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(locktab+hash)\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            136 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(locktab+hash)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_err_0,
                        );
                    }
                    abort();
                }
                return MFS_STATUS_OK;
            }
            isc = (*isc).next as *mut inode_stats;
        }
        if forceok != 0 {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"no sustained stats for node: %u - using defaults\0".as_ptr()
                    as *const ::core::ffi::c_char,
                inode,
            );
            isc = malloc(::core::mem::size_of::<inode_stats>()) as *mut inode_stats;
            if isc.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    144 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"isc\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    144 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"isc\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if isc
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut inode_stats
            {
                let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    144 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"isc\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    144 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"isc\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_3,
                );
                abort();
            }
            (*isc).inode = inode;
            memcpy(
                &raw mut (*isc).attr as *mut uint8_t as *mut ::core::ffi::c_void,
                &raw mut default_attr as *mut uint8_t as *const ::core::ffi::c_void,
                35 as size_t,
            );
            memcpy(
                attr as *mut ::core::ffi::c_void,
                &raw mut default_attr as *mut uint8_t as *const ::core::ffi::c_void,
                35 as size_t,
            );
            (*isc).lastrefresh = monotonic_seconds();
            (*isc).next = *ishash.offset(hash as isize) as *mut _inode_stats;
            *ishash.offset(hash as isize) = isc;
        }
        let mut _mfs_assert_ret_1: ::core::ffi::c_int =
            pthread_mutex_unlock(locktab.offset(hash as isize));
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
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    153 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(locktab+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    153 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(locktab+hash)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    153 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(locktab+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_5,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    153 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(locktab+hash)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    153 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(locktab+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    153 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(locktab+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
            }
            abort();
        }
        return MFS_ERROR_ENOENT;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sstats_set(
    mut inode: uint32_t,
    mut attr: *const uint8_t,
    mut createflag: uint8_t,
) {
    unsafe {
        let mut hash: uint32_t = 0;
        let mut isc: *mut inode_stats = ::core::ptr::null_mut::<inode_stats>();
        hash = inode.wrapping_rem(HASHSIZE as uint32_t);
        let mut _mfs_assert_ret: ::core::ffi::c_int =
            pthread_mutex_lock(locktab.offset(hash as isize));
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
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    165 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(locktab+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    165 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(locktab+hash)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    165 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(locktab+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    165 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(locktab+hash)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    165 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(locktab+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    165 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(locktab+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        isc = *ishash.offset(hash as isize);
        while !isc.is_null() {
            if (*isc).inode == inode {
                memcpy(
                    &raw mut (*isc).attr as *mut uint8_t as *mut ::core::ffi::c_void,
                    attr as *const ::core::ffi::c_void,
                    35 as size_t,
                );
                (*isc).lastrefresh = monotonic_seconds();
                let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                    pthread_mutex_unlock(locktab.offset(hash as isize));
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
                            b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            170 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(locktab+hash)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_1,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            170 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(locktab+hash)\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            170 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(locktab+hash)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_2,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            170 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(locktab+hash)\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            170 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(locktab+hash)\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            170 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(locktab+hash)\0".as_ptr()
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
            isc = (*isc).next as *mut inode_stats;
        }
        if createflag != 0 {
            isc = malloc(::core::mem::size_of::<inode_stats>()) as *mut inode_stats;
            if isc.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    176 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"isc\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    176 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"isc\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if isc
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut inode_stats
            {
                let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    176 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"isc\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    176 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"isc\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_3,
                );
                abort();
            }
            (*isc).inode = inode;
            memcpy(
                &raw mut (*isc).attr as *mut uint8_t as *mut ::core::ffi::c_void,
                attr as *const ::core::ffi::c_void,
                35 as size_t,
            );
            (*isc).lastrefresh = monotonic_seconds();
            (*isc).next = *ishash.offset(hash as isize) as *mut _inode_stats;
            *ishash.offset(hash as isize) = isc;
        }
        let mut _mfs_assert_ret_1: ::core::ffi::c_int =
            pthread_mutex_unlock(locktab.offset(hash as isize));
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
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    184 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(locktab+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    184 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(locktab+hash)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    184 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(locktab+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_5,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    184 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(locktab+hash)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    184 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(locktab+hash)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    184 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(locktab+hash)\0".as_ptr() as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn sstats_thread(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut hash: uint32_t = 0 as uint32_t;
        let mut isc: *mut inode_stats = ::core::ptr::null_mut::<inode_stats>();
        let mut isp: *mut *mut inode_stats = ::core::ptr::null_mut::<*mut inode_stats>();
        let mut now: ::core::ffi::c_double = 0.;
        loop {
            now = monotonic_seconds();
            let mut _mfs_assert_ret: ::core::ffi::c_int =
                pthread_mutex_lock(locktab.offset(hash as isize));
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
                        b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        195 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(locktab+hash)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        *__errno_location(),
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        195 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(locktab+hash)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        195 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(locktab+hash)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        195 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(locktab+hash)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        195 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(locktab+hash)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        195 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(locktab+hash)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_ret,
                        *__errno_location(),
                        _mfs_errorstring_err,
                    );
                }
                abort();
            }
            isp = ishash.offset(hash as isize);
            loop {
                isc = *isp;
                if isc.is_null() {
                    break;
                }
                if (*isc).lastrefresh + SSTATS_TIMEOUT < now {
                    *isp = (*isc).next as *mut inode_stats;
                    free(isc as *mut ::core::ffi::c_void);
                } else {
                    isp = &raw mut (*isc).next as *mut *mut inode_stats;
                }
            }
            let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                pthread_mutex_unlock(locktab.offset(hash as isize));
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
                        b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        205 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(locktab+hash)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        205 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(locktab+hash)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        205 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(locktab+hash)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        205 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(locktab+hash)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        205 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(locktab+hash)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        205 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(locktab+hash)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                }
                abort();
            }
            portable_usleep(100000 as uint64_t);
            if ::core::intrinsics::atomic_or::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
                &raw mut term,
                0 as uint8_t,
            ) as ::core::ffi::c_int
                == 1 as ::core::ffi::c_int
            {
                return NULL;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sstats_term() {
    unsafe {
        let mut i: uint32_t = 0;
        let mut isc: *mut inode_stats = ::core::ptr::null_mut::<inode_stats>();
        let mut isn: *mut inode_stats = ::core::ptr::null_mut::<inode_stats>();
        ::core::intrinsics::atomic_or::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut term,
            1 as uint8_t,
        );
        pthread_join(
            clthread,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        );
        i = 0 as uint32_t;
        while i < HASHSIZE as uint32_t {
            let mut _mfs_assert_ret: ::core::ffi::c_int =
                pthread_mutex_destroy(locktab.offset(i as isize));
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
                        b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        239 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_destroy(locktab+i)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        *__errno_location(),
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        239 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_destroy(locktab+i)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        239 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_destroy(locktab+i)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        239 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_destroy(locktab+i)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        239 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_destroy(locktab+i)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        239 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_destroy(locktab+i)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_ret,
                        *__errno_location(),
                        _mfs_errorstring_err,
                    );
                }
                abort();
            }
            isc = *ishash.offset(i as isize);
            while !isc.is_null() {
                isn = (*isc).next as *mut inode_stats;
                free(isc as *mut ::core::ffi::c_void);
                isc = isn;
            }
            i = i.wrapping_add(1);
        }
        free(locktab as *mut ::core::ffi::c_void);
        free(ishash as *mut ::core::ffi::c_void);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sstats_init() {
    unsafe {
        let mut i: uint32_t = 0;
        ishash = malloc(::core::mem::size_of::<*mut inode_stats>().wrapping_mul(HASHSIZE as size_t))
            as *mut *mut inode_stats;
        if ishash.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                252 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"ishash\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                252 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"ishash\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if ishash
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut *mut inode_stats
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                252 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"ishash\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                252 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"ishash\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        locktab = malloc(::core::mem::size_of::<pthread_mutex_t>().wrapping_mul(HASHSIZE as size_t))
            as *mut pthread_mutex_t;
        if locktab.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                254 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"locktab\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                254 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"locktab\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if locktab
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut pthread_mutex_t
        {
            let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                254 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"locktab\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                254 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"locktab\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            abort();
        }
        i = 0 as uint32_t;
        while i < HASHSIZE as uint32_t {
            *ishash.offset(i as isize) = ::core::ptr::null_mut::<inode_stats>();
            let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_init(
                locktab.offset(i as isize),
                ::core::ptr::null::<pthread_mutexattr_t>(),
            );
            if _mfs_assert_ret != 0 as ::core::ffi::c_int {
                if _mfs_assert_ret < 0 as ::core::ffi::c_int
                    && *__errno_location() != 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        257 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_init(locktab+i,NULL)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        257 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_init(locktab+i,NULL)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        257 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_init(locktab+i,NULL)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        257 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_init(locktab+i,NULL)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_2,
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
                        b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        257 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_init(locktab+i,NULL)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/sustained_stats.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        257 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_init(locktab+i,NULL)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_ret,
                        *__errno_location(),
                        _mfs_errorstring_err,
                    );
                }
                abort();
            }
            i = i.wrapping_add(1);
        }
        ::core::intrinsics::atomic_and::<_, _, { ::core::intrinsics::AtomicOrdering::SeqCst }>(
            &raw mut term,
            0 as uint8_t,
        );
        lwt_minthread_create(
            &raw mut clthread,
            0 as uint8_t,
            Some(
                sstats_thread
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
            ),
            NULL,
        );
    }
}
