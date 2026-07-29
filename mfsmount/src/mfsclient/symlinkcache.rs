use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn stats_counter_inc(node: *mut ::core::ffi::c_void);
    fn stats_counter_dec(node: *mut ::core::ffi::c_void);
    fn stats_get_subnode(
        node: *mut ::core::ffi::c_void,
        name: *const ::core::ffi::c_char,
        absolute: uint8_t,
        printflag: uint8_t,
    ) -> *mut ::core::ffi::c_void;
    fn monotonic_seconds() -> ::core::ffi::c_double;
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
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
pub type uint8_t = u8;
pub type uint32_t = u32;
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
pub struct _hashbucket {
    pub inode: [uint32_t; 16],
    pub time: [::core::ffi::c_double; 16],
    pub path: [*mut uint8_t; 16],
}
pub type hashbucket = _hashbucket;
pub type C2Rust_Unnamed_0 = ::core::ffi::c_uint;
pub const STATNODES: C2Rust_Unnamed_0 = 4;
pub const LINKS: C2Rust_Unnamed_0 = 3;
pub const SEARCH_MISSES: C2Rust_Unnamed_0 = 2;
pub const SEARCH_HITS: C2Rust_Unnamed_0 = 1;
pub const INSERTS: C2Rust_Unnamed_0 = 0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const HASH_FUNCTIONS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const HASH_BUCKET_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const HASH_BUCKETS: ::core::ffi::c_int = 6257 as ::core::ffi::c_int;
static mut primes: [uint32_t; 4] = [
    1072573589 as uint32_t,
    3465827623 as uint32_t,
    2848548977 as uint32_t,
    748191707 as uint32_t,
];
static mut symlinkhash: *mut hashbucket = ::core::ptr::null_mut::<hashbucket>();
static mut slcachelock: pthread_mutex_t = pthread_mutex_t {
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
static mut timeout: ::core::ffi::c_double = 0.;
static mut statsptr: [*mut ::core::ffi::c_void; 4] =
    [::core::ptr::null_mut::<::core::ffi::c_void>(); 4];
#[inline]
unsafe extern "C" fn symlink_cache_statsptr_init() {
    let mut s: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    s = stats_get_subnode(
        NULL,
        b"symlink_cache\0".as_ptr() as *const ::core::ffi::c_char,
        0 as uint8_t,
        0 as uint8_t,
    );
    statsptr[INSERTS as ::core::ffi::c_int as usize] = stats_get_subnode(
        s,
        b"inserts\0".as_ptr() as *const ::core::ffi::c_char,
        0 as uint8_t,
        1 as uint8_t,
    );
    statsptr[SEARCH_HITS as ::core::ffi::c_int as usize] = stats_get_subnode(
        s,
        b"search_hits\0".as_ptr() as *const ::core::ffi::c_char,
        0 as uint8_t,
        1 as uint8_t,
    );
    statsptr[SEARCH_MISSES as ::core::ffi::c_int as usize] = stats_get_subnode(
        s,
        b"search_misses\0".as_ptr() as *const ::core::ffi::c_char,
        0 as uint8_t,
        1 as uint8_t,
    );
    statsptr[LINKS as ::core::ffi::c_int as usize] = stats_get_subnode(
        s,
        b"#links\0".as_ptr() as *const ::core::ffi::c_char,
        1 as uint8_t,
        1 as uint8_t,
    );
}
#[inline]
unsafe extern "C" fn symlink_cache_stats_inc(mut id: uint8_t) {
    if (id as ::core::ffi::c_int) < STATNODES as ::core::ffi::c_int {
        stats_counter_inc(statsptr[id as usize]);
    }
}
#[inline]
unsafe extern "C" fn symlink_cache_stats_dec(mut id: uint8_t) {
    if (id as ::core::ffi::c_int) < STATNODES as ::core::ffi::c_int {
        stats_counter_dec(statsptr[id as usize]);
    }
}
#[no_mangle]
pub unsafe extern "C" fn symlink_cache_insert(mut inode: uint32_t, mut path: *const uint8_t) {
    let mut hb: *mut hashbucket = ::core::ptr::null_mut::<hashbucket>();
    let mut fhb: *mut hashbucket = ::core::ptr::null_mut::<hashbucket>();
    let mut h: uint8_t = 0;
    let mut i: uint8_t = 0;
    let mut fi: uint8_t = 0;
    let mut t: ::core::ffi::c_double = 0.;
    let mut mint: ::core::ffi::c_double = 0.;
    t = monotonic_seconds();
    mint = t;
    fi = 0 as uint8_t;
    fhb = ::core::ptr::null_mut::<hashbucket>();
    symlink_cache_stats_inc(INSERTS as ::core::ffi::c_int as uint8_t);
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut slcachelock);
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
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                98 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                98 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                98 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                98 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                98 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                98 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    h = 0 as uint8_t;
    while (h as ::core::ffi::c_int) < HASH_FUNCTIONS {
        hb = symlinkhash.offset(
            inode
                .wrapping_mul(primes[h as usize])
                .wrapping_rem(HASH_BUCKETS as uint32_t) as isize,
        );
        i = 0 as uint8_t;
        while (i as ::core::ffi::c_int) < HASH_BUCKET_SIZE {
            if (*hb).inode[i as usize] == inode {
                if !(*hb).path[i as usize].is_null() {
                    free((*hb).path[i as usize] as *mut ::core::ffi::c_void);
                }
                (*hb).path[i as usize] = strdup(path as *const ::core::ffi::c_char) as *mut uint8_t;
                (*hb).time[i as usize] = t;
                let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                    pthread_mutex_unlock(&raw mut slcachelock);
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
                            b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            108 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&slcachelock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_1,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            108 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&slcachelock)\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            108 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&slcachelock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_2,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            108 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&slcachelock)\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            108 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&slcachelock)\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            108 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&slcachelock)\0".as_ptr()
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
            if (*hb).time[i as usize] < mint {
                fhb = hb;
                fi = i;
                mint = (*hb).time[i as usize];
            }
            i = i.wrapping_add(1);
        }
        h = h.wrapping_add(1);
    }
    if !fhb.is_null() {
        if (*fhb).time[fi as usize] == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            symlink_cache_stats_inc(LINKS as ::core::ffi::c_int as uint8_t);
        }
        if !(*fhb).path[fi as usize].is_null() {
            free((*fhb).path[fi as usize] as *mut ::core::ffi::c_void);
        }
        (*fhb).inode[fi as usize] = inode;
        (*fhb).path[fi as usize] = strdup(path as *const ::core::ffi::c_char) as *mut uint8_t;
        (*fhb).time[fi as usize] = t;
    }
    let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut slcachelock);
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
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                129 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                129 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                129 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                129 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                129 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                129 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn symlink_cache_search(mut inode: uint32_t) -> *mut uint8_t {
    let mut hb: *mut hashbucket = ::core::ptr::null_mut::<hashbucket>();
    let mut h: uint8_t = 0;
    let mut i: uint8_t = 0;
    let mut t: ::core::ffi::c_double = 0.;
    let mut path: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    t = monotonic_seconds();
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut slcachelock);
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
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                140 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                140 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                140 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                140 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                140 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                140 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    h = 0 as uint8_t;
    while (h as ::core::ffi::c_int) < HASH_FUNCTIONS {
        hb = symlinkhash.offset(
            inode
                .wrapping_mul(primes[h as usize])
                .wrapping_rem(HASH_BUCKETS as uint32_t) as isize,
        );
        i = 0 as uint8_t;
        while (i as ::core::ffi::c_int) < HASH_BUCKET_SIZE {
            if (*hb).inode[i as usize] == inode {
                if (*hb).time[i as usize] + timeout < t {
                    if !(*hb).path[i as usize].is_null() {
                        free((*hb).path[i as usize] as *mut ::core::ffi::c_void);
                        (*hb).path[i as usize] = ::core::ptr::null_mut::<uint8_t>();
                    }
                    (*hb).time[i as usize] = 0.0f64;
                    (*hb).inode[i as usize] = 0 as uint32_t;
                    let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                        pthread_mutex_unlock(&raw mut slcachelock);
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
                                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                152 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&slcachelock)\0".as_ptr()
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
                                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                152 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&slcachelock)\0".as_ptr()
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
                                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                152 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&slcachelock)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_0,
                                _mfs_errorstring_2,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                152 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&slcachelock)\0".as_ptr()
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
                                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                152 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&slcachelock)\0".as_ptr()
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
                                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                152 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"pthread_mutex_unlock(&slcachelock)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                _mfs_assert_ret_0,
                                _mfs_errorstring_ret_0,
                                *__errno_location(),
                                _mfs_errorstring_err_0,
                            );
                        }
                        abort();
                    }
                    symlink_cache_stats_dec(LINKS as ::core::ffi::c_int as uint8_t);
                    symlink_cache_stats_inc(SEARCH_MISSES as ::core::ffi::c_int as uint8_t);
                    return ::core::ptr::null_mut::<uint8_t>();
                }
                path = strdup((*hb).path[i as usize] as *const ::core::ffi::c_char) as *mut uint8_t;
                let mut _mfs_assert_ret_1: ::core::ffi::c_int =
                    pthread_mutex_unlock(&raw mut slcachelock);
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
                            b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            158 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&slcachelock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_1,
                            *__errno_location(),
                            _mfs_errorstring_3,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            158 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&slcachelock)\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            158 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&slcachelock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_1,
                            _mfs_errorstring_4,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            158 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&slcachelock)\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            158 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&slcachelock)\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            158 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&slcachelock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_1,
                            _mfs_errorstring_ret_1,
                            *__errno_location(),
                            _mfs_errorstring_err_1,
                        );
                    }
                    abort();
                }
                symlink_cache_stats_inc(SEARCH_HITS as ::core::ffi::c_int as uint8_t);
                return path;
            }
            i = i.wrapping_add(1);
        }
        h = h.wrapping_add(1);
    }
    let mut _mfs_assert_ret_2: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut slcachelock);
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
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                164 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                *__errno_location(),
                _mfs_errorstring_5,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                164 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                164 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_6,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                164 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                164 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_ret_2,
                *__errno_location(),
                _mfs_errorstring_err_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                164 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_ret_2,
                *__errno_location(),
                _mfs_errorstring_err_2,
            );
        }
        abort();
    }
    symlink_cache_stats_inc(SEARCH_MISSES as ::core::ffi::c_int as uint8_t);
    return ::core::ptr::null_mut::<uint8_t>();
}
#[no_mangle]
pub unsafe extern "C" fn symlink_cache_init(mut to: ::core::ffi::c_double) {
    let mut hb: *mut hashbucket = ::core::ptr::null_mut::<hashbucket>();
    let mut i: uint8_t = 0;
    let mut hi: uint32_t = 0;
    symlinkhash = malloc(::core::mem::size_of::<hashbucket>().wrapping_mul(HASH_BUCKETS as size_t))
        as *mut hashbucket;
    if symlinkhash.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr() as *const ::core::ffi::c_char,
            175 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"symlinkhash\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr() as *const ::core::ffi::c_char,
            175 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"symlinkhash\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if symlinkhash
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut hashbucket
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr() as *const ::core::ffi::c_char,
            175 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"symlinkhash\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr() as *const ::core::ffi::c_char,
            175 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"symlinkhash\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    hi = 0 as uint32_t;
    while hi < HASH_BUCKETS as uint32_t {
        hb = symlinkhash.offset(hi as isize);
        i = 0 as uint8_t;
        while (i as ::core::ffi::c_int) < HASH_BUCKET_SIZE {
            (*hb).inode[i as usize] = 0 as uint32_t;
            (*hb).time[i as usize] = 0.0f64;
            (*hb).path[i as usize] = ::core::ptr::null_mut::<uint8_t>();
            i = i.wrapping_add(1);
        }
        hi = hi.wrapping_add(1);
    }
    timeout = to;
    symlink_cache_statsptr_init();
}
#[no_mangle]
pub unsafe extern "C" fn symlink_cache_term() {
    let mut hb: *mut hashbucket = ::core::ptr::null_mut::<hashbucket>();
    let mut i: uint8_t = 0;
    let mut hi: uint32_t = 0;
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut slcachelock);
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
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    hi = 0 as uint32_t;
    while hi < HASH_BUCKETS as uint32_t {
        hb = symlinkhash.offset(hi as isize);
        i = 0 as uint8_t;
        while (i as ::core::ffi::c_int) < HASH_BUCKET_SIZE {
            if !(*hb).path[i as usize].is_null() {
                free((*hb).path[i as usize] as *mut ::core::ffi::c_void);
            }
            (*hb).path[i as usize] = ::core::ptr::null_mut::<uint8_t>();
            i = i.wrapping_add(1);
        }
        hi = hi.wrapping_add(1);
    }
    free(symlinkhash as *mut ::core::ffi::c_void);
    symlinkhash = ::core::ptr::null_mut::<hashbucket>();
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut slcachelock);
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
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                205 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                205 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                205 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                205 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                205 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/symlinkcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                205 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&slcachelock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
}
