pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    unsafe fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    unsafe fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
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
pub type C2Rust_Unnamed_0 = ::core::ffi::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2Rust_Unnamed_0 = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2Rust_Unnamed_0 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2Rust_Unnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2Rust_Unnamed_0 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2Rust_Unnamed_0 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2Rust_Unnamed_0 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2Rust_Unnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2Rust_Unnamed_0 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2Rust_Unnamed_0 = 0;
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
pub struct chunkrec {
    pub inode: uint32_t,
    pub indx: uint32_t,
    pub writing: uint8_t,
    pub active_readers_cnt: uint32_t,
    pub waiting_readers_cnt: uint32_t,
    pub waiting_writers_cnt: uint32_t,
    pub rcond: pthread_cond_t,
    pub wcond: pthread_cond_t,
    pub next: *mut chunkrec,
    pub prev: *mut *mut chunkrec,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
static mut hashtab: [*mut chunkrec; 1024] = [::core::ptr::null_mut::<chunkrec>(); 1024];
static mut freeblocks: *mut chunkrec = ::core::ptr::null_mut::<chunkrec>();
static mut freeblockscnt: uint32_t = 0;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunkrwlock_init() {
    unsafe {
        let mut i: uint32_t = 0;
        pthread_mutex_lock(&raw mut glock);
        i = 0 as uint32_t;
        while i < 1024 as uint32_t {
            hashtab[i as usize] = ::core::ptr::null_mut::<chunkrec>();
            i = i.wrapping_add(1);
        }
        freeblocks = ::core::ptr::null_mut::<chunkrec>();
        freeblockscnt = 0 as uint32_t;
        pthread_mutex_unlock(&raw mut glock);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunkrwlock_term() {
    unsafe {
        let mut i: uint32_t = 0;
        let mut cr: *mut chunkrec = ::core::ptr::null_mut::<chunkrec>();
        pthread_mutex_lock(&raw mut glock);
        while !freeblocks.is_null() {
            cr = freeblocks;
            let mut _mfs_assert_ret: ::core::ffi::c_int =
                pthread_cond_destroy(&raw mut (*cr).rcond);
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        41 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_destroy(&(cr->rcond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        *__errno_location(),
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        41 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_destroy(&(cr->rcond))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        41 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_destroy(&(cr->rcond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        41 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_destroy(&(cr->rcond))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        41 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_destroy(&(cr->rcond))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        41 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_destroy(&(cr->rcond))\0".as_ptr()
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
                pthread_cond_destroy(&raw mut (*cr).wcond);
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        42 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_destroy(&(cr->wcond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        42 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_destroy(&(cr->wcond))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        42 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_destroy(&(cr->wcond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        42 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_destroy(&(cr->wcond))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        42 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_destroy(&(cr->wcond))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        42 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_destroy(&(cr->wcond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                }
                abort();
            }
            freeblocks = (*cr).next as *mut chunkrec;
            free(cr as *mut ::core::ffi::c_void);
        }
        i = 0 as uint32_t;
        while i < 1024 as uint32_t {
            if hashtab[i as usize].is_null() {
            } else {
                fprintf(
                    stderr,
                    b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    47 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"hashtab[i]==NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"chunkrwlock hashmap not empty during termination\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - failed assertion '%s' : %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    47 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"hashtab[i]==NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"chunkrwlock hashmap not empty during termination\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                abort();
            };
            i = i.wrapping_add(1);
        }
        pthread_mutex_unlock(&raw mut glock);
    }
}
#[inline]
unsafe extern "C" fn chunkrwlock_get(mut inode: uint32_t, mut indx: uint32_t) -> *mut chunkrec {
    unsafe {
        let mut cr: *mut chunkrec = ::core::ptr::null_mut::<chunkrec>();
        let mut hash: uint32_t = 0;
        pthread_mutex_lock(&raw mut glock);
        hash = inode
            .wrapping_mul(0xf52d as uint32_t)
            .wrapping_add(indx ^ 0x423 as uint32_t)
            & 1023 as uint32_t;
        cr = hashtab[hash as usize];
        while !cr.is_null() {
            if (*cr).inode == inode && (*cr).indx == indx {
                return cr;
            }
            cr = (*cr).next as *mut chunkrec;
        }
        if freeblocks.is_null() {
            cr = malloc(::core::mem::size_of::<chunkrec>()) as *mut chunkrec;
            if cr.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    64 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"cr\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    64 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"cr\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if cr
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut chunkrec
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    64 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"cr\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    64 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"cr\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_cond_init(
                &raw mut (*cr).rcond,
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        65 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_init(&(cr->rcond),NULL)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        *__errno_location(),
                        _mfs_errorstring_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        65 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_init(&(cr->rcond),NULL)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        *__errno_location(),
                        _mfs_errorstring_0,
                    );
                } else if _mfs_assert_ret > 0 as ::core::ffi::c_int
                    && *__errno_location() == 0 as ::core::ffi::c_int
                {
                    let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                        strerr(_mfs_assert_ret);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        65 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_init(&(cr->rcond),NULL)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        65 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_init(&(cr->rcond),NULL)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_1,
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        65 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_init(&(cr->rcond),NULL)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        65 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_init(&(cr->rcond),NULL)\0".as_ptr()
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
                &raw mut (*cr).wcond,
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        66 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_init(&(cr->wcond),NULL)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        66 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_init(&(cr->wcond),NULL)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        66 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_init(&(cr->wcond),NULL)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_3,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        66 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_init(&(cr->wcond),NULL)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        66 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_init(&(cr->wcond),NULL)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        66 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_init(&(cr->wcond),NULL)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                }
                abort();
            }
        } else {
            cr = freeblocks;
            freeblocks = (*cr).next as *mut chunkrec;
            freeblockscnt = freeblockscnt.wrapping_sub(1);
        }
        (*cr).inode = inode;
        (*cr).indx = indx;
        (*cr).writing = 0 as uint8_t;
        (*cr).active_readers_cnt = 0 as uint32_t;
        (*cr).waiting_readers_cnt = 0 as uint32_t;
        (*cr).waiting_writers_cnt = 0 as uint32_t;
        (*cr).prev =
            (&raw mut hashtab as *mut *mut chunkrec).offset(hash as isize) as *mut *mut chunkrec;
        (*cr).next = hashtab[hash as usize] as *mut chunkrec;
        if !(*cr).next.is_null() {
            (*(*cr).next).prev = &raw mut (*cr).next;
        }
        hashtab[hash as usize] = cr;
        return cr;
    }
}
#[inline]
unsafe extern "C" fn chunkrwlock_release(mut cr: *mut chunkrec) {
    unsafe {
        if (*cr).writing as uint32_t
            | (*cr).active_readers_cnt
            | (*cr).waiting_readers_cnt
            | (*cr).waiting_writers_cnt
            == 0 as uint32_t
        {
            *(*cr).prev = (*cr).next;
            if !(*cr).next.is_null() {
                (*(*cr).next).prev = (*cr).prev;
            }
            if freeblockscnt > 1024 as uint32_t {
                let mut _mfs_assert_ret: ::core::ffi::c_int =
                    pthread_cond_destroy(&raw mut (*cr).rcond);
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
                            b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            94 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_destroy(&(cr->rcond))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret,
                            *__errno_location(),
                            _mfs_errorstring,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            94 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_destroy(&(cr->rcond))\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            94 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_destroy(&(cr->rcond))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret,
                            _mfs_errorstring_0,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            94 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_destroy(&(cr->rcond))\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            94 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_destroy(&(cr->rcond))\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            94 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_destroy(&(cr->rcond))\0".as_ptr()
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
                    pthread_cond_destroy(&raw mut (*cr).wcond);
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
                            b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            95 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_destroy(&(cr->wcond))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_1,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            95 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_destroy(&(cr->wcond))\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            95 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_destroy(&(cr->wcond))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_2,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            95 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_destroy(&(cr->wcond))\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            95 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_destroy(&(cr->wcond))\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            95 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_cond_destroy(&(cr->wcond))\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_err_0,
                        );
                    }
                    abort();
                }
                free(cr as *mut ::core::ffi::c_void);
            } else {
                (*cr).prev = ::core::ptr::null_mut::<*mut chunkrec>();
                (*cr).next = freeblocks as *mut chunkrec;
                freeblocks = cr;
                freeblockscnt = freeblockscnt.wrapping_add(1);
            }
        }
        pthread_mutex_unlock(&raw mut glock);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunkrwlock_rlock(mut inode: uint32_t, mut indx: uint32_t) {
    unsafe {
        let mut cr: *mut chunkrec = ::core::ptr::null_mut::<chunkrec>();
        cr = chunkrwlock_get(inode, indx);
        (*cr).waiting_readers_cnt = (*cr).waiting_readers_cnt.wrapping_add(1);
        while (*cr).writing as uint32_t | (*cr).waiting_writers_cnt != 0 {
            let mut _mfs_assert_ret: ::core::ffi::c_int =
                pthread_cond_wait(&raw mut (*cr).rcond, &raw mut glock);
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        113 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(cr->rcond),&glock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        *__errno_location(),
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        113 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(cr->rcond),&glock)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        113 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(cr->rcond),&glock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        113 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(cr->rcond),&glock)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        113 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(cr->rcond),&glock)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        113 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(cr->rcond),&glock)\0".as_ptr()
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
        (*cr).waiting_readers_cnt = (*cr).waiting_readers_cnt.wrapping_sub(1);
        (*cr).active_readers_cnt = (*cr).active_readers_cnt.wrapping_add(1);
        chunkrwlock_release(cr);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunkrwlock_runlock(mut inode: uint32_t, mut indx: uint32_t) {
    unsafe {
        let mut cr: *mut chunkrec = ::core::ptr::null_mut::<chunkrec>();
        cr = chunkrwlock_get(inode, indx);
        (*cr).active_readers_cnt = (*cr).active_readers_cnt.wrapping_sub(1);
        if (*cr).active_readers_cnt == 0 as uint32_t && (*cr).waiting_writers_cnt != 0 {
            let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_cond_signal(&raw mut (*cr).wcond);
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        126 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&(cr->wcond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        *__errno_location(),
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        126 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&(cr->wcond))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        126 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&(cr->wcond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        126 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&(cr->wcond))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        126 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&(cr->wcond))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        126 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&(cr->wcond))\0".as_ptr()
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
        chunkrwlock_release(cr);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunkrwlock_wlock(mut inode: uint32_t, mut indx: uint32_t) {
    unsafe {
        let mut cr: *mut chunkrec = ::core::ptr::null_mut::<chunkrec>();
        cr = chunkrwlock_get(inode, indx);
        (*cr).waiting_writers_cnt = (*cr).waiting_writers_cnt.wrapping_add(1);
        while (*cr).active_readers_cnt | (*cr).writing as uint32_t != 0 {
            let mut _mfs_assert_ret: ::core::ffi::c_int =
                pthread_cond_wait(&raw mut (*cr).wcond, &raw mut glock);
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        137 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(cr->wcond),&glock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        *__errno_location(),
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        137 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(cr->wcond),&glock)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        137 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(cr->wcond),&glock)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        137 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(cr->wcond),&glock)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        137 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(cr->wcond),&glock)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        137 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_wait(&(cr->wcond),&glock)\0".as_ptr()
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
        (*cr).waiting_writers_cnt = (*cr).waiting_writers_cnt.wrapping_sub(1);
        (*cr).writing = 1 as uint8_t;
        chunkrwlock_release(cr);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunkrwlock_wunlock(mut inode: uint32_t, mut indx: uint32_t) {
    unsafe {
        let mut cr: *mut chunkrec = ::core::ptr::null_mut::<chunkrec>();
        cr = chunkrwlock_get(inode, indx);
        (*cr).writing = 0 as uint8_t;
        if (*cr).waiting_writers_cnt != 0 {
            let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_cond_signal(&raw mut (*cr).wcond);
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        150 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&(cr->wcond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        *__errno_location(),
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        150 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&(cr->wcond))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        150 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&(cr->wcond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        150 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&(cr->wcond))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        150 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&(cr->wcond))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        150 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_signal(&(cr->wcond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret,
                        _mfs_errorstring_ret,
                        *__errno_location(),
                        _mfs_errorstring_err,
                    );
                }
                abort();
            }
        } else if (*cr).waiting_readers_cnt != 0 {
            let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                pthread_cond_broadcast(&raw mut (*cr).rcond);
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        152 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_broadcast(&(cr->rcond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        152 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_broadcast(&(cr->rcond))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        152 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_broadcast(&(cr->rcond))\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        152 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_broadcast(&(cr->rcond))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        152 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_broadcast(&(cr->rcond))\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/chunkrwlock.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        152 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_cond_broadcast(&(cr->rcond))\0".as_ptr()
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
        chunkrwlock_release(cr);
    }
}
