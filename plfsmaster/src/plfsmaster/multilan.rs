pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
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
    unsafe fn cfg_isdefined(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn cfg_getstr(
        name: *const ::core::ffi::c_char,
        def: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn cfg_getuint8(name: *const ::core::ffi::c_char, def: uint8_t) -> uint8_t;
    unsafe fn main_destruct_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn main_reload_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn csipmap_map(servip: uint32_t, clientip: uint32_t) -> uint32_t;
    unsafe fn csipmap_loadmap(fname: *const ::core::ffi::c_char);
    unsafe fn csipmap_term();
    unsafe fn csipmap_init() -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type uint8_t = u8;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut MultiLanMask: uint32_t = 0 as uint32_t;
static mut MultiLanClasses: uint32_t = 0 as uint32_t;
static mut MultiLanClassTab: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
#[unsafe(no_mangle)]
pub unsafe extern "C" fn multilan_map(mut servip: uint32_t, mut clientip: uint32_t) -> uint32_t {
    unsafe {
        let mut vmask: uint8_t = 0 as uint8_t;
        let mut mapip: uint32_t = 0;
        let mut i: uint32_t = 0;
        if clientip == 0 as uint32_t {
            return servip;
        }
        mapip = csipmap_map(servip, clientip);
        if mapip != 0 as uint32_t {
            return mapip;
        }
        if MultiLanMask == 0 as uint32_t {
            return servip;
        }
        i = 0 as uint32_t;
        while i < MultiLanClasses && vmask as ::core::ffi::c_int != 3 as ::core::ffi::c_int {
            if clientip & MultiLanMask == *MultiLanClassTab.offset(i as isize) & MultiLanMask {
                vmask = (vmask as ::core::ffi::c_int | 1 as ::core::ffi::c_int) as uint8_t;
            }
            if servip & MultiLanMask == *MultiLanClassTab.offset(i as isize) & MultiLanMask {
                vmask = (vmask as ::core::ffi::c_int | 2 as ::core::ffi::c_int) as uint8_t;
            }
            i = i.wrapping_add(1);
        }
        if vmask as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            return clientip & MultiLanMask | servip & !MultiLanMask;
        }
        return servip;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn multilan_match(
    mut servip: uint32_t,
    mut iptab: *mut uint32_t,
    mut iptablen: uint32_t,
) -> uint32_t {
    unsafe {
        let mut i: uint32_t = 0;
        let mut j: uint32_t = 0;
        let mut res: uint32_t = 0;
        let mut f: uint8_t = 0;
        if MultiLanMask == 0 as uint32_t || iptablen == 0 as uint32_t {
            return servip;
        }
        f = 0 as uint8_t;
        i = 0 as uint32_t;
        while i < MultiLanClasses && f as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if servip & MultiLanMask == *MultiLanClassTab.offset(i as isize) & MultiLanMask {
                f = 1 as uint8_t;
            }
            i = i.wrapping_add(1);
        }
        if f as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return servip;
        }
        res = 0 as uint32_t;
        j = 0 as uint32_t;
        while j < iptablen {
            if *iptab.offset(j as isize) & !MultiLanMask == servip & !MultiLanMask {
                i = 0 as uint32_t;
                while i < MultiLanClasses {
                    if *iptab.offset(j as isize) & MultiLanMask
                        == *MultiLanClassTab.offset(i as isize) & MultiLanMask
                    {
                        if res == 0 as uint32_t {
                            res = *iptab.offset(j as isize);
                        } else {
                            return servip;
                        }
                    }
                    i = i.wrapping_add(1);
                }
            }
            j = j.wrapping_add(1);
        }
        if res == 0 as uint32_t {
            return servip;
        }
        return res;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn multilan_term() {
    unsafe {
        if !MultiLanClassTab.is_null() {
            free(MultiLanClassTab as *mut ::core::ffi::c_void);
        }
        csipmap_term();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn multilan_parse_netlist(
    mut netliststr: *const ::core::ffi::c_char,
    mut commonmask: uint32_t,
    mut nets: *mut uint32_t,
    mut nettab: *mut *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut rptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut c: ::core::ffi::c_char = 0;
        let mut ip: uint32_t = 0;
        let mut octet: uint32_t = 0;
        let mut i: uint32_t = 0;
        let mut j: uint32_t = 0;
        let mut n: uint32_t = 0;
        let mut t: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        n = 1 as uint32_t;
        rptr = netliststr;
        while *rptr != 0 {
            c = *rptr;
            if c as ::core::ffi::c_int == ',' as ::core::ffi::c_int
                || c as ::core::ffi::c_int == ';' as ::core::ffi::c_int
            {
                n = n.wrapping_add(1);
            }
            rptr = rptr.offset(1);
        }
        t = malloc(::core::mem::size_of::<uint32_t>().wrapping_mul(n as size_t)) as *mut uint32_t;
        if t.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/multilan.c\0".as_ptr() as *const ::core::ffi::c_char,
                137 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"t\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/multilan.c\0".as_ptr() as *const ::core::ffi::c_char,
                137 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"t\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if t
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut uint32_t
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/multilan.c\0".as_ptr() as *const ::core::ffi::c_char,
                137 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"t\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/multilan.c\0".as_ptr() as *const ::core::ffi::c_char,
                137 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"t\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        j = 0 as uint32_t;
        i = 0 as uint32_t;
        ip = 0 as uint32_t;
        octet = 0 as uint32_t;
        rptr = netliststr;
        while j < n {
            c = *rptr;
            if c as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                c = ',' as ::core::ffi::c_char;
            }
            if !(c as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                || c as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
                || c as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || c as ::core::ffi::c_int == '\n' as ::core::ffi::c_int)
            {
                if c as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && c as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    octet = octet.wrapping_mul(10 as uint32_t);
                    octet = octet.wrapping_add(
                        (c as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as uint32_t,
                    );
                    if octet > 255 as uint32_t {
                        free(t as *mut ::core::ffi::c_void);
                        return 1 as uint8_t;
                    }
                } else {
                    if c as ::core::ffi::c_int == '.' as ::core::ffi::c_int
                        || c as ::core::ffi::c_int == ',' as ::core::ffi::c_int
                        || c as ::core::ffi::c_int == ';' as ::core::ffi::c_int
                    {
                        i = i.wrapping_add(1);
                        if i > 4 as uint32_t {
                            free(t as *mut ::core::ffi::c_void);
                            return 2 as uint8_t;
                        }
                        ip = ip.wrapping_mul(256 as uint32_t);
                        ip = ip.wrapping_add(octet);
                        octet = 0 as uint32_t;
                    }
                    if c as ::core::ffi::c_int == ',' as ::core::ffi::c_int
                        || c as ::core::ffi::c_int == ';' as ::core::ffi::c_int
                    {
                        while i < 4 as uint32_t {
                            i = i.wrapping_add(1);
                            ip = ip.wrapping_mul(256 as uint32_t);
                        }
                        if ip & commonmask != ip {
                            free(t as *mut ::core::ffi::c_void);
                            return 3 as uint8_t;
                        }
                        if ip == 0 as uint32_t {
                            free(t as *mut ::core::ffi::c_void);
                            return 4 as uint8_t;
                        }
                        *t.offset(j as isize) = ip;
                        ip = 0 as uint32_t;
                        i = 0 as uint32_t;
                        j = j.wrapping_add(1);
                    }
                    if *rptr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        break;
                    }
                }
            }
            rptr = rptr.offset(1);
        }
        if j < n {
            free(t as *mut ::core::ffi::c_void);
            return 5 as uint8_t;
        }
        *nets = n;
        *nettab = t;
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn multilan_reload() {
    unsafe {
        let mut bits: uint8_t = 0;
        let mut err: uint8_t = 0;
        let mut netstr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut ipmapfname: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut mask: uint32_t = 0;
        let mut ipclass: uint32_t = 0;
        let mut i: uint32_t = 0;
        let mut nets: uint32_t = 0;
        let mut nettab: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        if cfg_isdefined(b"MULTILAN_BITS\0".as_ptr() as *const ::core::ffi::c_char) != 0
            && cfg_isdefined(b"MULTILAN_CLASSES\0".as_ptr() as *const ::core::ffi::c_char) != 0
        {
            bits = cfg_getuint8(
                b"MULTILAN_BITS\0".as_ptr() as *const ::core::ffi::c_char,
                24 as uint8_t,
            );
            netstr = cfg_getstr(
                b"MULTILAN_CLASSES\0".as_ptr() as *const ::core::ffi::c_char,
                b"192.168.15.0, 10.10.10.0, 172.16.5.0\0".as_ptr() as *const ::core::ffi::c_char,
            );
            err = 0 as uint8_t;
            if bits as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || bits as ::core::ffi::c_int > 32 as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"wrong value for MULTILAN_BITS (%hhu ; shlould be between 1 and 32)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    bits as ::core::ffi::c_int,
                );
                err = 1 as uint8_t;
                mask = 0 as uint32_t;
            } else {
                mask = ((0xffffffff as ::core::ffi::c_uint)
                    << 32 as ::core::ffi::c_int - bits as ::core::ffi::c_int)
                    as uint32_t;
            }
            if err as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                err = multilan_parse_netlist(netstr, mask, &raw mut nets, &raw mut nettab);
                match err as ::core::ffi::c_int {
                    1 => {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"error parsing ip class from MULTILAN_CLASSES - octet>255 (%s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            netstr,
                        );
                    }
                    2 => {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"error parsing ip class from MULTILAN_CLASSES - too many octets (%s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            netstr,
                        );
                    }
                    3 => {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"error parsing ip class from MULTILAN_CLASSES - garbage bits at the end of ip class (%s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            netstr,
                        );
                    }
                    4 => {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"error parsing ip class from MULTILAN_CLASSES - found empty class (%s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            netstr,
                        );
                    }
                    5 => {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_WARNING,
                            b"error parsing ip class from MULTILAN_CLASSES (%s)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            netstr,
                        );
                    }
                    _ => {}
                }
            }
            if err as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if !MultiLanClassTab.is_null() {
                    free(MultiLanClassTab as *mut ::core::ffi::c_void);
                }
                MultiLanMask = mask;
                MultiLanClasses = nets;
                MultiLanClassTab = nettab;
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_INFO,
                    b"accepted %u lans for multilan configuration with %u bits (mask: %u.%u.%u.%u)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    MultiLanClasses,
                    bits as ::core::ffi::c_int,
                    MultiLanMask >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                    MultiLanMask >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                    MultiLanMask >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                    MultiLanMask & 0xff as uint32_t,
                );
                i = 0 as uint32_t;
                while i < MultiLanClasses {
                    ipclass = *MultiLanClassTab.offset(i as isize) & MultiLanMask;
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_INFO,
                        b"class %u: %u.%u.%u.%u\0".as_ptr() as *const ::core::ffi::c_char,
                        i,
                        ipclass >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                        ipclass >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                        ipclass >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                        ipclass & 0xff as uint32_t,
                    );
                    i = i.wrapping_add(1);
                }
            }
            free(netstr as *mut ::core::ffi::c_void);
        } else {
            if !MultiLanClassTab.is_null() {
                free(MultiLanClassTab as *mut ::core::ffi::c_void);
            }
            MultiLanMask = 0 as uint32_t;
            MultiLanClasses = 0 as uint32_t;
            MultiLanClassTab = ::core::ptr::null_mut::<uint32_t>();
        }
        ipmapfname = cfg_getstr(
            b"MULTILAN_IPMAP_FILENAME\0".as_ptr() as *const ::core::ffi::c_char,
            b"/usr/local/etc/mfs/mfsipmap.cfg\0".as_ptr() as *const ::core::ffi::c_char,
        );
        csipmap_loadmap(ipmapfname);
        free(ipmapfname as *mut ::core::ffi::c_void);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn multilan_init() -> ::core::ffi::c_int {
    unsafe {
        if csipmap_init() == 0 as ::core::ffi::c_int {
            multilan_reload();
            main_reload_register_fname(
                Some(multilan_reload as unsafe extern "C" fn() -> ()),
                b"multilan_reload\0".as_ptr() as *const ::core::ffi::c_char,
            );
            main_destruct_register_fname(
                Some(multilan_term as unsafe extern "C" fn() -> ()),
                b"multilan_term\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_int;
        }
        return 1 as ::core::ffi::c_int;
    }
}
