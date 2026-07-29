use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _bio;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fputc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn bio_read(b: *mut bio, dst: *mut ::core::ffi::c_void, len: uint64_t) -> int64_t;
    fn bio_write(b: *mut bio, src: *const ::core::ffi::c_void, len: uint64_t) -> int64_t;
    fn changelog(format: *const ::core::ffi::c_char, ...);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
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
    fn matocsserv_getservdata(
        e: *mut ::core::ffi::c_void,
        ver: *mut uint32_t,
        uspc: *mut uint64_t,
        tspc: *mut uint64_t,
        chunkcnt: *mut uint32_t,
        tduspc: *mut uint64_t,
        tdtspc: *mut uint64_t,
        tdchunkcnt: *mut uint32_t,
        errcnt: *mut uint32_t,
        load: *mut uint32_t,
        hlstatus: *mut uint8_t,
        labelmask: *mut uint32_t,
        mfrstatus: *mut uint8_t,
    );
    fn cfg_getuint32(name: *const ::core::ffi::c_char, def: uint32_t) -> uint32_t;
    fn cfg_getdouble(
        name: *const ::core::ffi::c_char,
        def: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn cfg_getsperiod(
        name: *const ::core::ffi::c_char,
        def: *const ::core::ffi::c_char,
    ) -> uint32_t;
    fn main_reload_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    fn main_time_register_fname(
        seconds: uint32_t,
        offset: uint32_t,
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    fn main_time() -> uint32_t;
    fn meta_version_inc() -> uint64_t;
    fn multilan_map(servip: uint32_t, clientip: uint32_t) -> uint32_t;
    fn univmakestrip(strip: *mut ::core::ffi::c_char, ip: uint32_t);
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
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type bio = _bio;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csdbentry {
    pub ip: uint32_t,
    pub port: uint16_t,
    pub csid: uint16_t,
    pub number: uint16_t,
    pub heavyloadts: uint32_t,
    pub load: uint32_t,
    pub maintenance_timeout: uint32_t,
    pub disconnection_time: uint32_t,
    pub maintenance: uint8_t,
    pub eptr: *mut ::core::ffi::c_void,
    pub next: *mut csdbentry,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_EINVAL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MFS_ERROR_MISMATCH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MFS_ERROR_NOTFOUND: ::core::ffi::c_int = 41 as ::core::ffi::c_int;
pub const MFS_ERROR_ACTIVE: ::core::ffi::c_int = 42 as ::core::ffi::c_int;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const HLSTATUS_OK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const HLSTATUS_OVERLOADED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const HLSTATUS_LSREBALANCE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const HLSTATUS_HSREBALANCE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const CSERV_FLAG_DISCONNECTED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CSERV_FLAG_MAINTENANCE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CSERV_FLAG_TMPMAINTENANCE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
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
#[inline]
unsafe extern "C" fn hash32(mut key: uint32_t) -> uint32_t {
    key = (!key).wrapping_add(key << 15 as ::core::ffi::c_int);
    key = key ^ key >> 12 as ::core::ffi::c_int;
    key = key.wrapping_add(key << 2 as ::core::ffi::c_int);
    key = key ^ key >> 4 as ::core::ffi::c_int;
    key = key.wrapping_mul(2057 as uint32_t);
    key = key ^ key >> 16 as ::core::ffi::c_int;
    return key;
}
pub const CSDB_OP_ADD: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CSDB_OP_DEL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CSDB_OP_NEWIPPORT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CSDB_OP_NEWID: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CSDB_OP_MAINTENANCEON: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CSDB_OP_MAINTENANCEOFF: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const CSDB_OP_MAINTENANCETMP: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
static mut HeavyLoadGracePeriod: uint32_t = 0;
static mut HeavyLoadThreshold: uint32_t = 0;
static mut HeavyLoadRatioThreshold: ::core::ffi::c_double = 0.;
static mut MaintenanceModeTimeout: uint32_t = 0;
static mut TempMaintenanceModeTimeout: uint32_t = 0;
static mut SecondsToRemoveUnusedCS: uint32_t = 0;
pub const CSDBHASHSIZE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MAINTENANCE_OFF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MAINTENANCE_ON: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MAINTENANCE_TMP: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut csdbhash: [*mut csdbentry; 256] = [::core::ptr::null_mut::<csdbentry>(); 256];
static mut csdbtab: *mut *mut csdbentry = ::core::ptr::null_mut::<*mut csdbentry>();
static mut nextid: uint32_t = 0;
static mut disconnected_servers: uint32_t = 0;
static mut disconnected_servers_in_maintenance: uint32_t = 0;
static mut servers: uint32_t = 0;
static mut loadsum: uint32_t = 0;
#[no_mangle]
pub unsafe extern "C" fn csdb_self_check() {
    let mut hash: uint32_t = 0;
    let mut now: uint32_t = 0;
    let mut csptr: *mut csdbentry = ::core::ptr::null_mut::<csdbentry>();
    let mut ds: uint32_t = 0;
    let mut dsm: uint32_t = 0;
    let mut s: uint32_t = 0;
    now = main_time();
    ds = 0 as uint32_t;
    dsm = 0 as uint32_t;
    s = 0 as uint32_t;
    hash = 0 as uint32_t;
    while hash < CSDBHASHSIZE as uint32_t {
        csptr = csdbhash[hash as usize];
        while !csptr.is_null() {
            if (*csptr).maintenance as ::core::ffi::c_int == MAINTENANCE_TMP
                && !(*csptr).eptr.is_null()
                || (*csptr).maintenance as ::core::ffi::c_int != MAINTENANCE_OFF
                    && (*csptr).maintenance_timeout > 0 as uint32_t
                    && now > (*csptr).maintenance_timeout
            {
                if (*csptr).eptr.is_null() {
                    disconnected_servers_in_maintenance =
                        disconnected_servers_in_maintenance.wrapping_sub(1);
                }
                (*csptr).maintenance = MAINTENANCE_OFF as uint8_t;
                (*csptr).maintenance_timeout = 0 as uint32_t;
                changelog(
                    b"%u|CSDBOP(%u,%u,%hu,0)\0".as_ptr() as *const ::core::ffi::c_char,
                    main_time(),
                    CSDB_OP_MAINTENANCEOFF,
                    (*csptr).ip,
                    (*csptr).port as ::core::ffi::c_int,
                );
            }
            s = s.wrapping_add(1);
            if (*csptr).eptr.is_null() {
                ds = ds.wrapping_add(1);
                if (*csptr).maintenance as ::core::ffi::c_int != MAINTENANCE_OFF {
                    dsm = dsm.wrapping_add(1);
                }
            }
            csptr = (*csptr).next as *mut csdbentry;
        }
        hash = hash.wrapping_add(1);
    }
    if s != servers {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"csdb: servers counter mismatch - fixing (%u->%u)\0".as_ptr()
                as *const ::core::ffi::c_char,
            servers,
            s,
        );
        servers = s;
    }
    if ds != disconnected_servers {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"csdb: disconnected servers counter mismatch - fixing (%u->%u)\0".as_ptr()
                as *const ::core::ffi::c_char,
            disconnected_servers,
            ds,
        );
        disconnected_servers = ds;
    }
    if dsm != disconnected_servers_in_maintenance {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"csdb: disconnected and being maintained servers counter mismatch - fixing (%u->%u)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            disconnected_servers_in_maintenance,
            dsm,
        );
        disconnected_servers_in_maintenance = dsm;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csdb_newid() -> uint16_t {
    while nextid < 65536 as uint32_t && !(*csdbtab.offset(nextid as isize)).is_null() {
        nextid = nextid.wrapping_add(1);
    }
    return nextid as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn csdb_delid(mut csid: uint16_t) {
    *csdbtab.offset(csid as isize) = ::core::ptr::null_mut::<csdbentry>();
    if (csid as uint32_t) < nextid {
        nextid = csid as uint32_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csdb_new_connection(
    mut ip: uint32_t,
    mut port: uint16_t,
    mut csid: uint16_t,
    mut eptr: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut hash: uint32_t = 0;
    let mut hashid: uint32_t = 0;
    let mut csptr: *mut csdbentry = ::core::ptr::null_mut::<csdbentry>();
    let mut cspptr: *mut *mut csdbentry = ::core::ptr::null_mut::<*mut csdbentry>();
    let mut csidptr: *mut csdbentry = ::core::ptr::null_mut::<csdbentry>();
    let mut strip: [::core::ffi::c_char; 16] = [0; 16];
    let mut strtmpip: [::core::ffi::c_char; 16] = [0; 16];
    univmakestrip(&raw mut strip as *mut ::core::ffi::c_char, ip);
    if csid as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        csidptr = *csdbtab.offset(csid as isize);
    } else {
        csidptr = ::core::ptr::null_mut::<csdbentry>();
    }
    if !csidptr.is_null()
        && (*csidptr).ip == ip
        && (*csidptr).port as ::core::ffi::c_int == port as ::core::ffi::c_int
    {
        if !(*csidptr).eptr.is_null() {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_NOTICE,
                b"csdb: found cs using ip:port and csid (%s:%hu,%hu), but server is still connected\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                &raw mut strip as *mut ::core::ffi::c_char,
                port as ::core::ffi::c_int,
                csid as ::core::ffi::c_int,
            );
            return NULL;
        }
        (*csidptr).eptr = eptr;
        disconnected_servers = disconnected_servers.wrapping_sub(1);
        if (*csidptr).maintenance as ::core::ffi::c_int != MAINTENANCE_OFF {
            disconnected_servers_in_maintenance =
                disconnected_servers_in_maintenance.wrapping_sub(1);
        }
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_INFO,
            b"csdb: found cs using ip:port and csid (%s:%hu,%hu)\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut strip as *mut ::core::ffi::c_char,
            port as ::core::ffi::c_int,
            csid as ::core::ffi::c_int,
        );
        return csidptr as *mut ::core::ffi::c_void;
    }
    hash = hash32(ip ^ ((port as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as uint32_t)
        .wrapping_rem(CSDBHASHSIZE as uint32_t);
    csptr = csdbhash[hash as usize];
    while !csptr.is_null() {
        if (*csptr).ip == ip && (*csptr).port as ::core::ffi::c_int == port as ::core::ffi::c_int {
            if !(*csptr).eptr.is_null() {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_NOTICE,
                    b"csdb: found cs using ip:port (%s:%hu,%hu), but server is still connected\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    &raw mut strip as *mut ::core::ffi::c_char,
                    port as ::core::ffi::c_int,
                    csid as ::core::ffi::c_int,
                );
                return NULL;
            }
            (*csptr).eptr = eptr;
            disconnected_servers = disconnected_servers.wrapping_sub(1);
            if (*csptr).maintenance as ::core::ffi::c_int != MAINTENANCE_OFF {
                disconnected_servers_in_maintenance =
                    disconnected_servers_in_maintenance.wrapping_sub(1);
            }
            return csptr as *mut ::core::ffi::c_void;
        }
        csptr = (*csptr).next as *mut csdbentry;
    }
    if !csidptr.is_null() && (*csidptr).eptr.is_null() {
        univmakestrip(&raw mut strtmpip as *mut ::core::ffi::c_char, (*csidptr).ip);
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_NOTICE,
            b"csdb: found cs using csid (%s:%hu,%hu) - previous ip:port (%s:%hu)\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut strip as *mut ::core::ffi::c_char,
            port as ::core::ffi::c_int,
            csid as ::core::ffi::c_int,
            &raw mut strtmpip as *mut ::core::ffi::c_char,
            (*csidptr).port as ::core::ffi::c_int,
        );
        hashid = hash32(
            (*csidptr).ip
                ^ (((*csidptr).port as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as uint32_t,
        )
        .wrapping_rem(CSDBHASHSIZE as uint32_t);
        cspptr = (&raw mut csdbhash as *mut *mut csdbentry).offset(hashid as isize);
        loop {
            csptr = *cspptr;
            if csptr.is_null() {
                break;
            }
            if csptr == csidptr {
                *cspptr = (*csptr).next as *mut csdbentry;
                (*csptr).next = csdbhash[hash as usize] as *mut csdbentry;
                csdbhash[hash as usize] = csptr;
                break;
            } else {
                cspptr = &raw mut (*csptr).next as *mut *mut csdbentry;
            }
        }
        (*csidptr).ip = ip;
        (*csidptr).port = port;
        changelog(
            b"%u|CSDBOP(%u,%u,%hu,%hu)\0".as_ptr() as *const ::core::ffi::c_char,
            main_time(),
            CSDB_OP_NEWIPPORT,
            ip,
            port as ::core::ffi::c_int,
            (*csidptr).csid as ::core::ffi::c_int,
        );
        (*csidptr).eptr = eptr;
        disconnected_servers = disconnected_servers.wrapping_sub(1);
        if (*csidptr).maintenance as ::core::ffi::c_int != MAINTENANCE_OFF {
            disconnected_servers_in_maintenance =
                disconnected_servers_in_maintenance.wrapping_sub(1);
        }
        return csidptr as *mut ::core::ffi::c_void;
    }
    mfs_log(
        MFSLOG_SYSLOG,
        MFSLOG_INFO,
        b"csdb: server not found (%s:%hu,%hu), add it to database\0".as_ptr()
            as *const ::core::ffi::c_char,
        &raw mut strip as *mut ::core::ffi::c_char,
        port as ::core::ffi::c_int,
        csid as ::core::ffi::c_int,
    );
    csptr = malloc(::core::mem::size_of::<csdbentry>()) as *mut csdbentry;
    if csptr.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/csdb.c\0".as_ptr() as *const ::core::ffi::c_char,
            230 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"csptr\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/csdb.c\0".as_ptr() as *const ::core::ffi::c_char,
            230 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"csptr\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if csptr
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut csdbentry
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/csdb.c\0".as_ptr() as *const ::core::ffi::c_char,
            230 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"csptr\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/csdb.c\0".as_ptr() as *const ::core::ffi::c_char,
            230 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"csptr\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    (*csptr).ip = ip;
    (*csptr).port = port;
    if csid as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        if (*csdbtab.offset(csid as isize)).is_null() {
            *csdbtab.offset(csid as isize) = csptr;
        } else {
            csid = 0 as uint16_t;
        }
    }
    (*csptr).csid = csid;
    (*csptr).heavyloadts = 0 as uint32_t;
    (*csptr).maintenance_timeout = 0 as uint32_t;
    (*csptr).maintenance = MAINTENANCE_OFF as uint8_t;
    (*csptr).disconnection_time = main_time();
    (*csptr).load = 0 as uint32_t;
    (*csptr).eptr = eptr;
    (*csptr).next = csdbhash[hash as usize] as *mut csdbentry;
    csdbhash[hash as usize] = csptr;
    servers = servers.wrapping_add(1);
    changelog(
        b"%u|CSDBOP(%u,%u,%hu,%hu)\0".as_ptr() as *const ::core::ffi::c_char,
        main_time(),
        CSDB_OP_ADD,
        ip,
        port as ::core::ffi::c_int,
        (*csptr).csid as ::core::ffi::c_int,
    );
    return csptr as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn csdb_temporary_maintenance_mode(mut v_csptr: *mut ::core::ffi::c_void) {
    let mut csptr: *mut csdbentry = v_csptr as *mut csdbentry;
    if !csptr.is_null()
        && !(*csptr).eptr.is_null()
        && (*csptr).maintenance as ::core::ffi::c_int == MAINTENANCE_OFF
    {
        (*csptr).maintenance = MAINTENANCE_TMP as uint8_t;
        if TempMaintenanceModeTimeout > 0 as uint32_t {
            (*csptr).maintenance_timeout = main_time().wrapping_add(TempMaintenanceModeTimeout);
        } else {
            (*csptr).maintenance_timeout = 0 as uint32_t;
        }
        changelog(
            b"%u|CSDBOP(%u,%u,%hu,%u)\0".as_ptr() as *const ::core::ffi::c_char,
            main_time(),
            CSDB_OP_MAINTENANCETMP,
            (*csptr).ip,
            (*csptr).port as ::core::ffi::c_int,
            (*csptr).maintenance_timeout,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn csdb_lost_connection(mut v_csptr: *mut ::core::ffi::c_void) {
    let mut csptr: *mut csdbentry = v_csptr as *mut csdbentry;
    if !csptr.is_null() {
        (*csptr).disconnection_time = main_time();
        (*csptr).eptr = NULL;
        disconnected_servers = disconnected_servers.wrapping_add(1);
        if (*csptr).maintenance as ::core::ffi::c_int != MAINTENANCE_OFF {
            disconnected_servers_in_maintenance =
                disconnected_servers_in_maintenance.wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn csdb_server_load(
    mut v_csptr: *mut ::core::ffi::c_void,
    mut load: uint32_t,
) {
    let mut csptr: *mut csdbentry = v_csptr as *mut csdbentry;
    let mut loadavg: ::core::ffi::c_double = 0.;
    let mut strip: [::core::ffi::c_char; 16] = [0; 16];
    loadsum = loadsum.wrapping_sub((*csptr).load);
    if servers > 1 as uint32_t {
        loadavg =
            loadsum.wrapping_div(servers.wrapping_sub(1 as uint32_t)) as ::core::ffi::c_double;
    } else {
        loadavg = load as ::core::ffi::c_double;
    }
    (*csptr).load = load;
    loadsum = loadsum.wrapping_add(load);
    if load > HeavyLoadThreshold
        && load as ::core::ffi::c_double > loadavg * HeavyLoadRatioThreshold
    {
        univmakestrip(&raw mut strip as *mut ::core::ffi::c_char, (*csptr).ip);
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_NOTICE,
            b"Heavy load server detected (%s:%u); load: %u ; threshold: %u ; loadavg (without this server): %.2lf ; ratio_threshold: %.2lf\0"
                .as_ptr() as *const ::core::ffi::c_char,
            &raw mut strip as *mut ::core::ffi::c_char,
            (*csptr).port as ::core::ffi::c_int,
            (*csptr).load,
            HeavyLoadThreshold,
            loadavg,
            HeavyLoadRatioThreshold,
        );
        (*csptr).heavyloadts = main_time();
    }
}
#[no_mangle]
pub unsafe extern "C" fn csdb_get_csid(mut v_csptr: *mut ::core::ffi::c_void) -> uint16_t {
    let mut csptr: *mut csdbentry = v_csptr as *mut csdbentry;
    let mut strip: [::core::ffi::c_char; 16] = [0; 16];
    if (*csptr).csid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*csptr).csid = csdb_newid();
        *csdbtab.offset((*csptr).csid as isize) = csptr;
        changelog(
            b"%u|CSDBOP(%u,%u,%hu,%hu)\0".as_ptr() as *const ::core::ffi::c_char,
            main_time(),
            CSDB_OP_NEWID,
            (*csptr).ip,
            (*csptr).port as ::core::ffi::c_int,
            (*csptr).csid as ::core::ffi::c_int,
        );
        univmakestrip(&raw mut strip as *mut ::core::ffi::c_char, (*csptr).ip);
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_INFO,
            b"csdb: generate new server id for (%s:%hu): %hu\0".as_ptr()
                as *const ::core::ffi::c_char,
            &raw mut strip as *mut ::core::ffi::c_char,
            (*csptr).port as ::core::ffi::c_int,
            (*csptr).csid as ::core::ffi::c_int,
        );
    }
    return (*csptr).csid;
}
#[no_mangle]
pub unsafe extern "C" fn csdb_server_is_overloaded(
    mut v_csptr: *mut ::core::ffi::c_void,
    mut now: uint32_t,
) -> uint8_t {
    let mut csptr: *mut csdbentry = v_csptr as *mut csdbentry;
    return (if (*csptr).heavyloadts.wrapping_add(HeavyLoadGracePeriod) <= now {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    }) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn csdb_server_is_being_maintained(
    mut v_csptr: *mut ::core::ffi::c_void,
) -> uint8_t {
    let mut csptr: *mut csdbentry = v_csptr as *mut csdbentry;
    return (if (*csptr).maintenance as ::core::ffi::c_int != MAINTENANCE_OFF {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn csdb_servlist_data(
    mut mode: uint8_t,
    mut ptr: *mut uint8_t,
    mut clientip: uint32_t,
) -> uint32_t {
    let mut hash: uint32_t = 0;
    let mut now: uint32_t = main_time();
    let mut gracetime: uint32_t = 0;
    let mut maintenance_timeout: uint32_t = 0;
    let mut p: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut csptr: *mut csdbentry = ::core::ptr::null_mut::<csdbentry>();
    let mut i: uint32_t = 0;
    let mut recsize: uint32_t = 0;
    recsize = (if mode as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        73 as ::core::ffi::c_int
    } else {
        77 as ::core::ffi::c_int
    }) as uint32_t;
    i = 0 as uint32_t;
    hash = 0 as uint32_t;
    while hash < CSDBHASHSIZE as uint32_t {
        csptr = csdbhash[hash as usize];
        while !csptr.is_null() {
            if !ptr.is_null() {
                if (*csptr).heavyloadts.wrapping_add(HeavyLoadGracePeriod) > now {
                    gracetime = (*csptr)
                        .heavyloadts
                        .wrapping_add(HeavyLoadGracePeriod)
                        .wrapping_sub(now);
                } else {
                    gracetime = 0 as uint32_t;
                }
                if (*csptr).maintenance_timeout >= now {
                    maintenance_timeout = (*csptr).maintenance_timeout.wrapping_sub(now);
                } else if (*csptr).maintenance_timeout > 0 as uint32_t {
                    maintenance_timeout = 0 as uint32_t;
                } else {
                    maintenance_timeout = 0xffffffff as ::core::ffi::c_uint as uint32_t;
                }
                p = ptr;
                if !(*csptr).eptr.is_null() {
                    let mut version: uint32_t = 0;
                    let mut chunkscount: uint32_t = 0;
                    let mut tdchunkscount: uint32_t = 0;
                    let mut errorcounter: uint32_t = 0;
                    let mut load: uint32_t = 0;
                    let mut labelmask: uint32_t = 0;
                    let mut usedspace: uint64_t = 0;
                    let mut totalspace: uint64_t = 0;
                    let mut tdusedspace: uint64_t = 0;
                    let mut tdtotalspace: uint64_t = 0;
                    let mut hlstatus: uint8_t = 0;
                    let mut mfrstatus: uint8_t = 0;
                    matocsserv_getservdata(
                        (*csptr).eptr,
                        &raw mut version,
                        &raw mut usedspace,
                        &raw mut totalspace,
                        &raw mut chunkscount,
                        &raw mut tdusedspace,
                        &raw mut tdtotalspace,
                        &raw mut tdchunkscount,
                        &raw mut errorcounter,
                        &raw mut load,
                        &raw mut hlstatus,
                        &raw mut labelmask,
                        &raw mut mfrstatus,
                    );
                    if hlstatus as ::core::ffi::c_int == HLSTATUS_OK {
                        gracetime = 0 as uint32_t;
                    } else if hlstatus as ::core::ffi::c_int == HLSTATUS_OVERLOADED {
                        gracetime = 0xc0000000 as ::core::ffi::c_uint as uint32_t;
                    } else if hlstatus as ::core::ffi::c_int == HLSTATUS_LSREBALANCE {
                        gracetime = 0x80000000 as ::core::ffi::c_uint as uint32_t;
                    } else if hlstatus as ::core::ffi::c_int == HLSTATUS_HSREBALANCE {
                        gracetime = 0x40000000 as uint32_t;
                    }
                    put32bit(&raw mut ptr, version & 0xffffff as uint32_t);
                    put32bit(&raw mut ptr, (*csptr).ip);
                    put32bit(&raw mut ptr, multilan_map((*csptr).ip, clientip));
                    put16bit(&raw mut ptr, (*csptr).port);
                    put16bit(&raw mut ptr, (*csptr).csid);
                    put64bit(&raw mut ptr, usedspace);
                    put64bit(&raw mut ptr, totalspace);
                    put32bit(&raw mut ptr, chunkscount);
                    put64bit(&raw mut ptr, tdusedspace);
                    put64bit(&raw mut ptr, tdtotalspace);
                    put32bit(&raw mut ptr, tdchunkscount);
                    put32bit(&raw mut ptr, errorcounter);
                    put32bit(&raw mut ptr, load);
                    put32bit(&raw mut ptr, gracetime);
                    put32bit(&raw mut ptr, labelmask);
                    put8bit(&raw mut ptr, mfrstatus);
                } else {
                    put32bit(&raw mut ptr, 0 as uint32_t);
                    put32bit(&raw mut ptr, (*csptr).ip);
                    put32bit(&raw mut ptr, multilan_map((*csptr).ip, clientip));
                    put16bit(&raw mut ptr, (*csptr).port);
                    put16bit(&raw mut ptr, (*csptr).csid);
                    put64bit(&raw mut ptr, 0 as uint64_t);
                    put64bit(&raw mut ptr, 0 as uint64_t);
                    put32bit(&raw mut ptr, 0 as uint32_t);
                    put64bit(&raw mut ptr, 0 as uint64_t);
                    put64bit(&raw mut ptr, 0 as uint64_t);
                    put32bit(&raw mut ptr, 0 as uint32_t);
                    put32bit(&raw mut ptr, 0 as uint32_t);
                    put32bit(&raw mut ptr, 0 as uint32_t);
                    put32bit(&raw mut ptr, gracetime);
                    put32bit(&raw mut ptr, 0 as uint32_t);
                    put8bit(&raw mut ptr, 0 as uint8_t);
                }
                if mode as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    put32bit(&raw mut ptr, maintenance_timeout);
                }
                if (*csptr).eptr.is_null() {
                    *p = (*p as ::core::ffi::c_int | CSERV_FLAG_DISCONNECTED) as uint8_t;
                }
                if (*csptr).maintenance as ::core::ffi::c_int != MAINTENANCE_OFF {
                    *p = (*p as ::core::ffi::c_int | CSERV_FLAG_MAINTENANCE) as uint8_t;
                }
                if (*csptr).maintenance as ::core::ffi::c_int == MAINTENANCE_TMP {
                    *p = (*p as ::core::ffi::c_int | CSERV_FLAG_TMPMAINTENANCE) as uint8_t;
                }
            }
            i = i.wrapping_add(1);
            csptr = (*csptr).next as *mut csdbentry;
        }
        hash = hash.wrapping_add(1);
    }
    return i.wrapping_mul(recsize);
}
#[no_mangle]
pub unsafe extern "C" fn csdb_remove_server(mut ip: uint32_t, mut port: uint16_t) -> uint8_t {
    let mut hash: uint32_t = 0;
    let mut csptr: *mut csdbentry = ::core::ptr::null_mut::<csdbentry>();
    let mut cspptr: *mut *mut csdbentry = ::core::ptr::null_mut::<*mut csdbentry>();
    hash = hash32(ip ^ ((port as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as uint32_t)
        .wrapping_rem(CSDBHASHSIZE as uint32_t);
    cspptr = (&raw mut csdbhash as *mut *mut csdbentry).offset(hash as isize);
    loop {
        csptr = *cspptr;
        if csptr.is_null() {
            break;
        }
        if (*csptr).ip == ip && (*csptr).port as ::core::ffi::c_int == port as ::core::ffi::c_int {
            if !(*csptr).eptr.is_null() {
                return MFS_ERROR_ACTIVE as uint8_t;
            }
            if (*csptr).csid as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                csdb_delid((*csptr).csid);
            }
            if (*csptr).maintenance as ::core::ffi::c_int != MAINTENANCE_OFF {
                disconnected_servers_in_maintenance =
                    disconnected_servers_in_maintenance.wrapping_sub(1);
            }
            *cspptr = (*csptr).next as *mut csdbentry;
            free(csptr as *mut ::core::ffi::c_void);
            servers = servers.wrapping_sub(1);
            disconnected_servers = disconnected_servers.wrapping_sub(1);
            changelog(
                b"%u|CSDBOP(%u,%u,%hu,0)\0".as_ptr() as *const ::core::ffi::c_char,
                main_time(),
                CSDB_OP_DEL,
                ip,
                port as ::core::ffi::c_int,
            );
            return MFS_STATUS_OK as uint8_t;
        } else {
            cspptr = &raw mut (*csptr).next as *mut *mut csdbentry;
        }
    }
    return MFS_ERROR_NOTFOUND as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn csdb_remove_unused() {
    let mut hash: uint32_t = 0;
    let mut now: uint32_t = 0;
    let mut csptr: *mut csdbentry = ::core::ptr::null_mut::<csdbentry>();
    let mut cspptr: *mut *mut csdbentry = ::core::ptr::null_mut::<*mut csdbentry>();
    if SecondsToRemoveUnusedCS > 0 as uint32_t {
        now = main_time();
        hash = 0 as uint32_t;
        while hash < CSDBHASHSIZE as uint32_t {
            cspptr = (&raw mut csdbhash as *mut *mut csdbentry).offset(hash as isize);
            loop {
                csptr = *cspptr;
                if csptr.is_null() {
                    break;
                }
                if (*csptr).eptr.is_null()
                    && (*csptr)
                        .disconnection_time
                        .wrapping_add(SecondsToRemoveUnusedCS)
                        < now
                {
                    let mut ip: uint32_t = (*csptr).ip;
                    let mut port: uint16_t = (*csptr).port;
                    if (*csptr).csid as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                        csdb_delid((*csptr).csid);
                    }
                    if (*csptr).maintenance as ::core::ffi::c_int != MAINTENANCE_OFF {
                        disconnected_servers_in_maintenance =
                            disconnected_servers_in_maintenance.wrapping_sub(1);
                    }
                    *cspptr = (*csptr).next as *mut csdbentry;
                    free(csptr as *mut ::core::ffi::c_void);
                    servers = servers.wrapping_sub(1);
                    disconnected_servers = disconnected_servers.wrapping_sub(1);
                    changelog(
                        b"%u|CSDBOP(%u,%u,%hu,0)\0".as_ptr() as *const ::core::ffi::c_char,
                        main_time(),
                        CSDB_OP_DEL,
                        ip,
                        port as ::core::ffi::c_int,
                    );
                } else {
                    cspptr = &raw mut (*csptr).next as *mut *mut csdbentry;
                }
            }
            hash = hash.wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn csdb_mr_op(
    mut op: uint8_t,
    mut ip: uint32_t,
    mut port: uint16_t,
    mut arg: uint32_t,
) -> uint8_t {
    let mut hash: uint32_t = 0;
    let mut hashid: uint32_t = 0;
    let mut csptr: *mut csdbentry = ::core::ptr::null_mut::<csdbentry>();
    let mut cspptr: *mut *mut csdbentry = ::core::ptr::null_mut::<*mut csdbentry>();
    let mut csidptr: *mut csdbentry = ::core::ptr::null_mut::<csdbentry>();
    match op as ::core::ffi::c_int {
        CSDB_OP_ADD => {
            hash =
                hash32(ip ^ ((port as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as uint32_t)
                    .wrapping_rem(CSDBHASHSIZE as uint32_t);
            csptr = csdbhash[hash as usize];
            while !csptr.is_null() {
                if (*csptr).ip == ip
                    && (*csptr).port as ::core::ffi::c_int == port as ::core::ffi::c_int
                {
                    return MFS_ERROR_MISMATCH as uint8_t;
                }
                csptr = (*csptr).next as *mut csdbentry;
            }
            if arg > 65535 as uint32_t
                || arg > 0 as uint32_t && !(*csdbtab.offset(arg as isize)).is_null()
            {
                return MFS_ERROR_MISMATCH as uint8_t;
            }
            csptr = malloc(::core::mem::size_of::<csdbentry>()) as *mut csdbentry;
            if csptr.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/csdb.c\0".as_ptr() as *const ::core::ffi::c_char,
                    496 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"csptr\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/csdb.c\0".as_ptr() as *const ::core::ffi::c_char,
                    496 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"csptr\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if csptr
                == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut csdbentry
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/csdb.c\0".as_ptr() as *const ::core::ffi::c_char,
                    496 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"csptr\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/csdb.c\0".as_ptr() as *const ::core::ffi::c_char,
                    496 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"csptr\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            (*csptr).ip = ip;
            (*csptr).port = port;
            (*csptr).csid = arg as uint16_t;
            *csdbtab.offset(arg as isize) = csptr;
            (*csptr).heavyloadts = 0 as uint32_t;
            (*csptr).maintenance_timeout = 0 as uint32_t;
            (*csptr).maintenance = MAINTENANCE_OFF as uint8_t;
            (*csptr).disconnection_time = main_time();
            (*csptr).load = 0 as uint32_t;
            (*csptr).eptr = NULL;
            (*csptr).next = csdbhash[hash as usize] as *mut csdbentry;
            csdbhash[hash as usize] = csptr;
            servers = servers.wrapping_add(1);
            disconnected_servers = disconnected_servers.wrapping_add(1);
            meta_version_inc();
            return MFS_STATUS_OK as uint8_t;
        }
        CSDB_OP_DEL => {
            hash =
                hash32(ip ^ ((port as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as uint32_t)
                    .wrapping_rem(CSDBHASHSIZE as uint32_t);
            cspptr = (&raw mut csdbhash as *mut *mut csdbentry).offset(hash as isize);
            loop {
                csptr = *cspptr;
                if csptr.is_null() {
                    break;
                }
                if (*csptr).ip == ip
                    && (*csptr).port as ::core::ffi::c_int == port as ::core::ffi::c_int
                {
                    if !(*csptr).eptr.is_null() {
                        return MFS_ERROR_MISMATCH as uint8_t;
                    }
                    if (*csptr).csid as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                        csdb_delid((*csptr).csid);
                    }
                    if (*csptr).maintenance as ::core::ffi::c_int != MAINTENANCE_OFF {
                        disconnected_servers_in_maintenance =
                            disconnected_servers_in_maintenance.wrapping_sub(1);
                    }
                    *cspptr = (*csptr).next as *mut csdbentry;
                    free(csptr as *mut ::core::ffi::c_void);
                    servers = servers.wrapping_sub(1);
                    disconnected_servers = disconnected_servers.wrapping_sub(1);
                    meta_version_inc();
                    return MFS_STATUS_OK as uint8_t;
                } else {
                    cspptr = &raw mut (*csptr).next as *mut *mut csdbentry;
                }
            }
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        CSDB_OP_NEWIPPORT => {
            if arg > 65535 as uint32_t
                || arg == 0 as uint32_t
                || (*csdbtab.offset(arg as isize)).is_null()
            {
                return MFS_ERROR_MISMATCH as uint8_t;
            }
            csidptr = *csdbtab.offset(arg as isize);
            hashid = hash32(
                (*csidptr).ip
                    ^ (((*csidptr).port as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
                        as uint32_t,
            )
            .wrapping_rem(CSDBHASHSIZE as uint32_t);
            hash =
                hash32(ip ^ ((port as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as uint32_t)
                    .wrapping_rem(CSDBHASHSIZE as uint32_t);
            cspptr = (&raw mut csdbhash as *mut *mut csdbentry).offset(hashid as isize);
            loop {
                csptr = *cspptr;
                if csptr.is_null() {
                    break;
                }
                if csptr == csidptr {
                    *cspptr = (*csptr).next as *mut csdbentry;
                    (*csptr).next = csdbhash[hash as usize] as *mut csdbentry;
                    csdbhash[hash as usize] = csptr;
                    break;
                } else {
                    cspptr = &raw mut (*csptr).next as *mut *mut csdbentry;
                }
            }
            if csptr.is_null() {
                return MFS_ERROR_MISMATCH as uint8_t;
            }
            (*csptr).ip = ip;
            (*csptr).port = port;
            meta_version_inc();
            return MFS_STATUS_OK as uint8_t;
        }
        CSDB_OP_NEWID => {
            if arg > 65535 as uint32_t
                || arg == 0 as uint32_t
                || !(*csdbtab.offset(arg as isize)).is_null()
            {
                return MFS_ERROR_MISMATCH as uint8_t;
            }
            hash =
                hash32(ip ^ ((port as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as uint32_t)
                    .wrapping_rem(CSDBHASHSIZE as uint32_t);
            csptr = csdbhash[hash as usize];
            while !csptr.is_null() {
                if (*csptr).ip == ip
                    && (*csptr).port as ::core::ffi::c_int == port as ::core::ffi::c_int
                {
                    if (*csptr).csid as uint32_t != arg {
                        if (*csptr).csid as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                            csdb_delid((*csptr).csid);
                        }
                        (*csptr).csid = arg as uint16_t;
                        *csdbtab.offset(arg as isize) = csptr;
                    }
                    meta_version_inc();
                    return MFS_STATUS_OK as uint8_t;
                }
                csptr = (*csptr).next as *mut csdbentry;
            }
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        CSDB_OP_MAINTENANCEON => {
            hash =
                hash32(ip ^ ((port as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as uint32_t)
                    .wrapping_rem(CSDBHASHSIZE as uint32_t);
            csptr = csdbhash[hash as usize];
            while !csptr.is_null() {
                if (*csptr).ip == ip
                    && (*csptr).port as ::core::ffi::c_int == port as ::core::ffi::c_int
                {
                    if (*csptr).maintenance as ::core::ffi::c_int == MAINTENANCE_OFF {
                        if (*csptr).eptr.is_null() {
                            disconnected_servers_in_maintenance =
                                disconnected_servers_in_maintenance.wrapping_add(1);
                        }
                    }
                    (*csptr).maintenance = MAINTENANCE_ON as uint8_t;
                    (*csptr).maintenance_timeout = arg;
                    meta_version_inc();
                    return MFS_STATUS_OK as uint8_t;
                }
                csptr = (*csptr).next as *mut csdbentry;
            }
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        CSDB_OP_MAINTENANCEOFF => {
            hash =
                hash32(ip ^ ((port as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as uint32_t)
                    .wrapping_rem(CSDBHASHSIZE as uint32_t);
            csptr = csdbhash[hash as usize];
            while !csptr.is_null() {
                if (*csptr).ip == ip
                    && (*csptr).port as ::core::ffi::c_int == port as ::core::ffi::c_int
                {
                    if (*csptr).maintenance as ::core::ffi::c_int != MAINTENANCE_OFF {
                        if (*csptr).eptr.is_null() {
                            disconnected_servers_in_maintenance =
                                disconnected_servers_in_maintenance.wrapping_sub(1);
                        }
                    }
                    (*csptr).maintenance = MAINTENANCE_OFF as uint8_t;
                    (*csptr).maintenance_timeout = arg;
                    meta_version_inc();
                    return MFS_STATUS_OK as uint8_t;
                }
                csptr = (*csptr).next as *mut csdbentry;
            }
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        CSDB_OP_MAINTENANCETMP => {
            hash =
                hash32(ip ^ ((port as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as uint32_t)
                    .wrapping_rem(CSDBHASHSIZE as uint32_t);
            csptr = csdbhash[hash as usize];
            while !csptr.is_null() {
                if (*csptr).ip == ip
                    && (*csptr).port as ::core::ffi::c_int == port as ::core::ffi::c_int
                {
                    if (*csptr).maintenance as ::core::ffi::c_int == MAINTENANCE_OFF {
                        if (*csptr).eptr.is_null() {
                            disconnected_servers_in_maintenance =
                                disconnected_servers_in_maintenance.wrapping_add(1);
                        }
                    }
                    (*csptr).maintenance = MAINTENANCE_TMP as uint8_t;
                    (*csptr).maintenance_timeout = arg;
                    meta_version_inc();
                    return MFS_STATUS_OK as uint8_t;
                }
                csptr = (*csptr).next as *mut csdbentry;
            }
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        _ => {}
    }
    return MFS_ERROR_MISMATCH as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn csdb_back_to_work(mut ip: uint32_t, mut port: uint16_t) -> uint8_t {
    let mut hash: uint32_t = 0;
    let mut csptr: *mut csdbentry = ::core::ptr::null_mut::<csdbentry>();
    hash = hash32(ip ^ ((port as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as uint32_t)
        .wrapping_rem(CSDBHASHSIZE as uint32_t);
    csptr = csdbhash[hash as usize];
    while !csptr.is_null() {
        if (*csptr).ip == ip && (*csptr).port as ::core::ffi::c_int == port as ::core::ffi::c_int {
            (*csptr).heavyloadts = 0 as uint32_t;
            return MFS_STATUS_OK as uint8_t;
        }
        csptr = (*csptr).next as *mut csdbentry;
    }
    return MFS_ERROR_NOTFOUND as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn csdb_maintenance(
    mut ip: uint32_t,
    mut port: uint16_t,
    mut onoff: uint8_t,
) -> uint8_t {
    let mut hash: uint32_t = 0;
    let mut csptr: *mut csdbentry = ::core::ptr::null_mut::<csdbentry>();
    if onoff as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        && onoff as ::core::ffi::c_int != 1 as ::core::ffi::c_int
    {
        return MFS_ERROR_EINVAL as uint8_t;
    }
    hash = hash32(ip ^ ((port as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as uint32_t)
        .wrapping_rem(CSDBHASHSIZE as uint32_t);
    csptr = csdbhash[hash as usize];
    while !csptr.is_null() {
        if (*csptr).ip == ip && (*csptr).port as ::core::ffi::c_int == port as ::core::ffi::c_int {
            if (*csptr).maintenance as ::core::ffi::c_int != MAINTENANCE_OFF
                && onoff as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || (*csptr).maintenance as ::core::ffi::c_int == MAINTENANCE_OFF
                    && onoff as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            {
                (*csptr).maintenance = (if onoff as ::core::ffi::c_int != 0 {
                    MAINTENANCE_ON
                } else {
                    MAINTENANCE_OFF
                }) as uint8_t;
                if (*csptr).maintenance as ::core::ffi::c_int == MAINTENANCE_ON
                    && MaintenanceModeTimeout > 0 as uint32_t
                {
                    (*csptr).maintenance_timeout = main_time().wrapping_add(MaintenanceModeTimeout);
                } else {
                    (*csptr).maintenance_timeout = 0 as uint32_t;
                }
                if onoff != 0 {
                    changelog(
                        b"%u|CSDBOP(%u,%u,%hu,%u)\0".as_ptr() as *const ::core::ffi::c_char,
                        main_time(),
                        CSDB_OP_MAINTENANCEON,
                        ip,
                        port as ::core::ffi::c_int,
                        (*csptr).maintenance_timeout,
                    );
                } else {
                    changelog(
                        b"%u|CSDBOP(%u,%u,%hu,0)\0".as_ptr() as *const ::core::ffi::c_char,
                        main_time(),
                        CSDB_OP_MAINTENANCEOFF,
                        ip,
                        port as ::core::ffi::c_int,
                    );
                }
                if (*csptr).eptr.is_null() {
                    if onoff != 0 {
                        disconnected_servers_in_maintenance =
                            disconnected_servers_in_maintenance.wrapping_add(1);
                    } else {
                        disconnected_servers_in_maintenance =
                            disconnected_servers_in_maintenance.wrapping_sub(1);
                    }
                }
            }
            return MFS_STATUS_OK as uint8_t;
        }
        csptr = (*csptr).next as *mut csdbentry;
    }
    return MFS_ERROR_NOTFOUND as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn csdb_get_server_counters(
    mut servers_ptr: *mut uint32_t,
    mut disconnected_servers_ptr: *mut uint32_t,
    mut disconnected_servers_in_maintenance_ptr: *mut uint32_t,
) {
    if !servers_ptr.is_null() {
        *servers_ptr = servers;
    }
    if !disconnected_servers_ptr.is_null() {
        *disconnected_servers_ptr = disconnected_servers;
    }
    if !disconnected_servers_in_maintenance_ptr.is_null() {
        *disconnected_servers_in_maintenance_ptr = disconnected_servers_in_maintenance;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csdb_have_all_servers() -> uint8_t {
    return (if disconnected_servers > 0 as uint32_t {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    }) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn csdb_stop_chunk_jobs() -> uint8_t {
    return (if disconnected_servers > 0 as uint32_t
        && disconnected_servers == disconnected_servers_in_maintenance
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn csdb_compare(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut aa: *const csdbentry = *(a as *mut *const csdbentry);
    let mut bb: *const csdbentry = *(b as *mut *const csdbentry);
    if (*aa).ip < (*bb).ip {
        return -1 as ::core::ffi::c_int;
    } else if (*aa).ip > (*bb).ip {
        return 1 as ::core::ffi::c_int;
    } else if ((*aa).port as ::core::ffi::c_int) < (*bb).port as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    } else if (*aa).port as ::core::ffi::c_int > (*bb).port as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn csdb_sort_servers() -> uint16_t {
    let mut stab: *mut *mut csdbentry = ::core::ptr::null_mut::<*mut csdbentry>();
    let mut csptr: *mut csdbentry = ::core::ptr::null_mut::<csdbentry>();
    let mut i: uint32_t = 0;
    let mut hash: uint32_t = 0;
    stab = malloc(::core::mem::size_of::<*mut csdbentry>().wrapping_mul(servers as size_t))
        as *mut *mut csdbentry;
    i = 0 as uint32_t;
    hash = 0 as uint32_t;
    while hash < CSDBHASHSIZE as uint32_t {
        csptr = csdbhash[hash as usize];
        while !csptr.is_null() {
            if i < servers {
                *stab.offset(i as isize) = csptr;
                i = i.wrapping_add(1);
            } else {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"internal error: wrong chunk servers count !!!\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*csptr).number = 0 as uint16_t;
            }
            csptr = (*csptr).next as *mut csdbentry;
        }
        hash = hash.wrapping_add(1);
    }
    qsort(
        stab as *mut ::core::ffi::c_void,
        servers as size_t,
        ::core::mem::size_of::<*mut csdbentry>(),
        Some(
            csdb_compare
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    i = 0 as uint32_t;
    while i < servers {
        (**stab.offset(i as isize)).number = i.wrapping_add(1 as uint32_t) as uint16_t;
        i = i.wrapping_add(1);
    }
    free(stab as *mut ::core::ffi::c_void);
    return servers as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn csdb_servers_count() -> uint16_t {
    return servers as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn csdb_getnumber(mut v_csptr: *mut ::core::ffi::c_void) -> uint16_t {
    let mut csptr: *mut csdbentry = v_csptr as *mut csdbentry;
    if !csptr.is_null() {
        return (*csptr).number;
    }
    return 0 as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn csdb_store(mut fd: *mut bio) -> uint8_t {
    let mut hash: uint32_t = 0;
    let mut wbuff: [uint8_t; 13] = [0; 13];
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut csptr: *mut csdbentry = ::core::ptr::null_mut::<csdbentry>();
    let mut l: uint32_t = 0;
    l = 0 as uint32_t;
    if fd.is_null() {
        return 0x13 as uint8_t;
    }
    hash = 0 as uint32_t;
    while hash < CSDBHASHSIZE as uint32_t {
        csptr = csdbhash[hash as usize];
        while !csptr.is_null() {
            l = l.wrapping_add(1);
            csptr = (*csptr).next as *mut csdbentry;
        }
        hash = hash.wrapping_add(1);
    }
    ptr = &raw mut wbuff as *mut uint8_t;
    put32bit(&raw mut ptr, l);
    if bio_write(
        fd,
        &raw mut wbuff as *mut uint8_t as *const ::core::ffi::c_void,
        4 as uint64_t,
    ) != 4 as int64_t
    {
        return 0xff as uint8_t;
    }
    hash = 0 as uint32_t;
    while hash < CSDBHASHSIZE as uint32_t {
        csptr = csdbhash[hash as usize];
        while !csptr.is_null() {
            ptr = &raw mut wbuff as *mut uint8_t;
            put32bit(&raw mut ptr, (*csptr).ip);
            put16bit(&raw mut ptr, (*csptr).port);
            put16bit(&raw mut ptr, (*csptr).csid);
            put8bit(&raw mut ptr, (*csptr).maintenance);
            put32bit(&raw mut ptr, (*csptr).maintenance_timeout);
            if bio_write(
                fd,
                &raw mut wbuff as *mut uint8_t as *const ::core::ffi::c_void,
                13 as uint64_t,
            ) != 13 as int64_t
            {
                return 0xff as uint8_t;
            }
            csptr = (*csptr).next as *mut csdbentry;
        }
        hash = hash.wrapping_add(1);
    }
    return 0 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn csdb_load(
    mut fd: *mut bio,
    mut mver: uint8_t,
    mut ignoreflag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rbuff: [uint8_t; 13] = [0; 13];
    let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut csptr: *mut csdbentry = ::core::ptr::null_mut::<csdbentry>();
    let mut hash: uint32_t = 0;
    let mut l: uint32_t = 0;
    let mut ip: uint32_t = 0;
    let mut port: uint16_t = 0;
    let mut csid: uint16_t = 0;
    let mut maintenance: uint8_t = 0;
    let mut maintenance_timeout: uint32_t = 0;
    let mut nl: uint8_t = 1 as uint8_t;
    let mut bsize: uint32_t = 0;
    if bio_read(
        fd,
        &raw mut rbuff as *mut uint8_t as *mut ::core::ffi::c_void,
        4 as uint64_t,
    ) != 4 as int64_t
    {
        let mut err: ::core::ffi::c_int = *__errno_location();
        if nl != 0 {
            fputc('\n' as ::core::ffi::c_int, stderr);
        }
        *__errno_location() = err;
        mfs_log(
            MFSLOG_ERRNO_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"loading chunkservers: read error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    ptr = &raw mut rbuff as *mut uint8_t;
    l = get32bit(&raw mut ptr);
    if mver as ::core::ffi::c_int <= 0x10 as ::core::ffi::c_int {
        bsize = 6 as uint32_t;
    } else if mver as ::core::ffi::c_int <= 0x11 as ::core::ffi::c_int {
        bsize = 8 as uint32_t;
    } else if mver as ::core::ffi::c_int <= 0x12 as ::core::ffi::c_int {
        bsize = 9 as uint32_t;
    } else {
        bsize = 13 as uint32_t;
    }
    while l > 0 as uint32_t {
        if bio_read(
            fd,
            &raw mut rbuff as *mut uint8_t as *mut ::core::ffi::c_void,
            bsize as uint64_t,
        ) != bsize as int64_t
        {
            let mut err_0: ::core::ffi::c_int = *__errno_location();
            if nl != 0 {
                fputc('\n' as ::core::ffi::c_int, stderr);
            }
            *__errno_location() = err_0;
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"loading chunkservers: read error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = &raw mut rbuff as *mut uint8_t;
        ip = get32bit(&raw mut ptr);
        port = get16bit(&raw mut ptr);
        if mver as ::core::ffi::c_int >= 0x11 as ::core::ffi::c_int {
            csid = get16bit(&raw mut ptr);
        } else {
            csid = 0 as uint16_t;
        }
        if mver as ::core::ffi::c_int >= 0x12 as ::core::ffi::c_int {
            maintenance = get8bit(&raw mut ptr);
        } else {
            maintenance = MAINTENANCE_OFF as uint8_t;
        }
        if mver as ::core::ffi::c_int >= 0x13 as ::core::ffi::c_int {
            maintenance_timeout = get32bit(&raw mut ptr);
        } else {
            maintenance_timeout = 0 as uint32_t;
        }
        hash = hash32(ip ^ ((port as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as uint32_t)
            .wrapping_rem(CSDBHASHSIZE as uint32_t);
        csptr = csdbhash[hash as usize];
        while !csptr.is_null() {
            if (*csptr).ip == ip
                && (*csptr).port as ::core::ffi::c_int == port as ::core::ffi::c_int
            {
                if nl != 0 {
                    fputc('\n' as ::core::ffi::c_int, stderr);
                    nl = 0 as uint8_t;
                }
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"repeated chunkserver entry (ip:%u,port:%hu)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ip,
                    port as ::core::ffi::c_int,
                );
                if ignoreflag == 0 as ::core::ffi::c_int {
                    fprintf(
                        stderr,
                        b"use '-i' option to remove this chunkserver definition\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            }
            csptr = (*csptr).next as *mut csdbentry;
        }
        if csid as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            csptr = *csdbtab.offset(csid as isize);
            if !csptr.is_null() {
                if nl != 0 {
                    fputc('\n' as ::core::ffi::c_int, stderr);
                    nl = 0 as uint8_t;
                }
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"repeated chunkserver entry (csid:%hu)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    csid as ::core::ffi::c_int,
                );
                if ignoreflag == 0 as ::core::ffi::c_int {
                    fprintf(
                        stderr,
                        b"use '-i' option to remove this chunkserver definition\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            }
        }
        csptr = malloc(::core::mem::size_of::<csdbentry>()) as *mut csdbentry;
        if csptr.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/csdb.c\0".as_ptr() as *const ::core::ffi::c_char,
                957 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"csptr\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/csdb.c\0".as_ptr() as *const ::core::ffi::c_char,
                957 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"csptr\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if csptr
            == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut csdbentry
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/csdb.c\0".as_ptr() as *const ::core::ffi::c_char,
                957 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"csptr\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/csdb.c\0".as_ptr() as *const ::core::ffi::c_char,
                957 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"csptr\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*csptr).ip = ip;
        (*csptr).port = port;
        (*csptr).csid = csid;
        if csid as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            *csdbtab.offset(csid as isize) = csptr;
        }
        (*csptr).number = 0 as uint16_t;
        (*csptr).heavyloadts = 0 as uint32_t;
        (*csptr).load = 0 as uint32_t;
        (*csptr).eptr = NULL;
        (*csptr).maintenance = maintenance;
        (*csptr).maintenance_timeout = maintenance_timeout;
        (*csptr).disconnection_time = main_time();
        (*csptr).next = csdbhash[hash as usize] as *mut csdbentry;
        csdbhash[hash as usize] = csptr;
        servers = servers.wrapping_add(1);
        disconnected_servers = disconnected_servers.wrapping_add(1);
        if maintenance as ::core::ffi::c_int != MAINTENANCE_OFF {
            disconnected_servers_in_maintenance =
                disconnected_servers_in_maintenance.wrapping_add(1);
        }
        l = l.wrapping_sub(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn csdb_cleanup() {
    let mut hash: uint32_t = 0;
    let mut csptr: *mut csdbentry = ::core::ptr::null_mut::<csdbentry>();
    let mut csnptr: *mut csdbentry = ::core::ptr::null_mut::<csdbentry>();
    hash = 0 as uint32_t;
    while hash < CSDBHASHSIZE as uint32_t {
        csptr = csdbhash[hash as usize];
        while !csptr.is_null() {
            csnptr = (*csptr).next as *mut csdbentry;
            free(csptr as *mut ::core::ffi::c_void);
            csptr = csnptr;
        }
        csdbhash[hash as usize] = ::core::ptr::null_mut::<csdbentry>();
        hash = hash.wrapping_add(1);
    }
    hash = 0 as uint32_t;
    while hash < 65536 as uint32_t {
        *csdbtab.offset(hash as isize) = ::core::ptr::null_mut::<csdbentry>();
        hash = hash.wrapping_add(1);
    }
    nextid = 1 as uint32_t;
    disconnected_servers = 0 as uint32_t;
    disconnected_servers_in_maintenance = 0 as uint32_t;
    servers = 0 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn csdb_reload() {
    let mut dtr: uint32_t = 0;
    HeavyLoadGracePeriod = cfg_getuint32(
        b"CS_HEAVY_LOAD_GRACE_PERIOD\0".as_ptr() as *const ::core::ffi::c_char,
        900 as uint32_t,
    );
    HeavyLoadThreshold = cfg_getuint32(
        b"CS_HEAVY_LOAD_THRESHOLD\0".as_ptr() as *const ::core::ffi::c_char,
        150 as uint32_t,
    );
    HeavyLoadRatioThreshold = cfg_getdouble(
        b"CS_HEAVY_LOAD_RATIO_THRESHOLD\0".as_ptr() as *const ::core::ffi::c_char,
        3.0f64,
    );
    MaintenanceModeTimeout = cfg_getsperiod(
        b"CS_MAINTENANCE_MODE_TIMEOUT\0".as_ptr() as *const ::core::ffi::c_char,
        b"0\0".as_ptr() as *const ::core::ffi::c_char,
    );
    TempMaintenanceModeTimeout = cfg_getsperiod(
        b"CS_TEMP_MAINTENANCE_MODE_TIMEOUT\0".as_ptr() as *const ::core::ffi::c_char,
        b"30m\0".as_ptr() as *const ::core::ffi::c_char,
    );
    dtr = cfg_getuint32(
        b"CS_DAYS_TO_REMOVE_UNUSED\0".as_ptr() as *const ::core::ffi::c_char,
        7 as uint32_t,
    );
    if dtr > 365 as uint32_t {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"CS_DAYS_TO_REMOVE_UNUSED - value is too big (max=365) - use zero for infinite value\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        dtr = 0 as uint32_t;
    }
    SecondsToRemoveUnusedCS = dtr.wrapping_mul(86400 as uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn csdb_init() -> ::core::ffi::c_int {
    let mut hash: uint32_t = 0;
    csdb_reload();
    hash = 0 as uint32_t;
    while hash < CSDBHASHSIZE as uint32_t {
        csdbhash[hash as usize] = ::core::ptr::null_mut::<csdbentry>();
        hash = hash.wrapping_add(1);
    }
    csdbtab = malloc(
        ::core::mem::size_of::<*mut csdbentry>()
            .wrapping_mul(65536 as ::core::ffi::c_int as size_t),
    ) as *mut *mut csdbentry;
    if csdbtab.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/csdb.c\0".as_ptr() as *const ::core::ffi::c_char,
            1032 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"csdbtab\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/csdb.c\0".as_ptr() as *const ::core::ffi::c_char,
            1032 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"csdbtab\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if csdbtab
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut *mut csdbentry
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/csdb.c\0".as_ptr() as *const ::core::ffi::c_char,
            1032 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"csdbtab\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/csdb.c\0".as_ptr() as *const ::core::ffi::c_char,
            1032 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"csdbtab\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    hash = 0 as uint32_t;
    while hash < 65536 as uint32_t {
        *csdbtab.offset(hash as isize) = ::core::ptr::null_mut::<csdbentry>();
        hash = hash.wrapping_add(1);
    }
    nextid = 1 as uint32_t;
    disconnected_servers = 0 as uint32_t;
    disconnected_servers_in_maintenance = 0 as uint32_t;
    servers = 0 as uint32_t;
    loadsum = 0 as uint32_t;
    main_reload_register_fname(
        Some(csdb_reload as unsafe extern "C" fn() -> ()),
        b"csdb_reload\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_time_register_fname(
        1 as uint32_t,
        0 as uint32_t,
        Some(csdb_self_check as unsafe extern "C" fn() -> ()),
        b"csdb_self_check\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_time_register_fname(
        600 as uint32_t,
        300 as uint32_t,
        Some(csdb_remove_unused as unsafe extern "C" fn() -> ()),
        b"csdb_remove_unused\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
