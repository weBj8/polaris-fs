use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _bio;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn feof(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
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
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn univmakestrip(strip: *mut ::core::ffi::c_char, ip: uint32_t);
    fn bio_read(b: *mut bio, dst: *mut ::core::ffi::c_void, len: uint64_t) -> int64_t;
    fn bio_write(b: *mut bio, src: *const ::core::ffi::c_void, len: uint64_t) -> int64_t;
    fn sclass_get_export_group(sclassid: uint16_t) -> uint8_t;
    fn fs_getdirpath_size(inode: uint32_t) -> uint32_t;
    fn fs_getdirpath_data(inode: uint32_t, buff: *mut uint8_t, size: uint32_t);
    fn of_session_removed(sessionid: uint32_t);
    fn of_noofopenedfiles(sessionid: uint32_t) -> uint32_t;
    fn changelog(format: *const ::core::ffi::c_char, ...);
    fn changelog_escape_name(nleng: uint32_t, name: *const uint8_t) -> *mut ::core::ffi::c_char;
    fn meta_version_inc() -> uint64_t;
    fn cfg_getsperiod(
        name: *const ::core::ffi::c_char,
        def: *const ::core::ffi::c_char,
    ) -> uint32_t;
    fn main_reload_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    fn main_info_register_fname(
        fun: Option<unsafe extern "C" fn(*mut FILE) -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    fn main_time_register_fname(
        seconds: uint32_t,
        offset: uint32_t,
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    fn main_time() -> uint32_t;
    fn main_start_time() -> uint32_t;
    fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
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
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type bio = _bio;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct session {
    pub sessionid: uint32_t,
    pub exportscsum: uint64_t,
    pub info: *mut uint8_t,
    pub ileng: uint32_t,
    pub peerip: uint32_t,
    pub closed: uint8_t,
    pub sesflags: uint8_t,
    pub umaskval: uint16_t,
    pub sclassgroups: uint16_t,
    pub mintrashretention: uint32_t,
    pub maxtrashretention: uint32_t,
    pub rootuid: uint32_t,
    pub rootgid: uint32_t,
    pub mapalluid: uint32_t,
    pub mapallgid: uint32_t,
    pub disables: uint32_t,
    pub rootinode: uint32_t,
    pub disconnected: uint32_t,
    pub nsocks: uint32_t,
    pub infopeerip: uint32_t,
    pub infoversion: uint32_t,
    pub chouropstats: [uint32_t; 28],
    pub lhouropstats: [uint32_t; 28],
    pub cminopstats: [uint32_t; 28],
    pub lminopstats: [uint32_t; 28],
    pub next: *mut session,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_ERROR_MISMATCH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MFS_ERROR_BADSESSIONID: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
pub const MFS_ERROR_NOTFOUND: ::core::ffi::c_int = 41 as ::core::ffi::c_int;
pub const MFS_ERROR_ACTIVE: ::core::ffi::c_int = 42 as ::core::ffi::c_int;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SMODE_SET: ::core::ffi::c_int = 0;
pub const SMODE_INCREASE: ::core::ffi::c_int = 1;
pub const SMODE_DECREASE: ::core::ffi::c_int = 2;
pub const SMODE_EXCHANGE: ::core::ffi::c_int = 3;
pub const SESFLAG_MAPALL: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const SESFLAG_ATTRBIT: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SESFLAG_METARESTORE: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const SESSION_STATS: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn put64bit(mut ptr: *mut *mut uint8_t, mut val: uint64_t) {
    val = val.swap_bytes() as uint64_t;
    memcpy(
        *ptr as *mut ::core::ffi::c_void,
        &raw mut val as *const ::core::ffi::c_void,
        8 as size_t,
    );
    *ptr = (*ptr).offset(8 as ::core::ffi::c_int as isize);
}
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
unsafe extern "C" fn get64bit(mut ptr: *mut *const uint8_t) -> uint64_t {
    let mut t64: uint64_t = 0;
    memcpy(
        &raw mut t64 as *mut ::core::ffi::c_void,
        *ptr as *const ::core::ffi::c_void,
        8 as size_t,
    );
    *ptr = (*ptr).offset(8 as ::core::ffi::c_int as isize);
    return t64.swap_bytes();
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
static mut opname: [*const ::core::ffi::c_char; 28] = [
    b"STATFS\0".as_ptr() as *const ::core::ffi::c_char,
    b"GETATTR\0".as_ptr() as *const ::core::ffi::c_char,
    b"SETATTR\0".as_ptr() as *const ::core::ffi::c_char,
    b"LOOKUP\0".as_ptr() as *const ::core::ffi::c_char,
    b"MKDIR\0".as_ptr() as *const ::core::ffi::c_char,
    b"RMDIR\0".as_ptr() as *const ::core::ffi::c_char,
    b"SYMLINK\0".as_ptr() as *const ::core::ffi::c_char,
    b"READLINK\0".as_ptr() as *const ::core::ffi::c_char,
    b"MKNOD\0".as_ptr() as *const ::core::ffi::c_char,
    b"UNLINK\0".as_ptr() as *const ::core::ffi::c_char,
    b"RENAME\0".as_ptr() as *const ::core::ffi::c_char,
    b"LINK\0".as_ptr() as *const ::core::ffi::c_char,
    b"READDIR\0".as_ptr() as *const ::core::ffi::c_char,
    b"OPEN\0".as_ptr() as *const ::core::ffi::c_char,
    b"READCHUNK\0".as_ptr() as *const ::core::ffi::c_char,
    b"WRITECHUNK\0".as_ptr() as *const ::core::ffi::c_char,
    b"READ\0".as_ptr() as *const ::core::ffi::c_char,
    b"WRITE\0".as_ptr() as *const ::core::ffi::c_char,
    b"FSYNC\0".as_ptr() as *const ::core::ffi::c_char,
    b"SNAPSHOT\0".as_ptr() as *const ::core::ffi::c_char,
    b"TRUNCATE\0".as_ptr() as *const ::core::ffi::c_char,
    b"GETXATTR\0".as_ptr() as *const ::core::ffi::c_char,
    b"SETXATTR\0".as_ptr() as *const ::core::ffi::c_char,
    b"GETFACL\0".as_ptr() as *const ::core::ffi::c_char,
    b"SETFACL\0".as_ptr() as *const ::core::ffi::c_char,
    b"CREATE\0".as_ptr() as *const ::core::ffi::c_char,
    b"LOCK\0".as_ptr() as *const ::core::ffi::c_char,
    b"META\0".as_ptr() as *const ::core::ffi::c_char,
];
pub const SESSION_HASHSIZE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
static mut sessionshashtab: [*mut session; 256] = [::core::ptr::null_mut::<session>(); 256];
static mut nextsessionid: uint32_t = 0;
static mut SessionSustainTime: uint32_t = 0;
#[no_mangle]
pub unsafe extern "C" fn sessions_attach_session(
    mut vsesdata: *mut ::core::ffi::c_void,
    mut peerip: uint32_t,
    mut version: uint32_t,
) {
    let mut sesdata: *mut session = vsesdata as *mut session;
    (*sesdata).closed = 0 as uint8_t;
    (*sesdata).nsocks = (*sesdata).nsocks.wrapping_add(1);
    (*sesdata).infopeerip = peerip;
    (*sesdata).infoversion = version;
    if (*sesdata).disconnected != 0 as uint32_t {
        (*sesdata).disconnected = 0 as uint32_t;
        changelog(
            b"%u|SESCONNECTED(%u)\0".as_ptr() as *const ::core::ffi::c_char,
            main_time(),
            (*sesdata).sessionid,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sessions_close_session(mut vsesdata: *mut ::core::ffi::c_void) {
    let mut sesdata: *mut session = vsesdata as *mut session;
    if !sesdata.is_null() {
        if (*sesdata).nsocks == 1 as uint32_t {
            (*sesdata).closed = 1 as uint8_t;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sessions_disconnection(mut vsesdata: *mut ::core::ffi::c_void) {
    let mut sesdata: *mut session = vsesdata as *mut session;
    if !sesdata.is_null() {
        if (*sesdata).nsocks > 0 as uint32_t {
            (*sesdata).nsocks = (*sesdata).nsocks.wrapping_sub(1);
        }
        if (*sesdata).nsocks == 0 as uint32_t {
            (*sesdata).disconnected = main_time();
            changelog(
                b"%u|SESDISCONNECTED(%u)\0".as_ptr() as *const ::core::ffi::c_char,
                main_time(),
                (*sesdata).sessionid,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sessions_find_session(
    mut sessionid: uint32_t,
) -> *mut ::core::ffi::c_void {
    let mut asesdata: *mut session = ::core::ptr::null_mut::<session>();
    if sessionid == 0 as uint32_t || sessionid >= 0x80000000 as uint32_t {
        return NULL;
    }
    asesdata = sessionshashtab[sessionid.wrapping_rem(SESSION_HASHSIZE as uint32_t) as usize];
    while !asesdata.is_null() {
        if (*asesdata).sessionid == sessionid {
            return asesdata as *mut ::core::ffi::c_void;
        }
        asesdata = (*asesdata).next as *mut session;
    }
    return NULL;
}
#[inline]
unsafe extern "C" fn sessions_clean_session(mut sesdata: *mut session) {
    if (*sesdata).sessionid > 0 as uint32_t && (*sesdata).sessionid < 0x80000000 as uint32_t {
        of_session_removed((*sesdata).sessionid);
    }
    if !(*sesdata).info.is_null() {
        free((*sesdata).info as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sessions_store(mut fd: *mut bio) -> uint8_t {
    let mut asesdata: *mut session = ::core::ptr::null_mut::<session>();
    let mut fsesrecord: [uint8_t; 61] = [0; 61];
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut hpos: uint32_t = 0;
    if fd.is_null() {
        return 0x16 as uint8_t;
    }
    ptr = &raw mut fsesrecord as *mut uint8_t;
    put32bit(&raw mut ptr, nextsessionid);
    put16bit(&raw mut ptr, 0 as uint16_t);
    if bio_write(
        fd,
        &raw mut fsesrecord as *mut uint8_t as *const ::core::ffi::c_void,
        6 as uint64_t,
    ) != 6 as int64_t
    {
        return 0xff as uint8_t;
    }
    hpos = 0 as uint32_t;
    while hpos < SESSION_HASHSIZE as uint32_t {
        asesdata = sessionshashtab[hpos as usize];
        while !asesdata.is_null() {
            if (*asesdata).closed as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                ptr = &raw mut fsesrecord as *mut uint8_t;
                put32bit(&raw mut ptr, (*asesdata).sessionid);
                put64bit(&raw mut ptr, (*asesdata).exportscsum);
                put32bit(&raw mut ptr, (*asesdata).ileng);
                put32bit(&raw mut ptr, (*asesdata).peerip);
                put32bit(&raw mut ptr, (*asesdata).rootinode);
                put8bit(&raw mut ptr, (*asesdata).sesflags);
                put16bit(&raw mut ptr, (*asesdata).umaskval);
                put16bit(&raw mut ptr, (*asesdata).sclassgroups);
                put32bit(&raw mut ptr, (*asesdata).mintrashretention);
                put32bit(&raw mut ptr, (*asesdata).maxtrashretention);
                put32bit(&raw mut ptr, (*asesdata).rootuid);
                put32bit(&raw mut ptr, (*asesdata).rootgid);
                put32bit(&raw mut ptr, (*asesdata).mapalluid);
                put32bit(&raw mut ptr, (*asesdata).mapallgid);
                put32bit(&raw mut ptr, (*asesdata).disables);
                put32bit(&raw mut ptr, (*asesdata).disconnected);
                if bio_write(
                    fd,
                    &raw mut fsesrecord as *mut uint8_t as *const ::core::ffi::c_void,
                    61 as uint64_t,
                ) != 61 as int64_t
                {
                    return 0xff as uint8_t;
                }
                if (*asesdata).ileng > 0 as uint32_t {
                    if bio_write(
                        fd,
                        (*asesdata).info as *const ::core::ffi::c_void,
                        (*asesdata).ileng as uint64_t,
                    ) != (*asesdata).ileng as int64_t
                    {
                        return 0xff as uint8_t;
                    }
                }
            }
            asesdata = (*asesdata).next as *mut session;
        }
        hpos = hpos.wrapping_add(1);
    }
    memset(
        &raw mut fsesrecord as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        61 as size_t,
    );
    if bio_write(
        fd,
        &raw mut fsesrecord as *mut uint8_t as *const ::core::ffi::c_void,
        61 as uint64_t,
    ) != 61 as int64_t
    {
        return 0xff as uint8_t;
    }
    return 0 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_load(mut fd: *mut bio, mut mver: uint8_t) -> ::core::ffi::c_int {
    let mut asesdata: *mut session = ::core::ptr::null_mut::<session>();
    let mut hdr: [uint8_t; 8] = [0; 8];
    let mut fsesrecord: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut statsinfile: uint16_t = 0;
    let mut recsize: uint32_t = 0;
    let mut i: uint32_t = 0;
    let mut sessionid: uint32_t = 0;
    let mut hpos: uint32_t = 0;
    if (mver as ::core::ffi::c_int) < 0x12 as ::core::ffi::c_int {
        if bio_read(
            fd,
            &raw mut hdr as *mut uint8_t as *mut ::core::ffi::c_void,
            8 as uint64_t,
        ) != 8 as int64_t
        {
            return -1 as ::core::ffi::c_int;
        }
        ptr = &raw mut hdr as *mut uint8_t;
        match get16bit(&raw mut ptr) as ::core::ffi::c_int {
            1 => {
                mver = 0x10 as uint8_t;
            }
            2 => {
                mver = 0x11 as uint8_t;
            }
            _ => return -1 as ::core::ffi::c_int,
        }
    } else {
        if bio_read(
            fd,
            &raw mut hdr as *mut uint8_t as *mut ::core::ffi::c_void,
            6 as uint64_t,
        ) != 6 as int64_t
        {
            return -1 as ::core::ffi::c_int;
        }
        ptr = &raw mut hdr as *mut uint8_t;
    }
    nextsessionid = get32bit(&raw mut ptr);
    statsinfile = get16bit(&raw mut ptr);
    if (mver as ::core::ffi::c_int) < 0x11 as ::core::ffi::c_int {
        recsize = (43 as ::core::ffi::c_int
            + statsinfile as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint32_t;
    } else if (mver as ::core::ffi::c_int) < 0x13 as ::core::ffi::c_int {
        recsize = (47 as ::core::ffi::c_int
            + statsinfile as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint32_t;
    } else if (mver as ::core::ffi::c_int) < 0x14 as ::core::ffi::c_int {
        recsize = (55 as ::core::ffi::c_int
            + statsinfile as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint32_t;
    } else if (mver as ::core::ffi::c_int) < 0x15 as ::core::ffi::c_int {
        recsize = (57 as ::core::ffi::c_int
            + statsinfile as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint32_t;
    } else {
        recsize = (61 as ::core::ffi::c_int
            + statsinfile as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint32_t;
    }
    fsesrecord = malloc(recsize as size_t) as *mut uint8_t;
    if fsesrecord.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr() as *const ::core::ffi::c_char,
            255 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"fsesrecord\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr() as *const ::core::ffi::c_char,
            255 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"fsesrecord\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if fsesrecord
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut uint8_t
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr() as *const ::core::ffi::c_char,
            255 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"fsesrecord\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr() as *const ::core::ffi::c_char,
            255 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"fsesrecord\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    loop {
        if bio_read(
            fd,
            fsesrecord as *mut ::core::ffi::c_void,
            recsize as uint64_t,
        ) == recsize as int64_t
        {
            ptr = fsesrecord;
            sessionid = get32bit(&raw mut ptr);
            if sessionid == 0 as uint32_t {
                free(fsesrecord as *mut ::core::ffi::c_void);
                return 0 as ::core::ffi::c_int;
            }
            asesdata = malloc(::core::mem::size_of::<session>()) as *mut session;
            if asesdata.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    266 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"asesdata\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    266 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"asesdata\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if asesdata
                == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut session
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    266 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"asesdata\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    266 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"asesdata\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
            (*asesdata).sessionid = sessionid;
            if mver as ::core::ffi::c_int >= 0x13 as ::core::ffi::c_int {
                (*asesdata).exportscsum = get64bit(&raw mut ptr);
            } else {
                (*asesdata).exportscsum = 0 as uint64_t;
            }
            (*asesdata).ileng = get32bit(&raw mut ptr);
            (*asesdata).peerip = get32bit(&raw mut ptr);
            (*asesdata).rootinode = get32bit(&raw mut ptr);
            (*asesdata).sesflags = get8bit(&raw mut ptr);
            if mver as ::core::ffi::c_int >= 0x14 as ::core::ffi::c_int {
                (*asesdata).umaskval = get16bit(&raw mut ptr);
            } else {
                (*asesdata).umaskval = 0 as uint16_t;
            }
            if mver as ::core::ffi::c_int >= 0x16 as ::core::ffi::c_int {
                (*asesdata).sclassgroups = get16bit(&raw mut ptr);
            } else {
                let mut g: uint8_t = 0;
                let mut mingoal: uint8_t = 0;
                let mut maxgoal: uint8_t = 0;
                mingoal = get8bit(&raw mut ptr);
                maxgoal = get8bit(&raw mut ptr);
                (*asesdata).sclassgroups = 1 as uint16_t;
                g = mingoal;
                while g as ::core::ffi::c_int <= maxgoal as ::core::ffi::c_int {
                    (*asesdata).sclassgroups = ((*asesdata).sclassgroups as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << g as ::core::ffi::c_int)
                        as uint16_t;
                    g = g.wrapping_add(1);
                }
            }
            (*asesdata).mintrashretention = get32bit(&raw mut ptr);
            (*asesdata).maxtrashretention = get32bit(&raw mut ptr);
            (*asesdata).rootuid = get32bit(&raw mut ptr);
            (*asesdata).rootgid = get32bit(&raw mut ptr);
            (*asesdata).mapalluid = get32bit(&raw mut ptr);
            (*asesdata).mapallgid = get32bit(&raw mut ptr);
            if mver as ::core::ffi::c_int >= 0x15 as ::core::ffi::c_int {
                (*asesdata).disables = get32bit(&raw mut ptr);
            } else {
                (*asesdata).disables = 0 as uint32_t;
            }
            if mver as ::core::ffi::c_int >= 0x11 as ::core::ffi::c_int {
                (*asesdata).disconnected = get32bit(&raw mut ptr);
            } else {
                (*asesdata).disconnected = main_time();
            }
            (*asesdata).info = ::core::ptr::null_mut::<uint8_t>();
            (*asesdata).closed = 0 as uint8_t;
            (*asesdata).nsocks = 0 as uint32_t;
            (*asesdata).infopeerip = (*asesdata).peerip;
            (*asesdata).infoversion = 0 as uint32_t;
            i = 0 as uint32_t;
            while i < SESSION_STATS as uint32_t {
                (*asesdata).chouropstats[i as usize] = if i < statsinfile as uint32_t {
                    get32bit(&raw mut ptr)
                } else {
                    0 as uint32_t
                };
                i = i.wrapping_add(1);
            }
            if statsinfile as ::core::ffi::c_int > SESSION_STATS {
                ptr = ptr.offset(
                    (4 as ::core::ffi::c_int * (statsinfile as ::core::ffi::c_int - SESSION_STATS))
                        as isize,
                );
            }
            i = 0 as uint32_t;
            while i < SESSION_STATS as uint32_t {
                (*asesdata).lhouropstats[i as usize] = if i < statsinfile as uint32_t {
                    get32bit(&raw mut ptr)
                } else {
                    0 as uint32_t
                };
                i = i.wrapping_add(1);
            }
            if (*asesdata).ileng > 0 as uint32_t {
                (*asesdata).info =
                    malloc((*asesdata).ileng.wrapping_add(1 as uint32_t) as size_t) as *mut uint8_t;
                if (*asesdata).info.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        326 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"asesdata->info\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        326 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"asesdata->info\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if (*asesdata).info
                    == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut uint8_t
                {
                    let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        326 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"asesdata->info\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        326 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"asesdata->info\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_1,
                    );
                    abort();
                }
                if bio_read(
                    fd,
                    (*asesdata).info as *mut ::core::ffi::c_void,
                    (*asesdata).ileng as uint64_t,
                ) != (*asesdata).ileng as int64_t
                {
                    free((*asesdata).info as *mut ::core::ffi::c_void);
                    free(asesdata as *mut ::core::ffi::c_void);
                    free(fsesrecord as *mut ::core::ffi::c_void);
                    return -1 as ::core::ffi::c_int;
                }
                *(*asesdata).info.offset((*asesdata).ileng as isize) = 0 as uint8_t;
            }
            hpos = sessionid.wrapping_rem(SESSION_HASHSIZE as uint32_t);
            (*asesdata).next = sessionshashtab[hpos as usize] as *mut session;
            sessionshashtab[hpos as usize] = asesdata;
        } else {
            free(fsesrecord as *mut ::core::ffi::c_void);
            return -1 as ::core::ffi::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sessions_set_nextsessionid(mut nsi: uint32_t) {
    nextsessionid = nsi;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_import_data() -> ::core::ffi::c_int {
    let mut asesdata: *mut session = ::core::ptr::null_mut::<session>();
    let mut hdr: [uint8_t; 8] = [0; 8];
    let mut fsesrecord: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut mapalldata: uint8_t = 0;
    let mut goaltrashdata: uint8_t = 0;
    let mut i: uint32_t = 0;
    let mut statsinfile: uint32_t = 0;
    let mut hpos: uint32_t = 0;
    let mut r: ::core::ffi::c_int = 0;
    let mut fd: *mut FILE = ::core::ptr::null_mut::<FILE>();
    fd = fopen(
        b"sessions.mfs\0".as_ptr() as *const ::core::ffi::c_char,
        b"r\0".as_ptr() as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if fd.is_null() {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            b"can't load sessions, fopen error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if *__errno_location() == ENOENT {
            nextsessionid = 1 as uint32_t;
            return 0 as ::core::ffi::c_int;
        } else {
            return -1 as ::core::ffi::c_int;
        }
    }
    if fread(
        &raw mut hdr as *mut uint8_t as *mut ::core::ffi::c_void,
        8 as size_t,
        1 as size_t,
        fd,
    ) != 1 as ::core::ffi::c_ulong
    {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"can't load sessions, fread error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        fclose(fd);
        return -1 as ::core::ffi::c_int;
    }
    if memcmp(
        &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
        b"MFSS 1.5\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        8 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        mapalldata = 0 as uint8_t;
        goaltrashdata = 0 as uint8_t;
        statsinfile = 16 as uint32_t;
    } else if memcmp(
        &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
        b"MFSS \x01\x06\x01\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        8 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        mapalldata = 1 as uint8_t;
        goaltrashdata = 0 as uint8_t;
        statsinfile = 16 as uint32_t;
    } else if memcmp(
        &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
        b"MFSS \x01\x06\x02\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        8 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        mapalldata = 1 as uint8_t;
        goaltrashdata = 0 as uint8_t;
        statsinfile = 21 as uint32_t;
    } else if memcmp(
        &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
        b"MFSS \x01\x06\x03\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        8 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        mapalldata = 1 as uint8_t;
        goaltrashdata = 0 as uint8_t;
        if fread(
            &raw mut hdr as *mut uint8_t as *mut ::core::ffi::c_void,
            2 as size_t,
            1 as size_t,
            fd,
        ) != 1 as ::core::ffi::c_ulong
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"can't load sessions, fread error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            fclose(fd);
            return -1 as ::core::ffi::c_int;
        }
        ptr = &raw mut hdr as *mut uint8_t;
        statsinfile = get16bit(&raw mut ptr) as uint32_t;
    } else if memcmp(
        &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
        b"MFSS \x01\x06\x04\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        8 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        mapalldata = 1 as uint8_t;
        goaltrashdata = 1 as uint8_t;
        if fread(
            &raw mut hdr as *mut uint8_t as *mut ::core::ffi::c_void,
            2 as size_t,
            1 as size_t,
            fd,
        ) != 1 as ::core::ffi::c_ulong
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"can't load sessions, fread error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            fclose(fd);
            return -1 as ::core::ffi::c_int;
        }
        ptr = &raw mut hdr as *mut uint8_t;
        statsinfile = get16bit(&raw mut ptr) as uint32_t;
    } else {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"can't load sessions, bad header\0".as_ptr() as *const ::core::ffi::c_char,
        );
        fclose(fd);
        return -1 as ::core::ffi::c_int;
    }
    if mapalldata as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        fsesrecord = malloc(
            (25 as uint32_t).wrapping_add(statsinfile.wrapping_mul(8 as uint32_t)) as size_t,
        ) as *mut uint8_t;
    } else if goaltrashdata as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        fsesrecord = malloc(
            (33 as uint32_t).wrapping_add(statsinfile.wrapping_mul(8 as uint32_t)) as size_t,
        ) as *mut uint8_t;
    } else {
        fsesrecord = malloc(
            (43 as uint32_t).wrapping_add(statsinfile.wrapping_mul(8 as uint32_t)) as size_t,
        ) as *mut uint8_t;
    }
    if fsesrecord.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr() as *const ::core::ffi::c_char,
            427 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"fsesrecord\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr() as *const ::core::ffi::c_char,
            427 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"fsesrecord\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if fsesrecord
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut uint8_t
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr() as *const ::core::ffi::c_char,
            427 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"fsesrecord\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr() as *const ::core::ffi::c_char,
            427 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"fsesrecord\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    while feof(fd) == 0 {
        if mapalldata as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            r = fread(
                fsesrecord as *mut ::core::ffi::c_void,
                (25 as uint32_t).wrapping_add(statsinfile.wrapping_mul(8 as uint32_t)) as size_t,
                1 as size_t,
                fd,
            ) as ::core::ffi::c_int;
        } else if goaltrashdata as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            r = fread(
                fsesrecord as *mut ::core::ffi::c_void,
                (33 as uint32_t).wrapping_add(statsinfile.wrapping_mul(8 as uint32_t)) as size_t,
                1 as size_t,
                fd,
            ) as ::core::ffi::c_int;
        } else {
            r = fread(
                fsesrecord as *mut ::core::ffi::c_void,
                (43 as uint32_t).wrapping_add(statsinfile.wrapping_mul(8 as uint32_t)) as size_t,
                1 as size_t,
                fd,
            ) as ::core::ffi::c_int;
        }
        if r == 1 as ::core::ffi::c_int {
            ptr = fsesrecord;
            asesdata = malloc(::core::mem::size_of::<session>()) as *mut session;
            if asesdata.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    440 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"asesdata\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    440 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"asesdata\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if asesdata
                == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut session
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    440 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"asesdata\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    440 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"asesdata\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
            (*asesdata).sessionid = get32bit(&raw mut ptr);
            (*asesdata).ileng = get32bit(&raw mut ptr);
            (*asesdata).peerip = get32bit(&raw mut ptr);
            (*asesdata).rootinode = get32bit(&raw mut ptr);
            (*asesdata).sesflags = get8bit(&raw mut ptr);
            (*asesdata).umaskval = 0 as uint16_t;
            if goaltrashdata != 0 {
                let mut g: uint8_t = 0;
                let mut mingoal: uint8_t = 0;
                let mut maxgoal: uint8_t = 0;
                mingoal = get8bit(&raw mut ptr);
                maxgoal = get8bit(&raw mut ptr);
                (*asesdata).sclassgroups = 1 as uint16_t;
                g = mingoal;
                while g as ::core::ffi::c_int <= maxgoal as ::core::ffi::c_int {
                    (*asesdata).sclassgroups = ((*asesdata).sclassgroups as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << g as ::core::ffi::c_int)
                        as uint16_t;
                    g = g.wrapping_add(1);
                }
                (*asesdata).mintrashretention = get32bit(&raw mut ptr);
                (*asesdata).maxtrashretention = get32bit(&raw mut ptr);
            } else {
                (*asesdata).sclassgroups = 0x3ff as uint16_t;
                (*asesdata).mintrashretention = 0 as uint32_t;
                (*asesdata).maxtrashretention = 0xffffffff as ::core::ffi::c_uint as uint32_t;
            }
            (*asesdata).rootuid = get32bit(&raw mut ptr);
            (*asesdata).rootgid = get32bit(&raw mut ptr);
            if mapalldata != 0 {
                (*asesdata).mapalluid = get32bit(&raw mut ptr);
                (*asesdata).mapallgid = get32bit(&raw mut ptr);
            } else {
                (*asesdata).mapalluid = 0 as uint32_t;
                (*asesdata).mapallgid = 0 as uint32_t;
            }
            (*asesdata).disables = 0 as uint32_t;
            (*asesdata).info = ::core::ptr::null_mut::<uint8_t>();
            (*asesdata).closed = 0 as uint8_t;
            (*asesdata).disconnected = main_time();
            (*asesdata).nsocks = 0 as uint32_t;
            (*asesdata).infopeerip = 0 as uint32_t;
            (*asesdata).infoversion = 0 as uint32_t;
            (*asesdata).exportscsum = 0 as uint64_t;
            i = 0 as uint32_t;
            while i < SESSION_STATS as uint32_t {
                (*asesdata).chouropstats[i as usize] = if i < statsinfile {
                    get32bit(&raw mut ptr)
                } else {
                    0 as uint32_t
                };
                i = i.wrapping_add(1);
            }
            if statsinfile > SESSION_STATS as uint32_t {
                ptr = ptr.offset(
                    (4 as uint32_t)
                        .wrapping_mul(statsinfile.wrapping_sub(SESSION_STATS as uint32_t))
                        as isize,
                );
            }
            i = 0 as uint32_t;
            while i < SESSION_STATS as uint32_t {
                (*asesdata).lhouropstats[i as usize] = if i < statsinfile {
                    get32bit(&raw mut ptr)
                } else {
                    0 as uint32_t
                };
                i = i.wrapping_add(1);
            }
            if (*asesdata).ileng > 0 as uint32_t {
                (*asesdata).info =
                    malloc((*asesdata).ileng.wrapping_add(1 as uint32_t) as size_t) as *mut uint8_t;
                if (*asesdata).info.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        491 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"asesdata->info\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        491 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"asesdata->info\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if (*asesdata).info
                    == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut uint8_t
                {
                    let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        491 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"asesdata->info\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        491 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"asesdata->info\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_1,
                    );
                    abort();
                }
                if fread(
                    (*asesdata).info as *mut ::core::ffi::c_void,
                    (*asesdata).ileng as size_t,
                    1 as size_t,
                    fd,
                ) != 1 as ::core::ffi::c_ulong
                {
                    free((*asesdata).info as *mut ::core::ffi::c_void);
                    free(asesdata as *mut ::core::ffi::c_void);
                    free(fsesrecord as *mut ::core::ffi::c_void);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"can't load sessions, fread error\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    fclose(fd);
                    return -1 as ::core::ffi::c_int;
                }
                *(*asesdata).info.offset((*asesdata).ileng as isize) = 0 as uint8_t;
            }
            hpos = (*asesdata)
                .sessionid
                .wrapping_rem(SESSION_HASHSIZE as uint32_t);
            (*asesdata).next = sessionshashtab[hpos as usize] as *mut session;
            sessionshashtab[hpos as usize] = asesdata;
        }
        if ferror(fd) != 0 {
            free(fsesrecord as *mut ::core::ffi::c_void);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"can't load sessions, fread error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            fclose(fd);
            return -1 as ::core::ffi::c_int;
        }
    }
    free(fsesrecord as *mut ::core::ffi::c_void);
    mfs_log(
        MFSLOG_SYSLOG,
        MFSLOG_NOTICE,
        b"sessions have been loaded\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fclose(fd);
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_import() {
    fprintf(
        stderr,
        b"loading sessions ... \0".as_ptr() as *const ::core::ffi::c_char,
    );
    fflush(stderr);
    match sessions_import_data() {
        0 => {
            fprintf(
                stderr,
                b"file not found\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            fprintf(
                stderr,
                b"if it is not fresh installation then you have to restart all active mounts !!!\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
        1 => {
            fprintf(stderr, b"ok\n\0".as_ptr() as *const ::core::ffi::c_char);
            fprintf(
                stderr,
                b"sessions file has been loaded\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        _ => {
            fprintf(stderr, b"error\n\0".as_ptr() as *const ::core::ffi::c_char);
            fprintf(
                stderr,
                b"due to missing sessions you have to restart all active mounts !!!\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
}
#[inline]
unsafe extern "C" fn sessions_data_session(
    mut vmode: uint8_t,
    mut ptr: *mut uint8_t,
    mut now: uint32_t,
    mut sesdata: *mut session,
) -> uint32_t {
    let mut size: uint32_t = 0;
    let mut pleng: uint32_t = 0;
    let mut i: uint32_t = 0;
    if vmode as ::core::ffi::c_int == 0xff as ::core::ffi::c_int {
        size = (16 as uint32_t).wrapping_add((*sesdata).ileng);
        if !ptr.is_null() {
            put32bit(&raw mut ptr, (*sesdata).sessionid);
            if (*sesdata).infopeerip == 0 as uint32_t && (*sesdata).peerip != 0 as uint32_t {
                put32bit(&raw mut ptr, (*sesdata).peerip);
            } else {
                put32bit(&raw mut ptr, (*sesdata).infopeerip);
            }
            if (*sesdata).nsocks > 0 as uint32_t {
                put32bit(&raw mut ptr, 0xffffffff as uint32_t);
            } else if (*sesdata).closed as ::core::ffi::c_int != 0
                || (*sesdata).disconnected.wrapping_add(SessionSustainTime) < now
            {
                put32bit(&raw mut ptr, 0 as uint32_t);
            } else {
                put32bit(
                    &raw mut ptr,
                    (*sesdata)
                        .disconnected
                        .wrapping_add(SessionSustainTime)
                        .wrapping_sub(now),
                );
            }
            put32bit(&raw mut ptr, (*sesdata).ileng);
            if (*sesdata).ileng > 0 as uint32_t {
                memcpy(
                    ptr as *mut ::core::ffi::c_void,
                    (*sesdata).info as *const ::core::ffi::c_void,
                    (*sesdata).ileng as size_t,
                );
                ptr = ptr.offset((*sesdata).ileng as isize);
            }
        }
    } else if (vmode as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        if (*sesdata).nsocks > 0 as uint32_t {
            size = (37 as ::core::ffi::c_int
                + SESSION_STATS * 8 as ::core::ffi::c_int
                + (if vmode as ::core::ffi::c_int != 0 {
                    10 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint32_t;
            size = size.wrapping_add((*sesdata).ileng);
            if (*sesdata).rootinode == 0 as uint32_t {
                size = size.wrapping_add(1 as uint32_t);
            } else {
                size = size.wrapping_add(fs_getdirpath_size((*sesdata).rootinode));
            }
            if !ptr.is_null() {
                put32bit(&raw mut ptr, (*sesdata).sessionid);
                put32bit(&raw mut ptr, (*sesdata).infopeerip);
                put32bit(&raw mut ptr, (*sesdata).infoversion);
                put32bit(&raw mut ptr, (*sesdata).ileng);
                if (*sesdata).ileng > 0 as uint32_t {
                    memcpy(
                        ptr as *mut ::core::ffi::c_void,
                        (*sesdata).info as *const ::core::ffi::c_void,
                        (*sesdata).ileng as size_t,
                    );
                    ptr = ptr.offset((*sesdata).ileng as isize);
                }
                if (*sesdata).rootinode == 0 as uint32_t {
                    pleng = 1 as uint32_t;
                    put32bit(&raw mut ptr, pleng);
                    put8bit(&raw mut ptr, '.' as uint8_t);
                } else {
                    pleng = fs_getdirpath_size((*sesdata).rootinode);
                    put32bit(&raw mut ptr, pleng);
                    if pleng > 0 as uint32_t {
                        fs_getdirpath_data((*sesdata).rootinode, ptr, pleng);
                        ptr = ptr.offset(pleng as isize);
                    }
                }
                put8bit(&raw mut ptr, (*sesdata).sesflags);
                put32bit(&raw mut ptr, (*sesdata).rootuid);
                put32bit(&raw mut ptr, (*sesdata).rootgid);
                put32bit(&raw mut ptr, (*sesdata).mapalluid);
                put32bit(&raw mut ptr, (*sesdata).mapallgid);
                if vmode != 0 {
                    put8bit(&raw mut ptr, 0 as uint8_t);
                    put8bit(&raw mut ptr, 0 as uint8_t);
                    put32bit(&raw mut ptr, (*sesdata).mintrashretention);
                    put32bit(&raw mut ptr, (*sesdata).maxtrashretention);
                }
                i = 0 as uint32_t;
                while i < SESSION_STATS as uint32_t {
                    put32bit(&raw mut ptr, (*sesdata).chouropstats[i as usize]);
                    i = i.wrapping_add(1);
                }
                i = 0 as uint32_t;
                while i < SESSION_STATS as uint32_t {
                    put32bit(&raw mut ptr, (*sesdata).lhouropstats[i as usize]);
                    i = i.wrapping_add(1);
                }
            }
        } else {
            size = 0 as uint32_t;
        }
    } else {
        if (vmode as ::core::ffi::c_int) < 3 as ::core::ffi::c_int {
            size = (56 as ::core::ffi::c_int + SESSION_STATS * 8 as ::core::ffi::c_int) as uint32_t;
        } else if (vmode as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
            size = (58 as ::core::ffi::c_int + SESSION_STATS * 8 as ::core::ffi::c_int) as uint32_t;
        } else {
            size = (62 as ::core::ffi::c_int + SESSION_STATS * 8 as ::core::ffi::c_int) as uint32_t;
        }
        size = size.wrapping_add((*sesdata).ileng);
        if (*sesdata).rootinode == 0 as uint32_t {
            size = size.wrapping_add(1 as uint32_t);
        } else {
            size = size.wrapping_add(fs_getdirpath_size((*sesdata).rootinode));
        }
        if !ptr.is_null() {
            put32bit(&raw mut ptr, (*sesdata).sessionid);
            if (*sesdata).infopeerip == 0 as uint32_t && (*sesdata).peerip != 0 as uint32_t {
                put32bit(&raw mut ptr, (*sesdata).peerip);
            } else {
                put32bit(&raw mut ptr, (*sesdata).infopeerip);
            }
            put32bit(&raw mut ptr, (*sesdata).infoversion);
            put32bit(&raw mut ptr, of_noofopenedfiles((*sesdata).sessionid));
            if (*sesdata).nsocks > 255 as uint32_t {
                put8bit(&raw mut ptr, 255 as uint8_t);
            } else {
                put8bit(&raw mut ptr, (*sesdata).nsocks as uint8_t);
            }
            if (*sesdata).nsocks > 0 as uint32_t {
                put32bit(&raw mut ptr, 0xffffffff as uint32_t);
            } else if (*sesdata).closed as ::core::ffi::c_int != 0
                || (*sesdata).disconnected.wrapping_add(SessionSustainTime) < now
            {
                put32bit(&raw mut ptr, 0 as uint32_t);
            } else {
                put32bit(
                    &raw mut ptr,
                    (*sesdata)
                        .disconnected
                        .wrapping_add(SessionSustainTime)
                        .wrapping_sub(now),
                );
            }
            put32bit(&raw mut ptr, (*sesdata).ileng);
            if (*sesdata).ileng > 0 as uint32_t {
                memcpy(
                    ptr as *mut ::core::ffi::c_void,
                    (*sesdata).info as *const ::core::ffi::c_void,
                    (*sesdata).ileng as size_t,
                );
                ptr = ptr.offset((*sesdata).ileng as isize);
            }
            if (*sesdata).rootinode == 0 as uint32_t {
                pleng = 1 as uint32_t;
                put32bit(&raw mut ptr, pleng);
                put8bit(&raw mut ptr, '.' as uint8_t);
            } else {
                pleng = fs_getdirpath_size((*sesdata).rootinode);
                put32bit(&raw mut ptr, pleng);
                if pleng > 0 as uint32_t {
                    fs_getdirpath_data((*sesdata).rootinode, ptr, pleng);
                    ptr = ptr.offset(pleng as isize);
                }
            }
            put8bit(&raw mut ptr, (*sesdata).sesflags);
            if vmode as ::core::ffi::c_int >= 3 as ::core::ffi::c_int {
                put16bit(&raw mut ptr, (*sesdata).umaskval);
            }
            put32bit(&raw mut ptr, (*sesdata).rootuid);
            put32bit(&raw mut ptr, (*sesdata).rootgid);
            put32bit(&raw mut ptr, (*sesdata).mapalluid);
            put32bit(&raw mut ptr, (*sesdata).mapallgid);
            if vmode as ::core::ffi::c_int >= 5 as ::core::ffi::c_int {
                put16bit(&raw mut ptr, (*sesdata).sclassgroups);
            } else {
                put8bit(&raw mut ptr, 0 as uint8_t);
                put8bit(&raw mut ptr, 0 as uint8_t);
            }
            put32bit(&raw mut ptr, (*sesdata).mintrashretention);
            put32bit(&raw mut ptr, (*sesdata).maxtrashretention);
            if vmode as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                put32bit(&raw mut ptr, (*sesdata).disables);
            }
            i = 0 as uint32_t;
            while i < SESSION_STATS as uint32_t {
                put32bit(&raw mut ptr, (*sesdata).chouropstats[i as usize]);
                i = i.wrapping_add(1);
            }
            i = 0 as uint32_t;
            while i < SESSION_STATS as uint32_t {
                put32bit(&raw mut ptr, (*sesdata).lhouropstats[i as usize]);
                i = i.wrapping_add(1);
            }
        }
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_datasize(mut vmode: uint8_t) -> uint32_t {
    let mut sesdata: *mut session = ::core::ptr::null_mut::<session>();
    let mut hpos: uint32_t = 0;
    let mut size: uint32_t = 0;
    size = (if vmode as ::core::ffi::c_int == 0xff as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int
    }) as uint32_t;
    hpos = 0 as uint32_t;
    while hpos < SESSION_HASHSIZE as uint32_t {
        sesdata = sessionshashtab[hpos as usize];
        while !sesdata.is_null() {
            size = size.wrapping_add(sessions_data_session(
                vmode,
                ::core::ptr::null_mut::<uint8_t>(),
                0 as uint32_t,
                sesdata,
            ));
            sesdata = (*sesdata).next as *mut session;
        }
        hpos = hpos.wrapping_add(1);
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_datafill(mut ptr: *mut uint8_t, mut vmode: uint8_t) {
    let mut sesdata: *mut session = ::core::ptr::null_mut::<session>();
    let mut now: uint32_t = 0;
    let mut hpos: uint32_t = 0;
    now = main_time();
    if vmode as ::core::ffi::c_int != 0xff as ::core::ffi::c_int {
        put16bit(&raw mut ptr, SESSION_STATS as uint16_t);
    }
    hpos = 0 as uint32_t;
    while hpos < SESSION_HASHSIZE as uint32_t {
        sesdata = sessionshashtab[hpos as usize];
        while !sesdata.is_null() {
            ptr = ptr.offset(sessions_data_session(vmode, ptr, now, sesdata) as isize);
            sesdata = (*sesdata).next as *mut session;
        }
        hpos = hpos.wrapping_add(1);
    }
}
#[inline]
unsafe extern "C" fn sessions_create_session(
    mut exportscsum: uint64_t,
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut umaskval: uint16_t,
    mut rootuid: uint32_t,
    mut rootgid: uint32_t,
    mut mapalluid: uint32_t,
    mut mapallgid: uint32_t,
    mut sclassgroups: uint16_t,
    mut mintrashretention: uint32_t,
    mut maxtrashretention: uint32_t,
    mut disables: uint32_t,
    mut peerip: uint32_t,
    mut info: *const uint8_t,
    mut ileng: uint32_t,
) -> *mut ::core::ffi::c_void {
    let mut sesdata: *mut session = ::core::ptr::null_mut::<session>();
    let mut hpos: uint32_t = 0;
    sesdata = malloc(::core::mem::size_of::<session>()) as *mut session;
    if sesdata.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr() as *const ::core::ffi::c_char,
            738 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"sesdata\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr() as *const ::core::ffi::c_char,
            738 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"sesdata\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if sesdata
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut session
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr() as *const ::core::ffi::c_char,
            738 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"sesdata\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr() as *const ::core::ffi::c_char,
            738 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"sesdata\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    nextsessionid =
        (nextsessionid as ::core::ffi::c_uint & 0x7fffffff as ::core::ffi::c_uint) as uint32_t;
    let c2rust_fresh0 = nextsessionid;
    nextsessionid = nextsessionid.wrapping_add(1);
    (*sesdata).sessionid = c2rust_fresh0;
    if nextsessionid >= 0x80000000 as uint32_t {
        nextsessionid = 1 as uint32_t;
    }
    (*sesdata).exportscsum = exportscsum;
    (*sesdata).rootinode = rootinode;
    (*sesdata).sesflags = (sesflags as ::core::ffi::c_int & !SESFLAG_METARESTORE) as uint8_t;
    (*sesdata).umaskval = umaskval;
    (*sesdata).rootuid = rootuid;
    (*sesdata).rootgid = rootgid;
    (*sesdata).mapalluid = mapalluid;
    (*sesdata).mapallgid = mapallgid;
    (*sesdata).sclassgroups = sclassgroups;
    (*sesdata).mintrashretention = mintrashretention;
    (*sesdata).maxtrashretention = maxtrashretention;
    (*sesdata).disables = disables;
    (*sesdata).peerip = peerip;
    while ileng > 0 as uint32_t
        && *info.offset(ileng.wrapping_sub(1 as uint32_t) as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        ileng = ileng.wrapping_sub(1);
    }
    if ileng > 0 as uint32_t {
        (*sesdata).info = malloc(ileng.wrapping_add(1 as uint32_t) as size_t) as *mut uint8_t;
        if (*sesdata).info.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr() as *const ::core::ffi::c_char,
                762 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"sesdata->info\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr() as *const ::core::ffi::c_char,
                762 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"sesdata->info\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if (*sesdata).info
            == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut uint8_t
        {
            let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr() as *const ::core::ffi::c_char,
                762 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"sesdata->info\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr() as *const ::core::ffi::c_char,
                762 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"sesdata->info\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            abort();
        }
        memcpy(
            (*sesdata).info as *mut ::core::ffi::c_void,
            info as *const ::core::ffi::c_void,
            ileng as size_t,
        );
        *(*sesdata).info.offset(ileng as isize) = 0 as uint8_t;
        (*sesdata).ileng = ileng;
    } else {
        (*sesdata).info = ::core::ptr::null_mut::<uint8_t>();
        (*sesdata).ileng = 0 as uint32_t;
    }
    (*sesdata).closed = 0 as uint8_t;
    (*sesdata).disconnected = 0 as uint32_t;
    (*sesdata).nsocks = 0 as uint32_t;
    (*sesdata).infopeerip = 0 as uint32_t;
    (*sesdata).infoversion = 0 as uint32_t;
    memset(
        &raw mut (*sesdata).chouropstats as *mut uint32_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (4 as ::core::ffi::c_int * SESSION_STATS) as size_t,
    );
    memset(
        &raw mut (*sesdata).lhouropstats as *mut uint32_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (4 as ::core::ffi::c_int * SESSION_STATS) as size_t,
    );
    memset(
        &raw mut (*sesdata).cminopstats as *mut uint32_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (4 as ::core::ffi::c_int * SESSION_STATS) as size_t,
    );
    memset(
        &raw mut (*sesdata).lminopstats as *mut uint32_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (4 as ::core::ffi::c_int * SESSION_STATS) as size_t,
    );
    hpos = (*sesdata)
        .sessionid
        .wrapping_rem(SESSION_HASHSIZE as uint32_t);
    (*sesdata).next = sessionshashtab[hpos as usize] as *mut session;
    sessionshashtab[hpos as usize] = sesdata;
    if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
        changelog(
            b"%u|SESADD(#%lu,%u,%hhu,0%03ho,%u,%u,%u,%u,0x%04hX,%u,%u,0x%08X,%u,%s):%u\0".as_ptr()
                as *const ::core::ffi::c_char,
            main_time(),
            exportscsum,
            rootinode,
            sesflags as ::core::ffi::c_int,
            umaskval as ::core::ffi::c_int,
            rootuid,
            rootgid,
            mapalluid,
            mapallgid,
            sclassgroups as ::core::ffi::c_int,
            mintrashretention,
            maxtrashretention,
            disables,
            peerip,
            changelog_escape_name(ileng, info as *mut uint8_t),
            (*sesdata).sessionid,
        );
    } else {
        meta_version_inc();
    }
    return sesdata as *mut ::core::ffi::c_void;
}
#[inline]
unsafe extern "C" fn sessions_not_changed(
    mut sesdata: *mut session,
    mut exportscsum: uint64_t,
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut umaskval: uint16_t,
    mut rootuid: uint32_t,
    mut rootgid: uint32_t,
    mut mapalluid: uint32_t,
    mut mapallgid: uint32_t,
    mut sclassgroups: uint16_t,
    mut mintrashretention: uint32_t,
    mut maxtrashretention: uint32_t,
    mut disables: uint32_t,
    mut peerip: uint32_t,
    mut info: *const uint8_t,
    mut ileng: uint32_t,
) -> uint8_t {
    if (*sesdata).rootinode != rootinode {
        return 0 as uint8_t;
    }
    if (*sesdata).exportscsum != exportscsum {
        return 0 as uint8_t;
    }
    if (*sesdata).sesflags as ::core::ffi::c_int != sesflags as ::core::ffi::c_int {
        return 0 as uint8_t;
    }
    if (*sesdata).umaskval as ::core::ffi::c_int != umaskval as ::core::ffi::c_int {
        return 0 as uint8_t;
    }
    if (*sesdata).rootuid != rootuid || (*sesdata).rootgid != rootgid {
        return 0 as uint8_t;
    }
    if (*sesdata).mapalluid != mapalluid || (*sesdata).mapallgid != mapallgid {
        return 0 as uint8_t;
    }
    if (*sesdata).sclassgroups as ::core::ffi::c_int != sclassgroups as ::core::ffi::c_int {
        return 0 as uint8_t;
    }
    if (*sesdata).mintrashretention != mintrashretention
        || (*sesdata).maxtrashretention != maxtrashretention
    {
        return 0 as uint8_t;
    }
    if (*sesdata).disables != disables {
        return 0 as uint8_t;
    }
    if (*sesdata).peerip != peerip {
        return 0 as uint8_t;
    }
    if (*sesdata).ileng != ileng {
        return 0 as uint8_t;
    }
    if !(*sesdata).info.is_null() && info.is_null() || (*sesdata).info.is_null() && !info.is_null()
    {
        return 0 as uint8_t;
    }
    if memcmp(
        (*sesdata).info as *const ::core::ffi::c_void,
        info as *const ::core::ffi::c_void,
        ileng as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        return 0 as uint8_t;
    }
    return 1 as uint8_t;
}
#[inline]
unsafe extern "C" fn sessions_change_session(
    mut sesdata: *mut session,
    mut exportscsum: uint64_t,
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut umaskval: uint16_t,
    mut rootuid: uint32_t,
    mut rootgid: uint32_t,
    mut mapalluid: uint32_t,
    mut mapallgid: uint32_t,
    mut sclassgroups: uint16_t,
    mut mintrashretention: uint32_t,
    mut maxtrashretention: uint32_t,
    mut disables: uint32_t,
    mut peerip: uint32_t,
    mut info: *const uint8_t,
    mut ileng: uint32_t,
) -> uint32_t {
    if sessions_not_changed(
        sesdata,
        exportscsum,
        rootinode,
        sesflags,
        umaskval,
        rootuid,
        rootgid,
        mapalluid,
        mapallgid,
        sclassgroups,
        mintrashretention,
        maxtrashretention,
        disables,
        peerip,
        info,
        ileng,
    ) != 0
    {
        return (*sesdata).sessionid;
    }
    (*sesdata).rootinode = rootinode;
    (*sesdata).exportscsum = exportscsum;
    (*sesdata).sesflags = (sesflags as ::core::ffi::c_int & !SESFLAG_METARESTORE) as uint8_t;
    (*sesdata).umaskval = umaskval;
    (*sesdata).rootuid = rootuid;
    (*sesdata).rootgid = rootgid;
    (*sesdata).mapalluid = mapalluid;
    (*sesdata).mapallgid = mapallgid;
    (*sesdata).sclassgroups = sclassgroups;
    (*sesdata).mintrashretention = mintrashretention;
    (*sesdata).maxtrashretention = maxtrashretention;
    (*sesdata).disables = disables;
    (*sesdata).peerip = peerip;
    if !(*sesdata).info.is_null() {
        free((*sesdata).info as *mut ::core::ffi::c_void);
    }
    while ileng > 0 as uint32_t
        && *info.offset(ileng.wrapping_sub(1 as uint32_t) as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        ileng = ileng.wrapping_sub(1);
    }
    if ileng > 0 as uint32_t {
        (*sesdata).info = malloc(ileng.wrapping_add(1 as uint32_t) as size_t) as *mut uint8_t;
        if (*sesdata).info.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr() as *const ::core::ffi::c_char,
                859 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"sesdata->info\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr() as *const ::core::ffi::c_char,
                859 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"sesdata->info\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if (*sesdata).info
            == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut uint8_t
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr() as *const ::core::ffi::c_char,
                859 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"sesdata->info\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/sessions.c\0".as_ptr() as *const ::core::ffi::c_char,
                859 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"sesdata->info\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        memcpy(
            (*sesdata).info as *mut ::core::ffi::c_void,
            info as *const ::core::ffi::c_void,
            ileng as size_t,
        );
        *(*sesdata).info.offset(ileng as isize) = 0 as uint8_t;
        (*sesdata).ileng = ileng;
    } else {
        (*sesdata).info = ::core::ptr::null_mut::<uint8_t>();
        (*sesdata).ileng = 0 as uint32_t;
    }
    if sesflags as ::core::ffi::c_int & SESFLAG_METARESTORE == 0 as ::core::ffi::c_int {
        changelog(
            b"%u|SESCHANGED(%u,#%lu,%u,%hhu,0%03ho,%u,%u,%u,%u,0x%04hX,%u,%u,0x%08X,%u,%s)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            main_time(),
            (*sesdata).sessionid,
            exportscsum,
            rootinode,
            sesflags as ::core::ffi::c_int,
            umaskval as ::core::ffi::c_int,
            rootuid,
            rootgid,
            mapalluid,
            mapallgid,
            sclassgroups as ::core::ffi::c_int,
            mintrashretention,
            maxtrashretention,
            disables,
            peerip,
            changelog_escape_name(ileng, info),
        );
    } else {
        meta_version_inc();
    }
    return (*sesdata).sessionid;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_new_session(
    mut exportscsum: uint64_t,
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut umaskval: uint16_t,
    mut rootuid: uint32_t,
    mut rootgid: uint32_t,
    mut mapalluid: uint32_t,
    mut mapallgid: uint32_t,
    mut sclassgroups: uint16_t,
    mut mintrashretention: uint32_t,
    mut maxtrashretention: uint32_t,
    mut disables: uint32_t,
    mut peerip: uint32_t,
    mut info: *const uint8_t,
    mut ileng: uint32_t,
) -> *mut ::core::ffi::c_void {
    let mut sesdata: *mut session = ::core::ptr::null_mut::<session>();
    sesdata = sessions_create_session(
        exportscsum,
        rootinode,
        (sesflags as ::core::ffi::c_int & !SESFLAG_METARESTORE) as uint8_t,
        umaskval,
        rootuid,
        rootgid,
        mapalluid,
        mapallgid,
        sclassgroups,
        mintrashretention,
        maxtrashretention,
        disables,
        peerip,
        info,
        ileng,
    ) as *mut session;
    return sesdata as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_mr_sesadd(
    mut exportscsum: uint64_t,
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut umaskval: uint16_t,
    mut rootuid: uint32_t,
    mut rootgid: uint32_t,
    mut mapalluid: uint32_t,
    mut mapallgid: uint32_t,
    mut sclassgroups: uint16_t,
    mut mintrashretention: uint32_t,
    mut maxtrashretention: uint32_t,
    mut disables: uint32_t,
    mut peerip: uint32_t,
    mut info: *const uint8_t,
    mut ileng: uint32_t,
    mut sessionid: uint32_t,
) -> uint8_t {
    let mut sesdata: *mut session = ::core::ptr::null_mut::<session>();
    sesdata = sessions_create_session(
        exportscsum,
        rootinode,
        (sesflags as ::core::ffi::c_int | SESFLAG_METARESTORE) as uint8_t,
        umaskval,
        rootuid,
        rootgid,
        mapalluid,
        mapallgid,
        sclassgroups,
        mintrashretention,
        maxtrashretention,
        disables,
        peerip,
        info,
        ileng,
    ) as *mut session;
    if (*sesdata).sessionid != sessionid {
        return MFS_ERROR_MISMATCH as uint8_t;
    }
    return MFS_STATUS_OK as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_chg_session(
    mut vsesdata: *mut ::core::ffi::c_void,
    mut exportscsum: uint64_t,
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut umaskval: uint16_t,
    mut rootuid: uint32_t,
    mut rootgid: uint32_t,
    mut mapalluid: uint32_t,
    mut mapallgid: uint32_t,
    mut sclassgroups: uint16_t,
    mut mintrashretention: uint32_t,
    mut maxtrashretention: uint32_t,
    mut disables: uint32_t,
    mut peerip: uint32_t,
    mut info: *const uint8_t,
    mut ileng: uint32_t,
) -> uint32_t {
    let mut sesdata: *mut session = vsesdata as *mut session;
    return sessions_change_session(
        sesdata,
        exportscsum,
        rootinode,
        (sesflags as ::core::ffi::c_int & !SESFLAG_METARESTORE) as uint8_t,
        umaskval,
        rootuid,
        rootgid,
        mapalluid,
        mapallgid,
        sclassgroups,
        mintrashretention,
        maxtrashretention,
        disables,
        peerip,
        info,
        ileng,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sessions_mr_seschanged(
    mut sessionid: uint32_t,
    mut exportscsum: uint64_t,
    mut rootinode: uint32_t,
    mut sesflags: uint8_t,
    mut umaskval: uint16_t,
    mut rootuid: uint32_t,
    mut rootgid: uint32_t,
    mut mapalluid: uint32_t,
    mut mapallgid: uint32_t,
    mut sclassgroups: uint16_t,
    mut mintrashretention: uint32_t,
    mut maxtrashretention: uint32_t,
    mut disables: uint32_t,
    mut peerip: uint32_t,
    mut info: *const uint8_t,
    mut ileng: uint32_t,
) -> uint8_t {
    let mut sesdata: *mut session = ::core::ptr::null_mut::<session>();
    sesdata = sessions_find_session(sessionid) as *mut session;
    if sesdata.is_null() {
        return MFS_ERROR_MISMATCH as uint8_t;
    }
    sessions_change_session(
        sesdata,
        exportscsum,
        rootinode,
        (sesflags as ::core::ffi::c_int | SESFLAG_METARESTORE) as uint8_t,
        umaskval,
        rootuid,
        rootgid,
        mapalluid,
        mapallgid,
        sclassgroups,
        mintrashretention,
        maxtrashretention,
        disables,
        peerip,
        info,
        ileng,
    );
    return MFS_STATUS_OK as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_mr_sesdel(mut sessionid: uint32_t) -> uint8_t {
    let mut sesdata: *mut *mut session = ::core::ptr::null_mut::<*mut session>();
    let mut asesdata: *mut session = ::core::ptr::null_mut::<session>();
    let mut status: uint8_t = MFS_ERROR_BADSESSIONID as uint8_t;
    let mut hpos: uint32_t = 0;
    hpos = sessionid.wrapping_rem(SESSION_HASHSIZE as uint32_t);
    sesdata = (&raw mut sessionshashtab as *mut *mut session).offset(hpos as isize);
    loop {
        asesdata = *sesdata;
        if asesdata.is_null() {
            break;
        }
        if (*asesdata).sessionid == sessionid {
            sessions_clean_session(asesdata);
            *sesdata = (*asesdata).next as *mut session;
            free(asesdata as *mut ::core::ffi::c_void);
            status = MFS_STATUS_OK as uint8_t;
        } else {
            sesdata = &raw mut (*asesdata).next as *mut *mut session;
        }
    }
    if status as ::core::ffi::c_int == MFS_STATUS_OK {
        meta_version_inc();
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_mr_connected(mut sessionid: uint32_t) -> uint8_t {
    let mut sesdata: *mut session = ::core::ptr::null_mut::<session>();
    sesdata = sessionshashtab[sessionid.wrapping_rem(SESSION_HASHSIZE as uint32_t) as usize];
    while !sesdata.is_null() {
        if (*sesdata).sessionid == sessionid {
            (*sesdata).disconnected = 0 as uint32_t;
            meta_version_inc();
            return MFS_STATUS_OK as uint8_t;
        }
        sesdata = (*sesdata).next as *mut session;
    }
    return MFS_ERROR_NOTFOUND as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_mr_disconnected(
    mut sessionid: uint32_t,
    mut disctime: uint32_t,
) -> uint8_t {
    let mut sesdata: *mut session = ::core::ptr::null_mut::<session>();
    sesdata = sessionshashtab[sessionid.wrapping_rem(SESSION_HASHSIZE as uint32_t) as usize];
    while !sesdata.is_null() {
        if (*sesdata).sessionid == sessionid {
            (*sesdata).disconnected = disctime;
            meta_version_inc();
            return MFS_STATUS_OK as uint8_t;
        }
        sesdata = (*sesdata).next as *mut session;
    }
    return MFS_ERROR_NOTFOUND as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_mr_session(mut sessionid: uint32_t) -> uint8_t {
    if sessionid != nextsessionid {
        return MFS_ERROR_MISMATCH as uint8_t;
    }
    nextsessionid = nextsessionid.wrapping_add(1);
    meta_version_inc();
    return MFS_STATUS_OK as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_new() {
    nextsessionid = 1 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_get_id(mut vsesdata: *mut ::core::ffi::c_void) -> uint32_t {
    let mut sesdata: *mut session = vsesdata as *mut session;
    return (*sesdata).sessionid;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_get_exportscsum(
    mut vsesdata: *mut ::core::ffi::c_void,
) -> uint64_t {
    let mut sesdata: *mut session = vsesdata as *mut session;
    return (*sesdata).exportscsum;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_get_peerip(mut vsesdata: *mut ::core::ffi::c_void) -> uint32_t {
    let mut sesdata: *mut session = vsesdata as *mut session;
    return (*sesdata).peerip;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_get_rootinode(
    mut vsesdata: *mut ::core::ffi::c_void,
) -> uint32_t {
    let mut sesdata: *mut session = vsesdata as *mut session;
    return (*sesdata).rootinode;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_get_sesflags(mut vsesdata: *mut ::core::ffi::c_void) -> uint32_t {
    let mut sesdata: *mut session = vsesdata as *mut session;
    let mut sesflags: uint32_t = 0;
    sesflags = (*sesdata).sesflags as uint32_t;
    if (*sesdata).infoversion
        >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
            + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                32 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            } else {
                32 as ::core::ffi::c_int
            })) as uint32_t
    {
        sesflags |= SESFLAG_ATTRBIT as uint32_t;
    }
    return sesflags;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_get_umask(mut vsesdata: *mut ::core::ffi::c_void) -> uint16_t {
    let mut sesdata: *mut session = vsesdata as *mut session;
    return (*sesdata).umaskval;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_get_disables(mut vsesdata: *mut ::core::ffi::c_void) -> uint32_t {
    let mut sesdata: *mut session = vsesdata as *mut session;
    return (*sesdata).disables;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_is_root_remapped(
    mut vsesdata: *mut ::core::ffi::c_void,
) -> uint8_t {
    let mut sesdata: *mut session = vsesdata as *mut session;
    return (if (*sesdata).rootuid != 0 as uint32_t {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_check_sclass(
    mut vsesdata: *mut ::core::ffi::c_void,
    mut smode: uint8_t,
    mut sclassid: uint8_t,
) -> uint8_t {
    let mut sesdata: *mut session = vsesdata as *mut session;
    match smode as ::core::ffi::c_int {
        SMODE_EXCHANGE | SMODE_SET => {
            if (1 as ::core::ffi::c_int)
                << sclass_get_export_group(sclassid as uint16_t) as ::core::ffi::c_int
                & (*sesdata).sclassgroups as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                return MFS_ERROR_EPERM as uint8_t;
            }
        }
        SMODE_INCREASE => return MFS_ERROR_EPERM as uint8_t,
        SMODE_DECREASE => return MFS_ERROR_EPERM as uint8_t,
        _ => {}
    }
    return MFS_STATUS_OK as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_check_trashretention(
    mut vsesdata: *mut ::core::ffi::c_void,
    mut smode: uint8_t,
    mut trashretention: uint32_t,
) -> uint8_t {
    let mut sesdata: *mut session = vsesdata as *mut session;
    match smode as ::core::ffi::c_int {
        SMODE_EXCHANGE | SMODE_SET => {
            if trashretention < (*sesdata).mintrashretention
                || trashretention > (*sesdata).maxtrashretention
            {
                return MFS_ERROR_EPERM as uint8_t;
            }
        }
        SMODE_INCREASE => {
            if trashretention > (*sesdata).maxtrashretention {
                return MFS_ERROR_EPERM as uint8_t;
            }
        }
        SMODE_DECREASE => {
            if trashretention < (*sesdata).mintrashretention {
                return MFS_ERROR_EPERM as uint8_t;
            }
        }
        _ => {}
    }
    return MFS_STATUS_OK as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_inc_stats(
    mut vsesdata: *mut ::core::ffi::c_void,
    mut statid: uint8_t,
) {
    let mut sesdata: *mut session = vsesdata as *mut session;
    if !sesdata.is_null() && (statid as ::core::ffi::c_int) < SESSION_STATS {
        (*sesdata).chouropstats[statid as usize] =
            (*sesdata).chouropstats[statid as usize].wrapping_add(1);
        (*sesdata).cminopstats[statid as usize] =
            (*sesdata).cminopstats[statid as usize].wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sessions_add_stats(
    mut vsesdata: *mut ::core::ffi::c_void,
    mut statid: uint8_t,
    mut value: uint64_t,
) {
    let mut sesdata: *mut session = vsesdata as *mut session;
    if !sesdata.is_null() && (statid as ::core::ffi::c_int) < SESSION_STATS {
        (*sesdata).chouropstats[statid as usize] =
            ((*sesdata).chouropstats[statid as usize] as uint64_t).wrapping_add(value) as uint32_t;
        (*sesdata).cminopstats[statid as usize] =
            ((*sesdata).cminopstats[statid as usize] as uint64_t).wrapping_add(value) as uint32_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sessions_ugid_remap(
    mut vsesdata: *mut ::core::ffi::c_void,
    mut auid: *mut uint32_t,
    mut agid: *mut uint32_t,
) {
    let mut sesdata: *mut session = vsesdata as *mut session;
    if *auid == 0 as uint32_t {
        *auid = (*sesdata).rootuid;
        if !agid.is_null() {
            *agid = (*sesdata).rootgid;
        }
    } else if (*sesdata).sesflags as ::core::ffi::c_int & SESFLAG_MAPALL != 0 {
        *auid = (*sesdata).mapalluid;
        if !agid.is_null() {
            *agid = (*sesdata).mapallgid;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sessions_check() {
    let mut sesdata: *mut *mut session = ::core::ptr::null_mut::<*mut session>();
    let mut asesdata: *mut session = ::core::ptr::null_mut::<session>();
    let mut now: uint32_t = 0;
    let mut hpos: uint32_t = 0;
    now = main_time();
    if main_start_time().wrapping_add(120 as uint32_t) > now {
        return;
    }
    hpos = 0 as uint32_t;
    while hpos < SESSION_HASHSIZE as uint32_t {
        sesdata = (&raw mut sessionshashtab as *mut *mut session).offset(hpos as isize);
        loop {
            asesdata = *sesdata;
            if asesdata.is_null() {
                break;
            }
            if (*asesdata).nsocks == 0 as uint32_t
                && ((*asesdata).closed as ::core::ffi::c_int != 0
                    || (*asesdata).disconnected.wrapping_add(SessionSustainTime) < now)
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_INFO,
                    b"remove session: %u\0".as_ptr() as *const ::core::ffi::c_char,
                    (*asesdata).sessionid,
                );
                sessions_clean_session(asesdata);
                changelog(
                    b"%u|SESDEL(%u)\0".as_ptr() as *const ::core::ffi::c_char,
                    main_time(),
                    (*asesdata).sessionid,
                );
                *sesdata = (*asesdata).next as *mut session;
                free(asesdata as *mut ::core::ffi::c_void);
            } else {
                sesdata = &raw mut (*asesdata).next as *mut *mut session;
            }
        }
        hpos = hpos.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sessions_force_remove(mut sessionid: uint32_t) -> uint8_t {
    let mut sesdata: *mut *mut session = ::core::ptr::null_mut::<*mut session>();
    let mut asesdata: *mut session = ::core::ptr::null_mut::<session>();
    let mut hpos: uint32_t = 0;
    hpos = sessionid.wrapping_rem(SESSION_HASHSIZE as uint32_t);
    sesdata = (&raw mut sessionshashtab as *mut *mut session).offset(hpos as isize);
    loop {
        asesdata = *sesdata;
        if asesdata.is_null() {
            break;
        }
        if (*asesdata).sessionid == sessionid {
            if (*asesdata).nsocks == 0 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_INFO,
                    b"remove session: %u\0".as_ptr() as *const ::core::ffi::c_char,
                    (*asesdata).sessionid,
                );
                sessions_clean_session(asesdata);
                changelog(
                    b"%u|SESDEL(%u)\0".as_ptr() as *const ::core::ffi::c_char,
                    main_time(),
                    (*asesdata).sessionid,
                );
                *sesdata = (*asesdata).next as *mut session;
                free(asesdata as *mut ::core::ffi::c_void);
                return MFS_STATUS_OK as uint8_t;
            } else {
                return MFS_ERROR_ACTIVE as uint8_t;
            }
        } else {
            sesdata = &raw mut (*asesdata).next as *mut *mut session;
        }
    }
    return MFS_ERROR_NOTFOUND as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn sessions_statsmove() {
    let mut sesdata: *mut session = ::core::ptr::null_mut::<session>();
    let mut hpos: uint32_t = 0;
    hpos = 0 as uint32_t;
    while hpos < SESSION_HASHSIZE as uint32_t {
        sesdata = sessionshashtab[hpos as usize];
        while !sesdata.is_null() {
            memcpy(
                &raw mut (*sesdata).lhouropstats as *mut uint32_t as *mut ::core::ffi::c_void,
                &raw mut (*sesdata).chouropstats as *mut uint32_t as *const ::core::ffi::c_void,
                (4 as ::core::ffi::c_int * SESSION_STATS) as size_t,
            );
            memset(
                &raw mut (*sesdata).chouropstats as *mut uint32_t as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (4 as ::core::ffi::c_int * SESSION_STATS) as size_t,
            );
            sesdata = (*sesdata).next as *mut session;
        }
        hpos = hpos.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sessions_infostats_shift() {
    let mut sesdata: *mut session = ::core::ptr::null_mut::<session>();
    let mut hpos: uint32_t = 0;
    hpos = 0 as uint32_t;
    while hpos < SESSION_HASHSIZE as uint32_t {
        sesdata = sessionshashtab[hpos as usize];
        while !sesdata.is_null() {
            memcpy(
                &raw mut (*sesdata).lminopstats as *mut uint32_t as *mut ::core::ffi::c_void,
                &raw mut (*sesdata).cminopstats as *mut uint32_t as *const ::core::ffi::c_void,
                (4 as ::core::ffi::c_int * SESSION_STATS) as size_t,
            );
            memset(
                &raw mut (*sesdata).cminopstats as *mut uint32_t as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (4 as ::core::ffi::c_int * SESSION_STATS) as size_t,
            );
            sesdata = (*sesdata).next as *mut session;
        }
        hpos = hpos.wrapping_add(1);
    }
}
unsafe extern "C" fn sessions_info_session(mut fd: *mut FILE, mut sesdata: *mut session) {
    let mut i: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut ip: uint32_t = 0;
    let mut strip: [::core::ffi::c_char; 16] = [0; 16];
    let mut iptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*sesdata).infopeerip == 0 as uint32_t && (*sesdata).peerip != 0 as uint32_t {
        ip = (*sesdata).peerip;
    } else {
        ip = (*sesdata).infopeerip;
    }
    univmakestrip(&raw mut strip as *mut ::core::ffi::c_char, ip);
    if (*sesdata).info.is_null() {
        iptr = b"<NULL>\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    } else {
        iptr = (*sesdata).info as *mut ::core::ffi::c_char;
    }
    i = 0 as uint32_t;
    while i < SESSION_STATS as uint32_t {
        c = (*sesdata).lminopstats[i as usize];
        if c > 0 as uint32_t {
            fprintf(
                fd,
                b"session ip:%s mount_point:%s operation:%s count:%u\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                &raw mut strip as *mut ::core::ffi::c_char,
                iptr,
                opname[i as usize],
                c,
            );
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sessions_info(mut fd: *mut FILE) {
    let mut sesdata: *mut session = ::core::ptr::null_mut::<session>();
    let mut hpos: uint32_t = 0;
    fprintf(
        fd,
        b"[session ops]\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    hpos = 0 as uint32_t;
    while hpos < SESSION_HASHSIZE as uint32_t {
        sesdata = sessionshashtab[hpos as usize];
        while !sesdata.is_null() {
            sessions_info_session(fd, sesdata);
            sesdata = (*sesdata).next as *mut session;
        }
        hpos = hpos.wrapping_add(1);
    }
    fprintf(fd, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn sessions_cleanup() {
    let mut ss: *mut session = ::core::ptr::null_mut::<session>();
    let mut ssn: *mut session = ::core::ptr::null_mut::<session>();
    let mut hpos: uint32_t = 0;
    hpos = 0 as uint32_t;
    while hpos < SESSION_HASHSIZE as uint32_t {
        ss = sessionshashtab[hpos as usize];
        while !ss.is_null() {
            ssn = (*ss).next as *mut session;
            if !(*ss).info.is_null() {
                free((*ss).info as *mut ::core::ffi::c_void);
            }
            free(ss as *mut ::core::ffi::c_void);
            ss = ssn;
        }
        sessionshashtab[hpos as usize] = ::core::ptr::null_mut::<session>();
        hpos = hpos.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sessions_reload() {
    SessionSustainTime = cfg_getsperiod(
        b"SESSION_SUSTAIN_TIME\0".as_ptr() as *const ::core::ffi::c_char,
        b"1d\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if SessionSustainTime > (7 as ::core::ffi::c_int * 86400 as ::core::ffi::c_int) as uint32_t {
        SessionSustainTime = (7 as ::core::ffi::c_int * 86400 as ::core::ffi::c_int) as uint32_t;
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            b"SESSION_SUSTAIN_TIME too big (more than week) - setting this value to one week\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if SessionSustainTime < 60 as uint32_t {
        SessionSustainTime = 60 as uint32_t;
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_WARNING,
            b"SESSION_SUSTAIN_TIME too low (less than minute) - setting this value to one minute\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sessions_init() -> ::core::ffi::c_int {
    let mut hpos: uint32_t = 0;
    hpos = 0 as uint32_t;
    while hpos < SESSION_HASHSIZE as uint32_t {
        sessionshashtab[hpos as usize] = ::core::ptr::null_mut::<session>();
        hpos = hpos.wrapping_add(1);
    }
    sessions_reload();
    main_time_register_fname(
        10 as uint32_t,
        0 as uint32_t,
        Some(sessions_check as unsafe extern "C" fn() -> ()),
        b"sessions_check\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_time_register_fname(
        3600 as uint32_t,
        0 as uint32_t,
        Some(sessions_statsmove as unsafe extern "C" fn() -> ()),
        b"sessions_statsmove\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_time_register_fname(
        60 as uint32_t,
        0 as uint32_t,
        Some(sessions_infostats_shift as unsafe extern "C" fn() -> ()),
        b"sessions_infostats_shift\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_info_register_fname(
        Some(sessions_info as unsafe extern "C" fn(*mut FILE) -> ()),
        b"sessions_info\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_reload_register_fname(
        Some(sessions_reload as unsafe extern "C" fn() -> ()),
        b"sessions_reload\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
