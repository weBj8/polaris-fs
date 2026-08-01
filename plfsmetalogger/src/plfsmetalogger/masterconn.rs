pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
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
    unsafe fn pwrite(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __nbytes: size_t,
        __offset: __off64_t,
    ) -> ssize_t;
    unsafe fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn fsync(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn ftruncate(__fd: ::core::ffi::c_int, __length: __off64_t) -> ::core::ffi::c_int;
    static mut stderr: *mut FILE;
    unsafe fn rename(
        __old: *const ::core::ffi::c_char,
        __new: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
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
    unsafe fn memmove(
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
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn mycrc32(crc: uint32_t, block: *const ::core::ffi::c_void, leng: uint32_t)
    -> uint32_t;
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
    unsafe fn main_info_register_fname(
        fun: Option<unsafe extern "C" fn(*mut FILE) -> ()>,
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
    unsafe fn main_time_change(
        x: *mut ::core::ffi::c_void,
        seconds: uint32_t,
        offset: uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn univmakestrip(strip: *mut ::core::ffi::c_char, ip: uint32_t);
    unsafe fn univmakestripport(stripport: *mut ::core::ffi::c_char, ip: uint32_t, port: uint16_t);
    unsafe fn tcpsocket() -> ::core::ffi::c_int;
    unsafe fn tcpresolve(
        hostname: *const ::core::ffi::c_char,
        service: *const ::core::ffi::c_char,
        ip: *mut uint32_t,
        port: *mut uint16_t,
        passiveflag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn tcpnonblock(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpgetstatus(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpnodelay(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpnumbind(
        sock: ::core::ffi::c_int,
        ip: uint32_t,
        port: uint16_t,
    ) -> ::core::ffi::c_int;
    unsafe fn tcpnumconnect(
        sock: ::core::ffi::c_int,
        ip: uint32_t,
        port: uint16_t,
    ) -> ::core::ffi::c_int;
    unsafe fn tcpclose(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
    unsafe fn monotonic_useconds() -> uint64_t;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type ssize_t = isize;
pub type int32_t = i32;
pub type int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct masterconn {
    pub mode: uint8_t,
    pub sock: ::core::ffi::c_int,
    pub pdescpos: int32_t,
    pub lastread: ::core::ffi::c_double,
    pub lastwrite: ::core::ffi::c_double,
    pub conntime: ::core::ffi::c_double,
    pub input_hdr: [uint8_t; 8],
    pub input_startptr: *mut uint8_t,
    pub input_bytesleft: uint32_t,
    pub input_end: uint8_t,
    pub input_packet: *mut in_packetstruct,
    pub inputhead: *mut in_packetstruct,
    pub inputtail: *mut *mut in_packetstruct,
    pub outputhead: *mut out_packetstruct,
    pub outputtail: *mut *mut out_packetstruct,
    pub bindip: uint32_t,
    pub masterip: uint32_t,
    pub masterport: uint16_t,
    pub timeout: uint16_t,
    pub masteraddrvalid: uint8_t,
    pub downloadretrycnt: uint8_t,
    pub downloading: uint8_t,
    pub oldmode: uint8_t,
    pub logfd: *mut FILE,
    pub metafd: ::core::ffi::c_int,
    pub filesize: uint64_t,
    pub dloffset: uint64_t,
    pub dlstartuts: uint64_t,
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
pub const KILL: C2Rust_Unnamed = 3;
pub const DATA: C2Rust_Unnamed = 2;
pub const CONNECTING: C2Rust_Unnamed = 1;
pub const FREE: C2Rust_Unnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
}
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const __S_IFREG: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_TRUNC: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const S_IFMT: ::core::ffi::c_int = __S_IFMT;
pub const S_IFREG: ::core::ffi::c_int = __S_IFREG;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ANTOMA_MAXPACKETSIZE: ::core::ffi::c_int = 1500000 as ::core::ffi::c_int;
pub const MATOAN_MAXPACKETSIZE: ::core::ffi::c_int = 1500000 as ::core::ffi::c_int;
pub const ANTOAN_NOP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ANTOAN_UNKNOWN_COMMAND: uint32_t = 1 as uint32_t;
pub const ANTOAN_BAD_COMMAND_SIZE: uint32_t = 2 as uint32_t;
pub const ANTOAN_FORCE_TIMEOUT: uint32_t = 5 as uint32_t;
pub const ANTOMA_REGISTER: ::core::ffi::c_int = PROTO_BASE + 50 as ::core::ffi::c_int;
pub const MATOAN_METACHANGES_LOG: uint32_t = 51 as uint32_t;
pub const MATOAN_MASTER_ACK: uint32_t = 52 as uint32_t;
pub const ANTOMA_DOWNLOAD_START: ::core::ffi::c_int = PROTO_BASE + 60 as ::core::ffi::c_int;
pub const MATOAN_DOWNLOAD_INFO: uint32_t = 61 as uint32_t;
pub const ANTOMA_DOWNLOAD_REQUEST: ::core::ffi::c_int = PROTO_BASE + 62 as ::core::ffi::c_int;
pub const MATOAN_DOWNLOAD_DATA: uint32_t = 63 as uint32_t;
pub const ANTOMA_DOWNLOAD_END: ::core::ffi::c_int = PROTO_BASE + 64 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const DEFAULT_MASTERNAME: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"mfsmaster\0") };
pub const DEFAULT_MASTER_CONTROL_PORT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"9419\0") };
pub const PROTO_BASE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const VERSMAJ: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const VERSMID: ::core::ffi::c_int = 59 as ::core::ffi::c_int;
pub const VERSMIN: ::core::ffi::c_int = 2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
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
pub const META_DL_BLOCK: ::core::ffi::c_int =
    if (MATOAN_MAXPACKETSIZE - 1000 as ::core::ffi::c_int) < 1000000 as ::core::ffi::c_int {
        MATOAN_MAXPACKETSIZE - 1000 as ::core::ffi::c_int
    } else {
        1000000 as ::core::ffi::c_int
    };
static mut masterconnsingleton: *mut masterconn = ::core::ptr::null_mut::<masterconn>();
static mut BackLogsNumber: uint32_t = 0;
static mut BackMetaCopies: uint32_t = 0;
static mut MasterHost: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut MasterPort: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut BindHost: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut Timeout: uint32_t = 0;
static mut reconnect_hook: *mut ::core::ffi::c_void =
    ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut download_hook: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut lastlogversion: uint64_t = 0 as uint64_t;
static mut stats_bytesout: uint32_t = 0 as uint32_t;
static mut stats_bytesin: uint32_t = 0 as uint32_t;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_stats(mut bin: *mut uint32_t, mut bout: *mut uint32_t) {
    unsafe {
        *bin = stats_bytesin;
        *bout = stats_bytesout;
        stats_bytesin = 0 as uint32_t;
        stats_bytesout = 0 as uint32_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_findlastlogversion() {
    unsafe {
        let mut st: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        let mut buff: [uint8_t; 32800] = [0; 32800];
        let mut size: uint64_t = 0;
        let mut buffpos: uint32_t = 0;
        let mut lastnewline: uint64_t = 0;
        let mut fd: ::core::ffi::c_int = 0;
        lastlogversion = 0 as uint64_t;
        if stat(
            b"metadata_ml.mfs.back\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut st,
        ) < 0 as ::core::ffi::c_int
            || st.st_size == 0 as __off_t
            || st.st_mode & S_IFMT as __mode_t != S_IFREG as __mode_t
        {
            return;
        }
        fd = open(
            b"changelog_ml.0.back\0".as_ptr() as *const ::core::ffi::c_char,
            O_RDWR,
        );
        if fd < 0 as ::core::ffi::c_int {
            return;
        }
        fstat(fd, &raw mut st);
        size = st.st_size as uint64_t;
        memset(
            &raw mut buff as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            32 as size_t,
        );
        lastnewline = 0 as uint64_t;
        while size > 0 as uint64_t && size.wrapping_add(200000 as uint64_t) > st.st_size as uint64_t
        {
            if size > 32768 as uint64_t {
                memcpy(
                    (&raw mut buff as *mut uint8_t).offset(32768 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_void,
                    &raw mut buff as *mut uint8_t as *const ::core::ffi::c_void,
                    32 as size_t,
                );
                size = size.wrapping_sub(32768 as uint64_t);
                lseek(fd, size as __off64_t, SEEK_SET);
                if read(
                    fd,
                    &raw mut buff as *mut uint8_t as *mut ::core::ffi::c_void,
                    32768 as size_t,
                ) != 32768 as ::core::ffi::c_int as ssize_t
                {
                    lastlogversion = 0 as uint64_t;
                    close(fd);
                    return;
                }
                buffpos = 32768 as uint32_t;
            } else {
                memmove(
                    (&raw mut buff as *mut uint8_t).offset(size as isize)
                        as *mut ::core::ffi::c_void,
                    &raw mut buff as *mut uint8_t as *const ::core::ffi::c_void,
                    32 as size_t,
                );
                lseek(fd, 0 as __off64_t, SEEK_SET);
                if read(
                    fd,
                    &raw mut buff as *mut uint8_t as *mut ::core::ffi::c_void,
                    size as size_t,
                ) != size as ssize_t
                {
                    lastlogversion = 0 as uint64_t;
                    close(fd);
                    return;
                }
                buffpos = size as uint32_t;
                size = 0 as uint64_t;
            }
            while buffpos > 0 as uint32_t {
                buffpos = buffpos.wrapping_sub(1);
                if buff[buffpos as usize] as ::core::ffi::c_int == '\n' as ::core::ffi::c_int {
                    if lastnewline == 0 as uint64_t {
                        lastnewline = size.wrapping_add(buffpos as uint64_t);
                    } else {
                        if lastnewline.wrapping_add(1 as uint64_t) != st.st_size as uint64_t {
                            if ftruncate(fd, lastnewline.wrapping_add(1 as uint64_t) as __off64_t)
                                < 0 as ::core::ffi::c_int
                            {
                                lastlogversion = 0 as uint64_t;
                                close(fd);
                                return;
                            }
                        }
                        buffpos = buffpos.wrapping_add(1);
                        while buffpos < 32800 as uint32_t
                            && buff[buffpos as usize] as ::core::ffi::c_int
                                >= '0' as ::core::ffi::c_int
                            && buff[buffpos as usize] as ::core::ffi::c_int
                                <= '9' as ::core::ffi::c_int
                        {
                            lastlogversion = lastlogversion.wrapping_mul(10 as uint64_t);
                            lastlogversion = lastlogversion.wrapping_add(
                                (buff[buffpos as usize] as ::core::ffi::c_int
                                    - '0' as ::core::ffi::c_int)
                                    as uint64_t,
                            );
                            buffpos = buffpos.wrapping_add(1);
                        }
                        if buffpos == 32800 as uint32_t
                            || buff[buffpos as usize] as ::core::ffi::c_int
                                != ':' as ::core::ffi::c_int
                        {
                            lastlogversion = 0 as uint64_t;
                        }
                        close(fd);
                        return;
                    }
                }
            }
        }
        close(fd);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_createpacket(
    mut eptr: *mut masterconn,
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
                b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                211 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"outpacket\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                211 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                211 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"outpacket\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                211 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
pub unsafe extern "C" fn masterconn_sendregister(mut eptr: *mut masterconn) {
    unsafe {
        let mut buff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        (*eptr).downloading = 0 as uint8_t;
        (*eptr).metafd = -1 as ::core::ffi::c_int;
        (*eptr).logfd = ::core::ptr::null_mut::<FILE>();
        if lastlogversion > 0 as uint64_t {
            buff = masterconn_createpacket(
                eptr,
                ANTOMA_REGISTER as uint32_t,
                (1 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int) as uint32_t,
            );
            put8bit(&raw mut buff, 2 as uint8_t);
            put16bit(&raw mut buff, VERSMAJ as uint16_t);
            put8bit(&raw mut buff, VERSMID as uint8_t);
            put8bit(&raw mut buff, VERSMIN as uint8_t);
            put16bit(&raw mut buff, (*eptr).timeout);
            put64bit(&raw mut buff, lastlogversion.wrapping_add(1 as uint64_t));
        } else {
            buff = masterconn_createpacket(
                eptr,
                ANTOMA_REGISTER as uint32_t,
                (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                    as uint32_t,
            );
            put8bit(&raw mut buff, 1 as uint8_t);
            put16bit(&raw mut buff, VERSMAJ as uint16_t);
            put8bit(&raw mut buff, VERSMID as uint8_t);
            put8bit(&raw mut buff, VERSMIN as uint8_t);
            put16bit(&raw mut buff, (*eptr).timeout);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_master_ack(
    mut eptr: *mut masterconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut acktype: uint8_t = 0;
        if length != 5 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOAN_MASTER_ACK - wrong size (%u/5)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        acktype = get8bit(&raw mut data);
        if acktype as ::core::ffi::c_int >= 2 as ::core::ffi::c_int {
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_force_timeout(
    mut eptr: *mut masterconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        if length != 2 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOAN_FORCE_TIMEOUT - wrong size (%u/2)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        (*eptr).timeout = get16bit(&raw mut data);
        if ((*eptr).timeout as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
            (*eptr).timeout = 10 as uint16_t;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_metachanges_log(
    mut eptr: *mut masterconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut logname1: [::core::ffi::c_char; 100] = [0; 100];
        let mut logname2: [::core::ffi::c_char; 100] = [0; 100];
        let mut i: uint32_t = 0;
        let mut version: uint64_t = 0;
        if length == 1 as uint32_t
            && *data.offset(0 as isize) as ::core::ffi::c_int == 0x55 as ::core::ffi::c_int
        {
            if !(*eptr).logfd.is_null() {
                fclose((*eptr).logfd);
                (*eptr).logfd = ::core::ptr::null_mut::<FILE>();
            }
            if BackLogsNumber > 0 as uint32_t {
                i = BackLogsNumber;
                while i > 0 as uint32_t {
                    snprintf(
                        &raw mut logname1 as *mut ::core::ffi::c_char,
                        100 as size_t,
                        b"changelog_ml.%u.mfs\0".as_ptr() as *const ::core::ffi::c_char,
                        i,
                    );
                    snprintf(
                        &raw mut logname2 as *mut ::core::ffi::c_char,
                        100 as size_t,
                        b"changelog_ml.%u.mfs\0".as_ptr() as *const ::core::ffi::c_char,
                        i.wrapping_sub(1 as uint32_t),
                    );
                    rename(
                        &raw mut logname2 as *mut ::core::ffi::c_char,
                        &raw mut logname1 as *mut ::core::ffi::c_char,
                    );
                    i = i.wrapping_sub(1);
                }
            } else {
                unlink(b"changelog_ml.0.mfs\0".as_ptr() as *const ::core::ffi::c_char);
            }
            return;
        }
        if length < 10 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOAN_METACHANGES_LOG - wrong size (%u/9+data)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if *data.offset(0 as isize) as ::core::ffi::c_int != 0xff as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOAN_METACHANGES_LOG - wrong packet\0".as_ptr() as *const ::core::ffi::c_char,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if *data.offset(length.wrapping_sub(1 as uint32_t) as isize) as ::core::ffi::c_int
            != '\0' as ::core::ffi::c_int
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOAN_METACHANGES_LOG - invalid string\0".as_ptr() as *const ::core::ffi::c_char,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        data = data.offset(1);
        version = get64bit(&raw mut data);
        if lastlogversion > 0 as uint64_t && version != lastlogversion.wrapping_add(1 as uint64_t) {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"some changes lost: [%lu-%lu], download metadata again\0".as_ptr()
                    as *const ::core::ffi::c_char,
                lastlogversion,
                version.wrapping_sub(1 as uint64_t),
            );
            if !(*eptr).logfd.is_null() {
                fclose((*eptr).logfd);
                (*eptr).logfd = ::core::ptr::null_mut::<FILE>();
            }
            i = 0 as uint32_t;
            while i <= BackLogsNumber {
                snprintf(
                    &raw mut logname1 as *mut ::core::ffi::c_char,
                    100 as size_t,
                    b"changelog_ml.%u.mfs\0".as_ptr() as *const ::core::ffi::c_char,
                    i,
                );
                unlink(&raw mut logname1 as *mut ::core::ffi::c_char);
                i = i.wrapping_add(1);
            }
            lastlogversion = 0 as uint64_t;
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if (*eptr).logfd.is_null() {
            (*eptr).logfd = fopen(
                b"changelog_ml.0.mfs\0".as_ptr() as *const ::core::ffi::c_char,
                b"a\0".as_ptr() as *const ::core::ffi::c_char,
            ) as *mut FILE;
        }
        if !(*eptr).logfd.is_null() {
            fprintf(
                (*eptr).logfd,
                b"%lu: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                version,
                data,
            );
            lastlogversion = version;
        } else {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"lost MFS change %lu: %s\0".as_ptr() as *const ::core::ffi::c_char,
                version,
                data,
            );
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_metachanges_flush() {
    unsafe {
        let mut eptr: *mut masterconn = masterconnsingleton;
        if !(*eptr).logfd.is_null() {
            fflush((*eptr).logfd);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_download_end(mut eptr: *mut masterconn) -> ::core::ffi::c_int {
    unsafe {
        (*eptr).downloading = 0 as uint8_t;
        masterconn_createpacket(eptr, ANTOMA_DOWNLOAD_END as uint32_t, 0 as uint32_t);
        if (*eptr).metafd >= 0 as ::core::ffi::c_int {
            if close((*eptr).metafd) < 0 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"error closing metafile\0".as_ptr() as *const ::core::ffi::c_char,
                );
                (*eptr).metafd = -1 as ::core::ffi::c_int;
                return -1 as ::core::ffi::c_int;
            }
            (*eptr).metafd = -1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_download_init(mut eptr: *mut masterconn, mut filenum: uint8_t) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
            && (*eptr).downloading as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            ptr = masterconn_createpacket(eptr, ANTOMA_DOWNLOAD_START as uint32_t, 1 as uint32_t);
            put8bit(&raw mut ptr, filenum);
            (*eptr).downloading = filenum;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_metadownloadinit() {
    unsafe {
        masterconn_download_init(masterconnsingleton, 1 as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_metadata_check(
    mut name: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut fd: ::core::ffi::c_int = 0;
        let mut chkbuff: [::core::ffi::c_char; 16] = [0; 16];
        let mut eofmark: [::core::ffi::c_char; 16] = [0; 16];
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut metaversion: uint64_t = 0;
        let mut metaid: uint64_t = 0;
        fd = open(name, O_RDONLY);
        if fd < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"can't open downloaded metadata\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        if read(
            fd,
            &raw mut chkbuff as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            8 as size_t,
        ) != 8 as ssize_t
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"can't read downloaded metadata\0".as_ptr() as *const ::core::ffi::c_char,
            );
            close(fd);
            return -1 as ::core::ffi::c_int;
        }
        if memcmp(
            &raw mut chkbuff as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            b"MFSM NEW\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            8 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            close(fd);
            return -1 as ::core::ffi::c_int;
        }
        if memcmp(
            &raw mut chkbuff as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            b"MFSM \0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            5 as size_t,
        ) == 0 as ::core::ffi::c_int
            && chkbuff[5 as usize] as ::core::ffi::c_int >= '1' as ::core::ffi::c_int
            && chkbuff[5 as usize] as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
            && chkbuff[6 as usize] as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            && chkbuff[7 as usize] as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
            && chkbuff[7 as usize] as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
        {
            let mut fver: uint8_t = (((chkbuff[5 as usize] as ::core::ffi::c_int
                - '0' as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int)
                + (chkbuff[7 as usize] as ::core::ffi::c_int - '0' as ::core::ffi::c_int))
                as uint8_t;
            if (fver as ::core::ffi::c_int) < 0x17 as ::core::ffi::c_int {
                memset(
                    &raw mut eofmark as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    16 as size_t,
                );
            } else {
                memcpy(
                    &raw mut eofmark as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    b"[MFS EOF MARKER]\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    16 as size_t,
                );
                if fver as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int {
                    if read(
                        fd,
                        &raw mut chkbuff as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                        16 as size_t,
                    ) != 16 as ssize_t
                    {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"can't read downloaded metadata\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        close(fd);
                        return -1 as ::core::ffi::c_int;
                    }
                    rptr = &raw mut chkbuff as *mut ::core::ffi::c_char as *mut uint8_t;
                    metaversion = get64bit(&raw mut rptr);
                    metaid = get64bit(&raw mut rptr);
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_INFO,
                        b"meta data version: %lu, meta data id: 0x%016lX\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        metaversion,
                        metaid,
                    );
                }
            }
        } else {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"bad metadata file format\0".as_ptr() as *const ::core::ffi::c_char,
            );
            close(fd);
            return -1 as ::core::ffi::c_int;
        }
        lseek(fd, -16 as __off64_t, SEEK_END);
        if read(
            fd,
            &raw mut chkbuff as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            16 as size_t,
        ) != 16 as ssize_t
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"can't read downloaded metadata\0".as_ptr() as *const ::core::ffi::c_char,
            );
            close(fd);
            return -1 as ::core::ffi::c_int;
        }
        close(fd);
        if memcmp(
            &raw mut chkbuff as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            &raw mut eofmark as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            16 as size_t,
        ) != 0 as ::core::ffi::c_int
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"truncated metadata file !!!\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_download_next(mut eptr: *mut masterconn) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut filenum: uint8_t = 0;
        let mut dltime: int64_t = 0;
        if (*eptr).dloffset >= (*eptr).filesize {
            filenum = (*eptr).downloading;
            if masterconn_download_end(eptr) < 0 as ::core::ffi::c_int {
                return;
            }
            dltime = monotonic_useconds().wrapping_sub((*eptr).dlstartuts) as int64_t;
            if dltime <= 0 as int64_t {
                dltime = 1 as int64_t;
            }
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_INFO,
                b"%s downloaded %luB/%lu.%06us (%.3lf MB/s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                if filenum as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    b"metadata\0".as_ptr() as *const ::core::ffi::c_char
                } else if filenum as ::core::ffi::c_int == 11 as ::core::ffi::c_int {
                    b"changelog_0\0".as_ptr() as *const ::core::ffi::c_char
                } else if filenum as ::core::ffi::c_int == 12 as ::core::ffi::c_int {
                    b"changelog_1\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"???\0".as_ptr() as *const ::core::ffi::c_char
                },
                (*eptr).filesize,
                dltime / 1000000 as int64_t,
                (dltime % 1000000 as int64_t) as uint32_t,
                (*eptr).filesize as ::core::ffi::c_double / dltime as ::core::ffi::c_double,
            );
            if filenum as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                if masterconn_metadata_check(b"metadata_ml.tmp\0".as_ptr()
                    as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    if BackMetaCopies > 0 as uint32_t {
                        let mut metaname1: [::core::ffi::c_char; 100] = [0; 100];
                        let mut metaname2: [::core::ffi::c_char; 100] = [0; 100];
                        let mut i: ::core::ffi::c_int = 0;
                        i = BackMetaCopies.wrapping_sub(1 as uint32_t) as ::core::ffi::c_int;
                        while i > 0 as ::core::ffi::c_int {
                            snprintf(
                                &raw mut metaname1 as *mut ::core::ffi::c_char,
                                100 as size_t,
                                b"metadata_ml.mfs.back.%u\0".as_ptr() as *const ::core::ffi::c_char,
                                i + 1 as ::core::ffi::c_int,
                            );
                            snprintf(
                                &raw mut metaname2 as *mut ::core::ffi::c_char,
                                100 as size_t,
                                b"metadata_ml.mfs.back.%u\0".as_ptr() as *const ::core::ffi::c_char,
                                i,
                            );
                            rename(
                                &raw mut metaname2 as *mut ::core::ffi::c_char,
                                &raw mut metaname1 as *mut ::core::ffi::c_char,
                            );
                            i -= 1;
                        }
                        rename(
                            b"metadata_ml.mfs.back\0".as_ptr() as *const ::core::ffi::c_char,
                            b"metadata_ml.mfs.back.1\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                    if rename(
                        b"metadata_ml.tmp\0".as_ptr() as *const ::core::ffi::c_char,
                        b"metadata_ml.mfs.back\0".as_ptr() as *const ::core::ffi::c_char,
                    ) < 0 as ::core::ffi::c_int
                    {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"can't rename downloaded metadata - do it manually before next download\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                }
                if (*eptr).oldmode as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    masterconn_download_init(eptr, 11 as uint8_t);
                }
            } else if filenum as ::core::ffi::c_int == 11 as ::core::ffi::c_int {
                if rename(
                    b"changelog_ml.tmp\0".as_ptr() as *const ::core::ffi::c_char,
                    b"changelog_ml_back.0.mfs\0".as_ptr() as *const ::core::ffi::c_char,
                ) < 0 as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"can't rename downloaded changelog - do it manually before next download\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                masterconn_download_init(eptr, 12 as uint8_t);
            } else if filenum as ::core::ffi::c_int == 12 as ::core::ffi::c_int {
                if rename(
                    b"changelog_ml.tmp\0".as_ptr() as *const ::core::ffi::c_char,
                    b"changelog_ml_back.1.mfs\0".as_ptr() as *const ::core::ffi::c_char,
                ) < 0 as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"can't rename downloaded changelog - do it manually before next download\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            }
        } else {
            ptr =
                masterconn_createpacket(eptr, ANTOMA_DOWNLOAD_REQUEST as uint32_t, 12 as uint32_t);
            put64bit(&raw mut ptr, (*eptr).dloffset);
            if (*eptr).filesize.wrapping_sub((*eptr).dloffset) > META_DL_BLOCK as uint64_t {
                put32bit(&raw mut ptr, META_DL_BLOCK as uint32_t);
            } else {
                put32bit(
                    &raw mut ptr,
                    (*eptr).filesize.wrapping_sub((*eptr).dloffset) as uint32_t,
                );
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_download_info(
    mut eptr: *mut masterconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        if length != 1 as uint32_t && length != 8 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOAN_DOWNLOAD_INFO - wrong size (%u/1|8)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if data.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                498 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                498 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if data
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *const uint8_t
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                498 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                498 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        if length == 1 as uint32_t {
            (*eptr).downloading = 0 as uint8_t;
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"download start error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        (*eptr).filesize = get64bit(&raw mut data);
        (*eptr).dloffset = 0 as uint64_t;
        (*eptr).downloadretrycnt = 0 as uint8_t;
        (*eptr).dlstartuts = monotonic_useconds();
        if (*eptr).downloading as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            (*eptr).metafd = open(
                b"metadata_ml.tmp\0".as_ptr() as *const ::core::ffi::c_char,
                O_WRONLY | O_TRUNC | O_CREAT,
                0o666 as ::core::ffi::c_int,
            );
        } else if (*eptr).downloading as ::core::ffi::c_int == 11 as ::core::ffi::c_int
            || (*eptr).downloading as ::core::ffi::c_int == 12 as ::core::ffi::c_int
        {
            (*eptr).metafd = open(
                b"changelog_ml.tmp\0".as_ptr() as *const ::core::ffi::c_char,
                O_WRONLY | O_TRUNC | O_CREAT,
                0o666 as ::core::ffi::c_int,
            );
        } else {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"unexpected MATOAN_DOWNLOAD_INFO packet\0".as_ptr() as *const ::core::ffi::c_char,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if (*eptr).metafd < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"error opening metafile\0".as_ptr() as *const ::core::ffi::c_char,
            );
            masterconn_download_end(eptr);
            return;
        }
        masterconn_download_next(eptr);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_download_data(
    mut eptr: *mut masterconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut offset: uint64_t = 0;
        let mut leng: uint32_t = 0;
        let mut crc: uint32_t = 0;
        let mut ret: ssize_t = 0;
        if (*eptr).metafd < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOAN_DOWNLOAD_DATA - file not opened\0".as_ptr() as *const ::core::ffi::c_char,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length < 16 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOAN_DOWNLOAD_DATA - wrong size (%u/16+data)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if data.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                540 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                540 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if data
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *const uint8_t
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                540 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                540 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        offset = get64bit(&raw mut data);
        leng = get32bit(&raw mut data);
        crc = get32bit(&raw mut data);
        if leng.wrapping_add(16 as uint32_t) != length {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOAN_DOWNLOAD_DATA - wrong size (%u/16+%u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                leng,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if offset != (*eptr).dloffset {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOAN_DOWNLOAD_DATA - unexpected file offset (%lu/%lu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                offset,
                (*eptr).dloffset,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if offset.wrapping_add(leng as uint64_t) > (*eptr).filesize {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOAN_DOWNLOAD_DATA - unexpected file size (%lu/%lu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                offset.wrapping_add(leng as uint64_t),
                (*eptr).filesize,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        ret = pwrite(
            (*eptr).metafd,
            data as *const ::core::ffi::c_void,
            leng as size_t,
            offset as __off64_t,
        );
        if ret != leng as ssize_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"error writing metafile\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if (*eptr).downloadretrycnt as ::core::ffi::c_int >= 5 as ::core::ffi::c_int {
                masterconn_download_end(eptr);
            } else {
                (*eptr).downloadretrycnt = (*eptr).downloadretrycnt.wrapping_add(1);
                masterconn_download_next(eptr);
            }
            return;
        }
        if crc != mycrc32(0 as uint32_t, data as *const ::core::ffi::c_void, leng) {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"metafile data crc error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if (*eptr).downloadretrycnt as ::core::ffi::c_int >= 5 as ::core::ffi::c_int {
                masterconn_download_end(eptr);
            } else {
                (*eptr).downloadretrycnt = (*eptr).downloadretrycnt.wrapping_add(1);
                masterconn_download_next(eptr);
            }
            return;
        }
        if fsync((*eptr).metafd) < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"error syncing metafile\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if (*eptr).downloadretrycnt as ::core::ffi::c_int >= 5 as ::core::ffi::c_int {
                masterconn_download_end(eptr);
            } else {
                (*eptr).downloadretrycnt = (*eptr).downloadretrycnt.wrapping_add(1);
                masterconn_download_next(eptr);
            }
            return;
        }
        (*eptr).dloffset = (*eptr).dloffset.wrapping_add(leng as uint64_t);
        (*eptr).downloadretrycnt = 0 as uint8_t;
        masterconn_download_next(eptr);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_beforeclose(mut eptr: *mut masterconn) {
    unsafe {
        if (*eptr).downloading as ::core::ffi::c_int == 11 as ::core::ffi::c_int
            || (*eptr).downloading as ::core::ffi::c_int == 12 as ::core::ffi::c_int
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"old master detected - please upgrade your master server and then restart metalogger\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            (*eptr).oldmode = 1 as uint8_t;
        }
        if (*eptr).metafd >= 0 as ::core::ffi::c_int {
            close((*eptr).metafd);
            (*eptr).metafd = -1 as ::core::ffi::c_int;
            unlink(b"metadata_ml.tmp\0".as_ptr() as *const ::core::ffi::c_char);
            unlink(b"changelog_ml.tmp\0".as_ptr() as *const ::core::ffi::c_char);
        }
        if !(*eptr).logfd.is_null() {
            fclose((*eptr).logfd);
            (*eptr).logfd = ::core::ptr::null_mut::<FILE>();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_gotpacket(
    mut eptr: *mut masterconn,
    mut r#type: uint32_t,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        match r#type {
            0 | 1 | 2 => {}
            51 => {
                masterconn_metachanges_log(eptr, data, length);
            }
            5 => {
                masterconn_force_timeout(eptr, data, length);
            }
            52 => {
                masterconn_master_ack(eptr, data, length);
            }
            61 => {
                masterconn_download_info(eptr, data, length);
            }
            63 => {
                masterconn_download_data(eptr, data, length);
            }
            _ => {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"got unknown message (type:%u)\0".as_ptr() as *const ::core::ffi::c_char,
                    r#type,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_connected(mut eptr: *mut masterconn) {
    unsafe {
        let mut now: ::core::ffi::c_double = 0.;
        now = monotonic_seconds();
        tcpnodelay((*eptr).sock);
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
        (*eptr).timeout = Timeout as uint16_t;
        masterconn_sendregister(eptr);
        if lastlogversion == 0 as uint64_t {
            masterconn_metadownloadinit();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_initconnect(mut eptr: *mut masterconn) -> ::core::ffi::c_int {
    unsafe {
        let mut status: ::core::ffi::c_int = 0;
        if (*eptr).masteraddrvalid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut mip: uint32_t = 0;
            let mut bip: uint32_t = 0;
            let mut mport: uint16_t = 0;
            if tcpresolve(
                BindHost,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut bip,
                ::core::ptr::null_mut::<uint16_t>(),
                1 as ::core::ffi::c_int,
            ) < 0 as ::core::ffi::c_int
            {
                bip = 0 as uint32_t;
            }
            (*eptr).bindip = bip;
            if tcpresolve(
                MasterHost,
                MasterPort,
                &raw mut mip,
                &raw mut mport,
                0 as ::core::ffi::c_int,
            ) >= 0 as ::core::ffi::c_int
            {
                (*eptr).masterip = mip;
                (*eptr).masterport = mport;
                (*eptr).masteraddrvalid = 1 as uint8_t;
            } else {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"can't resolve master host/port (%s:%s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    MasterHost,
                    MasterPort,
                );
                return -1 as ::core::ffi::c_int;
            }
        }
        (*eptr).sock = tcpsocket();
        if (*eptr).sock < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"create socket, error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        if tcpnonblock((*eptr).sock) < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"set nonblock, error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            tcpclose((*eptr).sock);
            (*eptr).sock = -1 as ::core::ffi::c_int;
            return -1 as ::core::ffi::c_int;
        }
        if (*eptr).bindip > 0 as uint32_t {
            if tcpnumbind((*eptr).sock, (*eptr).bindip, 0 as uint16_t) < 0 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"can't bind socket to given ip\0".as_ptr() as *const ::core::ffi::c_char,
                );
                tcpclose((*eptr).sock);
                (*eptr).sock = -1 as ::core::ffi::c_int;
                return -1 as ::core::ffi::c_int;
            }
        }
        status = tcpnumconnect((*eptr).sock, (*eptr).masterip, (*eptr).masterport);
        if status < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"connect failed, error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            tcpclose((*eptr).sock);
            (*eptr).sock = -1 as ::core::ffi::c_int;
            (*eptr).masteraddrvalid = 0 as uint8_t;
            return -1 as ::core::ffi::c_int;
        }
        if status == 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_INFO,
                b"connected to Master immediately\0".as_ptr() as *const ::core::ffi::c_char,
            );
            masterconn_connected(eptr);
        } else {
            (*eptr).mode = CONNECTING as ::core::ffi::c_int as uint8_t;
            (*eptr).conntime = monotonic_seconds();
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_INFO,
                b"connecting ...\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_connecttimeout(mut eptr: *mut masterconn) {
    unsafe {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"connection timed out\0".as_ptr() as *const ::core::ffi::c_char,
        );
        tcpclose((*eptr).sock);
        (*eptr).sock = -1 as ::core::ffi::c_int;
        (*eptr).mode = FREE as ::core::ffi::c_int as uint8_t;
        (*eptr).masteraddrvalid = 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_connecttest(mut eptr: *mut masterconn) {
    unsafe {
        let mut status: ::core::ffi::c_int = 0;
        status = tcpgetstatus((*eptr).sock);
        if status != 0 {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"connection failed, error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            tcpclose((*eptr).sock);
            (*eptr).sock = -1 as ::core::ffi::c_int;
            (*eptr).mode = FREE as ::core::ffi::c_int as uint8_t;
            (*eptr).masteraddrvalid = 0 as uint8_t;
        } else {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_INFO,
                b"connected to Master\0".as_ptr() as *const ::core::ffi::c_char,
            );
            masterconn_connected(eptr);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_read(
    mut eptr: *mut masterconn,
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
                    b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    771 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    771 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                    b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    771 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    771 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                stats_bytesin = stats_bytesin.wrapping_add(i as uint32_t);
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
                        b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        793 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        793 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                        b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        793 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        793 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                        b"Master packet too long (%u/%u) ; command:%u\0".as_ptr()
                            as *const ::core::ffi::c_char,
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
                        b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        833 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr->input_packet\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        833 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                        b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        833 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr->input_packet\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        833 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_NOTICE,
                b"connection was reset by Master\0".as_ptr() as *const ::core::ffi::c_char,
            );
            (*eptr).input_end = 1 as uint8_t;
        } else if err != 0 {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"read from Master error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            (*eptr).input_end = 1 as uint8_t;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_parse(mut eptr: *mut masterconn) {
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
            masterconn_gotpacket(
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
pub unsafe extern "C" fn masterconn_write(
    mut eptr: *mut masterconn,
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
                        b"write to Master error\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                }
                return;
            }
            if i > 0 as int32_t {
                (*eptr).lastwrite = now;
            }
            stats_bytesout = stats_bytesout.wrapping_add(i as uint32_t);
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
pub unsafe extern "C" fn masterconn_desc(mut pdesc: *mut pollfd, mut ndesc: *mut uint32_t) {
    unsafe {
        let mut pos: uint32_t = *ndesc;
        let mut eptr: *mut masterconn = masterconnsingleton;
        (*eptr).pdescpos = -1 as ::core::ffi::c_int as int32_t;
        if (*eptr).mode as ::core::ffi::c_int == FREE as ::core::ffi::c_int
            || (*eptr).sock < 0 as ::core::ffi::c_int
        {
            return;
        }
        (*pdesc.offset(pos as isize)).events = 0 as ::core::ffi::c_short;
        if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
            && (*eptr).input_end as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            (*pdesc.offset(pos as isize)).events =
                ((*pdesc.offset(pos as isize)).events as ::core::ffi::c_int | POLLIN)
                    as ::core::ffi::c_short;
        }
        if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
            && !(*eptr).outputhead.is_null()
            || (*eptr).mode as ::core::ffi::c_int == CONNECTING as ::core::ffi::c_int
        {
            (*pdesc.offset(pos as isize)).events =
                ((*pdesc.offset(pos as isize)).events as ::core::ffi::c_int | POLLOUT)
                    as ::core::ffi::c_short;
        }
        if (*pdesc.offset(pos as isize)).events as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            (*pdesc.offset(pos as isize)).fd = (*eptr).sock;
            (*eptr).pdescpos = pos as int32_t;
            pos = pos.wrapping_add(1);
        }
        *ndesc = pos;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_disconnection_check() {
    unsafe {
        let mut eptr: *mut masterconn = masterconnsingleton;
        let mut ipptr: *mut in_packetstruct = ::core::ptr::null_mut::<in_packetstruct>();
        let mut ipaptr: *mut in_packetstruct = ::core::ptr::null_mut::<in_packetstruct>();
        let mut opptr: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
        let mut opaptr: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
        if (*eptr).mode as ::core::ffi::c_int == KILL as ::core::ffi::c_int {
            masterconn_beforeclose(eptr);
            tcpclose((*eptr).sock);
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
            (*eptr).mode = FREE as ::core::ffi::c_int as uint8_t;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_serve(mut pdesc: *mut pollfd) {
    unsafe {
        let mut now: ::core::ffi::c_double = 0.;
        let mut eptr: *mut masterconn = masterconnsingleton;
        now = monotonic_seconds();
        if (*eptr).mode as ::core::ffi::c_int == CONNECTING as ::core::ffi::c_int {
            if (*eptr).sock >= 0 as ::core::ffi::c_int
                && (*eptr).pdescpos >= 0 as int32_t
                && (*pdesc.offset((*eptr).pdescpos as isize)).revents as ::core::ffi::c_int
                    & (POLLOUT | POLLHUP | POLLERR)
                    != 0
            {
                masterconn_connecttest(eptr);
            } else if (*eptr).conntime + 1.0f64 < now {
                masterconn_connecttimeout(eptr);
            }
        } else {
            if (*eptr).pdescpos >= 0 as int32_t {
                if (*pdesc.offset((*eptr).pdescpos as isize)).revents as ::core::ffi::c_int
                    & (POLLERR | POLLIN)
                    == POLLIN
                    && (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
                {
                    masterconn_read(eptr, now);
                }
                if (*pdesc.offset((*eptr).pdescpos as isize)).revents as ::core::ffi::c_int
                    & (POLLERR | POLLHUP)
                    != 0
                {
                    (*eptr).input_end = 1 as uint8_t;
                }
                masterconn_parse(eptr);
            }
            if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
                && (*eptr).lastwrite + 1.0f64 < now
                && (*eptr).outputhead.is_null()
            {
                masterconn_createpacket(eptr, ANTOAN_NOP as uint32_t, 0 as uint32_t);
            }
            if (*eptr).pdescpos >= 0 as int32_t {
                if ((*pdesc.offset((*eptr).pdescpos as isize)).events as ::core::ffi::c_int
                    & POLLOUT
                    == 0 as ::core::ffi::c_int
                    && !(*eptr).outputhead.is_null()
                    || (*pdesc.offset((*eptr).pdescpos as isize)).revents as ::core::ffi::c_int
                        & POLLOUT
                        != 0)
                    && (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
                {
                    masterconn_write(eptr, now);
                }
            }
            if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
                && ((*eptr).lastread
                    + (*eptr).timeout as ::core::ffi::c_int as ::core::ffi::c_double)
                    < now
            {
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            }
        }
        masterconn_disconnection_check();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_reconnect() {
    unsafe {
        let mut eptr: *mut masterconn = masterconnsingleton;
        if (*eptr).mode as ::core::ffi::c_int == FREE as ::core::ffi::c_int {
            masterconn_initconnect(eptr);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_socketmode(mut mode: uint8_t) -> *const ::core::ffi::c_char {
    match mode as ::core::ffi::c_int {
        0 => return b"NOT CONNECTED\0".as_ptr() as *const ::core::ffi::c_char,
        1 => {
            return b"CONNECTING IN PROGRESS\0".as_ptr() as *const ::core::ffi::c_char;
        }
        2 => return b"CONNECTED\0".as_ptr() as *const ::core::ffi::c_char,
        3 => return b"DISCONNECTING\0".as_ptr() as *const ::core::ffi::c_char,
        _ => {}
    }
    return b"???\0".as_ptr() as *const ::core::ffi::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_info(mut fd: *mut FILE) {
    unsafe {
        let mut eptr: *mut masterconn = masterconnsingleton;
        let mut stripport: [::core::ffi::c_char; 32] = [0; 32];
        let mut strip: [::core::ffi::c_char; 16] = [0; 16];
        let mut dltime: uint64_t = 0;
        fprintf(
            fd,
            b"[master connection]\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"master address is valid: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
            (*eptr).masteraddrvalid as ::core::ffi::c_int,
        );
        fprintf(
            fd,
            b"working timeout: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
            (*eptr).timeout as ::core::ffi::c_int,
        );
        univmakestrip(&raw mut strip as *mut ::core::ffi::c_char, (*eptr).bindip);
        fprintf(
            fd,
            b"socket bind ip: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut strip as *mut ::core::ffi::c_char,
        );
        univmakestripport(
            &raw mut stripport as *mut ::core::ffi::c_char,
            (*eptr).masterip,
            (*eptr).masterport,
        );
        fprintf(
            fd,
            b"resolved ip:port number: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut stripport as *mut ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"socket mode: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            masterconn_socketmode((*eptr).mode),
        );
        if (*eptr).downloading as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            fprintf(
                fd,
                b"downloading metadata in progress\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if (*eptr).downloading as ::core::ffi::c_int == 11 as ::core::ffi::c_int
            || (*eptr).downloading as ::core::ffi::c_int == 12 as ::core::ffi::c_int
        {
            fprintf(
                fd,
                b"downloading last changelogs in progress\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if (*eptr).downloading as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            dltime = monotonic_useconds().wrapping_sub((*eptr).dlstartuts);
            fprintf(
                fd,
                b"downloading progress: %lu/%lu\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*eptr).dloffset,
                (*eptr).filesize,
            );
            fprintf(
                fd,
                b"downloading try counter: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*eptr).downloadretrycnt as ::core::ffi::c_int,
            );
            fprintf(
                fd,
                b"downloading time: %lu.%06us\0".as_ptr() as *const ::core::ffi::c_char,
                dltime.wrapping_div(1000000 as uint64_t),
                dltime.wrapping_rem(1000000 as uint64_t) as uint32_t,
            );
        }
        fprintf(fd, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_term() {
    unsafe {
        let mut eptr: *mut masterconn = masterconnsingleton;
        let mut ipptr: *mut in_packetstruct = ::core::ptr::null_mut::<in_packetstruct>();
        let mut ipaptr: *mut in_packetstruct = ::core::ptr::null_mut::<in_packetstruct>();
        let mut opptr: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
        let mut opaptr: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
        if (*eptr).mode as ::core::ffi::c_int != FREE as ::core::ffi::c_int {
            tcpclose((*eptr).sock);
            if (*eptr).mode as ::core::ffi::c_int != CONNECTING as ::core::ffi::c_int {
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
            }
        }
        masterconn_read(::core::ptr::null_mut::<masterconn>(), 0.0f64);
        free(eptr as *mut ::core::ffi::c_void);
        free(MasterHost as *mut ::core::ffi::c_void);
        free(MasterPort as *mut ::core::ffi::c_void);
        free(BindHost as *mut ::core::ffi::c_void);
        masterconnsingleton = ::core::ptr::null_mut::<masterconn>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_reload() {
    unsafe {
        let mut eptr: *mut masterconn = masterconnsingleton;
        let mut ReconnectionDelay: uint32_t = 0;
        let mut MetaDLFreq: uint32_t = 0;
        free(MasterHost as *mut ::core::ffi::c_void);
        free(MasterPort as *mut ::core::ffi::c_void);
        free(BindHost as *mut ::core::ffi::c_void);
        MasterHost = cfg_getstr(
            b"MASTER_HOST\0".as_ptr() as *const ::core::ffi::c_char,
            DEFAULT_MASTERNAME.as_ptr(),
        );
        MasterPort = cfg_getstr(
            b"MASTER_PORT\0".as_ptr() as *const ::core::ffi::c_char,
            DEFAULT_MASTER_CONTROL_PORT.as_ptr(),
        );
        BindHost = cfg_getstr(
            b"BIND_HOST\0".as_ptr() as *const ::core::ffi::c_char,
            b"*\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*eptr).masteraddrvalid = 0 as uint8_t;
        if (*eptr).mode as ::core::ffi::c_int != FREE as ::core::ffi::c_int {
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
        }
        Timeout = cfg_getuint32(
            b"MASTER_TIMEOUT\0".as_ptr() as *const ::core::ffi::c_char,
            10 as uint32_t,
        );
        BackLogsNumber = cfg_getuint32(
            b"BACK_LOGS\0".as_ptr() as *const ::core::ffi::c_char,
            50 as uint32_t,
        );
        BackMetaCopies = cfg_getuint32(
            b"BACK_META_KEEP_PREVIOUS\0".as_ptr() as *const ::core::ffi::c_char,
            3 as uint32_t,
        );
        ReconnectionDelay = cfg_getuint32(
            b"MASTER_RECONNECTION_DELAY\0".as_ptr() as *const ::core::ffi::c_char,
            5 as uint32_t,
        );
        MetaDLFreq = cfg_getuint32(
            b"META_DOWNLOAD_FREQ\0".as_ptr() as *const ::core::ffi::c_char,
            24 as uint32_t,
        );
        if Timeout > 65535 as uint32_t {
            Timeout = 65535 as uint32_t;
        }
        if Timeout < 10 as uint32_t {
            Timeout = 10 as uint32_t;
        }
        if BackLogsNumber < 5 as uint32_t {
            BackLogsNumber = 5 as uint32_t;
        }
        if BackLogsNumber > 10000 as uint32_t {
            BackLogsNumber = 10000 as uint32_t;
        }
        if MetaDLFreq > BackLogsNumber.wrapping_div(2 as uint32_t) {
            MetaDLFreq = BackLogsNumber.wrapping_div(2 as uint32_t);
        }
        if BackMetaCopies > 99 as uint32_t {
            BackMetaCopies = 99 as uint32_t;
        }
        main_time_change(reconnect_hook, ReconnectionDelay, 0 as uint32_t);
        main_time_change(
            download_hook,
            MetaDLFreq.wrapping_mul(3600 as uint32_t),
            630 as uint32_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_init() -> ::core::ffi::c_int {
    unsafe {
        let mut ReconnectionDelay: uint32_t = 0;
        let mut MetaDLFreq: uint32_t = 0;
        let mut eptr: *mut masterconn = ::core::ptr::null_mut::<masterconn>();
        ReconnectionDelay = cfg_getuint32(
            b"MASTER_RECONNECTION_DELAY\0".as_ptr() as *const ::core::ffi::c_char,
            5 as uint32_t,
        );
        MasterHost = cfg_getstr(
            b"MASTER_HOST\0".as_ptr() as *const ::core::ffi::c_char,
            DEFAULT_MASTERNAME.as_ptr(),
        );
        MasterPort = cfg_getstr(
            b"MASTER_PORT\0".as_ptr() as *const ::core::ffi::c_char,
            DEFAULT_MASTER_CONTROL_PORT.as_ptr(),
        );
        BindHost = cfg_getstr(
            b"BIND_HOST\0".as_ptr() as *const ::core::ffi::c_char,
            b"*\0".as_ptr() as *const ::core::ffi::c_char,
        );
        Timeout = cfg_getuint32(
            b"MASTER_TIMEOUT\0".as_ptr() as *const ::core::ffi::c_char,
            10 as uint32_t,
        );
        BackLogsNumber = cfg_getuint32(
            b"BACK_LOGS\0".as_ptr() as *const ::core::ffi::c_char,
            50 as uint32_t,
        );
        BackMetaCopies = cfg_getuint32(
            b"BACK_META_KEEP_PREVIOUS\0".as_ptr() as *const ::core::ffi::c_char,
            3 as uint32_t,
        );
        MetaDLFreq = cfg_getuint32(
            b"META_DOWNLOAD_FREQ\0".as_ptr() as *const ::core::ffi::c_char,
            24 as uint32_t,
        );
        if Timeout > 65535 as uint32_t {
            Timeout = 65535 as uint32_t;
        }
        if Timeout < 10 as uint32_t {
            Timeout = 10 as uint32_t;
        }
        if BackLogsNumber < 5 as uint32_t {
            BackLogsNumber = 5 as uint32_t;
        }
        if BackLogsNumber > 10000 as uint32_t {
            BackLogsNumber = 10000 as uint32_t;
        }
        if MetaDLFreq > BackLogsNumber.wrapping_div(2 as uint32_t) {
            MetaDLFreq = BackLogsNumber.wrapping_div(2 as uint32_t);
        }
        masterconnsingleton = malloc(::core::mem::size_of::<masterconn>()) as *mut masterconn;
        eptr = masterconnsingleton;
        if eptr.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1223 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1223 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if eptr
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut masterconn
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1223 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmetalogger/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1223 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*eptr).masteraddrvalid = 0 as uint8_t;
        (*eptr).mode = FREE as ::core::ffi::c_int as uint8_t;
        (*eptr).pdescpos = -1 as ::core::ffi::c_int as int32_t;
        (*eptr).logfd = ::core::ptr::null_mut::<FILE>();
        (*eptr).metafd = -1 as ::core::ffi::c_int;
        (*eptr).oldmode = 0 as uint8_t;
        (*eptr).timeout = Timeout as uint16_t;
        masterconn_findlastlogversion();
        if masterconn_initconnect(eptr) < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        reconnect_hook = main_time_register_fname(
            ReconnectionDelay,
            0 as uint32_t,
            Some(masterconn_reconnect as unsafe extern "C" fn() -> ()),
            b"masterconn_reconnect\0".as_ptr() as *const ::core::ffi::c_char,
        );
        download_hook = main_time_register_fname(
            MetaDLFreq.wrapping_mul(3600 as uint32_t),
            630 as uint32_t,
            Some(masterconn_metadownloadinit as unsafe extern "C" fn() -> ()),
            b"masterconn_metadownloadinit\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_destruct_register_fname(
            Some(masterconn_term as unsafe extern "C" fn() -> ()),
            b"masterconn_term\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_poll_register_fname(
            Some(masterconn_desc as unsafe extern "C" fn(*mut pollfd, *mut uint32_t) -> ()),
            Some(masterconn_serve as unsafe extern "C" fn(*mut pollfd) -> ()),
            b"masterconn_desc\0".as_ptr() as *const ::core::ffi::c_char,
            b"masterconn_serve\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_reload_register_fname(
            Some(masterconn_reload as unsafe extern "C" fn() -> ()),
            b"masterconn_reload\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_time_register_fname(
            1 as uint32_t,
            0 as uint32_t,
            Some(masterconn_metachanges_flush as unsafe extern "C" fn() -> ()),
            b"masterconn_metachanges_flush\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_info_register_fname(
            Some(masterconn_info as unsafe extern "C" fn(*mut FILE) -> ()),
            b"masterconn_info\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
}
