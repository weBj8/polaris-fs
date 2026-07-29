use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _bio;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fputc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
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
    fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn bio_read(b: *mut bio, dst: *mut ::core::ffi::c_void, len: uint64_t) -> int64_t;
    fn bio_write(b: *mut bio, src: *const ::core::ffi::c_void, len: uint64_t) -> int64_t;
    fn bio_skip(b: *mut bio, len: uint64_t);
    fn fs_check_inode(inode: uint32_t) -> uint8_t;
    fn fs_set_aclflag(inode: uint32_t, acltype: uint8_t);
    fn fs_del_aclflag(inode: uint32_t, acltype: uint8_t);
    fn fs_get_mode(inode: uint32_t) -> uint16_t;
    fn mmap(
        __addr: *mut ::core::ffi::c_void,
        __len: size_t,
        __prot: ::core::ffi::c_int,
        __flags: ::core::ffi::c_int,
        __fd: ::core::ffi::c_int,
        __offset: __off64_t,
    ) -> *mut ::core::ffi::c_void;
    fn munmap(__addr: *mut ::core::ffi::c_void, __len: size_t) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type int32_t = i32;
pub type int64_t = i64;
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
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type bio = _bio;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct acl_entry {
    pub id: uint32_t,
    pub perm: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct acl_node {
    pub inode: uint32_t,
    pub acltype: uint8_t,
    pub userperm: uint16_t,
    pub groupperm: uint16_t,
    pub otherperm: uint16_t,
    pub mask: uint16_t,
    pub namedusers: uint16_t,
    pub namedgroups: uint16_t,
    pub acltab: *mut acl_entry,
    pub next: *mut acl_node,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MODE_TO_ACCMODE: [uint8_t; 8] = [
    0x1 as uint8_t,
    0x3 as uint8_t,
    0x5 as uint8_t,
    0xf as uint8_t,
    0x11 as uint8_t,
    0x33 as uint8_t,
    0x55 as uint8_t,
    0xff as uint8_t,
];
pub const POSIX_ACL_ACCESS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const POSIX_ACL_DEFAULT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn put32bit(mut ptr: *mut *mut uint8_t, mut val: uint32_t) {
    val = val.swap_bytes() as uint32_t;
    memcpy(
        *ptr as *mut ::core::ffi::c_void,
        &raw mut val as *const ::core::ffi::c_void,
        4 as size_t,
    );
    *ptr = (*ptr).offset(4 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn put16bit(mut ptr: *mut *mut uint8_t, mut val: uint16_t) {
    val = val.swap_bytes() as uint16_t;
    memcpy(
        *ptr as *mut ::core::ffi::c_void,
        &raw mut val as *const ::core::ffi::c_void,
        2 as size_t,
    );
    *ptr = (*ptr).offset(2 as ::core::ffi::c_int as isize);
}
#[inline]
unsafe extern "C" fn put8bit(mut ptr: *mut *mut uint8_t, mut val: uint8_t) {
    *(*ptr).offset(0 as isize) =
        (val as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
    *ptr = (*ptr).offset(1);
}
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
#[inline]
unsafe extern "C" fn get16bit(mut ptr: *mut *const uint8_t) -> uint16_t {
    let mut t16: uint16_t = 0;
    memcpy(
        &raw mut t16 as *mut ::core::ffi::c_void,
        *ptr as *const ::core::ffi::c_void,
        2 as size_t,
    );
    *ptr = (*ptr).offset(2 as ::core::ffi::c_int as isize);
    return t16.swap_bytes();
}
#[inline]
unsafe extern "C" fn get8bit(mut ptr: *mut *const uint8_t) -> uint8_t {
    let mut t8: uint8_t = 0;
    t8 = *(*ptr).offset(0 as isize);
    *ptr = (*ptr).offset(1);
    return t8;
}
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_ERRNO_SYSLOG_STDERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const LOHASH_BITS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn posix_acl_xxx_cmp(
    mut e: *mut acl_node,
    mut inode: uint32_t,
    mut acltype: uint8_t,
) -> ::core::ffi::c_int {
    return ((*e).inode == inode
        && (*e).acltype as ::core::ffi::c_int == acltype as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn posix_acl_xxx_hash(mut inode: uint32_t, mut acltype: uint8_t) -> uint32_t {
    return inode
        .wrapping_mul(0x56bf7623 as uint32_t)
        .wrapping_add(acltype as uint32_t);
}
#[inline]
unsafe extern "C" fn posix_acl_xxx_ehash(mut e: *mut acl_node) -> uint32_t {
    return posix_acl_xxx_hash((*e).inode, (*e).acltype);
}
pub const PROT_READ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PROT_WRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MAP_PRIVATE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MAP_ANONYMOUS: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const MAP_ANON: ::core::ffi::c_int = MAP_ANONYMOUS;
pub const HASHTAB_LOBITS: ::core::ffi::c_int = LOHASH_BITS;
pub const HASHTAB_HISIZE: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint >> HASHTAB_LOBITS;
pub const HASHTAB_LOSIZE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << HASHTAB_LOBITS;
pub const HASHTAB_MASK: ::core::ffi::c_int = HASHTAB_LOSIZE - 1 as ::core::ffi::c_int;
pub const HASHTAB_MOVEFACTOR: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const HASHTAB_SIZEHINT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut paclhashtab: [*mut *mut acl_node; 2048] =
    [::core::ptr::null_mut::<*mut acl_node>(); 2048];
static mut paclrehashpos: uint32_t = 0;
static mut paclhashsize: uint32_t = 0;
static mut paclhashelem: uint32_t = 0;
#[inline]
unsafe extern "C" fn posix_acl_xxx_calc_hash_size(mut elements: uint32_t) -> uint32_t {
    let mut res: uint32_t = 1 as uint32_t;
    while elements != 0 {
        elements >>= 1 as ::core::ffi::c_int;
        res <<= 1 as ::core::ffi::c_int;
    }
    if res == 0 as uint32_t {
        res = 0x80000000 as ::core::ffi::c_uint as uint32_t;
    }
    if res < HASHTAB_LOSIZE as uint32_t {
        return HASHTAB_LOSIZE as uint32_t;
    }
    return res;
}
#[inline]
unsafe extern "C" fn posix_acl_xxx_hash_init() {
    let mut i: uint16_t = 0;
    paclhashsize = 0 as uint32_t;
    paclhashelem = 0 as uint32_t;
    paclrehashpos = 0 as uint32_t;
    i = 0 as uint16_t;
    while (i as ::core::ffi::c_uint) < HASHTAB_HISIZE {
        paclhashtab[i as usize] = ::core::ptr::null_mut::<*mut acl_node>();
        i = i.wrapping_add(1);
    }
}
#[inline]
unsafe extern "C" fn posix_acl_xxx_hash_cleanup() {
    let mut i: uint16_t = 0;
    let mut j: uint32_t = 0;
    paclhashelem = 0 as uint32_t;
    paclhashsize = 0 as uint32_t;
    paclrehashpos = 0 as uint32_t;
    i = 0 as uint16_t;
    while (i as ::core::ffi::c_uint) < HASHTAB_HISIZE {
        if !paclhashtab[i as usize].is_null() {
            j = 0 as uint32_t;
            while j < HASHTAB_LOSIZE as uint32_t {
                if (*paclhashtab[i as usize].offset(j as isize)).is_null() {
                } else {
                    fprintf(
                        stderr,
                        b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfscommon/hash_begin.h\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        133 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"GLUE_HASH_TAB_PREFIX(hashtab)[i][j]==NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"hash map has elements during clean up\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - failed assertion '%s' : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfscommon/hash_begin.h\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        133 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"GLUE_HASH_TAB_PREFIX(hashtab)[i][j]==NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"hash map has elements during clean up\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    abort();
                };
                j = j.wrapping_add(1);
            }
            munmap(
                paclhashtab[i as usize] as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<*mut acl_node>().wrapping_mul(HASHTAB_LOSIZE as size_t),
            );
        }
        paclhashtab[i as usize] = ::core::ptr::null_mut::<*mut acl_node>();
        i = i.wrapping_add(1);
    }
}
#[inline]
unsafe extern "C" fn posix_acl_xxx_hash_move() {
    let mut hash: uint32_t = 0;
    let mut mask: uint32_t = 0;
    let mut moved: uint32_t = 0 as uint32_t;
    let mut ehptr: *mut *mut acl_node = ::core::ptr::null_mut::<*mut acl_node>();
    let mut ehptralt: *mut *mut acl_node = ::core::ptr::null_mut::<*mut acl_node>();
    let mut e: *mut acl_node = ::core::ptr::null_mut::<acl_node>();
    mask = paclhashsize.wrapping_sub(1 as uint32_t);
    loop {
        if paclrehashpos >= paclhashsize {
            paclrehashpos = paclhashsize;
            return;
        }
        if paclhashtab[(paclrehashpos >> HASHTAB_LOBITS) as usize].is_null() {
            paclhashtab[(paclrehashpos >> HASHTAB_LOBITS) as usize] = mmap(
                NULL,
                ::core::mem::size_of::<*mut acl_node>().wrapping_mul(HASHTAB_LOSIZE as size_t),
                PROT_READ | PROT_WRITE,
                MAP_ANON | MAP_PRIVATE,
                -1 as ::core::ffi::c_int,
                0 as __off64_t,
            )
                as *mut *mut acl_node;
            if paclhashtab[(paclrehashpos >> 20 as ::core::ffi::c_int) as usize].is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfscommon/hash_begin.h\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    187 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"GLUE_HASH_TAB_PREFIX(hashtab)[GLUE_HASH_TAB_PREFIX(rehashpos) >> HASHTAB_LOBITS]\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfscommon/hash_begin.h\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    187 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"GLUE_HASH_TAB_PREFIX(hashtab)[GLUE_HASH_TAB_PREFIX(rehashpos) >> HASHTAB_LOBITS]\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if paclhashtab[(paclrehashpos >> 20 as ::core::ffi::c_int) as usize]
                == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut *mut acl_node
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfscommon/hash_begin.h\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    187 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"GLUE_HASH_TAB_PREFIX(hashtab)[GLUE_HASH_TAB_PREFIX(rehashpos) >> HASHTAB_LOBITS]\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfscommon/hash_begin.h\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    187 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"GLUE_HASH_TAB_PREFIX(hashtab)[GLUE_HASH_TAB_PREFIX(rehashpos) >> HASHTAB_LOBITS]\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
        }
        ehptr = paclhashtab[(paclrehashpos.wrapping_sub(paclhashsize.wrapping_div(2 as uint32_t))
            >> HASHTAB_LOBITS) as usize]
            .offset((paclrehashpos & HASHTAB_MASK as uint32_t) as isize);
        ehptralt = paclhashtab[(paclrehashpos >> HASHTAB_LOBITS) as usize]
            .offset((paclrehashpos & HASHTAB_MASK as uint32_t) as isize);
        *ehptralt = ::core::ptr::null_mut::<acl_node>();
        loop {
            e = *ehptr;
            if e.is_null() {
                break;
            }
            hash = posix_acl_xxx_ehash(e) & mask;
            if hash == paclrehashpos {
                *ehptralt = e;
                *ehptr = (*e).next as *mut acl_node;
                ehptralt = &raw mut (*e).next as *mut *mut acl_node;
                (*e).next = ::core::ptr::null_mut::<acl_node>();
            } else {
                ehptr = &raw mut (*e).next as *mut *mut acl_node;
            }
            moved = moved.wrapping_add(1);
        }
        paclrehashpos = paclrehashpos.wrapping_add(1);
        if moved >= HASHTAB_MOVEFACTOR as uint32_t {
            break;
        }
    }
}
#[inline]
unsafe extern "C" fn posix_acl_xxx_find(
    mut inode: uint32_t,
    mut acltype: uint8_t,
) -> *mut acl_node {
    let mut e: *mut acl_node = ::core::ptr::null_mut::<acl_node>();
    let mut hash: uint32_t = 0;
    let mut hashval: uint32_t = 0;
    if paclhashsize == 0 as uint32_t {
        return ::core::ptr::null_mut::<acl_node>();
    }
    hashval = posix_acl_xxx_hash(inode, acltype);
    hash = hashval & paclhashsize.wrapping_sub(1 as uint32_t);
    if paclrehashpos < paclhashsize {
        posix_acl_xxx_hash_move();
        if hash >= paclrehashpos {
            hash = hash.wrapping_sub(paclhashsize.wrapping_div(2 as uint32_t));
        }
    }
    e = *paclhashtab[(hash >> HASHTAB_LOBITS) as usize]
        .offset((hash & HASHTAB_MASK as uint32_t) as isize);
    while !e.is_null() {
        if posix_acl_xxx_cmp(e, inode, acltype) != 0 {
            return e;
        }
        e = (*e).next as *mut acl_node;
    }
    return ::core::ptr::null_mut::<acl_node>();
}
#[inline]
unsafe extern "C" fn posix_acl_xxx_delete(mut e: *mut acl_node) -> uint8_t {
    let mut ehptr: *mut *mut acl_node = ::core::ptr::null_mut::<*mut acl_node>();
    let mut eit: *mut acl_node = ::core::ptr::null_mut::<acl_node>();
    let mut hash: uint32_t = 0;
    if paclhashsize == 0 as uint32_t {
        return 0 as uint8_t;
    }
    hash = posix_acl_xxx_ehash(e) & paclhashsize.wrapping_sub(1 as uint32_t);
    if paclrehashpos < paclhashsize {
        posix_acl_xxx_hash_move();
        if hash >= paclrehashpos {
            hash = hash.wrapping_sub(paclhashsize.wrapping_div(2 as uint32_t));
        }
    }
    ehptr = paclhashtab[(hash >> HASHTAB_LOBITS) as usize]
        .offset((hash & HASHTAB_MASK as uint32_t) as isize);
    loop {
        eit = *ehptr;
        if eit.is_null() {
            break;
        }
        if eit == e {
            *ehptr = (*e).next as *mut acl_node;
            paclhashelem = paclhashelem.wrapping_sub(1);
            return 1 as uint8_t;
        }
        ehptr = &raw mut (*eit).next as *mut *mut acl_node;
    }
    return 0 as uint8_t;
}
#[inline]
unsafe extern "C" fn posix_acl_xxx_add(mut e: *mut acl_node) {
    let mut i: uint16_t = 0;
    let mut hash: uint32_t = 0;
    if paclhashsize == 0 as uint32_t {
        paclhashsize = posix_acl_xxx_calc_hash_size(HASHTAB_SIZEHINT as uint32_t);
        paclrehashpos = paclhashsize;
        paclhashelem = 0 as uint32_t;
        i = 0 as uint16_t;
        while (i as uint32_t) < paclhashsize >> HASHTAB_LOBITS {
            paclhashtab[i as usize] = mmap(
                NULL,
                ::core::mem::size_of::<*mut acl_node>().wrapping_mul(HASHTAB_LOSIZE as size_t),
                PROT_READ | PROT_WRITE,
                MAP_ANON | MAP_PRIVATE,
                -1 as ::core::ffi::c_int,
                0 as __off64_t,
            ) as *mut *mut acl_node;
            if paclhashtab[i as usize].is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfscommon/hash_begin.h\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    284 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"GLUE_HASH_TAB_PREFIX(hashtab)[i]\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfscommon/hash_begin.h\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    284 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"GLUE_HASH_TAB_PREFIX(hashtab)[i]\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if paclhashtab[i as usize]
                == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut *mut acl_node
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfscommon/hash_begin.h\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    284 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"GLUE_HASH_TAB_PREFIX(hashtab)[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfscommon/hash_begin.h\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    284 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"GLUE_HASH_TAB_PREFIX(hashtab)[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            memset(
                paclhashtab[i as usize] as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<*mut acl_node>(),
            );
            if (*paclhashtab[i as usize].offset(0 as isize)).is_null() {
                memset(
                    paclhashtab[i as usize] as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<*mut acl_node>().wrapping_mul(HASHTAB_LOSIZE as size_t),
                );
            } else {
                hash = 0 as uint32_t;
                while hash < HASHTAB_LOSIZE as uint32_t {
                    *paclhashtab[i as usize].offset(hash as isize) =
                        ::core::ptr::null_mut::<acl_node>();
                    hash = hash.wrapping_add(1);
                }
            }
            i = i.wrapping_add(1);
        }
    }
    hash = posix_acl_xxx_ehash(e) & paclhashsize.wrapping_sub(1 as uint32_t);
    if paclrehashpos < paclhashsize {
        posix_acl_xxx_hash_move();
        if hash >= paclrehashpos {
            hash = hash.wrapping_sub(paclhashsize.wrapping_div(2 as uint32_t));
        }
        (*e).next = *paclhashtab[(hash >> HASHTAB_LOBITS) as usize]
            .offset((hash & HASHTAB_MASK as uint32_t) as isize)
            as *mut acl_node;
        *paclhashtab[(hash >> HASHTAB_LOBITS) as usize]
            .offset((hash & HASHTAB_MASK as uint32_t) as isize) = e;
        paclhashelem = paclhashelem.wrapping_add(1);
    } else {
        (*e).next = *paclhashtab[(hash >> HASHTAB_LOBITS) as usize]
            .offset((hash & HASHTAB_MASK as uint32_t) as isize)
            as *mut acl_node;
        *paclhashtab[(hash >> HASHTAB_LOBITS) as usize]
            .offset((hash & HASHTAB_MASK as uint32_t) as isize) = e;
        paclhashelem = paclhashelem.wrapping_add(1);
        if paclhashelem > paclhashsize
            && paclhashsize >> HASHTAB_LOBITS < HASHTAB_HISIZE as uint32_t
        {
            paclrehashpos = paclhashsize;
            paclhashsize = paclhashsize.wrapping_mul(2 as uint32_t);
        }
    };
}
unsafe extern "C" fn posix_acl_create(mut inode: uint32_t, mut acltype: uint8_t) -> *mut acl_node {
    let mut acn: *mut acl_node = ::core::ptr::null_mut::<acl_node>();
    acn = malloc(::core::mem::size_of::<acl_node>()) as *mut acl_node;
    if acn.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr() as *const ::core::ffi::c_char,
            84 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"acn\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr() as *const ::core::ffi::c_char,
            84 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"acn\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if acn
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut acl_node
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr() as *const ::core::ffi::c_char,
            84 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"acn\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr() as *const ::core::ffi::c_char,
            84 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"acn\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    (*acn).inode = inode;
    (*acn).acltype = acltype;
    (*acn).userperm = 0 as uint16_t;
    (*acn).groupperm = 0 as uint16_t;
    (*acn).otherperm = 0 as uint16_t;
    (*acn).mask = 0 as uint16_t;
    (*acn).namedusers = 0 as uint16_t;
    (*acn).namedgroups = 0 as uint16_t;
    (*acn).acltab = ::core::ptr::null_mut::<acl_entry>();
    posix_acl_xxx_add(acn);
    return acn;
}
#[no_mangle]
pub unsafe extern "C" fn posix_acl_getmode(mut inode: uint32_t) -> uint16_t {
    let mut acn: *mut acl_node = ::core::ptr::null_mut::<acl_node>();
    acn = posix_acl_xxx_find(inode, POSIX_ACL_ACCESS as uint8_t);
    return (((*acn).userperm as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
        << 6 as ::core::ffi::c_int
        | ((*acn).mask as ::core::ffi::c_int & 7 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
        | (*acn).otherperm as ::core::ffi::c_int & 7 as ::core::ffi::c_int) as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn posix_acl_setmode(mut inode: uint32_t, mut mode: uint16_t) {
    let mut acn: *mut acl_node = ::core::ptr::null_mut::<acl_node>();
    acn = posix_acl_xxx_find(inode, POSIX_ACL_ACCESS as uint8_t);
    if !acn.is_null() {
        (*acn).userperm =
            ((*acn).userperm as ::core::ffi::c_int & 0xfff8 as ::core::ffi::c_int) as uint16_t;
        (*acn).userperm = ((*acn).userperm as ::core::ffi::c_int
            | mode as ::core::ffi::c_int >> 6 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as uint16_t;
        (*acn).mask =
            ((*acn).mask as ::core::ffi::c_int & 0xfff8 as ::core::ffi::c_int) as uint16_t;
        (*acn).mask = ((*acn).mask as ::core::ffi::c_int
            | mode as ::core::ffi::c_int >> 3 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as uint16_t;
        (*acn).otherperm =
            ((*acn).otherperm as ::core::ffi::c_int & 0xfff8 as ::core::ffi::c_int) as uint16_t;
        (*acn).otherperm = ((*acn).otherperm as ::core::ffi::c_int
            | mode as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as uint16_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn posix_acl_accmode(
    mut inode: uint32_t,
    mut auid: uint32_t,
    mut agids: uint32_t,
    mut agid: *mut uint32_t,
    mut fuid: uint32_t,
    mut fgid: uint32_t,
) -> uint8_t {
    static mut modetoaccmode: [uint8_t; 8] = MODE_TO_ACCMODE;
    let mut acn: *mut acl_node = ::core::ptr::null_mut::<acl_node>();
    let mut f: ::core::ffi::c_int = 0;
    let mut i: uint32_t = 0;
    let mut j: uint32_t = 0;
    let mut modemask: uint8_t = 0;
    if auid == 0 as uint32_t {
        return modetoaccmode[7 as usize];
    }
    acn = posix_acl_xxx_find(inode, POSIX_ACL_ACCESS as uint8_t);
    if acn.is_null() {
        return modetoaccmode[0 as usize];
    }
    if auid == fuid {
        return modetoaccmode
            [((*acn).userperm as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int) as usize];
    } else {
        i = 0 as uint32_t;
        while i < (*acn).namedusers as uint32_t {
            if auid == (*(*acn).acltab.offset(i as isize)).id {
                return modetoaccmode[((*(*acn).acltab.offset(i as isize)).perm
                    as ::core::ffi::c_int
                    & (*acn).mask as ::core::ffi::c_int
                    & 0x7 as ::core::ffi::c_int) as usize];
            }
            i = i.wrapping_add(1);
        }
        f = 0 as ::core::ffi::c_int;
        modemask = 0 as uint8_t;
        j = 0 as uint32_t;
        while j < agids {
            if *agid.offset(j as isize) == fgid {
                modemask = (modemask as ::core::ffi::c_int
                    | modetoaccmode[((*acn).groupperm as ::core::ffi::c_int
                        & (*acn).mask as ::core::ffi::c_int
                        & 0x7 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int) as uint8_t;
                f = 1 as ::core::ffi::c_int;
            }
            j = j.wrapping_add(1);
        }
        i = (*acn).namedusers as uint32_t;
        while i < ((*acn).namedusers as uint32_t).wrapping_add((*acn).namedgroups as uint32_t) {
            j = 0 as uint32_t;
            while j < agids {
                if *agid.offset(j as isize) == (*(*acn).acltab.offset(i as isize)).id {
                    modemask = (modemask as ::core::ffi::c_int
                        | modetoaccmode[((*(*acn).acltab.offset(i as isize)).perm
                            as ::core::ffi::c_int
                            & (*acn).mask as ::core::ffi::c_int
                            & 0x7 as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int)
                        as uint8_t;
                    f = 1 as ::core::ffi::c_int;
                }
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        if f == 1 as ::core::ffi::c_int {
            return modemask;
        }
        return modetoaccmode
            [((*acn).otherperm as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int) as usize];
    };
}
#[no_mangle]
pub unsafe extern "C" fn posix_acl_copydefaults(
    mut parent: uint32_t,
    mut inode: uint32_t,
    mut directory: uint8_t,
    mut mode: *mut uint16_t,
) -> uint8_t {
    let mut i: uint32_t = 0;
    let mut acls: uint32_t = 0;
    let mut ret: uint8_t = 0;
    let mut pacn: *mut acl_node = ::core::ptr::null_mut::<acl_node>();
    let mut acn: *mut acl_node = ::core::ptr::null_mut::<acl_node>();
    ret = 0 as uint8_t;
    pacn = posix_acl_xxx_find(parent, POSIX_ACL_DEFAULT as uint8_t);
    if pacn.is_null() {
        return ret;
    }
    acls = ((*pacn).namedusers as uint32_t).wrapping_add((*pacn).namedgroups as uint32_t);
    if acls == 0 as uint32_t
        && (*pacn).userperm as ::core::ffi::c_int <= 7 as ::core::ffi::c_int
        && (*pacn).groupperm as ::core::ffi::c_int <= 7 as ::core::ffi::c_int
        && (*pacn).otherperm as ::core::ffi::c_int <= 7 as ::core::ffi::c_int
        && (*pacn).mask as ::core::ffi::c_int == 0xffff as ::core::ffi::c_int
    {
        *mode = (*mode as ::core::ffi::c_int
            & (0xfe00 as ::core::ffi::c_int
                | ((*pacn).userperm as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                | ((*pacn).groupperm as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (*pacn).otherperm as ::core::ffi::c_int)) as uint16_t;
    } else {
        acn = posix_acl_xxx_find(inode, POSIX_ACL_ACCESS as uint8_t);
        if acn.is_null() {
            acn = posix_acl_create(inode, POSIX_ACL_ACCESS as uint8_t);
            ret = (ret as ::core::ffi::c_int | 1 as ::core::ffi::c_int) as uint8_t;
        }
        (*acn).userperm =
            ((*acn).userperm as ::core::ffi::c_int & 0xfff8 as ::core::ffi::c_int) as uint16_t;
        (*acn).userperm = ((*acn).userperm as ::core::ffi::c_int
            | *mode as ::core::ffi::c_int >> 6 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as uint16_t;
        (*acn).userperm = ((*acn).userperm as ::core::ffi::c_int
            & (*pacn).userperm as ::core::ffi::c_int) as uint16_t;
        (*acn).groupperm = (*pacn).groupperm;
        (*acn).otherperm =
            ((*acn).otherperm as ::core::ffi::c_int & 0xfff8 as ::core::ffi::c_int) as uint16_t;
        (*acn).otherperm = ((*acn).otherperm as ::core::ffi::c_int
            | *mode as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as uint16_t;
        (*acn).otherperm = ((*acn).otherperm as ::core::ffi::c_int
            & (*pacn).otherperm as ::core::ffi::c_int) as uint16_t;
        (*acn).mask =
            ((*acn).mask as ::core::ffi::c_int & 0xfff8 as ::core::ffi::c_int) as uint16_t;
        (*acn).mask = ((*acn).mask as ::core::ffi::c_int
            | *mode as ::core::ffi::c_int >> 3 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as uint16_t;
        (*acn).mask =
            ((*acn).mask as ::core::ffi::c_int & (*pacn).mask as ::core::ffi::c_int) as uint16_t;
        *mode = (*mode as ::core::ffi::c_int & 0xfe00 as ::core::ffi::c_int
            | ((*acn).userperm as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int
            | ((*acn).groupperm as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int
            | (*acn).otherperm as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
            as uint16_t;
        if ((*acn).namedusers as uint32_t).wrapping_add((*acn).namedgroups as uint32_t) != acls {
            if !(*acn).acltab.is_null() {
                free((*acn).acltab as *mut ::core::ffi::c_void);
            }
            if acls > 0 as uint32_t {
                (*acn).acltab =
                    malloc(::core::mem::size_of::<acl_entry>().wrapping_mul(acls as size_t))
                        as *mut acl_entry;
                if (*acn).acltab.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        203 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"acn->acltab\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        203 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"acn->acltab\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if (*acn).acltab
                    == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut acl_entry
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        203 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"acn->acltab\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        203 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"acn->acltab\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
            } else {
                (*acn).acltab = ::core::ptr::null_mut::<acl_entry>();
            }
        }
        (*acn).namedusers = (*pacn).namedusers;
        (*acn).namedgroups = (*pacn).namedgroups;
        i = 0 as uint32_t;
        while i < acls {
            (*(*acn).acltab.offset(i as isize)).id = (*(*pacn).acltab.offset(i as isize)).id;
            (*(*acn).acltab.offset(i as isize)).perm = (*(*pacn).acltab.offset(i as isize)).perm;
            i = i.wrapping_add(1);
        }
    }
    if directory != 0 {
        acn = posix_acl_xxx_find(inode, POSIX_ACL_DEFAULT as uint8_t);
        if acn.is_null() {
            acn = posix_acl_create(inode, POSIX_ACL_DEFAULT as uint8_t);
            ret = (ret as ::core::ffi::c_int | 2 as ::core::ffi::c_int) as uint8_t;
        }
        (*acn).userperm = (*pacn).userperm;
        (*acn).groupperm = (*pacn).groupperm;
        (*acn).otherperm = (*pacn).otherperm;
        (*acn).mask = (*pacn).mask;
        if ((*acn).namedusers as uint32_t).wrapping_add((*acn).namedgroups as uint32_t) != acls {
            if !(*acn).acltab.is_null() {
                free((*acn).acltab as *mut ::core::ffi::c_void);
            }
            if acls > 0 as uint32_t {
                (*acn).acltab =
                    malloc(::core::mem::size_of::<acl_entry>().wrapping_mul(acls as size_t))
                        as *mut acl_entry;
                if (*acn).acltab.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        231 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"acn->acltab\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        231 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"acn->acltab\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if (*acn).acltab
                    == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut acl_entry
                {
                    let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        231 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"acn->acltab\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        231 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"acn->acltab\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_0,
                    );
                    abort();
                }
            } else {
                (*acn).acltab = ::core::ptr::null_mut::<acl_entry>();
            }
        }
        (*acn).namedusers = (*pacn).namedusers;
        (*acn).namedgroups = (*pacn).namedgroups;
        i = 0 as uint32_t;
        while i < acls {
            (*(*acn).acltab.offset(i as isize)).id = (*(*pacn).acltab.offset(i as isize)).id;
            (*(*acn).acltab.offset(i as isize)).perm = (*(*pacn).acltab.offset(i as isize)).perm;
            i = i.wrapping_add(1);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn posix_acl_remove(mut inode: uint32_t, mut acltype: uint8_t) {
    let mut acn: *mut acl_node = ::core::ptr::null_mut::<acl_node>();
    acn = posix_acl_xxx_find(inode, acltype);
    if !acn.is_null() {
        posix_acl_xxx_delete(acn);
        if !(*acn).acltab.is_null() {
            free((*acn).acltab as *mut ::core::ffi::c_void);
        }
        free(acn as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn posix_acl_set(
    mut inode: uint32_t,
    mut acltype: uint8_t,
    mut userperm: uint16_t,
    mut groupperm: uint16_t,
    mut otherperm: uint16_t,
    mut mask: uint16_t,
    mut namedusers: uint16_t,
    mut namedgroups: uint16_t,
    mut aclblob: *const uint8_t,
) {
    let mut i: uint32_t = 0;
    let mut acls: uint32_t = 0;
    let mut acn: *mut acl_node = ::core::ptr::null_mut::<acl_node>();
    if acltype as ::core::ffi::c_int == POSIX_ACL_ACCESS
        && namedusers as ::core::ffi::c_int | namedgroups as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        && userperm as ::core::ffi::c_int <= 7 as ::core::ffi::c_int
        && groupperm as ::core::ffi::c_int <= 7 as ::core::ffi::c_int
        && otherperm as ::core::ffi::c_int <= 7 as ::core::ffi::c_int
        && mask as ::core::ffi::c_int == 0xffff as ::core::ffi::c_int
    {
        posix_acl_remove(inode, acltype);
        fs_del_aclflag(inode, acltype);
        return;
    }
    acn = posix_acl_xxx_find(inode, acltype);
    if acn.is_null() {
        acn = posix_acl_create(inode, acltype);
        fs_set_aclflag(inode, acltype);
    }
    acls = (namedusers as uint32_t).wrapping_add(namedgroups as uint32_t);
    (*acn).userperm = userperm;
    (*acn).groupperm = groupperm;
    (*acn).otherperm = otherperm;
    (*acn).mask = mask;
    if ((*acn).namedusers as uint32_t).wrapping_add((*acn).namedgroups as uint32_t) != acls {
        if !(*acn).acltab.is_null() {
            free((*acn).acltab as *mut ::core::ffi::c_void);
        }
        if acls > 0 as uint32_t {
            (*acn).acltab = malloc(::core::mem::size_of::<acl_entry>().wrapping_mul(acls as size_t))
                as *mut acl_entry;
            if (*acn).acltab.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    285 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"acn->acltab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    285 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"acn->acltab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if (*acn).acltab
                == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut acl_entry
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    285 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"acn->acltab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    285 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"acn->acltab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
        } else {
            (*acn).acltab = ::core::ptr::null_mut::<acl_entry>();
        }
    }
    (*acn).namedusers = namedusers;
    (*acn).namedgroups = namedgroups;
    i = 0 as uint32_t;
    while i < acls {
        (*(*acn).acltab.offset(i as isize)).id = get32bit(&raw mut aclblob);
        (*(*acn).acltab.offset(i as isize)).perm = get16bit(&raw mut aclblob);
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn posix_acl_get_blobsize(
    mut inode: uint32_t,
    mut acltype: uint8_t,
    mut aclnode: *mut *mut ::core::ffi::c_void,
) -> int32_t {
    let mut acn: *mut acl_node = ::core::ptr::null_mut::<acl_node>();
    acn = posix_acl_xxx_find(inode, acltype);
    *aclnode = acn as *mut ::core::ffi::c_void;
    if acn.is_null() {
        return -1 as int32_t;
    } else {
        return ((*acn).namedusers as uint32_t)
            .wrapping_add((*acn).namedgroups as uint32_t)
            .wrapping_mul(6 as uint32_t) as int32_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn posix_acl_get_data(
    mut aclnode: *mut ::core::ffi::c_void,
    mut userperm: *mut uint16_t,
    mut groupperm: *mut uint16_t,
    mut otherperm: *mut uint16_t,
    mut mask: *mut uint16_t,
    mut namedusers: *mut uint16_t,
    mut namedgroups: *mut uint16_t,
    mut aclblob: *mut uint8_t,
) {
    let mut acn: *mut acl_node = ::core::ptr::null_mut::<acl_node>();
    let mut i: uint32_t = 0;
    let mut acls: uint32_t = 0;
    acn = aclnode as *mut acl_node;
    *userperm = (*acn).userperm;
    *groupperm = (*acn).groupperm;
    *otherperm = (*acn).otherperm;
    *mask = (*acn).mask;
    *namedusers = (*acn).namedusers;
    *namedgroups = (*acn).namedgroups;
    acls = ((*acn).namedusers as uint32_t).wrapping_add((*acn).namedgroups as uint32_t);
    i = 0 as uint32_t;
    while i < acls {
        put32bit(&raw mut aclblob, (*(*acn).acltab.offset(i as isize)).id);
        put16bit(&raw mut aclblob, (*(*acn).acltab.offset(i as isize)).perm);
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn posix_acl_getall(
    mut inode: uint32_t,
    mut acltype: uint8_t,
    mut dbuff: *mut uint8_t,
) -> uint32_t {
    let mut acn: *mut acl_node = ::core::ptr::null_mut::<acl_node>();
    let mut i: uint32_t = 0;
    let mut acls: uint32_t = 0;
    acn = posix_acl_xxx_find(inode, acltype);
    if acn.is_null() {
        return 0 as uint32_t;
    } else {
        acls = ((*acn).namedusers as uint32_t).wrapping_add((*acn).namedgroups as uint32_t);
        if !dbuff.is_null() {
            put16bit(&raw mut dbuff, (*acn).userperm);
            put16bit(&raw mut dbuff, (*acn).groupperm);
            put16bit(&raw mut dbuff, (*acn).otherperm);
            put16bit(&raw mut dbuff, (*acn).mask);
            put16bit(&raw mut dbuff, (*acn).namedusers);
            put16bit(&raw mut dbuff, (*acn).namedgroups);
            i = 0 as uint32_t;
            while i < acls {
                put32bit(&raw mut dbuff, (*(*acn).acltab.offset(i as isize)).id);
                put16bit(&raw mut dbuff, (*(*acn).acltab.offset(i as isize)).perm);
                i = i.wrapping_add(1);
            }
        }
        return acls
            .wrapping_mul(6 as uint32_t)
            .wrapping_add(12 as uint32_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn posix_acl_check(
    mut inode: uint32_t,
    mut acltype: uint8_t,
    mut userperm: uint16_t,
    mut groupperm: uint16_t,
    mut otherperm: uint16_t,
    mut mask: uint16_t,
    mut namedusers: uint16_t,
    mut namedgroups: uint16_t,
    mut aclblob: *const uint8_t,
) -> uint8_t {
    let mut acn: *mut acl_node = ::core::ptr::null_mut::<acl_node>();
    let mut i: uint16_t = 0;
    let mut j: uint32_t = 0;
    let mut checkcnt: uint16_t = 0;
    let mut id: uint32_t = 0;
    let mut perm: uint16_t = 0;
    acn = posix_acl_xxx_find(inode, acltype);
    if acn.is_null() {
        return 0 as uint8_t;
    }
    if userperm as ::core::ffi::c_int != (*acn).userperm as ::core::ffi::c_int
        || groupperm as ::core::ffi::c_int != (*acn).groupperm as ::core::ffi::c_int
        || otherperm as ::core::ffi::c_int != (*acn).otherperm as ::core::ffi::c_int
        || mask as ::core::ffi::c_int != (*acn).mask as ::core::ffi::c_int
        || namedusers as ::core::ffi::c_int != (*acn).namedusers as ::core::ffi::c_int
        || namedgroups as ::core::ffi::c_int != (*acn).namedgroups as ::core::ffi::c_int
    {
        return 0 as uint8_t;
    }
    checkcnt = 0 as uint16_t;
    i = 0 as uint16_t;
    while (i as ::core::ffi::c_int) < namedusers as ::core::ffi::c_int {
        id = get32bit(&raw mut aclblob);
        perm = get16bit(&raw mut aclblob);
        j = 0 as uint32_t;
        while j < namedusers as uint32_t {
            if (*(*acn).acltab.offset(j as isize)).id == id
                && (*(*acn).acltab.offset(j as isize)).perm as ::core::ffi::c_int
                    == perm as ::core::ffi::c_int
            {
                checkcnt = checkcnt.wrapping_add(1);
            }
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    i = 0 as uint16_t;
    while (i as ::core::ffi::c_int) < namedgroups as ::core::ffi::c_int {
        id = get32bit(&raw mut aclblob);
        perm = get16bit(&raw mut aclblob);
        j = namedusers as uint32_t;
        while j < (namedusers as uint32_t).wrapping_add(namedgroups as uint32_t) {
            if (*(*acn).acltab.offset(j as isize)).id == id
                && (*(*acn).acltab.offset(j as isize)).perm as ::core::ffi::c_int
                    == perm as ::core::ffi::c_int
            {
                checkcnt = checkcnt.wrapping_add(1);
            }
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    return (if checkcnt as uint32_t
        != (namedusers as uint32_t).wrapping_add(namedgroups as uint32_t)
    {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    }) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn posix_acl_copy(
    mut srcinode: uint32_t,
    mut dstinode: uint32_t,
    mut acltype: uint8_t,
) -> uint8_t {
    let mut acls: uint32_t = 0;
    let mut sacn: *mut acl_node = ::core::ptr::null_mut::<acl_node>();
    let mut dacn: *mut acl_node = ::core::ptr::null_mut::<acl_node>();
    sacn = posix_acl_xxx_find(srcinode, acltype);
    dacn = posix_acl_xxx_find(dstinode, acltype);
    if dacn.is_null() {
        dacn = malloc(::core::mem::size_of::<acl_node>()) as *mut acl_node;
        if dacn.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr() as *const ::core::ffi::c_char,
                401 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dacn\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr() as *const ::core::ffi::c_char,
                401 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dacn\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if dacn
            == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut acl_node
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr() as *const ::core::ffi::c_char,
                401 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dacn\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr() as *const ::core::ffi::c_char,
                401 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dacn\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*dacn).inode = dstinode;
        (*dacn).acltype = acltype;
        posix_acl_xxx_add(dacn);
    } else if !(*dacn).acltab.is_null() {
        free((*dacn).acltab as *mut ::core::ffi::c_void);
    }
    (*dacn).userperm = (*sacn).userperm;
    (*dacn).groupperm = (*sacn).groupperm;
    (*dacn).otherperm = (*sacn).otherperm;
    (*dacn).mask = (*sacn).mask;
    (*dacn).namedusers = (*sacn).namedusers;
    (*dacn).namedgroups = (*sacn).namedgroups;
    acls = ((*sacn).namedusers as uint32_t).wrapping_add((*sacn).namedgroups as uint32_t);
    if acls > 0 as uint32_t {
        (*dacn).acltab = malloc(::core::mem::size_of::<acl_entry>().wrapping_mul(acls as size_t))
            as *mut acl_entry;
        if (*dacn).acltab.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr() as *const ::core::ffi::c_char,
                419 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dacn->acltab\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr() as *const ::core::ffi::c_char,
                419 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dacn->acltab\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if (*dacn).acltab
            == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut acl_entry
        {
            let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr() as *const ::core::ffi::c_char,
                419 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dacn->acltab\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr() as *const ::core::ffi::c_char,
                419 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dacn->acltab\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            abort();
        }
        memcpy(
            (*dacn).acltab as *mut ::core::ffi::c_void,
            (*sacn).acltab as *const ::core::ffi::c_void,
            ::core::mem::size_of::<acl_entry>().wrapping_mul(acls as size_t),
        );
    } else {
        (*dacn).acltab = ::core::ptr::null_mut::<acl_entry>();
    }
    return 1 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn posix_acl_cleanup() {
    let mut i: uint32_t = 0;
    let mut j: uint32_t = 0;
    let mut acn: *mut acl_node = ::core::ptr::null_mut::<acl_node>();
    let mut nacn: *mut acl_node = ::core::ptr::null_mut::<acl_node>();
    i = 0 as uint32_t;
    while i < HASHTAB_HISIZE as uint32_t {
        if !paclhashtab[i as usize].is_null() {
            j = 0 as uint32_t;
            while j < HASHTAB_LOSIZE as uint32_t {
                acn = *paclhashtab[i as usize].offset(j as isize);
                while !acn.is_null() {
                    nacn = (*acn).next as *mut acl_node;
                    if !(*acn).acltab.is_null() {
                        free((*acn).acltab as *mut ::core::ffi::c_void);
                    }
                    free(acn as *mut ::core::ffi::c_void);
                    acn = nacn;
                }
                *paclhashtab[i as usize].offset(j as isize) = ::core::ptr::null_mut::<acl_node>();
                j = j.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }
    posix_acl_xxx_hash_cleanup();
}
#[no_mangle]
pub unsafe extern "C" fn posix_acl_store(mut fd: *mut bio) -> uint8_t {
    let mut hdrbuff: [uint8_t; 17] = [0; 17];
    let mut aclbuff: [uint8_t; 600] = [0; 600];
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut i: uint32_t = 0;
    let mut j: uint32_t = 0;
    let mut accnt: uint32_t = 0;
    let mut acbcnt: uint32_t = 0;
    let mut acn: *mut acl_node = ::core::ptr::null_mut::<acl_node>();
    if fd.is_null() {
        return 0x11 as uint8_t;
    }
    i = 0 as uint32_t;
    while i < HASHTAB_HISIZE as uint32_t {
        if !paclhashtab[i as usize].is_null() {
            j = 0 as uint32_t;
            while j < HASHTAB_LOSIZE as uint32_t {
                acn = *paclhashtab[i as usize].offset(j as isize);
                while !acn.is_null() {
                    ptr = &raw mut hdrbuff as *mut uint8_t;
                    put32bit(&raw mut ptr, (*acn).inode);
                    put8bit(&raw mut ptr, (*acn).acltype);
                    put16bit(&raw mut ptr, (*acn).userperm);
                    put16bit(&raw mut ptr, (*acn).groupperm);
                    put16bit(&raw mut ptr, (*acn).otherperm);
                    put16bit(&raw mut ptr, (*acn).mask);
                    put16bit(&raw mut ptr, (*acn).namedusers);
                    put16bit(&raw mut ptr, (*acn).namedgroups);
                    if bio_write(
                        fd,
                        &raw mut hdrbuff as *mut uint8_t as *const ::core::ffi::c_void,
                        (4 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 6 as ::core::ffi::c_int)
                            as uint64_t,
                    ) != (4 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 6 as ::core::ffi::c_int)
                        as int64_t
                    {
                        return 0xff as uint8_t;
                    }
                    accnt = 0 as uint32_t;
                    acbcnt = 0 as uint32_t;
                    ptr = &raw mut aclbuff as *mut uint8_t;
                    while accnt
                        < ((*acn).namedusers as uint32_t)
                            .wrapping_add((*acn).namedgroups as uint32_t)
                    {
                        if acbcnt == 100 as uint32_t {
                            if bio_write(
                                fd,
                                &raw mut aclbuff as *mut uint8_t as *const ::core::ffi::c_void,
                                (6 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as uint64_t,
                            ) != (6 as ::core::ffi::c_int * 100 as ::core::ffi::c_int) as int64_t
                            {
                                return 0xff as uint8_t;
                            }
                            acbcnt = 0 as uint32_t;
                            ptr = &raw mut aclbuff as *mut uint8_t;
                        }
                        put32bit(&raw mut ptr, (*(*acn).acltab.offset(accnt as isize)).id);
                        put16bit(&raw mut ptr, (*(*acn).acltab.offset(accnt as isize)).perm);
                        accnt = accnt.wrapping_add(1);
                        acbcnt = acbcnt.wrapping_add(1);
                    }
                    if acbcnt > 0 as uint32_t {
                        if bio_write(
                            fd,
                            &raw mut aclbuff as *mut uint8_t as *const ::core::ffi::c_void,
                            (6 as uint32_t).wrapping_mul(acbcnt) as uint64_t,
                        ) != (6 as uint32_t).wrapping_mul(acbcnt) as int64_t
                        {
                            return 0xff as uint8_t;
                        }
                    }
                    acn = (*acn).next as *mut acl_node;
                }
                j = j.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }
    memset(
        &raw mut hdrbuff as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as size_t,
    );
    if bio_write(
        fd,
        &raw mut hdrbuff as *mut uint8_t as *const ::core::ffi::c_void,
        (4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as uint64_t,
    ) != (4 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as int64_t
    {
        return 0xff as uint8_t;
    }
    return 0 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn posix_acl_load(
    mut fd: *mut bio,
    mut mver: uint8_t,
    mut ignoreflag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut hdrbuff: [uint8_t; 17] = [0; 17];
    let mut aclbuff: [uint8_t; 600] = [0; 600];
    let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut inode: uint32_t = 0;
    let mut acltype: uint8_t = 0;
    let mut userperm: uint16_t = 0;
    let mut groupperm: uint16_t = 0;
    let mut otherperm: uint16_t = 0;
    let mut mask: uint16_t = 0;
    let mut namedusers: uint16_t = 0;
    let mut namedgroups: uint16_t = 0;
    let mut i: uint32_t = 0;
    let mut acls: uint32_t = 0;
    let mut acbcnt: uint32_t = 0;
    let mut nl: uint8_t = 1 as uint8_t;
    let mut acn: *mut acl_node = ::core::ptr::null_mut::<acl_node>();
    loop {
        if bio_read(
            fd,
            &raw mut hdrbuff as *mut uint8_t as *mut ::core::ffi::c_void,
            (4 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as uint64_t,
        ) != (4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as int64_t
        {
            let mut err: ::core::ffi::c_int = *__errno_location();
            if nl != 0 {
                fputc('\n' as ::core::ffi::c_int, stderr);
            }
            *__errno_location() = err;
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"loading posix_acl: read error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = &raw mut hdrbuff as *mut uint8_t;
        inode = get32bit(&raw mut ptr);
        if inode == 0 as uint32_t {
            return 1 as ::core::ffi::c_int;
        }
        acltype = get8bit(&raw mut ptr);
        userperm = get16bit(&raw mut ptr);
        groupperm = get16bit(&raw mut ptr);
        otherperm = get16bit(&raw mut ptr);
        mask = get16bit(&raw mut ptr);
        namedusers = get16bit(&raw mut ptr);
        namedgroups = get16bit(&raw mut ptr);
        acls = (namedusers as uint32_t).wrapping_add(namedgroups as uint32_t);
        if fs_check_inode(inode) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            bio_skip(fd, (6 as uint32_t).wrapping_mul(acls) as uint64_t);
        } else if acltype as ::core::ffi::c_int != POSIX_ACL_ACCESS
            && acltype as ::core::ffi::c_int != POSIX_ACL_DEFAULT
        {
            if nl != 0 {
                fputc('\n' as ::core::ffi::c_int, stderr);
                nl = 0 as uint8_t;
            }
            if ignoreflag != 0 {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_NOTICE,
                    b"loading posix_acl: wrong acl type - ignoring\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                bio_skip(fd, (6 as uint32_t).wrapping_mul(acls) as uint64_t);
            } else {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"loading posix_acl: wrong acl type\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
        } else {
            acn = posix_acl_xxx_find(inode, acltype);
            if !acn.is_null() {
                if nl != 0 {
                    fputc('\n' as ::core::ffi::c_int, stderr);
                    nl = 0 as uint8_t;
                }
                if ignoreflag != 0 {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_NOTICE,
                        b"loading posix_acl: repeated acl - ignoring\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    bio_skip(fd, (6 as uint32_t).wrapping_mul(acls) as uint64_t);
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"loading posix_acl: repeated acl\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            } else {
                acn = posix_acl_create(inode, acltype);
                fs_set_aclflag(inode, acltype);
                if mver as ::core::ffi::c_int == 0x10 as ::core::ffi::c_int
                    && mask as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    let mut mode: uint16_t = 0;
                    mode = fs_get_mode(inode);
                    userperm =
                        (userperm as ::core::ffi::c_int & 0xfff8 as ::core::ffi::c_int) as uint16_t;
                    userperm = (userperm as ::core::ffi::c_int
                        | mode as ::core::ffi::c_int >> 6 as ::core::ffi::c_int
                            & 7 as ::core::ffi::c_int) as uint16_t;
                    mask = 7 as uint16_t;
                    otherperm = (otherperm as ::core::ffi::c_int & 0xfff8 as ::core::ffi::c_int)
                        as uint16_t;
                    otherperm = (otherperm as ::core::ffi::c_int
                        | mode as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        as uint16_t;
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"emergency set ACL mask for inode %u to 'rwx'\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        inode,
                    );
                }
                (*acn).userperm = userperm;
                (*acn).groupperm = groupperm;
                (*acn).otherperm = otherperm;
                (*acn).mask = mask;
                (*acn).namedusers = namedusers;
                (*acn).namedgroups = namedgroups;
                if acls > 0 as uint32_t {
                    (*acn).acltab =
                        malloc(::core::mem::size_of::<acl_entry>().wrapping_mul(acls as size_t))
                            as *mut acl_entry;
                    if (*acn).acltab.is_null() {
                        fprintf(
                            stderr,
                            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            600 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"acn->acltab\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            600 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"acn->acltab\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        abort();
                    } else if (*acn).acltab
                        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                            -1 as ::core::ffi::c_int as usize,
                        ) as *mut acl_entry
                    {
                        let mut _mfs_errorstring: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            600 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"acn->acltab\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/posixacl.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            600 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"acn->acltab\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring,
                        );
                        abort();
                    }
                } else {
                    (*acn).acltab = ::core::ptr::null_mut::<acl_entry>();
                }
                acbcnt = 0 as uint32_t;
                i = 0 as uint32_t;
                while i < acls {
                    if acbcnt == 0 as uint32_t {
                        acbcnt = acls.wrapping_sub(i);
                        if acbcnt > 100 as uint32_t {
                            acbcnt = 100 as uint32_t;
                        }
                        if bio_read(
                            fd,
                            &raw mut aclbuff as *mut uint8_t as *mut ::core::ffi::c_void,
                            (6 as uint32_t).wrapping_mul(acbcnt) as uint64_t,
                        ) != (6 as uint32_t).wrapping_mul(acbcnt) as int64_t
                        {
                            let mut err_0: ::core::ffi::c_int = *__errno_location();
                            if nl != 0 {
                                fputc('\n' as ::core::ffi::c_int, stderr);
                            }
                            posix_acl_xxx_delete(acn);
                            if !(*acn).acltab.is_null() {
                                free((*acn).acltab as *mut ::core::ffi::c_void);
                            }
                            free(acn as *mut ::core::ffi::c_void);
                            *__errno_location() = err_0;
                            mfs_log(
                                MFSLOG_ERRNO_SYSLOG_STDERR,
                                MFSLOG_ERR,
                                b"loading posix_acl: read error\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                        ptr = &raw mut aclbuff as *mut uint8_t;
                    }
                    (*(*acn).acltab.offset(i as isize)).id = get32bit(&raw mut ptr);
                    (*(*acn).acltab.offset(i as isize)).perm = get16bit(&raw mut ptr);
                    acbcnt = acbcnt.wrapping_sub(1);
                    i = i.wrapping_add(1);
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn posix_acl_init() -> ::core::ffi::c_int {
    posix_acl_xxx_hash_init();
    return 0 as ::core::ffi::c_int;
}
