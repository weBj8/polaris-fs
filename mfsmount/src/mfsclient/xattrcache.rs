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
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
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
    fn monotonic_useconds() -> uint64_t;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type int64_t = i64;
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
pub struct _xattr_cache_value {
    pub lcnt: uint32_t,
    pub value: *const uint8_t,
}
pub type xattr_cache_value = _xattr_cache_value;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xattr_cache_entry {
    pub hash: uint32_t,
    pub node: uint32_t,
    pub uid: uint32_t,
    pub gid: uint32_t,
    pub nleng: uint32_t,
    pub vleng: uint32_t,
    pub status: ::core::ffi::c_int,
    pub name: *const uint8_t,
    pub value: *mut xattr_cache_value,
    pub utimestamp: int64_t,
    pub hashnext: *mut _xattr_cache_entry,
    pub hashprev: *mut *mut _xattr_cache_entry,
    pub lrunext: *mut _xattr_cache_entry,
    pub lruprev: *mut *mut _xattr_cache_entry,
}
pub type xattr_cache_entry = _xattr_cache_entry;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const HASHSIZE: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
static mut lruhead: *mut xattr_cache_entry = ::core::ptr::null_mut::<xattr_cache_entry>();
static mut lrutail: *mut *mut xattr_cache_entry = ::core::ptr::null_mut::<*mut xattr_cache_entry>();
static mut hashtab: *mut *mut xattr_cache_entry = ::core::ptr::null_mut::<*mut xattr_cache_entry>();
static mut xattr_cache_timeout: int64_t = 0;
static mut glock: pthread_mutex_t = pthread_mutex_t {
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
#[inline]
unsafe extern "C" fn xattr_cache_hash(
    mut node: uint32_t,
    mut nleng: uint32_t,
    mut name: *const uint8_t,
) -> uint32_t {
    let mut hash: uint32_t = 0;
    let mut i: uint32_t = 0;
    hash = node
        .wrapping_mul(0x5f2318bd as uint32_t)
        .wrapping_add(nleng);
    i = 0 as uint32_t;
    while i < nleng {
        hash = hash
            .wrapping_mul(33 as uint32_t)
            .wrapping_add(*name.offset(i as isize) as uint32_t);
        i = i.wrapping_add(1);
    }
    return hash;
}
#[inline]
unsafe extern "C" fn xattr_cache_value_alloc() -> *mut xattr_cache_value {
    let mut v: *mut xattr_cache_value = ::core::ptr::null_mut::<xattr_cache_value>();
    v = malloc(::core::mem::size_of::<xattr_cache_value>()) as *mut xattr_cache_value;
    if v.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
            72 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"v\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
            72 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"v\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if v
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut xattr_cache_value
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
            72 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"v\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
            72 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"v\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    (*v).lcnt = 1 as uint32_t;
    (*v).value = ::core::ptr::null::<uint8_t>();
    return v;
}
#[inline]
unsafe extern "C" fn xattr_cache_value_inc(mut v: *mut xattr_cache_value) {
    (*v).lcnt = (*v).lcnt.wrapping_add(1);
}
#[inline]
unsafe extern "C" fn xattr_cache_value_dec(mut v: *mut xattr_cache_value) {
    (*v).lcnt = (*v).lcnt.wrapping_sub(1);
    if (*v).lcnt == 0 as uint32_t {
        if !(*v).value.is_null() {
            free((*v).value as *mut uint8_t as *mut ::core::ffi::c_void);
        }
        free(v as *mut ::core::ffi::c_void);
    }
}
#[inline]
unsafe extern "C" fn xattr_cache_remove_entry(mut xce: *mut xattr_cache_entry) {
    if !(*xce).hashnext.is_null() {
        (*(*xce).hashnext).hashprev = (*xce).hashprev;
    }
    *(*xce).hashprev = (*xce).hashnext;
    if !(*xce).lrunext.is_null() {
        (*(*xce).lrunext).lruprev = (*xce).lruprev;
    } else {
        lrutail = (*xce).lruprev as *mut *mut xattr_cache_entry;
    }
    *(*xce).lruprev = (*xce).lrunext;
    if !(*xce).name.is_null() {
        free((*xce).name as *mut uint8_t as *mut ::core::ffi::c_void);
    }
    xattr_cache_value_dec((*xce).value);
    free(xce as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn xattr_cache_new(
    mut node: uint32_t,
    mut uid: uint32_t,
    mut gid: uint32_t,
    mut nleng: uint32_t,
    mut name: *const uint8_t,
    mut value: *const uint8_t,
    mut vleng: uint32_t,
    mut status: ::core::ffi::c_int,
    mut utimestamp: int64_t,
) {
    let mut xce: *mut xattr_cache_entry = ::core::ptr::null_mut::<xattr_cache_entry>();
    let mut hash: uint32_t = 0;
    hash = xattr_cache_hash(node, nleng, name);
    xce = malloc(::core::mem::size_of::<xattr_cache_entry>()) as *mut xattr_cache_entry;
    if xce.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
            115 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"xce\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
            115 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"xce\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if xce
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut xattr_cache_entry
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
            115 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"xce\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
            115 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"xce\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    (*xce).hash = hash;
    (*xce).node = node;
    (*xce).uid = uid;
    (*xce).gid = gid;
    (*xce).nleng = nleng;
    if nleng > 0 as uint32_t {
        (*xce).name = malloc(nleng as size_t) as *const uint8_t;
        if (*xce).name.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                123 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"xce->name\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                123 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"xce->name\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if (*xce).name
            == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *const uint8_t
        {
            let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                123 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"xce->name\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                123 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"xce->name\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            abort();
        }
        memcpy(
            (*xce).name as *mut uint8_t as *mut ::core::ffi::c_void,
            name as *const ::core::ffi::c_void,
            nleng as size_t,
        );
    } else {
        (*xce).name = ::core::ptr::null::<uint8_t>();
    }
    (*xce).vleng = vleng;
    (*xce).value = xattr_cache_value_alloc();
    if vleng > 0 as uint32_t {
        (*(*xce).value).value = malloc(vleng as size_t) as *const uint8_t;
        if (*(*xce).value).value.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                132 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"xce->value->value\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                132 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"xce->value->value\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if (*(*xce).value).value
            == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *const uint8_t
        {
            let mut _mfs_errorstring_1: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                132 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"xce->value->value\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                132 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"xce->value->value\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_1,
            );
            abort();
        }
        memcpy(
            (*(*xce).value).value as *mut uint8_t as *mut ::core::ffi::c_void,
            value as *const ::core::ffi::c_void,
            vleng as size_t,
        );
    }
    (*xce).status = status;
    (*xce).utimestamp = utimestamp;
    (*xce).hashnext = *hashtab.offset(hash.wrapping_rem(HASHSIZE as uint32_t) as isize)
        as *mut _xattr_cache_entry;
    (*xce).hashprev = hashtab.offset(hash.wrapping_rem(HASHSIZE as uint32_t) as isize)
        as *mut *mut _xattr_cache_entry;
    if !(*xce).hashnext.is_null() {
        (*(*xce).hashnext).hashprev = &raw mut (*xce).hashnext;
    }
    *hashtab.offset(hash.wrapping_rem(HASHSIZE as uint32_t) as isize) = xce;
    (*xce).lrunext = ::core::ptr::null_mut::<_xattr_cache_entry>();
    (*xce).lruprev = lrutail as *mut *mut _xattr_cache_entry;
    *lrutail = xce;
    lrutail = &raw mut (*xce).lrunext as *mut *mut xattr_cache_entry;
}
#[inline]
unsafe extern "C" fn xattr_cache_find(
    mut node: uint32_t,
    mut uid: uint32_t,
    mut gid: uint32_t,
    mut nleng: uint32_t,
    mut name: *const uint8_t,
) -> *mut xattr_cache_entry {
    let mut hash: uint32_t = 0;
    let mut xce: *mut xattr_cache_entry = ::core::ptr::null_mut::<xattr_cache_entry>();
    hash = xattr_cache_hash(node, nleng, name);
    xce = *hashtab.offset(hash.wrapping_rem(HASHSIZE as uint32_t) as isize);
    while !xce.is_null() {
        if (*xce).hash == hash
            && (*xce).node == node
            && (*xce).uid == uid
            && (*xce).gid == gid
            && (*xce).nleng == nleng
            && (nleng == 0 as uint32_t
                || nleng > 0 as uint32_t
                    && memcmp(
                        (*xce).name as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        nleng as size_t,
                    ) == 0 as ::core::ffi::c_int)
        {
            return xce;
        }
        xce = (*xce).hashnext as *mut xattr_cache_entry;
    }
    return ::core::ptr::null_mut::<xattr_cache_entry>();
}
#[inline]
unsafe extern "C" fn xattr_cache_delete(
    mut node: uint32_t,
    mut nleng: uint32_t,
    mut name: *const uint8_t,
) {
    let mut hash: uint32_t = 0;
    let mut xce: *mut xattr_cache_entry = ::core::ptr::null_mut::<xattr_cache_entry>();
    let mut nxce: *mut xattr_cache_entry = ::core::ptr::null_mut::<xattr_cache_entry>();
    hash = xattr_cache_hash(node, nleng, name);
    xce = *hashtab.offset(hash.wrapping_rem(HASHSIZE as uint32_t) as isize);
    while !xce.is_null() {
        nxce = (*xce).hashnext as *mut xattr_cache_entry;
        if (*xce).hash == hash
            && (*xce).node == node
            && (*xce).nleng == nleng
            && (nleng == 0 as uint32_t
                || nleng > 0 as uint32_t
                    && memcmp(
                        (*xce).name as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        nleng as size_t,
                    ) == 0 as ::core::ffi::c_int)
        {
            xattr_cache_remove_entry(xce);
        }
        xce = nxce;
    }
}
#[inline]
unsafe extern "C" fn xattr_cache_invalidate(mut utimestamp: int64_t) {
    let mut xce: *mut xattr_cache_entry = ::core::ptr::null_mut::<xattr_cache_entry>();
    let mut nxce: *mut xattr_cache_entry = ::core::ptr::null_mut::<xattr_cache_entry>();
    xce = lruhead;
    while !xce.is_null() && (*xce).utimestamp < utimestamp {
        nxce = (*xce).lrunext as *mut xattr_cache_entry;
        xattr_cache_remove_entry(xce);
        xce = nxce;
    }
    if lruhead.is_null() {
        lrutail = &raw mut lruhead;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xattr_cache_get(
    mut node: uint32_t,
    mut uid: uint32_t,
    mut gid: uint32_t,
    mut nleng: uint32_t,
    mut name: *const uint8_t,
    mut value: *mut *const uint8_t,
    mut vleng: *mut uint32_t,
    mut status: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut xce: *mut xattr_cache_entry = ::core::ptr::null_mut::<xattr_cache_entry>();
    let mut v: *mut xattr_cache_value = ::core::ptr::null_mut::<xattr_cache_value>();
    let mut utimestamp: int64_t = monotonic_useconds() as int64_t;
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                193 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                193 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                193 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                193 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    xattr_cache_invalidate(utimestamp);
    xce = xattr_cache_find(node, uid, gid, nleng, name);
    if xce.is_null() {
        v = ::core::ptr::null_mut::<xattr_cache_value>();
    } else {
        if !value.is_null() {
            *value = (*(*xce).value).value;
        }
        if !vleng.is_null() {
            *vleng = (*xce).vleng;
        }
        if !status.is_null() {
            *status = (*xce).status;
        }
        v = (*xce).value;
        xattr_cache_value_inc(v);
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                211 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                211 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                211 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                211 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                211 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                211 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
    return v as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn xattr_cache_set(
    mut node: uint32_t,
    mut uid: uint32_t,
    mut gid: uint32_t,
    mut nleng: uint32_t,
    mut name: *const uint8_t,
    mut value: *const uint8_t,
    mut vleng: uint32_t,
    mut status: ::core::ffi::c_int,
) {
    let mut utimestamp: int64_t = monotonic_useconds() as int64_t;
    let mut xce: *mut xattr_cache_entry = ::core::ptr::null_mut::<xattr_cache_entry>();
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                218 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                218 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                218 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                218 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                218 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                218 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    xce = xattr_cache_find(node, uid, gid, nleng, name);
    if !xce.is_null() {
        xattr_cache_remove_entry(xce);
    }
    xattr_cache_new(
        node,
        uid,
        gid,
        nleng,
        name,
        value,
        vleng,
        status,
        utimestamp + xattr_cache_timeout,
    );
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                224 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                224 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                224 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                224 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                224 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                224 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
pub unsafe extern "C" fn xattr_cache_del(
    mut node: uint32_t,
    mut nleng: uint32_t,
    mut name: *const uint8_t,
) {
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                228 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                228 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                228 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                228 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                228 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                228 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    xattr_cache_delete(node, nleng, name);
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                230 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                230 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                230 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                230 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                230 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                230 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
pub unsafe extern "C" fn xattr_cache_rel(mut vv: *mut ::core::ffi::c_void) {
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                234 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                234 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                234 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                234 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                234 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                234 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    xattr_cache_value_dec(vv as *mut xattr_cache_value);
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                236 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                236 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                236 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                236 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                236 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                236 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
pub unsafe extern "C" fn xattr_cache_term() {
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                243 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                243 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                243 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                243 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                243 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                243 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_lock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
    while !lruhead.is_null() {
        xattr_cache_remove_entry(lruhead);
    }
    free(hashtab as *mut ::core::ffi::c_void);
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                256 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                *__errno_location(),
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                256 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                256 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                256 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                256 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                256 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_unlock(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_0,
                _mfs_errorstring_ret_0,
                *__errno_location(),
                _mfs_errorstring_err_0,
            );
        }
        abort();
    }
    let mut _mfs_assert_ret_1: ::core::ffi::c_int = pthread_mutex_destroy(&raw mut glock);
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                257 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                *__errno_location(),
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                257 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                257 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                257 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                257 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret_1,
                _mfs_errorstring_ret_1,
                *__errno_location(),
                _mfs_errorstring_err_1,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                257 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_destroy(&glock)\0".as_ptr() as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn xattr_cache_init(mut timeout: ::core::ffi::c_double) {
    let mut i: uint32_t = 0;
    lruhead = ::core::ptr::null_mut::<xattr_cache_entry>();
    lrutail = &raw mut lruhead;
    hashtab =
        malloc(::core::mem::size_of::<*mut xattr_cache_entry>().wrapping_mul(HASHSIZE as size_t))
            as *mut *mut xattr_cache_entry;
    i = 0 as uint32_t;
    while i < HASHSIZE as uint32_t {
        *hashtab.offset(i as isize) = ::core::ptr::null_mut::<xattr_cache_entry>();
        i = i.wrapping_add(1);
    }
    xattr_cache_timeout = (1000000.0f64 * timeout) as int64_t;
    let mut _mfs_assert_ret: ::core::ffi::c_int =
        pthread_mutex_init(&raw mut glock, ::core::ptr::null::<pthread_mutexattr_t>());
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                269 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&glock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                *__errno_location(),
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                269 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&glock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                269 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&glock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                269 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&glock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                269 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&glock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
            fprintf(
                stderr,
                b"%s:%u - unexpected status, '%s' returned: %d : %s (errno=%d: %s)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsclient/xattrcache.c\0".as_ptr() as *const ::core::ffi::c_char,
                269 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"pthread_mutex_init(&glock,NULL)\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_assert_ret,
                _mfs_errorstring_ret,
                *__errno_location(),
                _mfs_errorstring_err,
            );
        }
        abort();
    }
}
