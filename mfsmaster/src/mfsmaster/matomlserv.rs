pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn writev(
        __fd: ::core::ffi::c_int,
        __iovec: *const iovec,
        __count: ::core::ffi::c_int,
    ) -> ssize_t;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn lseek(
        __fd: ::core::ffi::c_int,
        __offset: __off64_t,
        __whence: ::core::ffi::c_int,
    ) -> __off64_t;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn read(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    unsafe fn pread(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __nbytes: size_t,
        __offset: __off64_t,
    ) -> ssize_t;
    static mut stderr: *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn changelog_get_old_changes(
        version: uint64_t,
        sendfn: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void, uint64_t, *mut uint8_t, uint32_t) -> (),
        >,
        userdata: *mut ::core::ffi::c_void,
        limit: uint32_t,
    ) -> uint32_t;
    unsafe fn changelog_get_minversion() -> uint64_t;
    unsafe fn meta_version() -> uint64_t;
    unsafe fn meta_do_store_metadata();
    unsafe fn meta_get_id() -> uint64_t;
    unsafe fn mycrc32(crc: uint32_t, block: *const ::core::ffi::c_void, leng: uint32_t)
    -> uint32_t;
    unsafe fn cfg_isdefined(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn cfg_getdefaultstr(name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn cfg_getdefaultfile(
        name: *const ::core::ffi::c_char,
        maxleng: uint32_t,
    ) -> *mut cfg_buff;
    unsafe fn cfg_getstr(
        name: *const ::core::ffi::c_char,
        def: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn cfg_getuint32(name: *const ::core::ffi::c_char, def: uint32_t) -> uint32_t;
    unsafe fn main_destruct_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn main_reload_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn main_keepalive_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn main_poll_register_fname(
        desc: Option<unsafe extern "C" fn(*mut pollfd, *mut uint32_t) -> ()>,
        serve: Option<unsafe extern "C" fn(*mut pollfd) -> ()>,
        dname: *const ::core::ffi::c_char,
        sname: *const ::core::ffi::c_char,
    );
    unsafe fn main_time_register_fname(
        seconds: uint32_t,
        offset: uint32_t,
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn main_utime() -> uint64_t;
    unsafe fn univallocstrip(ip: uint32_t) -> *mut ::core::ffi::c_char;
    unsafe fn tcpsocket() -> ::core::ffi::c_int;
    unsafe fn tcpresolve(
        hostname: *const ::core::ffi::c_char,
        service: *const ::core::ffi::c_char,
        ip: *mut uint32_t,
        port: *mut uint16_t,
        passiveflag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn tcpnonblock(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpsetacceptfilter(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpreuseaddr(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpnodelay(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpnumlisten(
        sock: ::core::ffi::c_int,
        ip: uint32_t,
        port: uint16_t,
        queue: uint16_t,
    ) -> ::core::ffi::c_int;
    unsafe fn tcpaccept(lsock_0: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpgetpeer(
        sock: ::core::ffi::c_int,
        ip: *mut uint32_t,
        port: *mut uint16_t,
    ) -> ::core::ffi::c_int;
    unsafe fn tcpclose(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
    unsafe fn monotonic_useconds() -> uint64_t;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type ssize_t = isize;
pub type int32_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut ::core::ffi::c_void,
    pub iov_len: size_t,
}
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
pub const METALOGGER: C2Rust_Unnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct matomlserventry {
    pub mode: uint8_t,
    pub sock: ::core::ffi::c_int,
    pub pdescpos: int32_t,
    pub lastread: ::core::ffi::c_double,
    pub lastwrite: ::core::ffi::c_double,
    pub input_hdr: [uint8_t; 8],
    pub input_startptr: *mut uint8_t,
    pub input_bytesleft: uint32_t,
    pub input_end: uint8_t,
    pub input_packet: *mut in_packetstruct,
    pub inputhead: *mut in_packetstruct,
    pub inputtail: *mut *mut in_packetstruct,
    pub outputhead: *mut out_packetstruct,
    pub outputtail: *mut *mut out_packetstruct,
    pub timeout: uint16_t,
    pub next_log_version: uint64_t,
    pub servstrip: *mut ::core::ffi::c_char,
    pub version: uint32_t,
    pub servip: uint32_t,
    pub clienttype: uint8_t,
    pub logstate: uint8_t,
    pub upload_meta_fd: ::core::ffi::c_int,
    pub upload_chain1_fd: ::core::ffi::c_int,
    pub upload_chain2_fd: ::core::ffi::c_int,
    pub next: *mut matomlserventry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct out_packetstruct {
    pub next: *mut out_packetstruct,
    pub startptr: *mut uint8_t,
    pub bytesleft: uint32_t,
    pub data: [uint8_t; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_packetstruct {
    pub next: *mut in_packetstruct,
    pub r#type: uint32_t,
    pub leng: uint32_t,
    pub data: [uint8_t; 1],
}
pub const CLOSE: C2Rust_Unnamed = 2;
pub const KILL: C2Rust_Unnamed = 0;
pub const DELAYED: C2Rust_Unnamed_1 = 1;
pub const SYNC: C2Rust_Unnamed_1 = 2;
pub const DATA: C2Rust_Unnamed = 1;
pub const SUPERVISOR: C2Rust_Unnamed_0 = 2;
pub const UNKNOWN: C2Rust_Unnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
}
pub const NONE: C2Rust_Unnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cfg_buff {
    pub leng: uint32_t,
    pub data: [uint8_t; 1],
}
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub type C2Rust_Unnamed_0 = ::core::ffi::c_uint;
pub type C2Rust_Unnamed_1 = ::core::ffi::c_uint;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ENOTSUP: ::core::ffi::c_int = EOPNOTSUPP;
pub const MFS_ERROR_ENOENT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ANTOMA_MAXPACKETSIZE: ::core::ffi::c_int = 1500000 as ::core::ffi::c_int;
pub const MATOAN_MAXPACKETSIZE: ::core::ffi::c_int = 1500000 as ::core::ffi::c_int;
pub const ANTOAN_NOP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ANTOAN_UNKNOWN_COMMAND: uint32_t = 1 as uint32_t;
pub const ANTOAN_BAD_COMMAND_SIZE: uint32_t = 2 as uint32_t;
pub const ANTOAN_FORCE_TIMEOUT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const ANTOAN_GET_VERSION: uint32_t = 10 as uint32_t;
pub const ANTOAN_VERSION: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const ANTOMA_REGISTER: uint32_t = 50 as uint32_t;
pub const MATOAN_METACHANGES_LOG: ::core::ffi::c_int = PROTO_BASE + 51 as ::core::ffi::c_int;
pub const MATOAN_MASTER_ACK: ::core::ffi::c_int = PROTO_BASE + 52 as ::core::ffi::c_int;
pub const MATOAN_STATE: ::core::ffi::c_int = PROTO_BASE + 53 as ::core::ffi::c_int;
pub const ANTOMA_DOWNLOAD_START: uint32_t = 60 as uint32_t;
pub const MATOAN_DOWNLOAD_INFO: ::core::ffi::c_int = PROTO_BASE + 61 as ::core::ffi::c_int;
pub const ANTOMA_DOWNLOAD_REQUEST: uint32_t = 62 as uint32_t;
pub const MATOAN_DOWNLOAD_DATA: ::core::ffi::c_int = PROTO_BASE + 63 as ::core::ffi::c_int;
pub const ANTOMA_DOWNLOAD_END: uint32_t = 64 as uint32_t;
pub const ANTOMA_STORE_METADATA: uint32_t = 65 as uint32_t;
pub const ANTOMA_SYSLOG: uint32_t = 71 as uint32_t;
pub const ANTOAN_GET_CONFIG: uint32_t = 80 as uint32_t;
pub const ANTOAN_CONFIG_VALUE: ::core::ffi::c_int = PROTO_BASE + 81 as ::core::ffi::c_int;
pub const ANTOAN_CONFIG_FILE_CONTENT: ::core::ffi::c_int = PROTO_BASE + 83 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const EOPNOTSUPP: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const DEFAULT_MASTER_CONTROL_PORT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"9419\0") };
pub const PROTO_BASE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const VERSHEX: ::core::ffi::c_int = 4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
    + 59 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
    + 2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
pub const VERSMAJ: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const VERSMID: ::core::ffi::c_int = 59 as ::core::ffi::c_int;
pub const VERSMIN: ::core::ffi::c_int = 2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
pub const VERSSTR: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"4.59.2-1\0") };
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const POLLOUT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const POLLERR: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const POLLHUP: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn put64bit(mut ptr: *mut *mut uint8_t, mut val: uint64_t) {
    unsafe {
        val = val.swap_bytes() as uint64_t;
        memcpy(
            *ptr as *mut ::core::ffi::c_void,
            &raw mut val as *const ::core::ffi::c_void,
            8 as size_t,
        );
        *ptr = (*ptr).offset(8 as ::core::ffi::c_int as isize);
    }
}
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
unsafe extern "C" fn get64bit(mut ptr: *mut *const uint8_t) -> uint64_t {
    unsafe {
        let mut t64: uint64_t = 0;
        memcpy(
            &raw mut t64 as *mut ::core::ffi::c_void,
            *ptr as *const ::core::ffi::c_void,
            8 as size_t,
        );
        *ptr = (*ptr).offset(8 as ::core::ffi::c_int as isize);
        return t64.swap_bytes();
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
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
pub const MaxPacketSize: ::core::ffi::c_int = ANTOMA_MAXPACKETSIZE;
pub const ML_META_DL_BLOCK: ::core::ffi::c_int =
    if (MATOAN_MAXPACKETSIZE - 1000 as ::core::ffi::c_int) < 1000000 as ::core::ffi::c_int {
        MATOAN_MAXPACKETSIZE - 1000 as ::core::ffi::c_int
    } else {
        1000000 as ::core::ffi::c_int
    };
pub const OLD_CHANGES_GROUP_COUNT: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
static mut matomlservhead: *mut matomlserventry = ::core::ptr::null_mut::<matomlserventry>();
static mut lsock: ::core::ffi::c_int = 0;
static mut lsockpdescpos: int32_t = 0;
static mut ListenHost: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut ListenPort: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut listenip: uint32_t = 0;
static mut listenport: uint16_t = 0;
static mut DefaultTimeout: uint32_t = 0;
static mut ForceTimeout: uint32_t = 0;
static mut BackMetaCopies: uint32_t = 0;
#[inline]
unsafe extern "C" fn matomlserv_clientname(
    mut eptr: *mut matomlserventry,
) -> *const ::core::ffi::c_char {
    unsafe {
        match (*eptr).clienttype as ::core::ffi::c_int {
            1 => match (*eptr).logstate as ::core::ffi::c_int {
                1 => {
                    return b"METALOGGER-DELAYED\0".as_ptr() as *const ::core::ffi::c_char;
                }
                2 => {
                    return b"METALOGGER-SYNC\0".as_ptr() as *const ::core::ffi::c_char;
                }
                _ => return b"METALOGGER\0".as_ptr() as *const ::core::ffi::c_char,
            },
            2 => return b"SUPERVISOR\0".as_ptr() as *const ::core::ffi::c_char,
            _ => {}
        }
        return b"UNKNOWN\0".as_ptr() as *const ::core::ffi::c_char;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_mloglist_size() -> uint32_t {
    unsafe {
        let mut eptr: *mut matomlserventry = ::core::ptr::null_mut::<matomlserventry>();
        let mut i: uint32_t = 0;
        i = 0 as uint32_t;
        eptr = matomlservhead;
        while !eptr.is_null() {
            if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                && (*eptr).mode as ::core::ffi::c_int != CLOSE as ::core::ffi::c_int
                && (*eptr).clienttype as ::core::ffi::c_int == METALOGGER as ::core::ffi::c_int
            {
                i = i.wrapping_add(1);
            }
            eptr = (*eptr).next as *mut matomlserventry;
        }
        return i.wrapping_mul((4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_mloglist_data(mut ptr: *mut uint8_t) {
    unsafe {
        let mut eptr: *mut matomlserventry = ::core::ptr::null_mut::<matomlserventry>();
        eptr = matomlservhead;
        while !eptr.is_null() {
            if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                && (*eptr).mode as ::core::ffi::c_int != CLOSE as ::core::ffi::c_int
                && (*eptr).clienttype as ::core::ffi::c_int == METALOGGER as ::core::ffi::c_int
            {
                put32bit(&raw mut ptr, (*eptr).version);
                put32bit(&raw mut ptr, (*eptr).servip);
            }
            eptr = (*eptr).next as *mut matomlserventry;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_status() {
    unsafe {
        let mut eptr: *mut matomlserventry = ::core::ptr::null_mut::<matomlserventry>();
        eptr = matomlservhead;
        while !eptr.is_null() {
            if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int {
                return;
            }
            eptr = (*eptr).next as *mut matomlserventry;
        }
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_NOTICE,
            b"no metaloggers connected !!!\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_create_packet(
    mut eptr: *mut matomlserventry,
    mut r#type: uint32_t,
    mut size: uint32_t,
) -> *mut uint8_t {
    unsafe {
        let mut outpacket: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut psize: uint32_t = 0;
        psize = size.wrapping_add(8 as uint32_t);
        outpacket = malloc((20 as size_t).wrapping_add(psize as size_t)) as *mut out_packetstruct;
        if outpacket.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matomlserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                266 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"outpacket\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matomlserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                266 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"outpacket\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if outpacket
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut out_packetstruct
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matomlserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                266 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"outpacket\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matomlserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                266 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"outpacket\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*outpacket).bytesleft = psize;
        ptr = &raw mut (*outpacket).data as *mut uint8_t;
        put32bit(&raw mut ptr, r#type);
        put32bit(&raw mut ptr, size);
        (*outpacket).startptr = &raw mut (*outpacket).data as *mut uint8_t;
        (*outpacket).next = ::core::ptr::null_mut::<out_packetstruct>();
        *(*eptr).outputtail = outpacket;
        (*eptr).outputtail = &raw mut (*outpacket).next as *mut *mut out_packetstruct;
        return ptr;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_broadcast_timeout() {
    unsafe {
        let mut eptr: *mut matomlserventry = ::core::ptr::null_mut::<matomlserventry>();
        let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if ForceTimeout > 0 as uint32_t {
            eptr = matomlservhead;
            while !eptr.is_null() {
                if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
                    && ((*eptr).clienttype as ::core::ffi::c_int
                        == METALOGGER as ::core::ffi::c_int
                        && (*eptr).version
                            >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                                + 24 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                                + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                    0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                })) as uint32_t)
                {
                    (*eptr).timeout = ForceTimeout as uint16_t;
                    data = matomlserv_create_packet(
                        eptr,
                        ANTOAN_FORCE_TIMEOUT as uint32_t,
                        2 as uint32_t,
                    );
                    put16bit(&raw mut data, ForceTimeout as uint16_t);
                }
                eptr = (*eptr).next as *mut matomlserventry;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_send_old_change(
    mut veptr: *mut ::core::ffi::c_void,
    mut version: uint64_t,
    mut data: *mut uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut eptr: *mut matomlserventry = veptr as *mut matomlserventry;
        let mut pdata: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        pdata = matomlserv_create_packet(
            eptr,
            MATOAN_METACHANGES_LOG as uint32_t,
            (9 as uint32_t).wrapping_add(length),
        );
        put8bit(&raw mut pdata, 0xff as uint8_t);
        put64bit(&raw mut pdata, version);
        memcpy(
            pdata as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            length as size_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_get_version(
    mut eptr: *mut matomlserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0 as uint32_t;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        static mut vstring: [::core::ffi::c_char; 9] = VERSSTR;
        if length != 0 as uint32_t && length != 4 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOAN_GET_VERSION - wrong size (%u/4|0)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length == 4 as uint32_t {
            msgid = get32bit(&raw mut data);
            ptr = matomlserv_create_packet(
                eptr,
                ANTOAN_VERSION as uint32_t,
                ((4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as size_t)
                    .wrapping_add(strlen(&raw const vstring as *const ::core::ffi::c_char))
                    as uint32_t,
            );
            put32bit(&raw mut ptr, msgid);
        } else {
            ptr = matomlserv_create_packet(
                eptr,
                ANTOAN_VERSION as uint32_t,
                (4 as size_t).wrapping_add(strlen(&raw const vstring as *const ::core::ffi::c_char))
                    as uint32_t,
            );
        }
        put16bit(&raw mut ptr, VERSMAJ as uint16_t);
        put8bit(&raw mut ptr, VERSMID as uint8_t);
        put8bit(&raw mut ptr, VERSMIN as uint8_t);
        memcpy(
            ptr as *mut ::core::ffi::c_void,
            &raw const vstring as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            strlen(&raw const vstring as *const ::core::ffi::c_char),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_get_config(
    mut eptr: *mut matomlserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0;
        let mut name: [::core::ffi::c_char; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut vleng: uint32_t = 0;
        let mut val: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length < 5 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOAN_GET_CONFIG - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        nleng = get8bit(&raw mut data);
        if length != (5 as uint32_t).wrapping_add(nleng as uint32_t) {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOAN_GET_CONFIG - wrong size (%u:nleng=%hhu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                nleng as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        memcpy(
            &raw mut name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            nleng as size_t,
        );
        name[nleng as usize] = 0 as ::core::ffi::c_char;
        if strcmp(
            &raw mut name as *mut ::core::ffi::c_char,
            b"AUTH_CODE\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if cfg_isdefined(&raw mut name as *mut ::core::ffi::c_char) != 0 {
                val = strdup(b"[DEFINED]\0".as_ptr() as *const ::core::ffi::c_char);
            } else {
                val = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
        } else {
            val = cfg_getdefaultstr(&raw mut name as *mut ::core::ffi::c_char);
        }
        if !val.is_null() {
            vleng = strlen(val) as uint32_t;
            if vleng > 255 as uint32_t {
                vleng = 255 as uint32_t;
            }
        } else {
            vleng = 0 as uint32_t;
        }
        if msgid == 0 as uint32_t {
            ptr = matomlserv_create_packet(
                eptr,
                ANTOAN_CONFIG_VALUE as uint32_t,
                ((6 as ::core::ffi::c_int + nleng as ::core::ffi::c_int) as uint32_t)
                    .wrapping_add(vleng),
            );
            put32bit(&raw mut ptr, 0 as uint32_t);
            put8bit(&raw mut ptr, nleng);
            if nleng as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                memcpy(
                    ptr as *mut ::core::ffi::c_void,
                    &raw mut name as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                    nleng as size_t,
                );
                ptr = ptr.offset(nleng as ::core::ffi::c_int as isize);
            }
        } else {
            ptr = matomlserv_create_packet(
                eptr,
                ANTOAN_CONFIG_VALUE as uint32_t,
                (5 as uint32_t).wrapping_add(vleng),
            );
            put32bit(&raw mut ptr, msgid);
        }
        put8bit(&raw mut ptr, vleng as uint8_t);
        if vleng > 0 as uint32_t && !val.is_null() {
            memcpy(
                ptr as *mut ::core::ffi::c_void,
                val as *const ::core::ffi::c_void,
                vleng as size_t,
            );
        }
        if !val.is_null() {
            free(val as *mut ::core::ffi::c_void);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_get_config_file(
    mut eptr: *mut matomlserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut msgid: uint32_t = 0;
        let mut name: [::core::ffi::c_char; 256] = [0; 256];
        let mut nleng: uint8_t = 0;
        let mut fdata: *mut cfg_buff = ::core::ptr::null_mut::<cfg_buff>();
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length < 5 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOAN_GET_CONFIG_FILE - wrong size (%u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        msgid = get32bit(&raw mut data);
        nleng = get8bit(&raw mut data);
        if length != (5 as uint32_t).wrapping_add(nleng as uint32_t) {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOAN_GET_CONFIG_FILE - wrong size (%u:nleng=%hhu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                nleng as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        memcpy(
            &raw mut name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            nleng as size_t,
        );
        name[nleng as usize] = 0 as ::core::ffi::c_char;
        if strcmp(
            &raw mut name as *mut ::core::ffi::c_char,
            b"LICENCE_FILENAME\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            fdata =
                cfg_getdefaultfile(&raw mut name as *mut ::core::ffi::c_char, 65535 as uint32_t);
        } else {
            fdata = ::core::ptr::null_mut::<cfg_buff>();
        }
        if fdata.is_null() {
            ptr = matomlserv_create_packet(
                eptr,
                ANTOAN_CONFIG_FILE_CONTENT as uint32_t,
                5 as uint32_t,
            );
            put32bit(&raw mut ptr, msgid);
            put8bit(&raw mut ptr, MFS_ERROR_ENOENT as uint8_t);
        } else {
            ptr = matomlserv_create_packet(
                eptr,
                ANTOAN_CONFIG_FILE_CONTENT as uint32_t,
                (6 as uint32_t).wrapping_add((*fdata).leng),
            );
            put32bit(&raw mut ptr, msgid);
            put16bit(&raw mut ptr, (*fdata).leng as uint16_t);
            memcpy(
                ptr as *mut ::core::ffi::c_void,
                &raw mut (*fdata).data as *mut uint8_t as *const ::core::ffi::c_void,
                (*fdata).leng as size_t,
            );
            free(fdata as *mut ::core::ffi::c_void);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_syslog(
    mut eptr: *mut matomlserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut priority: uint8_t = 0;
        let mut timestamp: uint32_t = 0;
        let mut msgsize: uint16_t = 0;
        if length < 7 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOMA_SYSLOG - wrong size (%u/>=7)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        priority = get8bit(&raw mut data);
        timestamp = get32bit(&raw mut data);
        msgsize = get16bit(&raw mut data);
        if length != (7 as uint32_t).wrapping_add(msgsize as uint32_t) {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOMA_SYSLOG - wrong size (%u/7+msgsize(%hu))\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                msgsize as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_register(
    mut eptr: *mut matomlserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut rversion: uint8_t = 0;
        let mut req_minversion: uint64_t = 0;
        let mut chlog_minversion: uint64_t = 0;
        let mut n: uint32_t = 0;
        if (*eptr).version > 0 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"got register message from registered metalogger !!!\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length < 1 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOMA_REGISTER - wrong size (%u)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        } else {
            rversion = get8bit(&raw mut data);
            if rversion as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"ANTOMA_REGISTER - protocol not supported\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if rversion as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                (*eptr).clienttype = METALOGGER as ::core::ffi::c_int as uint8_t;
                if length != 7 as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"ANTOMA_REGISTER (logger 1) - wrong size (%u/7)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        length,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
                (*eptr).version = get32bit(&raw mut data);
                if ForceTimeout > 0 as uint32_t {
                    data = data.offset(2 as ::core::ffi::c_int as isize);
                } else {
                    (*eptr).timeout = get16bit(&raw mut data);
                }
                if (*eptr).version
                    >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            25 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            25 as ::core::ffi::c_int
                        })) as uint32_t
                    && (*eptr).version
                        < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })) as uint32_t
                    || (*eptr).version
                        >= (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })) as uint32_t
                        && (*eptr).version & 1 as uint32_t != 0
                    || (*eptr).version
                        >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })) as uint32_t
                {
                    let mut p: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
                    p = matomlserv_create_packet(
                        eptr,
                        MATOAN_MASTER_ACK as uint32_t,
                        5 as uint32_t,
                    );
                    put8bit(&raw mut p, 1 as uint8_t);
                    put32bit(&raw mut p, VERSHEX as uint32_t);
                    (*eptr).logstate = SYNC as ::core::ffi::c_int as uint8_t;
                } else {
                    (*eptr).logstate = SYNC as ::core::ffi::c_int as uint8_t;
                }
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_INFO,
                    b"metalogger %s registered (using simple register protocol)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*eptr).servstrip,
                );
            } else if rversion as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                (*eptr).clienttype = METALOGGER as ::core::ffi::c_int as uint8_t;
                if length != (7 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"ANTOMA_REGISTER (logger 2) - wrong size (%u/15)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        length,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
                (*eptr).version = get32bit(&raw mut data);
                if ForceTimeout > 0 as uint32_t {
                    data = data.offset(2 as ::core::ffi::c_int as isize);
                } else {
                    (*eptr).timeout = get16bit(&raw mut data);
                }
                req_minversion = get64bit(&raw mut data);
                chlog_minversion = changelog_get_minversion();
                if (*eptr).version
                    >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 7 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            25 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            25 as ::core::ffi::c_int
                        })) as uint32_t
                    && (*eptr).version
                        < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })) as uint32_t
                    || (*eptr).version
                        >= (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })) as uint32_t
                        && (*eptr).version & 1 as uint32_t != 0
                    || (*eptr).version
                        >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })) as uint32_t
                {
                    let mut p_0: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
                    p_0 = matomlserv_create_packet(
                        eptr,
                        MATOAN_MASTER_ACK as uint32_t,
                        5 as uint32_t,
                    );
                    if chlog_minversion == 0 as uint64_t || chlog_minversion > req_minversion {
                        put8bit(&raw mut p_0, 1 as uint8_t);
                        put32bit(&raw mut p_0, VERSHEX as uint32_t);
                        (*eptr).logstate = SYNC as ::core::ffi::c_int as uint8_t;
                    } else {
                        put8bit(&raw mut p_0, 0 as uint8_t);
                        put32bit(&raw mut p_0, VERSHEX as uint32_t);
                        n = changelog_get_old_changes(
                            req_minversion,
                            Some(
                                matomlserv_send_old_change
                                    as unsafe extern "C" fn(
                                        *mut ::core::ffi::c_void,
                                        uint64_t,
                                        *mut uint8_t,
                                        uint32_t,
                                    )
                                        -> (),
                            ),
                            eptr as *mut ::core::ffi::c_void,
                            OLD_CHANGES_GROUP_COUNT as uint32_t,
                        );
                        if n < OLD_CHANGES_GROUP_COUNT as uint32_t {
                            (*eptr).logstate = SYNC as ::core::ffi::c_int as uint8_t;
                        } else {
                            (*eptr).next_log_version = req_minversion.wrapping_add(n as uint64_t);
                            (*eptr).logstate = DELAYED as ::core::ffi::c_int as uint8_t;
                        }
                    }
                } else if chlog_minversion > 0 as uint64_t && chlog_minversion <= req_minversion {
                    n = changelog_get_old_changes(
                        req_minversion,
                        Some(
                            matomlserv_send_old_change
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    uint64_t,
                                    *mut uint8_t,
                                    uint32_t,
                                ) -> (),
                        ),
                        eptr as *mut ::core::ffi::c_void,
                        OLD_CHANGES_GROUP_COUNT as uint32_t,
                    );
                    if n < OLD_CHANGES_GROUP_COUNT as uint32_t {
                        (*eptr).logstate = SYNC as ::core::ffi::c_int as uint8_t;
                    } else {
                        (*eptr).next_log_version = req_minversion.wrapping_add(n as uint64_t);
                        (*eptr).logstate = DELAYED as ::core::ffi::c_int as uint8_t;
                    }
                } else {
                    (*eptr).logstate = SYNC as ::core::ffi::c_int as uint8_t;
                }
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_INFO,
                    b"metalogger %s registered (using advanced register protocol)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*eptr).servstrip,
                );
            } else if rversion as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
                let mut p_1: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
                let mut mode: uint8_t = 0;
                (*eptr).clienttype = SUPERVISOR as ::core::ffi::c_int as uint8_t;
                if length != 7 as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"ANTOMA_REGISTER (supervisor) - wrong size (%u/7)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        length,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
                (*eptr).version = get32bit(&raw mut data);
                if (*eptr).version
                    >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 23 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            5 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            5 as ::core::ffi::c_int
                        })) as uint32_t
                    && (*eptr).version
                        < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 48 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })) as uint32_t
                {
                    (*eptr).version |= 1 as uint32_t;
                }
                (*eptr).timeout = get16bit(&raw mut data);
                if (*eptr).version
                    < (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })) as uint32_t
                {
                    mode = (if (*eptr).version
                        >= (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                82 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                82 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint8_t;
                } else if (*eptr).version
                    < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })) as uint32_t
                {
                    mode = (if (*eptr).version
                        >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                107 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                107 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        2 as ::core::ffi::c_int
                    } else if (*eptr).version
                        >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                59 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                59 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint8_t;
                } else {
                    mode = (if (*eptr).version
                        >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 17 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        2 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    }) as uint8_t;
                }
                p_1 = matomlserv_create_packet(
                    eptr,
                    MATOAN_STATE as uint32_t,
                    (if mode as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        40 as ::core::ffi::c_int
                    } else if mode as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                        28 as ::core::ffi::c_int
                    } else {
                        20 as ::core::ffi::c_int
                    }) as uint32_t,
                );
                put8bit(&raw mut p_1, 0xff as uint8_t);
                put8bit(&raw mut p_1, 0xff as uint8_t);
                put8bit(&raw mut p_1, 0xff as uint8_t);
                put8bit(&raw mut p_1, 0xff as uint8_t);
                put32bit(&raw mut p_1, 0 as uint32_t);
                put32bit(&raw mut p_1, 0 as uint32_t);
                put64bit(&raw mut p_1, meta_version());
                if mode as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    put64bit(&raw mut p_1, meta_get_id());
                }
                if mode as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    put64bit(&raw mut p_1, main_utime());
                    put32bit(&raw mut p_1, 0 as uint32_t);
                }
            } else {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"ANTOMA_REGISTER - wrong version (%hhu/1)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    rversion as ::core::ffi::c_int,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if ((*eptr).timeout as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_NOTICE,
                    b"ANTOMA_REGISTER communication timeout too small (%hu seconds - should be at least 10 seconds)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    (*eptr).timeout as ::core::ffi::c_int,
                );
                if ((*eptr).timeout as ::core::ffi::c_int) < 3 as ::core::ffi::c_int {
                    (*eptr).timeout = 3 as uint16_t;
                }
                return;
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_store_metadata(
    mut eptr: *mut matomlserventry,
    _data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        if length != 0 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOMA_STORE_METADATA - wrong size (%u/0)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if (*eptr).clienttype as ::core::ffi::c_int != SUPERVISOR as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOMA_STORE_METADATA - wrong client type\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        meta_do_store_metadata();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_download_start(
    mut eptr: *mut matomlserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut filenum: uint8_t = 0;
        let mut size: uint64_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length != 1 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOMA_DOWNLOAD_START - wrong size (%u/1)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        filenum = get8bit(&raw mut data);
        if filenum as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            || filenum as ::core::ffi::c_int == 2 as ::core::ffi::c_int
        {
            if (*eptr).upload_meta_fd >= 0 as ::core::ffi::c_int {
                close((*eptr).upload_meta_fd);
                (*eptr).upload_meta_fd = -1 as ::core::ffi::c_int;
            }
            if (*eptr).upload_chain1_fd >= 0 as ::core::ffi::c_int {
                close((*eptr).upload_chain1_fd);
                (*eptr).upload_chain1_fd = -1 as ::core::ffi::c_int;
            }
            if (*eptr).upload_chain2_fd >= 0 as ::core::ffi::c_int {
                close((*eptr).upload_chain2_fd);
                (*eptr).upload_chain2_fd = -1 as ::core::ffi::c_int;
            }
        }
        if filenum as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (*eptr).upload_meta_fd = open(
                b"metadata.mfs.back\0".as_ptr() as *const ::core::ffi::c_char,
                O_RDONLY,
            );
            (*eptr).upload_chain1_fd = open(
                b"changelog.0.mfs\0".as_ptr() as *const ::core::ffi::c_char,
                O_RDONLY,
            );
            (*eptr).upload_chain2_fd = open(
                b"changelog.1.mfs\0".as_ptr() as *const ::core::ffi::c_char,
                O_RDONLY,
            );
        } else if filenum as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            (*eptr).upload_meta_fd = open(
                b"sessions.mfs\0".as_ptr() as *const ::core::ffi::c_char,
                O_RDONLY,
            );
        } else if filenum as ::core::ffi::c_int == 11 as ::core::ffi::c_int {
            if (*eptr).upload_meta_fd >= 0 as ::core::ffi::c_int {
                close((*eptr).upload_meta_fd);
            }
            (*eptr).upload_meta_fd = (*eptr).upload_chain1_fd;
            (*eptr).upload_chain1_fd = -1 as ::core::ffi::c_int;
        } else if filenum as ::core::ffi::c_int == 12 as ::core::ffi::c_int {
            if (*eptr).upload_meta_fd >= 0 as ::core::ffi::c_int {
                close((*eptr).upload_meta_fd);
            }
            (*eptr).upload_meta_fd = (*eptr).upload_chain2_fd;
            (*eptr).upload_chain2_fd = -1 as ::core::ffi::c_int;
        } else {
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if (*eptr).upload_meta_fd < 0 as ::core::ffi::c_int {
            if filenum as ::core::ffi::c_int == 11 as ::core::ffi::c_int
                || filenum as ::core::ffi::c_int == 12 as ::core::ffi::c_int
            {
                ptr =
                    matomlserv_create_packet(eptr, MATOAN_DOWNLOAD_INFO as uint32_t, 8 as uint32_t);
                put64bit(&raw mut ptr, 0 as uint64_t);
                return;
            } else {
                ptr =
                    matomlserv_create_packet(eptr, MATOAN_DOWNLOAD_INFO as uint32_t, 1 as uint32_t);
                put8bit(&raw mut ptr, 0xff as uint8_t);
                return;
            }
        }
        size = lseek((*eptr).upload_meta_fd, 0 as __off64_t, SEEK_END) as uint64_t;
        ptr = matomlserv_create_packet(eptr, MATOAN_DOWNLOAD_INFO as uint32_t, 8 as uint32_t);
        put64bit(&raw mut ptr, size);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_download_request(
    mut eptr: *mut matomlserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut offset: uint64_t = 0;
        let mut leng: uint32_t = 0;
        let mut crc: uint32_t = 0;
        let mut ret: ssize_t = 0;
        if length != 12 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOMA_DOWNLOAD_REQUEST - wrong size (%u/12)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if (*eptr).upload_meta_fd < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOMA_DOWNLOAD_REQUEST - file not opened\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        offset = get64bit(&raw mut data);
        leng = get32bit(&raw mut data);
        if leng > ML_META_DL_BLOCK as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOMA_DOWNLOAD_REQUEST - bad length\0".as_ptr() as *const ::core::ffi::c_char,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        ptr = matomlserv_create_packet(
            eptr,
            MATOAN_DOWNLOAD_DATA as uint32_t,
            (16 as uint32_t).wrapping_add(leng),
        );
        put64bit(&raw mut ptr, offset);
        put32bit(&raw mut ptr, leng);
        ret = pread(
            (*eptr).upload_meta_fd,
            ptr.offset(4 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            leng as size_t,
            offset as __off64_t,
        );
        if ret != leng as ssize_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"error reading metafile\0".as_ptr() as *const ::core::ffi::c_char,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        crc = mycrc32(
            0 as uint32_t,
            ptr.offset(4 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            leng,
        );
        put32bit(&raw mut ptr, crc);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_download_end(
    mut eptr: *mut matomlserventry,
    _data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        if length != 0 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOMA_DOWNLOAD_END - wrong size (%u/0)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if (*eptr).upload_meta_fd >= 0 as ::core::ffi::c_int {
            close((*eptr).upload_meta_fd);
            (*eptr).upload_meta_fd = -1 as ::core::ffi::c_int;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_broadcast_logstring(
    mut version: uint64_t,
    mut logstr: *mut uint8_t,
    mut logstrsize: uint32_t,
) {
    unsafe {
        let mut eptr: *mut matomlserventry = ::core::ptr::null_mut::<matomlserventry>();
        let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        eptr = matomlservhead;
        while !eptr.is_null() {
            if (*eptr).version > 0 as uint32_t
                && (*eptr).clienttype as ::core::ffi::c_int == METALOGGER as ::core::ffi::c_int
                && (*eptr).logstate as ::core::ffi::c_int == SYNC as ::core::ffi::c_int
            {
                data = matomlserv_create_packet(
                    eptr,
                    MATOAN_METACHANGES_LOG as uint32_t,
                    (9 as uint32_t).wrapping_add(logstrsize),
                );
                put8bit(&raw mut data, 0xff as uint8_t);
                put64bit(&raw mut data, version);
                memcpy(
                    data as *mut ::core::ffi::c_void,
                    logstr as *const ::core::ffi::c_void,
                    logstrsize as size_t,
                );
            }
            eptr = (*eptr).next as *mut matomlserventry;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_broadcast_logrotate() {
    unsafe {
        let mut eptr: *mut matomlserventry = ::core::ptr::null_mut::<matomlserventry>();
        let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        eptr = matomlservhead;
        while !eptr.is_null() {
            if (*eptr).version > 0 as uint32_t
                && (*eptr).clienttype as ::core::ffi::c_int == METALOGGER as ::core::ffi::c_int
            {
                data = matomlserv_create_packet(
                    eptr,
                    MATOAN_METACHANGES_LOG as uint32_t,
                    1 as uint32_t,
                );
                put8bit(&raw mut data, 0x55 as uint8_t);
            }
            eptr = (*eptr).next as *mut matomlserventry;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_beforeclose(mut eptr: *mut matomlserventry) {
    unsafe {
        if (*eptr).upload_meta_fd >= 0 as ::core::ffi::c_int {
            close((*eptr).upload_meta_fd);
            (*eptr).upload_meta_fd = -1 as ::core::ffi::c_int;
        }
        if (*eptr).upload_chain1_fd >= 0 as ::core::ffi::c_int {
            close((*eptr).upload_chain1_fd);
            (*eptr).upload_chain1_fd = -1 as ::core::ffi::c_int;
        }
        if (*eptr).upload_chain2_fd >= 0 as ::core::ffi::c_int {
            close((*eptr).upload_chain2_fd);
            (*eptr).upload_chain2_fd = -1 as ::core::ffi::c_int;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_gotpacket(
    mut eptr: *mut matomlserventry,
    mut r#type: uint32_t,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        match r#type {
            0 | 1 | 2 => {}
            10 => {
                matomlserv_get_version(eptr, data, length);
            }
            80 => {
                matomlserv_get_config(eptr, data, length);
            }
            71 => {
                matomlserv_syslog(eptr, data, length);
            }
            50 => {
                matomlserv_register(eptr, data, length);
            }
            65 => {
                matomlserv_store_metadata(eptr, data, length);
            }
            60 => {
                matomlserv_download_start(eptr, data, length);
            }
            62 => {
                matomlserv_download_request(eptr, data, length);
            }
            64 => {
                matomlserv_download_end(eptr, data, length);
            }
            _ => {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"master control module: got unknown message (type:%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    r#type,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_read(
    mut eptr: *mut matomlserventry,
    mut now: ::core::ffi::c_double,
) {
    unsafe {
        let mut i: int32_t = 0;
        let mut r#type: uint32_t = 0;
        let mut leng: uint32_t = 0;
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut rbleng: uint32_t = 0;
        let mut rbpos: uint32_t = 0;
        let mut err: uint8_t = 0;
        let mut hup: uint8_t = 0;
        static mut readbuff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        static mut readbuffsize: uint32_t = 0 as uint32_t;
        if eptr.is_null() {
            if !readbuff.is_null() {
                free(readbuff as *mut ::core::ffi::c_void);
            }
            readbuff = ::core::ptr::null_mut::<uint8_t>();
            readbuffsize = 0 as uint32_t;
            return;
        }
        if readbuffsize == 0 as uint32_t {
            readbuffsize = 65536 as uint32_t;
            readbuff = malloc(readbuffsize as size_t) as *mut uint8_t;
            if readbuff.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matomlserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    832 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matomlserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    832 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if readbuff
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint8_t
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matomlserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    832 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matomlserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    832 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
        }
        rbleng = 0 as uint32_t;
        err = 0 as uint8_t;
        hup = 0 as uint8_t;
        loop {
            i = read(
                (*eptr).sock,
                readbuff.offset(rbleng as isize) as *mut ::core::ffi::c_void,
                readbuffsize.wrapping_sub(rbleng) as size_t,
            ) as int32_t;
            if i == 0 as int32_t {
                hup = 1 as uint8_t;
                break;
            } else if i < 0 as int32_t {
                if *__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK {
                    err = 1 as uint8_t;
                }
                break;
            } else {
                rbleng = rbleng.wrapping_add(i as uint32_t);
                if rbleng != readbuffsize {
                    break;
                }
                readbuffsize = readbuffsize.wrapping_mul(2 as uint32_t);
                readbuff = mfsrealloc(readbuff as *mut ::core::ffi::c_void, readbuffsize as size_t)
                    as *mut uint8_t;
                if readbuff.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matomlserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        853 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matomlserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        853 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if readbuff
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut uint8_t
                {
                    let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matomlserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        853 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matomlserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        853 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_0,
                    );
                    abort();
                }
            }
        }
        if rbleng > 0 as uint32_t {
            (*eptr).lastread = now;
        }
        rbpos = 0 as uint32_t;
        while rbpos < rbleng {
            if rbleng.wrapping_sub(rbpos) >= (*eptr).input_bytesleft {
                memcpy(
                    (*eptr).input_startptr as *mut ::core::ffi::c_void,
                    readbuff.offset(rbpos as isize) as *const ::core::ffi::c_void,
                    (*eptr).input_bytesleft as size_t,
                );
                i = (*eptr).input_bytesleft as int32_t;
            } else {
                memcpy(
                    (*eptr).input_startptr as *mut ::core::ffi::c_void,
                    readbuff.offset(rbpos as isize) as *const ::core::ffi::c_void,
                    rbleng.wrapping_sub(rbpos) as size_t,
                );
                i = rbleng.wrapping_sub(rbpos) as int32_t;
            }
            rbpos = rbpos.wrapping_add(i as uint32_t);
            (*eptr).input_startptr = (*eptr).input_startptr.offset(i as isize);
            (*eptr).input_bytesleft = (*eptr).input_bytesleft.wrapping_sub(i as uint32_t);
            if (*eptr).input_bytesleft > 0 as uint32_t {
                break;
            }
            if (*eptr).input_packet.is_null() {
                ptr = &raw mut (*eptr).input_hdr as *mut uint8_t;
                r#type = get32bit(&raw mut ptr);
                leng = get32bit(&raw mut ptr);
                if leng > MaxPacketSize as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"ML(%s) packet too long (%u/%u) ; command:%u\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*eptr).servstrip,
                        leng,
                        MaxPacketSize,
                        r#type,
                    );
                    (*eptr).input_end = 1 as uint8_t;
                    return;
                }
                (*eptr).input_packet =
                    malloc((16 as size_t).wrapping_add(leng as size_t)) as *mut in_packetstruct;
                if (*eptr).input_packet.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matomlserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        893 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr->input_packet\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matomlserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        893 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr->input_packet\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if (*eptr).input_packet
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut in_packetstruct
                {
                    let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matomlserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        893 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr->input_packet\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matomlserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        893 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr->input_packet\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_1,
                    );
                    abort();
                }
                (*(*eptr).input_packet).next = ::core::ptr::null_mut::<in_packetstruct>();
                (*(*eptr).input_packet).r#type = r#type;
                (*(*eptr).input_packet).leng = leng;
                (*eptr).input_startptr = &raw mut (*(*eptr).input_packet).data as *mut uint8_t;
                (*eptr).input_bytesleft = leng;
            }
            if (*eptr).input_bytesleft > 0 as uint32_t {
                continue;
            }
            if !(*eptr).input_packet.is_null() {
                *(*eptr).inputtail = (*eptr).input_packet;
                (*eptr).inputtail =
                    &raw mut (*(*eptr).input_packet).next as *mut *mut in_packetstruct;
                (*eptr).input_packet = ::core::ptr::null_mut::<in_packetstruct>();
                (*eptr).input_bytesleft = 8 as uint32_t;
                (*eptr).input_startptr = &raw mut (*eptr).input_hdr as *mut uint8_t;
            }
        }
        if hup != 0 {
            if (*eptr).clienttype as ::core::ffi::c_int != SUPERVISOR as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_NOTICE,
                    b"connection with %s(%s) has been closed by peer\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    matomlserv_clientname(eptr),
                    (*eptr).servstrip,
                );
            }
            (*eptr).input_end = 1 as uint8_t;
        } else if err != 0 {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"read from ML(%s) error\0".as_ptr() as *const ::core::ffi::c_char,
                (*eptr).servstrip,
            );
            (*eptr).input_end = 1 as uint8_t;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_parse(mut eptr: *mut matomlserventry) {
    unsafe {
        let mut ipack: *mut in_packetstruct = ::core::ptr::null_mut::<in_packetstruct>();
        let mut starttime: uint64_t = 0;
        let mut currtime: uint64_t = 0;
        starttime = monotonic_useconds();
        currtime = starttime;
        while (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
            && {
                ipack = (*eptr).inputhead;
                !ipack.is_null()
            }
            && starttime.wrapping_add(10000 as uint64_t) > currtime
        {
            matomlserv_gotpacket(
                eptr,
                (*ipack).r#type,
                &raw mut (*ipack).data as *mut uint8_t,
                (*ipack).leng,
            );
            (*eptr).inputhead = (*ipack).next as *mut in_packetstruct;
            free(ipack as *mut ::core::ffi::c_void);
            if (*eptr).inputhead.is_null() {
                (*eptr).inputtail = &raw mut (*eptr).inputhead;
            } else {
                currtime = monotonic_useconds();
            }
        }
        if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
            && (*eptr).inputhead.is_null()
            && (*eptr).input_end as ::core::ffi::c_int != 0
        {
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_write(
    mut eptr: *mut matomlserventry,
    mut now: ::core::ffi::c_double,
) {
    unsafe {
        let mut opack: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
        let mut i: int32_t = 0;
        let mut iovtab: [iovec; 100] = [iovec {
            iov_base: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            iov_len: 0,
        }; 100];
        let mut iovdata: uint32_t = 0;
        let mut leng: uint32_t = 0;
        let mut left: uint32_t = 0;
        loop {
            leng = 0 as uint32_t;
            iovdata = 0 as uint32_t;
            opack = (*eptr).outputhead;
            while iovdata < 100 as uint32_t && !opack.is_null() {
                iovtab[iovdata as usize].iov_base = (*opack).startptr as *mut ::core::ffi::c_void;
                iovtab[iovdata as usize].iov_len = (*opack).bytesleft as size_t;
                leng = leng.wrapping_add((*opack).bytesleft);
                iovdata = iovdata.wrapping_add(1);
                opack = (*opack).next as *mut out_packetstruct;
            }
            if iovdata == 0 as uint32_t {
                return;
            }
            i = writev(
                (*eptr).sock,
                &raw mut iovtab as *mut iovec,
                iovdata as ::core::ffi::c_int,
            ) as int32_t;
            if i < 0 as int32_t {
                if *__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"write to ML(%s) error\0".as_ptr() as *const ::core::ffi::c_char,
                        (*eptr).servstrip,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                }
                return;
            }
            if i > 0 as int32_t {
                (*eptr).lastwrite = now;
            }
            left = i as uint32_t;
            while left > 0 as uint32_t && !(*eptr).outputhead.is_null() {
                opack = (*eptr).outputhead;
                if (*opack).bytesleft > left {
                    (*opack).startptr = (*opack).startptr.offset(left as isize);
                    (*opack).bytesleft = (*opack).bytesleft.wrapping_sub(left);
                    left = 0 as uint32_t;
                } else {
                    left = left.wrapping_sub((*opack).bytesleft);
                    (*eptr).outputhead = (*opack).next as *mut out_packetstruct;
                    if (*eptr).outputhead.is_null() {
                        (*eptr).outputtail = &raw mut (*eptr).outputhead;
                    }
                    free(opack as *mut ::core::ffi::c_void);
                }
            }
            if (i as uint32_t) < leng {
                return;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_desc(mut pdesc: *mut pollfd, mut ndesc: *mut uint32_t) {
    unsafe {
        let mut pos: uint32_t = *ndesc;
        let mut events: ::core::ffi::c_int = 0;
        let mut eptr: *mut matomlserventry = ::core::ptr::null_mut::<matomlserventry>();
        (*pdesc.offset(pos as isize)).fd = lsock;
        (*pdesc.offset(pos as isize)).events = POLLIN as ::core::ffi::c_short;
        lsockpdescpos = pos as int32_t;
        pos = pos.wrapping_add(1);
        eptr = matomlservhead;
        while !eptr.is_null() {
            events = 0 as ::core::ffi::c_int;
            if (*eptr).input_end as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                events |= POLLIN;
            }
            if !(*eptr).outputhead.is_null() {
                events |= POLLOUT;
            }
            if events != 0 {
                (*pdesc.offset(pos as isize)).fd = (*eptr).sock;
                (*pdesc.offset(pos as isize)).events = POLLIN as ::core::ffi::c_short;
                (*eptr).pdescpos = pos as int32_t;
                pos = pos.wrapping_add(1);
            } else {
                (*eptr).pdescpos = -1 as ::core::ffi::c_int as int32_t;
            }
            eptr = (*eptr).next as *mut matomlserventry;
        }
        *ndesc = pos;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_disconnection_loop() {
    unsafe {
        let mut eptr: *mut matomlserventry = ::core::ptr::null_mut::<matomlserventry>();
        let mut kptr: *mut *mut matomlserventry = ::core::ptr::null_mut::<*mut matomlserventry>();
        let mut ipptr: *mut in_packetstruct = ::core::ptr::null_mut::<in_packetstruct>();
        let mut ipaptr: *mut in_packetstruct = ::core::ptr::null_mut::<in_packetstruct>();
        let mut opptr: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
        let mut opaptr: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
        kptr = &raw mut matomlservhead;
        loop {
            eptr = *kptr;
            if eptr.is_null() {
                break;
            }
            if (*eptr).mode as ::core::ffi::c_int == KILL as ::core::ffi::c_int
                || (*eptr).mode as ::core::ffi::c_int == CLOSE as ::core::ffi::c_int
            {
                matomlserv_beforeclose(eptr);
                if (*eptr).mode as ::core::ffi::c_int == KILL as ::core::ffi::c_int {
                    tcpclose((*eptr).sock);
                } else {
                    close((*eptr).sock);
                }
                if !(*eptr).input_packet.is_null() {
                    free((*eptr).input_packet as *mut ::core::ffi::c_void);
                }
                ipptr = (*eptr).inputhead;
                while !ipptr.is_null() {
                    ipaptr = ipptr;
                    ipptr = (*ipptr).next as *mut in_packetstruct;
                    free(ipaptr as *mut ::core::ffi::c_void);
                }
                opptr = (*eptr).outputhead;
                while !opptr.is_null() {
                    opaptr = opptr;
                    opptr = (*opptr).next as *mut out_packetstruct;
                    free(opaptr as *mut ::core::ffi::c_void);
                }
                if !(*eptr).servstrip.is_null() {
                    free((*eptr).servstrip as *mut ::core::ffi::c_void);
                }
                *kptr = (*eptr).next as *mut matomlserventry;
                free(eptr as *mut ::core::ffi::c_void);
            } else {
                kptr = &raw mut (*eptr).next as *mut *mut matomlserventry;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_serve(mut pdesc: *mut pollfd) {
    unsafe {
        let mut now: ::core::ffi::c_double = 0.;
        let mut eptr: *mut matomlserventry = ::core::ptr::null_mut::<matomlserventry>();
        let mut ns: ::core::ffi::c_int = 0;
        static mut lastaction: ::core::ffi::c_double = 0.0f64;
        let mut timeoutadd: ::core::ffi::c_double = 0.;
        let mut n: uint32_t = 0;
        now = monotonic_seconds();
        if lastaction > 0.0f64 {
            timeoutadd = now - lastaction;
            if timeoutadd > 1.0f64 {
                eptr = matomlservhead;
                while !eptr.is_null() {
                    (*eptr).lastread += timeoutadd;
                    eptr = (*eptr).next as *mut matomlserventry;
                }
            }
        }
        lastaction = now;
        if lsockpdescpos >= 0 as int32_t
            && (*pdesc.offset(lsockpdescpos as isize)).revents as ::core::ffi::c_int & POLLIN != 0
        {
            ns = tcpaccept(lsock);
            if ns < 0 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"Master<->ML socket: accept error\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                tcpnonblock(ns);
                tcpnodelay(ns);
                eptr = malloc(::core::mem::size_of::<matomlserventry>()) as *mut matomlserventry;
                if eptr.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matomlserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1125 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matomlserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1125 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if eptr
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut matomlserventry
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matomlserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1125 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matomlserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1125 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
                (*eptr).next = matomlservhead as *mut matomlserventry;
                matomlservhead = eptr;
                (*eptr).sock = ns;
                (*eptr).pdescpos = -1 as ::core::ffi::c_int as int32_t;
                (*eptr).mode = DATA as ::core::ffi::c_int as uint8_t;
                (*eptr).lastread = now;
                (*eptr).lastwrite = now;
                (*eptr).input_bytesleft = 8 as uint32_t;
                (*eptr).input_startptr = &raw mut (*eptr).input_hdr as *mut uint8_t;
                (*eptr).input_end = 0 as uint8_t;
                (*eptr).input_packet = ::core::ptr::null_mut::<in_packetstruct>();
                (*eptr).inputhead = ::core::ptr::null_mut::<in_packetstruct>();
                (*eptr).inputtail = &raw mut (*eptr).inputhead;
                (*eptr).outputhead = ::core::ptr::null_mut::<out_packetstruct>();
                (*eptr).outputtail = &raw mut (*eptr).outputhead;
                if ForceTimeout > 0 as uint32_t {
                    (*eptr).timeout = ForceTimeout as uint16_t;
                } else {
                    (*eptr).timeout = DefaultTimeout as uint16_t;
                }
                tcpgetpeer(
                    (*eptr).sock,
                    &raw mut (*eptr).servip,
                    ::core::ptr::null_mut::<uint16_t>(),
                );
                (*eptr).servstrip = univallocstrip((*eptr).servip);
                (*eptr).version = 0 as uint32_t;
                (*eptr).clienttype = UNKNOWN as ::core::ffi::c_int as uint8_t;
                (*eptr).logstate = NONE as ::core::ffi::c_int as uint8_t;
                (*eptr).upload_meta_fd = -1 as ::core::ffi::c_int;
                (*eptr).upload_chain1_fd = -1 as ::core::ffi::c_int;
                (*eptr).upload_chain2_fd = -1 as ::core::ffi::c_int;
            }
        }
        eptr = matomlservhead;
        while !eptr.is_null() {
            if (*eptr).pdescpos >= 0 as int32_t {
                if (*pdesc.offset((*eptr).pdescpos as isize)).revents as ::core::ffi::c_int
                    & (POLLERR | POLLIN)
                    == POLLIN
                    && (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                {
                    matomlserv_read(eptr, now);
                }
                if (*pdesc.offset((*eptr).pdescpos as isize)).revents as ::core::ffi::c_int
                    & (POLLERR | POLLHUP)
                    != 0
                {
                    (*eptr).input_end = 1 as uint8_t;
                }
            }
            matomlserv_parse(eptr);
            eptr = (*eptr).next as *mut matomlserventry;
        }
        eptr = matomlservhead;
        while !eptr.is_null() {
            if (*eptr).lastwrite + 1.0f64 < now
                && (*eptr).outputhead.is_null()
                && (*eptr).clienttype as ::core::ffi::c_int != UNKNOWN as ::core::ffi::c_int
                && (*eptr).clienttype as ::core::ffi::c_int != SUPERVISOR as ::core::ffi::c_int
            {
                matomlserv_create_packet(eptr, ANTOAN_NOP as uint32_t, 0 as uint32_t);
            }
            if (*eptr).pdescpos >= 0 as int32_t {
                if ((*pdesc.offset((*eptr).pdescpos as isize)).events as ::core::ffi::c_int
                    & POLLOUT
                    == 0 as ::core::ffi::c_int
                    && !(*eptr).outputhead.is_null()
                    || (*pdesc.offset((*eptr).pdescpos as isize)).revents as ::core::ffi::c_int
                        & POLLOUT
                        != 0)
                    && (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                {
                    matomlserv_write(eptr, now);
                }
            }
            if ((*eptr).lastread + (*eptr).timeout as ::core::ffi::c_int as ::core::ffi::c_double)
                < now
            {
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            }
            if (*eptr).logstate as ::core::ffi::c_int == DELAYED as ::core::ffi::c_int
                && (*eptr).outputhead.is_null()
            {
                n = changelog_get_old_changes(
                    (*eptr).next_log_version,
                    Some(
                        matomlserv_send_old_change
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                uint64_t,
                                *mut uint8_t,
                                uint32_t,
                            ) -> (),
                    ),
                    eptr as *mut ::core::ffi::c_void,
                    OLD_CHANGES_GROUP_COUNT as uint32_t,
                );
                if n < OLD_CHANGES_GROUP_COUNT as uint32_t {
                    (*eptr).logstate = SYNC as ::core::ffi::c_int as uint8_t;
                } else {
                    (*eptr).next_log_version = (*eptr).next_log_version.wrapping_add(n as uint64_t);
                }
            }
            eptr = (*eptr).next as *mut matomlserventry;
        }
        matomlserv_disconnection_loop();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_get_min_version() -> uint64_t {
    unsafe {
        let mut eptr: *mut matomlserventry = ::core::ptr::null_mut::<matomlserventry>();
        let mut minversion: uint64_t = 0;
        minversion = meta_version();
        eptr = matomlservhead;
        while !eptr.is_null() {
            if (*eptr).logstate as ::core::ffi::c_int == DELAYED as ::core::ffi::c_int {
                if (*eptr).next_log_version < minversion {
                    minversion = (*eptr).next_log_version;
                }
            }
            eptr = (*eptr).next as *mut matomlserventry;
        }
        return minversion;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_keep_alive() {
    unsafe {
        let mut now: ::core::ffi::c_double = 0.;
        let mut eptr: *mut matomlserventry = ::core::ptr::null_mut::<matomlserventry>();
        now = monotonic_seconds();
        eptr = matomlservhead;
        while !eptr.is_null() {
            if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
                && (*eptr).input_end as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                matomlserv_read(eptr, now);
            }
            eptr = (*eptr).next as *mut matomlserventry;
        }
        eptr = matomlservhead;
        while !eptr.is_null() {
            if (*eptr).lastwrite + 1.0f64 < now
                && (*eptr).outputhead.is_null()
                && (*eptr).clienttype as ::core::ffi::c_int != UNKNOWN as ::core::ffi::c_int
                && (*eptr).clienttype as ::core::ffi::c_int != SUPERVISOR as ::core::ffi::c_int
            {
                matomlserv_create_packet(eptr, ANTOAN_NOP as uint32_t, 0 as uint32_t);
            }
            if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
                && !(*eptr).outputhead.is_null()
            {
                matomlserv_write(eptr, now);
            }
            eptr = (*eptr).next as *mut matomlserventry;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_close_lsock() {
    unsafe {
        if lsock >= 0 as ::core::ffi::c_int {
            close(lsock);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_term() {
    unsafe {
        let mut eptr: *mut matomlserventry = ::core::ptr::null_mut::<matomlserventry>();
        let mut eaptr: *mut matomlserventry = ::core::ptr::null_mut::<matomlserventry>();
        let mut ipptr: *mut in_packetstruct = ::core::ptr::null_mut::<in_packetstruct>();
        let mut ipaptr: *mut in_packetstruct = ::core::ptr::null_mut::<in_packetstruct>();
        let mut opptr: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
        let mut opaptr: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_INFO,
            b"master control module: closing %s:%s\0".as_ptr() as *const ::core::ffi::c_char,
            ListenHost,
            ListenPort,
        );
        tcpclose(lsock);
        eptr = matomlservhead;
        while !eptr.is_null() {
            if !(*eptr).input_packet.is_null() {
                free((*eptr).input_packet as *mut ::core::ffi::c_void);
            }
            ipptr = (*eptr).inputhead;
            while !ipptr.is_null() {
                ipaptr = ipptr;
                ipptr = (*ipptr).next as *mut in_packetstruct;
                free(ipaptr as *mut ::core::ffi::c_void);
            }
            opptr = (*eptr).outputhead;
            while !opptr.is_null() {
                opaptr = opptr;
                opptr = (*opptr).next as *mut out_packetstruct;
                free(opaptr as *mut ::core::ffi::c_void);
            }
            eaptr = eptr;
            eptr = (*eptr).next as *mut matomlserventry;
            free(eaptr as *mut ::core::ffi::c_void);
        }
        matomlservhead = ::core::ptr::null_mut::<matomlserventry>();
        matomlserv_read(::core::ptr::null_mut::<matomlserventry>(), 0.0f64);
        free(ListenHost as *mut ::core::ffi::c_void);
        free(ListenPort as *mut ::core::ffi::c_void);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_no_more_pending_jobs() -> ::core::ffi::c_int {
    unsafe {
        let mut eptr: *mut matomlserventry = ::core::ptr::null_mut::<matomlserventry>();
        eptr = matomlservhead;
        while !eptr.is_null() {
            if !(*eptr).outputhead.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            eptr = (*eptr).next as *mut matomlserventry;
        }
        return 1 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_disconnect_all() {
    unsafe {
        let mut eptr: *mut matomlserventry = ::core::ptr::null_mut::<matomlserventry>();
        eptr = matomlservhead;
        while !eptr.is_null() {
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            eptr = (*eptr).next as *mut matomlserventry;
        }
        matomlserv_disconnection_loop();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_getport() -> uint16_t {
    unsafe {
        return listenport;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_getportstr() -> *const ::core::ffi::c_char {
    unsafe {
        return ListenPort;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_reload_common() {
    unsafe {
        DefaultTimeout = cfg_getuint32(
            b"MATOML_TIMEOUT\0".as_ptr() as *const ::core::ffi::c_char,
            10 as uint32_t,
        );
        if DefaultTimeout > 65535 as uint32_t {
            DefaultTimeout = 65535 as uint32_t;
        } else if DefaultTimeout < 10 as uint32_t {
            DefaultTimeout = 10 as uint32_t;
        }
        ForceTimeout = cfg_getuint32(
            b"MATOML_FORCE_TIMEOUT\0".as_ptr() as *const ::core::ffi::c_char,
            0 as uint32_t,
        );
        if ForceTimeout > 0 as uint32_t && ForceTimeout < 10 as uint32_t {
            ForceTimeout = 10 as uint32_t;
        }
        if ForceTimeout > 65535 as uint32_t {
            ForceTimeout = 65535 as uint32_t;
        }
        BackMetaCopies = cfg_getuint32(
            b"BACK_META_KEEP_PREVIOUS\0".as_ptr() as *const ::core::ffi::c_char,
            1 as uint32_t,
        );
        if BackMetaCopies > 99 as uint32_t {
            BackMetaCopies = 99 as uint32_t;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_reload() {
    unsafe {
        let mut oldListenHost: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut oldListenPort: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut oldlistenip: uint32_t = 0;
        let mut oldlistenport: uint16_t = 0;
        let mut newlsock: ::core::ffi::c_int = 0;
        matomlserv_reload_common();
        oldListenHost = ListenHost;
        oldListenPort = ListenPort;
        oldlistenip = listenip;
        oldlistenport = listenport;
        ListenHost = cfg_getstr(
            b"MATOML_LISTEN_HOST\0".as_ptr() as *const ::core::ffi::c_char,
            b"*\0".as_ptr() as *const ::core::ffi::c_char,
        );
        ListenPort = cfg_getstr(
            b"MATOML_LISTEN_PORT\0".as_ptr() as *const ::core::ffi::c_char,
            DEFAULT_MASTER_CONTROL_PORT.as_ptr(),
        );
        if strcmp(oldListenHost, ListenHost) == 0 as ::core::ffi::c_int
            && strcmp(oldListenPort, ListenPort) == 0 as ::core::ffi::c_int
        {
            free(oldListenHost as *mut ::core::ffi::c_void);
            free(oldListenPort as *mut ::core::ffi::c_void);
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_INFO,
                b"master <-> metaloggers module: socket address hasn't changed (%s:%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ListenHost,
                ListenPort,
            );
            return;
        }
        newlsock = tcpsocket();
        if newlsock < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"master <-> metaloggers module: socket address has changed, but can't create new socket\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            free(ListenHost as *mut ::core::ffi::c_void);
            free(ListenPort as *mut ::core::ffi::c_void);
            ListenHost = oldListenHost;
            ListenPort = oldListenPort;
            return;
        }
        tcpnonblock(newlsock);
        tcpnodelay(newlsock);
        tcpreuseaddr(newlsock);
        if tcpresolve(
            ListenHost,
            ListenPort,
            &raw mut listenip,
            &raw mut listenport,
            1 as ::core::ffi::c_int,
        ) < 0 as ::core::ffi::c_int
        {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"master <-> metaloggers module: socket address has changed, but can't be resolved (%s:%s)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                ListenHost,
                ListenPort,
            );
            free(ListenHost as *mut ::core::ffi::c_void);
            free(ListenPort as *mut ::core::ffi::c_void);
            ListenHost = oldListenHost;
            ListenPort = oldListenPort;
            listenip = oldlistenip;
            listenport = oldlistenport;
            tcpclose(newlsock);
            return;
        }
        if tcpnumlisten(newlsock, listenip, listenport, 100 as uint16_t) < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"master <-> metaloggers module: socket address has changed, but can't listen on socket (%s:%s)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                ListenHost,
                ListenPort,
            );
            free(ListenHost as *mut ::core::ffi::c_void);
            free(ListenPort as *mut ::core::ffi::c_void);
            ListenHost = oldListenHost;
            ListenPort = oldListenPort;
            listenip = oldlistenip;
            listenport = oldlistenport;
            tcpclose(newlsock);
            return;
        }
        if tcpsetacceptfilter(newlsock) < 0 as ::core::ffi::c_int && *__errno_location() != ENOTSUP
        {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_NOTICE,
                b"master <-> metaloggers module: can't set accept filter\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_INFO,
            b"master <-> metaloggers module: socket address has changed, now listen on %s:%s\0"
                .as_ptr() as *const ::core::ffi::c_char,
            ListenHost,
            ListenPort,
        );
        free(oldListenHost as *mut ::core::ffi::c_void);
        free(oldListenPort as *mut ::core::ffi::c_void);
        tcpclose(lsock);
        lsock = newlsock;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matomlserv_init() -> ::core::ffi::c_int {
    unsafe {
        matomlserv_reload_common();
        ListenHost = cfg_getstr(
            b"MATOML_LISTEN_HOST\0".as_ptr() as *const ::core::ffi::c_char,
            b"*\0".as_ptr() as *const ::core::ffi::c_char,
        );
        ListenPort = cfg_getstr(
            b"MATOML_LISTEN_PORT\0".as_ptr() as *const ::core::ffi::c_char,
            DEFAULT_MASTER_CONTROL_PORT.as_ptr(),
        );
        lsock = tcpsocket();
        if lsock < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"master <-> metaloggers module: can't create socket\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        tcpnonblock(lsock);
        tcpnodelay(lsock);
        tcpreuseaddr(lsock);
        if tcpresolve(
            ListenHost,
            ListenPort,
            &raw mut listenip,
            &raw mut listenport,
            1 as ::core::ffi::c_int,
        ) < 0 as ::core::ffi::c_int
        {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"master <-> metaloggers module: can't resolve %s:%s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ListenHost,
                ListenPort,
            );
            return -1 as ::core::ffi::c_int;
        }
        if tcpnumlisten(lsock, listenip, listenport, 100 as uint16_t) < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"master <-> metaloggers module: can't listen on %s:%s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ListenHost,
                ListenPort,
            );
            return -1 as ::core::ffi::c_int;
        }
        if tcpsetacceptfilter(lsock) < 0 as ::core::ffi::c_int && *__errno_location() != ENOTSUP {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_NOTICE,
                b"master <-> metaloggers module: can't set accept filter\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_INFO,
            b"master <-> metaloggers module: listen on %s:%s\0".as_ptr()
                as *const ::core::ffi::c_char,
            ListenHost,
            ListenPort,
        );
        matomlservhead = ::core::ptr::null_mut::<matomlserventry>();
        main_reload_register_fname(
            Some(matomlserv_reload as unsafe extern "C" fn() -> ()),
            b"matomlserv_reload\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_destruct_register_fname(
            Some(matomlserv_term as unsafe extern "C" fn() -> ()),
            b"matomlserv_term\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_poll_register_fname(
            Some(matomlserv_desc as unsafe extern "C" fn(*mut pollfd, *mut uint32_t) -> ()),
            Some(matomlserv_serve as unsafe extern "C" fn(*mut pollfd) -> ()),
            b"matomlserv_desc\0".as_ptr() as *const ::core::ffi::c_char,
            b"matomlserv_serve\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_keepalive_register_fname(
            Some(matomlserv_keep_alive as unsafe extern "C" fn() -> ()),
            b"matomlserv_keep_alive\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_time_register_fname(
            10 as uint32_t,
            0 as uint32_t,
            Some(matomlserv_broadcast_timeout as unsafe extern "C" fn() -> ()),
            b"matomlserv_broadcast_timeout\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
}
