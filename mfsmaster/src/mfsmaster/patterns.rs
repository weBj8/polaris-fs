use ::c2rust_bitfields;
extern "C" {
    pub type _bio;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn glob_new(globstr: *const uint8_t) -> *mut ::core::ffi::c_void;
    fn glob_free(glob: *mut ::core::ffi::c_void);
    fn glob_match(glob: *mut ::core::ffi::c_void, name: *const uint8_t, nleng: uint8_t) -> uint8_t;
    fn bio_read(b: *mut bio, dst: *mut ::core::ffi::c_void, len: uint64_t) -> int64_t;
    fn bio_write(b: *mut bio, src: *const ::core::ffi::c_void, len: uint64_t) -> int64_t;
    fn bio_skip(b: *mut bio, len: uint64_t);
    fn sclass_find_by_name(nleng: uint8_t, name: *const uint8_t) -> uint8_t;
    fn sclass_get_nleng(sclassid: uint8_t) -> uint8_t;
    fn sclass_get_name(sclassid: uint8_t) -> *const uint8_t;
    static mut stderr: *mut FILE;
    fn fputc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn meta_version_inc() -> uint64_t;
    fn changelog(format: *const ::core::ffi::c_char, ...);
    fn changelog_escape_name(nleng: uint32_t, name: *const uint8_t) -> *mut ::core::ffi::c_char;
    fn main_time() -> uint32_t;
    fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type bio = _bio;
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
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _pattern {
    pub glob: *mut ::core::ffi::c_void,
    pub valid: uint8_t,
    pub modified: uint8_t,
    pub gnleng: uint8_t,
    pub gname: [uint8_t; 256],
    pub euid: uint32_t,
    pub egid: uint32_t,
    pub priority: uint8_t,
    pub omask: uint8_t,
    pub scid: uint8_t,
    pub seteattr: uint8_t,
    pub clreattr: uint8_t,
    pub trashretention: uint16_t,
}
pub type pattern = _pattern;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_EINVAL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MFS_ERROR_NOSUCHCLASS: ::core::ffi::c_int = 52 as ::core::ffi::c_int;
pub const MFS_ERROR_PATTERNEXISTS: ::core::ffi::c_int = 55 as ::core::ffi::c_int;
pub const MFS_ERROR_PATLIMITREACHED: ::core::ffi::c_int = 56 as ::core::ffi::c_int;
pub const MFS_ERROR_NOSUCHPATTERN: ::core::ffi::c_int = 57 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PATTERN_OMASK_SCLASS: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PATTERN_OMASK_EATTR: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_ERRNO_SYSLOG_STDERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const PATTERNS_MAX: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
static mut patterntab: [pattern; 1024] = [pattern {
    glob: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    valid: 0,
    modified: 0,
    gnleng: 0,
    gname: [0; 256],
    euid: 0,
    egid: 0,
    priority: 0,
    omask: 0,
    scid: 0,
    seteattr: 0,
    clreattr: 0,
    trashretention: 0,
}; 1024];
static mut validpatterns: uint32_t = 0;
#[inline]
unsafe extern "C" fn pattern_check_ugids(
    mut p: *mut pattern,
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut i: uint32_t = 0;
    if (*p).euid != 0xffffffff as uint32_t && (*p).euid != uid {
        return 0 as ::core::ffi::c_int;
    }
    if (*p).egid == 0xffffffff as uint32_t {
        return 1 as ::core::ffi::c_int;
    }
    i = 0 as uint32_t;
    while i < gids {
        if (*p).egid == *gid.offset(i as isize) {
            return 1 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn patterns_compare(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut aa: *const pattern = a as *const pattern;
    let mut bb: *const pattern = b as *const pattern;
    if ((*aa).valid as ::core::ffi::c_int) < (*bb).valid as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    } else if (*aa).valid as ::core::ffi::c_int > (*bb).valid as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    } else if ((*aa).priority as ::core::ffi::c_int) < (*bb).priority as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    } else if (*aa).priority as ::core::ffi::c_int > (*bb).priority as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    } else if ((*aa).scid as ::core::ffi::c_int) < (*bb).scid as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    } else if (*aa).scid as ::core::ffi::c_int > (*bb).scid as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    } else {
        return strcmp(
            &raw const (*aa).gname as *const uint8_t as *const ::core::ffi::c_char,
            &raw const (*bb).gname as *const uint8_t as *const ::core::ffi::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn patterns_have_changed() {
    let mut i: uint32_t = 0;
    i = 0 as uint32_t;
    while i < PATTERNS_MAX as uint32_t {
        if patterntab[i as usize].modified != 0 {
            if !patterntab[i as usize].glob.is_null() {
                glob_free(patterntab[i as usize].glob);
            }
            if patterntab[i as usize].valid != 0 {
                patterntab[i as usize].glob = glob_new(
                    &raw mut (*(&raw mut patterntab as *mut pattern).offset(i as isize)).gname
                        as *mut uint8_t,
                );
            } else {
                patterntab[i as usize].glob = NULL;
            }
            patterntab[i as usize].modified = 0 as uint8_t;
        }
        i = i.wrapping_add(1);
    }
    qsort(
        &raw mut patterntab as *mut pattern as *mut ::core::ffi::c_void,
        PATTERNS_MAX as size_t,
        ::core::mem::size_of::<pattern>(),
        Some(
            patterns_compare
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    i = 0 as uint32_t;
    while i < PATTERNS_MAX as uint32_t {
        if patterntab[i as usize].valid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            validpatterns = i;
            return;
        }
        i = i.wrapping_add(1);
    }
    validpatterns = PATTERNS_MAX as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn patterns_find_matching(
    mut uid: uint32_t,
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
    mut nleng: uint8_t,
    mut name: *const uint8_t,
    mut scid: *mut uint8_t,
    mut trashretention: *mut uint16_t,
    mut seteattr: *mut uint8_t,
    mut clreattr: *mut uint8_t,
) -> uint8_t {
    let mut i: uint32_t = 0;
    i = 0 as uint32_t;
    while i < validpatterns {
        if pattern_check_ugids(
            (&raw mut patterntab as *mut pattern).offset(i as isize),
            uid,
            gids,
            gid,
        ) != 0
            && glob_match(patterntab[i as usize].glob, name as *const uint8_t, nleng)
                as ::core::ffi::c_int
                != 0
        {
            *scid = patterntab[i as usize].scid;
            *trashretention = patterntab[i as usize].trashretention;
            *seteattr = patterntab[i as usize].seteattr;
            *clreattr = patterntab[i as usize].clreattr;
            return patterntab[i as usize].omask;
        }
        i = i.wrapping_add(1);
    }
    return 0 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn patterns_sclass_delete(mut scid: uint8_t) {
    let mut i: uint32_t = 0;
    i = 0 as uint32_t;
    while i < PATTERNS_MAX as uint32_t {
        if patterntab[i as usize].valid as ::core::ffi::c_int != 0
            && scid as ::core::ffi::c_int == patterntab[i as usize].scid as ::core::ffi::c_int
        {
            patterntab[i as usize].valid = 0 as uint8_t;
            patterntab[i as usize].modified = 1 as uint8_t;
        }
        i = i.wrapping_add(1);
    }
    patterns_have_changed();
}
#[inline]
unsafe extern "C" fn patterns_univ_add(
    mut gnleng: uint8_t,
    mut gname: *const uint8_t,
    mut euid: uint32_t,
    mut egid: uint32_t,
    mut priority: uint8_t,
    mut omask: uint8_t,
    mut scid: uint8_t,
    mut trashretention: uint16_t,
    mut seteattr: uint8_t,
    mut clreattr: uint8_t,
    mut mrflag: uint8_t,
) -> uint8_t {
    let mut i: uint32_t = 0;
    let mut npos: uint32_t = 0;
    if gnleng as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return MFS_ERROR_EINVAL as uint8_t;
    }
    if omask as ::core::ffi::c_int & PATTERN_OMASK_EATTR != 0
        && (seteattr as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && clreattr as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || seteattr as ::core::ffi::c_int & clreattr as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int)
    {
        return MFS_ERROR_EINVAL as uint8_t;
    }
    npos = PATTERNS_MAX as uint32_t;
    i = 0 as uint32_t;
    while i < PATTERNS_MAX as uint32_t {
        if patterntab[i as usize].valid != 0 {
            if gnleng as ::core::ffi::c_int == patterntab[i as usize].gnleng as ::core::ffi::c_int
                && euid == patterntab[i as usize].euid
                && egid == patterntab[i as usize].egid
                && memcmp(
                    gname as *const ::core::ffi::c_void,
                    &raw mut (*(&raw mut patterntab as *mut pattern).offset(i as isize)).gname
                        as *mut uint8_t as *const ::core::ffi::c_void,
                    gnleng as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                return MFS_ERROR_PATTERNEXISTS as uint8_t;
            }
        } else if npos == PATTERNS_MAX as uint32_t {
            npos = i;
        }
        i = i.wrapping_add(1);
    }
    if npos == PATTERNS_MAX as uint32_t {
        return MFS_ERROR_PATLIMITREACHED as uint8_t;
    }
    patterntab[npos as usize].valid = 1 as uint8_t;
    patterntab[npos as usize].modified = 1 as uint8_t;
    patterntab[npos as usize].gnleng = gnleng;
    memcpy(
        &raw mut (*(&raw mut patterntab as *mut pattern).offset(npos as isize)).gname
            as *mut uint8_t as *mut ::core::ffi::c_void,
        gname as *const ::core::ffi::c_void,
        gnleng as size_t,
    );
    patterntab[npos as usize].gname[gnleng as usize] = 0 as uint8_t;
    patterntab[npos as usize].euid = euid;
    patterntab[npos as usize].egid = egid;
    patterntab[npos as usize].priority = priority;
    patterntab[npos as usize].omask = omask;
    patterntab[npos as usize].scid = scid;
    patterntab[npos as usize].trashretention = trashretention;
    patterntab[npos as usize].seteattr = seteattr;
    patterntab[npos as usize].clreattr = clreattr;
    patterns_have_changed();
    if mrflag as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        changelog(
            b"%u|PATADD(%s,%u,%u,%hhu,%hhu,%hhu,%hu,%hhu,%hhu)\0".as_ptr()
                as *const ::core::ffi::c_char,
            main_time(),
            changelog_escape_name(gnleng as uint32_t, gname),
            euid,
            egid,
            priority as ::core::ffi::c_int,
            omask as ::core::ffi::c_int,
            scid as ::core::ffi::c_int,
            trashretention as ::core::ffi::c_int,
            seteattr as ::core::ffi::c_int,
            clreattr as ::core::ffi::c_int,
        );
    } else {
        meta_version_inc();
    }
    return MFS_STATUS_OK as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn patterns_add(
    mut gnleng: uint8_t,
    mut gname: *const uint8_t,
    mut euid: uint32_t,
    mut egid: uint32_t,
    mut priority: uint8_t,
    mut omask: uint8_t,
    mut scnleng: uint8_t,
    mut scname: *const uint8_t,
    mut trashretention: uint16_t,
    mut seteattr: uint8_t,
    mut clreattr: uint8_t,
) -> uint8_t {
    let mut scid: uint8_t = 0;
    if omask as ::core::ffi::c_int & PATTERN_OMASK_SCLASS != 0 {
        scid = sclass_find_by_name(scnleng, scname);
        if scid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return MFS_ERROR_NOSUCHCLASS as uint8_t;
        }
    } else {
        scid = 0 as uint8_t;
    }
    return patterns_univ_add(
        gnleng,
        gname,
        euid,
        egid,
        priority,
        omask,
        scid,
        trashretention,
        seteattr,
        clreattr,
        0 as uint8_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn patterns_mr_add(
    mut gnleng: uint8_t,
    mut gname: *const uint8_t,
    mut euid: uint32_t,
    mut egid: uint32_t,
    mut priority: uint8_t,
    mut omask: uint8_t,
    mut scid: uint8_t,
    mut trashretention: uint16_t,
    mut seteattr: uint8_t,
    mut clreattr: uint8_t,
) -> uint8_t {
    return patterns_univ_add(
        gnleng,
        gname,
        euid,
        egid,
        priority,
        omask,
        scid,
        trashretention,
        seteattr,
        clreattr,
        1 as uint8_t,
    );
}
#[inline]
unsafe extern "C" fn patterns_univ_delete(
    mut gnleng: uint8_t,
    mut gname: *const uint8_t,
    mut euid: uint32_t,
    mut egid: uint32_t,
    mut mrflag: uint8_t,
) -> uint8_t {
    let mut i: uint32_t = 0;
    let mut f: uint8_t = 0;
    f = 0 as uint8_t;
    i = 0 as uint32_t;
    while i < PATTERNS_MAX as uint32_t {
        if patterntab[i as usize].valid as ::core::ffi::c_int != 0
            && gnleng as ::core::ffi::c_int == patterntab[i as usize].gnleng as ::core::ffi::c_int
            && euid == patterntab[i as usize].euid
            && egid == patterntab[i as usize].egid
            && memcmp(
                gname as *const ::core::ffi::c_void,
                &raw mut (*(&raw mut patterntab as *mut pattern).offset(i as isize)).gname
                    as *mut uint8_t as *const ::core::ffi::c_void,
                gnleng as size_t,
            ) == 0 as ::core::ffi::c_int
        {
            patterntab[i as usize].valid = 0 as uint8_t;
            patterntab[i as usize].modified = 1 as uint8_t;
            f = 1 as uint8_t;
        }
        i = i.wrapping_add(1);
    }
    if f as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return MFS_ERROR_NOSUCHPATTERN as uint8_t;
    }
    patterns_have_changed();
    if mrflag as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        changelog(
            b"%u|PATDEL(%s,%u,%u)\0".as_ptr() as *const ::core::ffi::c_char,
            main_time(),
            changelog_escape_name(gnleng as uint32_t, gname),
            euid,
            egid,
        );
    } else {
        meta_version_inc();
    }
    return MFS_STATUS_OK as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn patterns_delete(
    mut gnleng: uint8_t,
    mut gname: *const uint8_t,
    mut euid: uint32_t,
    mut egid: uint32_t,
) -> uint8_t {
    return patterns_univ_delete(gnleng, gname, euid, egid, 0 as uint8_t);
}
#[no_mangle]
pub unsafe extern "C" fn patterns_mr_delete(
    mut gnleng: uint8_t,
    mut gname: *const uint8_t,
    mut euid: uint32_t,
    mut egid: uint32_t,
) -> uint8_t {
    return patterns_univ_delete(gnleng, gname, euid, egid, 1 as uint8_t);
}
#[no_mangle]
pub unsafe extern "C" fn patterns_list(mut buff: *mut uint8_t) -> uint32_t {
    let mut i: uint32_t = 0;
    let mut lsize: uint32_t = 0;
    lsize = 0 as uint32_t;
    i = 0 as uint32_t;
    while i < PATTERNS_MAX as uint32_t {
        if patterntab[i as usize].valid != 0 {
            if buff.is_null() {
                if patterntab[i as usize].omask as ::core::ffi::c_int & PATTERN_OMASK_SCLASS != 0 {
                    lsize = lsize.wrapping_add(
                        (patterntab[i as usize].gnleng as ::core::ffi::c_int
                            + sclass_get_nleng(patterntab[i as usize].scid) as ::core::ffi::c_int
                            + 16 as ::core::ffi::c_int) as uint32_t,
                    );
                } else {
                    lsize = lsize.wrapping_add(
                        (patterntab[i as usize].gnleng as ::core::ffi::c_int
                            + 16 as ::core::ffi::c_int) as uint32_t,
                    );
                }
            } else {
                put8bit(&raw mut buff, patterntab[i as usize].gnleng);
                memcpy(
                    buff as *mut ::core::ffi::c_void,
                    &raw mut (*(&raw mut patterntab as *mut pattern).offset(i as isize)).gname
                        as *mut uint8_t as *const ::core::ffi::c_void,
                    patterntab[i as usize].gnleng as size_t,
                );
                buff = buff.offset(patterntab[i as usize].gnleng as ::core::ffi::c_int as isize);
                put32bit(&raw mut buff, patterntab[i as usize].euid);
                put32bit(&raw mut buff, patterntab[i as usize].egid);
                put8bit(&raw mut buff, patterntab[i as usize].priority);
                put8bit(&raw mut buff, patterntab[i as usize].omask);
                if patterntab[i as usize].omask as ::core::ffi::c_int & PATTERN_OMASK_SCLASS != 0 {
                    let mut scnleng: uint8_t = 0;
                    scnleng = sclass_get_nleng(patterntab[i as usize].scid);
                    put8bit(&raw mut buff, scnleng);
                    memcpy(
                        buff as *mut ::core::ffi::c_void,
                        sclass_get_name(patterntab[i as usize].scid) as *const ::core::ffi::c_void,
                        scnleng as size_t,
                    );
                    buff = buff.offset(scnleng as ::core::ffi::c_int as isize);
                    lsize = lsize.wrapping_add(
                        (patterntab[i as usize].gnleng as ::core::ffi::c_int
                            + scnleng as ::core::ffi::c_int
                            + 16 as ::core::ffi::c_int) as uint32_t,
                    );
                } else {
                    put8bit(&raw mut buff, 0 as uint8_t);
                    lsize = lsize.wrapping_add(
                        (patterntab[i as usize].gnleng as ::core::ffi::c_int
                            + 16 as ::core::ffi::c_int) as uint32_t,
                    );
                }
                put16bit(&raw mut buff, patterntab[i as usize].trashretention);
                put8bit(&raw mut buff, patterntab[i as usize].seteattr);
                put8bit(&raw mut buff, patterntab[i as usize].clreattr);
            }
        }
        i = i.wrapping_add(1);
    }
    return lsize;
}
#[no_mangle]
pub unsafe extern "C" fn patterns_store(mut fd: *mut bio) -> uint8_t {
    let mut databuff: [uint8_t; 270] = [0; 270];
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut gnleng: uint8_t = 0;
    let mut wsize: int32_t = 0;
    let mut i: uint32_t = 0;
    if fd.is_null() {
        return 0x11 as uint8_t;
    }
    i = 0 as uint32_t;
    while i < PATTERNS_MAX as uint32_t {
        if patterntab[i as usize].valid != 0 {
            ptr = &raw mut databuff as *mut uint8_t;
            gnleng = patterntab[i as usize].gnleng;
            put8bit(&raw mut ptr, gnleng);
            put32bit(&raw mut ptr, patterntab[i as usize].euid);
            put32bit(&raw mut ptr, patterntab[i as usize].egid);
            put8bit(&raw mut ptr, patterntab[i as usize].priority);
            put8bit(&raw mut ptr, patterntab[i as usize].omask);
            put8bit(&raw mut ptr, patterntab[i as usize].scid);
            put16bit(&raw mut ptr, patterntab[i as usize].trashretention);
            put8bit(&raw mut ptr, patterntab[i as usize].seteattr);
            put8bit(&raw mut ptr, patterntab[i as usize].clreattr);
            memcpy(
                ptr as *mut ::core::ffi::c_void,
                &raw mut (*(&raw mut patterntab as *mut pattern).offset(i as isize)).gname
                    as *mut uint8_t as *const ::core::ffi::c_void,
                gnleng as size_t,
            );
            ptr = ptr.offset(gnleng as ::core::ffi::c_int as isize);
            wsize = ptr.offset_from(&raw mut databuff as *mut uint8_t) as int32_t;
            if bio_write(
                fd,
                &raw mut databuff as *mut uint8_t as *const ::core::ffi::c_void,
                wsize as uint64_t,
            ) != wsize as int64_t
            {
                return 0xff as uint8_t;
            }
        }
        i = i.wrapping_add(1);
    }
    ptr = &raw mut databuff as *mut uint8_t;
    put8bit(&raw mut ptr, 0 as uint8_t);
    put32bit(&raw mut ptr, 0xffffffff as uint32_t);
    put32bit(&raw mut ptr, 0xffffffff as uint32_t);
    wsize = ptr.offset_from(&raw mut databuff as *mut uint8_t) as int32_t;
    if bio_write(
        fd,
        &raw mut databuff as *mut uint8_t as *const ::core::ffi::c_void,
        wsize as uint64_t,
    ) != wsize as int64_t
    {
        return 0xff as uint8_t;
    }
    return 0 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn patterns_cleanup() {
    let mut i: uint32_t = 0;
    i = 0 as uint32_t;
    while i < PATTERNS_MAX as uint32_t {
        patterntab[i as usize].valid = 0 as uint8_t;
        patterntab[i as usize].modified = 1 as uint8_t;
        i = i.wrapping_add(1);
    }
    patterns_have_changed();
}
#[no_mangle]
pub unsafe extern "C" fn patterns_load(
    mut fd: *mut bio,
    mut mver: uint8_t,
    mut ignoreflag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut databuff: [uint8_t; 262] = [0; 262];
    let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut i: uint32_t = 0;
    let mut gnleng: uint8_t = 0;
    let mut euid: uint32_t = 0;
    let mut egid: uint32_t = 0;
    let mut psize: int32_t = 0;
    if mver as ::core::ffi::c_int > 0x11 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"loading patterns data: unsupported format\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    patterns_cleanup();
    i = 0 as uint32_t;
    psize = (if mver as ::core::ffi::c_int == 0x10 as ::core::ffi::c_int {
        6 as ::core::ffi::c_int
    } else {
        7 as ::core::ffi::c_int
    }) as int32_t;
    loop {
        if i >= PATTERNS_MAX as uint32_t {
            if ignoreflag != 0 {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_NOTICE,
                    b"loading patterns data: too many patterns - ignore\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            } else {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"loading patterns data: too many patterns\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
        }
        if bio_read(
            fd,
            &raw mut databuff as *mut uint8_t as *mut ::core::ffi::c_void,
            (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                as uint64_t,
        ) != (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            as int64_t
        {
            let mut err: ::core::ffi::c_int = *__errno_location();
            fputc('\n' as ::core::ffi::c_int, stderr);
            *__errno_location() = err;
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"loading patterns data: read error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = &raw mut databuff as *mut uint8_t;
        gnleng = get8bit(&raw mut ptr);
        euid = get32bit(&raw mut ptr);
        egid = get32bit(&raw mut ptr);
        if gnleng as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && euid == 0xffffffff as uint32_t
            && egid == 0xffffffff as uint32_t
        {
            break;
        }
        if i < PATTERNS_MAX as uint32_t {
            if bio_read(
                fd,
                &raw mut databuff as *mut uint8_t as *mut ::core::ffi::c_void,
                (psize + gnleng as int32_t) as uint64_t,
            ) != (psize + gnleng as int32_t) as int64_t
            {
                let mut err_0: ::core::ffi::c_int = *__errno_location();
                fputc('\n' as ::core::ffi::c_int, stderr);
                *__errno_location() = err_0;
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"loading patterns data: read error\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = &raw mut databuff as *mut uint8_t;
            patterntab[i as usize].gnleng = gnleng;
            patterntab[i as usize].euid = euid;
            patterntab[i as usize].egid = egid;
            patterntab[i as usize].priority = get8bit(&raw mut ptr);
            patterntab[i as usize].omask = get8bit(&raw mut ptr);
            patterntab[i as usize].scid = get8bit(&raw mut ptr);
            patterntab[i as usize].trashretention = get16bit(&raw mut ptr);
            patterntab[i as usize].seteattr = get8bit(&raw mut ptr);
            if mver as ::core::ffi::c_int == 0x10 as ::core::ffi::c_int {
                patterntab[i as usize].clreattr = 0 as uint8_t;
            } else {
                patterntab[i as usize].clreattr = get8bit(&raw mut ptr);
            }
            memcpy(
                &raw mut (*(&raw mut patterntab as *mut pattern).offset(i as isize)).gname
                    as *mut uint8_t as *mut ::core::ffi::c_void,
                ptr as *const ::core::ffi::c_void,
                gnleng as size_t,
            );
            patterntab[i as usize].gname[gnleng as usize] = 0 as uint8_t;
            patterntab[i as usize].valid = 1 as uint8_t;
            patterntab[i as usize].modified = 1 as uint8_t;
        } else {
            bio_skip(fd, (psize + gnleng as int32_t) as uint32_t as uint64_t);
        }
        i = i.wrapping_add(1);
    }
    patterns_have_changed();
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn patterns_init() -> ::core::ffi::c_int {
    let mut i: uint32_t = 0;
    i = 0 as uint32_t;
    while i < PATTERNS_MAX as uint32_t {
        patterntab[i as usize].glob = NULL;
        patterntab[i as usize].valid = 0 as uint8_t;
        patterntab[i as usize].modified = 0 as uint8_t;
        i = i.wrapping_add(1);
    }
    validpatterns = 0 as uint32_t;
    return 0 as ::core::ffi::c_int;
}
