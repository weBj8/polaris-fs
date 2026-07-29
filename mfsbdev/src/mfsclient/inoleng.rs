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
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
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
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type memory_order = ::core::ffi::c_uint;
pub const memory_order_seq_cst: memory_order = 5;
pub const memory_order_acq_rel: memory_order = 4;
pub const memory_order_release: memory_order = 3;
pub const memory_order_acquire: memory_order = 2;
pub const memory_order_consume: memory_order = 1;
pub const memory_order_relaxed: memory_order = 0;
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
pub struct _ileng {
    pub inode: uint32_t,
    pub refcnt: uint32_t,
    pub fleng: uint64_t,
    pub writing: uint8_t,
    pub readers_cnt: uint32_t,
    pub writers_cnt: uint32_t,
    pub rwlock: pthread_mutex_t,
    pub rwcond: pthread_cond_t,
    pub next: *mut _ileng,
}
pub type ileng = _ileng;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ileng_bucket {
    pub bucket: [ileng; 500],
    pub firstfree: uint32_t,
    pub next: *mut _ileng_bucket,
}
pub type ileng_bucket = _ileng_bucket;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INOLENG_HASHSIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
static mut ileng_used: uint64_t = 0 as uint64_t;
static mut ileng_lock: pthread_mutex_t = pthread_mutex_t {
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
unsafe extern "C" fn ileng_free_all() {
    let mut srb: *mut ileng_bucket = ::core::ptr::null_mut::<ileng_bucket>();
    let mut nsrb: *mut ileng_bucket = ::core::ptr::null_mut::<ileng_bucket>();
    pthread_mutex_lock(&raw mut ileng_lock);
    srb = ileng_buckets_head;
    while !srb.is_null() {
        nsrb = (*srb).next as *mut ileng_bucket;
        free(srb as *mut ::core::ffi::c_void);
        srb = nsrb;
    }
    ileng_buckets_head = ::core::ptr::null_mut::<ileng_bucket>();
    ileng_free_head = NULL;
    ileng_allocated = 0 as uint64_t;
    ileng_used = 0 as uint64_t;
    pthread_mutex_unlock(&raw mut ileng_lock);
}
#[inline]
unsafe extern "C" fn ileng_malloc() -> *mut ileng {
    let mut srb: *mut ileng_bucket = ::core::ptr::null_mut::<ileng_bucket>();
    let mut ret: *mut ileng = ::core::ptr::null_mut::<ileng>();
    pthread_mutex_lock(&raw mut ileng_lock);
    if !ileng_free_head.is_null() {
        ret = ileng_free_head as *mut ileng;
        ileng_free_head = *(ret as *mut *mut ::core::ffi::c_void);
        ileng_used = (ileng_used as ::core::ffi::c_ulong)
            .wrapping_add(::core::mem::size_of::<ileng>() as ::core::ffi::c_ulong)
            as uint64_t;
        pthread_mutex_unlock(&raw mut ileng_lock);
        return ret;
    }
    if ileng_buckets_head.is_null() || (*ileng_buckets_head).firstfree == 500 as uint32_t {
        srb = malloc(::core::mem::size_of::<ileng_bucket>()) as *mut ileng_bucket;
        if srb.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                75 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"srb\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                75 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"srb\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if srb
            == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut ileng_bucket
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                75 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                75 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*srb).next = ileng_buckets_head as *mut _ileng_bucket;
        (*srb).firstfree = 0 as uint32_t;
        ileng_buckets_head = srb;
        ileng_allocated = (ileng_allocated as ::core::ffi::c_ulong)
            .wrapping_add(::core::mem::size_of::<ileng_bucket>() as ::core::ffi::c_ulong)
            as uint64_t;
    }
    ret = (&raw mut (*ileng_buckets_head).bucket as *mut ileng)
        .offset((*ileng_buckets_head).firstfree as isize);
    (*ileng_buckets_head).firstfree = (*ileng_buckets_head).firstfree.wrapping_add(1);
    ileng_used = (ileng_used as ::core::ffi::c_ulong)
        .wrapping_add(::core::mem::size_of::<ileng>() as ::core::ffi::c_ulong)
        as uint64_t;
    pthread_mutex_unlock(&raw mut ileng_lock);
    return ret;
}
#[inline]
unsafe extern "C" fn ileng_free(mut p: *mut ileng) {
    pthread_mutex_lock(&raw mut ileng_lock);
    *(p as *mut *mut ::core::ffi::c_void) = ileng_free_head;
    ileng_free_head = p as *mut ::core::ffi::c_void;
    ileng_used = (ileng_used as ::core::ffi::c_ulong)
        .wrapping_sub(::core::mem::size_of::<ileng>() as ::core::ffi::c_ulong)
        as uint64_t;
    pthread_mutex_unlock(&raw mut ileng_lock);
}
#[inline]
unsafe extern "C" fn ileng_getusage(mut allocated: *mut uint64_t, mut used: *mut uint64_t) {
    pthread_mutex_lock(&raw mut ileng_lock);
    *allocated = ileng_allocated;
    *used = ileng_used;
    pthread_mutex_unlock(&raw mut ileng_lock);
}
static mut ileng_allocated: uint64_t = 0 as uint64_t;
static mut ileng_free_head: *mut ::core::ffi::c_void = NULL;
static mut ileng_buckets_head: *mut ileng_bucket = ::core::ptr::null_mut::<ileng_bucket>();
static mut inolenghashtab: [*mut ileng; 1024] = [::core::ptr::null_mut::<ileng>(); 1024];
static mut hashlock: [pthread_mutex_t; 1024] = [pthread_mutex_t {
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
}; 1024];
#[no_mangle]
pub unsafe extern "C" fn inoleng_acquire(mut inode: uint32_t) -> *mut ::core::ffi::c_void {
    let mut h: uint32_t = 0;
    let mut ilptr: *mut ileng = ::core::ptr::null_mut::<ileng>();
    h = inode.wrapping_rem(INOLENG_HASHSIZE as uint32_t);
    let mut _mfs_assert_ret: ::core::ffi::c_int =
        pthread_mutex_lock((&raw mut hashlock as *mut pthread_mutex_t).offset(h as isize));
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                85 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                85 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                85 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                85 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                85 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                85 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    ilptr = inolenghashtab[h as usize];
    while !ilptr.is_null() {
        if (*ilptr).inode == inode {
            ::core::intrinsics::atomic_xadd_seqcst(&raw mut (*ilptr).refcnt, 1 as uint32_t);
            let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(
                (&raw mut hashlock as *mut pthread_mutex_t).offset(h as isize),
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
                        b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        97 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(hashlock+h)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        97 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(hashlock+h)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        97 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(hashlock+h)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        97 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(hashlock+h)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        97 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(hashlock+h)\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        97 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(hashlock+h)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_ret_0,
                        *__errno_location(),
                        _mfs_errorstring_err_0,
                    );
                }
                abort();
            }
            return ilptr as *mut ::core::ffi::c_void;
        }
        ilptr = (*ilptr).next as *mut ileng;
    }
    ilptr = ileng_malloc();
    (*ilptr).inode = inode;
    *&raw mut (*ilptr).refcnt = 1 as uint32_t;
    *&raw mut (*ilptr).fleng = 0 as uint64_t;
    (*ilptr).writing = 0 as uint8_t;
    (*ilptr).writers_cnt = 0 as uint32_t;
    (*ilptr).readers_cnt = 0 as uint32_t;
    let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_init(
        &raw mut (*ilptr).rwlock,
        ::core::ptr::null::<pthread_mutexattr_t>(),
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                116 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(ilptr->rwlock),NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                116 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(ilptr->rwlock),NULL)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                116 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(ilptr->rwlock),NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                116 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(ilptr->rwlock),NULL)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                116 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(ilptr->rwlock),NULL)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                116 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(ilptr->rwlock),NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
        }
        abort();
    }
    let mut _mfs_assert_ret_2: ::core::ffi::c_int = pthread_cond_init(
        &raw mut (*ilptr).rwcond,
        ::core::ptr::null::<pthread_condattr_t>(),
    );
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                117 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_init(&(ilptr->rwcond),NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                *__errno_location(),
                _mfs_errorstring_5,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                117 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_init(&(ilptr->rwcond),NULL)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                117 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_init(&(ilptr->rwcond),NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_6,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                117 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_init(&(ilptr->rwcond),NULL)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                117 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_init(&(ilptr->rwcond),NULL)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                117 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_init(&(ilptr->rwcond),NULL)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_ret_2,
                *__errno_location(),
                _mfs_errorstring_err_2,
            );
        }
        abort();
    }
    (*ilptr).next = inolenghashtab[h as usize] as *mut _ileng;
    inolenghashtab[h as usize] = ilptr;
    let mut _mfs_assert_ret_3: ::core::ffi::c_int =
        pthread_mutex_unlock((&raw mut hashlock as *mut pthread_mutex_t).offset(h as isize));
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                120 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                *__errno_location(),
                _mfs_errorstring_7,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                120 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                120 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                _mfs_errorstring_8,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                120 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                120 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                _mfs_errorstring_ret_3,
                *__errno_location(),
                _mfs_errorstring_err_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                120 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                _mfs_errorstring_ret_3,
                *__errno_location(),
                _mfs_errorstring_err_3,
            );
        }
        abort();
    }
    return ilptr as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn inoleng_release(mut ptr: *mut ::core::ffi::c_void) {
    let mut h: uint32_t = 0;
    let mut ilptr: *mut ileng = ::core::ptr::null_mut::<ileng>();
    let mut ilpptr: *mut *mut ileng = ::core::ptr::null_mut::<*mut ileng>();
    let mut il: *mut ileng = ptr as *mut ileng;
    if ::core::intrinsics::atomic_xsub_seqcst(&raw mut (*il).refcnt, 1 as uint32_t) == 1 as uint32_t
    {
        h = (*il).inode.wrapping_rem(INOLENG_HASHSIZE as uint32_t);
        let mut _mfs_assert_ret: ::core::ffi::c_int =
            pthread_mutex_lock((&raw mut hashlock as *mut pthread_mutex_t).offset(h as isize));
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    140 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    140 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    140 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    140 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    140 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    140 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        if ::core::intrinsics::atomic_load_seqcst(&raw mut (*il).refcnt) == 0 as uint32_t {
            ilpptr = (&raw mut inolenghashtab as *mut *mut ileng).offset(h as isize);
            loop {
                ilptr = *ilpptr;
                if ilptr.is_null() {
                    break;
                }
                if il == ilptr {
                    *ilpptr = (*ilptr).next as *mut ileng;
                    let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                        pthread_mutex_destroy(&raw mut (*ilptr).rwlock);
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
                                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                157 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_destroy(&(ilptr->rwlock))\0".as_ptr()
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
                                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                157 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_destroy(&(ilptr->rwlock))\0".as_ptr()
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
                                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                157 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_destroy(&(ilptr->rwlock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_0,
                                _mfs_errorstring_2,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                157 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_destroy(&(ilptr->rwlock))\0".as_ptr()
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
                                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                157 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_destroy(&(ilptr->rwlock))\0".as_ptr()
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
                                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                157 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_destroy(&(ilptr->rwlock))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_0,
                                _mfs_errorstring_ret_0,
                                *__errno_location(),
                                _mfs_errorstring_err_0,
                            );
                        }
                        abort();
                    }
                    let mut _mfs_assert_ret_1: ::core::ffi::c_int =
                        pthread_cond_destroy(&raw mut (*ilptr).rwcond);
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
                                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                158 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_destroy(&(ilptr->rwcond))\0".as_ptr()
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
                                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                158 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_destroy(&(ilptr->rwcond))\0".as_ptr()
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
                                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                158 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_destroy(&(ilptr->rwcond))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_1,
                                _mfs_errorstring_4,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                158 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_destroy(&(ilptr->rwcond))\0".as_ptr()
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
                                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                158 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_destroy(&(ilptr->rwcond))\0".as_ptr()
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
                                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                158 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_cond_destroy(&(ilptr->rwcond))\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_1,
                                _mfs_errorstring_ret_1,
                                *__errno_location(),
                                _mfs_errorstring_err_1,
                            );
                        }
                        abort();
                    }
                    ileng_free(ilptr);
                } else {
                    ilpptr = &raw mut (*ilptr).next as *mut *mut ileng;
                }
            }
        }
        let mut _mfs_assert_ret_2: ::core::ffi::c_int =
            pthread_mutex_unlock((&raw mut hashlock as *mut pthread_mutex_t).offset(h as isize));
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    171 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_5,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    171 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    171 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_6,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    171 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    171 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    171 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_2,
                    _mfs_errorstring_ret_2,
                    *__errno_location(),
                    _mfs_errorstring_err_2,
                );
            }
            abort();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn inoleng_getfleng(mut ptr: *mut ::core::ffi::c_void) -> uint64_t {
    let mut il: *mut ileng = ptr as *mut ileng;
    return ::core::intrinsics::atomic_load_relaxed(&raw mut (*il).fleng);
}
#[no_mangle]
pub unsafe extern "C" fn inoleng_setfleng(mut ptr: *mut ::core::ffi::c_void, mut fleng: uint64_t) {
    let mut il: *mut ileng = ptr as *mut ileng;
    ::core::intrinsics::atomic_store_relaxed(&raw mut (*il).fleng, fleng);
}
#[no_mangle]
pub unsafe extern "C" fn inoleng_update_fleng(mut inode: uint32_t, mut fleng: uint64_t) {
    let mut h: uint32_t = 0;
    let mut ilptr: *mut ileng = ::core::ptr::null_mut::<ileng>();
    h = inode.wrapping_rem(INOLENG_HASHSIZE as uint32_t);
    let mut _mfs_assert_ret: ::core::ffi::c_int =
        pthread_mutex_lock((&raw mut hashlock as *mut pthread_mutex_t).offset(h as isize));
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                219 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                219 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                219 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                219 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                219 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                219 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    ilptr = inolenghashtab[h as usize];
    while !ilptr.is_null() {
        if (*ilptr).inode == inode {
            ::core::intrinsics::atomic_store_relaxed(&raw mut (*ilptr).fleng, fleng);
        }
        ilptr = (*ilptr).next as *mut ileng;
    }
    let mut _mfs_assert_ret_0: ::core::ffi::c_int =
        pthread_mutex_unlock((&raw mut hashlock as *mut pthread_mutex_t).offset(h as isize));
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                238 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                238 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                238 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                238 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                238 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                238 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn inoleng_write_start(mut ptr: *mut ::core::ffi::c_void) {
    let mut il: *mut ileng = ptr as *mut ileng;
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*il).rwlock);
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                244 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                244 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                244 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                244 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                244 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                244 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    (*il).writers_cnt = (*il).writers_cnt.wrapping_add(1);
    while (*il).readers_cnt | (*il).writing as uint32_t != 0 {
        let mut _mfs_assert_ret_0: ::core::ffi::c_int =
            pthread_cond_wait(&raw mut (*il).rwcond, &raw mut (*il).rwlock);
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    247 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(il->rwcond),&(il->rwlock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    247 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(il->rwcond),&(il->rwlock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    247 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(il->rwcond),&(il->rwlock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    247 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(il->rwcond),&(il->rwlock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    247 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(il->rwcond),&(il->rwlock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    247 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(il->rwcond),&(il->rwlock))\0".as_ptr()
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
    (*il).writers_cnt = (*il).writers_cnt.wrapping_sub(1);
    (*il).writing = 1 as uint8_t;
    let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*il).rwlock);
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                251 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                251 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                251 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                251 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                251 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                251 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn inoleng_write_end(mut ptr: *mut ::core::ffi::c_void) {
    let mut il: *mut ileng = ptr as *mut ileng;
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*il).rwlock);
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                257 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                257 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                257 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                257 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                257 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                257 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    (*il).writing = 0 as uint8_t;
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_cond_broadcast(&raw mut (*il).rwcond);
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                259 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_broadcast(&(il->rwcond))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                259 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_broadcast(&(il->rwcond))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                259 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_broadcast(&(il->rwcond))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                259 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_broadcast(&(il->rwcond))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                259 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_broadcast(&(il->rwcond))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                259 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_cond_broadcast(&(il->rwcond))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
    let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*il).rwlock);
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                260 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                260 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                260 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                260 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                260 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                260 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn inoleng_read_start(mut ptr: *mut ::core::ffi::c_void) {
    let mut il: *mut ileng = ptr as *mut ileng;
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*il).rwlock);
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                266 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                266 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                266 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                266 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                266 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                266 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    while (*il).writing as uint32_t | (*il).writers_cnt != 0 {
        let mut _mfs_assert_ret_0: ::core::ffi::c_int =
            pthread_cond_wait(&raw mut (*il).rwcond, &raw mut (*il).rwlock);
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    268 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(il->rwcond),&(il->rwlock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    268 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(il->rwcond),&(il->rwlock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    268 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(il->rwcond),&(il->rwlock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    268 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(il->rwcond),&(il->rwlock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    268 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(il->rwcond),&(il->rwlock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    268 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(il->rwcond),&(il->rwlock))\0".as_ptr()
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
    (*il).readers_cnt = (*il).readers_cnt.wrapping_add(1);
    let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*il).rwlock);
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                271 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                271 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                271 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                271 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                271 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                271 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn inoleng_read_end(mut ptr: *mut ::core::ffi::c_void) {
    let mut il: *mut ileng = ptr as *mut ileng;
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*il).rwlock);
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                277 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                277 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                277 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                277 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                277 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                277 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    (*il).readers_cnt = (*il).readers_cnt.wrapping_sub(1);
    if (*il).readers_cnt == 0 as uint32_t {
        let mut _mfs_assert_ret_0: ::core::ffi::c_int =
            pthread_cond_broadcast(&raw mut (*il).rwcond);
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    280 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_broadcast(&(il->rwcond))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    280 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_broadcast(&(il->rwcond))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    280 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_broadcast(&(il->rwcond))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    280 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_broadcast(&(il->rwcond))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    280 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_broadcast(&(il->rwcond))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    280 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_broadcast(&(il->rwcond))\0".as_ptr()
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
    let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*il).rwlock);
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                282 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                282 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                282 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                282 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                282 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                282 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn inoleng_io_wait(mut ptr: *mut ::core::ffi::c_void) {
    let mut il: *mut ileng = ptr as *mut ileng;
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*il).rwlock);
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                288 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                288 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                288 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                288 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                288 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                288 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    while (*il).readers_cnt | (*il).writers_cnt | (*il).writing as uint32_t != 0 {
        let mut _mfs_assert_ret_0: ::core::ffi::c_int =
            pthread_cond_wait(&raw mut (*il).rwcond, &raw mut (*il).rwlock);
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    290 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(il->rwcond),&(il->rwlock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    290 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(il->rwcond),&(il->rwlock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    290 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(il->rwcond),&(il->rwlock))\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    290 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(il->rwcond),&(il->rwlock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    290 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(il->rwcond),&(il->rwlock))\0".as_ptr()
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    290 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_cond_wait(&(il->rwcond),&(il->rwlock))\0".as_ptr()
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
    let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*il).rwlock);
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                292 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                292 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                292 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                292 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                292 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr() as *const ::core::ffi::c_char,
                292 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(il->rwlock))\0".as_ptr() as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn inoleng_term() {
    let mut ilptr: *mut ileng = ::core::ptr::null_mut::<ileng>();
    let mut ilnptr: *mut ileng = ::core::ptr::null_mut::<ileng>();
    let mut refcnt: uint32_t = 0;
    let mut h: uint32_t = 0;
    h = 0 as uint32_t;
    while h < INOLENG_HASHSIZE as uint32_t {
        let mut _mfs_assert_ret: ::core::ffi::c_int =
            pthread_mutex_lock((&raw mut hashlock as *mut pthread_mutex_t).offset(h as isize));
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    301 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    301 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    301 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    301 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    301 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    301 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        ilptr = inolenghashtab[h as usize];
        while !ilptr.is_null() {
            ilnptr = (*ilptr).next as *mut ileng;
            refcnt = ::core::intrinsics::atomic_load_seqcst(&raw mut (*ilptr).refcnt);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"inode fleng data structure leftovers (ino: %u ; refcnt: %u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*ilptr).inode,
                refcnt,
            );
            ileng_free(ilptr);
            ilptr = ilnptr;
        }
        let mut _mfs_assert_ret_0: ::core::ffi::c_int =
            pthread_mutex_unlock((&raw mut hashlock as *mut pthread_mutex_t).offset(h as isize));
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    317 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    317 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    317 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    317 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    317 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    317 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_0,
                    _mfs_errorstring_ret_0,
                    *__errno_location(),
                    _mfs_errorstring_err_0,
                );
            }
            abort();
        }
        let mut _mfs_assert_ret_1: ::core::ffi::c_int =
            pthread_mutex_destroy((&raw mut hashlock as *mut pthread_mutex_t).offset(h as isize));
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    318 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    318 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    318 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    318 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    318 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    318 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_destroy(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret_1,
                    _mfs_errorstring_ret_1,
                    *__errno_location(),
                    _mfs_errorstring_err_1,
                );
            }
            abort();
        }
        h = h.wrapping_add(1);
    }
    ileng_free_all();
}
#[no_mangle]
pub unsafe extern "C" fn inoleng_init() {
    let mut h: uint32_t = 0;
    h = 0 as uint32_t;
    while h < INOLENG_HASHSIZE as uint32_t {
        inolenghashtab[h as usize] = ::core::ptr::null_mut::<ileng>();
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_init(
            (&raw mut hashlock as *mut pthread_mutex_t).offset(h as isize),
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    330 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(hashlock+h,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    330 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(hashlock+h,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    330 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(hashlock+h,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    330 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(hashlock+h,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    330 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(hashlock+h,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/inoleng.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    330 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(hashlock+h,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
            }
            abort();
        }
        h = h.wrapping_add(1);
    }
}
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
