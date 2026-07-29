use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn cfg_isdefined(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn cfg_getdefaultstr(name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn cfg_getstr(
        name: *const ::core::ffi::c_char,
        def: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn main_destruct_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    fn main_wantexit_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
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
    fn monotonic_seconds() -> ::core::ffi::c_double;
    fn tcpsocket() -> ::core::ffi::c_int;
    fn tcpresolve(
        hostname: *const ::core::ffi::c_char,
        service: *const ::core::ffi::c_char,
        ip: *mut uint32_t,
        port: *mut uint16_t,
        passiveflag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tcpnonblock(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn tcpsetacceptfilter(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn tcpreuseaddr(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn tcpnodelay(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn tcpnumlisten(
        sock: ::core::ffi::c_int,
        ip: uint32_t,
        port: uint16_t,
        queue: uint16_t,
    ) -> ::core::ffi::c_int;
    fn tcpaccept(lsock_0: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn tcpclose(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn hdd_clear_errors(pleng: uint32_t, path: *const uint8_t) -> uint8_t;
    fn hdd_diskinfo_size() -> uint32_t;
    fn hdd_diskinfo_data(buff: *mut uint8_t);
    fn hdd_diskinfo_monotonic_size() -> uint32_t;
    fn hdd_diskinfo_monotonic_data(buff: *mut uint8_t);
    fn masterconn_getcsid() -> uint16_t;
    fn masterconn_getmetaid() -> uint64_t;
    fn masterconn_getmasterip() -> uint32_t;
    fn masterconn_getmasterport() -> uint16_t;
    fn masterconn_forcereconnect();
    fn charts_monotonic_data(buff: *mut uint8_t) -> uint32_t;
    fn charts_makedata(
        buff: *mut uint8_t,
        number: uint32_t,
        maxentries: uint32_t,
        multimode: uint8_t,
    ) -> uint32_t;
    fn charts_make_png(chartid: uint32_t, chartwidth: uint32_t, chartheight: uint32_t) -> uint32_t;
    fn charts_get_png(buff: *mut uint8_t);
    fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn job_pool_disable_job(jobid: uint32_t);
    fn job_pool_change_callback(
        jobid: uint32_t,
        callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
        extra: *mut ::core::ffi::c_void,
    );
    fn job_serv_read(
        callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
        extra: *mut ::core::ffi::c_void,
        sock: ::core::ffi::c_int,
        packet: *const uint8_t,
        length: uint32_t,
    ) -> uint32_t;
    fn job_serv_write(
        callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
        extra: *mut ::core::ffi::c_void,
        sock: ::core::ffi::c_int,
        packet: *const uint8_t,
        length: uint32_t,
    ) -> uint32_t;
    fn job_get_chunk_info(
        callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
        extra: *mut ::core::ffi::c_void,
        chunkid: uint64_t,
        version: uint32_t,
        requested_info: uint8_t,
        info_buff: *mut uint8_t,
    ) -> uint32_t;
    fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type ssize_t = isize;
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csserventry {
    pub state: uint8_t,
    pub mode: uint8_t,
    pub sock: ::core::ffi::c_int,
    pub pdescpos: int32_t,
    pub lastread: ::core::ffi::c_double,
    pub lastwrite: ::core::ffi::c_double,
    pub activity: uint32_t,
    pub hdrbuff: [uint8_t; 8],
    pub inputpacket: packetstruct,
    pub outputhead: *mut packetstruct,
    pub outputtail: *mut *mut packetstruct,
    pub jobid: uint32_t,
    pub idlejobs: *mut idlejob,
    pub next: *mut csserventry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct idlejob {
    pub jobid: uint32_t,
    pub op: uint8_t,
    pub chunkid: uint64_t,
    pub version: uint32_t,
    pub requested_info: uint8_t,
    pub eptr: *mut csserventry,
    pub next: *mut idlejob,
    pub prev: *mut *mut idlejob,
    pub buff: [uint8_t; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct packetstruct {
    pub next: *mut packetstruct,
    pub startptr: *mut uint8_t,
    pub bytesleft: uint32_t,
    pub packet: *mut uint8_t,
}
pub const WRITE: C2Rust_Unnamed_0 = 2;
pub const READ: C2Rust_Unnamed_0 = 1;
pub const CLOSE: C2Rust_Unnamed_0 = 3;
pub const IDLE: C2Rust_Unnamed_0 = 0;
pub type FILE = _IO_FILE;
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
pub const IJ_GET_CHUNK_INFO: C2Rust_Unnamed_1 = 3;
pub const IJ_GET_CHUNK_CHECKSUM_TAB: C2Rust_Unnamed_1 = 2;
pub const IJ_GET_CHUNK_CHECKSUM: C2Rust_Unnamed_1 = 1;
pub const IJ_GET_CHUNK_BLOCKS: C2Rust_Unnamed_1 = 0;
pub const HEADER: C2Rust_Unnamed = 0;
pub const DATA: C2Rust_Unnamed = 1;
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub type C2Rust_Unnamed_0 = ::core::ffi::c_uint;
pub type C2Rust_Unnamed_1 = ::core::ffi::c_uint;
pub const PATH_MAX: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const EOPNOTSUPP: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const ENOTSUP: ::core::ffi::c_int = EOPNOTSUPP;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_NOTDONE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MODULE_TYPE_CHUNKSERVER: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const REQUEST_BLOCKS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const REQUEST_CHECKSUM: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const REQUEST_CHECKSUM_TAB: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const REQUEST_FILE_PATH: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const CSTOCS_MAXPACKETSIZE: ::core::ffi::c_int = 100000 as ::core::ffi::c_int;
pub const ANTOAN_NOP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ANTOAN_UNKNOWN_COMMAND: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ANTOAN_BAD_COMMAND_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ANTOAN_GET_VERSION: uint32_t = 10 as uint32_t;
pub const ANTOAN_VERSION: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const ANTOAN_GET_CONFIG: uint32_t = 80 as uint32_t;
pub const ANTOAN_CONFIG_VALUE: ::core::ffi::c_int = PROTO_BASE + 81 as ::core::ffi::c_int;
pub const CLTOCS_READ: uint32_t = 200 as uint32_t;
pub const CSTOCL_READ_STATUS: ::core::ffi::c_int = PROTO_BASE + 201 as ::core::ffi::c_int;
pub const CLTOCS_WRITE: uint32_t = 210 as uint32_t;
pub const CSTOCL_WRITE_STATUS: ::core::ffi::c_int = PROTO_BASE + 211 as ::core::ffi::c_int;
pub const CLTOCS_WRITE_DATA: uint32_t = 212 as uint32_t;
pub const CLTOCS_WRITE_FINISH: uint32_t = 213 as uint32_t;
pub const ANTOCS_GET_CHUNK_BLOCKS: uint32_t = 250 as uint32_t;
pub const CSTOAN_CHUNK_BLOCKS: ::core::ffi::c_int = PROTO_BASE + 251 as ::core::ffi::c_int;
pub const ANTOCS_GET_CHUNK_CHECKSUM: uint32_t = 300 as uint32_t;
pub const CSTOAN_CHUNK_CHECKSUM: ::core::ffi::c_int = PROTO_BASE + 301 as ::core::ffi::c_int;
pub const ANTOCS_GET_CHUNK_CHECKSUM_TAB: uint32_t = 302 as uint32_t;
pub const CSTOAN_CHUNK_CHECKSUM_TAB: ::core::ffi::c_int = PROTO_BASE + 303 as ::core::ffi::c_int;
pub const ANTOCS_GET_CHUNK_INFO: uint32_t = 304 as uint32_t;
pub const CSTOAN_CHUNK_INFO: ::core::ffi::c_int = PROTO_BASE + 305 as ::core::ffi::c_int;
pub const ANTOCS_CLEAR_ERRORS: uint32_t = 306 as uint32_t;
pub const CSTOAN_CLEAR_ERRORS: ::core::ffi::c_int = PROTO_BASE + 307 as ::core::ffi::c_int;
pub const CLTOAN_MONOTONIC_DATA: uint32_t = 502 as uint32_t;
pub const ANTOCL_MONOTONIC_DATA: ::core::ffi::c_int = PROTO_BASE + 503 as ::core::ffi::c_int;
pub const CLTOAN_CHART: uint32_t = 504 as uint32_t;
pub const ANTOCL_CHART: ::core::ffi::c_int = PROTO_BASE + 505 as ::core::ffi::c_int;
pub const CLTOAN_CHART_DATA: uint32_t = 506 as uint32_t;
pub const ANTOCL_CHART_DATA: ::core::ffi::c_int = PROTO_BASE + 507 as ::core::ffi::c_int;
pub const CLTOAN_MODULE_INFO: uint32_t = 530 as uint32_t;
pub const ANTOCL_MODULE_INFO: ::core::ffi::c_int = PROTO_BASE + 531 as ::core::ffi::c_int;
pub const CLTOCS_HDD_LIST: uint32_t = 600 as uint32_t;
pub const CSTOCL_HDD_LIST: ::core::ffi::c_int = PROTO_BASE + 601 as ::core::ffi::c_int;
pub const DEFAULT_CS_DATA_PORT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"9422\0") };
pub const PROTO_BASE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const VERSMAJ: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const VERSMID: ::core::ffi::c_int = 59 as ::core::ffi::c_int;
pub const VERSMIN: ::core::ffi::c_int = 2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
pub const VERSSTR: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"4.59.2-1\0") };
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
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const POLLOUT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const POLLERR: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const POLLHUP: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_ERRNO_SYSLOG_STDERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CSSERV_TIMEOUT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MaxPacketSize: ::core::ffi::c_int = CSTOCS_MAXPACKETSIZE;
static mut csservhead: *mut csserventry = ::core::ptr::null_mut::<csserventry>();
static mut lsock: ::core::ffi::c_int = 0;
static mut lsockpdescpos: int32_t = 0;
static mut mylistenip: uint32_t = 0;
static mut mylistenport: uint16_t = 0;
static mut stats_bytesin: uint64_t = 0 as uint64_t;
static mut stats_bytesout: uint64_t = 0 as uint64_t;
static mut ListenHost: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut ListenPort: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[no_mangle]
pub unsafe extern "C" fn csserv_stats(mut bin: *mut uint64_t, mut bout: *mut uint64_t) {
    *bin = stats_bytesin;
    *bout = stats_bytesout;
    stats_bytesin = 0 as uint64_t;
    stats_bytesout = 0 as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn csserv_create_packet(
    mut eptr: *mut csserventry,
    mut r#type: uint32_t,
    mut size: uint32_t,
) -> *mut uint8_t {
    let mut outpacket: *mut packetstruct = ::core::ptr::null_mut::<packetstruct>();
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut psize: uint32_t = 0;
    outpacket = malloc(::core::mem::size_of::<packetstruct>()) as *mut packetstruct;
    if outpacket.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/csserv.c\0".as_ptr() as *const ::core::ffi::c_char,
            131 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"outpacket\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/csserv.c\0".as_ptr() as *const ::core::ffi::c_char,
            131 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"outpacket\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if outpacket
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut packetstruct
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/csserv.c\0".as_ptr() as *const ::core::ffi::c_char,
            131 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"outpacket\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/csserv.c\0".as_ptr() as *const ::core::ffi::c_char,
            131 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"outpacket\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    psize = size.wrapping_add(8 as uint32_t);
    (*outpacket).packet = malloc(psize as size_t) as *mut uint8_t;
    if (*outpacket).packet.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/csserv.c\0".as_ptr() as *const ::core::ffi::c_char,
            137 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"outpacket->packet\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/csserv.c\0".as_ptr() as *const ::core::ffi::c_char,
            137 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"outpacket->packet\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if (*outpacket).packet
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut uint8_t
    {
        let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/csserv.c\0".as_ptr() as *const ::core::ffi::c_char,
            137 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"outpacket->packet\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring_0,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfschunkserver/csserv.c\0".as_ptr() as *const ::core::ffi::c_char,
            137 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"outpacket->packet\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring_0,
        );
        abort();
    }
    (*outpacket).bytesleft = psize;
    ptr = (*outpacket).packet;
    put32bit(&raw mut ptr, r#type);
    put32bit(&raw mut ptr, size);
    (*outpacket).startptr = (*outpacket).packet;
    (*outpacket).next = ::core::ptr::null_mut::<packetstruct>();
    *(*eptr).outputtail = outpacket;
    (*eptr).outputtail = &raw mut (*outpacket).next as *mut *mut packetstruct;
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn csserv_get_version(
    mut eptr: *mut csserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    let mut msgid: uint32_t = 0 as uint32_t;
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    static mut vstring: [::core::ffi::c_char; 9] = VERSSTR;
    if length != 0 as uint32_t && length != 4 as uint32_t {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"ANTOAN_GET_VERSION - wrong size (%u/4|0)\0".as_ptr() as *const ::core::ffi::c_char,
            length,
        );
        (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
        return;
    }
    if length == 4 as uint32_t {
        msgid = get32bit(&raw mut data);
        ptr = csserv_create_packet(
            eptr,
            ANTOAN_VERSION as uint32_t,
            ((4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as size_t)
                .wrapping_add(strlen(&raw const vstring as *const ::core::ffi::c_char))
                as uint32_t,
        );
        put32bit(&raw mut ptr, msgid);
    } else {
        ptr = csserv_create_packet(
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
#[no_mangle]
pub unsafe extern "C" fn csserv_get_config(
    mut eptr: *mut csserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
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
        (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
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
        (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
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
    ptr = csserv_create_packet(
        eptr,
        ANTOAN_CONFIG_VALUE as uint32_t,
        (5 as uint32_t).wrapping_add(vleng),
    );
    put32bit(&raw mut ptr, msgid);
    put8bit(&raw mut ptr, vleng as uint8_t);
    if vleng > 0 as uint32_t && !val.is_null() {
        memcpy(
            ptr as *mut ::core::ffi::c_void,
            val as *const ::core::ffi::c_void,
            vleng as size_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn csserv_iothread_finished(
    mut status: uint8_t,
    mut e: *mut ::core::ffi::c_void,
) {
    let mut eptr: *mut csserventry = e as *mut csserventry;
    if status as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
    } else {
        (*eptr).state = IDLE as ::core::ffi::c_int as uint8_t;
        (*eptr).lastwrite = monotonic_seconds();
        (*eptr).lastread = (*eptr).lastwrite;
    }
    (*eptr).jobid = 0 as uint32_t;
    if !(*eptr).inputpacket.packet.is_null() {
        free((*eptr).inputpacket.packet as *mut ::core::ffi::c_void);
    }
    (*eptr).inputpacket.packet = ::core::ptr::null_mut::<uint8_t>();
}
#[no_mangle]
pub unsafe extern "C" fn csserv_read_init(
    mut eptr: *mut csserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    (*eptr).state = READ as ::core::ffi::c_int as uint8_t;
    (*eptr).jobid = job_serv_read(
        Some(
            csserv_iothread_finished
                as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
        ),
        eptr as *mut ::core::ffi::c_void,
        (*eptr).sock,
        data,
        length,
    );
    if (*eptr).jobid == 0 as uint32_t {
        if length != 20 as uint32_t && length != 21 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOCS_READ - wrong size (%u/20|21)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length == 21 as uint32_t {
            data = data.offset(1);
        }
        ptr = csserv_create_packet(
            eptr,
            CSTOCL_READ_STATUS as uint32_t,
            (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t,
        );
        memcpy(
            ptr as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            8 as size_t,
        );
        ptr = ptr.offset(8 as ::core::ffi::c_int as isize);
        put8bit(&raw mut ptr, MFS_ERROR_NOTDONE as uint8_t);
        (*eptr).state = IDLE as ::core::ffi::c_int as uint8_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csserv_write_init(
    mut eptr: *mut csserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    (*eptr).state = WRITE as ::core::ffi::c_int as uint8_t;
    (*eptr).jobid = job_serv_write(
        Some(
            csserv_iothread_finished
                as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
        ),
        eptr as *mut ::core::ffi::c_void,
        (*eptr).sock,
        data,
        length,
    );
    if (*eptr).jobid == 0 as uint32_t {
        if length & 1 as uint32_t != 0 {
            if length < 13 as uint32_t
                || length
                    .wrapping_sub(13 as uint32_t)
                    .wrapping_rem(6 as uint32_t)
                    != 0 as uint32_t
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CLTOCS_WRITE - wrong size (%u/13+N*6)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    length,
                );
                (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
                return;
            }
            data = data.offset(1);
        } else if length < 12 as uint32_t
            || length
                .wrapping_sub(12 as uint32_t)
                .wrapping_rem(6 as uint32_t)
                != 0 as uint32_t
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CLTOCS_WRITE - wrong size (%u/12+N*6)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
            return;
        }
        ptr = csserv_create_packet(
            eptr,
            CSTOCL_WRITE_STATUS as uint32_t,
            (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as uint32_t,
        );
        memcpy(
            ptr as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            8 as size_t,
        );
        ptr = ptr.offset(8 as ::core::ffi::c_int as isize);
        put32bit(&raw mut ptr, 0 as uint32_t);
        put8bit(&raw mut ptr, MFS_ERROR_NOTDONE as uint8_t);
        (*eptr).state = IDLE as ::core::ffi::c_int as uint8_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csserv_idlejob_finished(
    mut status: uint8_t,
    mut ijp: *mut ::core::ffi::c_void,
) {
    let mut ij: *mut idlejob = ijp as *mut idlejob;
    let mut eptr: *mut csserventry = (*ij).eptr as *mut csserventry;
    let mut cpsize: uint32_t = 0;
    let mut psize: uint32_t = 0;
    let mut pleng: uint32_t = 0;
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    if !eptr.is_null() {
        match (*ij).op as ::core::ffi::c_int {
            0 => {
                ptr = csserv_create_packet(
                    eptr,
                    CSTOAN_CHUNK_BLOCKS as uint32_t,
                    (8 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as uint32_t,
                );
                put64bit(&raw mut ptr, (*ij).chunkid);
                put32bit(&raw mut ptr, (*ij).version);
                if status as ::core::ffi::c_int == MFS_STATUS_OK {
                    memcpy(
                        ptr as *mut ::core::ffi::c_void,
                        &raw mut (*ij).buff as *mut uint8_t as *const ::core::ffi::c_void,
                        2 as size_t,
                    );
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                } else {
                    put16bit(&raw mut ptr, 0 as uint16_t);
                }
                put8bit(&raw mut ptr, status);
            }
            1 => {
                if status as ::core::ffi::c_int != MFS_STATUS_OK {
                    ptr = csserv_create_packet(
                        eptr,
                        CSTOAN_CHUNK_CHECKSUM as uint32_t,
                        (8 as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as uint32_t,
                    );
                } else {
                    ptr = csserv_create_packet(
                        eptr,
                        CSTOAN_CHUNK_CHECKSUM as uint32_t,
                        (8 as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int) as uint32_t,
                    );
                }
                put64bit(&raw mut ptr, (*ij).chunkid);
                put32bit(&raw mut ptr, (*ij).version);
                if status as ::core::ffi::c_int != MFS_STATUS_OK {
                    put8bit(&raw mut ptr, status);
                } else {
                    memcpy(
                        ptr as *mut ::core::ffi::c_void,
                        &raw mut (*ij).buff as *mut uint8_t as *const ::core::ffi::c_void,
                        4 as size_t,
                    );
                }
            }
            2 => {
                if status as ::core::ffi::c_int != MFS_STATUS_OK {
                    ptr = csserv_create_packet(
                        eptr,
                        CSTOAN_CHUNK_CHECKSUM_TAB as uint32_t,
                        (8 as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as uint32_t,
                    );
                } else {
                    ptr = csserv_create_packet(
                        eptr,
                        CSTOAN_CHUNK_CHECKSUM_TAB as uint32_t,
                        (8 as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int
                            + 4096 as ::core::ffi::c_int) as uint32_t,
                    );
                }
                put64bit(&raw mut ptr, (*ij).chunkid);
                put32bit(&raw mut ptr, (*ij).version);
                if status as ::core::ffi::c_int != MFS_STATUS_OK {
                    put8bit(&raw mut ptr, status);
                } else {
                    memcpy(
                        ptr as *mut ::core::ffi::c_void,
                        &raw mut (*ij).buff as *mut uint8_t as *const ::core::ffi::c_void,
                        4096 as size_t,
                    );
                }
            }
            3 => {
                if status as ::core::ffi::c_int != MFS_STATUS_OK {
                    ptr = csserv_create_packet(
                        eptr,
                        CSTOAN_CHUNK_INFO as uint32_t,
                        (8 as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as uint32_t,
                    );
                    put64bit(&raw mut ptr, (*ij).chunkid);
                    put32bit(&raw mut ptr, (*ij).version);
                    put8bit(&raw mut ptr, status);
                } else {
                    cpsize = 0 as uint32_t;
                    pleng = 0 as uint32_t;
                    if (*ij).requested_info as ::core::ffi::c_int & REQUEST_BLOCKS != 0 {
                        cpsize = cpsize.wrapping_add(2 as uint32_t);
                    }
                    if (*ij).requested_info as ::core::ffi::c_int & REQUEST_CHECKSUM != 0 {
                        cpsize = cpsize.wrapping_add(4 as uint32_t);
                    }
                    if (*ij).requested_info as ::core::ffi::c_int & REQUEST_CHECKSUM_TAB != 0 {
                        cpsize = cpsize.wrapping_add(4096 as uint32_t);
                    }
                    psize = cpsize;
                    if (*ij).requested_info as ::core::ffi::c_int & REQUEST_FILE_PATH != 0 {
                        pleng = strlen(
                            (&raw mut (*ij).buff as *mut uint8_t).offset(cpsize as isize)
                                as *mut ::core::ffi::c_char,
                        ) as uint32_t;
                        psize = psize.wrapping_add((4 as uint32_t).wrapping_add(pleng));
                    }
                    ptr = csserv_create_packet(
                        eptr,
                        CSTOAN_CHUNK_INFO as uint32_t,
                        ((8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t)
                            .wrapping_add(psize),
                    );
                    put64bit(&raw mut ptr, (*ij).chunkid);
                    put32bit(&raw mut ptr, (*ij).version);
                    psize = 0 as uint32_t;
                    memcpy(
                        ptr as *mut ::core::ffi::c_void,
                        &raw mut (*ij).buff as *mut uint8_t as *const ::core::ffi::c_void,
                        cpsize as size_t,
                    );
                    ptr = ptr.offset(cpsize as isize);
                    if (*ij).requested_info as ::core::ffi::c_int & REQUEST_FILE_PATH != 0 {
                        put32bit(&raw mut ptr, pleng);
                        memcpy(
                            ptr as *mut ::core::ffi::c_void,
                            (&raw mut (*ij).buff as *mut uint8_t).offset(cpsize as isize)
                                as *const ::core::ffi::c_void,
                            pleng as size_t,
                        );
                    }
                }
            }
            _ => {}
        }
        *(*ij).prev = (*ij).next;
        if !(*ij).next.is_null() {
            (*(*ij).next).prev = (*ij).prev;
        }
    }
    free(ij as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn csserv_get_chunk_blocks(
    mut eptr: *mut csserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    let mut ij: *mut idlejob = ::core::ptr::null_mut::<idlejob>();
    if length != (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"ANTOCS_GET_CHUNK_BLOCKS - wrong size (%u/12)\0".as_ptr()
                as *const ::core::ffi::c_char,
            length,
        );
        (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
        return;
    }
    ij = malloc(
        if (48 as usize).wrapping_add(2 as usize) < ::core::mem::size_of::<idlejob>() {
            ::core::mem::size_of::<idlejob>()
        } else {
            (48 as size_t).wrapping_add(2 as size_t)
        },
    ) as *mut idlejob;
    (*ij).op = IJ_GET_CHUNK_BLOCKS as ::core::ffi::c_int as uint8_t;
    (*ij).chunkid = get64bit(&raw mut data);
    (*ij).version = get32bit(&raw mut data);
    (*ij).requested_info = 0 as uint8_t;
    (*ij).eptr = eptr as *mut csserventry;
    (*ij).next = (*eptr).idlejobs;
    (*ij).prev = &raw mut (*eptr).idlejobs;
    (*eptr).idlejobs = ij as *mut idlejob;
    (*ij).jobid = job_get_chunk_info(
        Some(
            csserv_idlejob_finished
                as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
        ),
        ij as *mut ::core::ffi::c_void,
        (*ij).chunkid,
        (*ij).version,
        REQUEST_BLOCKS as uint8_t,
        &raw mut (*ij).buff as *mut uint8_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn csserv_get_chunk_checksum(
    mut eptr: *mut csserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    let mut ij: *mut idlejob = ::core::ptr::null_mut::<idlejob>();
    if length != (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"ANTOCS_GET_CHUNK_CHECKSUM - wrong size (%u/12)\0".as_ptr()
                as *const ::core::ffi::c_char,
            length,
        );
        (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
        return;
    }
    ij = malloc(
        if (48 as usize).wrapping_add(4 as usize) < ::core::mem::size_of::<idlejob>() {
            ::core::mem::size_of::<idlejob>()
        } else {
            (48 as size_t).wrapping_add(4 as size_t)
        },
    ) as *mut idlejob;
    (*ij).op = IJ_GET_CHUNK_CHECKSUM as ::core::ffi::c_int as uint8_t;
    (*ij).chunkid = get64bit(&raw mut data);
    (*ij).version = get32bit(&raw mut data);
    (*ij).requested_info = 0 as uint8_t;
    (*ij).eptr = eptr as *mut csserventry;
    (*ij).next = (*eptr).idlejobs;
    (*ij).prev = &raw mut (*eptr).idlejobs;
    (*eptr).idlejobs = ij as *mut idlejob;
    (*ij).jobid = job_get_chunk_info(
        Some(
            csserv_idlejob_finished
                as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
        ),
        ij as *mut ::core::ffi::c_void,
        (*ij).chunkid,
        (*ij).version,
        REQUEST_CHECKSUM as uint8_t,
        &raw mut (*ij).buff as *mut uint8_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn csserv_get_chunk_checksum_tab(
    mut eptr: *mut csserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    let mut ij: *mut idlejob = ::core::ptr::null_mut::<idlejob>();
    if length != (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"ANTOCS_GET_CHUNK_CHECKSUM_TAB - wrong size (%u/12)\0".as_ptr()
                as *const ::core::ffi::c_char,
            length,
        );
        (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
        return;
    }
    ij = malloc(
        if (48 as usize).wrapping_add(4096 as usize) < ::core::mem::size_of::<idlejob>() {
            ::core::mem::size_of::<idlejob>()
        } else {
            (48 as size_t).wrapping_add(4096 as size_t)
        },
    ) as *mut idlejob;
    (*ij).op = IJ_GET_CHUNK_CHECKSUM_TAB as ::core::ffi::c_int as uint8_t;
    (*ij).chunkid = get64bit(&raw mut data);
    (*ij).version = get32bit(&raw mut data);
    (*ij).requested_info = 0 as uint8_t;
    (*ij).eptr = eptr as *mut csserventry;
    (*ij).next = (*eptr).idlejobs;
    (*ij).prev = &raw mut (*eptr).idlejobs;
    (*eptr).idlejobs = ij as *mut idlejob;
    (*ij).jobid = job_get_chunk_info(
        Some(
            csserv_idlejob_finished
                as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
        ),
        ij as *mut ::core::ffi::c_void,
        (*ij).chunkid,
        (*ij).version,
        REQUEST_CHECKSUM_TAB as uint8_t,
        &raw mut (*ij).buff as *mut uint8_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn csserv_get_chunk_info(
    mut eptr: *mut csserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    let mut ij: *mut idlejob = ::core::ptr::null_mut::<idlejob>();
    let mut chunkid: uint64_t = 0;
    let mut version: uint32_t = 0;
    let mut requested_info: uint8_t = 0;
    let mut buffsize: uint32_t = 0;
    if length
        != (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t
    {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"ANTOCS_GET_CHUNK_INFO - wrong size (%u/13)\0".as_ptr() as *const ::core::ffi::c_char,
            length,
        );
        (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
        return;
    }
    chunkid = get64bit(&raw mut data);
    version = get32bit(&raw mut data);
    requested_info = get8bit(&raw mut data);
    buffsize = 0 as uint32_t;
    if requested_info as ::core::ffi::c_int & REQUEST_BLOCKS != 0 {
        buffsize = buffsize.wrapping_add(2 as uint32_t);
    }
    if requested_info as ::core::ffi::c_int & REQUEST_CHECKSUM != 0 {
        buffsize = buffsize.wrapping_add(4 as uint32_t);
    }
    if requested_info as ::core::ffi::c_int & REQUEST_CHECKSUM_TAB != 0 {
        buffsize = buffsize.wrapping_add(4096 as uint32_t);
    }
    if requested_info as ::core::ffi::c_int & REQUEST_FILE_PATH != 0 {
        buffsize = buffsize.wrapping_add(PATH_MAX as uint32_t);
    }
    ij = malloc(
        if (48 as usize).wrapping_add(buffsize as usize) < ::core::mem::size_of::<idlejob>() {
            ::core::mem::size_of::<idlejob>()
        } else {
            (48 as size_t).wrapping_add(buffsize as size_t)
        },
    ) as *mut idlejob;
    (*ij).op = IJ_GET_CHUNK_INFO as ::core::ffi::c_int as uint8_t;
    (*ij).chunkid = chunkid;
    (*ij).version = version;
    (*ij).requested_info = requested_info;
    (*ij).eptr = eptr as *mut csserventry;
    (*ij).next = (*eptr).idlejobs;
    (*ij).prev = &raw mut (*eptr).idlejobs;
    (*eptr).idlejobs = ij as *mut idlejob;
    (*ij).jobid = job_get_chunk_info(
        Some(
            csserv_idlejob_finished
                as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
        ),
        ij as *mut ::core::ffi::c_void,
        chunkid,
        version,
        requested_info,
        &raw mut (*ij).buff as *mut uint8_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn csserv_hdd_list(
    mut eptr: *mut csserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    let mut l: uint32_t = 0;
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    if length != 0 as uint32_t {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"CLTOCS_HDD_LIST - wrong size (%u/0)\0".as_ptr() as *const ::core::ffi::c_char,
            length,
        );
        (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
        return;
    }
    l = hdd_diskinfo_size();
    ptr = csserv_create_packet(eptr, CSTOCL_HDD_LIST as uint32_t, l);
    hdd_diskinfo_data(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn csserv_chart(
    mut eptr: *mut csserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    let mut chartid: uint32_t = 0;
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut l: uint32_t = 0;
    let mut w: uint16_t = 0;
    let mut h: uint16_t = 0;
    if length != 4 as uint32_t && length != 8 as uint32_t {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"CLTOAN_CHART - wrong size (%u/4|8)\0".as_ptr() as *const ::core::ffi::c_char,
            length,
        );
        (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
        return;
    }
    chartid = get32bit(&raw mut data);
    if length == 8 as uint32_t {
        w = get16bit(&raw mut data);
        h = get16bit(&raw mut data);
    } else {
        w = 0 as uint16_t;
        h = 0 as uint16_t;
    }
    l = charts_make_png(chartid, w as uint32_t, h as uint32_t);
    ptr = csserv_create_packet(eptr, ANTOCL_CHART as uint32_t, l);
    if l > 0 as uint32_t {
        charts_get_png(ptr);
    }
}
#[no_mangle]
pub unsafe extern "C" fn csserv_chart_data(
    mut eptr: *mut csserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    let mut chartid: uint32_t = 0;
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut l: uint32_t = 0;
    let mut maxentries: uint32_t = 0;
    let mut multimode: uint8_t = 0;
    if length != 4 as uint32_t && length != 8 as uint32_t && length != 9 as uint32_t {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"CLTOAN_CHART_DATA - wrong size (%u/4|8|9)\0".as_ptr() as *const ::core::ffi::c_char,
            length,
        );
        (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
        return;
    }
    maxentries = 0xffffffff as ::core::ffi::c_uint as uint32_t;
    multimode = 0 as uint8_t;
    chartid = get32bit(&raw mut data);
    if length >= 8 as uint32_t {
        maxentries = get32bit(&raw mut data);
    }
    if length >= 9 as uint32_t {
        multimode = get8bit(&raw mut data);
    }
    l = charts_makedata(
        ::core::ptr::null_mut::<uint8_t>(),
        chartid,
        maxentries,
        multimode,
    );
    ptr = csserv_create_packet(eptr, ANTOCL_CHART_DATA as uint32_t, l);
    if l > 0 as uint32_t {
        charts_makedata(ptr, chartid, maxentries, multimode);
    }
}
#[no_mangle]
pub unsafe extern "C" fn csserv_monotonic_data(
    mut eptr: *mut csserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut l: uint32_t = 0;
    let mut dil: uint32_t = 0;
    if length != 0 as uint32_t {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"CLTOAN_MONOTONIC_DATA - wrong size (%u/0)\0".as_ptr() as *const ::core::ffi::c_char,
            length,
        );
        (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
        return;
    }
    l = charts_monotonic_data(::core::ptr::null_mut::<uint8_t>());
    dil = hdd_diskinfo_monotonic_size();
    ptr = csserv_create_packet(eptr, ANTOCL_MONOTONIC_DATA as uint32_t, l.wrapping_add(dil));
    if l > 0 as uint32_t {
        charts_monotonic_data(ptr);
        ptr = ptr.offset(l as isize);
    }
    hdd_diskinfo_monotonic_data(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn csserv_module_info(
    mut eptr: *mut csserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    if length != 0 as uint32_t {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"CLTOAN_MODULE_INFO - wrong size (%u/0)\0".as_ptr() as *const ::core::ffi::c_char,
            length,
        );
        (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
        return;
    }
    ptr = csserv_create_packet(eptr, ANTOCL_MODULE_INFO as uint32_t, 21 as uint32_t);
    put8bit(&raw mut ptr, MODULE_TYPE_CHUNKSERVER as uint8_t);
    put16bit(&raw mut ptr, VERSMAJ as uint16_t);
    put8bit(&raw mut ptr, VERSMID as uint8_t);
    put8bit(&raw mut ptr, VERSMIN as uint8_t);
    put16bit(&raw mut ptr, masterconn_getcsid());
    put64bit(&raw mut ptr, masterconn_getmetaid());
    put32bit(&raw mut ptr, masterconn_getmasterip());
    put16bit(&raw mut ptr, masterconn_getmasterport());
}
#[no_mangle]
pub unsafe extern "C" fn csserv_clear_errors(
    mut eptr: *mut csserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut pleng: uint32_t = 0;
    let mut res: uint8_t = 0;
    if length < 4 as uint32_t {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"ANTOCS_CLEAR_ERRORS - wrong size (%u/>=4)\0".as_ptr() as *const ::core::ffi::c_char,
            length,
        );
        (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
        return;
    }
    pleng = get32bit(&raw mut data);
    if length != pleng.wrapping_add(4 as uint32_t) {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"ANTOCS_CLEAR_ERRORS - wrong size (%u/4+%u)\0".as_ptr() as *const ::core::ffi::c_char,
            length,
            pleng,
        );
        (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
        return;
    }
    res = hdd_clear_errors(pleng, data);
    ptr = csserv_create_packet(eptr, CSTOAN_CLEAR_ERRORS as uint32_t, 1 as uint32_t);
    put8bit(&raw mut ptr, res);
}
#[no_mangle]
pub unsafe extern "C" fn csserv_close(mut eptr: *mut csserventry) {
    let mut ij: *mut idlejob = ::core::ptr::null_mut::<idlejob>();
    let mut nij: *mut idlejob = ::core::ptr::null_mut::<idlejob>();
    if (*eptr).jobid > 0 as uint32_t
        && ((*eptr).state as ::core::ffi::c_int == READ as ::core::ffi::c_int
            || (*eptr).state as ::core::ffi::c_int == WRITE as ::core::ffi::c_int)
    {
        job_pool_disable_job((*eptr).jobid);
        job_pool_change_callback((*eptr).jobid, None, NULL);
    }
    ij = (*eptr).idlejobs as *mut idlejob;
    while !ij.is_null() {
        nij = (*ij).next as *mut idlejob;
        job_pool_disable_job((*ij).jobid);
        (*ij).next = ::core::ptr::null_mut::<idlejob>();
        (*ij).prev = ::core::ptr::null_mut::<*mut idlejob>();
        (*ij).eptr = ::core::ptr::null_mut::<csserventry>();
        ij = nij;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csserv_gotpacket(
    mut eptr: *mut csserventry,
    mut r#type: uint32_t,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    if r#type == ANTOAN_NOP as uint32_t {
        return;
    }
    if r#type == ANTOAN_UNKNOWN_COMMAND as uint32_t {
        return;
    }
    if r#type == ANTOAN_BAD_COMMAND_SIZE as uint32_t {
        return;
    }
    if (*eptr).state as ::core::ffi::c_int == IDLE as ::core::ffi::c_int {
        match r#type {
            10 => {
                csserv_get_version(eptr, data, length);
            }
            80 => {
                csserv_get_config(eptr, data, length);
            }
            200 => {
                csserv_read_init(eptr, data, length);
            }
            210 => {
                csserv_write_init(eptr, data, length);
            }
            250 => {
                csserv_get_chunk_blocks(eptr, data, length);
            }
            300 => {
                csserv_get_chunk_checksum(eptr, data, length);
            }
            302 => {
                csserv_get_chunk_checksum_tab(eptr, data, length);
            }
            304 => {
                csserv_get_chunk_info(eptr, data, length);
            }
            600 => {
                csserv_hdd_list(eptr, data, length);
            }
            504 => {
                csserv_chart(eptr, data, length);
            }
            506 => {
                csserv_chart_data(eptr, data, length);
            }
            502 => {
                csserv_monotonic_data(eptr, data, length);
            }
            530 => {
                csserv_module_info(eptr, data, length);
            }
            306 => {
                csserv_clear_errors(eptr, data, length);
            }
            212 | 213 => {
                (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
            }
            _ => {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"got unknown message (type:%u)\0".as_ptr() as *const ::core::ffi::c_char,
                    r#type,
                );
                (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
            }
        }
    } else {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_WARNING,
            b"got unknown message (type:%u)\0".as_ptr() as *const ::core::ffi::c_char,
            r#type,
        );
        (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn csserv_wantexit() {
    mfs_log(
        MFSLOG_SYSLOG,
        MFSLOG_INFO,
        b"closing %s:%s\0".as_ptr() as *const ::core::ffi::c_char,
        ListenHost,
        ListenPort,
    );
    tcpclose(lsock);
    lsock = -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn csserv_term() {
    let mut eptr: *mut csserventry = ::core::ptr::null_mut::<csserventry>();
    let mut eaptr: *mut csserventry = ::core::ptr::null_mut::<csserventry>();
    let mut pptr: *mut packetstruct = ::core::ptr::null_mut::<packetstruct>();
    let mut paptr: *mut packetstruct = ::core::ptr::null_mut::<packetstruct>();
    eptr = csservhead;
    while !eptr.is_null() {
        tcpclose((*eptr).sock);
        if !(*eptr).inputpacket.packet.is_null() {
            free((*eptr).inputpacket.packet as *mut ::core::ffi::c_void);
        }
        pptr = (*eptr).outputhead;
        while !pptr.is_null() {
            if !(*pptr).packet.is_null() {
                free((*pptr).packet as *mut ::core::ffi::c_void);
            }
            paptr = pptr;
            pptr = (*pptr).next as *mut packetstruct;
            free(paptr as *mut ::core::ffi::c_void);
        }
        eaptr = eptr;
        eptr = (*eptr).next as *mut csserventry;
        free(eaptr as *mut ::core::ffi::c_void);
    }
    csservhead = ::core::ptr::null_mut::<csserventry>();
    free(ListenHost as *mut ::core::ffi::c_void);
    free(ListenPort as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn csserv_read(mut eptr: *mut csserventry) {
    let mut i: int32_t = 0;
    let mut r#type: uint32_t = 0;
    let mut size: uint32_t = 0;
    let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
    if (*eptr).mode as ::core::ffi::c_int == HEADER as ::core::ffi::c_int {
        i = read(
            (*eptr).sock,
            (*eptr).inputpacket.startptr as *mut ::core::ffi::c_void,
            (*eptr).inputpacket.bytesleft as size_t,
        ) as int32_t;
        if i == 0 as int32_t {
            (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
            return;
        }
        if i < 0 as int32_t {
            if *__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"(read) read error\0".as_ptr() as *const ::core::ffi::c_char,
                );
                (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
            }
            return;
        }
        stats_bytesin = stats_bytesin.wrapping_add(i as uint64_t);
        (*eptr).inputpacket.startptr = (*eptr).inputpacket.startptr.offset(i as isize);
        (*eptr).inputpacket.bytesleft = (*eptr).inputpacket.bytesleft.wrapping_sub(i as uint32_t);
        if (*eptr).inputpacket.bytesleft > 0 as uint32_t {
            return;
        }
        ptr = &raw mut (*eptr).hdrbuff as *mut uint8_t;
        r#type = get32bit(&raw mut ptr);
        size = get32bit(&raw mut ptr);
        if size > 0 as uint32_t {
            if size > MaxPacketSize as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"(read) packet too long (%u/%u) ; command:%u\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    size,
                    MaxPacketSize,
                    r#type,
                );
                (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
                return;
            }
            (*eptr).inputpacket.packet = malloc(size as size_t) as *mut uint8_t;
            if (*eptr).inputpacket.packet.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/csserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    815 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"eptr->inputpacket.packet\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/csserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    815 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"eptr->inputpacket.packet\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if (*eptr).inputpacket.packet
                == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint8_t
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/csserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    815 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"eptr->inputpacket.packet\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/csserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    815 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"eptr->inputpacket.packet\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            (*eptr).inputpacket.startptr = (*eptr).inputpacket.packet;
        }
        (*eptr).inputpacket.bytesleft = size;
        (*eptr).mode = DATA as ::core::ffi::c_int as uint8_t;
    }
    if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int {
        if (*eptr).inputpacket.bytesleft > 0 as uint32_t {
            i = read(
                (*eptr).sock,
                (*eptr).inputpacket.startptr as *mut ::core::ffi::c_void,
                (*eptr).inputpacket.bytesleft as size_t,
            ) as int32_t;
            if i == 0 as int32_t {
                (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
                return;
            }
            if i < 0 as int32_t {
                if *__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"(read) read error\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
                }
                return;
            }
            stats_bytesin = stats_bytesin.wrapping_add(i as uint64_t);
            (*eptr).inputpacket.startptr = (*eptr).inputpacket.startptr.offset(i as isize);
            (*eptr).inputpacket.bytesleft =
                (*eptr).inputpacket.bytesleft.wrapping_sub(i as uint32_t);
            if (*eptr).inputpacket.bytesleft > 0 as uint32_t {
                return;
            }
        }
        ptr = &raw mut (*eptr).hdrbuff as *mut uint8_t;
        r#type = get32bit(&raw mut ptr);
        size = get32bit(&raw mut ptr);
        (*eptr).mode = HEADER as ::core::ffi::c_int as uint8_t;
        (*eptr).inputpacket.bytesleft = 8 as uint32_t;
        (*eptr).inputpacket.startptr = &raw mut (*eptr).hdrbuff as *mut uint8_t;
        csserv_gotpacket(eptr, r#type, (*eptr).inputpacket.packet, size);
        if (*eptr).state as ::core::ffi::c_int != READ as ::core::ffi::c_int
            && (*eptr).state as ::core::ffi::c_int != WRITE as ::core::ffi::c_int
        {
            if !(*eptr).inputpacket.packet.is_null() {
                free((*eptr).inputpacket.packet as *mut ::core::ffi::c_void);
            }
            (*eptr).inputpacket.packet = ::core::ptr::null_mut::<uint8_t>();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn csserv_write(mut eptr: *mut csserventry) {
    let mut pack: *mut packetstruct = ::core::ptr::null_mut::<packetstruct>();
    let mut i: int32_t = 0;
    loop {
        pack = (*eptr).outputhead;
        if pack.is_null() {
            return;
        }
        i = write(
            (*eptr).sock,
            (*pack).startptr as *const ::core::ffi::c_void,
            (*pack).bytesleft as size_t,
        ) as int32_t;
        if i == 0 as int32_t {
            (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
            return;
        }
        if i < 0 as int32_t {
            if *__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"(write) write error\0".as_ptr() as *const ::core::ffi::c_char,
                );
                (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
            }
            return;
        }
        stats_bytesout = stats_bytesout.wrapping_add(i as uint64_t);
        (*pack).startptr = (*pack).startptr.offset(i as isize);
        (*pack).bytesleft = (*pack).bytesleft.wrapping_sub(i as uint32_t);
        if (*pack).bytesleft > 0 as uint32_t {
            return;
        }
        free((*pack).packet as *mut ::core::ffi::c_void);
        (*eptr).outputhead = (*pack).next as *mut packetstruct;
        if (*eptr).outputhead.is_null() {
            (*eptr).outputtail = &raw mut (*eptr).outputhead;
        }
        free(pack as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn csserv_desc(mut pdesc: *mut pollfd, mut ndesc: *mut uint32_t) {
    let mut pos: uint32_t = *ndesc;
    let mut eptr: *mut csserventry = ::core::ptr::null_mut::<csserventry>();
    if lsock >= 0 as ::core::ffi::c_int {
        (*pdesc.offset(pos as isize)).fd = lsock;
        (*pdesc.offset(pos as isize)).events = POLLIN as ::core::ffi::c_short;
        lsockpdescpos = pos as int32_t;
        pos = pos.wrapping_add(1);
    } else {
        lsockpdescpos = 0 as ::core::ffi::c_int as int32_t;
    }
    eptr = csservhead;
    while !eptr.is_null() {
        (*eptr).pdescpos = -1 as ::core::ffi::c_int as int32_t;
        if (*eptr).state as ::core::ffi::c_int == IDLE as ::core::ffi::c_int {
            (*pdesc.offset(pos as isize)).events = POLLIN as ::core::ffi::c_short;
            if !(*eptr).outputhead.is_null() {
                (*pdesc.offset(pos as isize)).events =
                    ((*pdesc.offset(pos as isize)).events as ::core::ffi::c_int | POLLOUT)
                        as ::core::ffi::c_short;
            }
            (*eptr).pdescpos = pos as int32_t;
            (*pdesc.offset(pos as isize)).fd = (*eptr).sock;
            pos = pos.wrapping_add(1);
        }
        eptr = (*eptr).next as *mut csserventry;
    }
    *ndesc = pos;
}
#[no_mangle]
pub unsafe extern "C" fn csserv_serve(mut pdesc: *mut pollfd) {
    let mut now: ::core::ffi::c_double = 0.;
    let mut eptr: *mut csserventry = ::core::ptr::null_mut::<csserventry>();
    let mut kptr: *mut *mut csserventry = ::core::ptr::null_mut::<*mut csserventry>();
    let mut pptr: *mut packetstruct = ::core::ptr::null_mut::<packetstruct>();
    let mut paptr: *mut packetstruct = ::core::ptr::null_mut::<packetstruct>();
    let mut ns: ::core::ffi::c_int = 0;
    now = monotonic_seconds();
    if lsockpdescpos >= 0 as int32_t
        && (*pdesc.offset(lsockpdescpos as isize)).revents as ::core::ffi::c_int & POLLIN != 0
        && lsock >= 0 as ::core::ffi::c_int
    {
        ns = tcpaccept(lsock);
        if ns < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"accept error\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            tcpnonblock(ns);
            tcpnodelay(ns);
            eptr = malloc(::core::mem::size_of::<csserventry>()) as *mut csserventry;
            if eptr.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/csserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    942 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/csserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    942 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if eptr
                == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut csserventry
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/csserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    942 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/csserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    942 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            (*eptr).next = csservhead as *mut csserventry;
            csservhead = eptr;
            (*eptr).state = IDLE as ::core::ffi::c_int as uint8_t;
            (*eptr).mode = HEADER as ::core::ffi::c_int as uint8_t;
            (*eptr).sock = ns;
            (*eptr).pdescpos = -1 as ::core::ffi::c_int as int32_t;
            (*eptr).lastread = now;
            (*eptr).lastwrite = now;
            (*eptr).inputpacket.bytesleft = 8 as uint32_t;
            (*eptr).inputpacket.startptr = &raw mut (*eptr).hdrbuff as *mut uint8_t;
            (*eptr).inputpacket.packet = ::core::ptr::null_mut::<uint8_t>();
            (*eptr).outputhead = ::core::ptr::null_mut::<packetstruct>();
            (*eptr).outputtail = &raw mut (*eptr).outputhead;
            (*eptr).jobid = 0 as uint32_t;
            (*eptr).idlejobs = ::core::ptr::null_mut::<idlejob>();
        }
    }
    eptr = csservhead;
    while !eptr.is_null() {
        if (*eptr).pdescpos >= 0 as int32_t
            && (*pdesc.offset((*eptr).pdescpos as isize)).revents as ::core::ffi::c_int
                & (POLLERR | POLLHUP)
                != 0
        {
            (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
        }
        if (*eptr).pdescpos >= 0 as int32_t
            && (*pdesc.offset((*eptr).pdescpos as isize)).revents as ::core::ffi::c_int & POLLIN
                != 0
            && (*eptr).state as ::core::ffi::c_int == IDLE as ::core::ffi::c_int
        {
            (*eptr).lastread = now;
            csserv_read(eptr);
        }
        if (*eptr).state as ::core::ffi::c_int == IDLE as ::core::ffi::c_int
            && (*eptr).lastwrite + CSSERV_TIMEOUT as ::core::ffi::c_double / 3.0f64 < now
            && (*eptr).outputhead.is_null()
        {
            csserv_create_packet(eptr, ANTOAN_NOP as uint32_t, 0 as uint32_t);
        }
        if (*eptr).pdescpos >= 0 as int32_t
            && (*pdesc.offset((*eptr).pdescpos as isize)).revents as ::core::ffi::c_int & POLLOUT
                != 0
            && (*eptr).state as ::core::ffi::c_int == IDLE as ::core::ffi::c_int
        {
            (*eptr).lastwrite = now;
            csserv_write(eptr);
        }
        if (*eptr).state as ::core::ffi::c_int == IDLE as ::core::ffi::c_int
            && ((*eptr).lastread + CSSERV_TIMEOUT as ::core::ffi::c_double) < now
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_NOTICE,
                b"csserv: connection timed out\0".as_ptr() as *const ::core::ffi::c_char,
            );
            (*eptr).state = CLOSE as ::core::ffi::c_int as uint8_t;
        }
        eptr = (*eptr).next as *mut csserventry;
    }
    kptr = &raw mut csservhead;
    loop {
        eptr = *kptr;
        if eptr.is_null() {
            break;
        }
        if (*eptr).state as ::core::ffi::c_int == CLOSE as ::core::ffi::c_int {
            tcpclose((*eptr).sock);
            csserv_close(eptr);
            if !(*eptr).inputpacket.packet.is_null() {
                free((*eptr).inputpacket.packet as *mut ::core::ffi::c_void);
            }
            pptr = (*eptr).outputhead;
            while !pptr.is_null() {
                if !(*pptr).packet.is_null() {
                    free((*pptr).packet as *mut ::core::ffi::c_void);
                }
                paptr = pptr;
                pptr = (*pptr).next as *mut packetstruct;
                free(paptr as *mut ::core::ffi::c_void);
            }
            *kptr = (*eptr).next as *mut csserventry;
            free(eptr as *mut ::core::ffi::c_void);
        } else {
            kptr = &raw mut (*eptr).next as *mut *mut csserventry;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn csserv_getlistenip() -> uint32_t {
    return mylistenip;
}
#[no_mangle]
pub unsafe extern "C" fn csserv_getlistenport() -> uint16_t {
    return mylistenport;
}
#[no_mangle]
pub unsafe extern "C" fn csserv_reload() {
    let mut newListenHost: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut newListenPort: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut newmylistenip: uint32_t = 0;
    let mut newmylistenport: uint16_t = 0;
    let mut newlsock: ::core::ffi::c_int = 0;
    newmylistenip = 0 as uint32_t;
    newmylistenport = 0 as uint16_t;
    if lsock < 0 as ::core::ffi::c_int {
        return;
    }
    newListenHost = cfg_getstr(
        b"CSSERV_LISTEN_HOST\0".as_ptr() as *const ::core::ffi::c_char,
        b"*\0".as_ptr() as *const ::core::ffi::c_char,
    );
    newListenPort = cfg_getstr(
        b"CSSERV_LISTEN_PORT\0".as_ptr() as *const ::core::ffi::c_char,
        DEFAULT_CS_DATA_PORT.as_ptr(),
    );
    if strcmp(newListenHost, ListenHost) == 0 as ::core::ffi::c_int
        && strcmp(newListenPort, ListenPort) == 0 as ::core::ffi::c_int
    {
        free(newListenHost as *mut ::core::ffi::c_void);
        free(newListenPort as *mut ::core::ffi::c_void);
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_INFO,
            b"main server module: socket address hasn't changed (%s:%s)\0".as_ptr()
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
            b"main server module: socket address has changed, but can't create new socket\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
        free(newListenHost as *mut ::core::ffi::c_void);
        free(newListenPort as *mut ::core::ffi::c_void);
        return;
    }
    tcpnonblock(newlsock);
    tcpnodelay(newlsock);
    tcpreuseaddr(newlsock);
    tcpresolve(
        newListenHost,
        newListenPort,
        &raw mut newmylistenip,
        &raw mut newmylistenport,
        1 as ::core::ffi::c_int,
    );
    if tcpnumlisten(newlsock, newmylistenip, newmylistenport, 100 as uint16_t)
        < 0 as ::core::ffi::c_int
    {
        mfs_log(
            MFSLOG_ERRNO_SYSLOG_STDERR,
            MFSLOG_WARNING,
            b"main server module: socket address has changed, but can't listen on socket (%s:%s)\0"
                .as_ptr() as *const ::core::ffi::c_char,
            ListenHost,
            ListenPort,
        );
        free(newListenHost as *mut ::core::ffi::c_void);
        free(newListenPort as *mut ::core::ffi::c_void);
        tcpclose(newlsock);
        return;
    }
    if tcpsetacceptfilter(newlsock) < 0 as ::core::ffi::c_int && *__errno_location() != ENOTSUP {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_NOTICE,
            b"main server module: can't set accept filter\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    mfs_log(
        MFSLOG_SYSLOG_STDERR,
        MFSLOG_INFO,
        b"main server module: socket address has changed, now listen on %s:%s\0".as_ptr()
            as *const ::core::ffi::c_char,
        ListenHost,
        ListenPort,
    );
    free(ListenHost as *mut ::core::ffi::c_void);
    free(ListenPort as *mut ::core::ffi::c_void);
    ListenHost = newListenHost;
    ListenPort = newListenPort;
    tcpclose(lsock);
    lsock = newlsock;
    mylistenip = newmylistenip;
    mylistenport = newmylistenport;
    masterconn_forcereconnect();
}
#[no_mangle]
pub unsafe extern "C" fn csserv_init() -> ::core::ffi::c_int {
    ListenHost = cfg_getstr(
        b"CSSERV_LISTEN_HOST\0".as_ptr() as *const ::core::ffi::c_char,
        b"*\0".as_ptr() as *const ::core::ffi::c_char,
    );
    ListenPort = cfg_getstr(
        b"CSSERV_LISTEN_PORT\0".as_ptr() as *const ::core::ffi::c_char,
        DEFAULT_CS_DATA_PORT.as_ptr(),
    );
    lsock = tcpsocket();
    if lsock < 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_ERRNO_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"main server module: can't create socket\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    tcpnonblock(lsock);
    tcpnodelay(lsock);
    tcpreuseaddr(lsock);
    tcpresolve(
        ListenHost,
        ListenPort,
        &raw mut mylistenip,
        &raw mut mylistenport,
        1 as ::core::ffi::c_int,
    );
    if tcpnumlisten(lsock, mylistenip, mylistenport, 100 as uint16_t) < 0 as ::core::ffi::c_int {
        mfs_log(
            MFSLOG_ERRNO_SYSLOG_STDERR,
            MFSLOG_ERR,
            b"main server module: can't listen on socket\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -1 as ::core::ffi::c_int;
    }
    if tcpsetacceptfilter(lsock) < 0 as ::core::ffi::c_int && *__errno_location() != ENOTSUP {
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_NOTICE,
            b"main server module: can't set accept filter\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    mfs_log(
        MFSLOG_SYSLOG_STDERR,
        MFSLOG_INFO,
        b"main server module: listen on %s:%s\0".as_ptr() as *const ::core::ffi::c_char,
        ListenHost,
        ListenPort,
    );
    csservhead = ::core::ptr::null_mut::<csserventry>();
    main_wantexit_register_fname(
        Some(csserv_wantexit as unsafe extern "C" fn() -> ()),
        b"csserv_wantexit\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_reload_register_fname(
        Some(csserv_reload as unsafe extern "C" fn() -> ()),
        b"csserv_reload\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_destruct_register_fname(
        Some(csserv_term as unsafe extern "C" fn() -> ()),
        b"csserv_term\0".as_ptr() as *const ::core::ffi::c_char,
    );
    main_poll_register_fname(
        Some(csserv_desc as unsafe extern "C" fn(*mut pollfd, *mut uint32_t) -> ()),
        Some(csserv_serve as unsafe extern "C" fn(*mut pollfd) -> ()),
        b"csserv_desc\0".as_ptr() as *const ::core::ffi::c_char,
        b"csserv_serve\0".as_ptr() as *const ::core::ffi::c_char,
    );
    return 0 as ::core::ffi::c_int;
}
