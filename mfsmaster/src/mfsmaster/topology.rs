pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
    static mut stderr: *mut FILE;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn __getdelim(
        __lineptr: *mut *mut ::core::ffi::c_char,
        __n: *mut size_t,
        __delimiter: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    unsafe fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn itree_rebalance(o: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    unsafe fn itree_add_interval(
        o: *mut ::core::ffi::c_void,
        f: uint32_t,
        t: uint32_t,
        id: uint32_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn itree_find(o: *mut ::core::ffi::c_void, v: uint32_t) -> uint32_t;
    unsafe fn itree_freeall(o: *mut ::core::ffi::c_void);
    unsafe fn main_destruct_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn main_reload_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn cfg_isdefined(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn cfg_use_option(name: *const ::core::ffi::c_char, value: *const ::core::ffi::c_char);
    unsafe fn cfg_getstr(
        name: *const ::core::ffi::c_char,
        def: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
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
pub struct _rackhashentry {
    pub rackname: *mut ::core::ffi::c_char,
    pub rackid: uint32_t,
    pub hash: uint32_t,
    pub next: *mut _rackhashentry,
}
pub type rackhashentry = _rackhashentry;
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
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_ERRNO_SYSLOG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_ERRNO_SYSLOG_STDERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn mfsrealloc(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut pptr: *mut ::core::ffi::c_void = realloc(ptr, size);
        if pptr.is_null() {
            free(ptr);
        }
        return pptr;
    }
}
#[inline]
unsafe extern "C" fn hashstr_poly(
    mut key: *const ::core::ffi::c_char,
    mut hash: uint32_t,
) -> uint32_t {
    unsafe {
        let mut p: uint8_t = 0;
        hash = (hash as ::core::ffi::c_uint ^ 1000000007 as ::core::ffi::c_uint) as uint32_t;
        loop {
            p = *key as uint8_t;
            if p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                break;
            }
            key = key.offset(1);
            hash = hash
                .wrapping_mul(29791 as uint32_t)
                .wrapping_add(p as uint32_t);
        }
        return hash;
    }
}
static mut racktree: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut TopologyFileName: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
pub const HASHTABSIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
static mut rackhashtab: [*mut rackhashentry; 4096] =
    [::core::ptr::null_mut::<rackhashentry>(); 4096];
static mut rackidtab: *mut *mut rackhashentry = ::core::ptr::null_mut::<*mut rackhashentry>();
static mut rackidtabsize: uint32_t = 0 as uint32_t;
static mut rackidnext: uint32_t = 0 as uint32_t;
#[inline]
unsafe extern "C" fn topology_rackname_hash(mut rackname: *const ::core::ffi::c_char) -> uint32_t {
    unsafe {
        return hashstr_poly(rackname, 0 as uint32_t);
    }
}
#[inline]
unsafe extern "C" fn topology_get_next_free_rackid() -> uint32_t {
    unsafe {
        let mut i: uint32_t = 0;
        i = rackidtabsize;
        if rackidtabsize == 0 as uint32_t {
            rackidtabsize = 1024 as uint32_t;
            rackidtab = malloc(
                ::core::mem::size_of::<*mut rackhashentry>().wrapping_mul(rackidtabsize as size_t),
            ) as *mut *mut rackhashentry;
            if rackidtab.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/topology.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    72 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"rackidtab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/topology.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    72 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"rackidtab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if rackidtab
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut *mut rackhashentry
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/topology.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    72 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"rackidtab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/topology.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    72 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"rackidtab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            rackidnext = 1 as uint32_t;
        } else if rackidnext >= rackidtabsize {
            rackidtabsize = rackidtabsize
                .wrapping_mul(3 as uint32_t)
                .wrapping_div(2 as uint32_t);
            rackidtab = mfsrealloc(
                rackidtab as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<*mut rackhashentry>().wrapping_mul(rackidtabsize as size_t),
            ) as *mut *mut rackhashentry;
            if rackidtab.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/topology.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    77 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"rackidtab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/topology.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    77 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"rackidtab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if rackidtab
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut *mut rackhashentry
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/topology.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    77 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"rackidtab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/topology.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    77 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"rackidtab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
        }
        while i < rackidtabsize {
            *rackidtab.offset(i as isize) = ::core::ptr::null_mut::<rackhashentry>();
            i = i.wrapping_add(1);
        }
        let c2rust_fresh0 = rackidnext;
        rackidnext = rackidnext.wrapping_add(1);
        return c2rust_fresh0;
    }
}
unsafe extern "C" fn topology_rackname_to_rackid(
    mut rackname: *mut ::core::ffi::c_char,
) -> uint32_t {
    unsafe {
        let mut hash: uint32_t = 0;
        let mut hashpos: uint32_t = 0;
        let mut rhe: *mut rackhashentry = ::core::ptr::null_mut::<rackhashentry>();
        hash = topology_rackname_hash(rackname);
        hashpos = hash.wrapping_rem(HASHTABSIZE as uint32_t);
        rhe = rackhashtab[hashpos as usize];
        while !rhe.is_null() {
            if (*rhe).hash == hash && strcmp((*rhe).rackname, rackname) == 0 as ::core::ffi::c_int {
                return (*rhe).rackid;
            }
            rhe = (*rhe).next as *mut rackhashentry;
        }
        rhe = malloc(::core::mem::size_of::<rackhashentry>()) as *mut rackhashentry;
        (*rhe).rackname = strdup(rackname);
        (*rhe).rackid = topology_get_next_free_rackid();
        (*rhe).hash = hash;
        (*rhe).next = rackhashtab[hashpos as usize] as *mut _rackhashentry;
        rackhashtab[hashpos as usize] = rhe;
        *rackidtab.offset((*rhe).rackid as isize) = rhe;
        return (*rhe).rackid;
    }
}
unsafe extern "C" fn topology_rackid_to_rackname(mut rackid: uint32_t) -> *mut ::core::ffi::c_char {
    unsafe {
        if rackid == 0 as uint32_t {
            return b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        if rackid < rackidnext {
            return (**rackidtab.offset(rackid as isize)).rackname;
        }
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
}
unsafe extern "C" fn topology_rackname_init() {
    unsafe {
        let mut i: uint32_t = 0;
        rackidtab = ::core::ptr::null_mut::<*mut rackhashentry>();
        rackidtabsize = 0 as uint32_t;
        rackidnext = 0 as uint32_t;
        i = 0 as uint32_t;
        while i < HASHTABSIZE as uint32_t {
            rackhashtab[i as usize] = ::core::ptr::null_mut::<rackhashentry>();
            i = i.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn topology_rackname_cleanup() {
    unsafe {
        let mut i: uint32_t = 0;
        i = 1 as uint32_t;
        while i < rackidnext {
            free((**rackidtab.offset(i as isize)).rackname as *mut ::core::ffi::c_void);
            free(*rackidtab.offset(i as isize) as *mut ::core::ffi::c_void);
            i = i.wrapping_add(1);
        }
        free(rackidtab as *mut ::core::ffi::c_void);
        topology_rackname_init();
    }
}
static mut rackhashtab_stash: [*mut rackhashentry; 4096] =
    [::core::ptr::null_mut::<rackhashentry>(); 4096];
static mut rackidtab_stash: *mut *mut rackhashentry = ::core::ptr::null_mut::<*mut rackhashentry>();
static mut rackidtabsize_stash: uint32_t = 0 as uint32_t;
static mut rackidnext_stash: uint32_t = 0 as uint32_t;
unsafe extern "C" fn topology_rackname_stash() {
    unsafe {
        let mut i: uint32_t = 0;
        i = 0 as uint32_t;
        while i < HASHTABSIZE as uint32_t {
            rackhashtab_stash[i as usize] = rackhashtab[i as usize];
            rackhashtab[i as usize] = ::core::ptr::null_mut::<rackhashentry>();
            i = i.wrapping_add(1);
        }
        rackidtab_stash = rackidtab;
        rackidtab = ::core::ptr::null_mut::<*mut rackhashentry>();
        rackidtabsize_stash = rackidtabsize;
        rackidtabsize = 0 as uint32_t;
        rackidnext_stash = rackidnext;
        rackidnext = 0 as uint32_t;
    }
}
unsafe extern "C" fn topology_rackname_restore() {
    unsafe {
        topology_rackname_cleanup();
        let mut i: uint32_t = 0;
        i = 0 as uint32_t;
        while i < HASHTABSIZE as uint32_t {
            rackhashtab[i as usize] = rackhashtab_stash[i as usize];
            rackhashtab_stash[i as usize] = ::core::ptr::null_mut::<rackhashentry>();
            i = i.wrapping_add(1);
        }
        rackidtab = rackidtab_stash;
        rackidtab_stash = ::core::ptr::null_mut::<*mut rackhashentry>();
        rackidtabsize = rackidtabsize_stash;
        rackidtabsize_stash = 0 as uint32_t;
        rackidnext = rackidnext_stash;
        rackidnext_stash = 0 as uint32_t;
    }
}
unsafe extern "C" fn topology_rackname_cleanupstash() {
    unsafe {
        let mut i: uint32_t = 0;
        i = 1 as uint32_t;
        while i < rackidnext_stash {
            free((**rackidtab_stash.offset(i as isize)).rackname as *mut ::core::ffi::c_void);
            free(*rackidtab_stash.offset(i as isize) as *mut ::core::ffi::c_void);
            i = i.wrapping_add(1);
        }
        free(rackidtab_stash as *mut ::core::ffi::c_void);
        rackidtab_stash = ::core::ptr::null_mut::<*mut rackhashentry>();
        rackidtabsize_stash = 0 as uint32_t;
        rackidnext_stash = 0 as uint32_t;
        i = 0 as uint32_t;
        while i < HASHTABSIZE as uint32_t {
            rackhashtab_stash[i as usize] = ::core::ptr::null_mut::<rackhashentry>();
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn topology_parsenet(
    mut net: *mut ::core::ffi::c_char,
    mut fromip: *mut uint32_t,
    mut toip: *mut uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ip: uint32_t = 0;
        let mut i: uint32_t = 0;
        let mut octet: uint32_t = 0;
        if *net.offset(0 as isize) as ::core::ffi::c_int == '*' as ::core::ffi::c_int
            && *net.offset(1 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            *fromip = 0 as uint32_t;
            *toip = 0xffffffff as ::core::ffi::c_uint as uint32_t;
            return 0 as ::core::ffi::c_int;
        }
        ip = 0 as uint32_t;
        i = 0 as uint32_t;
        while i < 4 as uint32_t {
            if *net as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                && *net as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
            {
                octet = 0 as uint32_t;
                while *net as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && *net as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    octet = octet.wrapping_mul(10 as uint32_t);
                    octet = octet.wrapping_add(
                        (*net as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as uint32_t,
                    );
                    net = net.offset(1);
                    if octet > 255 as uint32_t {
                        return -1 as ::core::ffi::c_int;
                    }
                }
            } else {
                return -1 as ::core::ffi::c_int;
            }
            if i < 3 as uint32_t {
                if *net as ::core::ffi::c_int != '.' as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
                net = net.offset(1);
            }
            ip = ip.wrapping_mul(256 as uint32_t);
            ip = ip.wrapping_add(octet);
            i = i.wrapping_add(1);
        }
        if *net as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            *fromip = ip;
            *toip = ip;
            return 0 as ::core::ffi::c_int;
        }
        if *net as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
            *fromip = ip;
            ip = 0 as uint32_t;
            net = net.offset(1);
            i = 0 as uint32_t;
            while i < 4 as uint32_t {
                if *net as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && *net as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    octet = 0 as uint32_t;
                    while *net as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                        && *net as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                    {
                        octet = octet.wrapping_mul(10 as uint32_t);
                        octet = octet.wrapping_add(
                            (*net as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as uint32_t,
                        );
                        net = net.offset(1);
                        if octet > 255 as uint32_t {
                            return -1 as ::core::ffi::c_int;
                        }
                    }
                } else {
                    return -1 as ::core::ffi::c_int;
                }
                if i == 0 as uint32_t
                    && *net as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && octet <= 32 as uint32_t
                {
                    ip = 0xffffffff as ::core::ffi::c_uint as uint32_t;
                    if octet < 32 as uint32_t {
                        ip <<= (32 as uint32_t).wrapping_sub(octet);
                    }
                    break;
                } else {
                    if i < 3 as uint32_t {
                        if *net as ::core::ffi::c_int != '.' as ::core::ffi::c_int {
                            return -1 as ::core::ffi::c_int;
                        }
                        net = net.offset(1);
                    }
                    ip = ip.wrapping_mul(256 as uint32_t);
                    ip = ip.wrapping_add(octet);
                    i = i.wrapping_add(1);
                }
            }
            if *net as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
            *fromip &= ip;
            *toip = *fromip | ip ^ 0xffffffff as uint32_t;
            return 0 as ::core::ffi::c_int;
        }
        if *net as ::core::ffi::c_int == '-' as ::core::ffi::c_int {
            *fromip = ip;
            ip = 0 as uint32_t;
            net = net.offset(1);
            i = 0 as uint32_t;
            while i < 4 as uint32_t {
                if *net as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && *net as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    octet = 0 as uint32_t;
                    while *net as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                        && *net as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                    {
                        octet = octet.wrapping_mul(10 as uint32_t);
                        octet = octet.wrapping_add(
                            (*net as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as uint32_t,
                        );
                        net = net.offset(1);
                        if octet > 255 as uint32_t {
                            return -1 as ::core::ffi::c_int;
                        }
                    }
                } else {
                    return -1 as ::core::ffi::c_int;
                }
                if i < 3 as uint32_t {
                    if *net as ::core::ffi::c_int != '.' as ::core::ffi::c_int {
                        return -1 as ::core::ffi::c_int;
                    }
                    net = net.offset(1);
                }
                ip = ip.wrapping_mul(256 as uint32_t);
                ip = ip.wrapping_add(octet);
                i = i.wrapping_add(1);
            }
            if *net as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
            *toip = ip;
            return 0 as ::core::ffi::c_int;
        }
        return -1 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn topology_get_rackid(mut ip: uint32_t) -> uint32_t {
    unsafe {
        return itree_find(racktree, ip);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn topology_distance(mut ip1: uint32_t, mut ip2: uint32_t) -> uint8_t {
    unsafe {
        let mut rid1: uint32_t = 0;
        let mut rid2: uint32_t = 0;
        let mut rname1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut rname2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut pos: ::core::ffi::c_int = 0;
        let mut lastbar: ::core::ffi::c_int = 0;
        let mut l1: uint8_t = 0;
        let mut l2: uint8_t = 0;
        if ip1 == ip2 {
            return 0 as uint8_t;
        }
        rid1 = itree_find(racktree, ip1);
        rid2 = itree_find(racktree, ip2);
        if rid1 == rid2 {
            return 1 as uint8_t;
        }
        rname1 = topology_rackid_to_rackname(rid1);
        rname2 = topology_rackid_to_rackname(rid2);
        if rname1.is_null() && rname2.is_null() {
            return 1 as uint8_t;
        }
        lastbar = 0 as ::core::ffi::c_int;
        if !rname1.is_null() && !rname2.is_null() {
            pos = 0 as ::core::ffi::c_int;
            loop {
                if *rname1.offset(pos as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && *rname2.offset(pos as isize) as ::core::ffi::c_int
                        == '|' as ::core::ffi::c_int
                    || *rname1.offset(pos as isize) as ::core::ffi::c_int
                        == '|' as ::core::ffi::c_int
                        && *rname2.offset(pos as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                {
                    lastbar = pos;
                    break;
                } else {
                    if *rname1.offset(pos as isize) as ::core::ffi::c_int
                        != *rname2.offset(pos as isize) as ::core::ffi::c_int
                    {
                        break;
                    }
                    if *rname1.offset(pos as isize) as ::core::ffi::c_int
                        == '|' as ::core::ffi::c_int
                    {
                        lastbar = pos;
                    }
                    if *rname1.offset(pos as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        return 1 as uint8_t;
                    }
                    pos += 1;
                }
            }
        }
        l1 = 0 as uint8_t;
        l2 = 0 as uint8_t;
        if !rname1.is_null() {
            if *rname1.offset(lastbar as isize) as ::core::ffi::c_int == '|' as ::core::ffi::c_int {
                pos = lastbar + 1 as ::core::ffi::c_int;
            } else {
                pos = lastbar;
            }
            while *rname1.offset(pos as isize) != 0 {
                if *rname1.offset(pos as isize) as ::core::ffi::c_int == '|' as ::core::ffi::c_int {
                    l1 = l1.wrapping_add(1);
                }
                pos += 1;
            }
        }
        if !rname2.is_null() {
            if *rname2.offset(lastbar as isize) as ::core::ffi::c_int == '|' as ::core::ffi::c_int {
                pos = lastbar + 1 as ::core::ffi::c_int;
            } else {
                pos = lastbar;
            }
            while *rname2.offset(pos as isize) != 0 {
                if *rname2.offset(pos as isize) as ::core::ffi::c_int == '|' as ::core::ffi::c_int {
                    l2 = l2.wrapping_add(1);
                }
                pos += 1;
            }
        }
        if l1 as ::core::ffi::c_int > l2 as ::core::ffi::c_int {
            return (2 as ::core::ffi::c_int + l1 as ::core::ffi::c_int) as uint8_t;
        } else {
            return (2 as ::core::ffi::c_int + l2 as ::core::ffi::c_int) as uint8_t;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn topology_parseline(
    mut line: *mut ::core::ffi::c_char,
    mut lineno: uint32_t,
    mut fip: *mut uint32_t,
    mut tip: *mut uint32_t,
    mut rid: *mut uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: ::core::ffi::c_char = 0;
        let mut net: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut rackname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        p = line;
        while *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
        if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '#' as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        net = p;
        while *p as ::core::ffi::c_int != 0
            && *p as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
            && *p as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
        if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"mfstopology: incomplete definition in line: %u\0".as_ptr()
                    as *const ::core::ffi::c_char,
                lineno,
            );
            return -1 as ::core::ffi::c_int;
        }
        *p = 0 as ::core::ffi::c_char;
        p = p.offset(1);
        if topology_parsenet(net, fip, tip) < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"mfstopology: incorrect ip/network definition in line: %u\0".as_ptr()
                    as *const ::core::ffi::c_char,
                lineno,
            );
            return -1 as ::core::ffi::c_int;
        }
        while *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
        if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '#' as ::core::ffi::c_int
        {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"mfstopology: incorrect rack id in line: %u\0".as_ptr()
                    as *const ::core::ffi::c_char,
                lineno,
            );
            return -1 as ::core::ffi::c_int;
        }
        rackname = p;
        while *p as ::core::ffi::c_int != 0
            && *p as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
            && *p as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
        c = *p;
        *p = 0 as ::core::ffi::c_char;
        *rid = topology_rackname_to_rackid(rackname);
        *p = c;
        while *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
        if *p as ::core::ffi::c_int != 0 && *p as ::core::ffi::c_int != '#' as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"mfstopology: garbage found at the end of line: %u\0".as_ptr()
                    as *const ::core::ffi::c_char,
                lineno,
            );
            return -1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn topology_load() {
    unsafe {
        let mut fd: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut linebuff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut lbsize: size_t = 0;
        let mut s: uint32_t = 0;
        let mut lineno: uint32_t = 0;
        let mut fip: uint32_t = 0;
        let mut tip: uint32_t = 0;
        let mut rid: uint32_t = 0;
        let mut newtree: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        fd = fopen(
            TopologyFileName,
            b"r\0".as_ptr() as *const ::core::ffi::c_char,
        ) as *mut FILE;
        if fd.is_null() {
            if *__errno_location() == ENOENT {
                if !racktree.is_null() {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"mfstopology configuration file (%s) not found - network topology not changed\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        TopologyFileName,
                    );
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_NOTICE,
                        b"mfstopology configuration file (%s) not found - network topology not defined\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        TopologyFileName,
                    );
                }
            } else if !racktree.is_null() {
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG,
                    MFSLOG_WARNING,
                    b"can't open mfstopology configuration file (%s) - network topology not changed, error\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    TopologyFileName,
                );
            } else {
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"can't open mfstopology configuration file (%s) - network topology not defined, error\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    TopologyFileName,
                );
            }
            return;
        }
        topology_rackname_stash();
        newtree = NULL;
        lineno = 1 as uint32_t;
        lbsize = 10000 as size_t;
        linebuff = malloc(lbsize) as *mut ::core::ffi::c_char;
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
                if topology_parseline(linebuff, lineno, &raw mut fip, &raw mut tip, &raw mut rid)
                    >= 0 as ::core::ffi::c_int
                {
                    newtree = itree_add_interval(newtree, fip, tip, rid);
                }
            }
            lineno = lineno.wrapping_add(1);
        }
        free(linebuff as *mut ::core::ffi::c_void);
        if ferror(fd) != 0 {
            fclose(fd);
            if !racktree.is_null() {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"error reading mfstopology file - network topology not changed\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            } else {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"error reading mfstopology file - network topology not defined\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            itree_freeall(newtree);
            topology_rackname_restore();
            return;
        }
        fclose(fd);
        topology_rackname_cleanupstash();
        itree_freeall(racktree);
        racktree = newtree;
        if !racktree.is_null() {
            racktree = itree_rebalance(racktree);
        }
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_INFO,
            b"topology file has been loaded\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn topology_reload() {
    unsafe {
        let mut fd: ::core::ffi::c_int = 0;
        if !TopologyFileName.is_null() {
            free(TopologyFileName as *mut ::core::ffi::c_void);
        }
        if cfg_isdefined(b"TOPOLOGY_FILENAME\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
            TopologyFileName = strdup(
                b"/home/ston/Workspaces/polaris-fs/target/mfs-port/mfs-build/../mfs-install/etc/mfs/mfstopology.cfg\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            if TopologyFileName.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/topology.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    539 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"TopologyFileName\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/topology.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    539 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"TopologyFileName\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if TopologyFileName
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut ::core::ffi::c_char
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/topology.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    539 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"TopologyFileName\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/topology.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    539 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"TopologyFileName\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            fd = open(TopologyFileName, O_RDONLY);
            if fd < 0 as ::core::ffi::c_int && *__errno_location() == ENOENT {
                let mut tmpname: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                tmpname = strdup(
                    b"/home/ston/Workspaces/polaris-fs/target/mfs-port/mfs-build/../mfs-install/etc/mfstopology.cfg\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                fd = open(tmpname, O_RDONLY);
                if fd >= 0 as ::core::ffi::c_int {
                    free(TopologyFileName as *mut ::core::ffi::c_void);
                    TopologyFileName = tmpname;
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"default sysconf path has changed - please move mfstopology.cfg from /home/ston/Workspaces/polaris-fs/target/mfs-port/mfs-build/../mfs-install/etc/ to /home/ston/Workspaces/polaris-fs/target/mfs-port/mfs-build/../mfs-install/etc/mfs/\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                } else {
                    free(tmpname as *mut ::core::ffi::c_void);
                }
            }
            if fd >= 0 as ::core::ffi::c_int {
                close(fd);
            }
            cfg_use_option(
                b"TOPOLOGY_FILENAME\0".as_ptr() as *const ::core::ffi::c_char,
                TopologyFileName,
            );
        } else {
            TopologyFileName = cfg_getstr(
                b"TOPOLOGY_FILENAME\0".as_ptr() as *const ::core::ffi::c_char,
                b"/home/ston/Workspaces/polaris-fs/target/mfs-port/mfs-build/../mfs-install/etc/mfs/mfstopology.cfg\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
        topology_load();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn topology_term() {
    unsafe {
        itree_freeall(racktree);
        if !TopologyFileName.is_null() {
            free(TopologyFileName as *mut ::core::ffi::c_void);
        }
        topology_rackname_cleanup();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn topology_init() -> ::core::ffi::c_int {
    unsafe {
        TopologyFileName = ::core::ptr::null_mut::<::core::ffi::c_char>();
        racktree = NULL;
        topology_rackname_init();
        topology_reload();
        main_reload_register_fname(
            Some(topology_reload as unsafe extern "C" fn() -> ()),
            b"topology_reload\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_destruct_register_fname(
            Some(topology_term as unsafe extern "C" fn() -> ()),
            b"topology_term\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
}
