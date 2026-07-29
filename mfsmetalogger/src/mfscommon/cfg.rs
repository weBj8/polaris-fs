use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn __getdelim(
        __lineptr: *mut *mut ::core::ffi::c_char,
        __n: *mut size_t,
        __delimiter: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: ::core::ffi::c_long,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ftell(__stream: *mut FILE) -> ::core::ffi::c_long;
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strtod(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn strtoll(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
    fn strtoull(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulonglong;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn parse_speriod(str: *const ::core::ffi::c_char, ret: *mut uint32_t) -> ::core::ffi::c_int;
    fn parse_hperiod(str: *const ::core::ffi::c_char, ret: *mut uint32_t) -> ::core::ffi::c_int;
    fn md5_init(ctx: *mut md5ctx);
    fn md5_update(ctx: *mut md5ctx, buff: *const uint8_t, leng: uint32_t);
    fn md5_final(digest: *mut uint8_t, ctx: *mut md5ctx);
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
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
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cfg_buff {
    pub leng: uint32_t,
    pub data: [uint8_t; 1],
}
pub type paramstr = paramsstr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct paramsstr {
    pub name: *mut ::core::ffi::c_char,
    pub value: *mut ::core::ffi::c_char,
    pub next: *mut paramsstr,
}
pub type md5ctx = _md5ctx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _md5ctx {
    pub state: [uint32_t; 4],
    pub count: [uint32_t; 2],
    pub buffer: [uint8_t; 64],
}
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut ::core::ffi::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as ::core::ffi::c_int, __stream);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_ERRNO_SYSLOG_STDERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const TPARSE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TPARSE_UNEXPECTED_CHAR: ::core::ffi::c_int = -1;
pub const TPARSE_VALUE_TOO_BIG: ::core::ffi::c_int = -2;
static mut cfgfname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut paramhead: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
static mut usedhead: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
static mut logundefined: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut dangerous: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn cfg_reload() -> ::core::ffi::c_int {
    let mut fd: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut linebuff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut lbsize: size_t = 0;
    let mut nps: uint32_t = 0;
    let mut npe: uint32_t = 0;
    let mut vps: uint32_t = 0;
    let mut vpe: uint32_t = 0;
    let mut i: uint32_t = 0;
    let mut found: uint8_t = 0;
    let mut tmp: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
    fd = fopen(cfgfname, b"r\0".as_ptr() as *const ::core::ffi::c_char) as *mut FILE;
    if fd.is_null() {
        if *__errno_location() == ENOENT {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"main config file (%s) not found\0".as_ptr() as *const ::core::ffi::c_char,
                cfgfname,
            );
        } else {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"can't load main config file (%s), error\0".as_ptr() as *const ::core::ffi::c_char,
                cfgfname,
            );
        }
        return 0 as ::core::ffi::c_int;
    }
    while !paramhead.is_null() {
        tmp = paramhead;
        paramhead = (*tmp).next as *mut paramstr;
        free((*tmp).name as *mut ::core::ffi::c_void);
        free((*tmp).value as *mut ::core::ffi::c_void);
        free(tmp as *mut ::core::ffi::c_void);
    }
    lbsize = 1000 as size_t;
    linebuff = malloc(lbsize) as *mut ::core::ffi::c_char;
    while getline(&raw mut linebuff, &raw mut lbsize, fd) != -1 as __ssize_t {
        if *linebuff.offset(0 as isize) as ::core::ffi::c_int == '#' as ::core::ffi::c_int {
            continue;
        }
        i = 0 as uint32_t;
        while *linebuff.offset(i as isize) as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || *linebuff.offset(i as isize) as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
        {
            i = i.wrapping_add(1);
        }
        nps = i;
        while *linebuff.offset(i as isize) as ::core::ffi::c_int > 32 as ::core::ffi::c_int
            && (*linebuff.offset(i as isize) as ::core::ffi::c_int) < 127 as ::core::ffi::c_int
            && *linebuff.offset(i as isize) as ::core::ffi::c_int != '=' as ::core::ffi::c_int
        {
            i = i.wrapping_add(1);
        }
        npe = i;
        while *linebuff.offset(i as isize) as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || *linebuff.offset(i as isize) as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
        {
            i = i.wrapping_add(1);
        }
        if *linebuff.offset(i as isize) as ::core::ffi::c_int != '=' as ::core::ffi::c_int
            || npe == nps
        {
            if *linebuff.offset(i as isize) as ::core::ffi::c_int > 32 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"bad definition in config file '%s': %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    cfgfname,
                    linebuff,
                );
            }
        } else {
            i = i.wrapping_add(1);
            while *linebuff.offset(i as isize) as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                || *linebuff.offset(i as isize) as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
            {
                i = i.wrapping_add(1);
            }
            vps = i;
            while (*linebuff.offset(i as isize) as ::core::ffi::c_int) >= 32 as ::core::ffi::c_int {
                i = i.wrapping_add(1);
            }
            while i > vps
                && *linebuff.offset(i.wrapping_sub(1 as uint32_t) as isize) as ::core::ffi::c_int
                    == 32 as ::core::ffi::c_int
            {
                i = i.wrapping_sub(1);
            }
            vpe = i;
            while *linebuff.offset(i as isize) as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                || *linebuff.offset(i as isize) as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
            {
                i = i.wrapping_add(1);
            }
            if *linebuff.offset(i as isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
                && *linebuff.offset(i as isize) as ::core::ffi::c_int != '\r' as ::core::ffi::c_int
                && *linebuff.offset(i as isize) as ::core::ffi::c_int != '\n' as ::core::ffi::c_int
                && *linebuff.offset(i as isize) as ::core::ffi::c_int != '#' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"bad definition in config file '%s': %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    cfgfname,
                    linebuff,
                );
            } else {
                *linebuff.offset(npe as isize) = 0 as ::core::ffi::c_char;
                *linebuff.offset(vpe as isize) = 0 as ::core::ffi::c_char;
                found = 0 as uint8_t;
                if npe.wrapping_sub(nps) >= 10 as uint32_t
                    && memcmp(
                        linebuff.offset(nps as isize) as *const ::core::ffi::c_void,
                        b"DANGEROUS_\0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        10 as size_t,
                    ) == 0 as ::core::ffi::c_int
                {
                    dangerous = 1 as ::core::ffi::c_int;
                }
                tmp = paramhead;
                while !tmp.is_null() && found as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    if strcmp((*tmp).name, linebuff.offset(nps as isize)) == 0 as ::core::ffi::c_int
                    {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"variable '%s' defined more than once in the config file (previous value: %s, current value: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            (*tmp).name,
                            (*tmp).value,
                            linebuff.offset(vps as isize),
                        );
                        free((*tmp).value as *mut ::core::ffi::c_void);
                        (*tmp).value =
                            malloc(vpe.wrapping_sub(vps).wrapping_add(1 as uint32_t) as size_t)
                                as *mut ::core::ffi::c_char;
                        memcpy(
                            (*tmp).value as *mut ::core::ffi::c_void,
                            linebuff.offset(vps as isize) as *const ::core::ffi::c_void,
                            vpe.wrapping_sub(vps).wrapping_add(1 as uint32_t) as size_t,
                        );
                        found = 1 as uint8_t;
                    }
                    tmp = (*tmp).next as *mut paramstr;
                }
                if found as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    tmp = malloc(::core::mem::size_of::<paramstr>()) as *mut paramstr;
                    (*tmp).name = malloc(npe.wrapping_sub(nps).wrapping_add(1 as uint32_t) as size_t)
                        as *mut ::core::ffi::c_char;
                    (*tmp).value =
                        malloc(vpe.wrapping_sub(vps).wrapping_add(1 as uint32_t) as size_t)
                            as *mut ::core::ffi::c_char;
                    memcpy(
                        (*tmp).name as *mut ::core::ffi::c_void,
                        linebuff.offset(nps as isize) as *const ::core::ffi::c_void,
                        npe.wrapping_sub(nps).wrapping_add(1 as uint32_t) as size_t,
                    );
                    memcpy(
                        (*tmp).value as *mut ::core::ffi::c_void,
                        linebuff.offset(vps as isize) as *const ::core::ffi::c_void,
                        vpe.wrapping_sub(vps).wrapping_add(1 as uint32_t) as size_t,
                    );
                    (*tmp).next = paramhead as *mut paramsstr;
                    paramhead = tmp;
                }
            }
        }
    }
    free(linebuff as *mut ::core::ffi::c_void);
    fclose(fd);
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_use_option(
    mut name: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) {
    let mut i: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
    let mut inp: *mut *mut paramstr = ::core::ptr::null_mut::<*mut paramstr>();
    inp = &raw mut usedhead;
    loop {
        i = *inp;
        if i.is_null() {
            break;
        }
        if strcmp((*i).name, name) == 0 as ::core::ffi::c_int {
            free((*i).value as *mut ::core::ffi::c_void);
            (*i).value = strdup(value);
            return;
        }
        inp = &raw mut (*i).next as *mut *mut paramstr;
    }
    i = malloc(::core::mem::size_of::<paramstr>()) as *mut paramstr;
    (*i).name = strdup(name);
    (*i).value = strdup(value);
    (*i).next = ::core::ptr::null_mut::<paramsstr>();
    *inp = i;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_info(mut fd: *mut FILE) {
    let mut i: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
    fprintf(fd, b"[config]\n\0".as_ptr() as *const ::core::ffi::c_char);
    i = usedhead;
    while !i.is_null() {
        fprintf(
            fd,
            b"%s = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            (*i).name,
            (*i).value,
        );
        i = (*i).next as *mut paramstr;
    }
    fprintf(fd, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_load(
    mut configfname: *const ::core::ffi::c_char,
    mut _lu: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    paramhead = ::core::ptr::null_mut::<paramstr>();
    usedhead = ::core::ptr::null_mut::<paramstr>();
    logundefined = _lu;
    dangerous = 0 as ::core::ffi::c_int;
    cfgfname = strdup(configfname);
    return cfg_reload();
}
#[no_mangle]
pub unsafe extern "C" fn cfg_dangerous_options() -> ::core::ffi::c_int {
    return dangerous;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_isdefined(mut name: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut _cfg_tmp: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
    _cfg_tmp = paramhead;
    while !_cfg_tmp.is_null() {
        if strcmp(name, (*_cfg_tmp).name) == 0 as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
        _cfg_tmp = (*_cfg_tmp).next as *mut paramstr;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_term() {
    let mut i: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
    let mut r#in: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
    i = paramhead;
    while !i.is_null() {
        r#in = (*i).next as *mut paramstr;
        free((*i).value as *mut ::core::ffi::c_void);
        free((*i).name as *mut ::core::ffi::c_void);
        free(i as *mut ::core::ffi::c_void);
        i = r#in;
    }
    paramhead = ::core::ptr::null_mut::<paramstr>();
    i = usedhead;
    while !i.is_null() {
        r#in = (*i).next as *mut paramstr;
        free((*i).value as *mut ::core::ffi::c_void);
        free((*i).name as *mut ::core::ffi::c_void);
        free(i as *mut ::core::ffi::c_void);
        i = r#in;
    }
    usedhead = ::core::ptr::null_mut::<paramstr>();
    free(cfgfname as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn cfg_findname(
    mut name: *const ::core::ffi::c_char,
    mut usedflag: uint8_t,
) -> *mut paramstr {
    let mut psp: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
    psp = if usedflag as ::core::ffi::c_int != 0 {
        usedhead
    } else {
        paramhead
    };
    while !psp.is_null() {
        if strcmp(name, (*psp).name) == 0 as ::core::ffi::c_int {
            return psp;
        }
        psp = (*psp).next as *mut paramstr;
    }
    return ::core::ptr::null_mut::<paramstr>();
}
#[no_mangle]
pub unsafe extern "C" fn cfg_getdefaultstr(
    mut name: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut psp: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
    psp = cfg_findname(name, 0 as uint8_t);
    if psp.is_null() {
        psp = cfg_findname(name, 1 as uint8_t);
    }
    if psp.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return strdup((*psp).value);
}
#[no_mangle]
pub unsafe extern "C" fn cfg_getdefaultfile(
    mut name: *const ::core::ffi::c_char,
    mut maxleng: uint32_t,
) -> *mut cfg_buff {
    let mut psp: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
    let mut ret: *mut cfg_buff = ::core::ptr::null_mut::<cfg_buff>();
    let mut fd: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut fsize: ::core::ffi::c_ulong = 0;
    psp = cfg_findname(name, 0 as uint8_t);
    if psp.is_null() {
        psp = cfg_findname(name, 1 as uint8_t);
    }
    if psp.is_null() {
        return ::core::ptr::null_mut::<cfg_buff>();
    }
    fd = fopen((*psp).value, b"rb\0".as_ptr() as *const ::core::ffi::c_char) as *mut FILE;
    if fd.is_null() {
        return ::core::ptr::null_mut::<cfg_buff>();
    }
    fseek(fd, 0 as ::core::ffi::c_long, SEEK_END);
    fsize = ftell(fd) as ::core::ffi::c_ulong;
    fseek(fd, 0 as ::core::ffi::c_long, SEEK_SET);
    if fsize > maxleng as ::core::ffi::c_ulong {
        fclose(fd);
        return ::core::ptr::null_mut::<cfg_buff>();
    }
    ret = malloc((4 as size_t).wrapping_add(fsize as size_t)) as *mut cfg_buff;
    if ret.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/cfg.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            261 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"ret\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/cfg.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            261 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"ret\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if ret
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut cfg_buff
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/cfg.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            261 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"ret\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/cfg.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            261 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"ret\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    if fread(
        &raw mut (*ret).data as *mut uint8_t as *mut ::core::ffi::c_void,
        1 as size_t,
        fsize as size_t,
        fd,
    ) != fsize
    {
        free(ret as *mut ::core::ffi::c_void);
        fclose(fd);
        return ::core::ptr::null_mut::<cfg_buff>();
    }
    fclose(fd);
    (*ret).leng = fsize as uint32_t;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_getdefaultfilemd5(
    mut name: *const ::core::ffi::c_char,
    mut txtmode: uint8_t,
    mut digest: *mut uint8_t,
) -> ::core::ffi::c_int {
    let mut psp: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
    let mut fd: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut linebuff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut binarybuff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut fsize: ::core::ffi::c_ulong = 0;
    let mut lbsize: size_t = 0;
    let mut s: uint32_t = 0;
    let mut md5c: md5ctx = md5ctx {
        state: [0; 4],
        count: [0; 2],
        buffer: [0; 64],
    };
    psp = cfg_findname(name, 0 as uint8_t);
    if psp.is_null() {
        psp = cfg_findname(name, 1 as uint8_t);
    }
    if psp.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    fd = fopen((*psp).value, b"rb\0".as_ptr() as *const ::core::ffi::c_char) as *mut FILE;
    if fd.is_null() {
        return -1 as ::core::ffi::c_int;
    }
    md5_init(&raw mut md5c);
    if txtmode != 0 {
        lbsize = 10000 as size_t;
        linebuff = malloc(lbsize) as *mut ::core::ffi::c_char;
        if linebuff.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/cfg.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                297 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"linebuff\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/cfg.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                297 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"linebuff\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if linebuff
            == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut ::core::ffi::c_char
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/cfg.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                297 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"linebuff\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/cfg.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                297 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"linebuff\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        while getline(&raw mut linebuff, &raw mut lbsize, fd) != -1 as __ssize_t {
            s = strlen(linebuff) as uint32_t;
            while s > 0 as uint32_t
                && (*linebuff.offset(s.wrapping_sub(1 as uint32_t) as isize) as ::core::ffi::c_int
                    == '\r' as ::core::ffi::c_int
                    || *linebuff.offset(s.wrapping_sub(1 as uint32_t) as isize)
                        as ::core::ffi::c_int
                        == '\n' as ::core::ffi::c_int
                    || *linebuff.offset(s.wrapping_sub(1 as uint32_t) as isize)
                        as ::core::ffi::c_int
                        == '\t' as ::core::ffi::c_int
                    || *linebuff.offset(s.wrapping_sub(1 as uint32_t) as isize)
                        as ::core::ffi::c_int
                        == ' ' as ::core::ffi::c_int)
            {
                s = s.wrapping_sub(1);
            }
            if s > 0 as uint32_t {
                *linebuff.offset(s as isize) = 0 as ::core::ffi::c_char;
                sptr = linebuff;
                while *sptr as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                    || *sptr as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
                {
                    sptr = sptr.offset(1);
                    s = s.wrapping_sub(1);
                }
                if *sptr as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                    && *sptr as ::core::ffi::c_int != '#' as ::core::ffi::c_int
                {
                    md5_update(&raw mut md5c, sptr as *mut uint8_t, s);
                }
            }
        }
        free(linebuff as *mut ::core::ffi::c_void);
    } else {
        fseek(fd, 0 as ::core::ffi::c_long, SEEK_END);
        fsize = ftell(fd) as ::core::ffi::c_ulong;
        fseek(fd, 0 as ::core::ffi::c_long, SEEK_SET);
        if fsize <= 65536 as ::core::ffi::c_ulong {
            binarybuff = malloc(fsize as size_t) as *mut uint8_t;
            if binarybuff.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/cfg.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    322 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"binarybuff\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/cfg.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    322 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"binarybuff\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if binarybuff
                == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint8_t
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/cfg.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    322 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"binarybuff\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/cfg.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    322 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"binarybuff\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
        } else {
            binarybuff = malloc(65536 as ::core::ffi::c_int as size_t) as *mut uint8_t;
            if binarybuff.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/cfg.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    325 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"binarybuff\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/cfg.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    325 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"binarybuff\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if binarybuff
                == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint8_t
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/cfg.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    325 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"binarybuff\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/cfg.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    325 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"binarybuff\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_1,
                );
                abort();
            }
            while fsize > 65536 as ::core::ffi::c_ulong {
                if fread(
                    binarybuff as *mut ::core::ffi::c_void,
                    1 as size_t,
                    65536 as ::core::ffi::c_int as size_t,
                    fd,
                ) != 65536 as ::core::ffi::c_ulong
                {
                    free(binarybuff as *mut ::core::ffi::c_void);
                    fclose(fd);
                    return -1 as ::core::ffi::c_int;
                }
                md5_update(&raw mut md5c, binarybuff, 65536 as uint32_t);
                fsize = fsize.wrapping_sub(65536 as ::core::ffi::c_ulong);
            }
        }
        if fsize > 0 as ::core::ffi::c_ulong {
            if fread(
                binarybuff as *mut ::core::ffi::c_void,
                1 as size_t,
                fsize as size_t,
                fd,
            ) != fsize
            {
                free(binarybuff as *mut ::core::ffi::c_void);
                fclose(fd);
                return -1 as ::core::ffi::c_int;
            }
            md5_update(&raw mut md5c, binarybuff, fsize as uint32_t);
        }
        free(binarybuff as *mut ::core::ffi::c_void);
    }
    fclose(fd);
    md5_final(digest, &raw mut md5c);
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn str_to_int(mut x: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut e: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut r: ::core::ffi::c_int = 0;
    r = strtol(x, &raw mut e, 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if *e as ::core::ffi::c_int != 0
        && *e as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
        && *e as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
    {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            b"config: number expected, got '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            x,
        );
    }
    return r;
}
#[inline]
unsafe extern "C" fn str_to_int32(mut x: *mut ::core::ffi::c_char) -> int32_t {
    let mut e: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut r: int32_t = 0;
    r = strtol(x, &raw mut e, 0 as ::core::ffi::c_int) as int32_t;
    if *e as ::core::ffi::c_int != 0
        && *e as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
        && *e as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
    {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            b"config: number expected, got '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            x,
        );
    }
    return r;
}
#[inline]
unsafe extern "C" fn str_to_uint32(mut x: *mut ::core::ffi::c_char) -> uint32_t {
    let mut e: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut r: uint32_t = 0;
    r = strtoul(x, &raw mut e, 0 as ::core::ffi::c_int) as uint32_t;
    if *e as ::core::ffi::c_int != 0
        && *e as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
        && *e as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
    {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            b"config: number expected, got '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            x,
        );
    }
    return r;
}
#[inline]
unsafe extern "C" fn str_to_int64(mut x: *mut ::core::ffi::c_char) -> int64_t {
    let mut e: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut r: int64_t = 0;
    r = strtoll(x, &raw mut e, 0 as ::core::ffi::c_int) as int64_t;
    if *e as ::core::ffi::c_int != 0
        && *e as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
        && *e as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
    {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            b"config: number expected, got '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            x,
        );
    }
    return r;
}
#[inline]
unsafe extern "C" fn str_to_uint64(mut x: *mut ::core::ffi::c_char) -> uint64_t {
    let mut e: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut r: uint64_t = 0;
    r = strtoull(x, &raw mut e, 0 as ::core::ffi::c_int) as uint64_t;
    if *e as ::core::ffi::c_int != 0
        && *e as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
        && *e as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
    {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            b"config: number expected, got '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            x,
        );
    }
    return r;
}
#[inline]
unsafe extern "C" fn str_to_double(mut x: *mut ::core::ffi::c_char) -> ::core::ffi::c_double {
    let mut e: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut r: ::core::ffi::c_double = 0.;
    r = strtod(x, &raw mut e);
    if *e as ::core::ffi::c_int != 0
        && *e as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
        && *e as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
    {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            b"config: number expected, got '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            x,
        );
    }
    return r;
}
#[inline]
unsafe extern "C" fn str_to_charptr(mut x: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut _cfg_ret_tmp: *mut ::core::ffi::c_char = strdup(x);
    if _cfg_ret_tmp.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/cfg.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            380 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"_cfg_ret_tmp\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/cfg.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            380 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"_cfg_ret_tmp\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if _cfg_ret_tmp
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut ::core::ffi::c_char
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/cfg.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            380 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"_cfg_ret_tmp\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/cfg.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            380 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"_cfg_ret_tmp\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    return _cfg_ret_tmp;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_getstr(
    mut name: *const ::core::ffi::c_char,
    mut def: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut psp: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
    static mut usedvalue: [::core::ffi::c_char; 1000] = [0; 1000];
    psp = cfg_findname(name, 0 as uint8_t);
    if !psp.is_null() {
        cfg_use_option((*psp).name, (*psp).value);
        return str_to_charptr((*psp).value);
    }
    if logundefined != 0 {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_NOTICE,
            b"config: using default value for option '%s' - '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
            def,
        );
    }
    snprintf(
        &raw mut usedvalue as *mut ::core::ffi::c_char,
        1000 as size_t,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        def,
    );
    usedvalue[999 as usize] = 0 as ::core::ffi::c_char;
    cfg_use_option(name, &raw mut usedvalue as *mut ::core::ffi::c_char);
    let mut _cfg_ret_tmp: *mut ::core::ffi::c_char = strdup(def);
    if _cfg_ret_tmp.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/cfg.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            455 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"_cfg_ret_tmp\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/cfg.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            455 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"_cfg_ret_tmp\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if _cfg_ret_tmp
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut ::core::ffi::c_char
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/cfg.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            455 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"_cfg_ret_tmp\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmetalogger/../mfscommon/cfg.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            455 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"_cfg_ret_tmp\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    return _cfg_ret_tmp;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_getnum(
    mut name: *const ::core::ffi::c_char,
    def: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut psp: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
    static mut usedvalue: [::core::ffi::c_char; 1000] = [0; 1000];
    psp = cfg_findname(name, 0 as uint8_t);
    if !psp.is_null() {
        cfg_use_option((*psp).name, (*psp).value);
        return str_to_int((*psp).value);
    }
    if logundefined != 0 {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_NOTICE,
            b"config: using default value for option '%s' - '%d'\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
            def,
        );
    }
    snprintf(
        &raw mut usedvalue as *mut ::core::ffi::c_char,
        1000 as size_t,
        b"%d\0".as_ptr() as *const ::core::ffi::c_char,
        def,
    );
    usedvalue[999 as usize] = 0 as ::core::ffi::c_char;
    cfg_use_option(name, &raw mut usedvalue as *mut ::core::ffi::c_char);
    return def;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_getint8(mut name: *const ::core::ffi::c_char, def: int8_t) -> int8_t {
    let mut psp: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
    static mut usedvalue: [::core::ffi::c_char; 1000] = [0; 1000];
    psp = cfg_findname(name, 0 as uint8_t);
    if !psp.is_null() {
        cfg_use_option((*psp).name, (*psp).value);
        return str_to_int32((*psp).value) as int8_t;
    }
    if logundefined != 0 {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_NOTICE,
            b"config: using default value for option '%s' - '%hhd'\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
            def as ::core::ffi::c_int,
        );
    }
    snprintf(
        &raw mut usedvalue as *mut ::core::ffi::c_char,
        1000 as size_t,
        b"%hhd\0".as_ptr() as *const ::core::ffi::c_char,
        def as ::core::ffi::c_int,
    );
    usedvalue[999 as usize] = 0 as ::core::ffi::c_char;
    cfg_use_option(name, &raw mut usedvalue as *mut ::core::ffi::c_char);
    return def;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_getuint8(
    mut name: *const ::core::ffi::c_char,
    def: uint8_t,
) -> uint8_t {
    let mut psp: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
    static mut usedvalue: [::core::ffi::c_char; 1000] = [0; 1000];
    psp = cfg_findname(name, 0 as uint8_t);
    if !psp.is_null() {
        cfg_use_option((*psp).name, (*psp).value);
        return str_to_uint32((*psp).value) as uint8_t;
    }
    if logundefined != 0 {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_NOTICE,
            b"config: using default value for option '%s' - '%hhu'\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
            def as ::core::ffi::c_int,
        );
    }
    snprintf(
        &raw mut usedvalue as *mut ::core::ffi::c_char,
        1000 as size_t,
        b"%hhu\0".as_ptr() as *const ::core::ffi::c_char,
        def as ::core::ffi::c_int,
    );
    usedvalue[999 as usize] = 0 as ::core::ffi::c_char;
    cfg_use_option(name, &raw mut usedvalue as *mut ::core::ffi::c_char);
    return def;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_getint16(
    mut name: *const ::core::ffi::c_char,
    def: int16_t,
) -> int16_t {
    let mut psp: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
    static mut usedvalue: [::core::ffi::c_char; 1000] = [0; 1000];
    psp = cfg_findname(name, 0 as uint8_t);
    if !psp.is_null() {
        cfg_use_option((*psp).name, (*psp).value);
        return str_to_int32((*psp).value) as int16_t;
    }
    if logundefined != 0 {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_NOTICE,
            b"config: using default value for option '%s' - '%hd'\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
            def as ::core::ffi::c_int,
        );
    }
    snprintf(
        &raw mut usedvalue as *mut ::core::ffi::c_char,
        1000 as size_t,
        b"%hd\0".as_ptr() as *const ::core::ffi::c_char,
        def as ::core::ffi::c_int,
    );
    usedvalue[999 as usize] = 0 as ::core::ffi::c_char;
    cfg_use_option(name, &raw mut usedvalue as *mut ::core::ffi::c_char);
    return def;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_getuint16(
    mut name: *const ::core::ffi::c_char,
    def: uint16_t,
) -> uint16_t {
    let mut psp: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
    static mut usedvalue: [::core::ffi::c_char; 1000] = [0; 1000];
    psp = cfg_findname(name, 0 as uint8_t);
    if !psp.is_null() {
        cfg_use_option((*psp).name, (*psp).value);
        return str_to_uint32((*psp).value) as uint16_t;
    }
    if logundefined != 0 {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_NOTICE,
            b"config: using default value for option '%s' - '%hu'\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
            def as ::core::ffi::c_int,
        );
    }
    snprintf(
        &raw mut usedvalue as *mut ::core::ffi::c_char,
        1000 as size_t,
        b"%hu\0".as_ptr() as *const ::core::ffi::c_char,
        def as ::core::ffi::c_int,
    );
    usedvalue[999 as usize] = 0 as ::core::ffi::c_char;
    cfg_use_option(name, &raw mut usedvalue as *mut ::core::ffi::c_char);
    return def;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_getint32(
    mut name: *const ::core::ffi::c_char,
    def: int32_t,
) -> int32_t {
    let mut psp: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
    static mut usedvalue: [::core::ffi::c_char; 1000] = [0; 1000];
    psp = cfg_findname(name, 0 as uint8_t);
    if !psp.is_null() {
        cfg_use_option((*psp).name, (*psp).value);
        return str_to_int32((*psp).value);
    }
    if logundefined != 0 {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_NOTICE,
            b"config: using default value for option '%s' - '%d'\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
            def,
        );
    }
    snprintf(
        &raw mut usedvalue as *mut ::core::ffi::c_char,
        1000 as size_t,
        b"%d\0".as_ptr() as *const ::core::ffi::c_char,
        def,
    );
    usedvalue[999 as usize] = 0 as ::core::ffi::c_char;
    cfg_use_option(name, &raw mut usedvalue as *mut ::core::ffi::c_char);
    return def;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_getuint32(
    mut name: *const ::core::ffi::c_char,
    def: uint32_t,
) -> uint32_t {
    let mut psp: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
    static mut usedvalue: [::core::ffi::c_char; 1000] = [0; 1000];
    psp = cfg_findname(name, 0 as uint8_t);
    if !psp.is_null() {
        cfg_use_option((*psp).name, (*psp).value);
        return str_to_uint32((*psp).value);
    }
    if logundefined != 0 {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_NOTICE,
            b"config: using default value for option '%s' - '%u'\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
            def,
        );
    }
    snprintf(
        &raw mut usedvalue as *mut ::core::ffi::c_char,
        1000 as size_t,
        b"%u\0".as_ptr() as *const ::core::ffi::c_char,
        def,
    );
    usedvalue[999 as usize] = 0 as ::core::ffi::c_char;
    cfg_use_option(name, &raw mut usedvalue as *mut ::core::ffi::c_char);
    return def;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_getint64(
    mut name: *const ::core::ffi::c_char,
    def: int64_t,
) -> int64_t {
    let mut psp: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
    static mut usedvalue: [::core::ffi::c_char; 1000] = [0; 1000];
    psp = cfg_findname(name, 0 as uint8_t);
    if !psp.is_null() {
        cfg_use_option((*psp).name, (*psp).value);
        return str_to_int64((*psp).value);
    }
    if logundefined != 0 {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_NOTICE,
            b"config: using default value for option '%s' - '%ld'\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
            def,
        );
    }
    snprintf(
        &raw mut usedvalue as *mut ::core::ffi::c_char,
        1000 as size_t,
        b"%ld\0".as_ptr() as *const ::core::ffi::c_char,
        def,
    );
    usedvalue[999 as usize] = 0 as ::core::ffi::c_char;
    cfg_use_option(name, &raw mut usedvalue as *mut ::core::ffi::c_char);
    return def;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_getuint64(
    mut name: *const ::core::ffi::c_char,
    def: uint64_t,
) -> uint64_t {
    let mut psp: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
    static mut usedvalue: [::core::ffi::c_char; 1000] = [0; 1000];
    psp = cfg_findname(name, 0 as uint8_t);
    if !psp.is_null() {
        cfg_use_option((*psp).name, (*psp).value);
        return str_to_uint64((*psp).value);
    }
    if logundefined != 0 {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_NOTICE,
            b"config: using default value for option '%s' - '%lu'\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
            def,
        );
    }
    snprintf(
        &raw mut usedvalue as *mut ::core::ffi::c_char,
        1000 as size_t,
        b"%lu\0".as_ptr() as *const ::core::ffi::c_char,
        def,
    );
    usedvalue[999 as usize] = 0 as ::core::ffi::c_char;
    cfg_use_option(name, &raw mut usedvalue as *mut ::core::ffi::c_char);
    return def;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_getdouble(
    mut name: *const ::core::ffi::c_char,
    def: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut psp: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
    static mut usedvalue: [::core::ffi::c_char; 1000] = [0; 1000];
    psp = cfg_findname(name, 0 as uint8_t);
    if !psp.is_null() {
        cfg_use_option((*psp).name, (*psp).value);
        return str_to_double((*psp).value);
    }
    if logundefined != 0 {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_NOTICE,
            b"config: using default value for option '%s' - '%.6lf'\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
            def,
        );
    }
    snprintf(
        &raw mut usedvalue as *mut ::core::ffi::c_char,
        1000 as size_t,
        b"%.6lf\0".as_ptr() as *const ::core::ffi::c_char,
        def,
    );
    usedvalue[999 as usize] = 0 as ::core::ffi::c_char;
    cfg_use_option(name, &raw mut usedvalue as *mut ::core::ffi::c_char);
    return def;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_getsperiod(
    mut name: *const ::core::ffi::c_char,
    mut def: *const ::core::ffi::c_char,
) -> uint32_t {
    let mut psp: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
    static mut usedvalue: [::core::ffi::c_char; 1000] = [0; 1000];
    let mut ret: uint32_t = 0;
    psp = cfg_findname(name, 0 as uint8_t);
    if !psp.is_null() {
        match parse_speriod((*psp).value, &raw mut ret) {
            TPARSE_OK => {
                cfg_use_option((*psp).name, (*psp).value);
                return ret;
            }
            TPARSE_UNEXPECTED_CHAR => {
                if ret != 0 {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"config: unexpected char '%c' in '%s = %s' - using defaults\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ret as ::core::ffi::c_char as ::core::ffi::c_int,
                        (*psp).name,
                        (*psp).value,
                    );
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"config: unexpected end in '%s = %s' - using defaults\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*psp).name,
                        (*psp).value,
                    );
                }
            }
            TPARSE_VALUE_TOO_BIG => {
                if ret != 0 {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"config: value too big in section '%c' in '%s = %s' - using defaults\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        ret as ::core::ffi::c_char as ::core::ffi::c_int,
                        (*psp).name,
                        (*psp).value,
                    );
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"config: parsed value too big in '%s = %s' - using defaults\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*psp).name,
                        (*psp).value,
                    );
                }
            }
            _ => {}
        }
    }
    if logundefined != 0 {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_NOTICE,
            b"config: using default value for option '%s' - '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
            def,
        );
    }
    snprintf(
        &raw mut usedvalue as *mut ::core::ffi::c_char,
        1000 as size_t,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        def,
    );
    usedvalue[999 as usize] = 0 as ::core::ffi::c_char;
    cfg_use_option(name, &raw mut usedvalue as *mut ::core::ffi::c_char);
    if parse_speriod(def, &raw mut ret) != TPARSE_OK {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            b"config: wrong default value for option '%s' - '%s' !!!\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
            def,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn cfg_gethperiod(
    mut name: *const ::core::ffi::c_char,
    mut def: *const ::core::ffi::c_char,
) -> uint16_t {
    let mut psp: *mut paramstr = ::core::ptr::null_mut::<paramstr>();
    static mut usedvalue: [::core::ffi::c_char; 1000] = [0; 1000];
    let mut ret: uint32_t = 0;
    psp = cfg_findname(name, 0 as uint8_t);
    if !psp.is_null() {
        match parse_hperiod((*psp).value, &raw mut ret) {
            TPARSE_OK => {
                cfg_use_option((*psp).name, (*psp).value);
                return ret as uint16_t;
            }
            TPARSE_UNEXPECTED_CHAR => {
                if ret != 0 {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"config: unexpected char '%c' in '%s = %s' - using defaults\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ret as ::core::ffi::c_char as ::core::ffi::c_int,
                        (*psp).name,
                        (*psp).value,
                    );
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"config: unexpected end in '%s = %s' - using defaults\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*psp).name,
                        (*psp).value,
                    );
                }
            }
            TPARSE_VALUE_TOO_BIG => {
                if ret != 0 {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"config: value too big in section '%c' in '%s = %s' - using defaults\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        ret as ::core::ffi::c_char as ::core::ffi::c_int,
                        (*psp).name,
                        (*psp).value,
                    );
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"config: parsed value too big in '%s = %s' - using defaults\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*psp).name,
                        (*psp).value,
                    );
                }
            }
            _ => {}
        }
    }
    if logundefined != 0 {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_NOTICE,
            b"config: using default value for option '%s' - '%s'\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
            def,
        );
    }
    snprintf(
        &raw mut usedvalue as *mut ::core::ffi::c_char,
        1000 as size_t,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        def,
    );
    usedvalue[999 as usize] = 0 as ::core::ffi::c_char;
    cfg_use_option(name, &raw mut usedvalue as *mut ::core::ffi::c_char);
    if parse_hperiod(def, &raw mut ret) != TPARSE_OK {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            b"config: wrong default value for option '%s' - '%s' !!!\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
            def,
        );
    }
    return ret as uint16_t;
}
