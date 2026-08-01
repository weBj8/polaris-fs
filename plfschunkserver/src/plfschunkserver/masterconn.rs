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
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn read(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    unsafe fn write(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
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
    unsafe fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn cfg_isdefined(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn cfg_getstr(
        name: *const ::core::ffi::c_char,
        def: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn cfg_getuint32(name: *const ::core::ffi::c_char, def: uint32_t) -> uint32_t;
    unsafe fn main_destruct_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn main_wantexit_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn main_canexit_register_fname(
        fun: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
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
    unsafe fn main_eachloop_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
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
    unsafe fn main_exit();
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
    unsafe fn hdd_errorcounter() -> uint32_t;
    unsafe fn hdd_sendingchunks() -> uint8_t;
    unsafe fn hdd_chunk_status(chunkid: uint64_t, buff: *mut uint8_t) -> uint32_t;
    unsafe fn hdd_get_damaged_chunk_count() -> uint32_t;
    unsafe fn hdd_get_damaged_chunk_data(buff: *mut uint8_t);
    unsafe fn hdd_get_lost_chunk_count(limit: uint32_t) -> uint32_t;
    unsafe fn hdd_get_lost_chunk_data(buff: *mut uint8_t, limit: uint32_t);
    unsafe fn hdd_get_new_chunk_count(limit: uint32_t) -> uint32_t;
    unsafe fn hdd_get_new_chunk_data(buff: *mut uint8_t, limit: uint32_t);
    unsafe fn hdd_get_changed_chunk_count(limit: uint32_t) -> uint32_t;
    unsafe fn hdd_get_changed_chunk_data(buffl: *mut uint8_t, buffn: *mut uint8_t, limit: uint32_t);
    unsafe fn hdd_get_nonexistent_chunk_count(limit: uint32_t) -> uint32_t;
    unsafe fn hdd_get_nonexistent_chunk_data(buff: *mut uint8_t, limit: uint32_t);
    unsafe fn hdd_get_chunks_begin(partialmode: uint8_t);
    unsafe fn hdd_get_chunks_end();
    unsafe fn hdd_get_chunks_next_list_count(stopcount: uint32_t) -> uint32_t;
    unsafe fn hdd_get_chunks_next_list_data(stopcount: uint32_t, buff: *mut uint8_t);
    unsafe fn hdd_regfirst(chunkid: uint64_t);
    unsafe fn hdd_spacechanged() -> uint8_t;
    unsafe fn hdd_get_space(
        usedspace: *mut uint64_t,
        totalspace: *mut uint64_t,
        chunkcount: *mut uint32_t,
        tdusedspace: *mut uint64_t,
        tdtotalspace: *mut uint64_t,
        tdchunkcount: *mut uint32_t,
    );
    unsafe fn hdd_is_rebalance_on() -> uint8_t;
    unsafe fn hdd_setmetaid(metaid: uint64_t);
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn rndu32_ranged(range: uint32_t) -> uint32_t;
    unsafe fn job_get_load_and_hlstatus(load: *mut uint32_t, hlstatus: *mut uint8_t);
    unsafe fn job_pool_disable_job(jobid: uint32_t);
    unsafe fn job_inval(
        callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
        extra: *mut ::core::ffi::c_void,
    ) -> uint32_t;
    unsafe fn job_chunkop(
        callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
        extra: *mut ::core::ffi::c_void,
        chunkid: uint64_t,
        version: uint32_t,
        newversion: uint32_t,
        copychunkid: uint64_t,
        copyversion: uint32_t,
        length: uint32_t,
    ) -> uint32_t;
    unsafe fn job_replicate_simple(
        callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
        extra: *mut ::core::ffi::c_void,
        chunkid: uint64_t,
        version: uint32_t,
        srcip: uint32_t,
        srcport: uint16_t,
    ) -> uint32_t;
    unsafe fn job_replicate_split(
        callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
        extra: *mut ::core::ffi::c_void,
        chunkid: uint64_t,
        version: uint32_t,
        srcip: uint32_t,
        srcport: uint16_t,
        srcchunkid: uint64_t,
        partno: uint8_t,
        parts: uint8_t,
    ) -> uint32_t;
    unsafe fn job_replicate_recover(
        callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
        extra: *mut ::core::ffi::c_void,
        chunkid: uint64_t,
        version: uint32_t,
        parts: uint8_t,
        srcip: *mut uint32_t,
        srcport: *mut uint16_t,
        srcchunkid: *mut uint64_t,
    ) -> uint32_t;
    unsafe fn job_replicate_join(
        callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
        extra: *mut ::core::ffi::c_void,
        chunkid: uint64_t,
        version: uint32_t,
        parts: uint8_t,
        srcip: *mut uint32_t,
        srcport: *mut uint16_t,
        srcchunkid: *mut uint64_t,
    ) -> uint32_t;
    unsafe fn job_get_chunk_info(
        callback: Option<unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> ()>,
        extra: *mut ::core::ffi::c_void,
        chunkid: uint64_t,
        version: uint32_t,
        requested_info: uint8_t,
        info_buff: *mut uint8_t,
    ) -> uint32_t;
    unsafe fn busychunk_start(
        packet: *mut ::core::ffi::c_void,
        chunkid: uint64_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn busychunk_end(vbc: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    unsafe fn busychunk_isbusy(chunkid: uint64_t) -> uint8_t;
    unsafe fn busychunk_init();
    unsafe fn csserv_getlistenip() -> uint32_t;
    unsafe fn csserv_getlistenport() -> uint16_t;
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
    unsafe fn monotonic_useconds() -> uint64_t;
    unsafe fn md5_init(ctx: *mut md5ctx);
    unsafe fn md5_update(ctx: *mut md5ctx, buff: *const uint8_t, leng: uint32_t);
    unsafe fn md5_final(digest: *mut uint8_t, ctx: *mut md5ctx);
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
    pub masterversion: uint32_t,
    pub conncnt: uint32_t,
    pub bindip: uint32_t,
    pub masterip: uint32_t,
    pub masterport: uint16_t,
    pub timeout: uint16_t,
    pub masteraddrvalid: uint8_t,
    pub registerstate: uint8_t,
    pub gotrndblob: uint8_t,
    pub rndblob: [uint8_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct out_packetstruct {
    pub next: *mut out_packetstruct,
    pub startptr: *mut uint8_t,
    pub bytesleft: uint32_t,
    pub conncnt: uint32_t,
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
pub const DATA: C2Rust_Unnamed = 2;
pub const REGISTERED: C2Rust_Unnamed_1 = 3;
pub const CLOSE: C2Rust_Unnamed = 4;
pub const KILL: C2Rust_Unnamed = 3;
pub const CONNECTING: C2Rust_Unnamed = 1;
pub const FREE: C2Rust_Unnamed = 0;
pub const INPROGRESS: C2Rust_Unnamed_1 = 2;
pub const WAITING: C2Rust_Unnamed_1 = 1;
pub const UNREGISTERED: C2Rust_Unnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct idlejob {
    pub jobid: uint32_t,
    pub op: uint8_t,
    pub valid: uint8_t,
    pub chunkid: uint64_t,
    pub version: uint32_t,
    pub next: *mut idlejob,
    pub prev: *mut *mut idlejob,
    pub buff: [uint8_t; 1],
}
pub type md5ctx = _md5ctx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _md5ctx {
    pub state: [uint32_t; 4],
    pub count: [uint32_t; 2],
    pub buffer: [uint8_t; 64],
}
pub const IJ_GET_CHUNK_CHECKSUM_TAB: C2Rust_Unnamed_0 = 2;
pub const IJ_GET_CHUNK_CHECKSUM: C2Rust_Unnamed_0 = 1;
pub const IJ_GET_CHUNK_BLOCKS: C2Rust_Unnamed_0 = 0;
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub type C2Rust_Unnamed_0 = ::core::ffi::c_uint;
pub type C2Rust_Unnamed_1 = ::core::ffi::c_uint;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_TRUNC: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const MFSBLOCKSINCHUNK: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_NOTDONE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MAX_EC_PARTS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const HLSTATUS_OVERLOADED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const HLSTATUS_LSREBALANCE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const HLSTATUS_HSREBALANCE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const REQUEST_BLOCKS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const REQUEST_CHECKSUM: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const REQUEST_CHECKSUM_TAB: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MATOCS_MAXPACKETSIZE: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
pub const ANTOAN_NOP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ANTOAN_UNKNOWN_COMMAND: uint32_t = 1 as uint32_t;
pub const ANTOAN_BAD_COMMAND_SIZE: uint32_t = 2 as uint32_t;
pub const ANTOAN_FORCE_TIMEOUT: uint32_t = 5 as uint32_t;
pub const CSTOMA_CHUNK_DOESNT_EXIST: ::core::ffi::c_int = PROTO_BASE + 96 as ::core::ffi::c_int;
pub const MATOCS_CHUNK_STATUS: uint32_t = 97 as uint32_t;
pub const CSTOMA_CHUNK_STATUS: ::core::ffi::c_int = PROTO_BASE + 98 as ::core::ffi::c_int;
pub const MATOCS_REGISTER_FIRST: uint32_t = 99 as uint32_t;
pub const CSTOMA_REGISTER: ::core::ffi::c_int = PROTO_BASE + 100 as ::core::ffi::c_int;
pub const CSTOMA_SPACE: ::core::ffi::c_int = PROTO_BASE + 101 as ::core::ffi::c_int;
pub const CSTOMA_CHUNK_DAMAGED: ::core::ffi::c_int = PROTO_BASE + 102 as ::core::ffi::c_int;
pub const CSTOMA_CURRENT_LOAD: ::core::ffi::c_int = PROTO_BASE + 103 as ::core::ffi::c_int;
pub const MATOCS_MASTER_ACK: uint32_t = 104 as uint32_t;
pub const CSTOMA_CHUNK_LOST: ::core::ffi::c_int = PROTO_BASE + 105 as ::core::ffi::c_int;
pub const CSTOMA_ERROR_OCCURRED: ::core::ffi::c_int = PROTO_BASE + 106 as ::core::ffi::c_int;
pub const CSTOMA_CHUNK_NEW: ::core::ffi::c_int = PROTO_BASE + 107 as ::core::ffi::c_int;
pub const CSTOMA_LABELS: ::core::ffi::c_int = PROTO_BASE + 109 as ::core::ffi::c_int;
pub const MATOCS_CREATE: uint32_t = 110 as uint32_t;
pub const CSTOMA_CREATE: ::core::ffi::c_int = PROTO_BASE + 111 as ::core::ffi::c_int;
pub const MATOCS_DELETE: uint32_t = 120 as uint32_t;
pub const CSTOMA_DELETE: ::core::ffi::c_int = PROTO_BASE + 121 as ::core::ffi::c_int;
pub const MATOCS_DUPLICATE: uint32_t = 130 as uint32_t;
pub const CSTOMA_DUPLICATE: ::core::ffi::c_int = PROTO_BASE + 131 as ::core::ffi::c_int;
pub const MATOCS_SET_VERSION: uint32_t = 140 as uint32_t;
pub const CSTOMA_SET_VERSION: ::core::ffi::c_int = PROTO_BASE + 141 as ::core::ffi::c_int;
pub const MATOCS_REPLICATE: uint32_t = 150 as uint32_t;
pub const CSTOMA_REPLICATE: ::core::ffi::c_int = PROTO_BASE + 151 as ::core::ffi::c_int;
pub const MATOCS_CHUNKOP: uint32_t = 152 as uint32_t;
pub const CSTOMA_CHUNKOP: ::core::ffi::c_int = PROTO_BASE + 153 as ::core::ffi::c_int;
pub const MATOCS_REPLICATE_SPLIT: uint32_t = 154 as uint32_t;
pub const CSTOMA_REPLICATE_SPLIT: ::core::ffi::c_int = PROTO_BASE + 155 as ::core::ffi::c_int;
pub const MATOCS_REPLICATE_RECOVER: uint32_t = 156 as uint32_t;
pub const CSTOMA_REPLICATE_RECOVER: ::core::ffi::c_int = PROTO_BASE + 157 as ::core::ffi::c_int;
pub const MATOCS_REPLICATE_JOIN: uint32_t = 158 as uint32_t;
pub const CSTOMA_REPLICATE_JOIN: ::core::ffi::c_int = PROTO_BASE + 159 as ::core::ffi::c_int;
pub const MATOCS_TRUNCATE: uint32_t = 160 as uint32_t;
pub const CSTOMA_TRUNCATE: ::core::ffi::c_int = PROTO_BASE + 161 as ::core::ffi::c_int;
pub const MATOCS_DUPTRUNC: uint32_t = 170 as uint32_t;
pub const CSTOMA_DUPTRUNC: ::core::ffi::c_int = PROTO_BASE + 171 as ::core::ffi::c_int;
pub const MATOCS_LOCALSPLIT: uint32_t = 180 as uint32_t;
pub const CSTOMA_LOCALSPLIT: ::core::ffi::c_int = PROTO_BASE + 181 as ::core::ffi::c_int;
pub const ANTOCS_GET_CHUNK_BLOCKS: uint32_t = 250 as uint32_t;
pub const CSTOAN_CHUNK_BLOCKS: ::core::ffi::c_int = PROTO_BASE + 251 as ::core::ffi::c_int;
pub const ANTOCS_GET_CHUNK_CHECKSUM: uint32_t = 300 as uint32_t;
pub const CSTOAN_CHUNK_CHECKSUM: ::core::ffi::c_int = PROTO_BASE + 301 as ::core::ffi::c_int;
pub const ANTOCS_GET_CHUNK_CHECKSUM_TAB: uint32_t = 302 as uint32_t;
pub const CSTOAN_CHUNK_CHECKSUM_TAB: ::core::ffi::c_int = PROTO_BASE + 303 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const DEFAULT_MASTERNAME: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"mfsmaster\0") };
pub const DEFAULT_MASTER_CS_PORT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"9420\0") };
pub const PROTO_BASE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const VERSHEX: ::core::ffi::c_int = 4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
    + 59 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
    + 2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
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
pub const MaxPacketSize: ::core::ffi::c_int = MATOCS_MAXPACKETSIZE;
pub const LOSTCHUNKLIMIT: ::core::ffi::c_int = 25000 as ::core::ffi::c_int;
pub const NEWCHUNKLIMIT: ::core::ffi::c_int = 25000 as ::core::ffi::c_int;
pub const CHANGEDCHUNKLIMIT: ::core::ffi::c_int = 25000 as ::core::ffi::c_int;
pub const NONEXISTENTCHUNKLIMIT: ::core::ffi::c_int = 25000 as ::core::ffi::c_int;
pub const FORCE_DISCONNECTION_TO: ::core::ffi::c_double = 5.0f64;
static mut masterconnsingleton: *mut masterconn = ::core::ptr::null_mut::<masterconn>();
static mut idlejobs: *mut idlejob = ::core::ptr::null_mut::<idlejob>();
static mut csidvalid: uint8_t = 0 as uint8_t;
static mut reconnect_hook: *mut ::core::ffi::c_void =
    ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut manager_time_hook: *mut ::core::ffi::c_void =
    ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut wantexittime: ::core::ffi::c_double = 0.0f64;
static mut stats_bytesout: uint64_t = 0 as uint64_t;
static mut stats_bytesin: uint64_t = 0 as uint64_t;
static mut ChunksPerRegisterPacket: uint32_t = 0;
static mut MasterHost: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut MasterPort: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut BindHost: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut Timeout: uint32_t = 0;
static mut ChunkServerID: uint16_t = 0 as uint16_t;
static mut MetaID: uint64_t = 0 as uint64_t;
static mut AuthCode: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut LabelMask: uint32_t = 0 as uint32_t;
static mut hddmetaid: uint64_t = 0;
static mut reconnectisneeded: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_stats(mut bin: *mut uint64_t, mut bout: *mut uint64_t) {
    unsafe {
        *bin = stats_bytesin;
        *bout = stats_bytesout;
        stats_bytesin = 0 as uint64_t;
        stats_bytesout = 0 as uint64_t;
    }
}
#[inline]
unsafe extern "C" fn masterconn_initcsid() {
    unsafe {
        let mut fd: ::core::ffi::c_int = 0;
        let mut buff: [uint8_t; 10] = [0; 10];
        let mut rptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut ret: ssize_t = 0;
        if csidvalid != 0 {
            return;
        }
        hddmetaid = 0 as uint64_t;
        ChunkServerID = 0 as uint16_t;
        MetaID = 0 as uint64_t;
        csidvalid = 1 as uint8_t;
        fd = open(
            b"chunkserverid.mfs\0".as_ptr() as *const ::core::ffi::c_char,
            O_RDWR,
        );
        if fd >= 0 as ::core::ffi::c_int {
            ret = read(
                fd,
                &raw mut buff as *mut uint8_t as *mut ::core::ffi::c_void,
                10 as size_t,
            );
            rptr = &raw mut buff as *mut uint8_t;
            if ret >= 2 as ssize_t {
                ChunkServerID = get16bit(&raw mut rptr);
            }
            if ret >= 10 as ssize_t {
                MetaID = get64bit(&raw mut rptr);
            }
            close(fd);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_getcsid() -> uint16_t {
    unsafe {
        masterconn_initcsid();
        return ChunkServerID;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_getmetaid() -> uint64_t {
    unsafe {
        masterconn_initcsid();
        return MetaID;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_gethddmetaid() -> uint64_t {
    unsafe {
        return hddmetaid;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_sethddmetaid(mut metaid: uint64_t) {
    unsafe {
        hddmetaid = metaid;
    }
}
#[inline]
unsafe extern "C" fn masterconn_setcsid(mut csid: uint16_t, mut metaid: uint64_t) {
    unsafe {
        let mut fd: ::core::ffi::c_int = 0;
        let mut buff: [uint8_t; 10] = [0; 10];
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if ChunkServerID as ::core::ffi::c_int != csid as ::core::ffi::c_int || MetaID != metaid {
            if csid as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                ChunkServerID = csid;
            }
            if metaid > 0 as uint64_t {
                MetaID = metaid;
            }
            wptr = &raw mut buff as *mut uint8_t;
            put16bit(&raw mut wptr, ChunkServerID);
            put64bit(&raw mut wptr, MetaID);
            fd = open(
                b"chunkserverid.mfs\0".as_ptr() as *const ::core::ffi::c_char,
                O_CREAT | O_TRUNC | O_RDWR,
                0o666 as ::core::ffi::c_int,
            );
            if fd >= 0 as ::core::ffi::c_int {
                if write(
                    fd,
                    &raw mut buff as *mut uint8_t as *const ::core::ffi::c_void,
                    10 as size_t,
                ) != 10 as ssize_t
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"can't store chunkserver id (write error)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                close(fd);
            } else {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"can't store chunkserver id (open error)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            hdd_setmetaid(MetaID);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_getmasterip() -> uint32_t {
    unsafe {
        let mut eptr: *mut masterconn = masterconnsingleton;
        if (*eptr).registerstate as ::core::ffi::c_int == REGISTERED as ::core::ffi::c_int
            && (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
        {
            return (*eptr).masterip;
        }
        return 0 as uint32_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_getmasterport() -> uint16_t {
    unsafe {
        let mut eptr: *mut masterconn = masterconnsingleton;
        if (*eptr).registerstate as ::core::ffi::c_int == REGISTERED as ::core::ffi::c_int
            && (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
        {
            return (*eptr).masterport;
        }
        return 0 as uint16_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_create_detached_packet(
    mut eptr: *mut masterconn,
    mut r#type: uint32_t,
    mut size: uint32_t,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut outpacket: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut psize: uint32_t = 0;
        psize = size.wrapping_add(8 as uint32_t);
        outpacket = malloc((24 as size_t).wrapping_add(psize as size_t)) as *mut out_packetstruct;
        if outpacket.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                264 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"outpacket\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                264 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfschunkserver/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                264 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"outpacket\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                264 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
        (*outpacket).conncnt = (*eptr).conncnt;
        return outpacket as *mut ::core::ffi::c_void;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_get_packet_data(
    mut packet: *mut ::core::ffi::c_void,
) -> *mut uint8_t {
    unsafe {
        let mut outpacket: *mut out_packetstruct = packet as *mut out_packetstruct;
        return (&raw mut (*outpacket).data as *mut uint8_t)
            .offset(8 as ::core::ffi::c_int as isize);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_delete_packet(mut packet: *mut ::core::ffi::c_void) {
    unsafe {
        free(packet);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_attach_packet(
    mut eptr: *mut masterconn,
    mut packet: *mut ::core::ffi::c_void,
) {
    unsafe {
        let mut outpacket: *mut out_packetstruct = packet as *mut out_packetstruct;
        *(*eptr).outputtail = outpacket;
        (*eptr).outputtail = &raw mut (*outpacket).next as *mut *mut out_packetstruct;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_create_attached_packet(
    mut eptr: *mut masterconn,
    mut r#type: uint32_t,
    mut size: uint32_t,
) -> *mut uint8_t {
    unsafe {
        let mut outpacket: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut psize: uint32_t = 0;
        psize = size.wrapping_add(8 as uint32_t);
        outpacket = malloc((24 as size_t).wrapping_add(psize as size_t)) as *mut out_packetstruct;
        if outpacket.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                300 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"outpacket\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                300 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfschunkserver/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                300 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"outpacket\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                300 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
pub unsafe extern "C" fn masterconn_parselabels() -> uint8_t {
    unsafe {
        let mut labelsstr: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut c: ::core::ffi::c_char = 0;
        let mut mask: uint32_t = 0;
        let mut sep: uint8_t = 0;
        let mut perr: uint8_t = 0;
        let mut newlabelmask: uint32_t = 0;
        labelsstr = cfg_getstr(
            b"LABELS\0".as_ptr() as *const ::core::ffi::c_char,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
        );
        newlabelmask = 0 as uint32_t;
        perr = 0 as uint8_t;
        sep = 0 as uint8_t;
        p = labelsstr;
        while *p != 0 {
            c = *p;
            if c as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
            {
                mask = ((1 as ::core::ffi::c_int)
                    << c as ::core::ffi::c_int - 'A' as ::core::ffi::c_int)
                    as uint32_t;
            } else if c as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int
            {
                mask = ((1 as ::core::ffi::c_int)
                    << c as ::core::ffi::c_int - 'a' as ::core::ffi::c_int)
                    as uint32_t;
            } else {
                mask = 0 as uint32_t;
            }
            if mask != 0 {
                if sep != 0 {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"LABELS: separator not found before label %c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        c as ::core::ffi::c_int,
                    );
                    perr = 1 as uint8_t;
                } else {
                    sep = 1 as uint8_t;
                }
                if newlabelmask & mask != 0 {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"LABELS: found duplicate label %c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        c as ::core::ffi::c_int,
                    );
                    perr = 1 as uint8_t;
                }
                newlabelmask |= mask;
            } else if c as ::core::ffi::c_int == ',' as ::core::ffi::c_int
                || c as ::core::ffi::c_int == ';' as ::core::ffi::c_int
            {
                if sep != 0 {
                    sep = 0 as uint8_t;
                } else {
                    if newlabelmask != 0 as uint32_t {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"LABELS: more than one separator found\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"LABELS: found separator at the beginning of definition\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                    perr = 1 as uint8_t;
                }
            } else if c as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
                && c as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"LABELS: unrecognized character %c\0".as_ptr() as *const ::core::ffi::c_char,
                    c as ::core::ffi::c_int,
                );
                perr = 1 as uint8_t;
            }
            p = p.offset(1);
        }
        if sep as ::core::ffi::c_int == 0 as ::core::ffi::c_int && newlabelmask != 0 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"LABELS: found separator at the end of definition\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            perr = 1 as uint8_t;
        }
        if perr != 0 {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_NOTICE,
                b"in the current version of chunkserver the only correct LABELS format is a set of letters separated by ',' or ';' - please change your config file appropriately\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
        free(labelsstr as *mut ::core::ffi::c_void);
        if newlabelmask != LabelMask {
            LabelMask = newlabelmask;
            return 1 as uint8_t;
        }
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_sendlabels(mut eptr: *mut masterconn) {
    unsafe {
        let mut buff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        buff = masterconn_create_attached_packet(eptr, CSTOMA_LABELS as uint32_t, 4 as uint32_t);
        put32bit(&raw mut buff, LabelMask);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_sendregister(mut eptr: *mut masterconn) {
    unsafe {
        let mut buff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut myip: uint32_t = 0;
        let mut myport: uint16_t = 0;
        let mut usedspace: uint64_t = 0;
        let mut totalspace: uint64_t = 0;
        let mut tdusedspace: uint64_t = 0;
        let mut tdtotalspace: uint64_t = 0;
        let mut chunkcount: uint32_t = 0;
        let mut tdchunkcount: uint32_t = 0;
        myip = csserv_getlistenip();
        myport = csserv_getlistenport();
        hdd_get_space(
            &raw mut usedspace,
            &raw mut totalspace,
            &raw mut chunkcount,
            &raw mut tdusedspace,
            &raw mut tdtotalspace,
            &raw mut tdchunkcount,
        );
        if (*eptr).gotrndblob as ::core::ffi::c_int != 0 && !AuthCode.is_null() {
            let mut md5c: md5ctx = md5ctx {
                state: [0; 4],
                count: [0; 2],
                buffer: [0; 64],
            };
            buff = masterconn_create_attached_packet(
                eptr,
                CSTOMA_REGISTER as uint32_t,
                (1 as ::core::ffi::c_int
                    + 16 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int) as uint32_t,
            );
            put8bit(&raw mut buff, 60 as uint8_t);
            md5_init(&raw mut md5c);
            md5_update(
                &raw mut md5c,
                &raw mut (*eptr).rndblob as *mut uint8_t,
                16 as uint32_t,
            );
            md5_update(
                &raw mut md5c,
                AuthCode as *const uint8_t,
                strlen(AuthCode) as uint32_t,
            );
            md5_update(
                &raw mut md5c,
                (&raw mut (*eptr).rndblob as *mut uint8_t)
                    .offset(16 as ::core::ffi::c_int as isize),
                16 as uint32_t,
            );
            md5_final(buff as *mut uint8_t, &raw mut md5c);
            buff = buff.offset(16 as ::core::ffi::c_int as isize);
        } else {
            buff = masterconn_create_attached_packet(
                eptr,
                CSTOMA_REGISTER as uint32_t,
                (1 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int) as uint32_t,
            );
            put8bit(&raw mut buff, 60 as uint8_t);
        }
        put32bit(&raw mut buff, VERSHEX as uint32_t);
        put32bit(&raw mut buff, myip);
        put16bit(&raw mut buff, myport);
        put16bit(&raw mut buff, Timeout as uint16_t);
        put16bit(&raw mut buff, masterconn_getcsid());
        put64bit(&raw mut buff, usedspace);
        put64bit(&raw mut buff, totalspace);
        put32bit(&raw mut buff, chunkcount);
        put64bit(&raw mut buff, tdusedspace);
        put64bit(&raw mut buff, tdtotalspace);
        put32bit(&raw mut buff, tdchunkcount);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_sendnextchunks(mut eptr: *mut masterconn) {
    unsafe {
        let mut buff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut chunks: uint32_t = 0;
        chunks = hdd_get_chunks_next_list_count(ChunksPerRegisterPacket);
        if chunks == 0 as uint32_t {
            hdd_get_chunks_end();
            buff =
                masterconn_create_attached_packet(eptr, CSTOMA_REGISTER as uint32_t, 1 as uint32_t);
            put8bit(&raw mut buff, 62 as uint8_t);
            (*eptr).registerstate = REGISTERED as ::core::ffi::c_int as uint8_t;
        } else {
            buff =
                masterconn_create_attached_packet(
                    eptr,
                    CSTOMA_REGISTER as uint32_t,
                    (1 as uint32_t).wrapping_add(chunks.wrapping_mul(
                        (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t,
                    )),
                );
            put8bit(&raw mut buff, 61 as uint8_t);
            hdd_get_chunks_next_list_data(ChunksPerRegisterPacket, buff);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_register_first(
    mut eptr: *mut masterconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        if length != 8 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_REGISTER_FIRST - wrong size (%u/8)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        chunkid = get64bit(&raw mut data);
        if (*eptr).registerstate as ::core::ffi::c_int != REGISTERED as ::core::ffi::c_int {
            hdd_regfirst(chunkid);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_master_ack(
    mut eptr: *mut masterconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut atype: uint8_t = 0;
        let mut metaid: uint64_t = 0;
        let mut csid: uint16_t = 0;
        if length != 33 as uint32_t
            && length != 17 as uint32_t
            && length != 15 as uint32_t
            && length != 9 as uint32_t
            && length != 7 as uint32_t
            && length != 5 as uint32_t
            && length != 1 as uint32_t
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_MASTER_ACK - wrong size (%u/1|5|7|9|15|17|33)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        atype = get8bit(&raw mut data);
        if atype as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            csid = 0 as uint16_t;
            metaid = 0 as uint64_t;
            if length >= 5 as uint32_t {
                (*eptr).masterversion = get32bit(&raw mut data);
            }
            if length >= 9 as uint32_t {
                if Timeout == 0 as uint32_t {
                    (*eptr).timeout = get16bit(&raw mut data);
                } else {
                    data = data.offset(2 as ::core::ffi::c_int as isize);
                }
                csid = get16bit(&raw mut data);
            }
            if length >= 17 as uint32_t {
                metaid = get64bit(&raw mut data);
                if metaid > 0 as uint64_t && MetaID > 0 as uint64_t && metaid != MetaID {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"MATOCS_MASTER_ACK - wrong meta data id (file chunkserverid.mfs:%016lX ; received from master:%016lX). Can't connect to master\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        MetaID,
                        metaid,
                    );
                    (*eptr).registerstate = REGISTERED as ::core::ffi::c_int as uint8_t;
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    main_exit();
                    return;
                }
                if metaid > 0 as uint64_t
                    && MetaID == 0 as uint64_t
                    && hddmetaid > 0 as uint64_t
                    && metaid != hddmetaid
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"MATOCS_MASTER_ACK - wrong meta data id (files .metaid:%016lX ; received from master:%016lX). Can't connect to master\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        hddmetaid,
                        metaid,
                    );
                    (*eptr).registerstate = REGISTERED as ::core::ffi::c_int as uint8_t;
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    main_exit();
                    return;
                }
            }
            if csid as ::core::ffi::c_int > 0 as ::core::ffi::c_int || metaid > 0 as uint64_t {
                masterconn_setcsid(csid, metaid);
            }
            if (*eptr).masterversion
                < (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"MATOCS_MASTER_ACK - unsupported master version\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                main_exit();
                return;
            } else {
                if (*eptr).registerstate as ::core::ffi::c_int == UNREGISTERED as ::core::ffi::c_int
                    || (*eptr).registerstate as ::core::ffi::c_int == WAITING as ::core::ffi::c_int
                {
                    hdd_get_chunks_begin(1 as uint8_t);
                    (*eptr).registerstate = INPROGRESS as ::core::ffi::c_int as uint8_t;
                    if (*eptr).masterversion
                        >= (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        masterconn_sendlabels(eptr);
                    }
                }
                if (*eptr).registerstate as ::core::ffi::c_int == INPROGRESS as ::core::ffi::c_int {
                    masterconn_sendnextchunks(eptr);
                }
            }
        } else if atype as ::core::ffi::c_int == 1 as ::core::ffi::c_int && length == 5 as uint32_t
        {
            (*eptr).masteraddrvalid = 0 as uint8_t;
            (*eptr).mode = CLOSE as ::core::ffi::c_int as uint8_t;
        } else if atype as ::core::ffi::c_int == 2 as ::core::ffi::c_int
            && (length == 7 as uint32_t || length == 15 as uint32_t)
        {
            if (*eptr).registerstate as ::core::ffi::c_int == INPROGRESS as ::core::ffi::c_int {
                hdd_get_chunks_end();
            }
            (*eptr).registerstate = WAITING as ::core::ffi::c_int as uint8_t;
            (*eptr).masterversion = get32bit(&raw mut data);
            if Timeout == 0 as uint32_t {
                (*eptr).timeout = get16bit(&raw mut data);
            } else {
                data = data.offset(2 as ::core::ffi::c_int as isize);
            }
            if length >= 15 as uint32_t {
                metaid = get64bit(&raw mut data);
                if metaid > 0 as uint64_t && MetaID > 0 as uint64_t && metaid != MetaID {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"MATOCS_MASTER_ACK - wrong meta data id. Can't connect to master\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    (*eptr).registerstate = REGISTERED as ::core::ffi::c_int as uint8_t;
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
            }
        } else if atype as ::core::ffi::c_int == 3 as ::core::ffi::c_int && length == 33 as uint32_t
        {
            if AuthCode.is_null() {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"MATOCS_MASTER_ACK - master needs authorization, but password was not defined\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                (*eptr).registerstate = REGISTERED as ::core::ffi::c_int as uint8_t;
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            memcpy(
                &raw mut (*eptr).rndblob as *mut uint8_t as *mut ::core::ffi::c_void,
                data as *const ::core::ffi::c_void,
                32 as size_t,
            );
            (*eptr).gotrndblob = 1 as uint8_t;
            masterconn_sendregister(eptr);
        } else {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_MASTER_ACK - bad type/length: %u/%u\0".as_ptr()
                    as *const ::core::ffi::c_char,
                atype as ::core::ffi::c_int,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_send_disconnect_command() {
    unsafe {
        let mut eptr: *mut masterconn = masterconnsingleton;
        let mut buff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if (*eptr).registerstate as ::core::ffi::c_int == REGISTERED as ::core::ffi::c_int
            && (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
            && (*eptr).masterversion
                >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        75 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        75 as ::core::ffi::c_int
                    })) as uint32_t
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_INFO,
                b"sending unregister command ...\0".as_ptr() as *const ::core::ffi::c_char,
            );
            buff =
                masterconn_create_attached_packet(eptr, CSTOMA_REGISTER as uint32_t, 1 as uint32_t);
            put8bit(&raw mut buff, 63 as uint8_t);
            (*eptr).mode = CLOSE as ::core::ffi::c_int as uint8_t;
        } else if (*eptr).mode as ::core::ffi::c_int != FREE as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_INFO,
                b"killing master connection\0".as_ptr() as *const ::core::ffi::c_char,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_check_hdd_space() {
    unsafe {
        let mut eptr: *mut masterconn = masterconnsingleton;
        let mut buff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if ((*eptr).registerstate as ::core::ffi::c_int == REGISTERED as ::core::ffi::c_int
            || (*eptr).registerstate as ::core::ffi::c_int == INPROGRESS as ::core::ffi::c_int)
            && (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
        {
            if hdd_spacechanged() != 0 {
                let mut usedspace: uint64_t = 0;
                let mut totalspace: uint64_t = 0;
                let mut tdusedspace: uint64_t = 0;
                let mut tdtotalspace: uint64_t = 0;
                let mut chunkcount: uint32_t = 0;
                let mut tdchunkcount: uint32_t = 0;
                buff = masterconn_create_attached_packet(
                    eptr,
                    CSTOMA_SPACE as uint32_t,
                    (8 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int) as uint32_t,
                );
                hdd_get_space(
                    &raw mut usedspace,
                    &raw mut totalspace,
                    &raw mut chunkcount,
                    &raw mut tdusedspace,
                    &raw mut tdtotalspace,
                    &raw mut tdchunkcount,
                );
                put64bit(&raw mut buff, usedspace);
                put64bit(&raw mut buff, totalspace);
                put32bit(&raw mut buff, chunkcount);
                put64bit(&raw mut buff, tdusedspace);
                put64bit(&raw mut buff, tdtotalspace);
                put32bit(&raw mut buff, tdchunkcount);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_check_hdd_reports() {
    unsafe {
        let mut eptr: *mut masterconn = masterconnsingleton;
        let mut errorcounter: uint32_t = 0;
        let mut chunkcounter: uint32_t = 0;
        let mut buffl: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut buffn: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut buffd: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if reconnectisneeded != 0 {
            masterconn_send_disconnect_command();
            (*eptr).masteraddrvalid = 0 as uint8_t;
            reconnectisneeded = 0 as ::core::ffi::c_int;
        }
        if (*eptr).registerstate as ::core::ffi::c_int == REGISTERED as ::core::ffi::c_int
            && (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
        {
            errorcounter = hdd_errorcounter();
            while errorcounter != 0 {
                masterconn_create_attached_packet(
                    eptr,
                    CSTOMA_ERROR_OCCURRED as uint32_t,
                    0 as uint32_t,
                );
                errorcounter = errorcounter.wrapping_sub(1);
            }
            chunkcounter = hdd_get_damaged_chunk_count();
            if chunkcounter != 0 {
                buffd = masterconn_create_attached_packet(
                    eptr,
                    CSTOMA_CHUNK_DAMAGED as uint32_t,
                    (8 as uint32_t).wrapping_mul(chunkcounter),
                );
                hdd_get_damaged_chunk_data(buffd);
            } else {
                hdd_get_damaged_chunk_data(::core::ptr::null_mut::<uint8_t>());
            }
            chunkcounter = hdd_get_lost_chunk_count(LOSTCHUNKLIMIT as uint32_t);
            if chunkcounter != 0 {
                buffl = masterconn_create_attached_packet(
                    eptr,
                    CSTOMA_CHUNK_LOST as uint32_t,
                    (8 as uint32_t).wrapping_mul(chunkcounter),
                );
                hdd_get_lost_chunk_data(buffl, LOSTCHUNKLIMIT as uint32_t);
            } else {
                hdd_get_lost_chunk_data(::core::ptr::null_mut::<uint8_t>(), 0 as uint32_t);
            }
            chunkcounter = hdd_get_new_chunk_count(NEWCHUNKLIMIT as uint32_t);
            if chunkcounter != 0 {
                buffn = masterconn_create_attached_packet(
                    eptr,
                    CSTOMA_CHUNK_NEW as uint32_t,
                    (12 as uint32_t).wrapping_mul(chunkcounter),
                );
                hdd_get_new_chunk_data(buffn, NEWCHUNKLIMIT as uint32_t);
            } else {
                hdd_get_new_chunk_data(::core::ptr::null_mut::<uint8_t>(), 0 as uint32_t);
            }
            chunkcounter = hdd_get_changed_chunk_count(CHANGEDCHUNKLIMIT as uint32_t);
            if chunkcounter != 0 {
                buffl = masterconn_create_attached_packet(
                    eptr,
                    CSTOMA_CHUNK_LOST as uint32_t,
                    (8 as uint32_t).wrapping_mul(chunkcounter),
                );
                buffn = masterconn_create_attached_packet(
                    eptr,
                    CSTOMA_CHUNK_NEW as uint32_t,
                    (12 as uint32_t).wrapping_mul(chunkcounter),
                );
                hdd_get_changed_chunk_data(buffl, buffn, CHANGEDCHUNKLIMIT as uint32_t);
            } else {
                hdd_get_changed_chunk_data(
                    ::core::ptr::null_mut::<uint8_t>(),
                    ::core::ptr::null_mut::<uint8_t>(),
                    0 as uint32_t,
                );
            }
            chunkcounter = hdd_get_nonexistent_chunk_count(NONEXISTENTCHUNKLIMIT as uint32_t);
            if chunkcounter != 0 {
                if (*eptr).masterversion
                    >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 32 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })) as uint32_t
                {
                    buffl = masterconn_create_attached_packet(
                        eptr,
                        CSTOMA_CHUNK_DOESNT_EXIST as uint32_t,
                        (8 as uint32_t).wrapping_mul(chunkcounter),
                    );
                } else {
                    buffl = ::core::ptr::null_mut::<uint8_t>();
                }
                hdd_get_nonexistent_chunk_data(buffl, NONEXISTENTCHUNKLIMIT as uint32_t);
            } else {
                hdd_get_nonexistent_chunk_data(::core::ptr::null_mut::<uint8_t>(), 0 as uint32_t);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_reportload() {
    unsafe {
        let mut eptr: *mut masterconn = masterconnsingleton;
        let mut load: uint32_t = 0;
        let mut hltosend: uint8_t = 0;
        let mut rebalance: uint8_t = 0;
        let mut buff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
            && (*eptr).masterversion
                >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 6 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        28 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        28 as ::core::ffi::c_int
                    })) as uint32_t
            && (*eptr).registerstate as ::core::ffi::c_int == REGISTERED as ::core::ffi::c_int
        {
            job_get_load_and_hlstatus(&raw mut load, &raw mut hltosend);
            if (*eptr).masterversion
                >= (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        7 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        7 as ::core::ffi::c_int
                    })) as uint32_t
            {
                rebalance = hdd_is_rebalance_on();
                if rebalance as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0 {
                    hltosend = HLSTATUS_HSREBALANCE as uint8_t;
                }
                if hltosend as ::core::ffi::c_int != HLSTATUS_OVERLOADED
                    && hltosend as ::core::ffi::c_int != HLSTATUS_HSREBALANCE
                    && rebalance as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0
                {
                    hltosend = HLSTATUS_LSREBALANCE as uint8_t;
                }
                if (*eptr).masterversion
                    < (3 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 3 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            62 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            62 as ::core::ffi::c_int
                        })) as uint32_t
                    && hltosend as ::core::ffi::c_int == HLSTATUS_LSREBALANCE
                {
                    hltosend = HLSTATUS_OVERLOADED as uint8_t;
                }
                if (*eptr).masterversion
                    < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 37 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })) as uint32_t
                    && hltosend as ::core::ffi::c_int == HLSTATUS_HSREBALANCE
                {
                    hltosend = HLSTATUS_OVERLOADED as uint8_t;
                }
                if (*eptr).masterversion
                    >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 32 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })) as uint32_t
                {
                    buff = masterconn_create_attached_packet(
                        eptr,
                        CSTOMA_CURRENT_LOAD as uint32_t,
                        6 as uint32_t,
                    );
                } else {
                    buff = masterconn_create_attached_packet(
                        eptr,
                        CSTOMA_CURRENT_LOAD as uint32_t,
                        5 as uint32_t,
                    );
                }
                put32bit(&raw mut buff, load);
                put8bit(&raw mut buff, hltosend);
                if (*eptr).masterversion
                    >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 32 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })) as uint32_t
                {
                    put8bit(&raw mut buff, hdd_sendingchunks());
                }
            } else {
                buff = masterconn_create_attached_packet(
                    eptr,
                    CSTOMA_CURRENT_LOAD as uint32_t,
                    4 as uint32_t,
                );
                put32bit(&raw mut buff, load);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_chunk_status(
    mut eptr: *mut masterconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut size: uint32_t = 0;
        let mut buff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if length != 8 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_CHUNK_STATUS - wrong size (%u/8)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        chunkid = get64bit(&raw mut data);
        if busychunk_isbusy(chunkid) != 0 {
            return;
        }
        size = hdd_chunk_status(chunkid, ::core::ptr::null_mut::<uint8_t>());
        buff = masterconn_create_attached_packet(eptr, CSTOMA_CHUNK_STATUS as uint32_t, size);
        hdd_chunk_status(chunkid, buff);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_jobfinished(
    mut status: uint8_t,
    mut bc: *mut ::core::ffi::c_void,
) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut eptr: *mut masterconn = masterconnsingleton;
        let mut packet: *mut ::core::ffi::c_void = busychunk_end(bc);
        if !eptr.is_null()
            && (*eptr).conncnt == (*(packet as *mut out_packetstruct)).conncnt
            && (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
        {
            ptr = masterconn_get_packet_data(packet);
            *ptr.offset(8 as isize) = status;
            masterconn_attach_packet(eptr, packet);
        } else {
            masterconn_delete_packet(packet);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_localsplitfinished(
    mut status: uint8_t,
    mut bc: *mut ::core::ffi::c_void,
) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut eptr: *mut masterconn = masterconnsingleton;
        let mut packet: *mut ::core::ffi::c_void = busychunk_end(bc);
        if !eptr.is_null()
            && (*eptr).conncnt == (*(packet as *mut out_packetstruct)).conncnt
            && (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
        {
            ptr = masterconn_get_packet_data(packet);
            *ptr.offset(12 as isize) = status;
            masterconn_attach_packet(eptr, packet);
        } else {
            masterconn_delete_packet(packet);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_chunkopfinished(
    mut status: uint8_t,
    mut bc: *mut ::core::ffi::c_void,
) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut eptr: *mut masterconn = masterconnsingleton;
        let mut packet: *mut ::core::ffi::c_void = busychunk_end(bc);
        if !eptr.is_null()
            && (*eptr).conncnt == (*(packet as *mut out_packetstruct)).conncnt
            && (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
        {
            ptr = masterconn_get_packet_data(packet);
            *ptr.offset(32 as isize) = status;
            masterconn_attach_packet(eptr, packet);
        } else {
            masterconn_delete_packet(packet);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_replicationfinished(
    mut status: uint8_t,
    mut bc: *mut ::core::ffi::c_void,
) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut eptr: *mut masterconn = masterconnsingleton;
        let mut packet: *mut ::core::ffi::c_void = busychunk_end(bc);
        if !eptr.is_null()
            && (*eptr).conncnt == (*(packet as *mut out_packetstruct)).conncnt
            && (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
        {
            ptr = masterconn_get_packet_data(packet);
            *ptr.offset(12 as isize) = status;
            masterconn_attach_packet(eptr, packet);
        } else {
            masterconn_delete_packet(packet);
        };
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
pub unsafe extern "C" fn masterconn_create(
    mut eptr: *mut masterconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut packet: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if length != (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_CREATE - wrong size (%u/12)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        chunkid = get64bit(&raw mut data);
        version = get32bit(&raw mut data);
        packet = masterconn_create_detached_packet(
            eptr,
            CSTOMA_CREATE as uint32_t,
            (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t,
        );
        ptr = masterconn_get_packet_data(packet);
        put64bit(&raw mut ptr, chunkid);
        job_chunkop(
            Some(
                masterconn_jobfinished
                    as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
            ),
            busychunk_start(packet, chunkid),
            chunkid,
            version,
            0 as uint32_t,
            0 as uint64_t,
            0 as uint32_t,
            1 as uint32_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_delete(
    mut eptr: *mut masterconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut packet: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if length != (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_DELETE - wrong size (%u/12)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        chunkid = get64bit(&raw mut data);
        version = get32bit(&raw mut data);
        packet = masterconn_create_detached_packet(
            eptr,
            CSTOMA_DELETE as uint32_t,
            (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t,
        );
        ptr = masterconn_get_packet_data(packet);
        put64bit(&raw mut ptr, chunkid);
        if (*eptr).registerstate as ::core::ffi::c_int == REGISTERED as ::core::ffi::c_int {
            job_chunkop(
                Some(
                    masterconn_jobfinished
                        as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
                ),
                busychunk_start(packet, chunkid),
                chunkid,
                version,
                0 as uint32_t,
                0 as uint64_t,
                0 as uint32_t,
                0 as uint32_t,
            );
        } else {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_DELETE - got command while still registering\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            put8bit(&raw mut ptr, MFS_ERROR_NOTDONE as uint8_t);
            masterconn_attach_packet(eptr, packet);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_setversion(
    mut eptr: *mut masterconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut newversion: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut packet: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if length
            != (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                as uint32_t
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_SET_VERSION - wrong size (%u/16)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        chunkid = get64bit(&raw mut data);
        newversion = get32bit(&raw mut data);
        version = get32bit(&raw mut data);
        packet = masterconn_create_detached_packet(
            eptr,
            CSTOMA_SET_VERSION as uint32_t,
            (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t,
        );
        ptr = masterconn_get_packet_data(packet);
        put64bit(&raw mut ptr, chunkid);
        if newversion > 0 as uint32_t {
            job_chunkop(
                Some(
                    masterconn_jobfinished
                        as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
                ),
                busychunk_start(packet, chunkid),
                chunkid,
                version,
                newversion,
                0 as uint64_t,
                0 as uint32_t,
                0xffffffff as uint32_t,
            );
        } else {
            job_inval(
                Some(
                    masterconn_jobfinished
                        as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
                ),
                busychunk_start(packet, chunkid),
            );
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_duplicate(
    mut eptr: *mut masterconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut copychunkid: uint64_t = 0;
        let mut copyversion: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut packet: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if length
            != (8 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int) as uint32_t
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_DUPLICATE - wrong size (%u/24)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        copychunkid = get64bit(&raw mut data);
        copyversion = get32bit(&raw mut data);
        chunkid = get64bit(&raw mut data);
        version = get32bit(&raw mut data);
        packet = masterconn_create_detached_packet(
            eptr,
            CSTOMA_DUPLICATE as uint32_t,
            (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t,
        );
        ptr = masterconn_get_packet_data(packet);
        put64bit(&raw mut ptr, copychunkid);
        if version > 0 as uint32_t && copychunkid > 0 as uint64_t {
            job_chunkop(
                Some(
                    masterconn_jobfinished
                        as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
                ),
                busychunk_start(packet, chunkid),
                chunkid,
                version,
                version,
                copychunkid,
                copyversion,
                0xffffffff as uint32_t,
            );
        } else {
            job_inval(
                Some(
                    masterconn_jobfinished
                        as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
                ),
                busychunk_start(packet, chunkid),
            );
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_truncate(
    mut eptr: *mut masterconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut leng: uint32_t = 0;
        let mut newversion: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut packet: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if length
            != (8 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int) as uint32_t
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_TRUNCATE - wrong size (%u/20)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        chunkid = get64bit(&raw mut data);
        leng = get32bit(&raw mut data);
        newversion = get32bit(&raw mut data);
        version = get32bit(&raw mut data);
        packet = masterconn_create_detached_packet(
            eptr,
            CSTOMA_TRUNCATE as uint32_t,
            (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t,
        );
        ptr = masterconn_get_packet_data(packet);
        put64bit(&raw mut ptr, chunkid);
        if newversion > 0 as uint32_t && leng != 0xffffffff as uint32_t {
            job_chunkop(
                Some(
                    masterconn_jobfinished
                        as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
                ),
                busychunk_start(packet, chunkid),
                chunkid,
                version,
                newversion,
                0 as uint64_t,
                0 as uint32_t,
                leng,
            );
        } else {
            job_inval(
                Some(
                    masterconn_jobfinished
                        as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
                ),
                busychunk_start(packet, chunkid),
            );
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_duptrunc(
    mut eptr: *mut masterconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut copychunkid: uint64_t = 0;
        let mut copyversion: uint32_t = 0;
        let mut leng: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut packet: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if length
            != (8 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int) as uint32_t
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_DUPTRUNC - wrong size (%u/28)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        copychunkid = get64bit(&raw mut data);
        copyversion = get32bit(&raw mut data);
        chunkid = get64bit(&raw mut data);
        version = get32bit(&raw mut data);
        leng = get32bit(&raw mut data);
        packet = masterconn_create_detached_packet(
            eptr,
            CSTOMA_DUPTRUNC as uint32_t,
            (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t,
        );
        ptr = masterconn_get_packet_data(packet);
        put64bit(&raw mut ptr, copychunkid);
        if version > 0 as uint32_t && copychunkid > 0 as uint64_t && leng != 0xffffffff as uint32_t
        {
            job_chunkop(
                Some(
                    masterconn_jobfinished
                        as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
                ),
                busychunk_start(packet, chunkid),
                chunkid,
                version,
                version,
                copychunkid,
                copyversion,
                leng,
            );
        } else {
            job_inval(
                Some(
                    masterconn_jobfinished
                        as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
                ),
                busychunk_start(packet, chunkid),
            );
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_localsplit(
    mut eptr: *mut masterconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut parts: uint8_t = 0;
        let mut missingmask: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut packet: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if length
            != (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                as uint32_t
            && length
                != (8 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as uint32_t
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_LOCALSPLIT - wrong size (%u/16|17)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        chunkid = get64bit(&raw mut data);
        version = get32bit(&raw mut data);
        missingmask = get32bit(&raw mut data);
        if length == 17 as uint32_t {
            parts = get8bit(&raw mut data);
        } else {
            parts = 8 as uint8_t;
        }
        if parts as ::core::ffi::c_int != 8 as ::core::ffi::c_int
            && parts as ::core::ffi::c_int != 4 as ::core::ffi::c_int
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_LOCALSPLIT - unsupported parts number (%hhu/4|8)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                parts as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        packet = masterconn_create_detached_packet(
            eptr,
            CSTOMA_LOCALSPLIT as uint32_t,
            (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as uint32_t,
        );
        ptr = masterconn_get_packet_data(packet);
        put64bit(&raw mut ptr, chunkid);
        put32bit(&raw mut ptr, version);
        if (*eptr).registerstate as ::core::ffi::c_int == REGISTERED as ::core::ffi::c_int {
            job_chunkop(
                Some(
                    masterconn_localsplitfinished
                        as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
                ),
                busychunk_start(packet, chunkid),
                chunkid,
                version,
                version,
                0 as uint64_t,
                parts as uint32_t,
                0x80000000 as uint32_t | missingmask,
            );
        } else {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_LOCALSPLIT - got command while still registering\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            put8bit(&raw mut ptr, MFS_ERROR_NOTDONE as uint8_t);
            masterconn_attach_packet(eptr, packet);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_chunkop(
    mut eptr: *mut masterconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut newversion: uint32_t = 0;
        let mut copychunkid: uint64_t = 0;
        let mut copyversion: uint32_t = 0;
        let mut leng: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut packet: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if length
            != (8 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int) as uint32_t
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_CHUNKOP - wrong size (%u/32)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        chunkid = get64bit(&raw mut data);
        version = get32bit(&raw mut data);
        newversion = get32bit(&raw mut data);
        copychunkid = get64bit(&raw mut data);
        copyversion = get32bit(&raw mut data);
        leng = get32bit(&raw mut data);
        packet = masterconn_create_detached_packet(
            eptr,
            CSTOMA_CHUNKOP as uint32_t,
            (8 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as uint32_t,
        );
        ptr = masterconn_get_packet_data(packet);
        put64bit(&raw mut ptr, chunkid);
        put32bit(&raw mut ptr, version);
        put32bit(&raw mut ptr, newversion);
        put64bit(&raw mut ptr, copychunkid);
        put32bit(&raw mut ptr, copyversion);
        put32bit(&raw mut ptr, leng);
        if (*eptr).registerstate as ::core::ffi::c_int == REGISTERED as ::core::ffi::c_int {
            job_chunkop(
                Some(
                    masterconn_chunkopfinished
                        as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
                ),
                busychunk_start(packet, chunkid),
                chunkid,
                version,
                newversion,
                copychunkid,
                copyversion,
                leng,
            );
        } else {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_CHUNKOP - got command while still registering\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            put8bit(&raw mut ptr, MFS_ERROR_NOTDONE as uint8_t);
            masterconn_attach_packet(eptr, packet);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_replicate(
    mut eptr: *mut masterconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut ip: uint32_t = 0;
        let mut port: uint16_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut packet: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if length != 18 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_REPLICATE - wrong size (%u/18)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        chunkid = get64bit(&raw mut data);
        version = get32bit(&raw mut data);
        packet = masterconn_create_detached_packet(
            eptr,
            CSTOMA_REPLICATE as uint32_t,
            (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as uint32_t,
        );
        ptr = masterconn_get_packet_data(packet);
        put64bit(&raw mut ptr, chunkid);
        put32bit(&raw mut ptr, version);
        ip = get32bit(&raw mut data);
        port = get16bit(&raw mut data);
        if (*eptr).registerstate as ::core::ffi::c_int == REGISTERED as ::core::ffi::c_int {
            job_replicate_simple(
                Some(
                    masterconn_replicationfinished
                        as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
                ),
                busychunk_start(packet, chunkid),
                chunkid,
                version,
                ip,
                port,
            );
        } else {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_REPLICATE - got command while still registering\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            put8bit(&raw mut ptr, MFS_ERROR_NOTDONE as uint8_t);
            masterconn_attach_packet(eptr, packet);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_replicate_split(
    mut eptr: *mut masterconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut ip: uint32_t = 0;
        let mut port: uint16_t = 0;
        let mut srcchunkid: uint64_t = 0;
        let mut partno: uint8_t = 0;
        let mut parts: uint8_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut packet: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if length != 28 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_REPLICATE_SPLIT - wrong size (%u/28)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        chunkid = get64bit(&raw mut data);
        version = get32bit(&raw mut data);
        packet = masterconn_create_detached_packet(
            eptr,
            CSTOMA_REPLICATE_SPLIT as uint32_t,
            (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as uint32_t,
        );
        ptr = masterconn_get_packet_data(packet);
        put64bit(&raw mut ptr, chunkid);
        put32bit(&raw mut ptr, version);
        ip = get32bit(&raw mut data);
        port = get16bit(&raw mut data);
        srcchunkid = get64bit(&raw mut data);
        partno = get8bit(&raw mut data);
        parts = get8bit(&raw mut data);
        if (*eptr).registerstate as ::core::ffi::c_int == REGISTERED as ::core::ffi::c_int {
            job_replicate_split(
                Some(
                    masterconn_replicationfinished
                        as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
                ),
                busychunk_start(packet, chunkid),
                chunkid,
                version,
                ip,
                port,
                srcchunkid,
                partno,
                parts,
            );
        } else {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_REPLICATE_SPLIT - got command while still registering\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            put8bit(&raw mut ptr, MFS_ERROR_NOTDONE as uint8_t);
            masterconn_attach_packet(eptr, packet);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_replicate_recover(
    mut eptr: *mut masterconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut srcchunkid: [uint64_t; 8] = [0; 8];
        let mut ip: [uint32_t; 8] = [0; 8];
        let mut port: [uint16_t; 8] = [0; 8];
        let mut i: uint8_t = 0;
        let mut parts: uint8_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut packet: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut d1: uint32_t = 0;
        let mut d2: uint32_t = 0;
        let mut d3: uint32_t = 0;
        let mut d4: uint32_t = 0;
        if length < 29 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_REPLICATE_RECOVER - wrong size (%u/29+n*14)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        chunkid = get64bit(&raw mut data);
        version = get32bit(&raw mut data);
        d1 = get32bit(&raw mut data);
        d2 = get32bit(&raw mut data);
        d3 = get32bit(&raw mut data);
        d4 = get32bit(&raw mut data);
        parts = get8bit(&raw mut data);
        if length != (29 as uint32_t).wrapping_add((parts as uint32_t).wrapping_mul(14 as uint32_t))
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_REPLICATE_RECOVER - wrong size (%u/29+n*14:n=%hhu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                parts as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if parts as ::core::ffi::c_int > MAX_EC_PARTS {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_REPLICATE_RECOVER - too many parts (%hhu/%u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                parts as ::core::ffi::c_int,
                MAX_EC_PARTS,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
            if d1 != 0x88888888 as uint32_t
                || d2 != 0x44444444 as uint32_t
                || d3 != 0x22222222 as uint32_t
                || d4 != 0x11111111 as uint32_t
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"MATOCS_REPLICATE_RECOVER - wrong packet\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
        } else if parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            if d1 != 0x8888 as uint32_t
                || d2 != 0x4444 as uint32_t
                || d3 != 0x2222 as uint32_t
                || d4 != 0x1111 as uint32_t
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"MATOCS_REPLICATE_RECOVER - wrong packet\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
        } else {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_REPLICATE_RECOVER - wrong parts number (%hhu/4|8)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                parts as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        packet = masterconn_create_detached_packet(
            eptr,
            CSTOMA_REPLICATE_RECOVER as uint32_t,
            (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as uint32_t,
        );
        ptr = masterconn_get_packet_data(packet);
        put64bit(&raw mut ptr, chunkid);
        put32bit(&raw mut ptr, version);
        i = 0 as uint8_t;
        while (i as ::core::ffi::c_int) < parts as ::core::ffi::c_int {
            ip[i as usize] = get32bit(&raw mut data);
            port[i as usize] = get16bit(&raw mut data);
            srcchunkid[i as usize] = get64bit(&raw mut data);
            i = i.wrapping_add(1);
        }
        if (*eptr).registerstate as ::core::ffi::c_int == REGISTERED as ::core::ffi::c_int {
            job_replicate_recover(
                Some(
                    masterconn_replicationfinished
                        as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
                ),
                busychunk_start(packet, chunkid),
                chunkid,
                version,
                parts,
                &raw mut ip as *mut uint32_t,
                &raw mut port as *mut uint16_t,
                &raw mut srcchunkid as *mut uint64_t,
            );
        } else {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_REPLICATE_RECOVER - got command while still registering\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            put8bit(&raw mut ptr, MFS_ERROR_NOTDONE as uint8_t);
            masterconn_attach_packet(eptr, packet);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_replicate_join(
    mut eptr: *mut masterconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut srcchunkid: [uint64_t; 8] = [0; 8];
        let mut ip: [uint32_t; 8] = [0; 8];
        let mut port: [uint16_t; 8] = [0; 8];
        let mut i: uint8_t = 0;
        let mut parts: uint8_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut packet: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if length < 13 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_REPLICATE_JOIN - wrong size (%u/13+n*14)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        chunkid = get64bit(&raw mut data);
        version = get32bit(&raw mut data);
        parts = get8bit(&raw mut data);
        if length != (13 as uint32_t).wrapping_add((parts as uint32_t).wrapping_mul(14 as uint32_t))
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_REPLICATE_JOIN - wrong size (%u/13+n*14:n=%hhu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
                parts as ::core::ffi::c_int,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if parts as ::core::ffi::c_int > MAX_EC_PARTS {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_REPLICATE_JOIN - too many parts (%hhu/%u)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                parts as ::core::ffi::c_int,
                MAX_EC_PARTS,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        packet = masterconn_create_detached_packet(
            eptr,
            CSTOMA_REPLICATE_JOIN as uint32_t,
            (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as uint32_t,
        );
        ptr = masterconn_get_packet_data(packet);
        put64bit(&raw mut ptr, chunkid);
        put32bit(&raw mut ptr, version);
        i = 0 as uint8_t;
        while (i as ::core::ffi::c_int) < parts as ::core::ffi::c_int {
            ip[i as usize] = get32bit(&raw mut data);
            port[i as usize] = get16bit(&raw mut data);
            srcchunkid[i as usize] = get64bit(&raw mut data);
            i = i.wrapping_add(1);
        }
        if (*eptr).registerstate as ::core::ffi::c_int == REGISTERED as ::core::ffi::c_int {
            job_replicate_join(
                Some(
                    masterconn_replicationfinished
                        as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
                ),
                busychunk_start(packet, chunkid),
                chunkid,
                version,
                parts,
                &raw mut ip as *mut uint32_t,
                &raw mut port as *mut uint16_t,
                &raw mut srcchunkid as *mut uint64_t,
            );
        } else {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"MATOCS_REPLICATE_JOIN - got command while still registering\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            put8bit(&raw mut ptr, MFS_ERROR_NOTDONE as uint8_t);
            masterconn_attach_packet(eptr, packet);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_idlejob_finished(
    mut status: uint8_t,
    mut ijp: *mut ::core::ffi::c_void,
) {
    unsafe {
        let mut ij: *mut idlejob = ijp as *mut idlejob;
        let mut eptr: *mut masterconn = masterconnsingleton;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if !eptr.is_null()
            && (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
            && (*ij).valid as ::core::ffi::c_int != 0
        {
            match (*ij).op as ::core::ffi::c_int {
                0 => {
                    ptr = masterconn_create_attached_packet(
                        eptr,
                        CSTOAN_CHUNK_BLOCKS as uint32_t,
                        (8 as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as uint32_t,
                    );
                    put64bit(&raw mut ptr, (*ij).chunkid);
                    put32bit(&raw mut ptr, (*ij).version);
                    memcpy(
                        ptr as *mut ::core::ffi::c_void,
                        &raw mut (*ij).buff as *mut uint8_t as *const ::core::ffi::c_void,
                        2 as size_t,
                    );
                    ptr = ptr.offset(2 as ::core::ffi::c_int as isize);
                    put8bit(&raw mut ptr, status);
                }
                1 => {
                    if status as ::core::ffi::c_int != MFS_STATUS_OK {
                        ptr = masterconn_create_attached_packet(
                            eptr,
                            CSTOAN_CHUNK_CHECKSUM as uint32_t,
                            (8 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int) as uint32_t,
                        );
                    } else {
                        ptr = masterconn_create_attached_packet(
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
                        ptr = masterconn_create_attached_packet(
                            eptr,
                            CSTOAN_CHUNK_CHECKSUM_TAB as uint32_t,
                            (8 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int) as uint32_t,
                        );
                    } else {
                        ptr = masterconn_create_attached_packet(
                            eptr,
                            CSTOAN_CHUNK_CHECKSUM_TAB as uint32_t,
                            (8 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int * MFSBLOCKSINCHUNK)
                                as uint32_t,
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
                            (4 as ::core::ffi::c_int * MFSBLOCKSINCHUNK) as size_t,
                        );
                    }
                }
                _ => {}
            }
        }
        if (*ij).valid != 0 {
            *(*ij).prev = (*ij).next;
            if !(*ij).next.is_null() {
                (*(*ij).next).prev = (*ij).prev;
            }
        }
        free(ij as *mut ::core::ffi::c_void);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_get_chunk_blocks(
    mut eptr: *mut masterconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut ij: *mut idlejob = ::core::ptr::null_mut::<idlejob>();
        if length != (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOCS_GET_CHUNK_BLOCKS - wrong size (%u/12)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        ij = malloc(
            if (40 as usize).wrapping_add(2 as usize) < ::core::mem::size_of::<idlejob>() {
                ::core::mem::size_of::<idlejob>()
            } else {
                (40 as size_t).wrapping_add(2 as size_t)
            },
        ) as *mut idlejob;
        (*ij).op = IJ_GET_CHUNK_BLOCKS as ::core::ffi::c_int as uint8_t;
        (*ij).chunkid = get64bit(&raw mut data);
        (*ij).version = get32bit(&raw mut data);
        (*ij).valid = 1 as uint8_t;
        (*ij).next = idlejobs as *mut idlejob;
        (*ij).prev = &raw mut idlejobs as *mut *mut idlejob;
        idlejobs = ij;
        (*ij).jobid = job_get_chunk_info(
            Some(
                masterconn_idlejob_finished
                    as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
            ),
            ij as *mut ::core::ffi::c_void,
            (*ij).chunkid,
            (*ij).version,
            REQUEST_BLOCKS as uint8_t,
            &raw mut (*ij).buff as *mut uint8_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_get_chunk_checksum(
    mut eptr: *mut masterconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut ij: *mut idlejob = ::core::ptr::null_mut::<idlejob>();
        if length != (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOCS_GET_CHUNK_CHECKSUM - wrong size (%u/12)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        ij = malloc(
            if (40 as usize).wrapping_add(4 as usize) < ::core::mem::size_of::<idlejob>() {
                ::core::mem::size_of::<idlejob>()
            } else {
                (40 as size_t).wrapping_add(4 as size_t)
            },
        ) as *mut idlejob;
        (*ij).op = IJ_GET_CHUNK_CHECKSUM as ::core::ffi::c_int as uint8_t;
        (*ij).chunkid = get64bit(&raw mut data);
        (*ij).version = get32bit(&raw mut data);
        (*ij).valid = 1 as uint8_t;
        (*ij).next = idlejobs as *mut idlejob;
        (*ij).prev = &raw mut idlejobs as *mut *mut idlejob;
        idlejobs = ij;
        (*ij).jobid = job_get_chunk_info(
            Some(
                masterconn_idlejob_finished
                    as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
            ),
            ij as *mut ::core::ffi::c_void,
            (*ij).chunkid,
            (*ij).version,
            REQUEST_CHECKSUM as uint8_t,
            &raw mut (*ij).buff as *mut uint8_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_get_chunk_checksum_tab(
    mut eptr: *mut masterconn,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut ij: *mut idlejob = ::core::ptr::null_mut::<idlejob>();
        if length != (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"ANTOCS_GET_CHUNK_CHECKSUM_TAB - wrong size (%u/12)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        ij = malloc(
            if (40 as usize)
                .wrapping_add((4 as ::core::ffi::c_int * 0x400 as ::core::ffi::c_int) as usize)
                < ::core::mem::size_of::<idlejob>()
            {
                ::core::mem::size_of::<idlejob>()
            } else {
                (40 as size_t)
                    .wrapping_add((4 as ::core::ffi::c_int * 0x400 as ::core::ffi::c_int) as size_t)
            },
        ) as *mut idlejob;
        (*ij).op = IJ_GET_CHUNK_CHECKSUM_TAB as ::core::ffi::c_int as uint8_t;
        (*ij).chunkid = get64bit(&raw mut data);
        (*ij).version = get32bit(&raw mut data);
        (*ij).valid = 1 as uint8_t;
        (*ij).next = idlejobs as *mut idlejob;
        (*ij).prev = &raw mut idlejobs as *mut *mut idlejob;
        idlejobs = ij;
        (*ij).jobid = job_get_chunk_info(
            Some(
                masterconn_idlejob_finished
                    as unsafe extern "C" fn(uint8_t, *mut ::core::ffi::c_void) -> (),
            ),
            ij as *mut ::core::ffi::c_void,
            (*ij).chunkid,
            (*ij).version,
            REQUEST_CHECKSUM_TAB as uint8_t,
            &raw mut (*ij).buff as *mut uint8_t,
        );
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
            5 => {
                masterconn_force_timeout(eptr, data, length);
            }
            110 => {
                masterconn_create(eptr, data, length);
            }
            120 => {
                masterconn_delete(eptr, data, length);
            }
            140 => {
                masterconn_setversion(eptr, data, length);
            }
            130 => {
                masterconn_duplicate(eptr, data, length);
            }
            150 => {
                masterconn_replicate(eptr, data, length);
            }
            154 => {
                masterconn_replicate_split(eptr, data, length);
            }
            156 => {
                masterconn_replicate_recover(eptr, data, length);
            }
            158 => {
                masterconn_replicate_join(eptr, data, length);
            }
            180 => {
                masterconn_localsplit(eptr, data, length);
            }
            152 => {
                masterconn_chunkop(eptr, data, length);
            }
            160 => {
                masterconn_truncate(eptr, data, length);
            }
            170 => {
                masterconn_duptrunc(eptr, data, length);
            }
            250 => {
                masterconn_get_chunk_blocks(eptr, data, length);
            }
            300 => {
                masterconn_get_chunk_checksum(eptr, data, length);
            }
            302 => {
                masterconn_get_chunk_checksum_tab(eptr, data, length);
            }
            104 => {
                (*eptr).masteraddrvalid = 1 as uint8_t;
                masterconn_master_ack(eptr, data, length);
            }
            99 => {
                masterconn_register_first(eptr, data, length);
            }
            97 => {
                masterconn_chunk_status(eptr, data, length);
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
        (*eptr).conncnt = (*eptr).conncnt.wrapping_add(1);
        (*eptr).masterversion = 0 as uint32_t;
        (*eptr).gotrndblob = 0 as uint8_t;
        memset(
            &raw mut (*eptr).rndblob as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            32 as size_t,
        );
        (*eptr).registerstate = UNREGISTERED as ::core::ffi::c_int as uint8_t;
        masterconn_sendregister(eptr);
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
                if mip & 0xff000000 as uint32_t != 0x7f000000 as uint32_t {
                    (*eptr).masterip = mip;
                    (*eptr).masterport = mport;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"master connection module: localhost (%u.%u.%u.%u) can't be used for connecting with master (use ip address of network controller)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        mip >> 24 as ::core::ffi::c_int & 0xff as uint32_t,
                        mip >> 16 as ::core::ffi::c_int & 0xff as uint32_t,
                        mip >> 8 as ::core::ffi::c_int & 0xff as uint32_t,
                        mip & 0xff as uint32_t,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            } else {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"master connection module: can't resolve master host/port (%s:%s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    MasterHost,
                    MasterPort,
                );
                return -1 as ::core::ffi::c_int;
            }
        }
        (*eptr).masteraddrvalid = 0 as uint8_t;
        (*eptr).sock = tcpsocket();
        if (*eptr).sock < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"master connection module: create socket error\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        if tcpnonblock((*eptr).sock) < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"master connection module: set nonblock error\0".as_ptr()
                    as *const ::core::ffi::c_char,
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
                    b"master connection module: can't bind socket to given ip\0".as_ptr()
                        as *const ::core::ffi::c_char,
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
                b"master connection module: connect failed\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            tcpclose((*eptr).sock);
            (*eptr).sock = -1 as ::core::ffi::c_int;
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
                MFSLOG_ERRNO_SYSLOG_STDERR,
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
                    b"/tmp/moosefs-ref/mfschunkserver/masterconn.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1576 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/masterconn.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1576 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                    b"/tmp/moosefs-ref/mfschunkserver/masterconn.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1576 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfschunkserver/masterconn.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1576 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                stats_bytesin = stats_bytesin.wrapping_add(i as uint64_t);
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
                        b"/tmp/moosefs-ref/mfschunkserver/masterconn.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1598 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/masterconn.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1598 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                        b"/tmp/moosefs-ref/mfschunkserver/masterconn.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1598 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/masterconn.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1598 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                        b"/tmp/moosefs-ref/mfschunkserver/masterconn.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1638 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr->input_packet\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/masterconn.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1638 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                        b"/tmp/moosefs-ref/mfschunkserver/masterconn.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1638 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr->input_packet\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfschunkserver/masterconn.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1638 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                MFSLOG_WARNING,
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
            stats_bytesout = stats_bytesout.wrapping_add(i as uint64_t);
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
        if ((*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
            || (*eptr).mode as ::core::ffi::c_int == CLOSE as ::core::ffi::c_int)
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
        let mut ij: *mut idlejob = ::core::ptr::null_mut::<idlejob>();
        let mut nij: *mut idlejob = ::core::ptr::null_mut::<idlejob>();
        if (*eptr).mode as ::core::ffi::c_int == KILL as ::core::ffi::c_int
            || (*eptr).mode as ::core::ffi::c_int == CLOSE as ::core::ffi::c_int
                && (*eptr).outputhead.is_null()
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_NOTICE,
                b"closing connection with master\0".as_ptr() as *const ::core::ffi::c_char,
            );
            tcpclose((*eptr).sock);
            (*eptr).sock = -1 as ::core::ffi::c_int;
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
            ij = idlejobs;
            while !ij.is_null() {
                nij = (*ij).next as *mut idlejob;
                job_pool_disable_job((*ij).jobid);
                (*ij).next = ::core::ptr::null_mut::<idlejob>();
                (*ij).prev = ::core::ptr::null_mut::<*mut idlejob>();
                (*ij).valid = 0 as uint8_t;
                ij = nij;
            }
            idlejobs = ::core::ptr::null_mut::<idlejob>();
            if (*eptr).registerstate as ::core::ffi::c_int == INPROGRESS as ::core::ffi::c_int {
                hdd_get_chunks_end();
            }
            if (*eptr).registerstate as ::core::ffi::c_int == UNREGISTERED as ::core::ffi::c_int
                && (*eptr).mode as ::core::ffi::c_int == KILL as ::core::ffi::c_int
            {
                (*eptr).masteraddrvalid = 0 as uint8_t;
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
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"masterconn: connection closed by master\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    (*eptr).input_end = 1 as uint8_t;
                }
                masterconn_parse(eptr);
            }
            if ((*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
                || (*eptr).mode as ::core::ffi::c_int == CLOSE as ::core::ffi::c_int)
                && (*eptr).lastwrite + 1.0f64 < now
                && (*eptr).outputhead.is_null()
            {
                masterconn_create_attached_packet(eptr, ANTOAN_NOP as uint32_t, 0 as uint32_t);
            }
            if (*eptr).pdescpos >= 0 as int32_t {
                if ((*pdesc.offset((*eptr).pdescpos as isize)).events as ::core::ffi::c_int
                    & POLLOUT
                    == 0 as ::core::ffi::c_int
                    && !(*eptr).outputhead.is_null()
                    || (*pdesc.offset((*eptr).pdescpos as isize)).revents as ::core::ffi::c_int
                        & POLLOUT
                        != 0)
                    && ((*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
                        || (*eptr).mode as ::core::ffi::c_int == CLOSE as ::core::ffi::c_int)
                {
                    masterconn_write(eptr, now);
                }
            }
            if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
                && ((*eptr).lastread
                    + (*eptr).timeout as ::core::ffi::c_int as ::core::ffi::c_double)
                    < now
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"masterconn: connection timed out\0".as_ptr() as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            }
        }
        if (*eptr).mode as ::core::ffi::c_int == CLOSE as ::core::ffi::c_int
            && wantexittime > 0.0f64
            && wantexittime + FORCE_DISCONNECTION_TO < now
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"masterconn: unregistering timed out\0".as_ptr() as *const ::core::ffi::c_char,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
        }
        masterconn_disconnection_check();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_forcereconnect() {
    unsafe {
        reconnectisneeded = 1 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_reconnect() {
    unsafe {
        let mut eptr: *mut masterconn = masterconnsingleton;
        if (*eptr).mode as ::core::ffi::c_int == FREE as ::core::ffi::c_int
            && wantexittime == 0.0f64
        {
            masterconn_initconnect(eptr);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_regstate(
    mut registerstate: uint8_t,
) -> *const ::core::ffi::c_char {
    match registerstate as ::core::ffi::c_int {
        0 => return b"UNREGISTERED\0".as_ptr() as *const ::core::ffi::c_char,
        1 => return b"WAITING\0".as_ptr() as *const ::core::ffi::c_char,
        2 => return b"INPROGRESS\0".as_ptr() as *const ::core::ffi::c_char,
        3 => return b"REGISTERED\0".as_ptr() as *const ::core::ffi::c_char,
        _ => {}
    }
    return b"???\0".as_ptr() as *const ::core::ffi::c_char;
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
        4 => return b"FLUSHING DATA\0".as_ptr() as *const ::core::ffi::c_char,
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
            b"registered state: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            masterconn_regstate((*eptr).registerstate),
        );
        fprintf(
            fd,
            b"socket mode: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            masterconn_socketmode((*eptr).mode),
        );
        fprintf(
            fd,
            b"connection counter: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
            (*eptr).conncnt,
        );
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
        let mut newAuthCode: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut newMasterHost: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut newMasterPort: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut newBindHost: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut newTimeout: uint32_t = 0;
        ChunksPerRegisterPacket = cfg_getuint32(
            b"CHUNKS_PER_REGISTER_PACKET\0".as_ptr() as *const ::core::ffi::c_char,
            1000 as uint32_t,
        );
        if ChunksPerRegisterPacket < 100 as uint32_t {
            ChunksPerRegisterPacket = 100 as uint32_t;
        }
        if ChunksPerRegisterPacket > 10000 as uint32_t {
            ChunksPerRegisterPacket = 10000 as uint32_t;
        }
        if cfg_isdefined(b"AUTH_CODE\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
            newAuthCode = cfg_getstr(
                b"AUTH_CODE\0".as_ptr() as *const ::core::ffi::c_char,
                b"mfspassword\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            newAuthCode = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if AuthCode.is_null() && newAuthCode.is_null()
            || !AuthCode.is_null()
                && !newAuthCode.is_null()
                && strcmp(AuthCode, newAuthCode) == 0 as ::core::ffi::c_int
        {
            if !newAuthCode.is_null() {
                free(newAuthCode as *mut ::core::ffi::c_void);
            }
        } else {
            reconnectisneeded = 1 as ::core::ffi::c_int;
            if !AuthCode.is_null() {
                free(AuthCode as *mut ::core::ffi::c_void);
                AuthCode = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            AuthCode = newAuthCode;
        }
        newMasterHost = cfg_getstr(
            b"MASTER_HOST\0".as_ptr() as *const ::core::ffi::c_char,
            DEFAULT_MASTERNAME.as_ptr(),
        );
        newMasterPort = cfg_getstr(
            b"MASTER_PORT\0".as_ptr() as *const ::core::ffi::c_char,
            DEFAULT_MASTER_CS_PORT.as_ptr(),
        );
        newBindHost = cfg_getstr(
            b"BIND_HOST\0".as_ptr() as *const ::core::ffi::c_char,
            b"*\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if strcmp(newMasterHost, MasterHost) == 0 as ::core::ffi::c_int
            && strcmp(newMasterPort, MasterPort) == 0 as ::core::ffi::c_int
            && strcmp(newBindHost, BindHost) == 0 as ::core::ffi::c_int
        {
            free(newMasterHost as *mut ::core::ffi::c_void);
            free(newMasterPort as *mut ::core::ffi::c_void);
            free(newBindHost as *mut ::core::ffi::c_void);
        } else {
            reconnectisneeded = 1 as ::core::ffi::c_int;
            free(MasterHost as *mut ::core::ffi::c_void);
            free(MasterPort as *mut ::core::ffi::c_void);
            free(BindHost as *mut ::core::ffi::c_void);
            MasterHost = newMasterHost;
            MasterPort = newMasterPort;
            BindHost = newBindHost;
        }
        ReconnectionDelay = cfg_getuint32(
            b"MASTER_RECONNECTION_DELAY\0".as_ptr() as *const ::core::ffi::c_char,
            5 as uint32_t,
        );
        main_time_change(reconnect_hook, ReconnectionDelay, 0 as uint32_t);
        newTimeout = cfg_getuint32(
            b"MASTER_TIMEOUT\0".as_ptr() as *const ::core::ffi::c_char,
            0 as uint32_t,
        );
        if newTimeout > 65535 as uint32_t {
            newTimeout = 65535 as uint32_t;
        }
        if newTimeout < 10 as uint32_t && newTimeout > 0 as uint32_t {
            newTimeout = 10 as uint32_t;
        }
        if newTimeout != Timeout {
            reconnectisneeded = 1 as ::core::ffi::c_int;
            Timeout = newTimeout;
        }
        if masterconn_parselabels() != 0 {
            if reconnectisneeded == 0 as ::core::ffi::c_int {
                if !eptr.is_null()
                    && (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
                    && (*eptr).registerstate as ::core::ffi::c_int
                        == REGISTERED as ::core::ffi::c_int
                {
                    masterconn_sendlabels(eptr);
                } else {
                    reconnectisneeded = 1 as ::core::ffi::c_int;
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_wantexit() {
    unsafe {
        masterconn_send_disconnect_command();
        wantexittime = monotonic_seconds();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_canexit() -> ::core::ffi::c_int {
    unsafe {
        let mut eptr: *mut masterconn = masterconnsingleton;
        return if (*eptr).mode as ::core::ffi::c_int == FREE as ::core::ffi::c_int
            || (*eptr).outputhead.is_null()
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn masterconn_init() -> ::core::ffi::c_int {
    unsafe {
        let mut ReconnectionDelay: uint32_t = 0;
        let mut eptr: *mut masterconn = ::core::ptr::null_mut::<masterconn>();
        masterconn_initcsid();
        manager_time_hook = NULL;
        ChunksPerRegisterPacket = cfg_getuint32(
            b"CHUNKS_PER_REGISTER_PACKET\0".as_ptr() as *const ::core::ffi::c_char,
            1000 as uint32_t,
        );
        if ChunksPerRegisterPacket < 100 as uint32_t {
            ChunksPerRegisterPacket = 100 as uint32_t;
        }
        if ChunksPerRegisterPacket > 10000 as uint32_t {
            ChunksPerRegisterPacket = 10000 as uint32_t;
        }
        if cfg_isdefined(b"AUTH_CODE\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
            AuthCode = cfg_getstr(
                b"AUTH_CODE\0".as_ptr() as *const ::core::ffi::c_char,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
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
            DEFAULT_MASTER_CS_PORT.as_ptr(),
        );
        BindHost = cfg_getstr(
            b"BIND_HOST\0".as_ptr() as *const ::core::ffi::c_char,
            b"*\0".as_ptr() as *const ::core::ffi::c_char,
        );
        Timeout = cfg_getuint32(
            b"MASTER_TIMEOUT\0".as_ptr() as *const ::core::ffi::c_char,
            0 as uint32_t,
        );
        if Timeout > 65535 as uint32_t {
            Timeout = 65535 as uint32_t;
        }
        if Timeout < 10 as uint32_t && Timeout > 0 as uint32_t {
            Timeout = 10 as uint32_t;
        }
        masterconn_parselabels();
        masterconnsingleton = malloc(::core::mem::size_of::<masterconn>()) as *mut masterconn;
        eptr = masterconnsingleton;
        if eptr.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2111 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2111 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfschunkserver/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2111 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfschunkserver/masterconn.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                2111 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*eptr).masteraddrvalid = 0 as uint8_t;
        (*eptr).masterversion = 0 as uint32_t;
        (*eptr).mode = FREE as ::core::ffi::c_int as uint8_t;
        (*eptr).pdescpos = -1 as ::core::ffi::c_int as int32_t;
        (*eptr).conncnt = 0 as uint32_t;
        if Timeout > 0 as uint32_t {
            (*eptr).timeout = Timeout as uint16_t;
        } else {
            (*eptr).timeout = 10 as uint16_t;
        }
        wantexittime = 0.0f64;
        if masterconn_initconnect(eptr) < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        main_time_register_fname(
            1 as uint32_t,
            0 as uint32_t,
            Some(masterconn_reportload as unsafe extern "C" fn() -> ()),
            b"masterconn_reportload\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_time_register_fname(
            1 as uint32_t,
            0 as uint32_t,
            Some(masterconn_check_hdd_space as unsafe extern "C" fn() -> ()),
            b"masterconn_check_hdd_space\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_eachloop_register_fname(
            Some(masterconn_check_hdd_reports as unsafe extern "C" fn() -> ()),
            b"masterconn_check_hdd_reports\0".as_ptr() as *const ::core::ffi::c_char,
        );
        reconnect_hook = main_time_register_fname(
            ReconnectionDelay,
            rndu32_ranged(ReconnectionDelay),
            Some(masterconn_reconnect as unsafe extern "C" fn() -> ()),
            b"masterconn_reconnect\0".as_ptr() as *const ::core::ffi::c_char,
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
        main_wantexit_register_fname(
            Some(masterconn_wantexit as unsafe extern "C" fn() -> ()),
            b"masterconn_wantexit\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_canexit_register_fname(
            Some(masterconn_canexit as unsafe extern "C" fn() -> ::core::ffi::c_int),
            b"masterconn_canexit\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_reload_register_fname(
            Some(masterconn_reload as unsafe extern "C" fn() -> ()),
            b"masterconn_reload\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_info_register_fname(
            Some(masterconn_info as unsafe extern "C" fn(*mut FILE) -> ()),
            b"masterconn_info\0".as_ptr() as *const ::core::ffi::c_char,
        );
        busychunk_init();
        return 0 as ::core::ffi::c_int;
    }
}
