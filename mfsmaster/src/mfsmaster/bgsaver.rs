use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn pwrite(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __nbytes: size_t,
        __offset: __off64_t,
    ) -> ssize_t;
    fn pipe(__pipedes: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn usleep(__useconds: __useconds_t) -> ::core::ffi::c_int;
    fn dup(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn fork() -> __pid_t;
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn fsync(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn lockf(
        __fd: ::core::ffi::c_int,
        __cmd: ::core::ffi::c_int,
        __len: __off64_t,
    ) -> ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn poll(
        __fds: *mut pollfd,
        __nfds: nfds_t,
        __timeout: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn writev(
        __fd: ::core::ffi::c_int,
        __iovec: *const iovec,
        __count: ::core::ffi::c_int,
    ) -> ssize_t;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn mycrc32(crc: uint32_t, block: *const ::core::ffi::c_void, leng: uint32_t) -> uint32_t;
    static mut stderr: *mut FILE;
    fn rename(
        __old: *const ::core::ffi::c_char,
        __new: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
    fn main_destruct_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    fn main_wantexit_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    fn main_canexit_register_fname(
        fun: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        fname: *const ::core::ffi::c_char,
    );
    fn main_reload_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    fn main_poll_register_fname(
        desc: Option<unsafe extern "C" fn(*mut pollfd, *mut uint32_t) -> ()>,
        serve: Option<unsafe extern "C" fn(*mut pollfd) -> ()>,
        dname: *const ::core::ffi::c_char,
        sname: *const ::core::ffi::c_char,
    );
    fn main_time_register_fname(
        seconds: uint32_t,
        offset: uint32_t,
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    fn main_exit();
    fn main_time() -> uint32_t;
    fn cfg_getuint32(name: *const ::core::ffi::c_char, def: uint32_t) -> uint32_t;
    fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn univnonblock(fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn processname_set(name: *mut ::core::ffi::c_char);
    fn monotonic_seconds() -> ::core::ffi::c_double;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __useconds_t = ::core::ffi::c_uint;
pub type ssize_t = isize;
pub type int32_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut ::core::ffi::c_void,
    pub iov_len: size_t,
}
pub type nfds_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
}
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
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
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const BGSAVER_TERMINATE: C2Rust_Unnamed = 9;
pub const BGSAVER_ROTATELOG: C2Rust_Unnamed = 8;
pub const BGSAVER_CHANGELOG_NACK: C2Rust_Unnamed = 7;
pub const BGSAVER_CHANGELOG_ACK: C2Rust_Unnamed = 6;
pub const BGSAVER_CHANGELOG: C2Rust_Unnamed = 5;
pub const BGSAVER_DONE: C2Rust_Unnamed = 4;
pub const BGSAVER_FINISH: C2Rust_Unnamed = 3;
pub const BGSAVER_WRITE: C2Rust_Unnamed = 2;
pub const BGSAVER_START: C2Rust_Unnamed = 1;
pub const BGSAVER_ALIVE: C2Rust_Unnamed = 0;
pub type C2Rust_Unnamed_0 = ::core::ffi::c_uint;
pub const KILL: C2Rust_Unnamed_0 = 2;
pub const DATA: C2Rust_Unnamed_0 = 1;
pub const FREE: C2Rust_Unnamed_0 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bgsaverconn {
    pub data_pipe: [::core::ffi::c_int; 2],
    pub status_pipe: [::core::ffi::c_int; 2],
    pub mode: uint8_t,
    pub pdescpos_r: int32_t,
    pub pdescpos_w: int32_t,
    pub input_hdr: [uint8_t; 8],
    pub input_startptr: *mut uint8_t,
    pub input_bytesleft: uint32_t,
    pub input_end: uint8_t,
    pub input_packet: *mut in_packetstruct,
    pub inputhead: *mut in_packetstruct,
    pub inputtail: *mut *mut in_packetstruct,
    pub outputhead: *mut out_packetstruct,
    pub outputtail: *mut *mut out_packetstruct,
    pub ud: *mut ::core::ffi::c_void,
    pub donefn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int) -> ()>,
}
pub const STDIN_FILENO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STDERR_FILENO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const F_ULOCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const F_TLOCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const F_TEST: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_TRUNC: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const O_APPEND: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const POLLOUT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const POLLERR: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const POLLHUP: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
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
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_ERRNO_SYSLOG_STDERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn mfsrealloc(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    let mut pptr: *mut ::core::ffi::c_void = realloc(ptr, size);
    if pptr.is_null() {
        free(ptr);
    }
    return pptr;
}
pub const MAXLOGNUMBER: ::core::ffi::c_uint = 1000 as ::core::ffi::c_uint;
pub const PIPE_READ: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PIPE_WRITE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MAX_STATUS_SIZE: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
static mut bgsaversingleton: *mut bgsaverconn = ::core::ptr::null_mut::<bgsaverconn>();
static mut BackLogsNumber: uint32_t = 0;
static mut changelog_delay: uint32_t = 0 as uint32_t;
static mut bgsaver_last_activity: ::core::ffi::c_double = 0.;
static mut bgsaver_last_check: ::core::ffi::c_double = 0.;
static mut bgsaver_last_check_count: uint32_t = 0;
static mut bgsaver_last_report: uint32_t = 0;
static mut terminating: uint8_t = 0;
static mut termdelay: uint8_t = 0;
#[no_mangle]
pub unsafe extern "C" fn writeall(
    mut sock: ::core::ffi::c_int,
    mut buff: *mut uint8_t,
    mut leng: uint32_t,
) -> int32_t {
    let mut pfd: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    let mut bsent: uint32_t = 0;
    let mut res: ::core::ffi::c_int = 0;
    bsent = 0 as uint32_t;
    while bsent < leng {
        res = write(
            sock,
            buff.offset(bsent as isize) as *const ::core::ffi::c_void,
            leng.wrapping_sub(bsent) as size_t,
        ) as ::core::ffi::c_int;
        if res <= 0 as ::core::ffi::c_int {
            if *__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK {
                return -1 as int32_t;
            } else {
                pfd.fd = sock;
                pfd.revents = 0 as ::core::ffi::c_short;
                pfd.events = POLLOUT as ::core::ffi::c_short;
                if poll(&raw mut pfd, 1 as nfds_t, 100 as ::core::ffi::c_int)
                    < 0 as ::core::ffi::c_int
                {
                    if *__errno_location() != EINTR {
                        return -1 as int32_t;
                    }
                }
            }
            res = 0 as ::core::ffi::c_int;
        }
        bsent = bsent.wrapping_add(res as uint32_t);
    }
    return bsent as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn readall(
    mut sock: ::core::ffi::c_int,
    mut buf: *mut uint8_t,
    mut leng: uint32_t,
) -> int32_t {
    let mut pfd: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    let mut brecv: uint32_t = 0;
    let mut res: ::core::ffi::c_int = 0;
    brecv = 0 as uint32_t;
    while brecv < leng {
        res = read(
            sock,
            buf.offset(brecv as isize) as *mut ::core::ffi::c_void,
            leng.wrapping_sub(brecv) as size_t,
        ) as ::core::ffi::c_int;
        if res <= 0 as ::core::ffi::c_int {
            if *__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK {
                return -1 as int32_t;
            } else {
                pfd.fd = sock;
                pfd.revents = 0 as ::core::ffi::c_short;
                pfd.events = POLLIN as ::core::ffi::c_short;
                if poll(&raw mut pfd, 1 as nfds_t, 100 as ::core::ffi::c_int)
                    < 0 as ::core::ffi::c_int
                {
                    if *__errno_location() != EINTR {
                        return -1 as int32_t;
                    }
                }
            }
            res = 0 as ::core::ffi::c_int;
        }
        brecv = brecv.wrapping_add(res as uint32_t);
    }
    return brecv as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn bgsaver_worker() {
    let mut eptr: *mut bgsaverconn = bgsaversingleton;
    let mut pfd: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    let mut auxbuff: [uint8_t; 12] = [0; 12];
    let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut l: int32_t = 0;
    let mut cmd: uint32_t = 0;
    let mut leng: uint32_t = 0;
    let mut buff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut buffsize: uint32_t = 0;
    let mut woffset: uint64_t = 0;
    let mut wleng: uint32_t = 0;
    let mut wcrc: uint32_t = 0;
    let mut speedlimit: uint32_t = 0;
    let mut bytes: uint64_t = 0;
    let mut starttime: ::core::ffi::c_double = 0.;
    let mut last_alive_send: ::core::ffi::c_double = 0.;
    let mut status: uint8_t = 0;
    let mut ret: ssize_t = 0;
    let mut fd: ::core::ffi::c_int = 0;
    let mut logfd: ::core::ffi::c_int = 0;
    let mut lf: ::core::ffi::c_int = 0;
    let mut chlogbuff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut chlogbuffsize: uint32_t = 0;
    let mut chloglostcnt: uint32_t = 0;
    let mut timestamp: uint32_t = 0;
    static mut last_timestamp: uint32_t = 0;
    buff = ::core::ptr::null_mut::<uint8_t>();
    buffsize = 0 as uint32_t;
    fd = -1 as ::core::ffi::c_int;
    bytes = 0 as uint64_t;
    starttime = 0.0f64;
    speedlimit = 0 as uint32_t;
    logfd = -1 as ::core::ffi::c_int;
    chlogbuff = ::core::ptr::null_mut::<::core::ffi::c_char>();
    chlogbuffsize = 0 as uint32_t;
    chloglostcnt = 0 as uint32_t;
    pfd.fd = (*eptr).data_pipe[PIPE_READ as usize];
    last_alive_send = monotonic_seconds();
    timestamp = 0 as uint32_t;
    last_timestamp = 0 as uint32_t;
    lf = open(
        b".bgwriter.lock\0".as_ptr() as *const ::core::ffi::c_char,
        O_RDWR,
        0o666 as ::core::ffi::c_int,
    );
    '_err: {
        if lf < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"background data writer - can't open lockfile\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if lockf(lf, F_TLOCK, 0 as __off64_t) < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"background data writer - can't get lock on lockfile\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else {
            loop {
                if last_alive_send + 1.0f64 < monotonic_seconds() {
                    wptr = &raw mut auxbuff as *mut uint8_t;
                    put32bit(
                        &raw mut wptr,
                        BGSAVER_ALIVE as ::core::ffi::c_int as uint32_t,
                    );
                    put32bit(&raw mut wptr, 0 as uint32_t);
                    writeall(
                        (*eptr).status_pipe[PIPE_WRITE as usize],
                        &raw mut auxbuff as *mut uint8_t,
                        8 as uint32_t,
                    );
                    last_alive_send = monotonic_seconds();
                }
                pfd.revents = 0 as ::core::ffi::c_short;
                pfd.events = POLLIN as ::core::ffi::c_short;
                poll(&raw mut pfd, 1 as nfds_t, 100 as ::core::ffi::c_int);
                if pfd.revents as ::core::ffi::c_int & (POLLERR | POLLHUP)
                    != 0 as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"background data writer - HUP/ERR detected on data pipe\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    break '_err;
                } else {
                    if pfd.revents as ::core::ffi::c_int & POLLIN == 0 as ::core::ffi::c_int {
                        continue;
                    }
                    l = readall(
                        (*eptr).data_pipe[PIPE_READ as usize],
                        &raw mut auxbuff as *mut uint8_t,
                        8 as uint32_t,
                    );
                    if l != 8 as int32_t {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_ERR,
                            b"background data writer - reading pipe error\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        break '_err;
                    } else {
                        rptr = &raw mut auxbuff as *mut uint8_t;
                        cmd = get32bit(&raw mut rptr);
                        leng = get32bit(&raw mut rptr);
                        if leng <= 10000000 as uint32_t {
                            if leng > buffsize {
                                let mut newleng: uint32_t =
                                    leng.wrapping_mul(3 as uint32_t).wrapping_div(2 as uint32_t);
                                if !buff.is_null() {
                                    free(buff as *mut ::core::ffi::c_void);
                                }
                                buff = malloc(newleng as size_t) as *mut uint8_t;
                                if !buff.is_null() {
                                    buffsize = leng;
                                } else {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_ERR,
                                        b"background data writer - out of memory (alloc size: %u)\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        newleng,
                                    );
                                    break '_err;
                                }
                            }
                            l = readall((*eptr).data_pipe[PIPE_READ as usize], buff, leng);
                            if l != leng as int32_t {
                                mfs_log(
                                    MFSLOG_SYSLOG_STDERR,
                                    MFSLOG_ERR,
                                    b"background data writer - reading pipe error\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                                break '_err;
                            } else {
                                match cmd {
                                    1 => {
                                        if leng != 4 as uint32_t {
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_WARNING,
                                                b"background data writer - leng error (BGSAVER_START packet)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                            );
                                            status = 0 as uint8_t;
                                        } else {
                                            rptr = buff;
                                            speedlimit = get32bit(&raw mut rptr);
                                            if fd >= 0 as ::core::ffi::c_int {
                                                close(fd);
                                            }
                                            fd = open(
                                                b"metadata_download.tmp\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                O_WRONLY | O_TRUNC | O_CREAT,
                                                0o666 as ::core::ffi::c_int,
                                            );
                                            if fd < 0 as ::core::ffi::c_int {
                                                mfs_log(
                                                    MFSLOG_SYSLOG_STDERR,
                                                    MFSLOG_WARNING,
                                                    b"background data writer - error opening 'metadata_download.tmp'\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                );
                                                status = 0 as uint8_t;
                                            } else {
                                                bytes = 0 as uint64_t;
                                                starttime = monotonic_seconds();
                                                status = 1 as uint8_t;
                                            }
                                        }
                                    }
                                    2 => {
                                        if leng < 16 as uint32_t {
                                            status = 0 as uint8_t;
                                        } else {
                                            rptr = buff;
                                            woffset = get64bit(&raw mut rptr);
                                            wleng = get32bit(&raw mut rptr);
                                            wcrc = get32bit(&raw mut rptr);
                                            if wleng != leng.wrapping_sub(16 as uint32_t) {
                                                mfs_log(
                                                    MFSLOG_SYSLOG,
                                                    MFSLOG_WARNING,
                                                    b"background data writer - leng error (BGSAVER_WRITE packet)\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                );
                                                status = 0 as uint8_t;
                                            } else {
                                                ret = pwrite(
                                                    fd,
                                                    rptr as *const ::core::ffi::c_void,
                                                    wleng as size_t,
                                                    woffset as __off64_t,
                                                );
                                                if ret != wleng as ssize_t {
                                                    mfs_log(
                                                        MFSLOG_SYSLOG_STDERR,
                                                        MFSLOG_WARNING,
                                                        b"background data writer - error writing 'metadata_download.tmp'\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                    );
                                                    status = 0 as uint8_t;
                                                } else if wcrc
                                                    != mycrc32(
                                                        0 as uint32_t,
                                                        rptr as *const ::core::ffi::c_void,
                                                        wleng,
                                                    )
                                                {
                                                    mfs_log(
                                                        MFSLOG_SYSLOG,
                                                        MFSLOG_WARNING,
                                                        b"background data writer - crc error (BGSAVER_WRITE packet)\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                    );
                                                    status = 0 as uint8_t;
                                                } else {
                                                    if speedlimit > 0 as uint32_t {
                                                        let mut seconds_passed: ::core::ffi::c_double = monotonic_seconds()
                                                            - starttime;
                                                        let mut expected_seconds: ::core::ffi::c_double = 0.;
                                                        bytes =
                                                            bytes.wrapping_add(wleng as uint64_t);
                                                        expected_seconds = bytes
                                                            as ::core::ffi::c_double
                                                            / speedlimit as ::core::ffi::c_double;
                                                        if expected_seconds > seconds_passed {
                                                            usleep(
                                                                ((expected_seconds
                                                                    - seconds_passed)
                                                                    * 1000000 as ::core::ffi::c_int
                                                                        as ::core::ffi::c_double)
                                                                    as __useconds_t,
                                                            );
                                                        }
                                                    }
                                                    status = 1 as uint8_t;
                                                }
                                            }
                                        }
                                        if status as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                                            close(fd);
                                            fd = -1 as ::core::ffi::c_int;
                                        }
                                    }
                                    3 => {
                                        if leng != 0 as uint32_t {
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_WARNING,
                                                b"background data writer - leng error (BGSAVER_FINISH packet)\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                            );
                                            status = 0 as uint8_t;
                                        } else {
                                            status = 1 as uint8_t;
                                            if fsync(fd) < 0 as ::core::ffi::c_int {
                                                mfs_log(
                                                    MFSLOG_SYSLOG_STDERR,
                                                    MFSLOG_WARNING,
                                                    b"background data writer - error syncing 'metadata_download.tmp'\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                );
                                                status = 0 as uint8_t;
                                            }
                                            if close(fd) < 0 as ::core::ffi::c_int {
                                                mfs_log(
                                                    MFSLOG_SYSLOG_STDERR,
                                                    MFSLOG_WARNING,
                                                    b"background data writer - error closing 'metadata_download.tmp'\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                );
                                                status = 0 as uint8_t;
                                            }
                                            fd = -1 as ::core::ffi::c_int;
                                        }
                                    }
                                    5 => {
                                        status = 0 as uint8_t;
                                        if leng >= 12 as uint32_t {
                                            let mut version: uint64_t = 0;
                                            rptr = buff;
                                            version = get64bit(&raw mut rptr);
                                            timestamp = get32bit(&raw mut rptr);
                                            leng = leng.wrapping_sub(12 as uint32_t);
                                            if logfd < 0 as ::core::ffi::c_int {
                                                logfd = open(
                                                    b"changelog.0.mfs\0".as_ptr()
                                                        as *const ::core::ffi::c_char,
                                                    O_WRONLY | O_CREAT | O_APPEND,
                                                    0o666 as ::core::ffi::c_int,
                                                );
                                            }
                                            if logfd >= 0 as ::core::ffi::c_int {
                                                if leng.wrapping_add(50 as uint32_t) > chlogbuffsize
                                                {
                                                    if chlogbuff.is_null() {
                                                        chlogbuff = malloc(
                                                            leng.wrapping_add(500 as uint32_t)
                                                                as size_t,
                                                        )
                                                            as *mut ::core::ffi::c_char;
                                                        if chlogbuff.is_null() {
                                                            chlogbuffsize = 0 as uint32_t;
                                                        } else {
                                                            chlogbuffsize =
                                                                leng.wrapping_add(500 as uint32_t);
                                                        }
                                                    } else {
                                                        let mut prevptr: *mut ::core::ffi::c_char =
                                                            chlogbuff;
                                                        chlogbuff = realloc(
                                                            chlogbuff as *mut ::core::ffi::c_void,
                                                            leng.wrapping_add(500 as uint32_t)
                                                                as size_t,
                                                        )
                                                            as *mut ::core::ffi::c_char;
                                                        if chlogbuff.is_null() {
                                                            free(
                                                                prevptr as *mut ::core::ffi::c_void,
                                                            );
                                                            chlogbuffsize = 0 as uint32_t;
                                                        } else {
                                                            chlogbuffsize =
                                                                leng.wrapping_add(500 as uint32_t);
                                                        }
                                                    }
                                                }
                                                if !chlogbuff.is_null() {
                                                    wleng = snprintf(
                                                        chlogbuff,
                                                        chlogbuffsize as size_t,
                                                        b"%lu: %s\n\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        version,
                                                        rptr,
                                                    )
                                                        as uint32_t;
                                                    if write(
                                                        logfd,
                                                        chlogbuff as *const ::core::ffi::c_void,
                                                        wleng as size_t,
                                                    ) == wleng as ssize_t
                                                    {
                                                        status = 1 as uint8_t;
                                                    } else {
                                                        mfs_log(
                                                            MFSLOG_SYSLOG_STDERR,
                                                            MFSLOG_WARNING,
                                                            b"background data writer - error writing 'changelog.0.mfs'\0"
                                                                .as_ptr() as *const ::core::ffi::c_char,
                                                        );
                                                    }
                                                }
                                            } else {
                                                mfs_log(
                                                    MFSLOG_SYSLOG_STDERR,
                                                    MFSLOG_WARNING,
                                                    b"background data writer - error opening 'changelog.0.mfs'\0"
                                                        .as_ptr() as *const ::core::ffi::c_char,
                                                );
                                            }
                                        }
                                    }
                                    8 => {
                                        if leng != 4 as uint32_t {
                                            status = 0 as uint8_t;
                                        } else {
                                            let mut logname1: [::core::ffi::c_char; 100] = [0; 100];
                                            let mut logname2: [::core::ffi::c_char; 100] = [0; 100];
                                            let mut i: uint32_t = 0;
                                            let mut backlogsno: uint32_t = 0;
                                            rptr = buff;
                                            backlogsno = get32bit(&raw mut rptr);
                                            status = 1 as uint8_t;
                                            if logfd >= 0 as ::core::ffi::c_int {
                                                if fsync(logfd) < 0 as ::core::ffi::c_int {
                                                    mfs_log(
                                                        MFSLOG_SYSLOG_STDERR,
                                                        MFSLOG_WARNING,
                                                        b"background data writer - error syncing 'changelog.0.mfs'\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                    );
                                                    status = 0 as uint8_t;
                                                }
                                                if close(logfd) < 0 as ::core::ffi::c_int {
                                                    mfs_log(
                                                        MFSLOG_SYSLOG_STDERR,
                                                        MFSLOG_WARNING,
                                                        b"background data writer - error closing 'changelog.0.mfs'\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                    );
                                                    status = 0 as uint8_t;
                                                }
                                                logfd = -1 as ::core::ffi::c_int;
                                            }
                                            if backlogsno > 0 as uint32_t {
                                                i = backlogsno;
                                                while i > 0 as uint32_t {
                                                    snprintf(
                                                        &raw mut logname1
                                                            as *mut ::core::ffi::c_char,
                                                        100 as size_t,
                                                        b"changelog.%u.mfs\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        i,
                                                    );
                                                    snprintf(
                                                        &raw mut logname2
                                                            as *mut ::core::ffi::c_char,
                                                        100 as size_t,
                                                        b"changelog.%u.mfs\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                        i.wrapping_sub(1 as uint32_t),
                                                    );
                                                    if rename(
                                                        &raw mut logname2
                                                            as *mut ::core::ffi::c_char,
                                                        &raw mut logname1
                                                            as *mut ::core::ffi::c_char,
                                                    ) < 0 as ::core::ffi::c_int
                                                    {
                                                        if *__errno_location() != ENOENT {
                                                            mfs_log(
                                                                MFSLOG_SYSLOG_STDERR,
                                                                MFSLOG_WARNING,
                                                                b"background data writer - error renaming '%s'->'%s'\0"
                                                                    .as_ptr() as *const ::core::ffi::c_char,
                                                                &raw mut logname2 as *mut ::core::ffi::c_char,
                                                                &raw mut logname1 as *mut ::core::ffi::c_char,
                                                            );
                                                        }
                                                    }
                                                    i = i.wrapping_sub(1);
                                                }
                                            } else if unlink(b"changelog.0.mfs\0".as_ptr()
                                                as *const ::core::ffi::c_char)
                                                < 0 as ::core::ffi::c_int
                                            {
                                                if *__errno_location() != ENOENT {
                                                    mfs_log(
                                                        MFSLOG_SYSLOG_STDERR,
                                                        MFSLOG_WARNING,
                                                        b"background data writer - error deleting 'changelog.0.mfs'\0"
                                                            .as_ptr() as *const ::core::ffi::c_char,
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    9 => {
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_INFO,
                                            b"background data writer - terminating\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                        );
                                        if logfd >= 0 as ::core::ffi::c_int {
                                            fsync(logfd);
                                            close(logfd);
                                        }
                                        if fd >= 0 as ::core::ffi::c_int {
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_NOTICE,
                                                b"background data writer - removing unfinished metadata file\0"
                                                    .as_ptr() as *const ::core::ffi::c_char,
                                            );
                                            close(fd);
                                            unlink(b"metadata_download.tmp\0".as_ptr()
                                                as *const ::core::ffi::c_char);
                                        }
                                        break '_err;
                                    }
                                    _ => {
                                        mfs_log(
                                            MFSLOG_SYSLOG,
                                            MFSLOG_ERR,
                                            b"background data writer - got unrecognized command (%u)\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            cmd,
                                        );
                                        break '_err;
                                    }
                                }
                                if cmd == BGSAVER_START as ::core::ffi::c_int as uint32_t
                                    || cmd == BGSAVER_WRITE as ::core::ffi::c_int as uint32_t
                                    || cmd == BGSAVER_FINISH as ::core::ffi::c_int as uint32_t
                                {
                                    if status as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                                        unlink(b"metadata_download.tmp\0".as_ptr()
                                            as *const ::core::ffi::c_char);
                                    }
                                    wptr = &raw mut auxbuff as *mut uint8_t;
                                    put32bit(
                                        &raw mut wptr,
                                        BGSAVER_DONE as ::core::ffi::c_int as uint32_t,
                                    );
                                    put32bit(&raw mut wptr, 1 as uint32_t);
                                    *wptr = status;
                                    writeall(
                                        (*eptr).status_pipe[PIPE_WRITE as usize],
                                        &raw mut auxbuff as *mut uint8_t,
                                        9 as uint32_t,
                                    );
                                } else if cmd == BGSAVER_CHANGELOG as ::core::ffi::c_int as uint32_t
                                    || cmd == BGSAVER_ROTATELOG as ::core::ffi::c_int as uint32_t
                                {
                                    if status != 0 {
                                        if cmd
                                            == BGSAVER_CHANGELOG as ::core::ffi::c_int as uint32_t
                                            && timestamp >= last_timestamp
                                        {
                                            last_timestamp = timestamp;
                                            wptr = &raw mut auxbuff as *mut uint8_t;
                                            put32bit(
                                                &raw mut wptr,
                                                BGSAVER_CHANGELOG_ACK as ::core::ffi::c_int
                                                    as uint32_t,
                                            );
                                            put32bit(&raw mut wptr, 4 as uint32_t);
                                            put32bit(&raw mut wptr, timestamp);
                                            writeall(
                                                (*eptr).status_pipe[PIPE_WRITE as usize],
                                                &raw mut auxbuff as *mut uint8_t,
                                                12 as uint32_t,
                                            );
                                        }
                                        chloglostcnt = 0 as uint32_t;
                                    } else {
                                        if chloglostcnt == 0 as uint32_t {
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_WARNING,
                                                b"changelog lost !!!\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                            );
                                        } else if chloglostcnt == 100000 as uint32_t {
                                            mfs_log(
                                                MFSLOG_SYSLOG,
                                                MFSLOG_WARNING,
                                                b"next 100000 changelogs are lost !!!\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                            );
                                            chloglostcnt = 0 as uint32_t;
                                        }
                                        chloglostcnt = chloglostcnt.wrapping_add(1);
                                        wptr = &raw mut auxbuff as *mut uint8_t;
                                        put32bit(
                                            &raw mut wptr,
                                            BGSAVER_CHANGELOG_NACK as ::core::ffi::c_int
                                                as uint32_t,
                                        );
                                        put32bit(&raw mut wptr, 0 as uint32_t);
                                        writeall(
                                            (*eptr).status_pipe[PIPE_WRITE as usize],
                                            &raw mut auxbuff as *mut uint8_t,
                                            8 as uint32_t,
                                        );
                                    }
                                }
                            }
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"background data writer - packet too long (packet size: %u)\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                leng,
                            );
                            break '_err;
                        }
                    }
                }
            }
        }
    }
    if !buff.is_null() {
        free(buff as *mut ::core::ffi::c_void);
    }
    if !chlogbuff.is_null() {
        free(chlogbuff as *mut ::core::ffi::c_void);
    }
    if lf >= 0 as ::core::ffi::c_int {
        if lockf(lf, F_ULOCK, 0 as __off64_t) < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"background data writer - error removing lock from lockfile\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        close(lf);
    }
    mfs_log(
        MFSLOG_SYSLOG,
        MFSLOG_INFO,
        b"background data writer - exiting\0".as_ptr() as *const ::core::ffi::c_char,
    );
    exit(0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn bgsaver_createpacket(
    mut eptr: *mut bgsaverconn,
    mut r#type: uint32_t,
    mut size: uint32_t,
) -> *mut uint8_t {
    let mut outpacket: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut psize: uint32_t = 0;
    psize = size.wrapping_add(8 as uint32_t);
    outpacket = malloc((20 as size_t).wrapping_add(psize as size_t)) as *mut out_packetstruct;
    if outpacket.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr() as *const ::core::ffi::c_char,
            513 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"outpacket\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr() as *const ::core::ffi::c_char,
            513 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"outpacket\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if outpacket
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut out_packetstruct
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr() as *const ::core::ffi::c_char,
            513 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"outpacket\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr() as *const ::core::ffi::c_char,
            513 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
#[no_mangle]
pub unsafe extern "C" fn bgsaver_cancel() {
    let mut eptr: *mut bgsaverconn = bgsaversingleton;
    if eptr.is_null() || (*eptr).mode as ::core::ffi::c_int != DATA as ::core::ffi::c_int {
        return;
    }
    bgsaver_createpacket(
        eptr,
        BGSAVER_FINISH as ::core::ffi::c_int as uint32_t,
        0 as uint32_t,
    );
    (*eptr).ud = NULL;
    (*eptr).donefn = None;
}
#[no_mangle]
pub unsafe extern "C" fn bgsaver_open(
    mut speedlimit: uint32_t,
    mut ud: *mut ::core::ffi::c_void,
    mut donefn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int) -> ()>,
) {
    let mut eptr: *mut bgsaverconn = bgsaversingleton;
    let mut buff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    if eptr.is_null() || (*eptr).mode as ::core::ffi::c_int != DATA as ::core::ffi::c_int {
        donefn.expect("non-null function pointer")(ud, -1 as ::core::ffi::c_int);
        return;
    }
    buff = bgsaver_createpacket(
        eptr,
        BGSAVER_START as ::core::ffi::c_int as uint32_t,
        4 as uint32_t,
    );
    put32bit(&raw mut buff, speedlimit);
    (*eptr).ud = ud;
    (*eptr).donefn = donefn;
}
#[no_mangle]
pub unsafe extern "C" fn bgsaver_store(
    mut data: *const uint8_t,
    mut offset: uint64_t,
    mut leng: uint32_t,
    mut crc: uint32_t,
    mut ud: *mut ::core::ffi::c_void,
    mut donefn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int) -> ()>,
) {
    let mut eptr: *mut bgsaverconn = bgsaversingleton;
    let mut buff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    if eptr.is_null() || (*eptr).mode as ::core::ffi::c_int != DATA as ::core::ffi::c_int {
        donefn.expect("non-null function pointer")(ud, -1 as ::core::ffi::c_int);
        return;
    }
    buff = bgsaver_createpacket(
        eptr,
        BGSAVER_WRITE as ::core::ffi::c_int as uint32_t,
        (16 as uint32_t).wrapping_add(leng),
    );
    put64bit(&raw mut buff, offset);
    put32bit(&raw mut buff, leng);
    put32bit(&raw mut buff, crc);
    memcpy(
        buff as *mut ::core::ffi::c_void,
        data as *const ::core::ffi::c_void,
        leng as size_t,
    );
    (*eptr).ud = ud;
    (*eptr).donefn = donefn;
}
#[no_mangle]
pub unsafe extern "C" fn bgsaver_close(
    mut ud: *mut ::core::ffi::c_void,
    mut donefn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int) -> ()>,
) {
    let mut eptr: *mut bgsaverconn = bgsaversingleton;
    if eptr.is_null() || (*eptr).mode as ::core::ffi::c_int != DATA as ::core::ffi::c_int {
        donefn.expect("non-null function pointer")(ud, -1 as ::core::ffi::c_int);
        return;
    }
    bgsaver_createpacket(
        eptr,
        BGSAVER_FINISH as ::core::ffi::c_int as uint32_t,
        0 as uint32_t,
    );
    (*eptr).ud = ud;
    (*eptr).donefn = donefn;
}
#[no_mangle]
pub unsafe extern "C" fn bgsaver_done(
    mut eptr: *mut bgsaverconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    let mut ud: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut donefn: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_int) -> (),
    > = None;
    ud = (*eptr).ud;
    donefn = (*eptr).donefn;
    (*eptr).donefn = None;
    (*eptr).ud = NULL;
    if length != 1 as uint32_t {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"mallformed packet from bgworker\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
        if donefn.is_some() {
            donefn.expect("non-null function pointer")(ud, -1 as ::core::ffi::c_int);
        }
        return;
    }
    if donefn.is_some() {
        donefn.expect("non-null function pointer")(ud, *data as ::core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn bgsaver_changelog(
    mut version: uint64_t,
    mut message: *const ::core::ffi::c_char,
) {
    let mut eptr: *mut bgsaverconn = bgsaversingleton;
    let mut l: uint32_t = 0;
    let mut buff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    if terminating != 0 {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"changelog received during termination - changelog line lost\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if eptr.is_null() || (*eptr).mode as ::core::ffi::c_int != DATA as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"problems with data write subprocess detected - changelog line lost - force termination\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        main_exit();
        return;
    }
    l = strlen(message) as uint32_t;
    buff = bgsaver_createpacket(
        eptr,
        BGSAVER_CHANGELOG as ::core::ffi::c_int as uint32_t,
        ((8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t)
            .wrapping_add(l)
            .wrapping_add(1 as uint32_t),
    );
    put64bit(&raw mut buff, version);
    put32bit(&raw mut buff, main_time());
    memcpy(
        buff as *mut ::core::ffi::c_void,
        message as *const ::core::ffi::c_void,
        l.wrapping_add(1 as uint32_t) as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bgsaver_rotatelog() -> ::core::ffi::c_int {
    let mut eptr: *mut bgsaverconn = bgsaversingleton;
    let mut buff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    if eptr.is_null() || (*eptr).mode as ::core::ffi::c_int != DATA as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    buff = bgsaver_createpacket(
        eptr,
        BGSAVER_ROTATELOG as ::core::ffi::c_int as uint32_t,
        4 as uint32_t,
    );
    put32bit(&raw mut buff, BackLogsNumber);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bgsaver_changelog_ack(
    mut eptr: *mut bgsaverconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut timestamp: uint32_t = 0;
    let mut timestamp_ack: uint32_t = 0;
    if length != 4 as uint32_t {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"mallformed packet from bgworker\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
        return;
    }
    rptr = data;
    timestamp_ack = get32bit(&raw mut rptr);
    timestamp = main_time();
    if timestamp > timestamp_ack {
        changelog_delay = timestamp
            .wrapping_sub(timestamp_ack)
            .wrapping_sub(1 as uint32_t);
    } else {
        changelog_delay = 0 as uint32_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn bgsaver_changelog_nack(
    mut eptr: *mut bgsaverconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    if length != 0 as uint32_t {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"mallformed packet from bgworker\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
        return;
    }
    main_exit();
}
#[no_mangle]
pub unsafe extern "C" fn bgsaver_alive(
    mut eptr: *mut bgsaverconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    if length != 0 as uint32_t {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"mallformed packet from bgworker\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
        return;
    }
    bgsaver_last_activity = monotonic_seconds();
}
#[no_mangle]
pub unsafe extern "C" fn bgsaver_gotpacket(
    mut eptr: *mut bgsaverconn,
    mut r#type: uint32_t,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    match r#type {
        4 => {
            bgsaver_done(eptr, data, length);
        }
        6 => {
            bgsaver_changelog_ack(eptr, data, length);
        }
        7 => {
            bgsaver_changelog_nack(eptr, data, length);
        }
        0 => {
            bgsaver_alive(eptr, data, length);
        }
        _ => {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"got unknown message (type:%u)\0".as_ptr() as *const ::core::ffi::c_char,
                r#type,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn bgsaver_read(mut eptr: *mut bgsaverconn) {
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
                b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr() as *const ::core::ffi::c_char,
                723 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr() as *const ::core::ffi::c_char,
                723 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if readbuff
            == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut uint8_t
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr() as *const ::core::ffi::c_char,
                723 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr() as *const ::core::ffi::c_char,
                723 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
            (*eptr).status_pipe[PIPE_READ as usize],
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
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    744 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    744 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if readbuff
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
                    b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    744 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    744 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
        }
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
            if leng > MAX_STATUS_SIZE as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"bgworker packet too long (%u/%u) ; command:%u\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    leng,
                    MAX_STATUS_SIZE,
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
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    780 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"eptr->input_packet\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    780 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"eptr->input_packet\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if (*eptr).input_packet
                == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut in_packetstruct
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    780 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"eptr->input_packet\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    780 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
            (*eptr).inputtail = &raw mut (*(*eptr).input_packet).next as *mut *mut in_packetstruct;
            (*eptr).input_packet = ::core::ptr::null_mut::<in_packetstruct>();
            (*eptr).input_bytesleft = 8 as uint32_t;
            (*eptr).input_startptr = &raw mut (*eptr).input_hdr as *mut uint8_t;
        }
    }
    if hup != 0 {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"connection was reset by bgworker\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*eptr).input_end = 1 as uint8_t;
    } else if err != 0 {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"read from bgworker error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*eptr).input_end = 1 as uint8_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bgsaver_parse(mut eptr: *mut bgsaverconn) {
    let mut ipack: *mut in_packetstruct = ::core::ptr::null_mut::<in_packetstruct>();
    while (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int && {
        ipack = (*eptr).inputhead;
        !ipack.is_null()
    } {
        bgsaver_gotpacket(
            eptr,
            (*ipack).r#type,
            &raw mut (*ipack).data as *mut uint8_t,
            (*ipack).leng,
        );
        (*eptr).inputhead = (*ipack).next as *mut in_packetstruct;
        free(ipack as *mut ::core::ffi::c_void);
        if (*eptr).inputhead.is_null() {
            (*eptr).inputtail = &raw mut (*eptr).inputhead;
        }
    }
    if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
        && (*eptr).inputhead.is_null()
        && (*eptr).input_end as ::core::ffi::c_int != 0
    {
        (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bgsaver_write(mut eptr: *mut bgsaverconn) {
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
            (*eptr).data_pipe[PIPE_WRITE as usize],
            &raw mut iovtab as *mut iovec,
            iovdata as ::core::ffi::c_int,
        ) as int32_t;
        if i < 0 as int32_t {
            if *__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_ERR,
                    b"write to Master error\0".as_ptr() as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            }
            return;
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
#[no_mangle]
pub unsafe extern "C" fn bgsaver_desc(mut pdesc: *mut pollfd, mut ndesc: *mut uint32_t) {
    let mut pos: uint32_t = *ndesc;
    let mut eptr: *mut bgsaverconn = bgsaversingleton;
    (*eptr).pdescpos_r = -1 as ::core::ffi::c_int as int32_t;
    (*eptr).pdescpos_w = -1 as ::core::ffi::c_int as int32_t;
    if (*eptr).mode as ::core::ffi::c_int == FREE as ::core::ffi::c_int
        || (*eptr).data_pipe[1 as usize] < 0 as ::core::ffi::c_int
        || (*eptr).status_pipe[0 as usize] < 0 as ::core::ffi::c_int
    {
        return;
    }
    if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
        && (*eptr).input_end as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        (*pdesc.offset(pos as isize)).fd = (*eptr).status_pipe[0 as usize];
        (*pdesc.offset(pos as isize)).events = POLLIN as ::core::ffi::c_short;
        (*eptr).pdescpos_r = pos as int32_t;
        pos = pos.wrapping_add(1);
    }
    if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
        && !(*eptr).outputhead.is_null()
    {
        (*pdesc.offset(pos as isize)).fd = (*eptr).data_pipe[1 as usize];
        (*pdesc.offset(pos as isize)).events = POLLOUT as ::core::ffi::c_short;
        (*eptr).pdescpos_w = pos as int32_t;
        pos = pos.wrapping_add(1);
    }
    *ndesc = pos;
}
#[no_mangle]
pub unsafe extern "C" fn bgsaver_disconnection_check() {
    let mut eptr: *mut bgsaverconn = bgsaversingleton;
    let mut ipptr: *mut in_packetstruct = ::core::ptr::null_mut::<in_packetstruct>();
    let mut ipaptr: *mut in_packetstruct = ::core::ptr::null_mut::<in_packetstruct>();
    let mut opptr: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
    let mut opaptr: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
    if (*eptr).mode as ::core::ffi::c_int == KILL as ::core::ffi::c_int {
        close((*eptr).data_pipe[PIPE_WRITE as usize]);
        close((*eptr).status_pipe[PIPE_READ as usize]);
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
        (*eptr).data_pipe[PIPE_WRITE as usize] = -1 as ::core::ffi::c_int;
        (*eptr).status_pipe[PIPE_READ as usize] = -1 as ::core::ffi::c_int;
        (*eptr).input_packet = ::core::ptr::null_mut::<in_packetstruct>();
        (*eptr).inputhead = ::core::ptr::null_mut::<in_packetstruct>();
        (*eptr).inputtail = &raw mut (*eptr).inputhead;
        (*eptr).outputhead = ::core::ptr::null_mut::<out_packetstruct>();
        (*eptr).outputtail = &raw mut (*eptr).outputhead;
        (*eptr).mode = FREE as ::core::ffi::c_int as uint8_t;
        if terminating as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"connection with background data writer has been terminated - exiting\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            main_exit();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn bgsaver_serve(mut pdesc: *mut pollfd) {
    let mut eptr: *mut bgsaverconn = bgsaversingleton;
    if (*eptr).pdescpos_r >= 0 as int32_t {
        if (*pdesc.offset((*eptr).pdescpos_r as isize)).revents as ::core::ffi::c_int
            & (POLLERR | POLLIN)
            == POLLIN
            && (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
        {
            bgsaver_read(eptr);
        }
        if (*pdesc.offset((*eptr).pdescpos_r as isize)).revents as ::core::ffi::c_int
            & (POLLERR | POLLHUP)
            != 0
        {
            (*eptr).input_end = 1 as uint8_t;
        }
        bgsaver_parse(eptr);
    }
    if (*eptr).pdescpos_w >= 0 as int32_t {
        if ((*pdesc.offset((*eptr).pdescpos_w as isize)).events as ::core::ffi::c_int & POLLOUT
            == 0 as ::core::ffi::c_int
            && !(*eptr).outputhead.is_null()
            || (*pdesc.offset((*eptr).pdescpos_w as isize)).revents as ::core::ffi::c_int & POLLOUT
                != 0)
            && (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
        {
            bgsaver_write(eptr);
        }
    }
    bgsaver_disconnection_check();
}
#[no_mangle]
pub unsafe extern "C" fn bgsaver_alive_check() {
    let mut now: ::core::ffi::c_double = monotonic_seconds();
    if bgsaver_last_check + 5.0f64 < now {
        bgsaver_last_check_count = 0 as uint32_t;
    } else if bgsaver_last_check_count < 5 as uint32_t {
        bgsaver_last_check_count = bgsaver_last_check_count.wrapping_add(1);
    }
    bgsaver_last_check = now;
    if bgsaver_last_check_count >= 5 as uint32_t {
        if now - bgsaver_last_activity
            > bgsaver_last_report.wrapping_add(50 as uint32_t) as ::core::ffi::c_double
        {
            bgsaver_last_report = bgsaver_last_report.wrapping_add(50 as uint32_t);
            if bgsaver_last_report < 300 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"background data writer is not responding (last ping received more than %u seconds ago)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    bgsaver_last_report,
                );
            } else {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"background data writer is not responding (last ping received more than %u seconds ago) - terminating\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    bgsaver_last_report,
                );
                main_exit();
            }
        } else if now - bgsaver_last_activity < 5.0f64 {
            bgsaver_last_report = 0 as uint32_t;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn bgsaver_reload() {
    BackLogsNumber = cfg_getuint32(
        b"BACK_LOGS\0".as_ptr() as *const ::core::ffi::c_char,
        50 as uint32_t,
    );
    if BackLogsNumber > MAXLOGNUMBER as uint32_t {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"BACK_LOGS value too big !!!\0".as_ptr() as *const ::core::ffi::c_char,
        );
        BackLogsNumber = MAXLOGNUMBER as uint32_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bgsaver_termcheck() {
    let mut eptr: *mut bgsaverconn = bgsaversingleton;
    termdelay = termdelay.wrapping_add(1);
    if termdelay as ::core::ffi::c_int > 2 as ::core::ffi::c_int
        && terminating as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        terminating = 1 as uint8_t;
        if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int {
            bgsaver_createpacket(
                eptr,
                BGSAVER_TERMINATE as ::core::ffi::c_int as uint32_t,
                0 as uint32_t,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn bgsaver_wantexit() {
    main_time_register_fname(
        1 as uint32_t,
        0 as uint32_t,
        Some(bgsaver_termcheck as unsafe extern "C" fn() -> ()),
        b"bgsaver_termcheck\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bgsaver_canexit() -> ::core::ffi::c_int {
    let mut eptr: *mut bgsaverconn = bgsaversingleton;
    return if (*eptr).mode as ::core::ffi::c_int == FREE as ::core::ffi::c_int
        || bgsaver_last_report > 0 as uint32_t
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn bgsaver_term() {
    let mut eptr: *mut bgsaverconn = bgsaversingleton;
    let mut ipptr: *mut in_packetstruct = ::core::ptr::null_mut::<in_packetstruct>();
    let mut ipaptr: *mut in_packetstruct = ::core::ptr::null_mut::<in_packetstruct>();
    let mut opptr: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
    let mut opaptr: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
    if (*eptr).mode as ::core::ffi::c_int != FREE as ::core::ffi::c_int {
        close((*eptr).data_pipe[PIPE_WRITE as usize]);
        close((*eptr).status_pipe[PIPE_READ as usize]);
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
    bgsaver_read(::core::ptr::null_mut::<bgsaverconn>());
    free(eptr as *mut ::core::ffi::c_void);
    bgsaversingleton = ::core::ptr::null_mut::<bgsaverconn>();
}
#[no_mangle]
pub unsafe extern "C" fn bgsaver_init() -> ::core::ffi::c_int {
    let mut lf: ::core::ffi::c_int = 0;
    let mut e: ::core::ffi::c_int = 0;
    let mut eptr: *mut bgsaverconn = ::core::ptr::null_mut::<bgsaverconn>();
    lf = open(
        b".bgwriter.lock\0".as_ptr() as *const ::core::ffi::c_char,
        O_RDWR | O_CREAT,
        0o666 as ::core::ffi::c_int,
    );
    if lf < 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_ERRNO_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"can't create bgsaver lockfile\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if lockf(lf, F_TEST, 0 as __off64_t) < 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_ERRNO_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"bgsaver lock exists\0".as_ptr() as *const ::core::ffi::c_char,
        );
        close(lf);
        return -1 as ::core::ffi::c_int;
    }
    close(lf);
    bgsaver_reload();
    bgsaversingleton = malloc(::core::mem::size_of::<bgsaverconn>()) as *mut bgsaverconn;
    eptr = bgsaversingleton;
    if eptr.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr() as *const ::core::ffi::c_char,
            1094 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr() as *const ::core::ffi::c_char,
            1094 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if eptr
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut bgsaverconn
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr() as *const ::core::ffi::c_char,
            1094 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr() as *const ::core::ffi::c_char,
            1094 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    if pipe(&raw mut (*eptr).data_pipe as *mut ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_ERRNO_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"can't create pipe\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if pipe(&raw mut (*eptr).status_pipe as *mut ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_ERRNO_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"can't create pipe\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    univnonblock((*eptr).data_pipe[PIPE_READ as usize]);
    univnonblock((*eptr).data_pipe[PIPE_WRITE as usize]);
    univnonblock((*eptr).status_pipe[PIPE_READ as usize]);
    univnonblock((*eptr).status_pipe[PIPE_WRITE as usize]);
    e = fork() as ::core::ffi::c_int;
    if e < 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_ERRNO_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"fork error\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if e == 0 as ::core::ffi::c_int {
        let mut f: ::core::ffi::c_int = 0;
        f = open(
            b"/dev/null\0".as_ptr() as *const ::core::ffi::c_char,
            O_RDWR,
            0 as ::core::ffi::c_int,
        );
        close(STDIN_FILENO);
        if dup(f) == 0 as ::core::ffi::c_int {
        } else {
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr() as *const ::core::ffi::c_char,
                1117 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dup(f)==STDIN_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr() as *const ::core::ffi::c_char,
                1117 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dup(f)==STDIN_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        close(STDOUT_FILENO);
        if dup(f) == 1 as ::core::ffi::c_int {
        } else {
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr() as *const ::core::ffi::c_char,
                1119 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dup(f)==STDOUT_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr() as *const ::core::ffi::c_char,
                1119 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dup(f)==STDOUT_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        close(STDERR_FILENO);
        if dup(f) == 2 as ::core::ffi::c_int {
        } else {
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr() as *const ::core::ffi::c_char,
                1121 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dup(f)==STDERR_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/bgsaver.c\0".as_ptr() as *const ::core::ffi::c_char,
                1121 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"dup(f)==STDERR_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        processname_set(
            b"mfsmaster (data writer)\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        close((*eptr).data_pipe[PIPE_WRITE as usize]);
        close((*eptr).status_pipe[PIPE_READ as usize]);
        bgsaver_worker();
        exit(0 as ::core::ffi::c_int);
    }
    close((*eptr).data_pipe[PIPE_READ as usize]);
    close((*eptr).status_pipe[PIPE_WRITE as usize]);
    (*eptr).mode = DATA as ::core::ffi::c_int as uint8_t;
    (*eptr).input_end = 0 as uint8_t;
    (*eptr).input_bytesleft = 8 as uint32_t;
    (*eptr).input_startptr = &raw mut (*eptr).input_hdr as *mut uint8_t;
    (*eptr).input_packet = ::core::ptr::null_mut::<in_packetstruct>();
    (*eptr).inputhead = ::core::ptr::null_mut::<in_packetstruct>();
    (*eptr).inputtail = &raw mut (*eptr).inputhead;
    (*eptr).outputhead = ::core::ptr::null_mut::<out_packetstruct>();
    (*eptr).outputtail = &raw mut (*eptr).outputhead;
    (*eptr).pdescpos_r = -1 as ::core::ffi::c_int as int32_t;
    (*eptr).pdescpos_w = -1 as ::core::ffi::c_int as int32_t;
    terminating = 0 as uint8_t;
    bgsaver_last_activity = monotonic_seconds();
    bgsaver_last_check = monotonic_seconds();
    bgsaver_last_check_count = 0 as uint32_t;
    bgsaver_last_report = 0 as uint32_t;
    termdelay = 0 as uint8_t;
    main_wantexit_register_fname(
        Some(bgsaver_wantexit as unsafe extern "C" fn() -> ()),
        b"bgsaver_wantexit\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_time_register_fname(
        1 as uint32_t,
        0 as uint32_t,
        Some(bgsaver_alive_check as unsafe extern "C" fn() -> ()),
        b"bgsaver_alive_check\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_canexit_register_fname(
        Some(bgsaver_canexit as unsafe extern "C" fn() -> ::core::ffi::c_int),
        b"bgsaver_canexit\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_reload_register_fname(
        Some(bgsaver_reload as unsafe extern "C" fn() -> ()),
        b"bgsaver_reload\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_destruct_register_fname(
        Some(bgsaver_term as unsafe extern "C" fn() -> ()),
        b"bgsaver_term\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_poll_register_fname(
        Some(bgsaver_desc as unsafe extern "C" fn(*mut pollfd, *mut uint32_t) -> ()),
        Some(bgsaver_serve as unsafe extern "C" fn(*mut pollfd) -> ()),
        b"bgsaver_desc\0".as_ptr() as *const ::core::ffi::c_char,
        b"bgsaver_serve\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
