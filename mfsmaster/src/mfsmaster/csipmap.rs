pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    unsafe fn __getdelim(
        __lineptr: *mut *mut ::core::ffi::c_char,
        __n: *mut size_t,
        __delimiter: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
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
pub type uint8_t = u8;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ipclass {
    pub fromip: uint32_t,
    pub toip: uint32_t,
    pub section: uint8_t,
    pub next: *mut _ipclass,
}
pub type ipclass = _ipclass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ipmap {
    pub srcip: uint32_t,
    pub dstip: uint32_t,
    pub section: uint8_t,
    pub next: *mut _ipmap,
}
pub type ipmap = _ipmap;
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut ::core::ffi::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    unsafe {
        return __getdelim(__lineptr, __n, '\n' as ::core::ffi::c_int, __stream);
    }
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_ERRNO_SYSLOG_STDERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MAX_SECTIONS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const LSTATE_START: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LSTATE_CLASS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LSTATE_MAP: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const HASHBITS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const HASHSIZE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << HASHBITS;
pub const HASHMASK: ::core::ffi::c_int = HASHSIZE - 1 as ::core::ffi::c_int;
static mut ipclass_head: *mut ipclass = ::core::ptr::null_mut::<ipclass>();
static mut load_head: *mut ipclass = ::core::ptr::null_mut::<ipclass>();
static mut current_section: uint8_t = 0;
static mut lstate: uint8_t = 0;
static mut ipmap_hashtab: [*mut ipmap; 1024] = [::core::ptr::null_mut::<ipmap>(); 1024];
static mut load_hashtab: [*mut ipmap; 1024] = [::core::ptr::null_mut::<ipmap>(); 1024];
#[inline]
unsafe extern "C" fn csipmap_hash(mut ip: uint32_t, mut section: uint8_t) -> uint32_t {
    return (ip
        ^ (ip >> HASHBITS).wrapping_mul(17 as uint32_t)
        ^ (section as uint32_t).wrapping_mul(33 as uint32_t))
        & HASHMASK as uint32_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn csipmap_map(mut servip: uint32_t, mut clientip: uint32_t) -> uint32_t {
    unsafe {
        let mut ipcp: *mut ipclass = ::core::ptr::null_mut::<ipclass>();
        let mut ipmp: *mut ipmap = ::core::ptr::null_mut::<ipmap>();
        let mut hash: uint32_t = 0;
        ipcp = ipclass_head;
        while !ipcp.is_null() {
            if clientip >= (*ipcp).fromip && clientip <= (*ipcp).toip {
                break;
            }
            ipcp = (*ipcp).next as *mut ipclass;
        }
        if ipcp.is_null() {
            return 0 as uint32_t;
        }
        hash = csipmap_hash(servip, (*ipcp).section);
        ipmp = ipmap_hashtab[hash as usize];
        while !ipmp.is_null() {
            if (*ipmp).srcip == servip
                && (*ipmp).section as ::core::ffi::c_int == (*ipcp).section as ::core::ffi::c_int
            {
                return (*ipmp).dstip;
            }
            ipmp = (*ipmp).next as *mut ipmap;
        }
        return 0 as uint32_t;
    }
}
unsafe extern "C" fn csipmap_newrange(mut fromip: uint32_t, mut toip: uint32_t) {
    unsafe {
        let mut ipcp: *mut ipclass = ::core::ptr::null_mut::<ipclass>();
        if lstate as ::core::ffi::c_int == LSTATE_MAP {
            current_section = current_section.wrapping_add(1);
        }
        lstate = LSTATE_CLASS as uint8_t;
        ipcp = malloc(::core::mem::size_of::<ipclass>()) as *mut ipclass;
        (*ipcp).fromip = fromip;
        (*ipcp).toip = toip;
        (*ipcp).section = current_section;
        (*ipcp).next = load_head as *mut _ipclass;
        load_head = ipcp;
    }
}
unsafe extern "C" fn csipmap_newclass(mut ip: uint32_t, mut mask: uint32_t) {
    unsafe {
        csipmap_newrange(ip & mask, ip | !mask);
    }
}
unsafe extern "C" fn csipmap_newmap(mut src: uint32_t, mut dst: uint32_t) {
    unsafe {
        let mut ipmp: *mut ipmap = ::core::ptr::null_mut::<ipmap>();
        let mut hash: uint32_t = 0;
        lstate = LSTATE_MAP as uint8_t;
        hash = csipmap_hash(src, current_section);
        ipmp = load_hashtab[hash as usize];
        while !ipmp.is_null() {
            if (*ipmp).srcip == src
                && (*ipmp).section as ::core::ffi::c_int == current_section as ::core::ffi::c_int
            {
                (*ipmp).dstip = dst;
                return;
            }
            ipmp = (*ipmp).next as *mut ipmap;
        }
        ipmp = malloc(::core::mem::size_of::<ipmap>()) as *mut ipmap;
        (*ipmp).srcip = src;
        (*ipmp).dstip = dst;
        (*ipmp).section = current_section;
        (*ipmp).next = load_hashtab[hash as usize] as *mut _ipmap;
        load_hashtab[hash as usize] = ipmp;
    }
}
pub const PARSE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OCTET_TOOBIG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TOO_MANY_OCTESTS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BITS_TOOBIG: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const WRONG_MASK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const BAD_ORDER: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const BAD_SECTION_ORDER: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const PARSE_ERROR: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
unsafe extern "C" fn csipmap_parseline(mut str: *const ::core::ffi::c_char) -> uint8_t {
    unsafe {
        let mut fip: uint32_t = 0;
        let mut c: ::core::ffi::c_char = 0;
        let mut octet: uint32_t = 0;
        let mut ip: uint32_t = 0;
        let mut octcnt: uint32_t = 0;
        let mut state: uint32_t = 0;
        octet = 0 as uint32_t;
        octcnt = 0 as uint32_t;
        state = 0 as uint32_t;
        ip = 0 as uint32_t;
        fip = 0 as uint32_t;
        loop {
            let c2rust_fresh0 = str;
            str = str.offset(1);
            c = *c2rust_fresh0;
            if c as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                break;
            }
            if c as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                || c as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
                || c as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || c as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                continue;
            }
            if c as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
            {
                octet = octet.wrapping_mul(10 as uint32_t);
                octet = octet.wrapping_add(
                    (c as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as uint32_t,
                );
                if octet > 255 as uint32_t {
                    return OCTET_TOOBIG as uint8_t;
                }
            } else {
                if c as ::core::ffi::c_int == '#' as ::core::ffi::c_int {
                    break;
                }
                if c as ::core::ffi::c_int == '.' as ::core::ffi::c_int {
                    octcnt = octcnt.wrapping_add(1);
                    if octcnt >= 4 as uint32_t {
                        return TOO_MANY_OCTESTS as uint8_t;
                    }
                    ip = ip.wrapping_mul(256 as uint32_t);
                    ip = ip.wrapping_add(octet);
                    octet = 0 as uint32_t;
                } else if state == 0 as uint32_t {
                    if c as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
                        while octcnt < 4 as uint32_t {
                            ip = ip.wrapping_mul(256 as uint32_t);
                            ip = ip.wrapping_add(octet);
                            octet = 0 as uint32_t;
                            octcnt = octcnt.wrapping_add(1);
                        }
                        state = 1 as uint32_t;
                        fip = ip;
                        ip = 0 as uint32_t;
                        octcnt = 0 as uint32_t;
                    } else if c as ::core::ffi::c_int == '-' as ::core::ffi::c_int
                        && octcnt == 3 as uint32_t
                    {
                        ip = ip.wrapping_mul(256 as uint32_t);
                        ip = ip.wrapping_add(octet);
                        octet = 0 as uint32_t;
                        state = 2 as uint32_t;
                        fip = ip;
                        ip = 0 as uint32_t;
                        octcnt = 0 as uint32_t;
                    } else if c as ::core::ffi::c_int == ':' as ::core::ffi::c_int
                        && octcnt == 3 as uint32_t
                    {
                        ip = ip.wrapping_mul(256 as uint32_t);
                        ip = ip.wrapping_add(octet);
                        octet = 0 as uint32_t;
                        state = 3 as uint32_t;
                        fip = ip;
                        ip = 0 as uint32_t;
                        octcnt = 0 as uint32_t;
                    } else {
                        return PARSE_ERROR as uint8_t;
                    }
                } else {
                    return PARSE_ERROR as uint8_t;
                }
            }
        }
        if state == 0 as uint32_t
            && octet == 0 as uint32_t
            && ip == 0 as uint32_t
            && octcnt == 0 as uint32_t
        {
            return PARSE_OK as uint8_t;
        }
        if state == 0 as uint32_t {
            if octcnt == 3 as uint32_t {
                ip = ip.wrapping_mul(256 as uint32_t);
                ip = ip.wrapping_add(octet);
                csipmap_newrange(ip, ip);
            } else {
                return PARSE_ERROR as uint8_t;
            }
        } else if state == 1 as uint32_t {
            if octcnt == 0 as uint32_t {
                if octet > 32 as uint32_t {
                    return BITS_TOOBIG as uint8_t;
                }
                ip = ((0xffffffff as ::core::ffi::c_uint) << (32 as uint32_t).wrapping_sub(octet))
                    as uint32_t;
                csipmap_newclass(fip, ip);
            } else if octcnt == 3 as uint32_t {
                ip = ip.wrapping_mul(256 as uint32_t);
                ip = ip.wrapping_add(octet);
                csipmap_newclass(fip, ip);
                while ip & 0x80000000 as uint32_t != 0 {
                    ip <<= 1 as ::core::ffi::c_int;
                }
                if ip != 0 as uint32_t {
                    return WRONG_MASK as uint8_t;
                }
            } else {
                return PARSE_ERROR as uint8_t;
            }
        } else if state == 2 as uint32_t {
            if octcnt == 3 as uint32_t {
                ip = ip.wrapping_mul(256 as uint32_t);
                ip = ip.wrapping_add(octet);
                if ip < fip {
                    return BAD_ORDER as uint8_t;
                }
                csipmap_newrange(fip, ip);
            } else {
                return PARSE_ERROR as uint8_t;
            }
        } else if state == 3 as uint32_t {
            if octcnt == 3 as uint32_t {
                ip = ip.wrapping_mul(256 as uint32_t);
                ip = ip.wrapping_add(octet);
                if lstate as ::core::ffi::c_int == LSTATE_START {
                    return BAD_SECTION_ORDER as uint8_t;
                }
                csipmap_newmap(fip, ip);
            } else {
                return PARSE_ERROR as uint8_t;
            }
        } else {
            return PARSE_ERROR as uint8_t;
        }
        return PARSE_OK as uint8_t;
    }
}
unsafe extern "C" fn csipmap_cleanup(mut load_flag: uint8_t) {
    unsafe {
        let mut ipcp: *mut ipclass = ::core::ptr::null_mut::<ipclass>();
        let mut ipcpn: *mut ipclass = ::core::ptr::null_mut::<ipclass>();
        let mut ipmp: *mut ipmap = ::core::ptr::null_mut::<ipmap>();
        let mut ipmpn: *mut ipmap = ::core::ptr::null_mut::<ipmap>();
        let mut hash: uint32_t = 0;
        ipcp = if load_flag as ::core::ffi::c_int != 0 {
            load_head
        } else {
            ipclass_head
        };
        while !ipcp.is_null() {
            ipcpn = (*ipcp).next as *mut ipclass;
            free(ipcp as *mut ::core::ffi::c_void);
            ipcp = ipcpn;
        }
        if load_flag != 0 {
            load_head = ::core::ptr::null_mut::<ipclass>();
        } else {
            ipclass_head = ::core::ptr::null_mut::<ipclass>();
        }
        hash = 0 as uint32_t;
        while hash < HASHSIZE as uint32_t {
            ipmp = *if load_flag as ::core::ffi::c_int != 0 {
                &raw mut load_hashtab as *mut *mut ipmap
            } else {
                &raw mut ipmap_hashtab as *mut *mut ipmap
            }
            .offset(hash as isize);
            while !ipmp.is_null() {
                ipmpn = (*ipmp).next as *mut ipmap;
                free(ipmp as *mut ::core::ffi::c_void);
                ipmp = ipmpn;
            }
            *if load_flag as ::core::ffi::c_int != 0 {
                &raw mut load_hashtab as *mut *mut ipmap
            } else {
                &raw mut ipmap_hashtab as *mut *mut ipmap
            }
            .offset(hash as isize) = ::core::ptr::null_mut::<ipmap>();
            hash = hash.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn csipmap_use_loaded_map() {
    unsafe {
        let mut hash: uint32_t = 0;
        ipclass_head = load_head;
        load_head = ::core::ptr::null_mut::<ipclass>();
        hash = 0 as uint32_t;
        while hash < HASHSIZE as uint32_t {
            ipmap_hashtab[hash as usize] = load_hashtab[hash as usize];
            load_hashtab[hash as usize] = ::core::ptr::null_mut::<ipmap>();
            hash = hash.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn csipmap_loadmap(mut fname: *const ::core::ffi::c_char) {
    unsafe {
        let mut fd: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut linebuff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut lbsize: size_t = 0;
        let mut status: uint8_t = 0;
        let mut lno: uint32_t = 0;
        fd = fopen(fname, b"r\0".as_ptr() as *const ::core::ffi::c_char) as *mut FILE;
        if fd.is_null() {
            if *__errno_location() == ENOENT {
                if !ipclass_head.is_null() {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"mfscsipmap configuration file (%s) not found - chunkserver ip mappings not changed\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        fname,
                    );
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_NOTICE,
                        b"mfscsipmap configuration file (%s) not found - no chunkserver ip mappings\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        fname,
                    );
                }
            } else if !ipclass_head.is_null() {
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"can't open mfscsipmap configuration file (%s) - chunkserver ip mappings not changed, error\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    fname,
                );
            } else {
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"can't open mfscsipmap configuration file (%s) - no chunkserver ip mappings, error\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    fname,
                );
            }
            return;
        }
        current_section = 0 as uint8_t;
        lstate = LSTATE_START as uint8_t;
        lbsize = 10000 as size_t;
        lno = 1 as uint32_t;
        linebuff = malloc(lbsize) as *mut ::core::ffi::c_char;
        while getline(&raw mut linebuff, &raw mut lbsize, fd) != -1 as __ssize_t {
            status = csipmap_parseline(linebuff);
            match status as ::core::ffi::c_int {
                OCTET_TOOBIG => {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"error parsing ip number in file %s (line %u) - octet too big\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        fname,
                        lno,
                    );
                }
                TOO_MANY_OCTESTS => {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"error parsing ip number in file %s (line %u) - too many octets\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        fname,
                        lno,
                    );
                }
                BITS_TOOBIG => {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"error parsing ip class in file %s (line %u) - too many bits\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        fname,
                        lno,
                    );
                }
                WRONG_MASK => {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"error parsing ip class in file %s (line %u) - wrong mask\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        fname,
                        lno,
                    );
                }
                BAD_ORDER => {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"error parsing ip range in file %s (line %u) - incorrect ip order\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        fname,
                        lno,
                    );
                }
                BAD_SECTION_ORDER => {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"error parsing line in file %s (line %u) - ip mapping without ip class or ip range\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        fname,
                        lno,
                    );
                }
                PARSE_ERROR => {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"error parsing line in file %s (line %u)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        fname,
                        lno,
                    );
                }
                _ => {}
            }
            if current_section as ::core::ffi::c_int >= MAX_SECTIONS {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"error parsing line in file %s (line %u) - too many sections in file\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    fname,
                    lno,
                );
                csipmap_cleanup(1 as uint8_t);
                fclose(fd);
                return;
            }
            if status as ::core::ffi::c_int != PARSE_OK {
                csipmap_cleanup(1 as uint8_t);
                fclose(fd);
                return;
            }
            lno = lno.wrapping_add(1);
        }
        fclose(fd);
        if lstate as ::core::ffi::c_int == LSTATE_CLASS {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"error parsing line in file %s (end of file) - ip class / ip range without mappings\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                fname,
            );
            csipmap_cleanup(1 as uint8_t);
            return;
        }
        current_section = current_section.wrapping_add(1);
        csipmap_cleanup(0 as uint8_t);
        csipmap_use_loaded_map();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn csipmap_term() {
    unsafe {
        csipmap_cleanup(0 as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn csipmap_init() -> ::core::ffi::c_int {
    unsafe {
        let mut hash: uint32_t = 0;
        ipclass_head = ::core::ptr::null_mut::<ipclass>();
        load_head = ::core::ptr::null_mut::<ipclass>();
        hash = 0 as uint32_t;
        while hash < HASHSIZE as uint32_t {
            ipmap_hashtab[hash as usize] = ::core::ptr::null_mut::<ipmap>();
            load_hashtab[hash as usize] = ::core::ptr::null_mut::<ipmap>();
            hash = hash.wrapping_add(1);
        }
        return 0 as ::core::ffi::c_int;
    }
}
