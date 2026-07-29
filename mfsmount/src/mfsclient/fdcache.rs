use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn chunksdatacache_insert(
        inode: uint32_t,
        chindx: uint32_t,
        chunkid: uint64_t,
        version: uint32_t,
        csdataver: uint8_t,
        csdata: *const uint8_t,
        csdatasize: uint32_t,
    );
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
    fn monotonic_seconds() -> ::core::ffi::c_double;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __mode_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
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
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_ctx {
    pub uid: uid_t,
    pub gid: gid_t,
    pub pid: pid_t,
    pub umask: mode_t,
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
pub type fdcacheentry = _fdcacheentry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fdcacheentry {
    pub createtime: ::core::ffi::c_double,
    pub uid: uid_t,
    pub gid: gid_t,
    pub pid: pid_t,
    pub inode: uint32_t,
    pub attr: [uint8_t; 35],
    pub lflags: uint16_t,
    pub csdataver: uint8_t,
    pub chunkid: uint64_t,
    pub version: uint32_t,
    pub csdatasize: uint32_t,
    pub csdata: [uint8_t; 140],
    pub next: *mut _fdcacheentry,
}
pub type fdcachee_bucket = _fdcachee_bucket;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fdcachee_bucket {
    pub bucket: [fdcacheentry; 500],
    pub firstfree: uint32_t,
    pub next: *mut _fdcachee_bucket,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LOOKUP_CHUNK_ZERO_DATA: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FDCACHE_HASHSIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const FDCACHE_TIMEOUT: ::core::ffi::c_double = 1.0f64;
static mut fdcachee_lock: pthread_mutex_t = pthread_mutex_t {
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
static mut fdcachee_used: uint64_t = 0 as uint64_t;
static mut fdcachee_buckets_head: *mut fdcachee_bucket = ::core::ptr::null_mut::<fdcachee_bucket>();
static mut fdcachee_allocated: uint64_t = 0 as uint64_t;
static mut fdcachee_free_head: *mut ::core::ffi::c_void = NULL;
#[inline]
unsafe extern "C" fn fdcachee_free(mut p: *mut fdcacheentry) {
    pthread_mutex_lock(&raw mut fdcachee_lock);
    *(p as *mut *mut ::core::ffi::c_void) = fdcachee_free_head;
    fdcachee_free_head = p as *mut ::core::ffi::c_void;
    fdcachee_used = (fdcachee_used as ::core::ffi::c_ulong)
        .wrapping_sub(::core::mem::size_of::<fdcacheentry>() as ::core::ffi::c_ulong)
        as uint64_t;
    pthread_mutex_unlock(&raw mut fdcachee_lock);
}
#[inline]
unsafe extern "C" fn fdcachee_malloc() -> *mut fdcacheentry {
    let mut srb: *mut fdcachee_bucket = ::core::ptr::null_mut::<fdcachee_bucket>();
    let mut ret: *mut fdcacheentry = ::core::ptr::null_mut::<fdcacheentry>();
    pthread_mutex_lock(&raw mut fdcachee_lock);
    if !fdcachee_free_head.is_null() {
        ret = fdcachee_free_head as *mut fdcacheentry;
        fdcachee_free_head = *(ret as *mut *mut ::core::ffi::c_void);
        fdcachee_used = (fdcachee_used as ::core::ffi::c_ulong)
            .wrapping_add(::core::mem::size_of::<fdcacheentry>() as ::core::ffi::c_ulong)
            as uint64_t;
        pthread_mutex_unlock(&raw mut fdcachee_lock);
        return ret;
    }
    if fdcachee_buckets_head.is_null() || (*fdcachee_buckets_head).firstfree == 500 as uint32_t {
        srb = malloc(::core::mem::size_of::<fdcachee_bucket>()) as *mut fdcachee_bucket;
        if srb.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                59 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"srb\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                59 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"srb\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if srb
            == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut fdcachee_bucket
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                59 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                59 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*srb).next = fdcachee_buckets_head as *mut _fdcachee_bucket;
        (*srb).firstfree = 0 as uint32_t;
        fdcachee_buckets_head = srb;
        fdcachee_allocated = (fdcachee_allocated as ::core::ffi::c_ulong)
            .wrapping_add(::core::mem::size_of::<fdcachee_bucket>() as ::core::ffi::c_ulong)
            as uint64_t;
    }
    ret = (&raw mut (*fdcachee_buckets_head).bucket as *mut fdcacheentry)
        .offset((*fdcachee_buckets_head).firstfree as isize);
    (*fdcachee_buckets_head).firstfree = (*fdcachee_buckets_head).firstfree.wrapping_add(1);
    fdcachee_used = (fdcachee_used as ::core::ffi::c_ulong)
        .wrapping_add(::core::mem::size_of::<fdcacheentry>() as ::core::ffi::c_ulong)
        as uint64_t;
    pthread_mutex_unlock(&raw mut fdcachee_lock);
    return ret;
}
#[inline]
unsafe extern "C" fn fdcachee_getusage(mut allocated: *mut uint64_t, mut used: *mut uint64_t) {
    pthread_mutex_lock(&raw mut fdcachee_lock);
    *allocated = fdcachee_allocated;
    *used = fdcachee_used;
    pthread_mutex_unlock(&raw mut fdcachee_lock);
}
#[inline]
unsafe extern "C" fn fdcachee_free_all() {
    let mut srb: *mut fdcachee_bucket = ::core::ptr::null_mut::<fdcachee_bucket>();
    let mut nsrb: *mut fdcachee_bucket = ::core::ptr::null_mut::<fdcachee_bucket>();
    pthread_mutex_lock(&raw mut fdcachee_lock);
    srb = fdcachee_buckets_head;
    while !srb.is_null() {
        nsrb = (*srb).next as *mut fdcachee_bucket;
        free(srb as *mut ::core::ffi::c_void);
        srb = nsrb;
    }
    fdcachee_buckets_head = ::core::ptr::null_mut::<fdcachee_bucket>();
    fdcachee_free_head = NULL;
    fdcachee_allocated = 0 as uint64_t;
    fdcachee_used = 0 as uint64_t;
    pthread_mutex_unlock(&raw mut fdcachee_lock);
}
static mut fdhashtab: [*mut fdcacheentry; 1024] = [::core::ptr::null_mut::<fdcacheentry>(); 1024];
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
pub unsafe extern "C" fn fdcache_insert(
    mut ctx: *const fuse_ctx,
    mut inode: uint32_t,
    mut attr: *mut uint8_t,
    mut lflags: uint16_t,
    mut csdataver: uint8_t,
    mut chunkid: uint64_t,
    mut version: uint32_t,
    mut csdata: *const uint8_t,
    mut csdatasize: uint32_t,
) {
    let mut h: uint32_t = 0;
    let mut now: ::core::ffi::c_double = 0.;
    let mut f: *mut fdcacheentry = ::core::ptr::null_mut::<fdcacheentry>();
    let mut fdce: *mut fdcacheentry = ::core::ptr::null_mut::<fdcacheentry>();
    let mut fdcep: *mut *mut fdcacheentry = ::core::ptr::null_mut::<*mut fdcacheentry>();
    now = monotonic_seconds();
    h = inode.wrapping_rem(FDCACHE_HASHSIZE as uint32_t);
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                72 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                72 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                72 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                72 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                72 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                72 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    fdcep = (&raw mut fdhashtab as *mut *mut fdcacheentry).offset(h as isize);
    f = ::core::ptr::null_mut::<fdcacheentry>();
    loop {
        fdce = *fdcep;
        if fdce.is_null() {
            break;
        }
        if (*fdce).createtime + FDCACHE_TIMEOUT < now {
            *fdcep = (*fdce).next as *mut fdcacheentry;
            fdcachee_free(fdce);
        } else if (*fdce).inode == inode
            && (*fdce).uid == (*ctx).uid
            && (*fdce).gid == (*ctx).gid
            && (*fdce).pid == (*ctx).pid
        {
            if f.is_null() {
                f = fdce;
                fdcep = &raw mut (*fdce).next as *mut *mut fdcacheentry;
            } else {
                *fdcep = (*fdce).next as *mut fdcacheentry;
                fdcachee_free(fdce);
            }
        } else {
            fdcep = &raw mut (*fdce).next as *mut *mut fdcacheentry;
        }
    }
    if f.is_null() {
        fdce = fdcachee_malloc();
        (*fdce).uid = (*ctx).uid;
        (*fdce).gid = (*ctx).gid;
        (*fdce).pid = (*ctx).pid;
        (*fdce).inode = inode;
        (*fdce).next = fdhashtab[h as usize] as *mut _fdcacheentry;
        fdhashtab[h as usize] = fdce;
    } else {
        fdce = f;
    }
    (*fdce).createtime = now;
    memcpy(
        &raw mut (*fdce).attr as *mut uint8_t as *mut ::core::ffi::c_void,
        attr as *const ::core::ffi::c_void,
        35 as size_t,
    );
    (*fdce).lflags = lflags;
    if lflags as ::core::ffi::c_int & LOOKUP_CHUNK_ZERO_DATA != 0
        && csdatasize <= (10 as ::core::ffi::c_int * 14 as ::core::ffi::c_int) as uint32_t
    {
        (*fdce).csdataver = csdataver;
        (*fdce).chunkid = chunkid;
        (*fdce).version = version;
        (*fdce).csdatasize = csdatasize;
        memcpy(
            &raw mut (*fdce).csdata as *mut uint8_t as *mut ::core::ffi::c_void,
            csdata as *const ::core::ffi::c_void,
            csdatasize as size_t,
        );
    } else {
        (*fdce).lflags =
            ((*fdce).lflags as ::core::ffi::c_int & !LOOKUP_CHUNK_ZERO_DATA) as uint16_t;
        (*fdce).csdataver = 0 as uint8_t;
        (*fdce).chunkid = 0 as uint64_t;
        (*fdce).version = 0 as uint32_t;
        (*fdce).csdatasize = 0 as uint32_t;
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                120 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                120 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                120 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                120 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                120 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                120 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
pub unsafe extern "C" fn fdcache_invalidate(mut inode: uint32_t) {
    let mut h: uint32_t = 0;
    let mut fdce: *mut fdcacheentry = ::core::ptr::null_mut::<fdcacheentry>();
    let mut fdcep: *mut *mut fdcacheentry = ::core::ptr::null_mut::<*mut fdcacheentry>();
    h = inode.wrapping_rem(FDCACHE_HASHSIZE as uint32_t);
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                128 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                128 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                128 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                128 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                128 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                128 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    fdcep = (&raw mut fdhashtab as *mut *mut fdcacheentry).offset(h as isize);
    loop {
        fdce = *fdcep;
        if fdce.is_null() {
            break;
        }
        if (*fdce).inode == inode {
            *fdcep = (*fdce).next as *mut fdcacheentry;
            fdcachee_free(fdce);
        } else {
            fdcep = &raw mut (*fdce).next as *mut *mut fdcacheentry;
        }
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                138 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                138 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                138 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                138 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                138 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                138 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
pub unsafe extern "C" fn fdcache_find(
    mut ctx: *const fuse_ctx,
    mut inode: uint32_t,
    mut attr: *mut uint8_t,
    mut lflags: *mut uint16_t,
) -> uint8_t {
    let mut h: uint32_t = 0;
    let mut now: ::core::ffi::c_double = 0.;
    let mut fdce: *mut fdcacheentry = ::core::ptr::null_mut::<fdcacheentry>();
    now = monotonic_seconds();
    h = inode.wrapping_rem(FDCACHE_HASHSIZE as uint32_t);
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                147 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                147 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                147 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                147 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                147 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                147 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    fdce = fdhashtab[h as usize];
    while !fdce.is_null() {
        if (*fdce).inode == inode
            && (*fdce).uid == (*ctx).uid
            && (*fdce).gid == (*ctx).gid
            && (*fdce).pid == (*ctx).pid
            && (*fdce).createtime + FDCACHE_TIMEOUT >= now
        {
            if !attr.is_null() {
                memcpy(
                    attr as *mut ::core::ffi::c_void,
                    &raw mut (*fdce).attr as *mut uint8_t as *const ::core::ffi::c_void,
                    35 as size_t,
                );
            }
            if !lflags.is_null() {
                *lflags = (*fdce).lflags;
            }
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
                        b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        156 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                        b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        156 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                        b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        156 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(hashlock+h)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        156 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                        b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        156 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                        b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        156 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
            return 1 as uint8_t;
        }
        fdce = (*fdce).next as *mut fdcacheentry;
    }
    let mut _mfs_assert_ret_1: ::core::ffi::c_int =
        pthread_mutex_unlock((&raw mut hashlock as *mut pthread_mutex_t).offset(h as isize));
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                160 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                160 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                160 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                160 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                160 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                160 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
        }
        abort();
    }
    return 0 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn fdcache_acquire(
    mut ctx: *const fuse_ctx,
    mut inode: uint32_t,
    mut attr: *mut uint8_t,
    mut lflags: *mut uint16_t,
) -> *mut ::core::ffi::c_void {
    let mut h: uint32_t = 0;
    let mut now: ::core::ffi::c_double = 0.;
    let mut fdce: *mut fdcacheentry = ::core::ptr::null_mut::<fdcacheentry>();
    let mut fdcep: *mut *mut fdcacheentry = ::core::ptr::null_mut::<*mut fdcacheentry>();
    now = monotonic_seconds();
    h = inode.wrapping_rem(FDCACHE_HASHSIZE as uint32_t);
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                171 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                171 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                171 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                171 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                171 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                171 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    fdcep = (&raw mut fdhashtab as *mut *mut fdcacheentry).offset(h as isize);
    loop {
        fdce = *fdcep;
        if fdce.is_null() {
            break;
        }
        if (*fdce).inode == inode
            && (*fdce).uid == (*ctx).uid
            && (*fdce).gid == (*ctx).gid
            && (*fdce).pid == (*ctx).pid
            && (*fdce).createtime + FDCACHE_TIMEOUT >= now
        {
            if !attr.is_null() {
                memcpy(
                    attr as *mut ::core::ffi::c_void,
                    &raw mut (*fdce).attr as *mut uint8_t as *const ::core::ffi::c_void,
                    35 as size_t,
                );
            }
            if !lflags.is_null() {
                *lflags = (*fdce).lflags;
            }
            *fdcep = (*fdce).next as *mut fdcacheentry;
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
                        b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        182 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                        b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        182 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                        b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        182 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"pthread_mutex_unlock(hashlock+h)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_assert_ret_0,
                        _mfs_errorstring_2,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        182 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                        b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        182 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                        b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        182 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
            return fdce as *mut ::core::ffi::c_void;
        } else {
            fdcep = &raw mut (*fdce).next as *mut *mut fdcacheentry;
        }
    }
    let mut _mfs_assert_ret_1: ::core::ffi::c_int =
        pthread_mutex_unlock((&raw mut hashlock as *mut pthread_mutex_t).offset(h as isize));
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                188 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                188 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                188 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                188 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                188 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                188 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(hashlock+h)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
        }
        abort();
    }
    return NULL;
}
#[no_mangle]
pub unsafe extern "C" fn fdcache_release(mut vfdce: *mut ::core::ffi::c_void) {
    let mut fdce: *mut fdcacheentry = vfdce as *mut fdcacheentry;
    if !fdce.is_null() {
        fdcachee_free(fdce);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fdcache_inject_chunkdata(mut vfdce: *mut ::core::ffi::c_void) {
    let mut fdce: *mut fdcacheentry = vfdce as *mut fdcacheentry;
    if (*fdce).lflags as ::core::ffi::c_int & LOOKUP_CHUNK_ZERO_DATA != 0 {
        chunksdatacache_insert(
            (*fdce).inode,
            0 as uint32_t,
            (*fdce).chunkid,
            (*fdce).version,
            (*fdce).csdataver,
            &raw mut (*fdce).csdata as *mut uint8_t,
            (*fdce).csdatasize,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn fdcache_init() {
    let mut i: uint32_t = 0;
    i = 0 as uint32_t;
    while i < FDCACHE_HASHSIZE as uint32_t {
        fdhashtab[i as usize] = ::core::ptr::null_mut::<fdcacheentry>();
        let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_init(
            (&raw mut hashlock as *mut pthread_mutex_t).offset(i as isize),
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
                    b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    216 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(hashlock+i,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    *__errno_location(),
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    216 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(hashlock+i,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    216 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(hashlock+i,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    216 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(hashlock+i,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    216 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(hashlock+i,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_assert_ret,
                    _mfs_errorstring_ret,
                    *__errno_location(),
                    _mfs_errorstring_err,
                );
                fprintf(
                    stderr,
                    b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsclient/fdcache.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    216 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"pthread_mutex_init(hashlock+i,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
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
}
