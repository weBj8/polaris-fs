pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
pub enum _bio {}
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    static mut stderr: *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn fputc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn mmap(
        __addr: *mut ::core::ffi::c_void,
        __len: size_t,
        __prot: ::core::ffi::c_int,
        __flags: ::core::ffi::c_int,
        __fd: ::core::ffi::c_int,
        __offset: __off64_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn munmap(__addr: *mut ::core::ffi::c_void, __len: size_t) -> ::core::ffi::c_int;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn bio_read(b: *mut bio, dst: *mut ::core::ffi::c_void, len: uint64_t) -> int64_t;
    unsafe fn bio_write(b: *mut bio, src: *const ::core::ffi::c_void, len: uint64_t) -> int64_t;
    unsafe fn bio_skip(b: *mut bio, len: uint64_t);
    unsafe fn fs_check_inode(inode: uint32_t) -> uint8_t;
    unsafe fn fs_set_xattrflag(inode: uint32_t);
    unsafe fn fs_del_xattrflag(inode: uint32_t);
    unsafe fn dict_search(data: *const uint8_t, leng: uint32_t) -> *mut ::core::ffi::c_void;
    unsafe fn dict_insert(data: *const uint8_t, leng: uint32_t) -> *mut ::core::ffi::c_void;
    unsafe fn dict_get_ptr(dptr: *mut ::core::ffi::c_void) -> *const uint8_t;
    unsafe fn dict_get_leng(dptr: *mut ::core::ffi::c_void) -> uint32_t;
    unsafe fn dict_dec_ref(dptr: *mut ::core::ffi::c_void);
    unsafe fn dict_inc_ref(dptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
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
pub type xattrentry = _xattrentry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xattrentry {
    pub next: *mut _xattrentry,
    pub inode: uint32_t,
    pub pairhead: *mut xattrpair,
}
pub type xattrpair = _xattrpair;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xattrpair {
    pub next: *mut _xattrpair,
    pub dictname: *mut ::core::ffi::c_void,
    pub dictvalue: *mut ::core::ffi::c_void,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const PROT_READ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PROT_WRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MAP_PRIVATE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MAP_ANONYMOUS: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const MAP_ANON: ::core::ffi::c_int = MAP_ANONYMOUS;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_EEXIST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MFS_ERROR_EINVAL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MFS_ERROR_ENOATTR: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const MFS_ERROR_ERANGE: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFS_XATTR_CREATE_OR_REPLACE: ::core::ffi::c_int = 0;
pub const MFS_XATTR_CREATE_ONLY: ::core::ffi::c_int = 1;
pub const MFS_XATTR_REPLACE_ONLY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFS_XATTR_REMOVE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFS_XATTR_NAME_MAX: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
pub const MFS_XATTR_SIZE_MAX: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
pub const MFS_XATTR_LIST_MAX: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_ERRNO_SYSLOG_STDERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn put32bit(mut ptr: *mut *mut uint8_t, mut val: uint32_t) {
    unsafe {
        val = val.swap_bytes() as uint32_t;
        memcpy(
            *ptr as *mut ::core::ffi::c_void,
            &raw mut val as *const ::core::ffi::c_void,
            4 as size_t,
        );
        *ptr = (*ptr).offset(4 as ::core::ffi::c_int as isize);
    }
}
#[inline]
unsafe extern "C" fn put16bit(mut ptr: *mut *mut uint8_t, mut val: uint16_t) {
    unsafe {
        val = val.swap_bytes() as uint16_t;
        memcpy(
            *ptr as *mut ::core::ffi::c_void,
            &raw mut val as *const ::core::ffi::c_void,
            2 as size_t,
        );
        *ptr = (*ptr).offset(2 as ::core::ffi::c_int as isize);
    }
}
#[inline]
unsafe extern "C" fn put8bit(mut ptr: *mut *mut uint8_t, mut val: uint8_t) {
    unsafe {
        *(*ptr).offset(0 as isize) =
            (val as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
        *ptr = (*ptr).offset(1);
    }
}
#[inline]
unsafe extern "C" fn get32bit(mut ptr: *mut *const uint8_t) -> uint32_t {
    unsafe {
        let mut t32: uint32_t = 0;
        memcpy(
            &raw mut t32 as *mut ::core::ffi::c_void,
            *ptr as *const ::core::ffi::c_void,
            4 as size_t,
        );
        *ptr = (*ptr).offset(4 as ::core::ffi::c_int as isize);
        return t32.swap_bytes();
    }
}
#[inline]
unsafe extern "C" fn get16bit(mut ptr: *mut *const uint8_t) -> uint16_t {
    unsafe {
        let mut t16: uint16_t = 0;
        memcpy(
            &raw mut t16 as *mut ::core::ffi::c_void,
            *ptr as *const ::core::ffi::c_void,
            2 as size_t,
        );
        *ptr = (*ptr).offset(2 as ::core::ffi::c_int as isize);
        return t16.swap_bytes();
    }
}
#[inline]
unsafe extern "C" fn get8bit(mut ptr: *mut *const uint8_t) -> uint8_t {
    unsafe {
        let mut t8: uint8_t = 0;
        t8 = *(*ptr).offset(0 as isize);
        *ptr = (*ptr).offset(1);
        return t8;
    }
}
pub const LOHASH_BITS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn xattr_cmp(mut e: *mut xattrentry, mut inode: uint32_t) -> ::core::ffi::c_int {
    unsafe {
        return ((*e).inode == inode) as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn xattr_hash(mut inode: uint32_t) -> uint32_t {
    unsafe {
        return inode;
    }
}
#[inline]
unsafe extern "C" fn xattr_ehash(mut e: *mut xattrentry) -> uint32_t {
    unsafe {
        return (*e).inode;
    }
}
pub const HASHTAB_LOBITS: ::core::ffi::c_int = LOHASH_BITS;
pub const HASHTAB_HISIZE: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint >> HASHTAB_LOBITS;
pub const HASHTAB_LOSIZE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << HASHTAB_LOBITS;
pub const HASHTAB_MASK: ::core::ffi::c_int = HASHTAB_LOSIZE - 1 as ::core::ffi::c_int;
pub const HASHTAB_MOVEFACTOR: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const HASHTAB_SIZEHINT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut xattrhashtab: [*mut *mut xattrentry; 2048] =
    [::core::ptr::null_mut::<*mut xattrentry>(); 2048];
static mut xattrrehashpos: uint32_t = 0;
static mut xattrhashsize: uint32_t = 0;
static mut xattrhashelem: uint32_t = 0;
#[inline]
unsafe extern "C" fn xattr_calc_hash_size(mut elements: uint32_t) -> uint32_t {
    unsafe {
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
}
#[inline]
unsafe extern "C" fn xattr_hash_init() {
    unsafe {
        let mut i: uint16_t = 0;
        xattrhashsize = 0 as uint32_t;
        xattrhashelem = 0 as uint32_t;
        xattrrehashpos = 0 as uint32_t;
        i = 0 as uint16_t;
        while (i as ::core::ffi::c_uint) < HASHTAB_HISIZE {
            xattrhashtab[i as usize] = ::core::ptr::null_mut::<*mut xattrentry>();
            i = i.wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C" fn xattr_hash_cleanup() {
    unsafe {
        let mut i: uint16_t = 0;
        let mut j: uint32_t = 0;
        xattrhashelem = 0 as uint32_t;
        xattrhashsize = 0 as uint32_t;
        xattrrehashpos = 0 as uint32_t;
        i = 0 as uint16_t;
        while (i as ::core::ffi::c_uint) < HASHTAB_HISIZE {
            if !xattrhashtab[i as usize].is_null() {
                j = 0 as uint32_t;
                while j < HASHTAB_LOSIZE as uint32_t {
                    if (*xattrhashtab[i as usize].offset(j as isize)).is_null() {
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
                    xattrhashtab[i as usize] as *mut ::core::ffi::c_void,
                    ::core::mem::size_of::<*mut xattrentry>()
                        .wrapping_mul(HASHTAB_LOSIZE as size_t),
                );
            }
            xattrhashtab[i as usize] = ::core::ptr::null_mut::<*mut xattrentry>();
            i = i.wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C" fn xattr_hash_move() {
    unsafe {
        let mut hash: uint32_t = 0;
        let mut mask: uint32_t = 0;
        let mut moved: uint32_t = 0 as uint32_t;
        let mut ehptr: *mut *mut xattrentry = ::core::ptr::null_mut::<*mut xattrentry>();
        let mut ehptralt: *mut *mut xattrentry = ::core::ptr::null_mut::<*mut xattrentry>();
        let mut e: *mut xattrentry = ::core::ptr::null_mut::<xattrentry>();
        mask = xattrhashsize.wrapping_sub(1 as uint32_t);
        loop {
            if xattrrehashpos >= xattrhashsize {
                xattrrehashpos = xattrhashsize;
                return;
            }
            if xattrhashtab[(xattrrehashpos >> HASHTAB_LOBITS) as usize].is_null() {
                xattrhashtab[(xattrrehashpos >> HASHTAB_LOBITS) as usize] = mmap(
                    NULL,
                    ::core::mem::size_of::<*mut xattrentry>()
                        .wrapping_mul(HASHTAB_LOSIZE as size_t),
                    PROT_READ | PROT_WRITE,
                    MAP_ANON | MAP_PRIVATE,
                    -1 as ::core::ffi::c_int,
                    0 as __off64_t,
                )
                    as *mut *mut xattrentry;
                if xattrhashtab[(xattrrehashpos >> 20 as ::core::ffi::c_int) as usize].is_null() {
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
                } else if xattrhashtab[(xattrrehashpos >> 20 as ::core::ffi::c_int) as usize]
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut *mut xattrentry
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
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
            ehptr = xattrhashtab[(xattrrehashpos
                .wrapping_sub(xattrhashsize.wrapping_div(2 as uint32_t))
                >> HASHTAB_LOBITS) as usize]
                .offset((xattrrehashpos & HASHTAB_MASK as uint32_t) as isize);
            ehptralt = xattrhashtab[(xattrrehashpos >> HASHTAB_LOBITS) as usize]
                .offset((xattrrehashpos & HASHTAB_MASK as uint32_t) as isize);
            *ehptralt = ::core::ptr::null_mut::<xattrentry>();
            loop {
                e = *ehptr;
                if e.is_null() {
                    break;
                }
                hash = xattr_ehash(e) & mask;
                if hash == xattrrehashpos {
                    *ehptralt = e;
                    *ehptr = (*e).next as *mut xattrentry;
                    ehptralt = &raw mut (*e).next as *mut *mut xattrentry;
                    (*e).next = ::core::ptr::null_mut::<_xattrentry>();
                } else {
                    ehptr = &raw mut (*e).next as *mut *mut xattrentry;
                }
                moved = moved.wrapping_add(1);
            }
            xattrrehashpos = xattrrehashpos.wrapping_add(1);
            if moved >= HASHTAB_MOVEFACTOR as uint32_t {
                break;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn xattr_find(mut inode: uint32_t) -> *mut xattrentry {
    unsafe {
        let mut e: *mut xattrentry = ::core::ptr::null_mut::<xattrentry>();
        let mut hash: uint32_t = 0;
        let mut hashval: uint32_t = 0;
        if xattrhashsize == 0 as uint32_t {
            return ::core::ptr::null_mut::<xattrentry>();
        }
        hashval = xattr_hash(inode);
        hash = hashval & xattrhashsize.wrapping_sub(1 as uint32_t);
        if xattrrehashpos < xattrhashsize {
            xattr_hash_move();
            if hash >= xattrrehashpos {
                hash = hash.wrapping_sub(xattrhashsize.wrapping_div(2 as uint32_t));
            }
        }
        e = *xattrhashtab[(hash >> HASHTAB_LOBITS) as usize]
            .offset((hash & HASHTAB_MASK as uint32_t) as isize);
        while !e.is_null() {
            if xattr_cmp(e, inode) != 0 {
                return e;
            }
            e = (*e).next as *mut xattrentry;
        }
        return ::core::ptr::null_mut::<xattrentry>();
    }
}
#[inline]
unsafe extern "C" fn xattr_delete(mut e: *mut xattrentry) -> uint8_t {
    unsafe {
        let mut ehptr: *mut *mut xattrentry = ::core::ptr::null_mut::<*mut xattrentry>();
        let mut eit: *mut xattrentry = ::core::ptr::null_mut::<xattrentry>();
        let mut hash: uint32_t = 0;
        if xattrhashsize == 0 as uint32_t {
            return 0 as uint8_t;
        }
        hash = xattr_ehash(e) & xattrhashsize.wrapping_sub(1 as uint32_t);
        if xattrrehashpos < xattrhashsize {
            xattr_hash_move();
            if hash >= xattrrehashpos {
                hash = hash.wrapping_sub(xattrhashsize.wrapping_div(2 as uint32_t));
            }
        }
        ehptr = xattrhashtab[(hash >> HASHTAB_LOBITS) as usize]
            .offset((hash & HASHTAB_MASK as uint32_t) as isize);
        loop {
            eit = *ehptr;
            if eit.is_null() {
                break;
            }
            if eit == e {
                *ehptr = (*e).next as *mut xattrentry;
                xattrhashelem = xattrhashelem.wrapping_sub(1);
                return 1 as uint8_t;
            }
            ehptr = &raw mut (*eit).next as *mut *mut xattrentry;
        }
        return 0 as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn xattr_add(mut e: *mut xattrentry) {
    unsafe {
        let mut i: uint16_t = 0;
        let mut hash: uint32_t = 0;
        if xattrhashsize == 0 as uint32_t {
            xattrhashsize = xattr_calc_hash_size(HASHTAB_SIZEHINT as uint32_t);
            xattrrehashpos = xattrhashsize;
            xattrhashelem = 0 as uint32_t;
            i = 0 as uint16_t;
            while (i as uint32_t) < xattrhashsize >> HASHTAB_LOBITS {
                xattrhashtab[i as usize] = mmap(
                    NULL,
                    ::core::mem::size_of::<*mut xattrentry>()
                        .wrapping_mul(HASHTAB_LOSIZE as size_t),
                    PROT_READ | PROT_WRITE,
                    MAP_ANON | MAP_PRIVATE,
                    -1 as ::core::ffi::c_int,
                    0 as __off64_t,
                ) as *mut *mut xattrentry;
                if xattrhashtab[i as usize].is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfscommon/hash_begin.h\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        284 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"GLUE_HASH_TAB_PREFIX(hashtab)[i]\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfscommon/hash_begin.h\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        284 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"GLUE_HASH_TAB_PREFIX(hashtab)[i]\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if xattrhashtab[i as usize]
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut *mut xattrentry
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfscommon/hash_begin.h\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        284 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"GLUE_HASH_TAB_PREFIX(hashtab)[i]\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfscommon/hash_begin.h\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        284 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"GLUE_HASH_TAB_PREFIX(hashtab)[i]\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
                memset(
                    xattrhashtab[i as usize] as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<*mut xattrentry>(),
                );
                if (*xattrhashtab[i as usize].offset(0 as isize)).is_null() {
                    memset(
                        xattrhashtab[i as usize] as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<*mut xattrentry>()
                            .wrapping_mul(HASHTAB_LOSIZE as size_t),
                    );
                } else {
                    hash = 0 as uint32_t;
                    while hash < HASHTAB_LOSIZE as uint32_t {
                        *xattrhashtab[i as usize].offset(hash as isize) =
                            ::core::ptr::null_mut::<xattrentry>();
                        hash = hash.wrapping_add(1);
                    }
                }
                i = i.wrapping_add(1);
            }
        }
        hash = xattr_ehash(e) & xattrhashsize.wrapping_sub(1 as uint32_t);
        if xattrrehashpos < xattrhashsize {
            xattr_hash_move();
            if hash >= xattrrehashpos {
                hash = hash.wrapping_sub(xattrhashsize.wrapping_div(2 as uint32_t));
            }
            (*e).next = *xattrhashtab[(hash >> HASHTAB_LOBITS) as usize]
                .offset((hash & HASHTAB_MASK as uint32_t) as isize)
                as *mut _xattrentry;
            *xattrhashtab[(hash >> HASHTAB_LOBITS) as usize]
                .offset((hash & HASHTAB_MASK as uint32_t) as isize) = e;
            xattrhashelem = xattrhashelem.wrapping_add(1);
        } else {
            (*e).next = *xattrhashtab[(hash >> HASHTAB_LOBITS) as usize]
                .offset((hash & HASHTAB_MASK as uint32_t) as isize)
                as *mut _xattrentry;
            *xattrhashtab[(hash >> HASHTAB_LOBITS) as usize]
                .offset((hash & HASHTAB_MASK as uint32_t) as isize) = e;
            xattrhashelem = xattrhashelem.wrapping_add(1);
            if xattrhashelem > xattrhashsize
                && xattrhashsize >> HASHTAB_LOBITS < HASHTAB_HISIZE as uint32_t
            {
                xattrrehashpos = xattrhashsize;
                xattrhashsize = xattrhashsize.wrapping_mul(2 as uint32_t);
            }
        };
    }
}
#[inline]
unsafe extern "C" fn xattr_cleanup_node(mut xe: *mut xattrentry) {
    unsafe {
        let mut xp: *mut xattrpair = ::core::ptr::null_mut::<xattrpair>();
        loop {
            xp = (*xe).pairhead;
            if xp.is_null() {
                break;
            }
            dict_dec_ref((*xp).dictname);
            dict_dec_ref((*xp).dictvalue);
            (*xe).pairhead = (*xp).next as *mut xattrpair;
            free(xp as *mut ::core::ffi::c_void);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xattr_namecheck(
    mut anleng: uint8_t,
    mut attrname: *const uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: uint32_t = 0;
        i = 0 as uint32_t;
        while i < anleng as uint32_t {
            if *attrname.offset(i as isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
            i = i.wrapping_add(1);
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xattr_removeinode(mut inode: uint32_t) {
    unsafe {
        let mut xe: *mut xattrentry = ::core::ptr::null_mut::<xattrentry>();
        xe = xattr_find(inode);
        if !xe.is_null() {
            xattr_cleanup_node(xe);
            xattr_delete(xe);
            free(xe as *mut ::core::ffi::c_void);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xattr_setattr(
    mut inode: uint32_t,
    mut anleng: uint8_t,
    mut attrname: *const uint8_t,
    mut avleng: uint32_t,
    mut attrvalue: *const uint8_t,
    mut mode: uint8_t,
) -> uint8_t {
    unsafe {
        let mut xe: *mut xattrentry = ::core::ptr::null_mut::<xattrentry>();
        let mut xp: *mut xattrpair = ::core::ptr::null_mut::<xattrpair>();
        let mut xpp: *mut *mut xattrpair = ::core::ptr::null_mut::<*mut xattrpair>();
        let mut dictname: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut dictvalue: *mut ::core::ffi::c_void =
            ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut inode_anleng: uint32_t = 0;
        if anleng as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || mode as ::core::ffi::c_int > MFS_XATTR_REMOVE
        {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if avleng > MFS_XATTR_SIZE_MAX as uint32_t && mode as ::core::ffi::c_int != MFS_XATTR_REMOVE
        {
            return MFS_ERROR_ERANGE as uint8_t;
        }
        if anleng as ::core::ffi::c_int + 1 as ::core::ffi::c_int > MFS_XATTR_LIST_MAX {
            return MFS_ERROR_ERANGE as uint8_t;
        }
        xe = xattr_find(inode);
        if mode as ::core::ffi::c_int == MFS_XATTR_REPLACE_ONLY
            || mode as ::core::ffi::c_int == MFS_XATTR_REMOVE
        {
            if xe.is_null() {
                return MFS_ERROR_ENOATTR as uint8_t;
            }
            dictname = dict_search(attrname, anleng as uint32_t);
            if dictname.is_null() {
                return MFS_ERROR_ENOATTR as uint8_t;
            }
        } else {
            if xe.is_null() {
                xe = malloc(::core::mem::size_of::<xattrentry>()) as *mut xattrentry;
                if xe.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/xattr.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        149 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"xe\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/xattr.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        149 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"xe\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if xe
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut xattrentry
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/xattr.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        149 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"xe\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/xattr.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        149 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"xe\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
                (*xe).inode = inode;
                (*xe).pairhead = ::core::ptr::null_mut::<xattrpair>();
                xattr_add(xe);
                fs_set_xattrflag(inode);
            }
            dictname = dict_insert(attrname, anleng as uint32_t);
        }
        inode_anleng = 0 as uint32_t;
        xpp = &raw mut (*xe).pairhead;
        loop {
            xp = *xpp;
            if xp.is_null() {
                break;
            }
            if (*xp).dictname == dictname {
                match mode as ::core::ffi::c_int {
                    MFS_XATTR_CREATE_ONLY => {
                        dict_dec_ref(dictname);
                        return MFS_ERROR_EEXIST as uint8_t;
                    }
                    MFS_XATTR_REMOVE => {
                        dict_dec_ref((*xp).dictname);
                        dict_dec_ref((*xp).dictvalue);
                        *xpp = (*xp).next as *mut xattrpair;
                        free(xp as *mut ::core::ffi::c_void);
                        if (*xe).pairhead.is_null() {
                            xattr_delete(xe);
                            free(xe as *mut ::core::ffi::c_void);
                            fs_del_xattrflag(inode);
                        }
                        return MFS_STATUS_OK as uint8_t;
                    }
                    MFS_XATTR_CREATE_OR_REPLACE => {
                        dict_dec_ref(dictname);
                    }
                    MFS_XATTR_REPLACE_ONLY => {}
                    _ => {
                        dict_dec_ref(dictname);
                        return MFS_ERROR_EINVAL as uint8_t;
                    }
                }
                if avleng > MFS_XATTR_SIZE_MAX as uint32_t {
                    return MFS_ERROR_ERANGE as uint8_t;
                }
                dictvalue = dict_insert(attrvalue, avleng);
                if (*xp).dictvalue == dictvalue {
                    dict_dec_ref(dictvalue);
                } else {
                    dict_dec_ref((*xp).dictvalue);
                    (*xp).dictvalue = dictvalue;
                }
                return MFS_STATUS_OK as uint8_t;
            }
            inode_anleng = inode_anleng
                .wrapping_add(dict_get_leng((*xp).dictname).wrapping_add(1 as uint32_t));
            xpp = &raw mut (*xp).next as *mut *mut xattrpair;
        }
        if mode as ::core::ffi::c_int == MFS_XATTR_REPLACE_ONLY
            || mode as ::core::ffi::c_int == MFS_XATTR_REMOVE
        {
            return MFS_ERROR_ENOATTR as uint8_t;
        }
        if inode_anleng
            .wrapping_add(anleng as uint32_t)
            .wrapping_add(1 as uint32_t)
            > MFS_XATTR_LIST_MAX as uint32_t
        {
            dict_dec_ref(dictname);
            return MFS_ERROR_ERANGE as uint8_t;
        }
        xp = malloc(::core::mem::size_of::<xattrpair>()) as *mut xattrpair;
        (*xp).dictname = dictname;
        (*xp).dictvalue = dict_insert(attrvalue, avleng);
        (*xp).next = (*xe).pairhead as *mut _xattrpair;
        (*xe).pairhead = xp;
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xattr_getattr(
    mut inode: uint32_t,
    mut anleng: uint8_t,
    mut attrname: *const uint8_t,
    mut avleng: *mut uint32_t,
    mut attrvalue: *mut *const uint8_t,
) -> uint8_t {
    unsafe {
        let mut xe: *mut xattrentry = ::core::ptr::null_mut::<xattrentry>();
        let mut xp: *mut xattrpair = ::core::ptr::null_mut::<xattrpair>();
        let mut dictname: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        xe = xattr_find(inode);
        if !xe.is_null() {
            dictname = dict_search(attrname, anleng as uint32_t);
            if !dictname.is_null() {
                xp = (*xe).pairhead;
                while !xp.is_null() {
                    if (*xp).dictname == dictname {
                        *avleng = dict_get_leng((*xp).dictvalue);
                        *attrvalue = dict_get_ptr((*xp).dictvalue);
                        return MFS_STATUS_OK as uint8_t;
                    }
                    xp = (*xp).next as *mut xattrpair;
                }
            }
        }
        return MFS_ERROR_ENOATTR as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xattr_listattr_leng(
    mut inode: uint32_t,
    mut xanode: *mut *mut ::core::ffi::c_void,
    mut xasize: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut xe: *mut xattrentry = ::core::ptr::null_mut::<xattrentry>();
        let mut xp: *mut xattrpair = ::core::ptr::null_mut::<xattrpair>();
        let mut inode_anleng: uint32_t = 0;
        inode_anleng = 0 as uint32_t;
        xe = xattr_find(inode);
        if !xe.is_null() {
            xp = (*xe).pairhead;
            while !xp.is_null() {
                inode_anleng = inode_anleng
                    .wrapping_add(dict_get_leng((*xp).dictname).wrapping_add(1 as uint32_t));
                xp = (*xp).next as *mut xattrpair;
            }
        }
        if inode_anleng > MFS_XATTR_LIST_MAX as uint32_t {
            return MFS_ERROR_ERANGE as uint8_t;
        }
        *xanode = xe as *mut ::core::ffi::c_void;
        *xasize = inode_anleng;
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xattr_listattr_data(
    mut xanode: *mut ::core::ffi::c_void,
    mut xabuff: *mut uint8_t,
) {
    unsafe {
        let mut xe: *mut xattrentry = xanode as *mut xattrentry;
        let mut xp: *mut xattrpair = ::core::ptr::null_mut::<xattrpair>();
        let mut i: uint32_t = 0;
        i = 0 as uint32_t;
        if !xe.is_null() {
            xp = (*xe).pairhead;
            while !xp.is_null() {
                memcpy(
                    xabuff.offset(i as isize) as *mut ::core::ffi::c_void,
                    dict_get_ptr((*xp).dictname) as *const ::core::ffi::c_void,
                    dict_get_leng((*xp).dictname) as size_t,
                );
                i = i.wrapping_add(dict_get_leng((*xp).dictname));
                let c2rust_fresh0 = i;
                i = i.wrapping_add(1);
                *xabuff.offset(c2rust_fresh0 as isize) = 0 as uint8_t;
                xp = (*xp).next as *mut xattrpair;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xattr_getall(mut inode: uint32_t, mut dbuff: *mut uint8_t) -> uint32_t {
    unsafe {
        let mut xe: *mut xattrentry = ::core::ptr::null_mut::<xattrentry>();
        let mut xp: *mut xattrpair = ::core::ptr::null_mut::<xattrpair>();
        let mut leng: uint32_t = 0;
        let mut cnt: uint16_t = 0;
        let mut anleng: uint32_t = 0;
        let mut avleng: uint32_t = 0;
        let mut cntptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        leng = 2 as uint32_t;
        cnt = 0 as uint16_t;
        cntptr = dbuff;
        if !dbuff.is_null() {
            dbuff = dbuff.offset(2 as ::core::ffi::c_int as isize);
        }
        xe = xattr_find(inode);
        if !xe.is_null() {
            xp = (*xe).pairhead;
            while !xp.is_null() {
                cnt = cnt.wrapping_add(1);
                anleng = dict_get_leng((*xp).dictname);
                avleng = dict_get_leng((*xp).dictvalue);
                leng = (leng as ::core::ffi::c_uint).wrapping_add(
                    (1 as uint32_t)
                        .wrapping_add(anleng)
                        .wrapping_add(4 as uint32_t)
                        .wrapping_add(avleng) as ::core::ffi::c_uint,
                ) as uint32_t;
                if !dbuff.is_null() {
                    put8bit(&raw mut dbuff, anleng as uint8_t);
                    memcpy(
                        dbuff as *mut ::core::ffi::c_void,
                        dict_get_ptr((*xp).dictname) as *const ::core::ffi::c_void,
                        anleng as size_t,
                    );
                    dbuff = dbuff.offset(anleng as isize);
                    put32bit(&raw mut dbuff, avleng);
                    memcpy(
                        dbuff as *mut ::core::ffi::c_void,
                        dict_get_ptr((*xp).dictvalue) as *const ::core::ffi::c_void,
                        avleng as size_t,
                    );
                    dbuff = dbuff.offset(avleng as isize);
                }
                xp = (*xp).next as *mut xattrpair;
            }
        }
        if !cntptr.is_null() {
            put16bit(&raw mut cntptr, cnt);
        }
        return leng;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xattr_check(
    mut inode: uint32_t,
    mut dbuff: *const uint8_t,
    mut leng: uint32_t,
    mut pleng: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut xe: *mut xattrentry = ::core::ptr::null_mut::<xattrentry>();
        let mut xp: *mut xattrpair = ::core::ptr::null_mut::<xattrpair>();
        let mut i: uint16_t = 0;
        let mut cnt: uint16_t = 0;
        let mut xattrcnt: uint16_t = 0;
        let mut checkcnt: uint16_t = 0;
        let mut anleng: uint8_t = 0;
        let mut xattranleng: uint8_t = 0;
        let mut attrname: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut avleng: uint32_t = 0;
        let mut xattravleng: uint32_t = 0;
        let mut attrvalue: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut ileng: uint32_t = 0;
        ileng = leng;
        *pleng = 0 as uint32_t;
        if leng < 2 as uint32_t {
            return 2 as uint8_t;
        }
        cnt = get16bit(&raw mut dbuff);
        leng = leng.wrapping_sub(2 as uint32_t);
        xe = xattr_find(inode);
        xattrcnt = 0 as uint16_t;
        if !xe.is_null() {
            xp = (*xe).pairhead;
            while !xp.is_null() {
                xattrcnt = xattrcnt.wrapping_add(1);
                xp = (*xp).next as *mut xattrpair;
            }
        }
        checkcnt = 0 as uint16_t;
        i = 0 as uint16_t;
        while (i as ::core::ffi::c_int) < cnt as ::core::ffi::c_int {
            if leng < 1 as uint32_t {
                return 2 as uint8_t;
            }
            anleng = get8bit(&raw mut dbuff);
            leng = leng.wrapping_sub(1);
            if leng < anleng as uint32_t {
                return 2 as uint8_t;
            }
            attrname = dbuff;
            dbuff = dbuff.offset(anleng as ::core::ffi::c_int as isize);
            leng = leng.wrapping_sub(anleng as uint32_t);
            if leng < 4 as uint32_t {
                return 2 as uint8_t;
            }
            avleng = get32bit(&raw mut dbuff);
            leng = leng.wrapping_sub(4 as uint32_t);
            if leng < avleng {
                return 2 as uint8_t;
            }
            attrvalue = dbuff;
            dbuff = dbuff.offset(avleng as isize);
            leng = leng.wrapping_sub(avleng);
            if !xe.is_null()
                && xattrcnt as ::core::ffi::c_int == cnt as ::core::ffi::c_int
                && i as ::core::ffi::c_int == checkcnt as ::core::ffi::c_int
            {
                xp = (*xe).pairhead;
                while !xp.is_null() {
                    xattranleng = dict_get_leng((*xp).dictname) as uint8_t;
                    xattravleng = dict_get_leng((*xp).dictvalue);
                    if xattranleng as ::core::ffi::c_int == anleng as ::core::ffi::c_int
                        && xattravleng == avleng
                    {
                        if memcmp(
                            attrname as *const ::core::ffi::c_void,
                            dict_get_ptr((*xp).dictname) as *const ::core::ffi::c_void,
                            anleng as size_t,
                        ) == 0 as ::core::ffi::c_int
                            && memcmp(
                                attrvalue as *const ::core::ffi::c_void,
                                dict_get_ptr((*xp).dictvalue) as *const ::core::ffi::c_void,
                                avleng as size_t,
                            ) == 0 as ::core::ffi::c_int
                        {
                            checkcnt = checkcnt.wrapping_add(1);
                        }
                    }
                    xp = (*xp).next as *mut xattrpair;
                }
            }
            i = i.wrapping_add(1);
        }
        *pleng = ileng.wrapping_sub(leng);
        return (if checkcnt as ::core::ffi::c_int == cnt as ::core::ffi::c_int
            && xattrcnt as ::core::ffi::c_int == cnt as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xattr_setall(mut inode: uint32_t, mut dbuff: *const uint8_t) -> uint8_t {
    unsafe {
        let mut xe: *mut xattrentry = ::core::ptr::null_mut::<xattrentry>();
        let mut xp: *mut xattrpair = ::core::ptr::null_mut::<xattrpair>();
        let mut xpnew: *mut xattrpair = ::core::ptr::null_mut::<xattrpair>();
        let mut xptail: *mut *mut xattrpair = ::core::ptr::null_mut::<*mut xattrpair>();
        let mut xpprev: *mut *mut xattrpair = ::core::ptr::null_mut::<*mut xattrpair>();
        let mut i: uint16_t = 0;
        let mut cnt: uint16_t = 0;
        let mut moved: uint16_t = 0;
        let mut anleng: uint8_t = 0;
        let mut attrname: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut avleng: uint32_t = 0;
        let mut attrvalue: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut dictname: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut dictvalue: *mut ::core::ffi::c_void =
            ::core::ptr::null_mut::<::core::ffi::c_void>();
        cnt = get16bit(&raw mut dbuff);
        xe = xattr_find(inode);
        if xe.is_null() {
            if cnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return 0 as uint8_t;
            } else {
                xe = malloc(::core::mem::size_of::<xattrentry>()) as *mut xattrentry;
                if xe.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/xattr.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        410 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"xe\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/xattr.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        410 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"xe\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if xe
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut xattrentry
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/xattr.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        410 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"xe\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/xattr.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        410 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"xe\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
                (*xe).inode = inode;
                (*xe).pairhead = ::core::ptr::null_mut::<xattrpair>();
                xattr_add(xe);
            }
        }
        xpnew = ::core::ptr::null_mut::<xattrpair>();
        xptail = &raw mut xpnew;
        i = 0 as uint16_t;
        while (i as ::core::ffi::c_int) < cnt as ::core::ffi::c_int {
            anleng = get8bit(&raw mut dbuff);
            attrname = dbuff;
            dbuff = dbuff.offset(anleng as ::core::ffi::c_int as isize);
            avleng = get32bit(&raw mut dbuff);
            attrvalue = dbuff;
            dbuff = dbuff.offset(avleng as isize);
            dictname = dict_insert(attrname, anleng as uint32_t);
            moved = 0 as uint16_t;
            xpprev = &raw mut (*xe).pairhead;
            while moved as ::core::ffi::c_int == 0 as ::core::ffi::c_int && {
                xp = *xpprev;
                !xp.is_null()
            } {
                if (*xp).dictname == dictname {
                    dict_dec_ref((*xp).dictname);
                    dictvalue = dict_insert(attrvalue, avleng);
                    if (*xp).dictvalue == dictvalue {
                        dict_dec_ref(dictvalue);
                    } else {
                        dict_dec_ref((*xp).dictvalue);
                        (*xp).dictvalue = dictvalue;
                    }
                    *xpprev = (*xp).next as *mut xattrpair;
                    *xptail = xp;
                    (*xp).next = ::core::ptr::null_mut::<_xattrpair>();
                    xptail = &raw mut (*xp).next as *mut *mut xattrpair;
                    moved = 1 as uint16_t;
                } else {
                    xpprev = &raw mut (*xp).next as *mut *mut xattrpair;
                }
            }
            if moved as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                xp = malloc(::core::mem::size_of::<xattrpair>()) as *mut xattrpair;
                (*xp).dictname = dictname;
                (*xp).dictvalue = dict_insert(attrvalue, avleng);
                *xptail = xp;
                (*xp).next = ::core::ptr::null_mut::<_xattrpair>();
                xptail = &raw mut (*xp).next as *mut *mut xattrpair;
            }
            i = i.wrapping_add(1);
        }
        loop {
            xp = (*xe).pairhead;
            if xp.is_null() {
                break;
            }
            dict_dec_ref((*xp).dictname);
            dict_dec_ref((*xp).dictvalue);
            (*xe).pairhead = (*xp).next as *mut xattrpair;
            free(xp as *mut ::core::ffi::c_void);
        }
        if xpnew.is_null() {
            xattr_delete(xe);
            free(xe as *mut ::core::ffi::c_void);
            return 0 as uint8_t;
        }
        (*xe).pairhead = xpnew;
        return 1 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xattr_copy(mut srcinode: uint32_t, mut dstinode: uint32_t) -> uint8_t {
    unsafe {
        let mut sxe: *mut xattrentry = ::core::ptr::null_mut::<xattrentry>();
        let mut dxe: *mut xattrentry = ::core::ptr::null_mut::<xattrentry>();
        let mut sxp: *mut xattrpair = ::core::ptr::null_mut::<xattrpair>();
        let mut dxp: *mut xattrpair = ::core::ptr::null_mut::<xattrpair>();
        let mut xpt: *mut *mut xattrpair = ::core::ptr::null_mut::<*mut xattrpair>();
        sxe = xattr_find(srcinode);
        if sxe.is_null() {
            return 0 as uint8_t;
        }
        dxe = malloc(::core::mem::size_of::<xattrentry>()) as *mut xattrentry;
        if dxe.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/xattr.c\0".as_ptr() as *const ::core::ffi::c_char,
                485 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dxe\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/xattr.c\0".as_ptr() as *const ::core::ffi::c_char,
                485 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dxe\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if dxe
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut xattrentry
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/xattr.c\0".as_ptr() as *const ::core::ffi::c_char,
                485 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dxe\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/xattr.c\0".as_ptr() as *const ::core::ffi::c_char,
                485 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dxe\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*dxe).inode = dstinode;
        (*dxe).pairhead = ::core::ptr::null_mut::<xattrpair>();
        xpt = &raw mut (*dxe).pairhead;
        xattr_add(dxe);
        sxp = (*sxe).pairhead;
        while !sxp.is_null() {
            dxp = malloc(::core::mem::size_of::<xattrpair>()) as *mut xattrpair;
            if dxp.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/xattr.c\0".as_ptr() as *const ::core::ffi::c_char,
                    492 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"dxp\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/xattr.c\0".as_ptr() as *const ::core::ffi::c_char,
                    492 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"dxp\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if dxp
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut xattrpair
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/xattr.c\0".as_ptr() as *const ::core::ffi::c_char,
                    492 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"dxp\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/xattr.c\0".as_ptr() as *const ::core::ffi::c_char,
                    492 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"dxp\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
            (*dxp).dictname = (*sxp).dictname;
            (*dxp).dictvalue = (*sxp).dictvalue;
            dict_inc_ref((*dxp).dictname);
            dict_inc_ref((*dxp).dictvalue);
            (*dxp).next = ::core::ptr::null_mut::<_xattrpair>();
            *xpt = dxp;
            xpt = &raw mut (*dxp).next as *mut *mut xattrpair;
            sxp = (*sxp).next as *mut xattrpair;
        }
        return 1 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xattr_cleanup() {
    unsafe {
        let mut i: uint16_t = 0;
        let mut j: uint32_t = 0;
        let mut xe: *mut xattrentry = ::core::ptr::null_mut::<xattrentry>();
        let mut xen: *mut xattrentry = ::core::ptr::null_mut::<xattrentry>();
        i = 0 as uint16_t;
        while (i as ::core::ffi::c_uint) < HASHTAB_HISIZE {
            if !xattrhashtab[i as usize].is_null() {
                j = 0 as uint32_t;
                while j < HASHTAB_LOSIZE as uint32_t {
                    xe = *xattrhashtab[i as usize].offset(j as isize);
                    while !xe.is_null() {
                        xattr_cleanup_node(xe);
                        xen = (*xe).next as *mut xattrentry;
                        free(xe as *mut ::core::ffi::c_void);
                        xe = xen;
                    }
                    *xattrhashtab[i as usize].offset(j as isize) =
                        ::core::ptr::null_mut::<xattrentry>();
                    j = j.wrapping_add(1);
                }
            }
            i = i.wrapping_add(1);
        }
        xattr_hash_cleanup();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xattr_store(mut fd: *mut bio) -> uint8_t {
    unsafe {
        let mut hdrbuff: [uint8_t; 9] = [0; 9];
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut i: uint16_t = 0;
        let mut j: uint32_t = 0;
        let mut anleng: uint32_t = 0;
        let mut avleng: uint32_t = 0;
        let mut xe: *mut xattrentry = ::core::ptr::null_mut::<xattrentry>();
        let mut xp: *mut xattrpair = ::core::ptr::null_mut::<xattrpair>();
        if fd.is_null() {
            return 0x10 as uint8_t;
        }
        i = 0 as uint16_t;
        while (i as ::core::ffi::c_uint) < HASHTAB_HISIZE {
            if !xattrhashtab[i as usize].is_null() {
                j = 0 as uint32_t;
                while j < HASHTAB_LOSIZE as uint32_t {
                    xe = *xattrhashtab[i as usize].offset(j as isize);
                    while !xe.is_null() {
                        xp = (*xe).pairhead;
                        while !xp.is_null() {
                            anleng = dict_get_leng((*xp).dictname);
                            avleng = dict_get_leng((*xp).dictvalue);
                            if anleng > 255 as uint32_t {
                                anleng = 255 as uint32_t;
                            }
                            ptr = &raw mut hdrbuff as *mut uint8_t;
                            put32bit(&raw mut ptr, (*xe).inode);
                            put8bit(&raw mut ptr, anleng as uint8_t);
                            put32bit(&raw mut ptr, avleng);
                            if bio_write(
                                fd,
                                &raw mut hdrbuff as *mut uint8_t as *const ::core::ffi::c_void,
                                (4 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int
                                    + 4 as ::core::ffi::c_int)
                                    as uint64_t,
                            ) != (4 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int)
                                as int64_t
                            {
                                return 0xff as uint8_t;
                            }
                            if bio_write(
                                fd,
                                dict_get_ptr((*xp).dictname) as *const ::core::ffi::c_void,
                                anleng as uint64_t,
                            ) != anleng as int64_t
                            {
                                return 0xff as uint8_t;
                            }
                            if avleng > 0 as uint32_t {
                                if bio_write(
                                    fd,
                                    dict_get_ptr((*xp).dictvalue) as *const ::core::ffi::c_void,
                                    avleng as uint64_t,
                                ) != avleng as int64_t
                                {
                                    return 0xff as uint8_t;
                                }
                            }
                            xp = (*xp).next as *mut xattrpair;
                        }
                        xe = (*xe).next as *mut xattrentry;
                    }
                    j = j.wrapping_add(1);
                }
            }
            i = i.wrapping_add(1);
        }
        memset(
            &raw mut hdrbuff as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as size_t,
        );
        if bio_write(
            fd,
            &raw mut hdrbuff as *mut uint8_t as *const ::core::ffi::c_void,
            (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                as uint64_t,
        ) != (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            as int64_t
        {
            return 0xff as uint8_t;
        }
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xattr_load(
    mut fd: *mut bio,
    mut mver: uint8_t,
    mut ignoreflag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut hdrbuff: [uint8_t; 9] = [0; 9];
        let mut databuff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut inode: uint32_t = 0;
        let mut lastinode: uint32_t = 0;
        let mut anleng: uint8_t = 0;
        let mut avleng: uint32_t = 0;
        let mut nl: uint8_t = 1 as uint8_t;
        let mut xe: *mut xattrentry = ::core::ptr::null_mut::<xattrentry>();
        let mut xp: *mut xattrpair = ::core::ptr::null_mut::<xattrpair>();
        let mut xpt: *mut *mut xattrpair = ::core::ptr::null_mut::<*mut xattrpair>();
        xpt = ::core::ptr::null_mut::<*mut xattrpair>();
        databuff = malloc((MFS_XATTR_NAME_MAX + MFS_XATTR_SIZE_MAX) as size_t) as *mut uint8_t;
        if databuff.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/xattr.c\0".as_ptr() as *const ::core::ffi::c_char,
                588 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"databuff\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/xattr.c\0".as_ptr() as *const ::core::ffi::c_char,
                588 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"databuff\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if databuff
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut uint8_t
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/xattr.c\0".as_ptr() as *const ::core::ffi::c_char,
                588 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"databuff\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/xattr.c\0".as_ptr() as *const ::core::ffi::c_char,
                588 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"databuff\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        lastinode = 0 as uint32_t;
        loop {
            if bio_read(
                fd,
                &raw mut hdrbuff as *mut uint8_t as *mut ::core::ffi::c_void,
                (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    as uint64_t,
            ) != (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                as int64_t
            {
                let mut err: ::core::ffi::c_int = *__errno_location();
                if nl != 0 {
                    fputc('\n' as ::core::ffi::c_int, stderr);
                }
                *__errno_location() = err;
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"loading xattr: read error\0".as_ptr() as *const ::core::ffi::c_char,
                );
                free(databuff as *mut ::core::ffi::c_void);
                return -1 as ::core::ffi::c_int;
            }
            ptr = &raw mut hdrbuff as *mut uint8_t;
            inode = get32bit(&raw mut ptr);
            anleng = get8bit(&raw mut ptr);
            avleng = get32bit(&raw mut ptr);
            if inode == 0 as uint32_t {
                free(databuff as *mut ::core::ffi::c_void);
                return 1 as ::core::ffi::c_int;
            }
            if fs_check_inode(inode) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                bio_skip(fd, (anleng as uint32_t).wrapping_add(avleng) as uint64_t);
            } else if anleng as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if nl != 0 {
                    fputc('\n' as ::core::ffi::c_int, stderr);
                    nl = 0 as uint8_t;
                }
                if ignoreflag != 0 {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_NOTICE,
                        b"loading xattr: empty name - ignoring\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    bio_skip(fd, (anleng as uint32_t).wrapping_add(avleng) as uint64_t);
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"loading xattr: empty name\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    free(databuff as *mut ::core::ffi::c_void);
                    return -1 as ::core::ffi::c_int;
                }
            } else if avleng > MFS_XATTR_SIZE_MAX as uint32_t {
                if nl != 0 {
                    fputc('\n' as ::core::ffi::c_int, stderr);
                    nl = 0 as uint8_t;
                }
                if ignoreflag != 0 {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_NOTICE,
                        b"loading xattr: value oversized - ignoring\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    bio_skip(fd, (anleng as uint32_t).wrapping_add(avleng) as uint64_t);
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"loading xattr: value oversized\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    free(databuff as *mut ::core::ffi::c_void);
                    return -1 as ::core::ffi::c_int;
                }
            } else {
                if inode != lastinode {
                    xe = xattr_find(inode);
                    if xe.is_null() {
                        xe = malloc(::core::mem::size_of::<xattrentry>()) as *mut xattrentry;
                        if xe.is_null() {
                            fprintf(
                                stderr,
                                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/xattr.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                652 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"xe\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/xattr.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                652 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"xe\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            abort();
                        } else if xe
                            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                                -1 as ::core::ffi::c_int as usize,
                            ) as *mut xattrentry
                        {
                            let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/xattr.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                652 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"xe\0".as_ptr() as *const ::core::ffi::c_char,
                                _mfs_errorstring_0,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/xattr.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                652 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"xe\0".as_ptr() as *const ::core::ffi::c_char,
                                _mfs_errorstring_0,
                            );
                            abort();
                        }
                        (*xe).inode = inode;
                        (*xe).pairhead = ::core::ptr::null_mut::<xattrpair>();
                        xattr_add(xe);
                        fs_set_xattrflag(inode);
                        xpt = &raw mut (*xe).pairhead;
                    } else {
                        xpt = &raw mut (*xe).pairhead;
                        while !(*xpt).is_null() {
                            xpt = &raw mut (**xpt).next as *mut *mut xattrpair;
                        }
                    }
                    lastinode = inode;
                }
                if bio_read(
                    fd,
                    databuff as *mut ::core::ffi::c_void,
                    (anleng as uint32_t).wrapping_add(avleng) as uint64_t,
                ) != (anleng as uint32_t).wrapping_add(avleng) as int64_t
                {
                    let mut err_0: ::core::ffi::c_int = *__errno_location();
                    if nl != 0 {
                        fputc('\n' as ::core::ffi::c_int, stderr);
                    }
                    *__errno_location() = err_0;
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"loading xattr: read error\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    free(databuff as *mut ::core::ffi::c_void);
                    return -1 as ::core::ffi::c_int;
                }
                xp = malloc(::core::mem::size_of::<xattrpair>()) as *mut xattrpair;
                (*xp).dictname = dict_insert(databuff, anleng as uint32_t);
                (*xp).dictvalue = dict_insert(
                    databuff.offset(anleng as ::core::ffi::c_int as isize),
                    avleng,
                );
                (*xp).next = ::core::ptr::null_mut::<_xattrpair>();
                *xpt = xp;
                xpt = &raw mut (*xp).next as *mut *mut xattrpair;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xattr_init() -> ::core::ffi::c_int {
    unsafe {
        xattr_hash_init();
        return 0 as ::core::ffi::c_int;
    }
}
