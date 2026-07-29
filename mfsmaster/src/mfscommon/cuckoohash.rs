use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn rndu8() -> uint8_t;
    fn rndu32() -> uint32_t;
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
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type hash_key_t = uint64_t;
pub type htab = _htab;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _htab {
    pub hashtab: *mut *mut hentry,
    pub groot: *mut treap_node,
    pub rehashgarbage: uint8_t,
    pub helements: uint32_t,
    pub telements: uint32_t,
    pub size: uint32_t,
    pub mask: uint32_t,
    pub rehashpos: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct treap_node {
    pub key: hash_key_t,
    pub val: *mut ::core::ffi::c_void,
    pub pri: uint32_t,
    pub left: *mut treap_node,
    pub right: *mut treap_node,
}
pub type hentry = _hentry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _hentry {
    pub e: uint8_t,
    pub k: [hash_key_t; 6],
    pub v: [*mut ::core::ffi::c_void; 6],
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn tree_free(mut node: *mut treap_node) {
    if !node.is_null() {
        tree_free((*node).left as *mut treap_node);
        tree_free((*node).right as *mut treap_node);
        free(node as *mut ::core::ffi::c_void);
    }
}
#[inline]
unsafe extern "C" fn tree_rotate_with_left_child(mut node: *mut *mut treap_node) {
    let mut tmp: *mut treap_node = ::core::ptr::null_mut::<treap_node>();
    let mut n: *mut treap_node = *node;
    tmp = (*n).left as *mut treap_node;
    (*n).left = (*tmp).right;
    (*tmp).right = n as *mut treap_node;
    *node = tmp;
}
#[inline]
unsafe extern "C" fn tree_rotate_with_right_child(mut node: *mut *mut treap_node) {
    let mut tmp: *mut treap_node = ::core::ptr::null_mut::<treap_node>();
    let mut n: *mut treap_node = *node;
    tmp = (*n).right as *mut treap_node;
    (*n).right = (*tmp).left;
    (*tmp).left = n as *mut treap_node;
    *node = tmp;
}
#[inline]
unsafe extern "C" fn tree_insert(
    mut node: *mut *mut treap_node,
    mut key: hash_key_t,
    mut val: *mut ::core::ffi::c_void,
) -> uint8_t {
    let mut n: *mut treap_node = *node;
    if n.is_null() {
        n = malloc(::core::mem::size_of::<treap_node>()) as *mut treap_node;
        if n.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/cuckoohash.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                68 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/cuckoohash.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                68 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if n
            == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut treap_node
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/cuckoohash.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                68 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"n\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/cuckoohash.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                68 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"n\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*n).left = ::core::ptr::null_mut::<treap_node>();
        (*n).right = ::core::ptr::null_mut::<treap_node>();
        (*n).key = key;
        (*n).val = val;
        (*n).pri = rndu32();
        *node = n;
        return 1 as uint8_t;
    } else if key < (*n).key {
        if tree_insert(&raw mut (*n).left, key, val) != 0 {
            if (*(*n).left).pri < (*n).pri {
                tree_rotate_with_left_child(node);
            }
            return 1 as uint8_t;
        }
    } else if key > (*n).key {
        if tree_insert(&raw mut (*n).right, key, val) != 0 {
            if (*(*n).right).pri < (*n).pri {
                tree_rotate_with_right_child(node);
            }
            return 1 as uint8_t;
        }
    }
    return 0 as uint8_t;
}
#[inline]
unsafe extern "C" fn tree_delete(mut node: *mut *mut treap_node, mut key: hash_key_t) -> uint8_t {
    let mut n: *mut treap_node = ::core::ptr::null_mut::<treap_node>();
    let mut c: *mut treap_node = ::core::ptr::null_mut::<treap_node>();
    loop {
        n = *node;
        if n.is_null() {
            break;
        }
        if key < (*n).key {
            node = &raw mut (*n).left as *mut *mut treap_node;
        } else if key > (*n).key {
            node = &raw mut (*n).right as *mut *mut treap_node;
        } else {
            if (*n).left.is_null() && (*n).right.is_null() {
                *node = ::core::ptr::null_mut::<treap_node>();
                free(n as *mut ::core::ffi::c_void);
            } else if (*n).left.is_null() {
                *node = (*n).right as *mut treap_node;
                free(n as *mut ::core::ffi::c_void);
            } else if (*n).right.is_null() {
                *node = (*n).left as *mut treap_node;
                free(n as *mut ::core::ffi::c_void);
            } else if rndu8() as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                node = &raw mut (*n).left as *mut *mut treap_node;
                loop {
                    c = *node;
                    if !(!c.is_null() && !(*c).right.is_null()) {
                        break;
                    }
                    node = &raw mut (*c).right as *mut *mut treap_node;
                }
                (*n).key = (*c).key;
                (*n).val = (*c).val;
                *node = (*c).left as *mut treap_node;
                free(c as *mut ::core::ffi::c_void);
            } else {
                node = &raw mut (*n).right as *mut *mut treap_node;
                loop {
                    c = *node;
                    if !(!c.is_null() && !(*c).left.is_null()) {
                        break;
                    }
                    node = &raw mut (*c).left as *mut *mut treap_node;
                }
                (*n).key = (*c).key;
                (*n).val = (*c).val;
                *node = (*c).right as *mut treap_node;
                free(c as *mut ::core::ffi::c_void);
            }
            return 1 as uint8_t;
        }
    }
    return 0 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn tree_find(
    mut n: *mut treap_node,
    mut key: hash_key_t,
) -> *mut ::core::ffi::c_void {
    while !n.is_null() {
        if key < (*n).key {
            n = (*n).left as *mut treap_node;
        } else if key > (*n).key {
            n = (*n).right as *mut treap_node;
        } else {
            return (*n).val;
        }
    }
    return NULL;
}
pub const HASHTAB_MOVEFACTOR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const HASHTAB_LOBITS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const HASHTAB_HISIZE: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint >> HASHTAB_LOBITS;
pub const HASHTAB_LOSIZE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << HASHTAB_LOBITS;
pub const HASHTAB_MASK: ::core::ffi::c_int = HASHTAB_LOSIZE - 1 as ::core::ffi::c_int;
pub const HASH_BUCKET_SIZE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn hash_cuckoo(
    mut ht: *mut htab,
    mut he: *mut hentry,
    mut x: hash_key_t,
    mut v: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut cuckoohe1: *mut hentry = ::core::ptr::null_mut::<hentry>();
    let mut cuckoohe2: *mut hentry = ::core::ptr::null_mut::<hentry>();
    let mut cuckoohe: *mut hentry = ::core::ptr::null_mut::<hentry>();
    let mut i: uint32_t = 0;
    let mut cuckoohash: uint32_t = 0;
    i = 0 as uint32_t;
    while i < (*he).e as uint32_t {
        cuckoohash = ((*he).k[i as usize] & (*ht).mask as hash_key_t) as uint32_t;
        if cuckoohash >= (*ht).rehashpos {
            cuckoohash = cuckoohash & (*ht).mask >> 1 as ::core::ffi::c_int;
        }
        cuckoohe1 = (*(*ht)
            .hashtab
            .offset((cuckoohash >> HASHTAB_LOBITS) as isize))
        .offset((cuckoohash & HASHTAB_MASK as uint32_t) as isize);
        cuckoohash = ((*he).k[i as usize].wrapping_mul(167 as hash_key_t)
            >> 8 as ::core::ffi::c_int
            & (*ht).mask as hash_key_t) as uint32_t;
        if cuckoohash >= (*ht).rehashpos {
            cuckoohash = cuckoohash & (*ht).mask >> 1 as ::core::ffi::c_int;
        }
        cuckoohe2 = (*(*ht)
            .hashtab
            .offset((cuckoohash >> HASHTAB_LOBITS) as isize))
        .offset((cuckoohash & HASHTAB_MASK as uint32_t) as isize);
        if cuckoohe1 != he {
            cuckoohe = cuckoohe1;
        } else if cuckoohe2 != he {
            cuckoohe = cuckoohe2;
        } else {
            cuckoohe = ::core::ptr::null_mut::<hentry>();
        }
        if !cuckoohe.is_null() {
            if ((*cuckoohe).e as ::core::ffi::c_int) < HASH_BUCKET_SIZE {
                (*cuckoohe).k[(*cuckoohe).e as usize] = (*he).k[i as usize];
                (*cuckoohe).v[(*cuckoohe).e as usize] = (*he).v[i as usize];
                (*cuckoohe).e = (*cuckoohe).e.wrapping_add(1);
                (*he).k[i as usize] = x;
                (*he).v[i as usize] = v;
                (*ht).helements = (*ht).helements.wrapping_add(1);
                return 1 as ::core::ffi::c_int;
            }
        }
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn tree_rebuild(mut ht: *mut htab, mut node: *mut treap_node) {
    if !node.is_null() {
        tree_rebuild(ht, (*node).left as *mut treap_node);
        tree_rebuild(ht, (*node).right as *mut treap_node);
        chash_add(ht as *mut ::core::ffi::c_void, (*node).key, (*node).val);
        free(node as *mut ::core::ffi::c_void);
    }
}
#[inline]
unsafe extern "C" fn hash_garbage_to_hash(mut ht: *mut htab) {
    let mut nroot: *mut treap_node = ::core::ptr::null_mut::<treap_node>();
    nroot = (*ht).groot;
    (*ht).telements = 0 as uint32_t;
    (*ht).groot = ::core::ptr::null_mut::<treap_node>();
    tree_rebuild(ht, nroot);
}
#[inline]
unsafe extern "C" fn hash_rehash_job(mut ht: *mut htab) {
    let mut i: uint32_t = 0;
    let mut lhash: uint32_t = 0;
    let mut hash1: uint32_t = 0;
    let mut hash2: uint32_t = 0;
    let mut he: *mut hentry = ::core::ptr::null_mut::<hentry>();
    let mut hen: *mut hentry = ::core::ptr::null_mut::<hentry>();
    if (*ht).rehashpos < (*ht).size {
        i = 0 as uint32_t;
        while i < HASHTAB_MOVEFACTOR as uint32_t && (*ht).rehashpos < (*ht).size {
            lhash = (*ht).rehashpos & (*ht).mask >> 1 as ::core::ffi::c_int;
            he = (*(*ht).hashtab.offset((lhash >> HASHTAB_LOBITS) as isize))
                .offset((lhash & HASHTAB_MASK as uint32_t) as isize);
            hen = (*(*ht)
                .hashtab
                .offset(((*ht).rehashpos >> HASHTAB_LOBITS) as isize))
            .offset(((*ht).rehashpos & HASHTAB_MASK as uint32_t) as isize);
            (*hen).e = 0 as uint8_t;
            i = 0 as uint32_t;
            while i < (*he).e as uint32_t {
                hash1 = ((*he).k[i as usize] & (*ht).mask as hash_key_t) as uint32_t;
                hash2 = ((*he).k[i as usize].wrapping_mul(167 as hash_key_t)
                    >> 8 as ::core::ffi::c_int
                    & (*ht).mask as hash_key_t) as uint32_t;
                if hash1 == (*ht).rehashpos || hash2 == (*ht).rehashpos {
                    (*hen).k[(*hen).e as usize] = (*he).k[i as usize];
                    (*hen).v[(*hen).e as usize] = (*he).v[i as usize];
                    (*hen).e = (*hen).e.wrapping_add(1);
                    (*he).e = (*he).e.wrapping_sub(1);
                    if i < (*he).e as uint32_t {
                        (*he).k[i as usize] = (*he).k[(*he).e as usize];
                        (*he).v[i as usize] = (*he).v[(*he).e as usize];
                    }
                } else {
                    i = i.wrapping_add(1);
                }
            }
            (*ht).rehashpos = (*ht).rehashpos.wrapping_add(1);
            i = i.wrapping_add(1);
        }
    } else if (*ht).rehashgarbage != 0 {
        hash_garbage_to_hash(ht);
        (*ht).rehashgarbage = 0 as uint8_t;
    } else if (*ht).helements.wrapping_mul(3 as uint32_t)
        > (*ht)
            .size
            .wrapping_mul(2 as uint32_t)
            .wrapping_mul(HASH_BUCKET_SIZE as uint32_t)
    {
        (*ht).size = (*ht).size.wrapping_mul(2 as uint32_t);
        (*ht).mask = (*ht).size.wrapping_sub(1 as uint32_t);
        i = (*ht).rehashpos >> HASHTAB_LOBITS;
        while i < (*ht).size >> HASHTAB_LOBITS {
            *(*ht).hashtab.offset(i as isize) =
                malloc(::core::mem::size_of::<hentry>().wrapping_mul(HASHTAB_LOSIZE as size_t))
                    as *mut hentry;
            if (*(*ht).hashtab.offset(i as isize)).is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/cuckoohash.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    277 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ht->hashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/cuckoohash.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    277 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ht->hashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if *(*ht).hashtab.offset(i as isize)
                == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut hentry
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/cuckoohash.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    277 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ht->hashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/cuckoohash.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    277 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ht->hashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            i = i.wrapping_add(1);
        }
        (*ht).rehashgarbage = 1 as uint8_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn chash_new() -> *mut ::core::ffi::c_void {
    let mut ht: *mut htab = ::core::ptr::null_mut::<htab>();
    ht = malloc(::core::mem::size_of::<htab>()) as *mut htab;
    if ht.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/cuckoohash.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            287 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"ht\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/cuckoohash.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            287 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"ht\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if ht
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut htab
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/cuckoohash.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            287 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"ht\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/cuckoohash.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            287 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"ht\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    (*ht).mask = HASHTAB_MASK as uint32_t;
    (*ht).size = HASHTAB_LOSIZE as uint32_t;
    (*ht).helements = 0 as uint32_t;
    (*ht).telements = 0 as uint32_t;
    (*ht).rehashgarbage = 0 as uint8_t;
    (*ht).rehashpos = (*ht).size;
    (*ht).hashtab =
        malloc(::core::mem::size_of::<*mut hentry>().wrapping_mul(HASHTAB_HISIZE as size_t))
            as *mut *mut hentry;
    if (*ht).hashtab.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/cuckoohash.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            295 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"ht->hashtab\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/cuckoohash.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            295 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"ht->hashtab\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if (*ht).hashtab
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut *mut hentry
    {
        let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/cuckoohash.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            295 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"ht->hashtab\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring_0,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/cuckoohash.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            295 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"ht->hashtab\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring_0,
        );
        abort();
    }
    *(*ht).hashtab.offset(0 as isize) =
        malloc(::core::mem::size_of::<hentry>().wrapping_mul(HASHTAB_LOSIZE as size_t))
            as *mut hentry;
    if (*(*ht).hashtab.offset(0 as isize)).is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/cuckoohash.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            297 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"ht->hashtab[0]\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/cuckoohash.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            297 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"ht->hashtab[0]\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if *(*ht).hashtab.offset(0 as isize)
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut hentry
    {
        let mut _mfs_errorstring_1: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/cuckoohash.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            297 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"ht->hashtab[0]\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring_1,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/cuckoohash.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            297 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"ht->hashtab[0]\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring_1,
        );
        abort();
    }
    memset(
        *(*ht).hashtab.offset(0 as isize) as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<hentry>().wrapping_mul(HASHTAB_LOSIZE as size_t),
    );
    (*ht).groot = ::core::ptr::null_mut::<treap_node>();
    return ht as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn chash_erase(mut h: *mut ::core::ffi::c_void) {
    let mut ht: *mut htab = h as *mut htab;
    let mut i: uint32_t = 0;
    tree_free((*ht).groot);
    (*ht).groot = ::core::ptr::null_mut::<treap_node>();
    i = 1 as uint32_t;
    while i < (*ht).size >> HASHTAB_LOBITS {
        if !(*(*ht).hashtab.offset(i as isize)).is_null() {
            free(*(*ht).hashtab.offset(i as isize) as *mut ::core::ffi::c_void);
            *(*ht).hashtab.offset(i as isize) = ::core::ptr::null_mut::<hentry>();
        }
        i = i.wrapping_add(1);
    }
    memset(
        *(*ht).hashtab.offset(0 as isize) as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<hentry>().wrapping_mul(HASHTAB_LOSIZE as size_t),
    );
    (*ht).mask = HASHTAB_MASK as uint32_t;
    (*ht).size = HASHTAB_LOSIZE as uint32_t;
    (*ht).helements = 0 as uint32_t;
    (*ht).telements = 0 as uint32_t;
    (*ht).rehashgarbage = 0 as uint8_t;
    (*ht).rehashpos = (*ht).size;
}
#[no_mangle]
pub unsafe extern "C" fn chash_free(mut h: *mut ::core::ffi::c_void) {
    let mut ht: *mut htab = h as *mut htab;
    let mut i: uint32_t = 0;
    tree_free((*ht).groot);
    i = 0 as uint32_t;
    while i < (*ht).size >> HASHTAB_LOBITS {
        if !(*(*ht).hashtab.offset(i as isize)).is_null() {
            free(*(*ht).hashtab.offset(i as isize) as *mut ::core::ffi::c_void);
        }
        i = i.wrapping_add(1);
    }
    free(ht as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn chash_find(
    mut h: *mut ::core::ffi::c_void,
    mut x: hash_key_t,
) -> *mut ::core::ffi::c_void {
    let mut ht: *mut htab = h as *mut htab;
    let mut he: *mut hentry = ::core::ptr::null_mut::<hentry>();
    let mut i: uint32_t = 0;
    let mut hash: uint32_t = 0;
    hash = (x & (*ht).mask as hash_key_t) as uint32_t;
    if hash >= (*ht).rehashpos {
        hash = hash & (*ht).mask >> 1 as ::core::ffi::c_int;
    }
    he = (*(*ht).hashtab.offset((hash >> HASHTAB_LOBITS) as isize))
        .offset((hash & HASHTAB_MASK as uint32_t) as isize);
    i = 0 as uint32_t;
    while i < (*he).e as uint32_t {
        if (*he).k[i as usize] == x {
            return (*he).v[i as usize];
        }
        i = i.wrapping_add(1);
    }
    hash = (x.wrapping_mul(167 as hash_key_t) >> 8 as ::core::ffi::c_int & (*ht).mask as hash_key_t)
        as uint32_t;
    if hash >= (*ht).rehashpos {
        hash = hash & (*ht).mask >> 1 as ::core::ffi::c_int;
    }
    he = (*(*ht).hashtab.offset((hash >> HASHTAB_LOBITS) as isize))
        .offset((hash & HASHTAB_MASK as uint32_t) as isize);
    i = 0 as uint32_t;
    while i < (*he).e as uint32_t {
        if (*he).k[i as usize] == x {
            return (*he).v[i as usize];
        }
        i = i.wrapping_add(1);
    }
    return tree_find((*ht).groot, x);
}
#[no_mangle]
pub unsafe extern "C" fn chash_delete(mut h: *mut ::core::ffi::c_void, mut x: hash_key_t) {
    let mut ht: *mut htab = h as *mut htab;
    let mut he: *mut hentry = ::core::ptr::null_mut::<hentry>();
    let mut i: uint32_t = 0;
    let mut hash: uint32_t = 0;
    hash = (x & (*ht).mask as hash_key_t) as uint32_t;
    if hash >= (*ht).rehashpos {
        hash = hash & (*ht).mask >> 1 as ::core::ffi::c_int;
    }
    he = (*(*ht).hashtab.offset((hash >> HASHTAB_LOBITS) as isize))
        .offset((hash & HASHTAB_MASK as uint32_t) as isize);
    i = 0 as uint32_t;
    while i < (*he).e as uint32_t {
        if (*he).k[i as usize] == x {
            (*he).e = (*he).e.wrapping_sub(1);
            if i < (*he).e as uint32_t {
                (*he).k[i as usize] = (*he).k[(*he).e as usize];
                (*he).v[i as usize] = (*he).v[(*he).e as usize];
            }
            (*ht).helements = (*ht).helements.wrapping_sub(1);
            return;
        }
        i = i.wrapping_add(1);
    }
    hash = (x.wrapping_mul(167 as hash_key_t) >> 8 as ::core::ffi::c_int & (*ht).mask as hash_key_t)
        as uint32_t;
    if hash >= (*ht).rehashpos {
        hash = hash & (*ht).mask >> 1 as ::core::ffi::c_int;
    }
    he = (*(*ht).hashtab.offset((hash >> HASHTAB_LOBITS) as isize))
        .offset((hash & HASHTAB_MASK as uint32_t) as isize);
    i = 0 as uint32_t;
    while i < (*he).e as uint32_t {
        if (*he).k[i as usize] == x {
            (*he).e = (*he).e.wrapping_sub(1);
            if i < (*he).e as uint32_t {
                (*he).k[i as usize] = (*he).k[(*he).e as usize];
                (*he).v[i as usize] = (*he).v[(*he).e as usize];
            }
            (*ht).helements = (*ht).helements.wrapping_sub(1);
            return;
        }
        i = i.wrapping_add(1);
    }
    (*ht).telements = (*ht)
        .telements
        .wrapping_sub(tree_delete(&raw mut (*ht).groot, x) as uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn chash_add(
    mut h: *mut ::core::ffi::c_void,
    mut x: hash_key_t,
    mut v: *mut ::core::ffi::c_void,
) {
    let mut ht: *mut htab = h as *mut htab;
    let mut he1: *mut hentry = ::core::ptr::null_mut::<hentry>();
    let mut he2: *mut hentry = ::core::ptr::null_mut::<hentry>();
    let mut i: uint32_t = 0;
    let mut hash1: uint32_t = 0;
    let mut hash2: uint32_t = 0;
    hash_rehash_job(ht);
    hash1 = (x & (*ht).mask as hash_key_t) as uint32_t;
    if hash1 >= (*ht).rehashpos {
        hash1 = hash1 & (*ht).mask >> 1 as ::core::ffi::c_int;
    }
    hash2 = (x.wrapping_mul(167 as hash_key_t) >> 8 as ::core::ffi::c_int
        & (*ht).mask as hash_key_t) as uint32_t;
    if hash2 >= (*ht).rehashpos {
        hash2 = hash2 & (*ht).mask >> 1 as ::core::ffi::c_int;
    }
    he1 = (*(*ht).hashtab.offset((hash1 >> HASHTAB_LOBITS) as isize))
        .offset((hash1 & HASHTAB_MASK as uint32_t) as isize);
    he2 = (*(*ht).hashtab.offset((hash2 >> HASHTAB_LOBITS) as isize))
        .offset((hash2 & HASHTAB_MASK as uint32_t) as isize);
    i = 0 as uint32_t;
    while i < (*he1).e as uint32_t {
        if (*he1).k[i as usize] == x {
            return;
        }
        i = i.wrapping_add(1);
    }
    i = 0 as uint32_t;
    while i < (*he2).e as uint32_t {
        if (*he2).k[i as usize] == x {
            return;
        }
        i = i.wrapping_add(1);
    }
    if !tree_find((*ht).groot, x).is_null() {
        return;
    }
    if (*he1).e as ::core::ffi::c_int == HASH_BUCKET_SIZE
        && (*he2).e as ::core::ffi::c_int == HASH_BUCKET_SIZE
    {
        if hash_cuckoo(ht, he1, x, v) != 0 {
            return;
        }
        if hash_cuckoo(ht, he2, x, v) != 0 {
            return;
        }
        (*ht).telements = (*ht)
            .telements
            .wrapping_add(tree_insert(&raw mut (*ht).groot, x, v) as uint32_t);
        return;
    }
    if (*he1).e as ::core::ffi::c_int > (*he2).e as ::core::ffi::c_int {
        (*he2).k[(*he2).e as usize] = x;
        (*he2).v[(*he2).e as usize] = v;
        (*he2).e = (*he2).e.wrapping_add(1);
    } else {
        (*he1).k[(*he1).e as usize] = x;
        (*he1).v[(*he1).e as usize] = v;
        (*he1).e = (*he1).e.wrapping_add(1);
    }
    (*ht).helements = (*ht).helements.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn chash_get_elemcount(mut h: *mut ::core::ffi::c_void) -> uint32_t {
    let mut ht: *mut htab = h as *mut htab;
    return (*ht).helements.wrapping_add((*ht).telements);
}
#[no_mangle]
pub unsafe extern "C" fn chash_get_size(mut h: *mut ::core::ffi::c_void) -> uint32_t {
    let mut ht: *mut htab = h as *mut htab;
    return ((*ht).size as usize)
        .wrapping_mul(::core::mem::size_of::<hentry>())
        .wrapping_add(((*ht).telements as usize).wrapping_mul(::core::mem::size_of::<treap_node>()))
        as uint32_t;
}
