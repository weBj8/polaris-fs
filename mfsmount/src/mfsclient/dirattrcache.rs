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
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn name_index_create(minelements: uint32_t) -> *mut ::core::ffi::c_void;
    fn name_index_destroy(vidx: *mut ::core::ffi::c_void);
    fn name_index_add(vidx: *mut ::core::ffi::c_void, ptr: *mut uint8_t);
    fn name_index_find(
        vidx: *mut ::core::ffi::c_void,
        str: *const uint8_t,
        len: uint8_t,
    ) -> *mut uint8_t;
    fn node_index_create(minelements: uint32_t) -> *mut ::core::ffi::c_void;
    fn node_index_destroy(vidx: *mut ::core::ffi::c_void);
    fn node_index_add(vidx: *mut ::core::ffi::c_void, ptr: *mut uint8_t);
    fn node_index_find(vidx: *mut ::core::ffi::c_void, node: uint32_t) -> *mut uint8_t;
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
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __mode_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fuse_ctx {
    pub uid: uid_t,
    pub gid: gid_t,
    pub pid: pid_t,
    pub umask: mode_t,
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
pub type dircache = _dircache;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dircache {
    pub ctx: fuse_ctx,
    pub parent: uint32_t,
    pub dbhead: *mut dirbuff,
    pub attrsize: uint8_t,
    pub name_index: *mut ::core::ffi::c_void,
    pub node_index: *mut ::core::ffi::c_void,
    pub lock: pthread_mutex_t,
    pub next: *mut _dircache,
    pub prev: *mut *mut _dircache,
}
pub type dirbuff = _dirbuff;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dirbuff {
    pub dbuff: *mut uint8_t,
    pub dsize: uint32_t,
    pub next: *mut _dirbuff,
}
pub type FILE = _IO_FILE;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ATTR_RECORD_SIZE: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn get32bit(mut ptr: *mut *const uint8_t) -> uint32_t {
    let mut t32: uint32_t = 0;
    memcpy(
        &raw mut t32 as *mut ::core::ffi::c_void,
        *ptr as *const ::core::ffi::c_void,
        4 as size_t,
    );
    *ptr = (*ptr).offset(4 as ::core::ffi::c_int as isize);
    return t32.swap_bytes();
}
pub const NAME_INDEX_FLAG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NODE_INDEX_FLAG: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut head: *mut dircache = ::core::ptr::null_mut::<dircache>();
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
unsafe extern "C" fn dcache_elemcount(
    mut dbuff: *const uint8_t,
    mut dsize: uint32_t,
    mut attrsize: uint8_t,
) -> uint32_t {
    let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut eptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut enleng: uint16_t = 0;
    let mut ret: uint32_t = 0;
    ptr = dbuff;
    eptr = dbuff.offset(dsize as isize);
    ret = 0 as uint32_t;
    while ptr < eptr {
        enleng = *ptr as uint16_t;
        if ptr
            .offset(enleng as ::core::ffi::c_int as isize)
            .offset(5 as ::core::ffi::c_uint as isize)
            .offset(attrsize as ::core::ffi::c_int as isize)
            <= eptr
        {
            ret = ret.wrapping_add(1);
        }
        ptr = ptr.offset(
            (enleng as ::core::ffi::c_uint)
                .wrapping_add(5 as ::core::ffi::c_uint)
                .wrapping_add(attrsize as ::core::ffi::c_uint) as isize,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn dcache_new(
    mut ctx: *const fuse_ctx,
    mut parent: uint32_t,
    mut attrsize: uint8_t,
) -> *mut ::core::ffi::c_void {
    let mut d: *mut dircache = ::core::ptr::null_mut::<dircache>();
    d = malloc(::core::mem::size_of::<dircache>()) as *mut dircache;
    (*d).ctx.pid = (*ctx).pid;
    (*d).ctx.uid = (*ctx).uid;
    (*d).ctx.gid = (*ctx).gid;
    (*d).parent = parent;
    (*d).dbhead = ::core::ptr::null_mut::<dirbuff>();
    (*d).attrsize = attrsize;
    (*d).name_index = NULL;
    (*d).node_index = NULL;
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_init(
        &raw mut (*d).lock,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                88 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(d->lock),NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                88 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(d->lock),NULL)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                88 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(d->lock),NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                88 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(d->lock),NULL)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                88 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(d->lock),NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                88 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&(d->lock),NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_lock(&raw mut glock);
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                89 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                89 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                89 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                89 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                89 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                89 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
    if !head.is_null() {
        (*head).prev = &raw mut (*d).next;
    }
    (*d).next = head as *mut _dircache;
    (*d).prev = &raw mut head as *mut *mut _dircache;
    head = d;
    let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut glock);
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                96 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
        }
        abort();
    }
    return d as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn dcache_release(mut r: *mut ::core::ffi::c_void) {
    let mut d: *mut dircache = r as *mut dircache;
    let mut db: *mut dirbuff = ::core::ptr::null_mut::<dirbuff>();
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                104 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                104 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                104 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                104 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                104 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                104 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    if !(*d).next.is_null() {
        (*(*d).next).prev = (*d).prev;
    }
    *(*d).prev = (*d).next;
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut glock);
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                109 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                109 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                109 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                109 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                109 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                109 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
    let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*d).lock);
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                110 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                110 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                110 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                110 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                110 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                110 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
        }
        abort();
    }
    if !(*d).name_index.is_null() {
        name_index_destroy((*d).name_index);
    }
    if !(*d).node_index.is_null() {
        node_index_destroy((*d).node_index);
    }
    while !(*d).dbhead.is_null() {
        db = (*d).dbhead;
        (*d).dbhead = (*db).next as *mut dirbuff;
        free(db as *mut ::core::ffi::c_void);
    }
    let mut _mfs_assert_ret_2: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*d).lock);
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                122 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                *__errno_location(),
                _mfs_errorstring_5,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                122 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                122 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_6,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                122 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                122 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_ret_2,
                *__errno_location(),
                _mfs_errorstring_err_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                122 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_2,
                _mfs_errorstring_ret_2,
                *__errno_location(),
                _mfs_errorstring_err_2,
            );
        }
        abort();
    }
    let mut _mfs_assert_ret_3: ::core::ffi::c_int = pthread_mutex_destroy(&raw mut (*d).lock);
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                123 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                *__errno_location(),
                _mfs_errorstring_7,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                123 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                123 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                _mfs_errorstring_8,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                123 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                123 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                _mfs_errorstring_ret_3,
                *__errno_location(),
                _mfs_errorstring_err_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                123 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_3,
                _mfs_errorstring_ret_3,
                *__errno_location(),
                _mfs_errorstring_err_3,
            );
        }
        abort();
    }
    free(d as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn dcache_add_blob_to_indexes(
    mut d: *mut dircache,
    mut db: *mut dirbuff,
    mut index_mask: uint8_t,
) {
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    ptr = (*db).dbuff;
    while ptr < (*db).dbuff.offset((*db).dsize as isize) {
        if index_mask as ::core::ffi::c_int & NAME_INDEX_FLAG != 0 {
            name_index_add((*d).name_index, ptr);
        }
        if index_mask as ::core::ffi::c_int & NODE_INDEX_FLAG != 0 {
            node_index_add((*d).node_index, ptr);
        }
        ptr = ptr
            .offset(*ptr as ::core::ffi::c_int as isize)
            .offset(5 as ::core::ffi::c_int as isize)
            .offset((*d).attrsize as ::core::ffi::c_int as isize);
    }
}
#[inline]
unsafe extern "C" fn dcache_make_name_index(mut d: *mut dircache) {
    let mut db: *mut dirbuff = ::core::ptr::null_mut::<dirbuff>();
    let mut elemcount: uint32_t = 0;
    elemcount = 0 as uint32_t;
    db = (*d).dbhead;
    while !db.is_null() {
        elemcount =
            elemcount.wrapping_add(dcache_elemcount((*db).dbuff, (*db).dsize, (*d).attrsize));
        db = (*db).next as *mut dirbuff;
    }
    (*d).name_index = name_index_create(elemcount);
    db = (*d).dbhead;
    while !db.is_null() {
        dcache_add_blob_to_indexes(d, db, NAME_INDEX_FLAG as uint8_t);
        db = (*db).next as *mut dirbuff;
    }
}
#[inline]
unsafe extern "C" fn dcache_make_node_index(mut d: *mut dircache) {
    let mut db: *mut dirbuff = ::core::ptr::null_mut::<dirbuff>();
    let mut elemcount: uint32_t = 0;
    elemcount = 0 as uint32_t;
    db = (*d).dbhead;
    while !db.is_null() {
        elemcount =
            elemcount.wrapping_add(dcache_elemcount((*db).dbuff, (*db).dsize, (*d).attrsize));
        db = (*db).next as *mut dirbuff;
    }
    (*d).node_index = node_index_create(elemcount);
    db = (*d).dbhead;
    while !db.is_null() {
        dcache_add_blob_to_indexes(d, db, NODE_INDEX_FLAG as uint8_t);
        db = (*db).next as *mut dirbuff;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dcache_append(
    mut r: *mut ::core::ffi::c_void,
    mut dbuff: *mut uint8_t,
    mut dsize: uint32_t,
) {
    let mut d: *mut dircache = r as *mut dircache;
    let mut db: *mut dirbuff = ::core::ptr::null_mut::<dirbuff>();
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*d).lock);
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    db = malloc(::core::mem::size_of::<dirbuff>()) as *mut dirbuff;
    (*db).dbuff = dbuff;
    (*db).dsize = dsize;
    (*db).next = (*d).dbhead as *mut _dirbuff;
    (*d).dbhead = db;
    dcache_add_blob_to_indexes(
        d,
        db,
        ((if !(*d).name_index.is_null() {
            NAME_INDEX_FLAG
        } else {
            0 as ::core::ffi::c_int
        }) | (if !(*d).node_index.is_null() {
            NODE_INDEX_FLAG
        } else {
            0 as ::core::ffi::c_int
        })) as uint8_t,
    );
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*d).lock);
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
}
#[inline]
unsafe extern "C" fn dcache_namehash_invalidate(
    mut d: *mut dircache,
    mut nleng: uint8_t,
    mut name: *const uint8_t,
) {
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*d).lock);
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                187 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                187 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                187 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                187 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                187 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                187 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    if (*d).name_index.is_null() {
        dcache_make_name_index(d);
    }
    ptr = name_index_find((*d).name_index, name, nleng);
    if !ptr.is_null() {
        ptr = ptr.offset((*ptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
        memset(
            ptr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<uint32_t>().wrapping_add((*d).attrsize as size_t),
        );
    }
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*d).lock);
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                196 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                196 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                196 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                196 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                196 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                196 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
}
#[inline]
unsafe extern "C" fn dcache_namehash_get(
    mut d: *mut dircache,
    mut nleng: uint8_t,
    mut name: *const uint8_t,
    mut inode: *mut uint32_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut res: uint8_t = 0;
    res = 0 as uint8_t;
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*d).lock);
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                205 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                205 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                205 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                205 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                205 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                205 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    if (*d).name_index.is_null() {
        dcache_make_name_index(d);
    }
    ptr = name_index_find((*d).name_index, name, nleng);
    if !ptr.is_null() {
        rptr = ptr
            .offset(*ptr as ::core::ffi::c_int as isize)
            .offset(1 as ::core::ffi::c_int as isize);
        *inode = get32bit(&raw mut rptr);
        if *rptr != 0 {
            if (*d).attrsize as ::core::ffi::c_int >= ATTR_RECORD_SIZE {
                memcpy(
                    attr as *mut ::core::ffi::c_void,
                    rptr as *const ::core::ffi::c_void,
                    ATTR_RECORD_SIZE as size_t,
                );
            } else {
                memcpy(
                    attr as *mut ::core::ffi::c_void,
                    rptr as *const ::core::ffi::c_void,
                    (*d).attrsize as size_t,
                );
                memset(
                    attr.offset((*d).attrsize as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (ATTR_RECORD_SIZE - (*d).attrsize as ::core::ffi::c_int) as size_t,
                );
            }
            res = 1 as uint8_t;
        }
    }
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*d).lock);
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                223 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                223 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                223 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                223 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                223 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                223 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
    return res;
}
#[inline]
unsafe extern "C" fn dcache_inodehash_get(
    mut d: *mut dircache,
    mut inode: uint32_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut res: uint8_t = 0;
    res = 0 as uint8_t;
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*d).lock);
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                233 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                233 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                233 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                233 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                233 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                233 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    if (*d).node_index.is_null() {
        dcache_make_node_index(d);
    }
    ptr = node_index_find((*d).node_index, inode);
    if !ptr.is_null() {
        rptr = ptr
            .offset(*ptr as ::core::ffi::c_int as isize)
            .offset(5 as ::core::ffi::c_int as isize);
        if *rptr != 0 {
            if (*d).attrsize as ::core::ffi::c_int >= ATTR_RECORD_SIZE {
                memcpy(
                    attr as *mut ::core::ffi::c_void,
                    rptr as *const ::core::ffi::c_void,
                    ATTR_RECORD_SIZE as size_t,
                );
            } else {
                memcpy(
                    attr as *mut ::core::ffi::c_void,
                    rptr as *const ::core::ffi::c_void,
                    (*d).attrsize as size_t,
                );
                memset(
                    attr.offset((*d).attrsize as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (ATTR_RECORD_SIZE - (*d).attrsize as ::core::ffi::c_int) as size_t,
                );
            }
            res = 1 as uint8_t;
        }
    }
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*d).lock);
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                250 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                250 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                250 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                250 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                250 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                250 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
    return res;
}
#[inline]
unsafe extern "C" fn dcache_inodehash_set(
    mut d: *mut dircache,
    mut inode: uint32_t,
    mut attr: *const uint8_t,
) -> uint8_t {
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut res: uint8_t = 0;
    res = 0 as uint8_t;
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*d).lock);
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                260 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                260 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                260 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                260 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                260 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                260 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    if (*d).node_index.is_null() {
        dcache_make_node_index(d);
    }
    ptr = node_index_find((*d).node_index, inode);
    if !ptr.is_null() {
        wptr = ptr
            .offset(*ptr as ::core::ffi::c_int as isize)
            .offset(5 as ::core::ffi::c_int as isize);
        if ((*d).attrsize as ::core::ffi::c_int) < ATTR_RECORD_SIZE {
            memcpy(
                wptr as *mut ::core::ffi::c_void,
                attr as *const ::core::ffi::c_void,
                (*d).attrsize as size_t,
            );
        } else {
            memcpy(
                wptr as *mut ::core::ffi::c_void,
                attr as *const ::core::ffi::c_void,
                ATTR_RECORD_SIZE as size_t,
            );
        }
        res = 1 as uint8_t;
    }
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*d).lock);
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                274 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                274 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                274 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                274 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                274 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                274 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
    return res;
}
#[inline]
unsafe extern "C" fn dcache_inodehash_invalidate_attr(
    mut d: *mut dircache,
    mut inode: uint32_t,
) -> uint8_t {
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut res: uint8_t = 0;
    res = 0 as uint8_t;
    let mut _mfs_assert_ret: ::core::ffi::c_int = pthread_mutex_lock(&raw mut (*d).lock);
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                284 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                284 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                284 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                284 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                284 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                284 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    if (*d).node_index.is_null() {
        dcache_make_node_index(d);
    }
    ptr = node_index_find((*d).node_index, inode);
    if !ptr.is_null() {
        wptr = ptr
            .offset(*ptr as ::core::ffi::c_int as isize)
            .offset(5 as ::core::ffi::c_int as isize);
        memset(
            wptr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (*d).attrsize as size_t,
        );
        res = 1 as uint8_t;
    }
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut (*d).lock);
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                294 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                294 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                294 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                294 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                294 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                294 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&(d->lock))\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn dcache_lookup(
    mut ctx: *const fuse_ctx,
    mut parent: uint32_t,
    mut nleng: uint8_t,
    mut name: *const uint8_t,
    mut inode: *mut uint32_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    let mut d: *mut dircache = ::core::ptr::null_mut::<dircache>();
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                301 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                301 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                301 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                301 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                301 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                301 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    d = head;
    while !d.is_null() {
        if parent == (*d).parent
            && (*ctx).pid == (*d).ctx.pid
            && (*ctx).uid == (*d).ctx.uid
            && (*ctx).gid == (*d).ctx.gid
        {
            if dcache_namehash_get(d, nleng, name, inode, attr) != 0 {
                let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                    pthread_mutex_unlock(&raw mut glock);
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
                            b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            305 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&glock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_1,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            305 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&glock)\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            305 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&glock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_2,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            305 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&glock)\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            305 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&glock)\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            305 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&glock)\0".as_ptr()
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
        }
        d = (*d).next as *mut dircache;
    }
    let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut glock);
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                310 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                310 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                310 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                310 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                310 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                310 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn dcache_getattr(
    mut ctx: *const fuse_ctx,
    mut inode: uint32_t,
    mut attr: *mut uint8_t,
) -> uint8_t {
    let mut d: *mut dircache = ::core::ptr::null_mut::<dircache>();
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                316 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                316 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                316 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                316 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                316 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                316 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    d = head;
    while !d.is_null() {
        if (*ctx).pid == (*d).ctx.pid && (*ctx).uid == (*d).ctx.uid && (*ctx).gid == (*d).ctx.gid {
            if dcache_inodehash_get(d, inode, attr) != 0 {
                let mut _mfs_assert_ret_0: ::core::ffi::c_int =
                    pthread_mutex_unlock(&raw mut glock);
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
                            b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            320 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&glock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            *__errno_location(),
                            _mfs_errorstring_1,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            320 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&glock)\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            320 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&glock)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            _mfs_assert_ret_0,
                            _mfs_errorstring_2,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            320 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&glock)\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            320 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&glock)\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            320 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"pthread_mutex_unlock(&glock)\0".as_ptr()
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
        }
        d = (*d).next as *mut dircache;
    }
    let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut glock);
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                325 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                325 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                325 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                325 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                325 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                325 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn dcache_setattr(mut inode: uint32_t, mut attr: *const uint8_t) {
    let mut d: *mut dircache = ::core::ptr::null_mut::<dircache>();
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                331 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                331 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                331 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                331 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                331 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                331 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    d = head;
    while !d.is_null() {
        dcache_inodehash_set(d, inode, attr);
        d = (*d).next as *mut dircache;
    }
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut glock);
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                335 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                335 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                335 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                335 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                335 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                335 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
#[no_mangle]
pub unsafe extern "C" fn dcache_invalidate_attr(mut inode: uint32_t) {
    let mut d: *mut dircache = ::core::ptr::null_mut::<dircache>();
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                340 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                340 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                340 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                340 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                340 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                340 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    d = head;
    while !d.is_null() {
        dcache_inodehash_invalidate_attr(d, inode);
        d = (*d).next as *mut dircache;
    }
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut glock);
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                344 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                344 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                344 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                344 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                344 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                344 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
#[no_mangle]
pub unsafe extern "C" fn dcache_invalidate_name(
    mut parent: uint32_t,
    mut nleng: uint8_t,
    mut name: *const uint8_t,
) {
    let mut d: *mut dircache = ::core::ptr::null_mut::<dircache>();
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                349 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                349 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                349 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                349 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                349 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                349 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    d = head;
    while !d.is_null() {
        if parent == (*d).parent {
            dcache_namehash_invalidate(d, nleng, name);
        }
        d = (*d).next as *mut dircache;
    }
    let mut _mfs_assert_ret_0: ::core::ffi::c_int = pthread_mutex_unlock(&raw mut glock);
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                355 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                355 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                355 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                355 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                355 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/dirattrcache.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                355 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
