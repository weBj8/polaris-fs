pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
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
    unsafe fn mmap(
        __addr: *mut ::core::ffi::c_void,
        __len: size_t,
        __prot: ::core::ffi::c_int,
        __flags: ::core::ffi::c_int,
        __fd: ::core::ffi::c_int,
        __offset: __off64_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn munmap(__addr: *mut ::core::ffi::c_void, __len: size_t) -> ::core::ffi::c_int;
    static mut stderr: *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
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
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
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
pub struct _dictentry {
    pub next: *mut _dictentry,
    pub hashval: uint32_t,
    pub refcnt: uint32_t,
    pub leng: uint32_t,
    pub data: [uint8_t; 1],
}
pub type dictentry = _dictentry;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const PROT_READ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PROT_WRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MAP_PRIVATE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MAP_ANONYMOUS: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const MAP_ANON: ::core::ffi::c_int = MAP_ANONYMOUS;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LOHASH_BITS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn dict_cmp(
    mut e: *mut dictentry,
    mut data: *const uint8_t,
    mut leng: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        return ((*e).leng == leng
            && memcmp(
                &raw const (*e).data as *const uint8_t as *mut ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                data as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                leng as size_t,
            ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn dict_hash(mut data: *const uint8_t, mut leng: uint32_t) -> uint32_t {
    unsafe {
        let mut hash: uint32_t = 0;
        let mut i: uint32_t = 0;
        hash = leng;
        i = 0 as uint32_t;
        while i < leng {
            hash = hash
                .wrapping_mul(33 as uint32_t)
                .wrapping_add(*data.offset(i as isize) as uint32_t);
            i = i.wrapping_add(1);
        }
        return hash;
    }
}
#[inline]
unsafe extern "C" fn dict_print(mut e: *mut dictentry) {
    unsafe {
        let mut i: uint32_t = 0;
        let mut c: uint8_t = 0;
        printf(
            b"(refcnt:%u,leng:%u,data:\0".as_ptr() as *const ::core::ffi::c_char,
            (*e).refcnt,
            (*e).leng,
        );
        i = 0 as uint32_t;
        while i < (*e).leng {
            c = *(&raw const (*e).data as *const uint8_t).offset(i as isize);
            if c as ::core::ffi::c_int >= 32 as ::core::ffi::c_int
                && (c as ::core::ffi::c_int) < 127 as ::core::ffi::c_int
            {
                printf(
                    b"%c\0".as_ptr() as *const ::core::ffi::c_char,
                    c as ::core::ffi::c_int,
                );
            } else {
                printf(b".\0".as_ptr() as *const ::core::ffi::c_char);
            }
            i = i.wrapping_add(1);
        }
        printf(b")\0".as_ptr() as *const ::core::ffi::c_char);
    }
}
pub const HASHTAB_LOBITS: ::core::ffi::c_int = LOHASH_BITS;
pub const HASHTAB_HISIZE: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint >> HASHTAB_LOBITS;
pub const HASHTAB_LOSIZE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << HASHTAB_LOBITS;
pub const HASHTAB_MASK: ::core::ffi::c_int = HASHTAB_LOSIZE - 1 as ::core::ffi::c_int;
pub const HASHTAB_MOVEFACTOR: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const HASHTAB_SIZEHINT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut dicthashtab: [*mut *mut dictentry; 2048] =
    [::core::ptr::null_mut::<*mut dictentry>(); 2048];
static mut dictrehashpos: uint32_t = 0;
static mut dicthashsize: uint32_t = 0;
static mut dicthashelem: uint32_t = 0;
#[inline]
unsafe extern "C" fn dict_calc_hash_size(mut elements: uint32_t) -> uint32_t {
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
unsafe extern "C" fn dict_hash_init() {
    unsafe {
        let mut i: uint16_t = 0;
        dicthashsize = 0 as uint32_t;
        dicthashelem = 0 as uint32_t;
        dictrehashpos = 0 as uint32_t;
        i = 0 as uint16_t;
        while (i as ::core::ffi::c_uint) < HASHTAB_HISIZE {
            dicthashtab[i as usize] = ::core::ptr::null_mut::<*mut dictentry>();
            i = i.wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C" fn dict_hash_cleanup() {
    unsafe {
        let mut i: uint16_t = 0;
        let mut j: uint32_t = 0;
        dicthashelem = 0 as uint32_t;
        dicthashsize = 0 as uint32_t;
        dictrehashpos = 0 as uint32_t;
        i = 0 as uint16_t;
        while (i as ::core::ffi::c_uint) < HASHTAB_HISIZE {
            if !dicthashtab[i as usize].is_null() {
                j = 0 as uint32_t;
                while j < HASHTAB_LOSIZE as uint32_t {
                    if (*dicthashtab[i as usize].offset(j as isize)).is_null() {
                    } else {
                        fprintf(
                            stderr,
                            b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/hash_begin.h\0".as_ptr()
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
                            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/hash_begin.h\0".as_ptr()
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
                    dicthashtab[i as usize] as *mut ::core::ffi::c_void,
                    ::core::mem::size_of::<*mut dictentry>().wrapping_mul(HASHTAB_LOSIZE as size_t),
                );
            }
            dicthashtab[i as usize] = ::core::ptr::null_mut::<*mut dictentry>();
            i = i.wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C" fn dict_hash_print() {
    unsafe {
        let mut i: uint16_t = 0;
        let mut j: uint32_t = 0;
        let mut e: *mut dictentry = ::core::ptr::null_mut::<dictentry>();
        printf(
            b"hash elem: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
            dicthashelem,
        );
        printf(
            b"hash size: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
            dicthashsize,
        );
        i = 0 as uint16_t;
        while (i as ::core::ffi::c_uint) < HASHTAB_HISIZE {
            if !dicthashtab[i as usize].is_null() {
                j = 0 as uint32_t;
                while j < HASHTAB_LOSIZE as uint32_t {
                    if !(*dicthashtab[i as usize].offset(j as isize)).is_null() {
                        printf(
                            b"hash pos: %hu,%u:\0".as_ptr() as *const ::core::ffi::c_char,
                            i as ::core::ffi::c_int,
                            j,
                        );
                        e = *dicthashtab[i as usize].offset(j as isize);
                        while !e.is_null() {
                            dict_print(e);
                            if !(*e).next.is_null() {
                                printf(b" , \0".as_ptr() as *const ::core::ffi::c_char);
                            }
                            e = (*e).next as *mut dictentry;
                        }
                        printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
                    }
                    j = j.wrapping_add(1);
                }
            }
            i = i.wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C" fn dict_hash_move() {
    unsafe {
        let mut hash: uint32_t = 0;
        let mut mask: uint32_t = 0;
        let mut moved: uint32_t = 0 as uint32_t;
        let mut ehptr: *mut *mut dictentry = ::core::ptr::null_mut::<*mut dictentry>();
        let mut ehptralt: *mut *mut dictentry = ::core::ptr::null_mut::<*mut dictentry>();
        let mut e: *mut dictentry = ::core::ptr::null_mut::<dictentry>();
        mask = dicthashsize.wrapping_sub(1 as uint32_t);
        loop {
            if dictrehashpos >= dicthashsize {
                dictrehashpos = dicthashsize;
                return;
            }
            if dicthashtab[(dictrehashpos >> HASHTAB_LOBITS) as usize].is_null() {
                dicthashtab[(dictrehashpos >> HASHTAB_LOBITS) as usize] = mmap(
                    NULL,
                    ::core::mem::size_of::<*mut dictentry>().wrapping_mul(HASHTAB_LOSIZE as size_t),
                    PROT_READ | PROT_WRITE,
                    MAP_ANON | MAP_PRIVATE,
                    -1 as ::core::ffi::c_int,
                    0 as __off64_t,
                )
                    as *mut *mut dictentry;
                if dicthashtab[(dictrehashpos >> 20 as ::core::ffi::c_int) as usize].is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/../mfscommon/hash_begin.h\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        187 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"GLUE_HASH_TAB_PREFIX(hashtab)[GLUE_HASH_TAB_PREFIX(rehashpos) >> HASHTAB_LOBITS]\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/../mfscommon/hash_begin.h\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        187 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"GLUE_HASH_TAB_PREFIX(hashtab)[GLUE_HASH_TAB_PREFIX(rehashpos) >> HASHTAB_LOBITS]\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if dicthashtab[(dictrehashpos >> 20 as ::core::ffi::c_int) as usize]
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut *mut dictentry
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/../mfscommon/hash_begin.h\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        187 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"GLUE_HASH_TAB_PREFIX(hashtab)[GLUE_HASH_TAB_PREFIX(rehashpos) >> HASHTAB_LOBITS]\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/../mfscommon/hash_begin.h\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        187 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"GLUE_HASH_TAB_PREFIX(hashtab)[GLUE_HASH_TAB_PREFIX(rehashpos) >> HASHTAB_LOBITS]\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
            }
            ehptr = dicthashtab[(dictrehashpos
                .wrapping_sub(dicthashsize.wrapping_div(2 as uint32_t))
                >> HASHTAB_LOBITS) as usize]
                .offset((dictrehashpos & HASHTAB_MASK as uint32_t) as isize);
            ehptralt = dicthashtab[(dictrehashpos >> HASHTAB_LOBITS) as usize]
                .offset((dictrehashpos & HASHTAB_MASK as uint32_t) as isize);
            *ehptralt = ::core::ptr::null_mut::<dictentry>();
            loop {
                e = *ehptr;
                if e.is_null() {
                    break;
                }
                hash = (*e).hashval & mask;
                if hash == dictrehashpos {
                    *ehptralt = e;
                    *ehptr = (*e).next as *mut dictentry;
                    ehptralt = &raw mut (*e).next as *mut *mut dictentry;
                    (*e).next = ::core::ptr::null_mut::<_dictentry>();
                } else {
                    ehptr = &raw mut (*e).next as *mut *mut dictentry;
                }
                moved = moved.wrapping_add(1);
            }
            dictrehashpos = dictrehashpos.wrapping_add(1);
            if moved >= HASHTAB_MOVEFACTOR as uint32_t {
                break;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn dict_find(mut data: *const uint8_t, mut leng: uint32_t) -> *mut dictentry {
    unsafe {
        let mut e: *mut dictentry = ::core::ptr::null_mut::<dictentry>();
        let mut hash: uint32_t = 0;
        let mut hashval: uint32_t = 0;
        if dicthashsize == 0 as uint32_t {
            return ::core::ptr::null_mut::<dictentry>();
        }
        hashval = dict_hash(data, leng);
        hash = hashval & dicthashsize.wrapping_sub(1 as uint32_t);
        if dictrehashpos < dicthashsize {
            dict_hash_move();
            if hash >= dictrehashpos {
                hash = hash.wrapping_sub(dicthashsize.wrapping_div(2 as uint32_t));
            }
        }
        e = *dicthashtab[(hash >> HASHTAB_LOBITS) as usize]
            .offset((hash & HASHTAB_MASK as uint32_t) as isize);
        while !e.is_null() {
            if (*e).hashval == hashval && dict_cmp(e, data, leng) != 0 {
                return e;
            }
            e = (*e).next as *mut dictentry;
        }
        return ::core::ptr::null_mut::<dictentry>();
    }
}
#[inline]
unsafe extern "C" fn dict_delete(mut e: *mut dictentry) -> uint8_t {
    unsafe {
        let mut ehptr: *mut *mut dictentry = ::core::ptr::null_mut::<*mut dictentry>();
        let mut eit: *mut dictentry = ::core::ptr::null_mut::<dictentry>();
        let mut hash: uint32_t = 0;
        if dicthashsize == 0 as uint32_t {
            return 0 as uint8_t;
        }
        hash = (*e).hashval & dicthashsize.wrapping_sub(1 as uint32_t);
        if dictrehashpos < dicthashsize {
            dict_hash_move();
            if hash >= dictrehashpos {
                hash = hash.wrapping_sub(dicthashsize.wrapping_div(2 as uint32_t));
            }
        }
        ehptr = dicthashtab[(hash >> HASHTAB_LOBITS) as usize]
            .offset((hash & HASHTAB_MASK as uint32_t) as isize);
        loop {
            eit = *ehptr;
            if eit.is_null() {
                break;
            }
            if eit == e {
                *ehptr = (*e).next as *mut dictentry;
                dicthashelem = dicthashelem.wrapping_sub(1);
                return 1 as uint8_t;
            }
            ehptr = &raw mut (*eit).next as *mut *mut dictentry;
        }
        return 0 as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn dict_add(mut e: *mut dictentry) {
    unsafe {
        let mut i: uint16_t = 0;
        let mut hash: uint32_t = 0;
        if dicthashsize == 0 as uint32_t {
            dicthashsize = dict_calc_hash_size(HASHTAB_SIZEHINT as uint32_t);
            dictrehashpos = dicthashsize;
            dicthashelem = 0 as uint32_t;
            i = 0 as uint16_t;
            while (i as uint32_t) < dicthashsize >> HASHTAB_LOBITS {
                dicthashtab[i as usize] = mmap(
                    NULL,
                    ::core::mem::size_of::<*mut dictentry>().wrapping_mul(HASHTAB_LOSIZE as size_t),
                    PROT_READ | PROT_WRITE,
                    MAP_ANON | MAP_PRIVATE,
                    -1 as ::core::ffi::c_int,
                    0 as __off64_t,
                ) as *mut *mut dictentry;
                if dicthashtab[i as usize].is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/../mfscommon/hash_begin.h\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsmaster/../mfscommon/hash_begin.h\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        284 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"GLUE_HASH_TAB_PREFIX(hashtab)[i]\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if dicthashtab[i as usize]
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut *mut dictentry
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/../mfscommon/hash_begin.h\0".as_ptr()
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
                        b"/tmp/moosefs-ref/mfsmaster/../mfscommon/hash_begin.h\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        284 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"GLUE_HASH_TAB_PREFIX(hashtab)[i]\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
                memset(
                    dicthashtab[i as usize] as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<*mut dictentry>(),
                );
                if (*dicthashtab[i as usize].offset(0 as isize)).is_null() {
                    memset(
                        dicthashtab[i as usize] as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<*mut dictentry>()
                            .wrapping_mul(HASHTAB_LOSIZE as size_t),
                    );
                } else {
                    hash = 0 as uint32_t;
                    while hash < HASHTAB_LOSIZE as uint32_t {
                        *dicthashtab[i as usize].offset(hash as isize) =
                            ::core::ptr::null_mut::<dictentry>();
                        hash = hash.wrapping_add(1);
                    }
                }
                i = i.wrapping_add(1);
            }
        }
        (*e).hashval = dict_hash(&raw const (*e).data as *const uint8_t, (*e).leng);
        hash = (*e).hashval & dicthashsize.wrapping_sub(1 as uint32_t);
        if dictrehashpos < dicthashsize {
            dict_hash_move();
            if hash >= dictrehashpos {
                hash = hash.wrapping_sub(dicthashsize.wrapping_div(2 as uint32_t));
            }
            (*e).next = *dicthashtab[(hash >> HASHTAB_LOBITS) as usize]
                .offset((hash & HASHTAB_MASK as uint32_t) as isize)
                as *mut _dictentry;
            *dicthashtab[(hash >> HASHTAB_LOBITS) as usize]
                .offset((hash & HASHTAB_MASK as uint32_t) as isize) = e;
            dicthashelem = dicthashelem.wrapping_add(1);
        } else {
            (*e).next = *dicthashtab[(hash >> HASHTAB_LOBITS) as usize]
                .offset((hash & HASHTAB_MASK as uint32_t) as isize)
                as *mut _dictentry;
            *dicthashtab[(hash >> HASHTAB_LOBITS) as usize]
                .offset((hash & HASHTAB_MASK as uint32_t) as isize) = e;
            dicthashelem = dicthashelem.wrapping_add(1);
            if dicthashelem > dicthashsize
                && dicthashsize >> HASHTAB_LOBITS < HASHTAB_HISIZE as uint32_t
            {
                dictrehashpos = dicthashsize;
                dicthashsize = dicthashsize.wrapping_mul(2 as uint32_t);
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_init() -> ::core::ffi::c_int {
    unsafe {
        dict_hash_init();
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_cleanup() {
    unsafe {
        dict_hash_cleanup();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_printall() {
    unsafe {
        dict_hash_print();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_search(
    mut data: *const uint8_t,
    mut leng: uint32_t,
) -> *mut ::core::ffi::c_void {
    unsafe {
        return dict_find(data, leng) as *mut ::core::ffi::c_void;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_insert(
    mut data: *const uint8_t,
    mut leng: uint32_t,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut de: *mut dictentry = ::core::ptr::null_mut::<dictentry>();
        de = dict_find(data, leng);
        if !de.is_null() {
            (*de).refcnt = (*de).refcnt.wrapping_add(1);
            return de as *mut ::core::ffi::c_void;
        }
        de = malloc((20 as size_t).wrapping_add(leng as size_t)) as *mut dictentry;
        if de.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/dictionary.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                111 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"de\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/dictionary.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                111 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"de\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if de
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut dictentry
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/dictionary.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                111 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"de\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/dictionary.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                111 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"de\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*de).refcnt = 1 as uint32_t;
        (*de).leng = leng;
        memcpy(
            &raw const (*de).data as *const uint8_t as *mut uint8_t as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            leng as size_t,
        );
        dict_add(de);
        return de as *mut ::core::ffi::c_void;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_get_ptr(mut dptr: *mut ::core::ffi::c_void) -> *const uint8_t {
    unsafe {
        let mut de: *mut dictentry = dptr as *mut dictentry;
        return &raw const (*de).data as *const uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_get_leng(mut dptr: *mut ::core::ffi::c_void) -> uint32_t {
    unsafe {
        let mut de: *mut dictentry = dptr as *mut dictentry;
        return (*de).leng;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_get_hash(mut dptr: *mut ::core::ffi::c_void) -> uint32_t {
    unsafe {
        let mut de: *mut dictentry = dptr as *mut dictentry;
        return (*de).hashval;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_dec_ref(mut dptr: *mut ::core::ffi::c_void) {
    unsafe {
        let mut de: *mut dictentry = dptr as *mut dictentry;
        if (*de).refcnt > 0 as uint32_t {
        } else {
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/dictionary.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                139 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"de->refcnt>0\0".as_ptr() as *const ::core::ffi::c_char,
                b"dictionary reference counter is zero\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s' : %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/dictionary.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                139 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"de->refcnt>0\0".as_ptr() as *const ::core::ffi::c_char,
                b"dictionary reference counter is zero\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        (*de).refcnt = (*de).refcnt.wrapping_sub(1);
        if (*de).refcnt == 0 as uint32_t {
            dict_delete(de);
            free(de as *mut ::core::ffi::c_void);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_inc_ref(mut dptr: *mut ::core::ffi::c_void) {
    unsafe {
        let mut de: *mut dictentry = dptr as *mut dictentry;
        if (*de).refcnt > 0 as uint32_t {
        } else {
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/dictionary.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                149 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"de->refcnt>0\0".as_ptr() as *const ::core::ffi::c_char,
                b"dictionary reference counter is zero\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s' : %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/dictionary.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                149 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"de->refcnt>0\0".as_ptr() as *const ::core::ffi::c_char,
                b"dictionary reference counter is zero\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        (*de).refcnt = (*de).refcnt.wrapping_add(1);
    }
}
