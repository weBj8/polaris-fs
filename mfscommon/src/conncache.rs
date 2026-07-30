pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
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
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn monotonic_useconds() -> uint64_t;
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
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn tcpclose(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn lwt_minthread_create(
        th: *mut pthread_t,
        detached: uint8_t,
        r#fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
        arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type ssize_t = isize;
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
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2Rust_Unnamed = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2Rust_Unnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2Rust_Unnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2Rust_Unnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2Rust_Unnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2Rust_Unnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2Rust_Unnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2Rust_Unnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2Rust_Unnamed = 0;
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
pub struct _connentry {
    pub ip: uint32_t,
    pub port: uint16_t,
    pub fd: ::core::ffi::c_int,
    pub lrunext: *mut _connentry,
    pub lruprev: *mut *mut _connentry,
    pub hashnext: *mut _connentry,
    pub hashprev: *mut *mut _connentry,
}
pub type connentry = _connentry;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn hash32(mut key: uint32_t) -> uint32_t {
    unsafe {
        key = (!key).wrapping_add(key << 15 as ::core::ffi::c_int);
        key = key ^ key >> 12 as ::core::ffi::c_int;
        key = key.wrapping_add(key << 2 as ::core::ffi::c_int);
        key = key ^ key >> 4 as ::core::ffi::c_int;
        key = key.wrapping_mul(2057 as uint32_t);
        key = key ^ key >> 16 as ::core::ffi::c_int;
        return key;
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
pub const CONN_CACHE_HASHSIZE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
static mut main_thread: pthread_t = 0;
static mut keep_alive: ::core::ffi::c_int = 0;
static mut conncachetab: *mut connentry = ::core::ptr::null_mut::<connentry>();
static mut conncachehash: [*mut connentry; 256] = [::core::ptr::null_mut::<connentry>(); 256];
static mut capacity: uint32_t = 0;
static mut lruhead: *mut connentry = ::core::ptr::null_mut::<connentry>();
static mut lrutail: *mut *mut connentry = ::core::ptr::null_mut::<*mut connentry>();
static mut freehead: *mut connentry = ::core::ptr::null_mut::<connentry>();
static mut glock: pthread_mutex_t = pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0 as ::core::ffi::c_int,
        __count: 0 as ::core::ffi::c_uint,
        __owner: 0 as ::core::ffi::c_int,
        __nusers: 0 as ::core::ffi::c_uint,
        __kind: PTHREAD_MUTEX_TIMED_NP as ::core::ffi::c_int,
        __spins: 0 as ::core::ffi::c_short,
        __glibc_reserved: 0 as ::core::ffi::c_short,
        __list: __pthread_internal_list {
            __prev: ::core::ptr::null_mut::<__pthread_internal_list>(),
            __next: ::core::ptr::null_mut::<__pthread_internal_list>(),
        },
    },
};
#[inline]
unsafe extern "C" fn conncache_remove(mut ce: *mut connentry, mut closeflag: ::core::ffi::c_int) {
    unsafe {
        if !(*ce).lrunext.is_null() {
            (*(*ce).lrunext).lruprev = (*ce).lruprev;
        } else {
            lrutail = (*ce).lruprev as *mut *mut connentry;
        }
        *(*ce).lruprev = (*ce).lrunext;
        if !(*ce).hashnext.is_null() {
            (*(*ce).hashnext).hashprev = (*ce).hashprev;
        }
        *(*ce).hashprev = (*ce).hashnext;
        (*ce).lrunext = ::core::ptr::null_mut::<_connentry>();
        (*ce).lruprev = ::core::ptr::null_mut::<*mut _connentry>();
        (*ce).hashnext = freehead as *mut _connentry;
        (*ce).hashprev = ::core::ptr::null_mut::<*mut _connentry>();
        freehead = ce;
        if closeflag != 0 {
            tcpclose((*ce).fd);
        }
        (*ce).fd = -1 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn conncache_insert(
    mut ip: uint32_t,
    mut port: uint16_t,
    mut fd: ::core::ffi::c_int,
) {
    unsafe {
        let mut hash: uint32_t = 0;
        let mut ce: *mut connentry = ::core::ptr::null_mut::<connentry>();
        hash = hash32(ip ^ ((port as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as uint32_t)
            .wrapping_rem(CONN_CACHE_HASHSIZE as uint32_t);
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut glock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    84 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    84 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    84 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    84 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    84 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    84 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        if freehead.is_null() {
            conncache_remove(lruhead, 1 as ::core::ffi::c_int);
        }
        ce = freehead;
        freehead = (*ce).hashnext as *mut connentry;
        (*ce).ip = ip;
        (*ce).port = port;
        (*ce).fd = fd;
        (*ce).lrunext = ::core::ptr::null_mut::<_connentry>();
        (*ce).lruprev = lrutail as *mut *mut _connentry;
        *lrutail = ce;
        lrutail = &raw mut (*ce).lrunext as *mut *mut connentry;
        (*ce).hashnext = conncachehash[hash as usize] as *mut _connentry;
        if !(*ce).hashnext.is_null() {
            (*(*ce).hashnext).hashprev = &raw mut (*ce).hashnext;
        }
        (*ce).hashprev = (&raw mut conncachehash as *mut *mut connentry).offset(hash as isize)
            as *mut *mut _connentry;
        conncachehash[hash as usize] = ce;
        let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut glock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    103 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    103 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    103 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    103 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    103 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    103 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn conncache_get(mut ip: uint32_t, mut port: uint16_t) -> ::core::ffi::c_int {
    unsafe {
        let mut hash: uint32_t = 0;
        let mut ce: *mut connentry = ::core::ptr::null_mut::<connentry>();
        let mut fd: ::core::ffi::c_int = 0;
        hash = hash32(ip ^ ((port as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as uint32_t)
            .wrapping_rem(CONN_CACHE_HASHSIZE as uint32_t);
        fd = -1 as ::core::ffi::c_int;
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut glock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    113 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    113 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    113 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    113 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    113 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    113 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        ce = conncachehash[hash as usize];
        while !ce.is_null() && fd == -1 as ::core::ffi::c_int {
            if (*ce).ip == ip
                && (*ce).port as ::core::ffi::c_int == port as ::core::ffi::c_int
                && (*ce).fd >= 0 as ::core::ffi::c_int
            {
                fd = (*ce).fd;
                conncache_remove(ce, 0 as ::core::ffi::c_int);
            }
            ce = (*ce).hashnext as *mut connentry;
        }
        let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut glock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    120 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    120 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    120 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    120 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    120 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    120 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
        return fd;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn conncache_keepalive_thread(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut nopbuff: [uint8_t; 8] = [0; 8];
        let mut i: ::core::ffi::c_int = 0;
        let mut ka: ::core::ffi::c_int = 0;
        let mut p: uint32_t = 0;
        let mut q: uint32_t = 0;
        let mut ce: *mut connentry = ::core::ptr::null_mut::<connentry>();
        let mut st: uint64_t = 0;
        let mut en: uint64_t = 0;
        p = 0 as uint32_t;
        ka = 1 as ::core::ffi::c_int;
        while ka != 0 {
            st = monotonic_useconds();
            let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut glock);
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
                        b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        135 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        *__errno_location(),
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        135 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        135 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        135 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        135 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_ret,
                        *__errno_location(),
                        _mfs_errorstring_err,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        135 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_ret,
                        *__errno_location(),
                        _mfs_errorstring_err,
                    );
                }
                abort();
            }
            q = p;
            while q < capacity {
                ce = conncachetab.offset(q as isize);
                if (*ce).fd >= 0 as ::core::ffi::c_int {
                    i = read(
                        (*ce).fd,
                        &raw mut nopbuff as *mut uint8_t as *mut ::core::ffi::c_void,
                        8 as size_t,
                    ) as ::core::ffi::c_int;
                    if i < 0 as ::core::ffi::c_int {
                        if !(*__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK) {
                            memset(
                                &raw mut nopbuff as *mut uint8_t as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                8 as size_t,
                            );
                            i = 8 as ::core::ffi::c_int;
                        }
                    }
                    if i != 8 as ::core::ffi::c_int {
                        conncache_remove(ce, 1 as ::core::ffi::c_int);
                    } else if nopbuff[0 as usize] as ::core::ffi::c_int
                        | nopbuff[1 as usize] as ::core::ffi::c_int
                        | nopbuff[2 as usize] as ::core::ffi::c_int
                        | nopbuff[3 as usize] as ::core::ffi::c_int
                        | nopbuff[4 as usize] as ::core::ffi::c_int
                        | nopbuff[5 as usize] as ::core::ffi::c_int
                        | nopbuff[6 as usize] as ::core::ffi::c_int
                        | nopbuff[7 as usize] as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int
                    {
                        conncache_remove(ce, 1 as ::core::ffi::c_int);
                    } else {
                        memset(
                            &raw mut nopbuff as *mut uint8_t as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            8 as size_t,
                        );
                        i = write(
                            (*ce).fd,
                            &raw mut nopbuff as *mut uint8_t as *const ::core::ffi::c_void,
                            8 as size_t,
                        ) as ::core::ffi::c_int;
                        if i != 8 as ::core::ffi::c_int {
                            conncache_remove(ce, 1 as ::core::ffi::c_int);
                        }
                    }
                }
                q = q.wrapping_add(200 as uint32_t);
            }
            p = p.wrapping_add(1);
            if p >= 200 as uint32_t {
                p = 0 as uint32_t;
            }
            ka = keep_alive;
            let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut glock);
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
                        b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        167 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        167 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        167 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        167 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        167 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        167 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                }
                abort();
            }
            en = monotonic_useconds();
            if en.wrapping_sub(st) < 5000 as uint64_t {
                portable_usleep((5000 as uint64_t).wrapping_sub(en.wrapping_sub(st)));
            }
        }
        return arg;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn conncache_term() {
    unsafe {
        let mut p: uint32_t = 0;
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut glock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    179 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    179 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    179 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    179 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    179 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    179 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        keep_alive = 0 as ::core::ffi::c_int;
        let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut glock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
        pthread_join(
            main_thread,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        );
        let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_lock(&raw mut glock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    183 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    183 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    183 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    183 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    183 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    183 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
            }
            abort();
        }
        p = 0 as uint32_t;
        while p < capacity {
            if (*conncachetab.offset(p as isize)).fd >= 0 as ::core::ffi::c_int {
                tcpclose((*conncachetab.offset(p as isize)).fd);
            }
            p = p.wrapping_add(1);
        }
        free(conncachetab as *mut ::core::ffi::c_void);
        let mut _mfs_assert_ret_2: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut glock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    191 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_5,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    191 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    191 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_6,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    191 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    191 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    191 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_3: ::core::ffi::c_int = pthread_mutex_destroy(&raw mut glock);
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    192 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_7,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    192 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    192 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_8,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    192 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    192 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_err_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/../mfscommon/conncache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    192 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_3,
                    _mfs_errorstring_ret_3,
                    *__errno_location(),
                    _mfs_errorstring_err_3,
                );
            }
            abort();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn conncache_init(mut cap: uint32_t) -> ::core::ffi::c_int {
    unsafe {
        let mut p: uint32_t = 0;
        capacity = cap;
        conncachetab = malloc(::core::mem::size_of::<connentry>().wrapping_mul(capacity as size_t))
            as *mut connentry;
        p = 0 as uint32_t;
        while p < capacity {
            (*conncachetab.offset(p as isize)).fd = -1 as ::core::ffi::c_int;
            (*conncachetab.offset(p as isize)).lrunext = ::core::ptr::null_mut::<_connentry>();
            (*conncachetab.offset(p as isize)).lruprev = ::core::ptr::null_mut::<*mut _connentry>();
            (*conncachetab.offset(p as isize)).hashnext =
                (if p.wrapping_add(1 as uint32_t) < capacity {
                    conncachetab.offset(p.wrapping_add(1 as uint32_t) as isize)
                } else {
                    ::core::ptr::null_mut::<connentry>()
                }) as *mut _connentry;
            (*conncachetab.offset(p as isize)).hashprev =
                ::core::ptr::null_mut::<*mut _connentry>();
            p = p.wrapping_add(1);
        }
        freehead = conncachetab;
        p = 0 as uint32_t;
        while p < CONN_CACHE_HASHSIZE as uint32_t {
            conncachehash[p as usize] = ::core::ptr::null_mut::<connentry>();
            p = p.wrapping_add(1);
        }
        lruhead = ::core::ptr::null_mut::<connentry>();
        lrutail = &raw mut lruhead;
        keep_alive = 1 as ::core::ffi::c_int;
        if lwt_minthread_create(
            &raw mut main_thread,
            0 as uint8_t,
            Some(
                conncache_keepalive_thread
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
            ),
            NULL,
        ) < 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        return 1 as ::core::ffi::c_int;
    }
}
