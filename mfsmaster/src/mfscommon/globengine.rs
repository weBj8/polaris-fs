use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn monotonic_seconds() -> ::core::ffi::c_double;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn main_destruct_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
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
pub type globpattern = _globpattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _globpattern {
    pub sptab: *mut subpattern,
    pub spelements: uint8_t,
    pub minleng: uint32_t,
    pub flags: patflags,
}
pub type patflags = _patflags;
pub type _patflags = ::core::ffi::c_uint;
pub const PATFLAG_LASTASTERISK: _patflags = 2;
pub const PATFLAG_FIRSTASTERISK: _patflags = 1;
pub type subpattern = _subpattern;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _subpattern {
    pub attab: *mut atom,
    pub atelements: uint8_t,
    pub leng: uint8_t,
}
pub type atom = _atom;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _atom {
    pub r#type: atomtype,
    pub leng: uint32_t,
    pub str: *mut uint8_t,
    pub rangebits: [uint32_t; 8],
    pub next: *mut _atom,
}
pub type atomtype = _atomtype;
pub type _atomtype = ::core::ffi::c_uint;
pub const ATOM_RANGE: _atomtype = 3;
pub const ATOM_QMARK: _atomtype = 2;
pub const ATOM_ASTERISK: _atomtype = 1;
pub const ATOM_STRING: _atomtype = 0;
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
pub type globcache = _globcache;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _globcache {
    pub glob: *mut ::core::ffi::c_void,
    pub valid: uint8_t,
    pub gnleng: uint8_t,
    pub gname: *mut uint8_t,
    pub mt: ::core::ffi::c_double,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const GLOB_CACHE_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
static mut globtab: [globcache; 16] = [globcache {
    glob: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    valid: 0,
    gnleng: 0,
    gname: ::core::ptr::null_mut::<uint8_t>(),
    mt: 0.,
}; 16];
#[inline]
unsafe extern "C" fn parse_range(
    mut rangebits: *mut uint32_t,
    mut start: *const uint8_t,
    mut end: *const uint8_t,
) {
    let mut s: *const uint8_t = start;
    let mut e: *const uint8_t = end;
    let mut mask: uint32_t = 0;
    let mut pos: uint8_t = 0;
    let mut neg: uint8_t = 0 as uint8_t;
    let mut i: uint8_t = 0;
    if s < e && *s as ::core::ffi::c_int == '!' as ::core::ffi::c_int {
        s = s.offset(1);
        neg = 1 as uint8_t;
    }
    if neg != 0 {
        memset(
            rangebits as *mut ::core::ffi::c_void,
            0xffffffff as ::core::ffi::c_uint as ::core::ffi::c_int,
            ::core::mem::size_of::<uint32_t>().wrapping_mul(8 as size_t),
        );
    } else {
        memset(
            rangebits as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<uint32_t>().wrapping_mul(8 as size_t),
        );
    }
    while s < e {
        if s.offset(2 as ::core::ffi::c_int as isize) < e
            && *s.offset(1 as isize) as ::core::ffi::c_int == '-' as ::core::ffi::c_int
        {
            i = *s.offset(0 as isize);
            while i as ::core::ffi::c_int <= *s.offset(2 as isize) as ::core::ffi::c_int {
                pos = (i as ::core::ffi::c_int >> 5 as ::core::ffi::c_int) as uint8_t;
                mask = ((1 as ::core::ffi::c_uint)
                    << (i as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
                    as uint32_t;
                if neg != 0 {
                    *rangebits.offset(pos as isize) &= !mask;
                } else {
                    *rangebits.offset(pos as isize) |= mask;
                }
                i = i.wrapping_add(1);
            }
            s = s.offset(3 as ::core::ffi::c_int as isize);
        } else {
            let c2rust_fresh1 = s;
            s = s.offset(1);
            i = *c2rust_fresh1;
            pos = (i as ::core::ffi::c_int >> 5 as ::core::ffi::c_int) as uint8_t;
            mask = ((1 as ::core::ffi::c_uint)
                << (i as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
                as uint32_t;
            if neg != 0 {
                *rangebits.offset(pos as isize) &= !mask;
            } else {
                *rangebits.offset(pos as isize) |= mask;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn unescape_string(
    mut src: *const uint8_t,
    mut srcend: *mut *const uint8_t,
    mut dst: *mut uint8_t,
) -> uint32_t {
    let mut l: uint32_t = 0;
    let mut r: *const uint8_t = ::core::ptr::null::<uint8_t>();
    l = 0 as uint32_t;
    r = src;
    while *r as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if *r as ::core::ffi::c_int == '\\' as ::core::ffi::c_int {
            if *r.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            {
                if !dst.is_null() {
                    *dst.offset(l as isize) = *r.offset(1 as ::core::ffi::c_int as isize);
                }
                r = r.offset(2 as ::core::ffi::c_int as isize);
                l = l.wrapping_add(1);
            } else {
                if !dst.is_null() {
                    *dst.offset(l as isize) = *r;
                }
                r = r.offset(1);
                l = l.wrapping_add(1);
            }
        } else {
            if *r as ::core::ffi::c_int == '[' as ::core::ffi::c_int
                || *r as ::core::ffi::c_int == '*' as ::core::ffi::c_int
                || *r as ::core::ffi::c_int == '?' as ::core::ffi::c_int
            {
                break;
            }
            if !dst.is_null() {
                *dst.offset(l as isize) = *r;
            }
            r = r.offset(1);
            l = l.wrapping_add(1);
        }
    }
    if !srcend.is_null() {
        *srcend = r;
    }
    return l;
}
#[inline]
unsafe extern "C" fn pattern_to_atoms_list(mut globstr: *const uint8_t) -> *mut atom {
    let mut p: *const uint8_t = globstr;
    let mut r: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut c: uint8_t = 0;
    let mut head: *mut atom = ::core::ptr::null_mut::<atom>();
    let mut tail: *mut *mut atom = ::core::ptr::null_mut::<*mut atom>();
    let mut a: *mut atom = ::core::ptr::null_mut::<atom>();
    let mut last_asterisk: uint8_t = 0 as uint8_t;
    let mut l: uint32_t = 0;
    head = ::core::ptr::null_mut::<atom>();
    tail = &raw mut head;
    a = ::core::ptr::null_mut::<atom>();
    loop {
        let c2rust_fresh0 = p;
        p = p.offset(1);
        c = *c2rust_fresh0;
        if c as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            break;
        }
        if c as ::core::ffi::c_int == '*' as ::core::ffi::c_int {
            if last_asterisk as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                a = malloc(::core::mem::size_of::<atom>()) as *mut atom;
                if a.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        170 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"a\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        170 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"a\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if a
                    == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut atom
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        170 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"a\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        170 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"a\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
                memset(
                    a as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<atom>(),
                );
                (*a).r#type = ATOM_ASTERISK;
                (*a).next = ::core::ptr::null_mut::<_atom>();
                *tail = a;
                tail = &raw mut (*a).next as *mut *mut atom;
                last_asterisk = 1 as uint8_t;
            }
        } else if c as ::core::ffi::c_int == '?' as ::core::ffi::c_int {
            a = malloc(::core::mem::size_of::<atom>()) as *mut atom;
            if a.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"a\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"a\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if a
                == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut atom
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"a\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    181 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"a\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
            memset(
                a as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<atom>(),
            );
            (*a).r#type = ATOM_QMARK;
            (*a).next = ::core::ptr::null_mut::<_atom>();
            *tail = a;
            tail = &raw mut (*a).next as *mut *mut atom;
            last_asterisk = 0 as uint8_t;
        } else {
            if c as ::core::ffi::c_int == '[' as ::core::ffi::c_int {
                r = p;
                while *r as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                    && *r as ::core::ffi::c_int != ']' as ::core::ffi::c_int
                {
                    r = r.offset(1);
                }
                if *r as ::core::ffi::c_int == ']' as ::core::ffi::c_int {
                    a = malloc(::core::mem::size_of::<atom>()) as *mut atom;
                    if a.is_null() {
                        fprintf(
                            stderr,
                            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            196 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"a\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            196 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"a\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        abort();
                    } else if a
                        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                            -1 as ::core::ffi::c_int as usize,
                        ) as *mut atom
                    {
                        let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            196 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"a\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring_1,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            196 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"a\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring_1,
                        );
                        abort();
                    }
                    memset(
                        a as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<atom>(),
                    );
                    (*a).r#type = ATOM_RANGE;
                    parse_range(&raw mut (*a).rangebits as *mut uint32_t, p, r);
                    p = r.offset(1 as ::core::ffi::c_int as isize);
                    (*a).next = ::core::ptr::null_mut::<_atom>();
                    *tail = a;
                    tail = &raw mut (*a).next as *mut *mut atom;
                    last_asterisk = 0 as uint8_t;
                    continue;
                }
            }
            if c as ::core::ffi::c_int == '[' as ::core::ffi::c_int {
                l = unescape_string(
                    p,
                    ::core::ptr::null_mut::<*const uint8_t>(),
                    ::core::ptr::null_mut::<uint8_t>(),
                )
                .wrapping_add(1 as uint32_t);
            } else {
                l = unescape_string(
                    p.offset(-(1 as ::core::ffi::c_int as isize)),
                    ::core::ptr::null_mut::<*const uint8_t>(),
                    ::core::ptr::null_mut::<uint8_t>(),
                );
            }
            a = malloc(::core::mem::size_of::<atom>()) as *mut atom;
            if a.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    214 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"a\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    214 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"a\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if a
                == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut atom
            {
                let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    214 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"a\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    214 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"a\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_2,
                );
                abort();
            }
            memset(
                a as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<atom>(),
            );
            (*a).r#type = ATOM_STRING;
            (*a).leng = l;
            (*a).str = malloc((*a).leng as size_t) as *mut uint8_t;
            if (*a).str.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    219 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"a->str\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    219 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"a->str\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if (*a).str
                == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint8_t
            {
                let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    219 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"a->str\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    219 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"a->str\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_3,
                );
                abort();
            }
            if c as ::core::ffi::c_int == '[' as ::core::ffi::c_int {
                *(*a).str.offset(0 as isize) = '[' as uint8_t;
                unescape_string(
                    p,
                    &raw mut p,
                    (*a).str.offset(1 as ::core::ffi::c_int as isize),
                );
            } else {
                unescape_string(
                    p.offset(-(1 as ::core::ffi::c_int as isize)),
                    &raw mut p,
                    (*a).str,
                );
            }
            (*a).next = ::core::ptr::null_mut::<_atom>();
            *tail = a;
            tail = &raw mut (*a).next as *mut *mut atom;
            last_asterisk = 0 as uint8_t;
        }
    }
    return head;
}
#[inline]
unsafe extern "C" fn atom_range_match(mut rangebits: *mut uint32_t, mut c: uint8_t) -> uint8_t {
    let mut pos: uint8_t = (c as ::core::ffi::c_int >> 5 as ::core::ffi::c_int) as uint8_t;
    let mut mask: uint32_t =
        (1 as uint32_t) << (c as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int);
    return (if *rangebits.offset(pos as isize) & mask != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as uint8_t;
}
#[inline]
unsafe extern "C" fn subpattern_match_exact(
    mut sp: *mut subpattern,
    mut name: *const uint8_t,
    mut nleng: uint8_t,
) -> uint8_t {
    let mut a: *mut atom = ::core::ptr::null_mut::<atom>();
    let mut atpos: uint8_t = 0;
    atpos = 0 as uint8_t;
    while (atpos as ::core::ffi::c_int) < (*sp).atelements as ::core::ffi::c_int {
        a = (*sp).attab.offset(atpos as ::core::ffi::c_int as isize);
        match (*a).r#type as ::core::ffi::c_uint {
            0 => {
                if (*a).leng > nleng as uint32_t
                    || memcmp(
                        name as *const ::core::ffi::c_void,
                        (*a).str as *const ::core::ffi::c_void,
                        (*a).leng as size_t,
                    ) != 0 as ::core::ffi::c_int
                {
                    return 0 as uint8_t;
                }
                name = name.offset((*a).leng as isize);
                nleng = (nleng as uint32_t).wrapping_sub((*a).leng) as uint8_t;
            }
            2 => {
                if nleng as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    return 0 as uint8_t;
                }
                name = name.offset(1 as ::core::ffi::c_int as isize);
                nleng = (nleng as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as uint8_t;
            }
            3 => {
                if nleng as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    || atom_range_match(
                        &raw mut (*a).rangebits as *mut uint32_t,
                        *name.offset(0 as isize),
                    ) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                {
                    return 0 as uint8_t;
                }
                name = name.offset(1 as ::core::ffi::c_int as isize);
                nleng = (nleng as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as uint8_t;
            }
            _ => return 0 as uint8_t,
        }
        atpos = atpos.wrapping_add(1);
    }
    return 1 as uint8_t;
}
#[inline]
unsafe extern "C" fn subpattern_closest_match(
    mut sp: *mut subpattern,
    mut name: *const uint8_t,
    mut nleng: uint8_t,
) -> ::core::ffi::c_int {
    let mut pos: uint32_t = 0;
    pos = 0 as uint32_t;
    while (*sp).leng as uint32_t <= (nleng as uint32_t).wrapping_sub(pos) {
        if subpattern_match_exact(
            sp,
            name.offset(pos as isize),
            (nleng as uint32_t).wrapping_sub(pos) as uint8_t,
        ) != 0
        {
            return pos as ::core::ffi::c_int;
        }
        pos = pos.wrapping_add(1);
    }
    return -1 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn pattern_match(
    mut p: *mut globpattern,
    mut name: *const uint8_t,
    mut nleng: uint8_t,
) -> uint8_t {
    let mut pos: ::core::ffi::c_int = 0;
    let mut i: uint8_t = 0;
    if (nleng as uint32_t) < (*p).minleng {
        return 0 as uint8_t;
    }
    if (*p).spelements as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        if (*p).flags as ::core::ffi::c_uint
            & (PATFLAG_LASTASTERISK as ::core::ffi::c_int
                | PATFLAG_FIRSTASTERISK as ::core::ffi::c_int) as ::core::ffi::c_uint
            != 0
        {
            return 1 as uint8_t;
        }
        return 0 as uint8_t;
    }
    if (*p).spelements as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        if (*(*p).sptab.offset(0 as isize)).leng as ::core::ffi::c_int > nleng as ::core::ffi::c_int
        {
            return 0 as uint8_t;
        }
        if (*p).flags as ::core::ffi::c_uint
            & (PATFLAG_LASTASTERISK as ::core::ffi::c_int
                | PATFLAG_FIRSTASTERISK as ::core::ffi::c_int) as ::core::ffi::c_uint
            == (PATFLAG_LASTASTERISK as ::core::ffi::c_int
                | PATFLAG_FIRSTASTERISK as ::core::ffi::c_int) as ::core::ffi::c_uint
        {
            if subpattern_closest_match((*p).sptab, name, nleng) >= 0 as ::core::ffi::c_int {
                return 1 as uint8_t;
            }
            return 0 as uint8_t;
        }
        if (*p).flags as ::core::ffi::c_uint
            & PATFLAG_LASTASTERISK as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            return subpattern_match_exact((*p).sptab, name, nleng);
        }
        if (*p).flags as ::core::ffi::c_uint
            & PATFLAG_FIRSTASTERISK as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            return subpattern_match_exact(
                (*p).sptab,
                name.offset(
                    (nleng as ::core::ffi::c_int
                        - (*(*p).sptab.offset(0 as isize)).leng as ::core::ffi::c_int)
                        as isize,
                ),
                (*(*p).sptab.offset(0 as isize)).leng,
            );
        }
        if (*(*p).sptab.offset(0 as isize)).leng as ::core::ffi::c_int
            != nleng as ::core::ffi::c_int
        {
            return 0 as uint8_t;
        }
        return subpattern_match_exact((*p).sptab, name, nleng);
    }
    if (*p).flags as ::core::ffi::c_uint
        & PATFLAG_FIRSTASTERISK as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        pos = subpattern_closest_match((*p).sptab, name, nleng);
        if pos < 0 as ::core::ffi::c_int {
            return 0 as uint8_t;
        }
        name = name
            .offset((pos + (*(*p).sptab.offset(0 as isize)).leng as ::core::ffi::c_int) as isize);
        nleng = (nleng as ::core::ffi::c_int
            - (pos + (*(*p).sptab.offset(0 as isize)).leng as ::core::ffi::c_int))
            as uint8_t;
    } else {
        if subpattern_match_exact((*p).sptab, name, nleng) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            return 0 as uint8_t;
        }
        name = name.offset((*(*p).sptab.offset(0 as isize)).leng as ::core::ffi::c_int as isize);
        nleng = (nleng as ::core::ffi::c_int
            - (*(*p).sptab.offset(0 as isize)).leng as ::core::ffi::c_int)
            as uint8_t;
    }
    i = 1 as uint8_t;
    while (i as ::core::ffi::c_int)
        < (*p).spelements as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        pos = subpattern_closest_match(
            (*p).sptab.offset(i as ::core::ffi::c_int as isize),
            name,
            nleng,
        );
        if pos < 0 as ::core::ffi::c_int {
            return 0 as uint8_t;
        }
        name = name
            .offset((pos + (*(*p).sptab.offset(0 as isize)).leng as ::core::ffi::c_int) as isize);
        nleng = (nleng as ::core::ffi::c_int
            - (pos + (*(*p).sptab.offset(0 as isize)).leng as ::core::ffi::c_int))
            as uint8_t;
        i = i.wrapping_add(1);
    }
    if (*p).flags as ::core::ffi::c_uint
        & PATFLAG_LASTASTERISK as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0
    {
        pos = subpattern_closest_match(
            (*p).sptab.offset(i as ::core::ffi::c_int as isize),
            name,
            nleng,
        );
        return (if pos < 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as uint8_t;
    } else {
        if (nleng as ::core::ffi::c_int)
            < (*(*p).sptab.offset(i as isize)).leng as ::core::ffi::c_int
        {
            return 0 as uint8_t;
        }
        return subpattern_match_exact(
            (*p).sptab.offset(i as ::core::ffi::c_int as isize),
            name.offset(
                (nleng as ::core::ffi::c_int
                    - (*(*p).sptab.offset(i as isize)).leng as ::core::ffi::c_int)
                    as isize,
            ),
            (*(*p).sptab.offset(i as isize)).leng,
        );
    };
}
#[inline]
unsafe extern "C" fn atoms_list_to_pattern_structure(
    mut p: *mut globpattern,
    mut atlist: *mut atom,
) {
    let mut a: *mut atom = ::core::ptr::null_mut::<atom>();
    let mut an: *mut atom = ::core::ptr::null_mut::<atom>();
    let mut l: uint8_t = 0;
    let mut m: uint8_t = 0;
    (*p).minleng = 0 as uint32_t;
    (*p).flags = 0 as patflags;
    (*p).spelements = 0 as uint8_t;
    (*p).sptab = ::core::ptr::null_mut::<subpattern>();
    if atlist.is_null() {
        return;
    }
    if (*atlist).r#type as ::core::ffi::c_uint
        == ATOM_ASTERISK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*p).flags = ((*p).flags as ::core::ffi::c_uint
            | PATFLAG_FIRSTASTERISK as ::core::ffi::c_int as ::core::ffi::c_uint)
            as patflags;
        an = (*atlist).next as *mut atom;
        free(atlist as *mut ::core::ffi::c_void);
        atlist = an;
        if atlist.is_null() {
            (*p).flags = ((*p).flags as ::core::ffi::c_uint
                | PATFLAG_LASTASTERISK as ::core::ffi::c_int as ::core::ffi::c_uint)
                as patflags;
            return;
        }
    }
    (*p).spelements = 1 as uint8_t;
    a = atlist;
    while !a.is_null() {
        if (*a).r#type as ::core::ffi::c_uint
            == ATOM_ASTERISK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if !(*a).next.is_null() {
                (*p).spelements = (*p).spelements.wrapping_add(1);
            } else {
                (*p).flags = ((*p).flags as ::core::ffi::c_uint
                    | PATFLAG_LASTASTERISK as ::core::ffi::c_int as ::core::ffi::c_uint)
                    as patflags;
            }
        }
        a = (*a).next as *mut atom;
    }
    (*p).sptab =
        malloc(::core::mem::size_of::<subpattern>().wrapping_mul((*p).spelements as size_t))
            as *mut subpattern;
    if (*p).sptab.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            435 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"p->sptab\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            435 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"p->sptab\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if (*p).sptab
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut subpattern
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            435 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"p->sptab\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            435 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"p->sptab\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    l = 0 as uint8_t;
    while (l as ::core::ffi::c_int) < (*p).spelements as ::core::ffi::c_int {
        (*(*p).sptab.offset(l as isize)).atelements = 0 as uint8_t;
        (*(*p).sptab.offset(l as isize)).leng = 0 as uint8_t;
        l = l.wrapping_add(1);
    }
    l = 0 as uint8_t;
    a = atlist;
    while !a.is_null() {
        if (*a).r#type as ::core::ffi::c_uint
            == ATOM_ASTERISK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            l = l.wrapping_add(1);
        } else {
            (*(*p).sptab.offset(l as isize)).atelements =
                (*(*p).sptab.offset(l as isize)).atelements.wrapping_add(1);
            if (*a).r#type as ::core::ffi::c_uint
                == ATOM_STRING as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*(*p).sptab.offset(l as isize)).leng =
                    ((*(*p).sptab.offset(l as isize)).leng as uint32_t).wrapping_add((*a).leng)
                        as uint8_t;
            } else {
                (*(*p).sptab.offset(l as isize)).leng =
                    ((*(*p).sptab.offset(l as isize)).leng as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as uint8_t;
            }
        }
        a = (*a).next as *mut atom;
    }
    l = 0 as uint8_t;
    while (l as ::core::ffi::c_int) < (*p).spelements as ::core::ffi::c_int {
        (*(*p).sptab.offset(l as isize)).attab = malloc(
            ::core::mem::size_of::<atom>()
                .wrapping_mul((*(*p).sptab.offset(l as isize)).atelements as size_t),
        ) as *mut atom;
        if (*(*p).sptab.offset(l as isize)).attab.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                455 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"p->sptab[l].attab\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                455 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"p->sptab[l].attab\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if (*(*p).sptab.offset(l as isize)).attab
            == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut atom
        {
            let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                455 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"p->sptab[l].attab\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                455 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"p->sptab[l].attab\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            abort();
        }
        (*p).minleng = (*p)
            .minleng
            .wrapping_add((*(*p).sptab.offset(l as isize)).leng as uint32_t);
        l = l.wrapping_add(1);
    }
    l = 0 as uint8_t;
    m = 0 as uint8_t;
    a = atlist;
    while !a.is_null() {
        if (*a).r#type as ::core::ffi::c_uint
            == ATOM_ASTERISK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            l = l.wrapping_add(1);
            m = 0 as uint8_t;
        } else {
            if (l as ::core::ffi::c_int) < (*p).spelements as ::core::ffi::c_int {
            } else {
                fprintf(
                    stderr,
                    b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    465 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"l<p->spelements\0".as_ptr() as *const ::core::ffi::c_char,
                    b"wrong pattern elements count\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - failed assertion '%s' : %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    465 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"l<p->spelements\0".as_ptr() as *const ::core::ffi::c_char,
                    b"wrong pattern elements count\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            };
            memcpy(
                (*(*p).sptab.offset(l as isize))
                    .attab
                    .offset(m as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_void,
                a as *const ::core::ffi::c_void,
                ::core::mem::size_of::<atom>(),
            );
            m = m.wrapping_add(1);
        }
        an = (*a).next as *mut atom;
        free(a as *mut ::core::ffi::c_void);
        a = an;
    }
}
#[inline]
unsafe extern "C" fn free_pattern_structure(mut p: *mut globpattern) {
    let mut i: uint8_t = 0;
    let mut j: uint8_t = 0;
    i = 0 as uint8_t;
    while (i as ::core::ffi::c_int) < (*p).spelements as ::core::ffi::c_int {
        j = 0 as uint8_t;
        while (j as ::core::ffi::c_int)
            < (*(*p).sptab.offset(i as isize)).atelements as ::core::ffi::c_int
        {
            if (*(*(*p).sptab.offset(i as isize)).attab.offset(j as isize)).r#type
                as ::core::ffi::c_uint
                == ATOM_STRING as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                free(
                    (*(*(*p).sptab.offset(i as isize)).attab.offset(j as isize)).str
                        as *mut ::core::ffi::c_void,
                );
            }
            j = j.wrapping_add(1);
        }
        if !(*(*p).sptab.offset(i as isize)).attab.is_null() {
            free((*(*p).sptab.offset(i as isize)).attab as *mut ::core::ffi::c_void);
        }
        i = i.wrapping_add(1);
    }
    if !(*p).sptab.is_null() {
        free((*p).sptab as *mut ::core::ffi::c_void);
    }
    (*p).spelements = 0 as uint8_t;
    (*p).minleng = 0 as uint32_t;
    (*p).sptab = ::core::ptr::null_mut::<subpattern>();
}
#[no_mangle]
pub unsafe extern "C" fn glob_new(mut globstr: *const uint8_t) -> *mut ::core::ffi::c_void {
    let mut p: *mut globpattern = ::core::ptr::null_mut::<globpattern>();
    p = malloc(::core::mem::size_of::<globpattern>()) as *mut globpattern;
    if p.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            530 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"p\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            530 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"p\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if p
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut globpattern
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            530 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"p\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            530 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"p\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    atoms_list_to_pattern_structure(p, pattern_to_atoms_list(globstr));
    return p as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn glob_free(mut glob: *mut ::core::ffi::c_void) {
    let mut p: *mut globpattern = glob as *mut globpattern;
    free_pattern_structure(p);
    free(p as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn glob_match(
    mut glob: *mut ::core::ffi::c_void,
    mut name: *const uint8_t,
    mut nleng: uint8_t,
) -> uint8_t {
    let mut p: *mut globpattern = glob as *mut globpattern;
    return pattern_match(p, name, nleng);
}
#[no_mangle]
pub unsafe extern "C" fn glob_cache_get(
    mut gnleng: uint8_t,
    mut gname: *const uint8_t,
) -> *mut ::core::ffi::c_void {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    j = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < GLOB_CACHE_SIZE {
        if globtab[i as usize].valid as ::core::ffi::c_int != 0
            && globtab[i as usize].gnleng as ::core::ffi::c_int == gnleng as ::core::ffi::c_int
            && (gnleng as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || memcmp(
                    globtab[i as usize].gname as *const ::core::ffi::c_void,
                    gname as *const ::core::ffi::c_void,
                    gnleng as size_t,
                ) == 0 as ::core::ffi::c_int)
        {
            globtab[i as usize].mt = monotonic_seconds();
            return globtab[i as usize].glob;
        }
        if globtab[i as usize].valid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || globtab[i as usize].mt < globtab[j as usize].mt
        {
            j = i;
        }
        i += 1;
    }
    if globtab[j as usize].valid != 0 {
        glob_free(globtab[j as usize].glob);
        free(globtab[j as usize].gname as *mut ::core::ffi::c_void);
    }
    globtab[j as usize].gname =
        malloc((gnleng as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t) as *mut uint8_t;
    if globtab[j as usize].gname.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            563 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"globtab[j].gname\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            563 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"globtab[j].gname\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if globtab[j as usize].gname
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut uint8_t
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            563 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"globtab[j].gname\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/globengine.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            563 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"globtab[j].gname\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    memcpy(
        globtab[j as usize].gname as *mut ::core::ffi::c_void,
        gname as *const ::core::ffi::c_void,
        gnleng as size_t,
    );
    *globtab[j as usize].gname.offset(gnleng as isize) = 0 as uint8_t;
    globtab[j as usize].glob = glob_new(globtab[j as usize].gname);
    globtab[j as usize].mt = monotonic_seconds();
    globtab[j as usize].valid = 1 as uint8_t;
    return globtab[j as usize].glob;
}
#[no_mangle]
pub unsafe extern "C" fn glob_cache_term() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < GLOB_CACHE_SIZE {
        if globtab[i as usize].valid != 0 {
            glob_free(globtab[i as usize].glob);
            free(globtab[i as usize].gname as *mut ::core::ffi::c_void);
        }
        globtab[i as usize].valid = 0 as uint8_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn glob_cache_init() -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < GLOB_CACHE_SIZE {
        globtab[i as usize].glob = NULL;
        globtab[i as usize].valid = 0 as uint8_t;
        globtab[i as usize].gnleng = 0 as uint8_t;
        globtab[i as usize].gname = ::core::ptr::null_mut::<uint8_t>();
        globtab[i as usize].mt = 0.0f64;
        i += 1;
    }
    main_destruct_register_fname(
        Some(glob_cache_term as unsafe extern "C" fn() -> ()),
        b"glob_cache_term\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
