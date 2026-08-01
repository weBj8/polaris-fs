pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
pub enum csdbentry {}
use ::c2rust_bitfields;
unsafe extern "C" {
    static mut stderr: *mut FILE;
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
    unsafe fn writev(
        __fd: ::core::ffi::c_int,
        __iovec: *const iovec,
        __count: ::core::ffi::c_int,
    ) -> ssize_t;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn read(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    unsafe fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
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
    unsafe fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn csdb_new_connection(
        ip: uint32_t,
        port: uint16_t,
        csid: uint16_t,
        eptr: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn csdb_get_csid(v_csptr: *mut ::core::ffi::c_void) -> uint16_t;
    unsafe fn csdb_temporary_maintenance_mode(v_csptr: *mut ::core::ffi::c_void);
    unsafe fn csdb_lost_connection(v_csptr: *mut ::core::ffi::c_void);
    unsafe fn csdb_server_load(v_csptr: *mut ::core::ffi::c_void, load: uint32_t);
    unsafe fn csdb_server_is_overloaded(
        v_csptr: *mut ::core::ffi::c_void,
        now: uint32_t,
    ) -> uint8_t;
    unsafe fn csdb_server_is_being_maintained(v_csptr: *mut ::core::ffi::c_void) -> uint8_t;
    unsafe fn sclass_ec_version() -> uint8_t;
    unsafe fn chunk_got_status_data(
        chunkid: uint64_t,
        servdesc: *const ::core::ffi::c_char,
        csid: uint16_t,
        parts: uint8_t,
        ecid: *mut uint8_t,
        version: *mut uint32_t,
        damaged: *mut uint8_t,
        blocks: *mut uint16_t,
        fixmode: uint8_t,
    );
    unsafe fn chunk_get_mfrstatus(csid: uint16_t) -> uint8_t;
    unsafe fn chunk_server_connected(ptr: *mut ::core::ffi::c_void) -> uint16_t;
    unsafe fn chunk_server_has_chunk(
        csid: uint16_t,
        chunkid: uint64_t,
        ecid: uint8_t,
        version: uint32_t,
    );
    unsafe fn chunk_damaged(csid: uint16_t, chunkid: uint64_t, ecid: uint8_t);
    unsafe fn chunk_lost(csid: uint16_t, chunkid: uint64_t, ecid: uint8_t, report: uint8_t);
    unsafe fn chunk_server_register_end(csid: uint16_t);
    unsafe fn chunk_server_disconnected(csid: uint16_t);
    unsafe fn chunk_got_delete_status(
        csid: uint16_t,
        chunkid: uint64_t,
        ecid: uint8_t,
        status: uint8_t,
    );
    unsafe fn chunk_got_replicate_status(
        csid: uint16_t,
        chunkid: uint64_t,
        ecid: uint8_t,
        version: uint32_t,
        status: uint8_t,
    );
    unsafe fn chunk_got_chunkop_status(
        csid: uint16_t,
        chunkid: uint64_t,
        ecid: uint8_t,
        status: uint8_t,
    );
    unsafe fn chunk_got_create_status(
        csid: uint16_t,
        chunkid: uint64_t,
        ecid: uint8_t,
        status: uint8_t,
    );
    unsafe fn chunk_got_duplicate_status(
        csid: uint16_t,
        chunkid: uint64_t,
        ecid: uint8_t,
        status: uint8_t,
    );
    unsafe fn chunk_got_setversion_status(
        csid: uint16_t,
        chunkid: uint64_t,
        ecid: uint8_t,
        status: uint8_t,
    );
    unsafe fn chunk_got_truncate_status(
        csid: uint16_t,
        chunkid: uint64_t,
        ecid: uint8_t,
        status: uint8_t,
    );
    unsafe fn chunk_got_duptrunc_status(
        csid: uint16_t,
        chunkid: uint64_t,
        ecid: uint8_t,
        status: uint8_t,
    );
    unsafe fn chunk_got_localsplit_status(
        csid: uint16_t,
        chunkid: uint64_t,
        version: uint32_t,
        status: uint8_t,
    );
    unsafe fn meta_get_id() -> uint64_t;
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
    unsafe fn cfg_getuint8(name: *const ::core::ffi::c_char, def: uint8_t) -> uint8_t;
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
    unsafe fn main_time() -> uint32_t;
    unsafe fn univmakestrip(strip: *mut ::core::ffi::c_char, ip: uint32_t);
    unsafe fn univallocstripport(ip: uint32_t, port: uint16_t) -> *mut ::core::ffi::c_char;
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
    unsafe fn rndu8() -> uint8_t;
    unsafe fn rndu32_ranged(range: uint32_t) -> uint32_t;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
    unsafe fn monotonic_useconds() -> uint64_t;
    unsafe fn labelmask_matches_labelexpr(
        labelmask: uint32_t,
        labelexpr: *const uint8_t,
    ) -> uint8_t;
    unsafe fn multilan_map(servip: uint32_t, clientip: uint32_t) -> uint32_t;
    unsafe fn md5_init(ctx: *mut md5ctx);
    unsafe fn md5_update(ctx: *mut md5ctx, buff: *const uint8_t, leng: uint32_t);
    unsafe fn md5_final(digest: *mut uint8_t, ctx: *mut md5ctx);
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
pub type ssize_t = isize;
pub type int32_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut ::core::ffi::c_void,
    pub iov_len: size_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _storagemode {
    pub uniqmask: uint32_t,
    pub ec_data_chksum_parts: uint8_t,
    pub has_labels: uint8_t,
    pub matching_servers: uint8_t,
    pub valid_ec_counters: uint8_t,
    pub replallowed: uint16_t,
    pub overloaded: uint16_t,
    pub allvalid: uint16_t,
    pub data_replallowed: uint16_t,
    pub data_overloaded: uint16_t,
    pub data_allvalid: uint16_t,
    pub chksum_replallowed: uint16_t,
    pub chksum_overloaded: uint16_t,
    pub chksum_allvalid: uint16_t,
    pub both_replallowed: uint16_t,
    pub both_overloaded: uint16_t,
    pub both_allvalid: uint16_t,
    pub labels_mode: uint8_t,
    pub labelscnt: uint8_t,
    pub labelexpr: [[uint8_t; 128]; 9],
}
pub type storagemode = _storagemode;
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const REPL_REASONS: C2Rust_Unnamed = 17;
pub const RECOVER_IO: C2Rust_Unnamed = 16;
pub const SPLIT_EC_GENERIC: C2Rust_Unnamed = 15;
pub const JOIN_EC_GENERIC: C2Rust_Unnamed = 14;
pub const JOIN_EC_NOSERVERS: C2Rust_Unnamed = 13;
pub const JOIN_EC_CHANGE: C2Rust_Unnamed = 12;
pub const JOIN_EC_IO: C2Rust_Unnamed = 11;
pub const LOCALSPLIT_TO_EC8: C2Rust_Unnamed = 10;
pub const LOCALSPLIT_TO_EC4: C2Rust_Unnamed = 9;
pub const REPL_EC_REBALANCE: C2Rust_Unnamed = 8;
pub const REPL_EC_WRONGLABEL: C2Rust_Unnamed = 7;
pub const REPL_EC_UNDERGOAL: C2Rust_Unnamed = 6;
pub const REPL_EC_ENDANGERED: C2Rust_Unnamed = 5;
pub const REPL_COPY_REBALANCE: C2Rust_Unnamed = 4;
pub const REPL_COPY_WRONGLABEL: C2Rust_Unnamed = 3;
pub const REPL_COPY_UNDERGOAL: C2Rust_Unnamed = 2;
pub const REPL_COPY_ENDANGERED: C2Rust_Unnamed = 1;
pub const REPL_COPY_IO: C2Rust_Unnamed = 0;
pub type C2Rust_Unnamed_0 = ::core::ffi::c_uint;
pub const OP_REASONS: C2Rust_Unnamed_0 = 4;
pub const OP_DEL_OVERGOAL: C2Rust_Unnamed_0 = 3;
pub const OP_DEL_NOTUSED: C2Rust_Unnamed_0 = 2;
pub const OP_DEL_INVALID: C2Rust_Unnamed_0 = 1;
pub const OP_GENERIC_IO: C2Rust_Unnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct matocsserventry {
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
    pub servdesc: *mut ::core::ffi::c_char,
    pub peerip: uint32_t,
    pub version: uint32_t,
    pub servip: uint32_t,
    pub servport: uint16_t,
    pub timeout: uint16_t,
    pub load: uint32_t,
    pub hlstatus: uint8_t,
    pub usedspace: uint64_t,
    pub totalspace: uint64_t,
    pub chunkscount: uint32_t,
    pub todelusedspace: uint64_t,
    pub todeltotalspace: uint64_t,
    pub todelchunkscount: uint32_t,
    pub errorcounter: uint32_t,
    pub writecounter: uint16_t,
    pub rrepcounter: uint16_t,
    pub wrepcounter: uint16_t,
    pub delcounter: uint16_t,
    pub labelmask: uint32_t,
    pub labelstr: *mut ::core::ffi::c_char,
    pub create_total_counter: uint32_t,
    pub rrep_total_counter: uint32_t,
    pub wrep_total_counter: uint32_t,
    pub del_total_counter: uint32_t,
    pub total_counter_begin: ::core::ffi::c_double,
    pub csid: uint16_t,
    pub registered: uint8_t,
    pub lostchunkdelay: uint8_t,
    pub newchunkdelay: uint8_t,
    pub receivingchunks: uint8_t,
    pub passwordrnd: [uint8_t; 32],
    pub lreplreadok: [uint32_t; 17],
    pub lreplreaderr: [uint32_t; 17],
    pub lreplwriteok: [uint32_t; 17],
    pub lreplwriteerr: [uint32_t; 17],
    pub ldelok: [uint32_t; 4],
    pub ldelerr: [uint32_t; 4],
    pub replreadok: [uint32_t; 17],
    pub replreaderr: [uint32_t; 17],
    pub replwriteok: [uint32_t; 17],
    pub replwriteerr: [uint32_t; 17],
    pub delok: [uint32_t; 4],
    pub delerr: [uint32_t; 4],
    pub dist: uint32_t,
    pub first: uint8_t,
    pub corr: ::core::ffi::c_double,
    pub csptr: *mut csdbentry,
    pub next: *mut matocsserventry,
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
pub const KILL: C2Rust_Unnamed_2 = 0;
pub const REGISTERED: C2Rust_Unnamed_3 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct servsort {
    pub space: ::core::ffi::c_double,
    pub csid: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct servsort_0 {
    pub space: ::core::ffi::c_double,
    pub csid: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rservsort {
    pub err: ::core::ffi::c_double,
    pub ptr: *mut matocsserventry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rservsort_0 {
    pub err: ::core::ffi::c_double,
    pub ptr: *mut matocsserventry,
}
pub const WAITING: C2Rust_Unnamed_3 = 1;
pub const DATA: C2Rust_Unnamed_2 = 1;
pub const REPTYPE_SIMPLE: C2Rust_Unnamed_5 = 0;
pub type repsrc = _repsrc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _repsrc {
    pub src: *mut ::core::ffi::c_void,
    pub next: *mut _repsrc,
}
pub type repdst = _repdst;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _repdst {
    pub chunkid: uint64_t,
    pub version: uint32_t,
    pub rweight: uint8_t,
    pub wweight: uint8_t,
    pub reptype: uint8_t,
    pub reason: uint8_t,
    pub dst: *mut ::core::ffi::c_void,
    pub srchead: *mut repsrc,
    pub next: *mut _repdst,
}
pub const REPTYPE_SPLIT: C2Rust_Unnamed_5 = 1;
pub const REPTYPE_RECOVER: C2Rust_Unnamed_5 = 2;
pub const REPTYPE_JOIN: C2Rust_Unnamed_5 = 3;
pub const OPTYPE_DELETE: C2Rust_Unnamed_4 = 0;
pub type opsrv = _opsrv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opsrv {
    pub chunkid: uint64_t,
    pub version: uint32_t,
    pub optype: uint8_t,
    pub reason: uint8_t,
    pub srv: *mut ::core::ffi::c_void,
    pub next: *mut _opsrv,
}
pub const REPTYPE_LOCALSPLIT: C2Rust_Unnamed_5 = 4;
pub const RESERVE_BYTES: C2Rust_Unnamed_1 = 0;
pub const RESERVE_CHUNKSERVER_TOTAL: C2Rust_Unnamed_1 = 3;
pub const RESERVE_CHUNKSERVER_USED: C2Rust_Unnamed_1 = 2;
pub const RESERVE_PERCENT: C2Rust_Unnamed_1 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
}
pub const FINISH: C2Rust_Unnamed_2 = 2;
pub type md5ctx = _md5ctx;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _md5ctx {
    pub state: [uint32_t; 4],
    pub count: [uint32_t; 2],
    pub buffer: [uint8_t; 64],
}
pub const UNREGISTERED: C2Rust_Unnamed_3 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cfg_buff {
    pub leng: uint32_t,
    pub data: [uint8_t; 1],
}
pub type C2Rust_Unnamed_1 = ::core::ffi::c_uint;
pub type C2Rust_Unnamed_2 = ::core::ffi::c_uint;
pub type C2Rust_Unnamed_3 = ::core::ffi::c_uint;
pub type C2Rust_Unnamed_4 = ::core::ffi::c_uint;
pub const OPTYPE_DUPTRUNC: C2Rust_Unnamed_4 = 5;
pub const OPTYPE_DUPLICATE: C2Rust_Unnamed_4 = 4;
pub const OPTYPE_TRUNCATE: C2Rust_Unnamed_4 = 3;
pub const OPTYPE_SETVERSION: C2Rust_Unnamed_4 = 2;
pub const OPTYPE_CREATE: C2Rust_Unnamed_4 = 1;
pub type C2Rust_Unnamed_5 = ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ENOTSUP: ::core::ffi::c_int = EOPNOTSUPP;
pub const MFSCHUNKSIZE: ::core::ffi::c_int = 0x4000000 as ::core::ffi::c_int;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_ENOENT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFS_ERROR_MAX: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const HLSTATUS_DEFAULT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const HLSTATUS_OK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const HLSTATUS_OVERLOADED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const HLSTATUS_LSREBALANCE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const HLSTATUS_GRACEFUL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const HLSTATUS_HSREBALANCE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const TRANSFERRING_LOST_CHUNKS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TRANSFERRING_NEW_CHUNKS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CSTOMA_MAXPACKETSIZE: ::core::ffi::c_int = 500000000 as ::core::ffi::c_int;
pub const ANTOAN_NOP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ANTOAN_UNKNOWN_COMMAND: uint32_t = 1 as uint32_t;
pub const ANTOAN_BAD_COMMAND_SIZE: uint32_t = 2 as uint32_t;
pub const ANTOAN_FORCE_TIMEOUT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const ANTOAN_GET_VERSION: uint32_t = 10 as uint32_t;
pub const ANTOAN_VERSION: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const ANTOMA_SYSLOG: uint32_t = 71 as uint32_t;
pub const ANTOAN_GET_CONFIG: uint32_t = 80 as uint32_t;
pub const ANTOAN_CONFIG_VALUE: ::core::ffi::c_int = PROTO_BASE + 81 as ::core::ffi::c_int;
pub const ANTOAN_CONFIG_FILE_CONTENT: ::core::ffi::c_int = PROTO_BASE + 83 as ::core::ffi::c_int;
pub const CSTOMA_CHUNK_DOESNT_EXIST: uint32_t = 96 as uint32_t;
pub const MATOCS_CHUNK_STATUS: ::core::ffi::c_int = PROTO_BASE + 97 as ::core::ffi::c_int;
pub const CSTOMA_CHUNK_STATUS: uint32_t = 98 as uint32_t;
pub const MATOCS_REGISTER_FIRST: ::core::ffi::c_int = PROTO_BASE + 99 as ::core::ffi::c_int;
pub const CSTOMA_REGISTER: ::core::ffi::c_int = PROTO_BASE + 100 as ::core::ffi::c_int;
pub const CSTOMA_SPACE: uint32_t = 101 as uint32_t;
pub const CSTOMA_CHUNK_DAMAGED: uint32_t = 102 as uint32_t;
pub const CSTOMA_CURRENT_LOAD: uint32_t = 103 as uint32_t;
pub const MATOCS_MASTER_ACK: ::core::ffi::c_int = PROTO_BASE + 104 as ::core::ffi::c_int;
pub const CSTOMA_CHUNK_LOST: uint32_t = 105 as uint32_t;
pub const CSTOMA_ERROR_OCCURRED: uint32_t = 106 as uint32_t;
pub const CSTOMA_CHUNK_NEW: uint32_t = 107 as uint32_t;
pub const CSTOMA_LABELS: uint32_t = 109 as uint32_t;
pub const MATOCS_CREATE: ::core::ffi::c_int = PROTO_BASE + 110 as ::core::ffi::c_int;
pub const CSTOMA_CREATE: uint32_t = 111 as uint32_t;
pub const MATOCS_DELETE: ::core::ffi::c_int = PROTO_BASE + 120 as ::core::ffi::c_int;
pub const CSTOMA_DELETE: uint32_t = 121 as uint32_t;
pub const MATOCS_DUPLICATE: ::core::ffi::c_int = PROTO_BASE + 130 as ::core::ffi::c_int;
pub const CSTOMA_DUPLICATE: uint32_t = 131 as uint32_t;
pub const MATOCS_SET_VERSION: ::core::ffi::c_int = PROTO_BASE + 140 as ::core::ffi::c_int;
pub const CSTOMA_SET_VERSION: uint32_t = 141 as uint32_t;
pub const MATOCS_REPLICATE: ::core::ffi::c_int = PROTO_BASE + 150 as ::core::ffi::c_int;
pub const CSTOMA_REPLICATE: uint32_t = 151 as uint32_t;
pub const MATOCS_CHUNKOP: ::core::ffi::c_int = PROTO_BASE + 152 as ::core::ffi::c_int;
pub const MATOCS_REPLICATE_SPLIT: ::core::ffi::c_int = PROTO_BASE + 154 as ::core::ffi::c_int;
pub const CSTOMA_REPLICATE_SPLIT: uint32_t = 155 as uint32_t;
pub const MATOCS_REPLICATE_RECOVER: ::core::ffi::c_int = PROTO_BASE + 156 as ::core::ffi::c_int;
pub const CSTOMA_REPLICATE_RECOVER: uint32_t = 157 as uint32_t;
pub const MATOCS_REPLICATE_JOIN: ::core::ffi::c_int = PROTO_BASE + 158 as ::core::ffi::c_int;
pub const CSTOMA_REPLICATE_JOIN: uint32_t = 159 as uint32_t;
pub const MATOCS_TRUNCATE: ::core::ffi::c_int = PROTO_BASE + 160 as ::core::ffi::c_int;
pub const CSTOMA_TRUNCATE: uint32_t = 161 as uint32_t;
pub const MATOCS_DUPTRUNC: ::core::ffi::c_int = PROTO_BASE + 170 as ::core::ffi::c_int;
pub const CSTOMA_DUPTRUNC: uint32_t = 171 as uint32_t;
pub const MATOCS_LOCALSPLIT: ::core::ffi::c_int = PROTO_BASE + 180 as ::core::ffi::c_int;
pub const CSTOMA_LOCALSPLIT: uint32_t = 181 as uint32_t;
pub const ANTOCS_GET_CHUNK_CHECKSUM: ::core::ffi::c_int = PROTO_BASE + 300 as ::core::ffi::c_int;
pub const CSTOAN_CHUNK_CHECKSUM: uint32_t = 301 as uint32_t;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const EOPNOTSUPP: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const DEFAULT_MASTER_CS_PORT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"9420\0") };
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
pub const MAXCSCOUNT: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
pub const CSSTATE_NO_SPACE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CSSTATE_LIMIT_REACHED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CSSTATE_OVERLOADED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CSSTATE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn sizestrtod(
    mut str: *const ::core::ffi::c_char,
    mut endptr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_double {
    unsafe {
        let mut val: ::core::ffi::c_double = 0.;
        let mut frac: ::core::ffi::c_double = 0.;
        let mut f: ::core::ffi::c_int = 0;
        val = 0.0f64;
        f = 0 as ::core::ffi::c_int;
        while *str as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
            && *str as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
        {
            val *= 10.0f64;
            val +=
                (*str as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as ::core::ffi::c_double;
            str = str.offset(1);
            f = 1 as ::core::ffi::c_int;
        }
        if *str as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            && *str.offset(1 as isize) as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
            && *str.offset(1 as isize) as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
        {
            f = 1 as ::core::ffi::c_int;
            str = str.offset(1);
            frac = 1.0f64;
            while *str as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                && *str as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
            {
                frac /= 10.0f64;
                val += (*str as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                    as ::core::ffi::c_double
                    * frac;
                str = str.offset(1);
            }
        }
        if f != 0 {
            match *str as ::core::ffi::c_int {
                107 => {
                    str = str.offset(1);
                    val *= 1e3f64;
                }
                75 => {
                    if *str.offset(1 as isize) as ::core::ffi::c_int == 'i' as ::core::ffi::c_int {
                        str = str.offset(2 as ::core::ffi::c_int as isize);
                        val *= 1024.0f64;
                    }
                }
                77 => {
                    str = str.offset(1);
                    if *str as ::core::ffi::c_int == 'i' as ::core::ffi::c_int {
                        str = str.offset(1);
                        val *= 1048576.0f64;
                    } else {
                        val *= 1e6f64;
                    }
                }
                71 => {
                    str = str.offset(1);
                    if *str as ::core::ffi::c_int == 'i' as ::core::ffi::c_int {
                        str = str.offset(1);
                        val *= 1073741824.0f64;
                    } else {
                        val *= 1e9f64;
                    }
                }
                84 => {
                    str = str.offset(1);
                    if *str as ::core::ffi::c_int == 'i' as ::core::ffi::c_int {
                        str = str.offset(1);
                        val *= 1099511627776.0f64;
                    } else {
                        val *= 1e12f64;
                    }
                }
                80 => {
                    str = str.offset(1);
                    if *str as ::core::ffi::c_int == 'i' as ::core::ffi::c_int {
                        str = str.offset(1);
                        val *= 1125899906842624.0f64;
                    } else {
                        val *= 1e15f64;
                    }
                }
                69 => {
                    str = str.offset(1);
                    if *str as ::core::ffi::c_int == 'i' as ::core::ffi::c_int {
                        str = str.offset(1);
                        val *= 1152921504606846976.0f64;
                    } else {
                        val *= 1e18f64;
                    }
                }
                _ => {}
            }
        }
        if !endptr.is_null() {
            *endptr = str;
        }
        return val;
    }
}
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_ERRNO_SYSLOG_STDERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn mfsstrerr(mut status: uint8_t) -> *const ::core::ffi::c_char {
    unsafe {
        static mut errtab: [*const ::core::ffi::c_char; 65] = [
            b"OK\0".as_ptr() as *const ::core::ffi::c_char,
            b"Operation not permitted\0".as_ptr() as *const ::core::ffi::c_char,
            b"Not a directory\0".as_ptr() as *const ::core::ffi::c_char,
            b"No such file or directory\0".as_ptr() as *const ::core::ffi::c_char,
            b"Permission denied\0".as_ptr() as *const ::core::ffi::c_char,
            b"File exists\0".as_ptr() as *const ::core::ffi::c_char,
            b"Invalid argument\0".as_ptr() as *const ::core::ffi::c_char,
            b"Directory not empty\0".as_ptr() as *const ::core::ffi::c_char,
            b"Chunk lost\0".as_ptr() as *const ::core::ffi::c_char,
            b"Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            b"Index too big\0".as_ptr() as *const ::core::ffi::c_char,
            b"Chunk locked\0".as_ptr() as *const ::core::ffi::c_char,
            b"No chunk servers\0".as_ptr() as *const ::core::ffi::c_char,
            b"No such chunk\0".as_ptr() as *const ::core::ffi::c_char,
            b"Chunk is busy\0".as_ptr() as *const ::core::ffi::c_char,
            b"Incorrect register BLOB\0".as_ptr() as *const ::core::ffi::c_char,
            b"Operation not completed\0".as_ptr() as *const ::core::ffi::c_char,
            b"File not opened\0".as_ptr() as *const ::core::ffi::c_char,
            b"Write not started\0".as_ptr() as *const ::core::ffi::c_char,
            b"Wrong chunk version\0".as_ptr() as *const ::core::ffi::c_char,
            b"Chunk already exists\0".as_ptr() as *const ::core::ffi::c_char,
            b"No space left\0".as_ptr() as *const ::core::ffi::c_char,
            b"IO error\0".as_ptr() as *const ::core::ffi::c_char,
            b"Incorrect block number\0".as_ptr() as *const ::core::ffi::c_char,
            b"Incorrect size\0".as_ptr() as *const ::core::ffi::c_char,
            b"Incorrect offset\0".as_ptr() as *const ::core::ffi::c_char,
            b"Can't connect\0".as_ptr() as *const ::core::ffi::c_char,
            b"Incorrect chunk id\0".as_ptr() as *const ::core::ffi::c_char,
            b"Disconnected\0".as_ptr() as *const ::core::ffi::c_char,
            b"CRC error\0".as_ptr() as *const ::core::ffi::c_char,
            b"Operation delayed\0".as_ptr() as *const ::core::ffi::c_char,
            b"Can't create path\0".as_ptr() as *const ::core::ffi::c_char,
            b"Data mismatch\0".as_ptr() as *const ::core::ffi::c_char,
            b"Read-only file system\0".as_ptr() as *const ::core::ffi::c_char,
            b"Quota exceeded\0".as_ptr() as *const ::core::ffi::c_char,
            b"Bad session id\0".as_ptr() as *const ::core::ffi::c_char,
            b"Password is needed\0".as_ptr() as *const ::core::ffi::c_char,
            b"Incorrect password\0".as_ptr() as *const ::core::ffi::c_char,
            b"Attribute not found\0".as_ptr() as *const ::core::ffi::c_char,
            b"Operation not supported\0".as_ptr() as *const ::core::ffi::c_char,
            b"Result too large\0".as_ptr() as *const ::core::ffi::c_char,
            b"Entity not found\0".as_ptr() as *const ::core::ffi::c_char,
            b"Entity is active\0".as_ptr() as *const ::core::ffi::c_char,
            b"Chunkserver not present\0".as_ptr() as *const ::core::ffi::c_char,
            b"Waiting on lock\0".as_ptr() as *const ::core::ffi::c_char,
            b"Resource temporarily unavailable\0".as_ptr() as *const ::core::ffi::c_char,
            b"Interrupted system call\0".as_ptr() as *const ::core::ffi::c_char,
            b"Operation canceled\0".as_ptr() as *const ::core::ffi::c_char,
            b"No such file or directory (not cacheable)\0".as_ptr() as *const ::core::ffi::c_char,
            b"Operation not permitted (mfs admin only)\0".as_ptr() as *const ::core::ffi::c_char,
            b"Class name already in use\0".as_ptr() as *const ::core::ffi::c_char,
            b"Maximum number of classes reached\0".as_ptr() as *const ::core::ffi::c_char,
            b"No such class\0".as_ptr() as *const ::core::ffi::c_char,
            b"Class in use\0".as_ptr() as *const ::core::ffi::c_char,
            b"One of MFS instance components is too old to perform this operation\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"Pattern already defined\0".as_ptr() as *const ::core::ffi::c_char,
            b"Maximum number of patterns reached\0".as_ptr() as *const ::core::ffi::c_char,
            b"No such pattern\0".as_ptr() as *const ::core::ffi::c_char,
            b"File name too long\0".as_ptr() as *const ::core::ffi::c_char,
            b"Too many links\0".as_ptr() as *const ::core::ffi::c_char,
            b"Operation timed out\0".as_ptr() as *const ::core::ffi::c_char,
            b"Bad file descriptor\0".as_ptr() as *const ::core::ffi::c_char,
            b"File too large\0".as_ptr() as *const ::core::ffi::c_char,
            b"Is a directory\0".as_ptr() as *const ::core::ffi::c_char,
            b"Unknown MFS error\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        if status as ::core::ffi::c_int > MFS_ERROR_MAX {
            status = MFS_ERROR_MAX as uint8_t;
        }
        return errtab[status as usize];
    }
}
static mut bitcount_tab: [uint8_t; 256] = [
    0 as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (0 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    1 as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    1 as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    2 as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint8_t,
    (2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint8_t,
];
#[inline]
unsafe extern "C" fn bitcount(mut v: uint32_t) -> uint8_t {
    unsafe {
        return (bitcount_tab[(v & 0xff as uint32_t) as usize] as ::core::ffi::c_int
            + bitcount_tab[(v >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                as ::core::ffi::c_int
            + bitcount_tab[(v >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                as ::core::ffi::c_int
            + bitcount_tab[(v >> 24 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int)
            as uint8_t;
    }
}
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
pub const MaxPacketSize: ::core::ffi::c_int = CSTOMA_MAXPACKETSIZE;
pub const FULL_REPLICATION_WEIGHT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const EC_REPLICATION_WEIGHT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LOCALPART_REPLICATION_WEIGHT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NEWCHUNKDELAY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const LOSTCHUNKDELAY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SOMETHING_OVER_ANY_LIMIT: ::core::ffi::c_int = 10000000 as ::core::ffi::c_int;
static mut replreasons: [*const ::core::ffi::c_char; 17] = [
    b"COPY I/O\0".as_ptr() as *const ::core::ffi::c_char,
    b"COPY ENDANGERED\0".as_ptr() as *const ::core::ffi::c_char,
    b"COPY UNDERGOAL\0".as_ptr() as *const ::core::ffi::c_char,
    b"COPY WRONG LABEL\0".as_ptr() as *const ::core::ffi::c_char,
    b"COPY REBALANCE\0".as_ptr() as *const ::core::ffi::c_char,
    b"EC ENDANGERED\0".as_ptr() as *const ::core::ffi::c_char,
    b"EC UNDERGOAL\0".as_ptr() as *const ::core::ffi::c_char,
    b"EC WRONG LABEL\0".as_ptr() as *const ::core::ffi::c_char,
    b"EC REBALANCE\0".as_ptr() as *const ::core::ffi::c_char,
    b"LOCAL-SPLIT TO EC4\0".as_ptr() as *const ::core::ffi::c_char,
    b"LOCAL-SPLIT TO EC8\0".as_ptr() as *const ::core::ffi::c_char,
    b"JOIN EC I/O\0".as_ptr() as *const ::core::ffi::c_char,
    b"JOIN EC CHANGE\0".as_ptr() as *const ::core::ffi::c_char,
    b"JOIN EC NOT ENOUGH SERVERS\0".as_ptr() as *const ::core::ffi::c_char,
    b"JOIN EC GENERIC\0".as_ptr() as *const ::core::ffi::c_char,
    b"SPLIT EC GENERIC\0".as_ptr() as *const ::core::ffi::c_char,
    b"RECOVER EC I/O\0".as_ptr() as *const ::core::ffi::c_char,
];
static mut opreasons: [*const ::core::ffi::c_char; 4] = [
    b"GENERIC I/O\0".as_ptr() as *const ::core::ffi::c_char,
    b"DELETE INVALID\0".as_ptr() as *const ::core::ffi::c_char,
    b"DELETE NOT NEEDED\0".as_ptr() as *const ::core::ffi::c_char,
    b"DELETE OVERGOAL\0".as_ptr() as *const ::core::ffi::c_char,
];
static mut matocsservhead: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
static mut lsock: ::core::ffi::c_int = 0;
static mut lsockpdescpos: int32_t = 0;
static mut receivingchunks: uint8_t = 0;
static mut gusagediff: uint32_t = 0 as uint32_t;
static mut gtotalspace: uint64_t = 0 as uint64_t;
static mut gusedspace: uint64_t = 0 as uint64_t;
static mut gavailspace: uint64_t = 0 as uint64_t;
static mut gfreespace: uint64_t = 0 as uint64_t;
static mut valid_servers_count: uint16_t = 0;
static mut almostfull_servers_count: uint16_t = 0;
static mut replallowed_servers_count: uint16_t = 0;
static mut ListenHost: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut ListenPort: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut listenip: uint32_t = 0;
static mut listenport: uint16_t = 0;
static mut DefaultTimeout: uint32_t = 0;
static mut ForceTimeout: uint32_t = 0;
static mut AuthCode: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut RemapMask: uint32_t = 0 as uint32_t;
static mut RemapSrc: uint32_t = 0 as uint32_t;
static mut RemapDst: uint32_t = 0 as uint32_t;
static mut ReserveSpaceValue: ::core::ffi::c_double = 0.0f64;
static mut ReserveSpaceMode: uint8_t = RESERVE_BYTES as ::core::ffi::c_int as uint8_t;
static mut ChunkServerCheck: uint8_t = 0;
pub const OPHASHSIZE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub static mut optype_str: [*const ::core::ffi::c_char; 6] = [
    b"DELETE\0".as_ptr() as *const ::core::ffi::c_char,
    b"CREATE\0".as_ptr() as *const ::core::ffi::c_char,
    b"SETVERSION\0".as_ptr() as *const ::core::ffi::c_char,
    b"TRUNCATE\0".as_ptr() as *const ::core::ffi::c_char,
    b"DUPLICATE\0".as_ptr() as *const ::core::ffi::c_char,
    b"DUPTRUNC\0".as_ptr() as *const ::core::ffi::c_char,
];
static mut ophash: [*mut opsrv; 256] = [::core::ptr::null_mut::<opsrv>(); 256];
static mut opsrvfreehead: *mut opsrv = ::core::ptr::null_mut::<opsrv>();
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_opsrv_malloc() -> *mut opsrv {
    unsafe {
        let mut r: *mut opsrv = ::core::ptr::null_mut::<opsrv>();
        if !opsrvfreehead.is_null() {
            r = opsrvfreehead;
            opsrvfreehead = (*r).next as *mut opsrv;
        } else {
            r = malloc(::core::mem::size_of::<opsrv>()) as *mut opsrv;
            if r.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    269 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"r\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    269 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"r\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if r
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut opsrv
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    269 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"r\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    269 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"r\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
        }
        return r;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_opsrv_free(mut r: *mut opsrv) {
    unsafe {
        (*r).next = opsrvfreehead as *mut _opsrv;
        opsrvfreehead = r;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocssrv_operation_init() {
    unsafe {
        let mut hash: uint32_t = 0;
        hash = 0 as uint32_t;
        while hash < OPHASHSIZE as uint32_t {
            ophash[hash as usize] = ::core::ptr::null_mut::<opsrv>();
            hash = hash.wrapping_add(1);
        }
        opsrvfreehead = ::core::ptr::null_mut::<opsrv>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_operation_find(
    mut chunkid: uint64_t,
    mut srv: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut hash: uint32_t = (chunkid ^ chunkid >> 8 as ::core::ffi::c_int)
            .wrapping_rem(OPHASHSIZE as uint64_t) as uint32_t;
        let mut r: *mut opsrv = ::core::ptr::null_mut::<opsrv>();
        r = ophash[hash as usize];
        while !r.is_null() {
            if (*r).chunkid == chunkid && (*r).srv == srv {
                return 1 as ::core::ffi::c_int;
            }
            r = (*r).next as *mut opsrv;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_operation_begin(
    mut chunkid: uint64_t,
    mut version: uint32_t,
    mut srv: *mut ::core::ffi::c_void,
    mut optype: uint8_t,
    mut reason: uint8_t,
) {
    unsafe {
        let mut hash: uint32_t = (chunkid ^ chunkid >> 8 as ::core::ffi::c_int)
            .wrapping_rem(OPHASHSIZE as uint64_t) as uint32_t;
        let mut r: *mut opsrv = ::core::ptr::null_mut::<opsrv>();
        r = matocsserv_opsrv_malloc();
        (*r).chunkid = chunkid;
        (*r).version = version;
        (*r).srv = srv;
        (*r).optype = optype;
        (*r).reason = reason;
        (*r).next = ophash[hash as usize] as *mut _opsrv;
        ophash[hash as usize] = r;
        if optype as ::core::ffi::c_int == OPTYPE_DELETE as ::core::ffi::c_int {
            (*(srv as *mut matocsserventry)).delcounter =
                (*(srv as *mut matocsserventry)).delcounter.wrapping_add(1);
            (*(srv as *mut matocsserventry)).del_total_counter = (*(srv as *mut matocsserventry))
                .del_total_counter
                .wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_operation_end(
    mut chunkid: uint64_t,
    mut srv: *mut ::core::ffi::c_void,
    mut ok: uint8_t,
) {
    unsafe {
        let mut hash: uint32_t = (chunkid ^ chunkid >> 8 as ::core::ffi::c_int)
            .wrapping_rem(OPHASHSIZE as uint64_t) as uint32_t;
        let mut r: *mut opsrv = ::core::ptr::null_mut::<opsrv>();
        let mut rp: *mut *mut opsrv = ::core::ptr::null_mut::<*mut opsrv>();
        rp = (&raw mut ophash as *mut *mut opsrv).offset(hash as isize);
        loop {
            r = *rp;
            if r.is_null() {
                break;
            }
            if (*r).chunkid == chunkid && (*r).srv == srv {
                if (*r).optype as ::core::ffi::c_int == OPTYPE_DELETE as ::core::ffi::c_int {
                    (*(srv as *mut matocsserventry)).delcounter =
                        (*(srv as *mut matocsserventry)).delcounter.wrapping_sub(1);
                    if ok != 0 {
                        (*(srv as *mut matocsserventry)).delok[(*r).reason as usize] =
                            (*(srv as *mut matocsserventry)).delok[(*r).reason as usize]
                                .wrapping_add(1);
                    } else {
                        (*(srv as *mut matocsserventry)).delerr[(*r).reason as usize] =
                            (*(srv as *mut matocsserventry)).delerr[(*r).reason as usize]
                                .wrapping_add(1);
                    }
                }
                *rp = (*r).next as *mut opsrv;
                matocsserv_opsrv_free(r);
            } else {
                rp = &raw mut (*r).next as *mut *mut opsrv;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_operation_info(mut fd: *mut FILE) {
    unsafe {
        let mut hash: uint32_t = 0;
        let mut r: *mut opsrv = ::core::ptr::null_mut::<opsrv>();
        let mut e: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        hash = 0 as uint32_t;
        while hash < OPHASHSIZE as uint32_t {
            r = ophash[hash as usize];
            while !r.is_null() {
                fprintf(
                    fd,
                    b"operation %s : chunk %lX_%X ; reason: %s ; server: \0".as_ptr()
                        as *const ::core::ffi::c_char,
                    optype_str[(*r).optype as usize],
                    (*r).chunkid,
                    (*r).version,
                    opreasons[(*r).reason as usize],
                );
                e = (*r).srv as *mut matocsserventry;
                fprintf(
                    fd,
                    b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    (*e).servdesc,
                );
                r = (*r).next as *mut opsrv;
            }
            hash = hash.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_operation_disconnected(mut srv: *mut ::core::ffi::c_void) {
    unsafe {
        let mut hash: uint32_t = 0;
        let mut r: *mut opsrv = ::core::ptr::null_mut::<opsrv>();
        let mut rp: *mut *mut opsrv = ::core::ptr::null_mut::<*mut opsrv>();
        hash = 0 as uint32_t;
        while hash < OPHASHSIZE as uint32_t {
            rp = (&raw mut ophash as *mut *mut opsrv).offset(hash as isize);
            loop {
                r = *rp;
                if r.is_null() {
                    break;
                }
                if (*r).srv == srv {
                    if (*r).optype as ::core::ffi::c_int == OPTYPE_DELETE as ::core::ffi::c_int {
                        (*(srv as *mut matocsserventry)).delcounter =
                            (*(srv as *mut matocsserventry)).delcounter.wrapping_sub(1);
                        (*(srv as *mut matocsserventry)).delerr[(*r).reason as usize] =
                            (*(srv as *mut matocsserventry)).delerr[(*r).reason as usize]
                                .wrapping_add(1);
                    }
                    *rp = (*r).next as *mut opsrv;
                    matocsserv_opsrv_free(r);
                } else {
                    rp = &raw mut (*r).next as *mut *mut opsrv;
                }
            }
            hash = hash.wrapping_add(1);
        }
    }
}
pub const REPHASHSIZE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub static mut reptype_str: [*const ::core::ffi::c_char; 5] = [
    b"SIMPLE\0".as_ptr() as *const ::core::ffi::c_char,
    b"SPLIT\0".as_ptr() as *const ::core::ffi::c_char,
    b"RECOVER\0".as_ptr() as *const ::core::ffi::c_char,
    b"JOIN\0".as_ptr() as *const ::core::ffi::c_char,
    b"LOCALSPLIT\0".as_ptr() as *const ::core::ffi::c_char,
];
static mut rephash: [*mut repdst; 256] = [::core::ptr::null_mut::<repdst>(); 256];
static mut repsrcfreehead: *mut repsrc = ::core::ptr::null_mut::<repsrc>();
static mut repdstfreehead: *mut repdst = ::core::ptr::null_mut::<repdst>();
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_repsrc_malloc() -> *mut repsrc {
    unsafe {
        let mut r: *mut repsrc = ::core::ptr::null_mut::<repsrc>();
        if !repsrcfreehead.is_null() {
            r = repsrcfreehead;
            repsrcfreehead = (*r).next as *mut repsrc;
        } else {
            r = malloc(::core::mem::size_of::<repsrc>()) as *mut repsrc;
            if r.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    422 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"r\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    422 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"r\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if r
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut repsrc
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    422 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"r\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    422 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"r\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
        }
        return r;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_repsrc_free(mut r: *mut repsrc) {
    unsafe {
        (*r).next = repsrcfreehead as *mut _repsrc;
        repsrcfreehead = r;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_repdst_malloc() -> *mut repdst {
    unsafe {
        let mut r: *mut repdst = ::core::ptr::null_mut::<repdst>();
        if !repdstfreehead.is_null() {
            r = repdstfreehead;
            repdstfreehead = (*r).next as *mut repdst;
        } else {
            r = malloc(::core::mem::size_of::<repdst>()) as *mut repdst;
            if r.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    439 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"r\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    439 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"r\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if r
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut repdst
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    439 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"r\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    439 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"r\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
        }
        return r;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_repdst_free(mut r: *mut repdst) {
    unsafe {
        (*r).next = repdstfreehead as *mut _repdst;
        repdstfreehead = r;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_replication_init() {
    unsafe {
        let mut hash: uint32_t = 0;
        hash = 0 as uint32_t;
        while hash < REPHASHSIZE as uint32_t {
            rephash[hash as usize] = ::core::ptr::null_mut::<repdst>();
            hash = hash.wrapping_add(1);
        }
        repsrcfreehead = ::core::ptr::null_mut::<repsrc>();
        repdstfreehead = ::core::ptr::null_mut::<repdst>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_replication_find(
    mut chunkid: uint64_t,
    mut version: uint32_t,
    mut dst: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut hash: uint32_t =
            (chunkid ^ version as uint64_t ^ chunkid >> 8 as ::core::ffi::c_int)
                .wrapping_rem(REPHASHSIZE as uint64_t) as uint32_t;
        let mut r: *mut repdst = ::core::ptr::null_mut::<repdst>();
        r = rephash[hash as usize];
        while !r.is_null() {
            if (*r).chunkid == chunkid && (*r).version == version && (*r).dst == dst {
                return 1 as ::core::ffi::c_int;
            }
            r = (*r).next as *mut repdst;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_replication_begin(
    mut chunkid: uint64_t,
    mut version: uint32_t,
    mut dst: *mut ::core::ffi::c_void,
    mut srccnt: uint8_t,
    mut src: *mut *mut ::core::ffi::c_void,
    mut rweight: uint8_t,
    mut wweight: uint8_t,
    mut reptype: uint8_t,
    mut reason: uint8_t,
) {
    unsafe {
        let mut hash: uint32_t =
            (chunkid ^ version as uint64_t ^ chunkid >> 8 as ::core::ffi::c_int)
                .wrapping_rem(REPHASHSIZE as uint64_t) as uint32_t;
        let mut i: uint8_t = 0;
        let mut r: *mut repdst = ::core::ptr::null_mut::<repdst>();
        let mut rs: *mut repsrc = ::core::ptr::null_mut::<repsrc>();
        if srccnt as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            r = matocsserv_repdst_malloc();
            (*r).chunkid = chunkid;
            (*r).version = version;
            (*r).dst = dst;
            (*r).rweight = rweight;
            (*r).wweight = wweight;
            (*r).reptype = reptype;
            (*r).reason = reason;
            (*r).srchead = ::core::ptr::null_mut::<repsrc>();
            (*r).next = rephash[hash as usize] as *mut _repdst;
            rephash[hash as usize] = r;
            i = 0 as uint8_t;
            while (i as ::core::ffi::c_int) < srccnt as ::core::ffi::c_int {
                rs = matocsserv_repsrc_malloc();
                (*rs).src = *src.offset(i as isize);
                (*rs).next = (*r).srchead as *mut _repsrc;
                (*r).srchead = rs;
                (*(*src.offset(i as isize) as *mut matocsserventry)).rrepcounter =
                    ((*(*src.offset(i as isize) as *mut matocsserventry)).rrepcounter
                        as ::core::ffi::c_int
                        + rweight as ::core::ffi::c_int) as uint16_t;
                (*(*src.offset(i as isize) as *mut matocsserventry)).rrep_total_counter =
                    (*(*src.offset(i as isize) as *mut matocsserventry))
                        .rrep_total_counter
                        .wrapping_add(1);
                i = i.wrapping_add(1);
            }
            (*(dst as *mut matocsserventry)).wrepcounter =
                ((*(dst as *mut matocsserventry)).wrepcounter as ::core::ffi::c_int
                    + wweight as ::core::ffi::c_int) as uint16_t;
            (*(dst as *mut matocsserventry)).wrep_total_counter = (*(dst as *mut matocsserventry))
                .wrep_total_counter
                .wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_replication_end(
    mut chunkid: uint64_t,
    mut version: uint32_t,
    mut dst: *mut ::core::ffi::c_void,
    mut ok: uint8_t,
) {
    unsafe {
        let mut hash: uint32_t =
            (chunkid ^ version as uint64_t ^ chunkid >> 8 as ::core::ffi::c_int)
                .wrapping_rem(REPHASHSIZE as uint64_t) as uint32_t;
        let mut r: *mut repdst = ::core::ptr::null_mut::<repdst>();
        let mut rp: *mut *mut repdst = ::core::ptr::null_mut::<*mut repdst>();
        let mut rs: *mut repsrc = ::core::ptr::null_mut::<repsrc>();
        let mut rsdel: *mut repsrc = ::core::ptr::null_mut::<repsrc>();
        rp = (&raw mut rephash as *mut *mut repdst).offset(hash as isize);
        loop {
            r = *rp;
            if r.is_null() {
                break;
            }
            if (*r).chunkid == chunkid && (*r).version == version && (*r).dst == dst {
                rs = (*r).srchead;
                while !rs.is_null() {
                    rsdel = rs;
                    rs = (*rs).next as *mut repsrc;
                    (*((*rsdel).src as *mut matocsserventry)).rrepcounter =
                        ((*((*rsdel).src as *mut matocsserventry)).rrepcounter
                            as ::core::ffi::c_int
                            - (*r).rweight as ::core::ffi::c_int)
                            as uint16_t;
                    if ok != 0 {
                        (*((*rsdel).src as *mut matocsserventry)).replreadok
                            [(*r).reason as usize] = (*((*rsdel).src as *mut matocsserventry))
                            .replreadok[(*r).reason as usize]
                            .wrapping_add((*r).rweight as uint32_t);
                    } else {
                        (*((*rsdel).src as *mut matocsserventry)).replreaderr
                            [(*r).reason as usize] = (*((*rsdel).src as *mut matocsserventry))
                            .replreaderr[(*r).reason as usize]
                            .wrapping_add((*r).rweight as uint32_t);
                    }
                    matocsserv_repsrc_free(rsdel);
                }
                (*(dst as *mut matocsserventry)).wrepcounter =
                    ((*(dst as *mut matocsserventry)).wrepcounter as ::core::ffi::c_int
                        - (*r).wweight as ::core::ffi::c_int) as uint16_t;
                if ok != 0 {
                    (*(dst as *mut matocsserventry)).replwriteok[(*r).reason as usize] =
                        (*(dst as *mut matocsserventry)).replwriteok[(*r).reason as usize]
                            .wrapping_add((*r).wweight as uint32_t);
                } else {
                    (*(dst as *mut matocsserventry)).replwriteerr[(*r).reason as usize] =
                        (*(dst as *mut matocsserventry)).replwriteerr[(*r).reason as usize]
                            .wrapping_add((*r).wweight as uint32_t);
                }
                *rp = (*r).next as *mut repdst;
                matocsserv_repdst_free(r);
            } else {
                rp = &raw mut (*r).next as *mut *mut repdst;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_replication_print(
    mut buff: *mut ::core::ffi::c_char,
    mut bleng: uint32_t,
    mut chunkid: uint64_t,
    mut version: uint32_t,
    mut dst: *mut ::core::ffi::c_void,
    mut reptype: *mut uint8_t,
) -> uint32_t {
    unsafe {
        let mut hash: uint32_t =
            (chunkid ^ version as uint64_t ^ chunkid >> 8 as ::core::ffi::c_int)
                .wrapping_rem(REPHASHSIZE as uint64_t) as uint32_t;
        let mut r: *mut repdst = ::core::ptr::null_mut::<repdst>();
        let mut rs: *mut repsrc = ::core::ptr::null_mut::<repsrc>();
        let mut leng: uint32_t = 0;
        leng = 0 as uint32_t;
        *reptype = 0 as uint8_t;
        r = rephash[hash as usize];
        while !r.is_null() {
            if (*r).chunkid == chunkid && (*r).version == version && (*r).dst == dst {
                rs = (*r).srchead;
                while !rs.is_null() {
                    if leng > 0 as uint32_t && leng < bleng {
                        let c2rust_fresh1 = leng;
                        leng = leng.wrapping_add(1);
                        *buff.offset(c2rust_fresh1 as isize) = ',' as ::core::ffi::c_char;
                    }
                    if leng < bleng {
                        leng = leng.wrapping_add(snprintf(
                            buff.offset(leng as isize),
                            bleng.wrapping_sub(leng) as size_t,
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            (*((*rs).src as *mut matocsserventry)).servdesc,
                        ) as uint32_t);
                    }
                    rs = (*rs).next as *mut repsrc;
                }
                if leng < bleng {
                    leng = leng.wrapping_add(snprintf(
                        buff.offset(leng as isize),
                        bleng.wrapping_sub(leng) as size_t,
                        b" -> %s\0".as_ptr() as *const ::core::ffi::c_char,
                        (*(dst as *mut matocsserventry)).servdesc,
                    ) as uint32_t);
                }
                *reptype = (*r).reptype;
                return leng;
            }
            r = (*r).next as *mut repdst;
        }
        return 0 as uint32_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_replication_info(mut fd: *mut FILE) {
    unsafe {
        let mut hash: uint32_t = 0;
        let mut r: *mut repdst = ::core::ptr::null_mut::<repdst>();
        let mut rs: *mut repsrc = ::core::ptr::null_mut::<repsrc>();
        let mut e: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        hash = 0 as uint32_t;
        while hash < REPHASHSIZE as uint32_t {
            r = rephash[hash as usize];
            while !r.is_null() {
                fprintf(
                    fd,
                    b"operation REPLICATE_%s : chunk %lX_%X ; reason: %s ; servers: \0".as_ptr()
                        as *const ::core::ffi::c_char,
                    reptype_str[(*r).reptype as usize],
                    (*r).chunkid,
                    (*r).version,
                    replreasons[(*r).reason as usize],
                );
                rs = (*r).srchead;
                while !rs.is_null() {
                    e = (*rs).src as *mut matocsserventry;
                    fprintf(
                        fd,
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        (*e).servdesc,
                    );
                    if !(*rs).next.is_null() {
                        fprintf(fd, b",\0".as_ptr() as *const ::core::ffi::c_char);
                    }
                    rs = (*rs).next as *mut repsrc;
                }
                e = (*r).dst as *mut matocsserventry;
                fprintf(
                    fd,
                    b" -> %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    (*e).servdesc,
                );
                r = (*r).next as *mut repdst;
            }
            hash = hash.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_replication_disconnected(mut srv: *mut ::core::ffi::c_void) {
    unsafe {
        let mut hash: uint32_t = 0;
        let mut r: *mut repdst = ::core::ptr::null_mut::<repdst>();
        let mut rp: *mut *mut repdst = ::core::ptr::null_mut::<*mut repdst>();
        let mut rs: *mut repsrc = ::core::ptr::null_mut::<repsrc>();
        let mut rsdel: *mut repsrc = ::core::ptr::null_mut::<repsrc>();
        let mut rsp: *mut *mut repsrc = ::core::ptr::null_mut::<*mut repsrc>();
        hash = 0 as uint32_t;
        while hash < REPHASHSIZE as uint32_t {
            rp = (&raw mut rephash as *mut *mut repdst).offset(hash as isize);
            loop {
                r = *rp;
                if r.is_null() {
                    break;
                }
                if (*r).dst == srv {
                    rs = (*r).srchead;
                    while !rs.is_null() {
                        rsdel = rs;
                        rs = (*rs).next as *mut repsrc;
                        (*((*rsdel).src as *mut matocsserventry)).rrepcounter =
                            ((*((*rsdel).src as *mut matocsserventry)).rrepcounter
                                as ::core::ffi::c_int
                                - (*r).rweight as ::core::ffi::c_int)
                                as uint16_t;
                        (*((*rsdel).src as *mut matocsserventry)).replreaderr
                            [(*r).reason as usize] = (*((*rsdel).src as *mut matocsserventry))
                            .replreaderr[(*r).reason as usize]
                            .wrapping_add((*r).rweight as uint32_t);
                        matocsserv_repsrc_free(rsdel);
                    }
                    (*(srv as *mut matocsserventry)).wrepcounter =
                        ((*(srv as *mut matocsserventry)).wrepcounter as ::core::ffi::c_int
                            - (*r).wweight as ::core::ffi::c_int)
                            as uint16_t;
                    (*(srv as *mut matocsserventry)).replwriteerr[(*r).reason as usize] =
                        (*(srv as *mut matocsserventry)).replwriteerr[(*r).reason as usize]
                            .wrapping_add((*r).wweight as uint32_t);
                    *rp = (*r).next as *mut repdst;
                    matocsserv_repdst_free(r);
                } else {
                    rsp = &raw mut (*r).srchead;
                    loop {
                        rs = *rsp;
                        if rs.is_null() {
                            break;
                        }
                        if (*rs).src == srv {
                            (*(srv as *mut matocsserventry)).rrepcounter =
                                ((*(srv as *mut matocsserventry)).rrepcounter as ::core::ffi::c_int
                                    - (*r).rweight as ::core::ffi::c_int)
                                    as uint16_t;
                            (*(srv as *mut matocsserventry)).replreaderr[(*r).reason as usize] =
                                (*(srv as *mut matocsserventry)).replreaderr[(*r).reason as usize]
                                    .wrapping_add((*r).rweight as uint32_t);
                            *rsp = (*rs).next as *mut repsrc;
                            matocsserv_repsrc_free(rs);
                        } else {
                            rsp = &raw mut (*rs).next as *mut *mut repsrc;
                        }
                    }
                    rp = &raw mut (*r).next as *mut *mut repdst;
                }
            }
            hash = hash.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn matocsserv_ecid_to_str(mut ecid: uint8_t) -> *const ::core::ffi::c_char {
    let mut ecid8names: [*const ::core::ffi::c_char; 17] = [
        b" (DE0)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (DE1)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (DE2)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (DE3)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (DE4)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (DE5)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (DE6)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (DE7)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (CE0)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (CE1)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (CE2)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (CE3)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (CE4)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (CE5)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (CE6)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (CE7)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (CE8)\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut ecid4names: [*const ::core::ffi::c_char; 13] = [
        b" (DF0)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (DF1)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (DF2)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (DF3)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (CF0)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (CF1)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (CF2)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (CF3)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (CF4)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (CF5)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (CF6)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (CF7)\0".as_ptr() as *const ::core::ffi::c_char,
        b" (CF8)\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    if ecid as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int != 0 {
        if (ecid as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int) < 17 as ::core::ffi::c_int {
            return ecid8names[(ecid as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int) as usize];
        }
    } else if ecid as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int != 0 {
        if (ecid as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) < 13 as ::core::ffi::c_int {
            return ecid4names[(ecid as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as usize];
        }
    } else if ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return b" (COPY)\0".as_ptr() as *const ::core::ffi::c_char;
    }
    return b" (???)\0".as_ptr() as *const ::core::ffi::c_char;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_check_password(
    mut rndcode: *const uint8_t,
    mut csdigest: *const uint8_t,
) -> uint8_t {
    unsafe {
        let mut md5c: md5ctx = md5ctx {
            state: [0; 4],
            count: [0; 2],
            buffer: [0; 64],
        };
        let mut digest: [uint8_t; 16] = [0; 16];
        md5_init(&raw mut md5c);
        md5_update(&raw mut md5c, rndcode as *const uint8_t, 16 as uint32_t);
        md5_update(
            &raw mut md5c,
            AuthCode as *const uint8_t,
            strlen(AuthCode) as uint32_t,
        );
        md5_update(
            &raw mut md5c,
            rndcode.offset(16 as ::core::ffi::c_int as isize),
            16 as uint32_t,
        );
        md5_final(&raw mut digest as *mut uint8_t, &raw mut md5c);
        if memcmp(
            &raw mut digest as *mut uint8_t as *const ::core::ffi::c_void,
            csdigest as *const ::core::ffi::c_void,
            16 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            return 1 as uint8_t;
        }
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_log_extra_info(mut fd: *mut FILE) {
    unsafe {
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        let mut dur: ::core::ffi::c_double = 0.;
        let mut usage: ::core::ffi::c_double = 0.;
        let mut hlstatus_name: *const ::core::ffi::c_char =
            ::core::ptr::null::<::core::ffi::c_char>();
        let mut overloaded: uint8_t = 0;
        let mut maintained: uint8_t = 0;
        let mut i: uint8_t = 0;
        let mut now: uint32_t = main_time();
        eptr = matocsservhead;
        while !eptr.is_null() {
            if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                && !(*eptr).csptr.is_null()
            {
                fprintf(
                    fd,
                    b"[chunkserver %s]\n\0".as_ptr() as *const ::core::ffi::c_char,
                    (*eptr).servdesc,
                );
                dur = monotonic_seconds() - (*eptr).total_counter_begin;
                if dur < 1.0f64 {
                    dur = 1.0f64;
                }
                overloaded =
                    (if csdb_server_is_overloaded((*eptr).csptr as *mut ::core::ffi::c_void, now)
                        as ::core::ffi::c_int
                        != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint8_t;
                maintained =
                    (if csdb_server_is_being_maintained((*eptr).csptr as *mut ::core::ffi::c_void)
                        as ::core::ffi::c_int
                        != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint8_t;
                match (*eptr).hlstatus as ::core::ffi::c_int {
                    HLSTATUS_DEFAULT => {
                        hlstatus_name = b"DEFAULT\0".as_ptr() as *const ::core::ffi::c_char;
                    }
                    HLSTATUS_OK => {
                        hlstatus_name = b"OK\0".as_ptr() as *const ::core::ffi::c_char;
                    }
                    HLSTATUS_OVERLOADED => {
                        hlstatus_name = b"OVERLOADED\0".as_ptr() as *const ::core::ffi::c_char;
                    }
                    HLSTATUS_LSREBALANCE => {
                        hlstatus_name = b"LSREBALANCE\0".as_ptr() as *const ::core::ffi::c_char;
                    }
                    HLSTATUS_GRACEFUL => {
                        hlstatus_name = b"GRACEFUL\0".as_ptr() as *const ::core::ffi::c_char;
                    }
                    HLSTATUS_HSREBALANCE => {
                        hlstatus_name = b"HSREBALANCE\0".as_ptr() as *const ::core::ffi::c_char;
                    }
                    _ => {
                        hlstatus_name = b"UNKNOWN\0".as_ptr() as *const ::core::ffi::c_char;
                    }
                }
                if (*eptr).totalspace > 0 as uint64_t && (*eptr).usedspace <= (*eptr).totalspace {
                    usage = 100.0f64 * (*eptr).usedspace as ::core::ffi::c_double
                        / (*eptr).totalspace as ::core::ffi::c_double;
                } else if (*eptr).totalspace > 0 as uint64_t {
                    usage = 100.0f64;
                } else {
                    usage = 0.0f64;
                }
                fprintf(
                    fd,
                    b"usedspace: %lu\ntotalspace: %lu\nusage: %.2lf%%\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*eptr).usedspace,
                    (*eptr).totalspace,
                    usage,
                );
                fprintf(
                    fd,
                    b"load: %u\ntimeout: %hu\nchunkscount: %u\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*eptr).load,
                    (*eptr).timeout as ::core::ffi::c_int,
                    (*eptr).chunkscount,
                );
                fprintf(
                    fd,
                    b"errorcounter: %u\nwritecounter: %hu\nrrepcounter: %.3lf\nwrepcounter: %.3lf\ndelcounter: %u\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    (*eptr).errorcounter,
                    (*eptr).writecounter as ::core::ffi::c_int,
                    (*eptr).rrepcounter as ::core::ffi::c_int as ::core::ffi::c_double
                        / FULL_REPLICATION_WEIGHT as ::core::ffi::c_double,
                    (*eptr).wrepcounter as ::core::ffi::c_int as ::core::ffi::c_double
                        / FULL_REPLICATION_WEIGHT as ::core::ffi::c_double,
                    (*eptr).delcounter as ::core::ffi::c_int,
                );
                fprintf(
                    fd,
                    b"create_total: %u\nrrep_total: %u\nwrep_total: %u\ndel_total: %u\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*eptr).create_total_counter,
                    (*eptr).rrep_total_counter,
                    (*eptr).wrep_total_counter,
                    (*eptr).del_total_counter,
                );
                fprintf(
                    fd,
                    b"create/s: %.4lf\nrrep/s: %.4lf\nwrep/s: %.4lf\ndel/s: %.4lf\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*eptr).create_total_counter as ::core::ffi::c_double / dur,
                    (*eptr).rrep_total_counter as ::core::ffi::c_double / dur,
                    (*eptr).wrep_total_counter as ::core::ffi::c_double / dur,
                    (*eptr).del_total_counter as ::core::ffi::c_double / dur,
                );
                fprintf(
                    fd,
                    b"csid: %hu\ndist: %u\nfirst: %hhu\ncorr: %.4lf\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*eptr).csid as ::core::ffi::c_int,
                    (*eptr).dist,
                    (*eptr).first as ::core::ffi::c_int,
                    (*eptr).corr,
                );
                fprintf(
                    fd,
                    b"hlstatus: %hhu (%s)\noverloaded: %hhu\nmaintained: %hhu\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*eptr).hlstatus as ::core::ffi::c_int,
                    hlstatus_name,
                    overloaded as ::core::ffi::c_int,
                    maintained as ::core::ffi::c_int,
                );
                (*eptr).create_total_counter = 0 as uint32_t;
                (*eptr).rrep_total_counter = 0 as uint32_t;
                (*eptr).wrep_total_counter = 0 as uint32_t;
                (*eptr).del_total_counter = 0 as uint32_t;
                (*eptr).total_counter_begin = monotonic_seconds();
                fprintf(fd, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
            }
            eptr = (*eptr).next as *mut matocsserventry;
        }
        fprintf(
            fd,
            b"[replications/deletions stats]\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        eptr = matocsservhead;
        while !eptr.is_null() {
            if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                && !(*eptr).csptr.is_null()
            {
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int) < REPL_REASONS as ::core::ffi::c_int {
                    if (*eptr).lreplreadok[i as usize] != 0 || (*eptr).lreplreaderr[i as usize] != 0
                    {
                        fprintf(
                            fd,
                            b"cs %s ; replication source ; reason: %s ; ok/err: %.3lf/%.3lf\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            (*eptr).servdesc,
                            replreasons[i as usize],
                            (*eptr).lreplreadok[i as usize] as ::core::ffi::c_double
                                / FULL_REPLICATION_WEIGHT as ::core::ffi::c_double,
                            (*eptr).lreplreaderr[i as usize] as ::core::ffi::c_double
                                / FULL_REPLICATION_WEIGHT as ::core::ffi::c_double,
                        );
                    }
                    if (*eptr).lreplwriteok[i as usize] != 0
                        || (*eptr).lreplwriteerr[i as usize] != 0
                    {
                        fprintf(
                            fd,
                            b"cs %s ; replication target ; reason: %s ; ok/err: %.3lf/%.3lf\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            (*eptr).servdesc,
                            replreasons[i as usize],
                            (*eptr).lreplwriteok[i as usize] as ::core::ffi::c_double
                                / FULL_REPLICATION_WEIGHT as ::core::ffi::c_double,
                            (*eptr).lreplwriteerr[i as usize] as ::core::ffi::c_double
                                / FULL_REPLICATION_WEIGHT as ::core::ffi::c_double,
                        );
                    }
                    i = i.wrapping_add(1);
                }
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int) < OP_REASONS as ::core::ffi::c_int {
                    if (*eptr).ldelok[i as usize] != 0 || (*eptr).ldelerr[i as usize] != 0 {
                        fprintf(
                            fd,
                            b"cs %s ; deletion ; reason: %s ; ok/err: %u/%u\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*eptr).servdesc,
                            opreasons[i as usize],
                            (*eptr).ldelok[i as usize],
                            (*eptr).ldelerr[i as usize],
                        );
                    }
                    i = i.wrapping_add(1);
                }
            }
            eptr = (*eptr).next as *mut matocsserventry;
        }
        fprintf(fd, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        fprintf(
            fd,
            b"[pending operations]\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        matocsserv_replication_info(fd);
        matocsserv_operation_info(fd);
        fprintf(fd, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_space_compare(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aa: *const servsort_0 = a as *const servsort_0;
        let mut bb: *const servsort_0 = b as *const servsort_0;
        if (*aa).space > (*bb).space {
            return 1 as ::core::ffi::c_int;
        }
        if (*aa).space < (*bb).space {
            return -1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_getservers_ordered(mut csids: *mut uint16_t) -> uint16_t {
    unsafe {
        static mut servtab: [servsort; 10000] = [servsort { space: 0., csid: 0 }; 10000];
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        let mut i: uint32_t = 0;
        let mut scnt: uint32_t = 0;
        scnt = 0 as uint32_t;
        eptr = matocsservhead;
        while !eptr.is_null() && scnt < MAXCSCOUNT as uint32_t {
            if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                && (*eptr).totalspace > 0 as uint64_t
                && (*eptr).usedspace <= (*eptr).totalspace
                && !(*eptr).csptr.is_null()
                && ((*eptr).hlstatus as ::core::ffi::c_int == HLSTATUS_DEFAULT
                    || (*eptr).hlstatus as ::core::ffi::c_int == HLSTATUS_OK
                    || (*eptr).hlstatus as ::core::ffi::c_int == HLSTATUS_LSREBALANCE)
                && csdb_server_is_being_maintained((*eptr).csptr as *mut ::core::ffi::c_void)
                    as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                servtab[scnt as usize].csid = (*eptr).csid;
                servtab[scnt as usize].space = (*eptr).usedspace as ::core::ffi::c_double
                    / (*eptr).totalspace as ::core::ffi::c_double;
                scnt = scnt.wrapping_add(1);
            }
            eptr = (*eptr).next as *mut matocsserventry;
        }
        if scnt == 0 as uint32_t {
            return 0 as uint16_t;
        }
        qsort(
            &raw mut servtab as *mut servsort as *mut ::core::ffi::c_void,
            scnt as size_t,
            ::core::mem::size_of::<servsort>(),
            Some(
                matocsserv_space_compare
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
        i = 0 as uint32_t;
        while i < scnt {
            *csids.offset(i as isize) = servtab[i as usize].csid;
            i = i.wrapping_add(1);
        }
        return scnt as uint16_t;
    }
}
unsafe extern "C" fn matocsserv_err_compare(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aa: *const rservsort_0 = a as *const rservsort_0;
        let mut bb: *const rservsort_0 = b as *const rservsort_0;
        if (*aa).err < (*bb).err {
            return -1 as ::core::ffi::c_int;
        }
        if (*aa).err > (*bb).err {
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn matocsserv_weighted_roundrobin_sort(
    mut servers: *mut *mut matocsserventry,
    mut cnt: uint32_t,
) {
    unsafe {
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        let mut expdist: ::core::ffi::c_double = 0.;
        let mut i: uint32_t = 0;
        let mut totalspace: uint64_t = 0;
        static mut servtab: [rservsort; 10000] = [rservsort {
            err: 0.,
            ptr: ::core::ptr::null_mut::<matocsserventry>(),
        }; 10000];
        totalspace = 0 as uint64_t;
        eptr = matocsservhead;
        while !eptr.is_null() {
            if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                && (*eptr).totalspace > 0 as uint64_t
                && (*eptr).usedspace <= (*eptr).totalspace
                && !(*eptr).csptr.is_null()
            {
                totalspace = totalspace.wrapping_add((*eptr).totalspace);
            }
            eptr = (*eptr).next as *mut matocsserventry;
        }
        i = 0 as uint32_t;
        while i < cnt {
            if (**servers.offset(i as isize)).first != 0 {
                servtab[i as usize].err = 1.0f64;
            } else {
                expdist = totalspace as ::core::ffi::c_double;
                expdist /= (**servers.offset(i as isize)).totalspace as ::core::ffi::c_double;
                servtab[i as usize].err = (expdist + (**servers.offset(i as isize)).corr)
                    / (**servers.offset(i as isize))
                        .dist
                        .wrapping_add(1 as uint32_t) as ::core::ffi::c_double;
            }
            servtab[i as usize].err += 1000.0f64
                * ((**servers.offset(i as isize)).writecounter as ::core::ffi::c_int
                    as ::core::ffi::c_double
                    + (**servers.offset(i as isize)).wrepcounter as ::core::ffi::c_int
                        as ::core::ffi::c_double
                        / FULL_REPLICATION_WEIGHT as ::core::ffi::c_double);
            servtab[i as usize].ptr = *servers.offset(i as isize);
            i = i.wrapping_add(1);
        }
        qsort(
            &raw mut servtab as *mut rservsort as *mut ::core::ffi::c_void,
            cnt as size_t,
            ::core::mem::size_of::<rservsort>(),
            Some(
                matocsserv_err_compare
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
        i = 0 as uint32_t;
        while i < cnt {
            *servers.offset(i as isize) = servtab[i as usize].ptr;
            i = i.wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C" fn matocsserv_weighted_roundrobin_used(
    mut servers: *mut *mut matocsserventry,
    mut cnt: uint32_t,
) {
    unsafe {
        static mut fcnt: uint32_t = 0 as uint32_t;
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        let mut expdist: ::core::ffi::c_double = 0.;
        let mut dist: ::core::ffi::c_double = 0.;
        let mut i: uint32_t = 0;
        let mut totalcnt: uint32_t = 0;
        let mut totalspace: uint64_t = 0;
        totalspace = 0 as uint64_t;
        totalcnt = 0 as uint32_t;
        eptr = matocsservhead;
        while !eptr.is_null() {
            if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                && (*eptr).totalspace > 0 as uint64_t
                && (*eptr).usedspace <= (*eptr).totalspace
                && !(*eptr).csptr.is_null()
            {
                totalspace = totalspace.wrapping_add((*eptr).totalspace);
                totalcnt = totalcnt.wrapping_add(1);
                (*eptr).dist = (*eptr).dist.wrapping_add(cnt);
            }
            eptr = (*eptr).next as *mut matocsserventry;
        }
        fcnt = fcnt.wrapping_add(cnt);
        if fcnt > totalcnt.wrapping_mul(10 as uint32_t) {
            fcnt = 0 as uint32_t;
            eptr = matocsservhead;
            while !eptr.is_null() {
                if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                    && (*eptr).totalspace > 0 as uint64_t
                    && (*eptr).usedspace <= (*eptr).totalspace
                    && !(*eptr).csptr.is_null()
                {
                    dist = totalspace as ::core::ffi::c_double;
                    dist /= (*eptr).totalspace as ::core::ffi::c_double;
                    (*eptr).dist = rndu32_ranged(
                        (dist * 1000 as ::core::ffi::c_int as ::core::ffi::c_double) as uint32_t,
                    )
                    .wrapping_div(1000 as uint32_t);
                    (*eptr).corr = 0.0f64;
                }
                eptr = (*eptr).next as *mut matocsserventry;
            }
            i = 0 as uint32_t;
            while i < cnt {
                (**servers.offset(i as isize)).create_total_counter = (**servers
                    .offset(i as isize))
                .create_total_counter
                .wrapping_add(1);
                i = i.wrapping_add(1);
            }
        } else {
            i = 0 as uint32_t;
            while i < cnt {
                if (**servers.offset(i as isize)).first != 0 {
                    (**servers.offset(i as isize)).first = 0 as uint8_t;
                } else {
                    expdist = totalspace as ::core::ffi::c_double;
                    expdist /= (**servers.offset(i as isize)).totalspace as ::core::ffi::c_double;
                    (**servers.offset(i as isize)).corr += expdist
                        - (**servers.offset(i as isize))
                            .dist
                            .wrapping_add(i)
                            .wrapping_add(1 as uint32_t)
                            .wrapping_sub(cnt) as ::core::ffi::c_double;
                }
                (**servers.offset(i as isize)).dist =
                    cnt.wrapping_sub(i).wrapping_sub(1 as uint32_t);
                (**servers.offset(i as isize)).create_total_counter = (**servers
                    .offset(i as isize))
                .create_total_counter
                .wrapping_add(1);
                i = i.wrapping_add(1);
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_server_matches_labelexpr(
    mut e: *mut ::core::ffi::c_void,
    mut labelexpr: *const uint8_t,
) -> uint8_t {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        if !eptr.is_null() {
            return labelmask_matches_labelexpr((*eptr).labelmask, labelexpr);
        }
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_servers_matches_labelexpr(
    mut labelexpr: *const uint8_t,
) -> uint16_t {
    unsafe {
        let mut cnt: uint16_t = 0;
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        cnt = 0 as uint16_t;
        eptr = matocsservhead;
        while !eptr.is_null() {
            if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                && (*eptr).totalspace > 0 as uint64_t
                && (*eptr).usedspace <= (*eptr).totalspace
                && !(*eptr).csptr.is_null()
            {
                if matocsserv_server_matches_labelexpr(eptr as *mut ::core::ffi::c_void, labelexpr)
                    != 0
                {
                    cnt = cnt.wrapping_add(1);
                }
            }
            eptr = (*eptr).next as *mut matocsserventry;
        }
        return cnt;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_servers_with_label(mut label: uint8_t) -> uint16_t {
    unsafe {
        let mut cnt: uint16_t = 0;
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        cnt = 0 as uint16_t;
        eptr = matocsservhead;
        while !eptr.is_null() {
            if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                && (*eptr).totalspace > 0 as uint64_t
                && (*eptr).usedspace <= (*eptr).totalspace
                && !(*eptr).csptr.is_null()
            {
                if (*eptr).labelmask
                    & ((1 as ::core::ffi::c_int) << label as ::core::ffi::c_int) as uint32_t
                    != 0
                {
                    cnt = cnt.wrapping_add(1);
                }
            }
            eptr = (*eptr).next as *mut matocsserventry;
        }
        return cnt;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_server_get_labelmask(
    mut e: *mut ::core::ffi::c_void,
) -> uint32_t {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        return (*eptr).labelmask;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_server_get_labelstr(
    mut e: *mut ::core::ffi::c_void,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        if !(*eptr).labelstr.is_null() {
            return (*eptr).labelstr;
        } else {
            return b"(undefined)\0".as_ptr() as *const ::core::ffi::c_char;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_server_get_ip(mut e: *mut ::core::ffi::c_void) -> uint32_t {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        return (*eptr).servip;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_getservers_test(
    mut stdcscnt: *mut uint16_t,
    mut stdcsids: *mut uint16_t,
    mut olcscnt: *mut uint16_t,
    mut olcsids: *mut uint16_t,
    mut allcscnt: *mut uint16_t,
    mut allcsids: *mut uint16_t,
) {
    unsafe {
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        let mut gracecnt: uint32_t = 0;
        let mut stdcnt: uint32_t = 0;
        let mut totalcnt: uint32_t = 0;
        gracecnt = 0 as uint32_t;
        stdcnt = 0 as uint32_t;
        totalcnt = 0 as uint32_t;
        *stdcscnt = 0 as uint16_t;
        *olcscnt = 0 as uint16_t;
        *allcscnt = 0 as uint16_t;
        eptr = matocsservhead;
        while !eptr.is_null() && totalcnt < MAXCSCOUNT as uint32_t {
            if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                && (*eptr).totalspace > 0 as uint64_t
                && (*eptr).usedspace <= (*eptr).totalspace
                && !(*eptr).csptr.is_null()
            {
                *allcsids.offset(*allcscnt as isize) = (*eptr).csid;
                *allcscnt = (*allcscnt).wrapping_add(1);
                totalcnt = totalcnt.wrapping_add(1);
                if (*eptr).totalspace.wrapping_sub((*eptr).usedspace)
                    > (MFSCHUNKSIZE as ::core::ffi::c_uint).wrapping_mul(
                        (1 as ::core::ffi::c_uint).wrapping_add(
                            ((*eptr).writecounter as ::core::ffi::c_uint)
                                .wrapping_mul(10 as ::core::ffi::c_uint),
                        ),
                    ) as uint64_t
                {
                    *olcsids.offset(*olcscnt as isize) = (*eptr).csid;
                    *olcscnt = (*olcscnt).wrapping_add(1);
                    if (*eptr).hlstatus as ::core::ffi::c_int != HLSTATUS_OVERLOADED
                        && (*eptr).hlstatus as ::core::ffi::c_int != HLSTATUS_HSREBALANCE
                    {
                        if (*eptr).hlstatus as ::core::ffi::c_int != HLSTATUS_DEFAULT
                            && (*eptr).hlstatus as ::core::ffi::c_int != HLSTATUS_OK
                            && (*eptr).hlstatus as ::core::ffi::c_int != HLSTATUS_LSREBALANCE
                            || csdb_server_is_being_maintained(
                                (*eptr).csptr as *mut ::core::ffi::c_void,
                            ) as ::core::ffi::c_int
                                != 0
                        {
                            gracecnt = gracecnt.wrapping_add(1);
                            *stdcsids
                                .offset((MAXCSCOUNT as uint32_t).wrapping_sub(gracecnt) as isize) =
                                (*eptr).csid;
                        } else {
                            *stdcsids.offset(*stdcscnt as isize) = (*eptr).csid;
                            *stdcscnt = (*stdcscnt).wrapping_add(1);
                            stdcnt = stdcnt.wrapping_add(1);
                        }
                    }
                }
            }
            eptr = (*eptr).next as *mut matocsserventry;
        }
        if gracecnt.wrapping_mul(5 as uint32_t) > gracecnt.wrapping_add(stdcnt) {
            while gracecnt > 0 as uint32_t {
                *stdcsids.offset(*stdcscnt as isize) =
                    *stdcsids.offset((MAXCSCOUNT as uint32_t).wrapping_sub(gracecnt) as isize);
                *stdcscnt = (*stdcscnt).wrapping_add(1);
                gracecnt = gracecnt.wrapping_sub(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_getservers_wrandom(
    mut csids: *mut uint16_t,
    mut overloaded: *mut uint16_t,
) -> uint16_t {
    unsafe {
        static mut servtab: *mut *mut matocsserventry =
            ::core::ptr::null_mut::<*mut matocsserventry>();
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        let mut i: uint32_t = 0;
        let mut gracecnt: uint32_t = 0;
        let mut stdcnt: uint32_t = 0;
        let mut totalcnt: uint32_t = 0;
        if csids.is_null() || overloaded.is_null() {
            if !servtab.is_null() {
                free(servtab as *mut ::core::ffi::c_void);
            }
            servtab = ::core::ptr::null_mut::<*mut matocsserventry>();
            return 0 as uint16_t;
        }
        if servtab.is_null() {
            servtab = malloc(
                ::core::mem::size_of::<*mut matocsserventry>().wrapping_mul(MAXCSCOUNT as size_t),
            ) as *mut *mut matocsserventry;
            if servtab.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1203 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"servtab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1203 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"servtab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if servtab
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut *mut matocsserventry
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1203 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"servtab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1203 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"servtab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
        }
        gracecnt = 0 as uint32_t;
        stdcnt = 0 as uint32_t;
        totalcnt = 0 as uint32_t;
        *overloaded = 0 as uint16_t;
        eptr = matocsservhead;
        while !eptr.is_null() && totalcnt < MAXCSCOUNT as uint32_t {
            if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                && (*eptr).totalspace > 0 as uint64_t
                && (*eptr).usedspace <= (*eptr).totalspace
                && !(*eptr).csptr.is_null()
            {
                if (*eptr).totalspace.wrapping_sub((*eptr).usedspace)
                    > (MFSCHUNKSIZE as ::core::ffi::c_uint).wrapping_mul(
                        (1 as ::core::ffi::c_uint).wrapping_add(
                            ((*eptr).writecounter as ::core::ffi::c_uint)
                                .wrapping_mul(10 as ::core::ffi::c_uint),
                        ),
                    ) as uint64_t
                {
                    if (*eptr).hlstatus as ::core::ffi::c_int != HLSTATUS_OVERLOADED
                        && (*eptr).hlstatus as ::core::ffi::c_int != HLSTATUS_HSREBALANCE
                    {
                        totalcnt = totalcnt.wrapping_add(1);
                        if (*eptr).hlstatus as ::core::ffi::c_int != HLSTATUS_DEFAULT
                            && (*eptr).hlstatus as ::core::ffi::c_int != HLSTATUS_OK
                            || csdb_server_is_being_maintained(
                                (*eptr).csptr as *mut ::core::ffi::c_void,
                            ) as ::core::ffi::c_int
                                != 0
                        {
                            gracecnt = gracecnt.wrapping_add(1);
                            *servtab
                                .offset((MAXCSCOUNT as uint32_t).wrapping_sub(gracecnt) as isize) =
                                eptr;
                        } else {
                            *servtab.offset(stdcnt as isize) = eptr;
                            stdcnt = stdcnt.wrapping_add(1);
                        }
                    } else {
                        *overloaded = (*overloaded).wrapping_add(1);
                    }
                }
            }
            eptr = (*eptr).next as *mut matocsserventry;
        }
        if gracecnt.wrapping_mul(5 as uint32_t) > gracecnt.wrapping_add(stdcnt) {
            while gracecnt > 0 as uint32_t {
                *servtab.offset(stdcnt as isize) =
                    *servtab.offset((MAXCSCOUNT as uint32_t).wrapping_sub(gracecnt) as isize);
                stdcnt = stdcnt.wrapping_add(1);
                gracecnt = gracecnt.wrapping_sub(1);
            }
        }
        matocsserv_weighted_roundrobin_sort(servtab as *mut *mut matocsserventry, stdcnt);
        i = 0 as uint32_t;
        while i < stdcnt {
            *csids.offset(i as isize) = (**servtab.offset(i as isize)).csid;
            i = i.wrapping_add(1);
        }
        return stdcnt as uint16_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_useservers_wrandom(
    mut servers: *mut *mut ::core::ffi::c_void,
    mut cnt: uint16_t,
) {
    unsafe {
        matocsserv_weighted_roundrobin_used(servers as *mut *mut matocsserventry, cnt as uint32_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_getservers_replpossible(mut csids: *mut uint16_t) -> uint16_t {
    unsafe {
        let mut scount: uint16_t = 0;
        let mut replpossible: uint8_t = 0;
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        scount = 0 as uint16_t;
        replpossible = 0 as uint8_t;
        eptr = matocsservhead;
        while !eptr.is_null() && (scount as ::core::ffi::c_int) < MAXCSCOUNT {
            if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                && !(*eptr).csptr.is_null()
            {
                if (*eptr).totalspace == 0 as uint64_t
                    || (*eptr).totalspace.wrapping_sub((*eptr).usedspace)
                        <= (*eptr).totalspace.wrapping_div(100 as uint64_t)
                {
                    replpossible = 0 as uint8_t;
                } else if (*eptr).registered as ::core::ffi::c_int
                    != REGISTERED as ::core::ffi::c_int
                    || (*eptr).receivingchunks as ::core::ffi::c_int & TRANSFERRING_NEW_CHUNKS
                        != 0 as ::core::ffi::c_int
                {
                    replpossible = 1 as uint8_t;
                } else if !(((*eptr).hlstatus as ::core::ffi::c_int == HLSTATUS_DEFAULT
                    || (*eptr).hlstatus as ::core::ffi::c_int == HLSTATUS_OK
                    || (*eptr).hlstatus as ::core::ffi::c_int == HLSTATUS_LSREBALANCE)
                    && csdb_server_is_being_maintained((*eptr).csptr as *mut ::core::ffi::c_void)
                        as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int)
                {
                    replpossible = 1 as uint8_t;
                } else {
                    replpossible = 1 as uint8_t;
                }
                if replpossible != 0 {
                    let c2rust_fresh0 = scount;
                    scount = scount.wrapping_add(1);
                    *csids.offset(c2rust_fresh0 as isize) = (*eptr).csid;
                }
            }
            eptr = (*eptr).next as *mut matocsserventry;
        }
        return scount;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_get_server_groups(
    mut csids: *mut uint16_t,
    mut replimit: ::core::ffi::c_double,
    mut positions: *mut uint16_t,
) {
    unsafe {
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        let mut i: uint16_t = 0;
        let mut j: uint16_t = 0;
        let mut r: uint16_t = 0;
        let mut x: uint16_t = 0;
        let mut csstate: uint8_t = 0;
        let mut stage: uint8_t = 0;
        let mut counters: [uint16_t; 4] = [0; 4];
        let mut now: uint32_t = main_time();
        let mut a: ::core::ffi::c_double = 0.;
        counters[CSSTATE_OK as usize] = 0 as uint16_t;
        counters[CSSTATE_OVERLOADED as usize] = 0 as uint16_t;
        counters[CSSTATE_LIMIT_REACHED as usize] = 0 as uint16_t;
        counters[CSSTATE_NO_SPACE as usize] = 0 as uint16_t;
        stage = 0 as uint8_t;
        while (stage as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
            i = 0 as uint16_t;
            eptr = matocsservhead;
            while !eptr.is_null() && (i as ::core::ffi::c_int) < MAXCSCOUNT {
                if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                    && !(*eptr).csptr.is_null()
                {
                    a = ((*eptr).csid as uint32_t)
                        .wrapping_mul(0x9874bf31 as uint32_t)
                        .wrapping_add(now.wrapping_mul(0xb489fc37 as uint32_t))
                        as ::core::ffi::c_double
                        / 4294967296.0f64;
                    if (*eptr).totalspace == 0 as uint64_t
                        || (*eptr).totalspace.wrapping_sub((*eptr).usedspace)
                            <= (*eptr).totalspace.wrapping_div(100 as uint64_t)
                    {
                        csstate = CSSTATE_NO_SPACE as uint8_t;
                    } else if (*eptr).wrepcounter as ::core::ffi::c_int as ::core::ffi::c_double
                        / FULL_REPLICATION_WEIGHT as ::core::ffi::c_double
                        + a
                        >= replimit
                        || (*eptr).registered as ::core::ffi::c_int
                            != REGISTERED as ::core::ffi::c_int
                        || (*eptr).receivingchunks as ::core::ffi::c_int & TRANSFERRING_NEW_CHUNKS
                            != 0 as ::core::ffi::c_int
                    {
                        csstate = CSSTATE_LIMIT_REACHED as uint8_t;
                    } else if !(((*eptr).hlstatus as ::core::ffi::c_int == HLSTATUS_DEFAULT
                        || (*eptr).hlstatus as ::core::ffi::c_int == HLSTATUS_OK
                        || (*eptr).hlstatus as ::core::ffi::c_int == HLSTATUS_LSREBALANCE)
                        && csdb_server_is_being_maintained(
                            (*eptr).csptr as *mut ::core::ffi::c_void,
                        ) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int)
                    {
                        csstate = CSSTATE_OVERLOADED as uint8_t;
                    } else {
                        csstate = CSSTATE_OK as uint8_t;
                    }
                    if stage as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        counters[csstate as usize] = counters[csstate as usize].wrapping_add(1);
                    } else {
                        *csids.offset(*positions.offset(csstate as isize) as isize) = (*eptr).csid;
                        *positions.offset(csstate as isize) =
                            (*positions.offset(csstate as isize)).wrapping_add(1);
                    }
                    i = i.wrapping_add(1);
                }
                eptr = (*eptr).next as *mut matocsserventry;
            }
            if stage as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                *positions.offset(CSSTATE_OK as isize) = 0 as uint16_t;
                *positions.offset(CSSTATE_OVERLOADED as isize) = counters[CSSTATE_OK as usize];
                *positions.offset(CSSTATE_LIMIT_REACHED as isize) =
                    (*positions.offset(CSSTATE_OVERLOADED as isize) as ::core::ffi::c_int
                        + counters[CSSTATE_OVERLOADED as usize] as ::core::ffi::c_int)
                        as uint16_t;
                *positions.offset(CSSTATE_NO_SPACE as isize) =
                    (*positions.offset(CSSTATE_LIMIT_REACHED as isize) as ::core::ffi::c_int
                        + counters[CSSTATE_LIMIT_REACHED as usize] as ::core::ffi::c_int)
                        as uint16_t;
            }
            stage = stage.wrapping_add(1);
        }
        if *positions.offset(0 as isize) as ::core::ffi::c_int
            == counters[0 as usize] as ::core::ffi::c_int
        {
        } else {
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                1327 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"positions[CSSTATE_OK]==counters[CSSTATE_OK]\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"data integrity error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s' : %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                1327 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"positions[CSSTATE_OK]==counters[CSSTATE_OK]\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"data integrity error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        if *positions.offset(1 as isize) as ::core::ffi::c_int
            == counters[0 as usize] as ::core::ffi::c_int
                + counters[1 as usize] as ::core::ffi::c_int
        {
        } else {
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1328 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"positions[CSSTATE_OVERLOADED]==(counters[CSSTATE_OK]+counters[CSSTATE_OVERLOADED])\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"data integrity error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s' : %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1328 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"positions[CSSTATE_OVERLOADED]==(counters[CSSTATE_OK]+counters[CSSTATE_OVERLOADED])\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"data integrity error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        if *positions.offset(2 as isize) as ::core::ffi::c_int
            == counters[0 as usize] as ::core::ffi::c_int
                + counters[1 as usize] as ::core::ffi::c_int
                + counters[2 as usize] as ::core::ffi::c_int
        {
        } else {
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1329 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"positions[CSSTATE_LIMIT_REACHED]==(counters[CSSTATE_OK]+counters[CSSTATE_OVERLOADED]+counters[CSSTATE_LIMIT_REACHED])\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"data integrity error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s' : %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1329 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"positions[CSSTATE_LIMIT_REACHED]==(counters[CSSTATE_OK]+counters[CSSTATE_OVERLOADED]+counters[CSSTATE_LIMIT_REACHED])\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"data integrity error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        if *positions.offset(3 as isize) as ::core::ffi::c_int
            == counters[0 as usize] as ::core::ffi::c_int
                + counters[1 as usize] as ::core::ffi::c_int
                + counters[2 as usize] as ::core::ffi::c_int
                + counters[3 as usize] as ::core::ffi::c_int
        {
        } else {
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1330 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"positions[CSSTATE_NO_SPACE]==(counters[CSSTATE_OK]+counters[CSSTATE_OVERLOADED]+counters[CSSTATE_LIMIT_REACHED]+counters[CSSTATE_NO_SPACE])\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"data integrity error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s' : %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1330 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"positions[CSSTATE_NO_SPACE]==(counters[CSSTATE_OK]+counters[CSSTATE_OVERLOADED]+counters[CSSTATE_LIMIT_REACHED]+counters[CSSTATE_NO_SPACE])\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"data integrity error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        i = 1 as uint16_t;
        while (i as ::core::ffi::c_int) < counters[CSSTATE_OK as usize] as ::core::ffi::c_int {
            r = rndu32_ranged((i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t)
                as uint16_t;
            if r as ::core::ffi::c_int != i as ::core::ffi::c_int {
                x = *csids.offset(i as isize);
                *csids.offset(i as isize) = *csids.offset(r as isize);
                *csids.offset(r as isize) = x;
            }
            i = i.wrapping_add(1);
        }
        i = 1 as uint16_t;
        while (i as ::core::ffi::c_int)
            < counters[CSSTATE_OVERLOADED as usize] as ::core::ffi::c_int
        {
            r = (*positions.offset(CSSTATE_OK as isize) as uint32_t).wrapping_add(rndu32_ranged(
                (i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t,
            )) as uint16_t;
            j = (*positions.offset(CSSTATE_OK as isize) as ::core::ffi::c_int
                + i as ::core::ffi::c_int) as uint16_t;
            if r as ::core::ffi::c_int != j as ::core::ffi::c_int {
                x = *csids.offset(j as isize);
                *csids.offset(j as isize) = *csids.offset(r as isize);
                *csids.offset(r as isize) = x;
            }
            i = i.wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C" fn matocsserv_server_can_be_used_in_replication(
    mut eptr: *mut matocsserventry,
) -> uint8_t {
    unsafe {
        if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
            && (*eptr).totalspace > 0 as uint64_t
            && (*eptr).usedspace <= (*eptr).totalspace
            && (*eptr).totalspace.wrapping_sub((*eptr).usedspace)
                > (*eptr).totalspace.wrapping_div(100 as uint64_t)
            && !(*eptr).csptr.is_null()
            && (*eptr).registered as ::core::ffi::c_int == REGISTERED as ::core::ffi::c_int
            && (*eptr).receivingchunks as ::core::ffi::c_int & TRANSFERRING_NEW_CHUNKS
                == 0 as ::core::ffi::c_int
        {
            if ((*eptr).hlstatus as ::core::ffi::c_int == HLSTATUS_DEFAULT
                || (*eptr).hlstatus as ::core::ffi::c_int == HLSTATUS_OK
                || (*eptr).hlstatus as ::core::ffi::c_int == HLSTATUS_LSREBALANCE)
                && csdb_server_is_being_maintained((*eptr).csptr as *mut ::core::ffi::c_void)
                    as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                return 2 as uint8_t;
            } else {
                return 1 as uint8_t;
            }
        }
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_recalculate_storagemode_scounts(mut sm: *mut storagemode) {
    unsafe {
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        let mut datamatch: uint8_t = 0;
        let mut chksummatch: uint8_t = 0;
        (*sm).replallowed = 0 as uint16_t;
        (*sm).overloaded = 0 as uint16_t;
        (*sm).allvalid = 0 as uint16_t;
        (*sm).data_replallowed = 0 as uint16_t;
        (*sm).data_overloaded = 0 as uint16_t;
        (*sm).data_allvalid = 0 as uint16_t;
        (*sm).chksum_replallowed = 0 as uint16_t;
        (*sm).chksum_overloaded = 0 as uint16_t;
        (*sm).chksum_allvalid = 0 as uint16_t;
        (*sm).both_replallowed = 0 as uint16_t;
        (*sm).both_overloaded = 0 as uint16_t;
        (*sm).both_allvalid = 0 as uint16_t;
        if (*sm).has_labels as ::core::ffi::c_int != 0
            && (*sm).labelscnt as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        {
            eptr = matocsservhead;
            while !eptr.is_null() {
                if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                    && (*eptr).totalspace > 0 as uint64_t
                    && (*eptr).usedspace <= (*eptr).totalspace
                    && !(*eptr).csptr.is_null()
                {
                    if matocsserv_server_matches_labelexpr(
                        eptr as *mut ::core::ffi::c_void,
                        &raw mut *(&raw mut (*sm).labelexpr as *mut [uint8_t; 128])
                            .offset(0 as isize) as *mut uint8_t
                            as *const uint8_t,
                    ) != 0
                    {
                        (*sm).allvalid = (*sm).allvalid.wrapping_add(1);
                        's_81: {
                            match matocsserv_server_can_be_used_in_replication(eptr)
                                as ::core::ffi::c_int
                            {
                                2 => {
                                    (*sm).replallowed = (*sm).replallowed.wrapping_add(1);
                                }
                                1 => {}
                                _ => {
                                    break 's_81;
                                }
                            }
                            (*sm).overloaded = (*sm).overloaded.wrapping_add(1);
                        }
                    }
                }
                eptr = (*eptr).next as *mut matocsserventry;
            }
            (*sm).valid_ec_counters = 1 as uint8_t;
        } else if (*sm).has_labels as ::core::ffi::c_int != 0
            && (*sm).labelscnt as ::core::ffi::c_int == 2 as ::core::ffi::c_int
        {
            eptr = matocsservhead;
            while !eptr.is_null() {
                if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                    && (*eptr).totalspace > 0 as uint64_t
                    && (*eptr).usedspace <= (*eptr).totalspace
                    && !(*eptr).csptr.is_null()
                {
                    datamatch = (if matocsserv_server_matches_labelexpr(
                        eptr as *mut ::core::ffi::c_void,
                        &raw mut *(&raw mut (*sm).labelexpr as *mut [uint8_t; 128])
                            .offset(0 as isize) as *mut uint8_t
                            as *const uint8_t,
                    ) as ::core::ffi::c_int
                        != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint8_t;
                    chksummatch = (if matocsserv_server_matches_labelexpr(
                        eptr as *mut ::core::ffi::c_void,
                        &raw mut *(&raw mut (*sm).labelexpr as *mut [uint8_t; 128])
                            .offset(1 as isize) as *mut uint8_t
                            as *const uint8_t,
                    ) as ::core::ffi::c_int
                        != 0
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint8_t;
                    (*sm).allvalid = (*sm).allvalid.wrapping_add(1);
                    if datamatch as ::core::ffi::c_int & chksummatch as ::core::ffi::c_int != 0 {
                        (*sm).both_allvalid = (*sm).both_allvalid.wrapping_add(1);
                    } else if datamatch != 0 {
                        (*sm).data_allvalid = (*sm).data_allvalid.wrapping_add(1);
                    } else if chksummatch != 0 {
                        (*sm).chksum_allvalid = (*sm).chksum_allvalid.wrapping_add(1);
                    }
                    's_265: {
                        match matocsserv_server_can_be_used_in_replication(eptr)
                            as ::core::ffi::c_int
                        {
                            2 => {
                                (*sm).replallowed = (*sm).replallowed.wrapping_add(1);
                                if datamatch as ::core::ffi::c_int
                                    & chksummatch as ::core::ffi::c_int
                                    != 0
                                {
                                    (*sm).both_replallowed = (*sm).both_replallowed.wrapping_add(1);
                                } else if datamatch != 0 {
                                    (*sm).data_replallowed = (*sm).data_replallowed.wrapping_add(1);
                                } else if chksummatch != 0 {
                                    (*sm).chksum_replallowed =
                                        (*sm).chksum_replallowed.wrapping_add(1);
                                }
                            }
                            1 => {}
                            _ => {
                                break 's_265;
                            }
                        }
                        (*sm).overloaded = (*sm).overloaded.wrapping_add(1);
                        if datamatch as ::core::ffi::c_int & chksummatch as ::core::ffi::c_int != 0
                        {
                            (*sm).both_overloaded = (*sm).both_overloaded.wrapping_add(1);
                        } else if datamatch != 0 {
                            (*sm).data_overloaded = (*sm).data_overloaded.wrapping_add(1);
                        } else if chksummatch != 0 {
                            (*sm).chksum_overloaded = (*sm).chksum_overloaded.wrapping_add(1);
                        }
                    }
                }
                eptr = (*eptr).next as *mut matocsserventry;
            }
            (*sm).valid_ec_counters = 2 as uint8_t;
        } else {
            (*sm).valid_ec_counters = 0 as uint8_t;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_getservers_replallowed(mut csids: *mut uint16_t) -> uint16_t {
    unsafe {
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        let mut j: uint32_t = 0;
        j = 0 as uint32_t;
        eptr = matocsservhead;
        while !eptr.is_null() && j < MAXCSCOUNT as uint32_t {
            if matocsserv_server_can_be_used_in_replication(eptr) as ::core::ffi::c_int
                == 2 as ::core::ffi::c_int
            {
                *csids.offset(j as isize) = (*eptr).csid;
                j = j.wrapping_add(1);
            }
            eptr = (*eptr).next as *mut matocsserventry;
        }
        return j as uint16_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_getservers_lessrepl(
    mut csids: *mut uint16_t,
    mut replimit: ::core::ffi::c_double,
    mut highpriority: uint8_t,
    mut allservflag: *mut uint8_t,
) -> uint16_t {
    unsafe {
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        let mut j: uint32_t = 0;
        let mut k: uint32_t = 0;
        let mut r: uint32_t = 0;
        let mut hpadd: uint32_t = 0;
        let mut x: uint16_t = 0;
        let mut now: uint32_t = main_time();
        let mut a: ::core::ffi::c_double = 0.;
        j = 0 as uint32_t;
        k = 0 as uint32_t;
        hpadd = 0 as uint32_t;
        *allservflag = 1 as uint8_t;
        eptr = matocsservhead;
        while !eptr.is_null() && j < MAXCSCOUNT as uint32_t {
            a = ((*eptr).csid as uint32_t)
                .wrapping_mul(0x9874bf31 as uint32_t)
                .wrapping_add(now.wrapping_mul(0xb489fc37 as uint32_t))
                as ::core::ffi::c_double
                / 4294967296.0f64;
            match matocsserv_server_can_be_used_in_replication(eptr) as ::core::ffi::c_int {
                1 => {
                    if (*eptr).wrepcounter as ::core::ffi::c_int as ::core::ffi::c_double
                        / FULL_REPLICATION_WEIGHT as ::core::ffi::c_double
                        + a
                        < replimit
                    {
                        hpadd = 1 as uint32_t;
                    }
                }
                2 => {
                    if (*eptr).wrepcounter as ::core::ffi::c_int as ::core::ffi::c_double
                        / FULL_REPLICATION_WEIGHT as ::core::ffi::c_double
                        + a
                        < replimit
                    {
                        *csids.offset(j as isize) = (*eptr).csid;
                        j = j.wrapping_add(1);
                    } else {
                        *allservflag = 0 as uint8_t;
                    }
                }
                _ => {}
            }
            eptr = (*eptr).next as *mut matocsserventry;
        }
        while j > k.wrapping_add(1 as uint32_t) {
            r = k.wrapping_add(rndu32_ranged(j.wrapping_sub(k)));
            if r != k {
                x = *csids.offset(k as isize);
                *csids.offset(k as isize) = *csids.offset(r as isize);
                *csids.offset(r as isize) = x;
            }
            k = k.wrapping_add(1);
        }
        if highpriority as ::core::ffi::c_int != 0 && hpadd != 0 {
            k = j;
            eptr = matocsservhead;
            while !eptr.is_null() && j < MAXCSCOUNT as uint32_t {
                a = ((*eptr).csid as uint32_t)
                    .wrapping_mul(0x9874bf31 as uint32_t)
                    .wrapping_add(now.wrapping_mul(0xb489fc37 as uint32_t))
                    as ::core::ffi::c_double
                    / 4294967296.0f64;
                if matocsserv_server_can_be_used_in_replication(eptr) as ::core::ffi::c_int
                    == 1 as ::core::ffi::c_int
                {
                    if (*eptr).wrepcounter as ::core::ffi::c_int as ::core::ffi::c_double
                        / FULL_REPLICATION_WEIGHT as ::core::ffi::c_double
                        + a
                        < replimit
                    {
                        *csids.offset(j as isize) = (*eptr).csid;
                        j = j.wrapping_add(1);
                    } else {
                        *allservflag = 0 as uint8_t;
                    }
                }
                eptr = (*eptr).next as *mut matocsserventry;
            }
            while j > k.wrapping_add(1 as uint32_t) {
                r = k.wrapping_add(rndu32_ranged(j.wrapping_sub(k)));
                if r != k {
                    x = *csids.offset(k as isize);
                    *csids.offset(k as isize) = *csids.offset(r as isize);
                    *csids.offset(r as isize) = x;
                }
                k = k.wrapping_add(1);
            }
        }
        return j as uint16_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_recalculate_server_counters() {
    unsafe {
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        valid_servers_count = 0 as uint16_t;
        almostfull_servers_count = 0 as uint16_t;
        replallowed_servers_count = 0 as uint16_t;
        eptr = matocsservhead;
        while !eptr.is_null() {
            if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                && (*eptr).totalspace > 0 as uint64_t
                && (*eptr).usedspace <= (*eptr).totalspace
                && !(*eptr).csptr.is_null()
            {
                valid_servers_count = valid_servers_count.wrapping_add(1);
                if (*eptr).totalspace.wrapping_sub((*eptr).usedspace)
                    <= (*eptr).totalspace.wrapping_div(100 as uint64_t)
                {
                    almostfull_servers_count = almostfull_servers_count.wrapping_add(1);
                }
                if matocsserv_server_can_be_used_in_replication(eptr) as ::core::ffi::c_int
                    == 2 as ::core::ffi::c_int
                {
                    replallowed_servers_count = replallowed_servers_count.wrapping_add(1);
                }
            }
            eptr = (*eptr).next as *mut matocsserventry;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_servers_count() -> uint16_t {
    unsafe {
        return valid_servers_count;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_almostfull_servers() -> uint16_t {
    unsafe {
        return almostfull_servers_count;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_replallowed_servers() -> uint16_t {
    unsafe {
        return replallowed_servers_count;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_calculate_space() {
    unsafe {
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        let mut tspace: uint64_t = 0;
        let mut uspace: uint64_t = 0;
        let mut rspace: uint64_t = 0;
        let mut muspace: uint64_t = 0;
        let mut mtspace: uint64_t = 0;
        let mut usagemax: uint32_t = 0;
        let mut usagemin: uint32_t = 0;
        let mut dusage: ::core::ffi::c_double = 0.;
        let mut mpusage: uint32_t = 0;
        tspace = 0 as uint64_t;
        uspace = 0 as uint64_t;
        muspace = 0 as uint64_t;
        mtspace = 0 as uint64_t;
        usagemax = 0 as uint32_t;
        usagemin = 0 as uint32_t;
        eptr = matocsservhead;
        while !eptr.is_null() {
            if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                && (*eptr).totalspace > 0 as uint64_t
            {
                dusage = (*eptr).usedspace as ::core::ffi::c_double;
                dusage /= (*eptr).totalspace as ::core::ffi::c_double;
                if dusage < 0.0f64 {
                    dusage = 0.0f64;
                }
                if dusage > 1.0f64 {
                    dusage = 1.0f64;
                }
                mpusage =
                    (100000 as ::core::ffi::c_int as ::core::ffi::c_double * dusage) as uint32_t;
                if usagemax == 0 as uint32_t {
                    usagemax = mpusage;
                    usagemin = mpusage;
                } else {
                    if mpusage > usagemax {
                        usagemax = mpusage;
                    }
                    if mpusage < usagemin {
                        usagemin = mpusage;
                    }
                }
                tspace = tspace.wrapping_add((*eptr).totalspace);
                uspace = uspace.wrapping_add((*eptr).usedspace);
                if (*eptr).usedspace > muspace {
                    muspace = (*eptr).usedspace;
                }
                if (*eptr).totalspace > mtspace {
                    mtspace = (*eptr).totalspace;
                }
            }
            eptr = (*eptr).next as *mut matocsserventry;
        }
        match ReserveSpaceMode as ::core::ffi::c_int {
            1 => {
                rspace =
                    (ReserveSpaceValue * (tspace as ::core::ffi::c_double / 100.0f64)) as uint64_t;
            }
            2 => {
                rspace = (ReserveSpaceValue * muspace as ::core::ffi::c_double) as uint64_t;
            }
            3 => {
                rspace = (ReserveSpaceValue * mtspace as ::core::ffi::c_double) as uint64_t;
            }
            0 | _ => {
                rspace = ReserveSpaceValue as uint64_t;
            }
        }
        gtotalspace = tspace;
        gusedspace = uspace;
        gfreespace = tspace.wrapping_sub(uspace);
        if rspace > gfreespace {
            gavailspace = 0 as uint64_t;
        } else {
            gavailspace = gfreespace.wrapping_sub(rspace);
        }
        gusagediff = usagemax.wrapping_sub(usagemin);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_gettotalspace() -> uint64_t {
    unsafe {
        return gtotalspace;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_getusedspace() -> uint64_t {
    unsafe {
        return gusedspace;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_have_availspace() -> ::core::ffi::c_int {
    unsafe {
        return if gavailspace > 0 as uint64_t {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_getusagediff() -> uint32_t {
    unsafe {
        return gusagediff;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_getspace(
    mut totalspace: *mut uint64_t,
    mut availspace: *mut uint64_t,
    mut freespace: *mut uint64_t,
) {
    unsafe {
        if !totalspace.is_null() {
            *totalspace = gtotalspace;
        }
        if !availspace.is_null() {
            *availspace = gavailspace;
        }
        if !freespace.is_null() {
            *freespace = gfreespace;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_getstrip(
    mut e: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        static mut empty: *mut ::core::ffi::c_char =
            b"???\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
            && !(*eptr).servdesc.is_null()
        {
            return (*eptr).servdesc;
        }
        return empty;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_get_csdata(
    mut e: *mut ::core::ffi::c_void,
    mut clientip: uint32_t,
    mut servip: *mut uint32_t,
    mut servport: *mut uint16_t,
    mut servver: *mut uint32_t,
    mut servlabelmask: *mut uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int {
            *servip = multilan_map((*eptr).servip, clientip);
            *servport = (*eptr).servport;
            if !servver.is_null() {
                *servver = (*eptr).version;
            }
            if !servlabelmask.is_null() {
                *servlabelmask = (*eptr).labelmask;
            }
            return 0 as ::core::ffi::c_int;
        }
        return -1 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_getservdata(
    mut e: *mut ::core::ffi::c_void,
    mut ver: *mut uint32_t,
    mut uspc: *mut uint64_t,
    mut tspc: *mut uint64_t,
    mut chunkcnt: *mut uint32_t,
    mut tduspc: *mut uint64_t,
    mut tdtspc: *mut uint64_t,
    mut tdchunkcnt: *mut uint32_t,
    mut errcnt: *mut uint32_t,
    mut load: *mut uint32_t,
    mut hlstatus: *mut uint8_t,
    mut labelmask: *mut uint32_t,
    mut mfrstatus: *mut uint8_t,
) {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int {
            *ver = (*eptr).version;
            *uspc = (*eptr).usedspace;
            *tspc = (*eptr).totalspace;
            *chunkcnt = (*eptr).chunkscount;
            *tduspc = (*eptr).todelusedspace;
            *tdtspc = (*eptr).todeltotalspace;
            *tdchunkcnt = (*eptr).todelchunkscount;
            *errcnt = (*eptr).errorcounter;
            *load = (*eptr).load;
            *hlstatus = (*eptr).hlstatus;
            *labelmask = (*eptr).labelmask;
            *mfrstatus = chunk_get_mfrstatus((*eptr).csid);
        } else {
            *ver = 0 as uint32_t;
            *uspc = 0 as uint64_t;
            *tspc = 0 as uint64_t;
            *chunkcnt = 0 as uint32_t;
            *tduspc = 0 as uint64_t;
            *tdtspc = 0 as uint64_t;
            *tdchunkcnt = 0 as uint32_t;
            *errcnt = 0 as uint32_t;
            *load = 0 as uint32_t;
            *hlstatus = HLSTATUS_DEFAULT as uint8_t;
            *labelmask = 0 as uint32_t;
            *mfrstatus = 0 as uint8_t;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_can_split_chunks(
    mut e: *mut ::core::ffi::c_void,
    _ecmode: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        if matocsserv_server_can_be_used_in_replication(eptr) as ::core::ffi::c_int
            != 2 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        return if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
            && (*eptr).version
                >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 49 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_write_counters(
    mut e: *mut ::core::ffi::c_void,
    mut x: uint8_t,
) {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        if x != 0 {
            (*eptr).writecounter = (*eptr).writecounter.wrapping_add(1);
        } else if (*eptr).writecounter as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"can't decrease write counter - structure error\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else {
            (*eptr).writecounter = (*eptr).writecounter.wrapping_sub(1);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_hlstatus_fix() {
    unsafe {
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        let mut now: uint32_t = main_time();
        eptr = matocsservhead;
        while !eptr.is_null() {
            if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                && (*eptr).totalspace > 0 as uint64_t
                && !(*eptr).csptr.is_null()
            {
                if (*eptr).hlstatus as ::core::ffi::c_int == HLSTATUS_DEFAULT
                    || (*eptr).hlstatus as ::core::ffi::c_int == HLSTATUS_GRACEFUL
                {
                    if csdb_server_is_overloaded((*eptr).csptr as *mut ::core::ffi::c_void, now)
                        != 0
                    {
                        (*eptr).hlstatus = HLSTATUS_GRACEFUL as uint8_t;
                    } else {
                        (*eptr).hlstatus = HLSTATUS_DEFAULT as uint8_t;
                    }
                }
            }
            eptr = (*eptr).next as *mut matocsserventry;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_has_avail_space(mut e: *mut ::core::ffi::c_void) -> uint8_t {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        return (if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
            && (*eptr).totalspace > 0 as uint64_t
            && (*eptr).usedspace <= (*eptr).totalspace
            && !(*eptr).csptr.is_null()
            && (*eptr).totalspace.wrapping_sub((*eptr).usedspace)
                > (*eptr).totalspace.wrapping_div(100 as uint64_t)
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_get_usage(
    mut e: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_double {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        if (*eptr).totalspace > 0 as uint64_t {
            return (*eptr).usedspace as ::core::ffi::c_double
                / (*eptr).totalspace as ::core::ffi::c_double;
        } else {
            return 1.0f64;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_replication_write_counter(
    mut e: *mut ::core::ffi::c_void,
    mut now: uint32_t,
) -> ::core::ffi::c_double {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        let mut a: ::core::ffi::c_double = 0.;
        if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
            && (*eptr).totalspace > 0 as uint64_t
            && (*eptr).usedspace <= (*eptr).totalspace
            && (*eptr).totalspace.wrapping_sub((*eptr).usedspace)
                > (*eptr).totalspace.wrapping_div(100 as uint64_t)
            && !(*eptr).csptr.is_null()
            && (*eptr).registered as ::core::ffi::c_int == REGISTERED as ::core::ffi::c_int
            && (*eptr).receivingchunks as ::core::ffi::c_int & TRANSFERRING_NEW_CHUNKS
                == 0 as ::core::ffi::c_int
        {
            a = ((*eptr).csid as uint32_t)
                .wrapping_mul(0x9874bf31 as uint32_t)
                .wrapping_add(now.wrapping_mul(0xb489fc37 as uint32_t))
                as ::core::ffi::c_double
                / 4294967296.0f64;
            return (*eptr).wrepcounter as ::core::ffi::c_int as ::core::ffi::c_double
                / FULL_REPLICATION_WEIGHT as ::core::ffi::c_double
                + a;
        } else {
            return SOMETHING_OVER_ANY_LIMIT as ::core::ffi::c_double;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_replication_read_counter(
    mut e: *mut ::core::ffi::c_void,
    mut now: uint32_t,
) -> ::core::ffi::c_double {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        let mut a: ::core::ffi::c_double = 0.;
        if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
            && !(*eptr).csptr.is_null()
            && (*eptr).receivingchunks as ::core::ffi::c_int & TRANSFERRING_LOST_CHUNKS
                == 0 as ::core::ffi::c_int
        {
            a = ((*eptr).csid as uint32_t)
                .wrapping_mul(0x9874bf31 as uint32_t)
                .wrapping_add(now.wrapping_mul(0xb489fc37 as uint32_t))
                as ::core::ffi::c_double
                / 4294967296.0f64;
            return (*eptr).rrepcounter as ::core::ffi::c_int as ::core::ffi::c_double
                / FULL_REPLICATION_WEIGHT as ::core::ffi::c_double
                + a;
        } else {
            return SOMETHING_OVER_ANY_LIMIT as ::core::ffi::c_double;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_deletion_counter(mut e: *mut ::core::ffi::c_void) -> uint16_t {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        return (*eptr).delcounter;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_create_packet(
    mut eptr: *mut matocsserventry,
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
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                1799 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"outpacket\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                1799 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                1799 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"outpacket\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                1799 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
pub unsafe extern "C" fn matocsserv_send_chunk_checksum(
    mut e: *mut ::core::ffi::c_void,
    mut chunkid: uint64_t,
    mut ecid: uint8_t,
    mut version: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int {
            data = matocsserv_create_packet(
                eptr,
                ANTOCS_GET_CHUNK_CHECKSUM as uint32_t,
                (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t,
            );
            put64bit(
                &raw mut data,
                chunkid & 0xffffffffffffff as uint64_t
                    | (ecid as uint64_t) << 56 as ::core::ffi::c_int,
            );
            put32bit(&raw mut data, version);
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_got_chunk_checksum(
    mut eptr: *mut matocsserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut checksum: uint32_t = 0;
        let mut ecid: uint8_t = 0;
        let mut status: uint8_t = 0;
        if length
            != (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as uint32_t
            && length
                != (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    as uint32_t
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CSTOAN_CHUNK_CHECKSUM - wrong size (%u/13|16)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                1836 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                1836 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                1836 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                1836 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        chunkid = get64bit(&raw mut data);
        ecid = (chunkid >> 56 as ::core::ffi::c_int) as uint8_t;
        chunkid = (chunkid as ::core::ffi::c_ulong & 0xffffffffffffff as ::core::ffi::c_ulong)
            as uint64_t;
        version = get32bit(&raw mut data);
        if length
            == (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as uint32_t
        {
            status = get8bit(&raw mut data);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_NOTICE,
                b"(%s) chunk: %016lX%s calculate checksum status: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*eptr).servdesc,
                chunkid,
                matocsserv_ecid_to_str(ecid),
                mfsstrerr(status),
            );
        } else {
            checksum = get32bit(&raw mut data);
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_INFO,
                b"(%s) chunk: %016lX%s calculate checksum: %08X\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*eptr).servdesc,
                chunkid,
                matocsserv_ecid_to_str(ecid),
                checksum,
            );
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_broadcast_chunk_status(mut chunkid: uint64_t) {
    unsafe {
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if ChunkServerCheck as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            eptr = matocsservhead;
            while !eptr.is_null() {
                if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
                    && (*eptr).version
                        >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 32 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })) as uint32_t
                    && (*eptr).registered as ::core::ffi::c_int != 0
                    && (*eptr).receivingchunks as ::core::ffi::c_int
                        & (TRANSFERRING_LOST_CHUNKS | TRANSFERRING_NEW_CHUNKS)
                        == 0 as ::core::ffi::c_int
                {
                    data = matocsserv_create_packet(
                        eptr,
                        MATOCS_CHUNK_STATUS as uint32_t,
                        8 as uint32_t,
                    );
                    put64bit(&raw mut data, chunkid);
                }
                eptr = (*eptr).next as *mut matocsserventry;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_got_chunk_status(
    mut eptr: *mut matocsserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut parts: uint8_t = 0;
        let mut version: [uint32_t; 255] = [0; 255];
        let mut blocks: [uint16_t; 255] = [0; 255];
        let mut damaged: [uint8_t; 255] = [0; 255];
        let mut ecid: [uint8_t; 255] = [0; 255];
        if length < 8 as uint32_t
            || length.wrapping_rem(8 as uint32_t) != 0 as uint32_t
            || length.wrapping_div(8 as uint32_t) > 256 as uint32_t
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CSTOMA_CHUNK_STATUS - wrong size (%u/8+8*n)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        chunkid = get64bit(&raw mut data);
        if (*eptr).receivingchunks as ::core::ffi::c_int
            & (TRANSFERRING_LOST_CHUNKS | TRANSFERRING_NEW_CHUNKS)
            != 0 as ::core::ffi::c_int
            || (*eptr).registered as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || ChunkServerCheck as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            return;
        }
        length = length.wrapping_sub(8 as uint32_t);
        parts = 0 as uint8_t;
        while length > 0 as uint32_t {
            ecid[parts as usize] = get8bit(&raw mut data);
            damaged[parts as usize] = get8bit(&raw mut data);
            blocks[parts as usize] = get16bit(&raw mut data);
            version[parts as usize] = get32bit(&raw mut data);
            length = length.wrapping_sub(8 as uint32_t);
            parts = parts.wrapping_add(1);
        }
        chunk_got_status_data(
            chunkid,
            (*eptr).servdesc,
            (*eptr).csid,
            parts,
            &raw mut ecid as *mut uint8_t,
            &raw mut version as *mut uint32_t,
            &raw mut damaged as *mut uint8_t,
            &raw mut blocks as *mut uint16_t,
            (if ChunkServerCheck as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_send_createchunk(
    mut e: *mut ::core::ffi::c_void,
    mut chunkid: uint64_t,
    mut ecid: uint8_t,
    mut version: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int {
            data = matocsserv_create_packet(
                eptr,
                MATOCS_CREATE as uint32_t,
                (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t,
            );
            put64bit(
                &raw mut data,
                chunkid & 0xffffffffffffff as uint64_t
                    | (ecid as uint64_t) << 56 as ::core::ffi::c_int,
            );
            put32bit(&raw mut data, version);
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_got_createchunk_status(
    mut eptr: *mut matocsserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut ecid: uint8_t = 0;
        let mut status: uint8_t = 0;
        if length != (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CSTOMA_CREATE - wrong size (%u/9)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if data.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                1921 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                1921 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                1921 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                1921 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        chunkid = get64bit(&raw mut data);
        ecid = (chunkid >> 56 as ::core::ffi::c_int) as uint8_t;
        chunkid = (chunkid as ::core::ffi::c_ulong & 0xffffffffffffff as ::core::ffi::c_ulong)
            as uint64_t;
        status = get8bit(&raw mut data);
        chunk_got_create_status((*eptr).csid, chunkid, ecid, status);
        if status as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_NOTICE,
                b"(%s) chunk: %016lX%s creation status: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*eptr).servdesc,
                chunkid,
                matocsserv_ecid_to_str(ecid),
                mfsstrerr(status),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_send_deletechunk(
    mut e: *mut ::core::ffi::c_void,
    mut chunkid: uint64_t,
    mut ecid: uint8_t,
    mut version: uint32_t,
    mut reason: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        let mut dstchunkid: uint64_t = 0;
        let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        dstchunkid =
            chunkid & 0xffffffffffffff as uint64_t | (ecid as uint64_t) << 56 as ::core::ffi::c_int;
        if matocsserv_operation_find(dstchunkid, eptr as *mut ::core::ffi::c_void) != 0 {
            return -1 as ::core::ffi::c_int;
        }
        if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int {
            data = matocsserv_create_packet(
                eptr,
                MATOCS_DELETE as uint32_t,
                (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t,
            );
            put64bit(&raw mut data, dstchunkid);
            put32bit(&raw mut data, version);
            matocsserv_operation_begin(
                dstchunkid,
                version,
                eptr as *mut ::core::ffi::c_void,
                OPTYPE_DELETE as ::core::ffi::c_int as uint8_t,
                reason,
            );
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_got_deletechunk_status(
    mut eptr: *mut matocsserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut ecid: uint8_t = 0;
        let mut status: uint8_t = 0;
        if length != (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CSTOMA_DELETE - wrong size (%u/9)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if data.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                1963 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                1963 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                1963 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                1963 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        chunkid = get64bit(&raw mut data);
        ecid = (chunkid >> 56 as ::core::ffi::c_int) as uint8_t;
        chunkid = (chunkid as ::core::ffi::c_ulong & 0xffffffffffffff as ::core::ffi::c_ulong)
            as uint64_t;
        status = get8bit(&raw mut data);
        matocsserv_operation_end(
            chunkid & 0xffffffffffffff as uint64_t | (ecid as uint64_t) << 56 as ::core::ffi::c_int,
            eptr as *mut ::core::ffi::c_void,
            (status as ::core::ffi::c_int == MFS_STATUS_OK) as ::core::ffi::c_int as uint8_t,
        );
        chunk_got_delete_status((*eptr).csid, chunkid, ecid, status);
        if status as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_NOTICE,
                b"(%s) chunk: %016lX%s deletion status: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*eptr).servdesc,
                chunkid,
                matocsserv_ecid_to_str(ecid),
                mfsstrerr(status),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_send_replicatechunk(
    mut e: *mut ::core::ffi::c_void,
    mut chunkid: uint64_t,
    mut ecid: uint8_t,
    mut version: uint32_t,
    mut src: *mut ::core::ffi::c_void,
    mut reason: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut dsteptr: *mut matocsserventry = e as *mut matocsserventry;
        let mut srceptr: *mut matocsserventry = src as *mut matocsserventry;
        let mut dstchunkid: uint64_t = 0;
        let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        dstchunkid =
            chunkid & 0xffffffffffffff as uint64_t | (ecid as uint64_t) << 56 as ::core::ffi::c_int;
        if matocsserv_replication_find(dstchunkid, version, dsteptr as *mut ::core::ffi::c_void)
            != 0
        {
            return -1 as ::core::ffi::c_int;
        }
        if (*dsteptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
            && (*srceptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
        {
            data = matocsserv_create_packet(
                dsteptr,
                MATOCS_REPLICATE as uint32_t,
                (8 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as uint32_t,
            );
            put64bit(&raw mut data, dstchunkid);
            put32bit(&raw mut data, version);
            put32bit(&raw mut data, (*srceptr).servip);
            put16bit(&raw mut data, (*srceptr).servport);
            matocsserv_replication_begin(
                dstchunkid,
                version,
                dsteptr as *mut ::core::ffi::c_void,
                1 as uint8_t,
                &raw mut src,
                (if ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    FULL_REPLICATION_WEIGHT
                } else {
                    EC_REPLICATION_WEIGHT
                }) as uint8_t,
                (if ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    FULL_REPLICATION_WEIGHT
                } else {
                    EC_REPLICATION_WEIGHT
                }) as uint8_t,
                REPTYPE_SIMPLE as ::core::ffi::c_int as uint8_t,
                reason,
            );
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_send_replicatechunk_split(
    mut e: *mut ::core::ffi::c_void,
    mut chunkid: uint64_t,
    mut ecid: uint8_t,
    mut version: uint32_t,
    mut src: *mut ::core::ffi::c_void,
    mut srcecid: uint8_t,
    mut partno: uint8_t,
    mut parts: uint8_t,
    mut reason: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut dsteptr: *mut matocsserventry = e as *mut matocsserventry;
        let mut srceptr: *mut matocsserventry = src as *mut matocsserventry;
        let mut dstchunkid: uint64_t = 0;
        let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        dstchunkid =
            chunkid & 0xffffffffffffff as uint64_t | (ecid as uint64_t) << 56 as ::core::ffi::c_int;
        if matocsserv_replication_find(dstchunkid, version, dsteptr as *mut ::core::ffi::c_void)
            != 0
        {
            return -1 as ::core::ffi::c_int;
        }
        if (*dsteptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
            && (*srceptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
        {
            data = matocsserv_create_packet(
                dsteptr,
                MATOCS_REPLICATE_SPLIT as uint32_t,
                (8 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as uint32_t,
            );
            put64bit(&raw mut data, dstchunkid);
            put32bit(&raw mut data, version);
            put32bit(&raw mut data, (*srceptr).servip);
            put16bit(&raw mut data, (*srceptr).servport);
            put64bit(
                &raw mut data,
                chunkid & 0xffffffffffffff as uint64_t
                    | (srcecid as uint64_t) << 56 as ::core::ffi::c_int,
            );
            put8bit(&raw mut data, partno);
            put8bit(&raw mut data, parts);
            matocsserv_replication_begin(
                dstchunkid,
                version,
                dsteptr as *mut ::core::ffi::c_void,
                1 as uint8_t,
                &raw mut src,
                EC_REPLICATION_WEIGHT as uint8_t,
                EC_REPLICATION_WEIGHT as uint8_t,
                REPTYPE_SPLIT as ::core::ffi::c_int as uint8_t,
                reason,
            );
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_send_replicatechunk_recover(
    mut e: *mut ::core::ffi::c_void,
    mut chunkid: uint64_t,
    mut ecid: uint8_t,
    mut version: uint32_t,
    mut parts: uint8_t,
    mut survivors: *mut *mut ::core::ffi::c_void,
    mut survivorecids: *mut uint8_t,
    mut reason: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut dsteptr: *mut matocsserventry = e as *mut matocsserventry;
        let mut srceptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        let mut dstchunkid: uint64_t = 0;
        let mut srcchunkid: uint64_t = 0;
        let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut i: uint8_t = 0;
        dstchunkid =
            chunkid & 0xffffffffffffff as uint64_t | (ecid as uint64_t) << 56 as ::core::ffi::c_int;
        if matocsserv_replication_find(dstchunkid, version, dsteptr as *mut ::core::ffi::c_void)
            != 0
        {
            return -1 as ::core::ffi::c_int;
        }
        if (*dsteptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int {
            i = 0 as uint8_t;
            while (i as ::core::ffi::c_int) < parts as ::core::ffi::c_int {
                srceptr = *survivors.offset(i as isize) as *mut matocsserventry;
                if (*srceptr).mode as ::core::ffi::c_int == KILL as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int;
                }
                i = i.wrapping_add(1);
            }
            data = matocsserv_create_packet(
                dsteptr,
                MATOCS_REPLICATE_RECOVER as uint32_t,
                (8 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 16 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
                    + parts as ::core::ffi::c_int
                        * (8 as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int)) as uint32_t,
            );
            put64bit(&raw mut data, dstchunkid);
            put32bit(&raw mut data, version);
            if parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                put32bit(&raw mut data, 0x88888888 as uint32_t);
                put32bit(&raw mut data, 0x44444444 as uint32_t);
                put32bit(&raw mut data, 0x22222222 as uint32_t);
                put32bit(&raw mut data, 0x11111111 as uint32_t);
            } else if parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
                put32bit(&raw mut data, 0x8888 as uint32_t);
                put32bit(&raw mut data, 0x4444 as uint32_t);
                put32bit(&raw mut data, 0x2222 as uint32_t);
                put32bit(&raw mut data, 0x1111 as uint32_t);
            } else {
                put32bit(&raw mut data, 0 as uint32_t);
                put32bit(&raw mut data, 0 as uint32_t);
                put32bit(&raw mut data, 0 as uint32_t);
                put32bit(&raw mut data, 0 as uint32_t);
            }
            put8bit(&raw mut data, parts);
            i = 0 as uint8_t;
            while (i as ::core::ffi::c_int) < parts as ::core::ffi::c_int {
                srceptr = *survivors.offset(i as isize) as *mut matocsserventry;
                srcchunkid = chunkid & 0xffffffffffffff as uint64_t
                    | (*survivorecids.offset(i as isize) as uint64_t) << 56 as ::core::ffi::c_int;
                put32bit(&raw mut data, (*srceptr).servip);
                put16bit(&raw mut data, (*srceptr).servport);
                put64bit(&raw mut data, srcchunkid);
                i = i.wrapping_add(1);
            }
            matocsserv_replication_begin(
                dstchunkid,
                version,
                dsteptr as *mut ::core::ffi::c_void,
                parts,
                survivors,
                EC_REPLICATION_WEIGHT as uint8_t,
                EC_REPLICATION_WEIGHT as uint8_t,
                REPTYPE_RECOVER as ::core::ffi::c_int as uint8_t,
                reason,
            );
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_send_replicatechunk_join(
    mut e: *mut ::core::ffi::c_void,
    mut chunkid: uint64_t,
    mut ecid: uint8_t,
    mut version: uint32_t,
    mut parts: uint8_t,
    mut survivors: *mut *mut ::core::ffi::c_void,
    mut survivorecids: *mut uint8_t,
    mut reason: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut dsteptr: *mut matocsserventry = e as *mut matocsserventry;
        let mut srceptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        let mut dstchunkid: uint64_t = 0;
        let mut srcchunkid: uint64_t = 0;
        let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut i: uint8_t = 0;
        dstchunkid =
            chunkid & 0xffffffffffffff as uint64_t | (ecid as uint64_t) << 56 as ::core::ffi::c_int;
        if matocsserv_replication_find(dstchunkid, version, dsteptr as *mut ::core::ffi::c_void)
            != 0
        {
            return -1 as ::core::ffi::c_int;
        }
        if (*dsteptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int {
            i = 0 as uint8_t;
            while (i as ::core::ffi::c_int) < parts as ::core::ffi::c_int {
                srceptr = *survivors.offset(i as isize) as *mut matocsserventry;
                if (*srceptr).mode as ::core::ffi::c_int == KILL as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int;
                }
                i = i.wrapping_add(1);
            }
            data = matocsserv_create_packet(
                dsteptr,
                MATOCS_REPLICATE_JOIN as uint32_t,
                (8 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int
                    + parts as ::core::ffi::c_int
                        * (8 as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int)) as uint32_t,
            );
            put64bit(&raw mut data, dstchunkid);
            put32bit(&raw mut data, version);
            put8bit(&raw mut data, parts);
            i = 0 as uint8_t;
            while (i as ::core::ffi::c_int) < parts as ::core::ffi::c_int {
                srceptr = *survivors.offset(i as isize) as *mut matocsserventry;
                srcchunkid = chunkid & 0xffffffffffffff as uint64_t
                    | (*survivorecids.offset(i as isize) as uint64_t) << 56 as ::core::ffi::c_int;
                put32bit(&raw mut data, (*srceptr).servip);
                put16bit(&raw mut data, (*srceptr).servport);
                put64bit(&raw mut data, srcchunkid);
                i = i.wrapping_add(1);
            }
            matocsserv_replication_begin(
                dstchunkid,
                version,
                dsteptr as *mut ::core::ffi::c_void,
                parts,
                survivors,
                EC_REPLICATION_WEIGHT as uint8_t,
                FULL_REPLICATION_WEIGHT as uint8_t,
                REPTYPE_JOIN as ::core::ffi::c_int as uint8_t,
                reason,
            );
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_got_replicatechunk_status(
    mut eptr: *mut matocsserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut reptype: uint8_t = 0;
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut ecid: uint8_t = 0;
        let mut status: uint8_t = 0;
        let mut servbuff: [::core::ffi::c_char; 1000] = [0; 1000];
        let mut leng: uint32_t = 0;
        if length
            != (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as uint32_t
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CSTOMA_REPLICATE - wrong size (%u/13)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if data.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2133 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2133 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2133 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2133 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        chunkid = get64bit(&raw mut data);
        ecid = (chunkid >> 56 as ::core::ffi::c_int) as uint8_t;
        chunkid = (chunkid as ::core::ffi::c_ulong & 0xffffffffffffff as ::core::ffi::c_ulong)
            as uint64_t;
        version = get32bit(&raw mut data);
        status = get8bit(&raw mut data);
        if status as ::core::ffi::c_int != MFS_STATUS_OK {
            leng = matocsserv_replication_print(
                &raw mut servbuff as *mut ::core::ffi::c_char,
                1000 as uint32_t,
                chunkid & 0xffffffffffffff as uint64_t
                    | (ecid as uint64_t) << 56 as ::core::ffi::c_int,
                version,
                eptr as *mut ::core::ffi::c_void,
                &raw mut reptype,
            );
            if leng >= 1000 as uint32_t {
                servbuff[999 as usize] = '\0' as ::core::ffi::c_char;
            } else {
                servbuff[leng as usize] = '\0' as ::core::ffi::c_char;
            }
            if leng > 0 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_NOTICE,
                    b"(%s) chunk: %016lX%s %s replication status: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut servbuff as *mut ::core::ffi::c_char,
                    chunkid,
                    matocsserv_ecid_to_str(ecid),
                    reptype_str[(reptype as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize],
                    mfsstrerr(status),
                );
            } else {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_NOTICE,
                    b"(unknown -> %s) chunk: %016lX%s %s replication status: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*eptr).servdesc,
                    chunkid,
                    matocsserv_ecid_to_str(ecid),
                    reptype_str[(reptype as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize],
                    mfsstrerr(status),
                );
            }
        }
        matocsserv_replication_end(
            chunkid & 0xffffffffffffff as uint64_t | (ecid as uint64_t) << 56 as ::core::ffi::c_int,
            version,
            eptr as *mut ::core::ffi::c_void,
            (status as ::core::ffi::c_int == MFS_STATUS_OK) as ::core::ffi::c_int as uint8_t,
        );
        chunk_got_replicate_status((*eptr).csid, chunkid, ecid, version, status);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_send_setchunkversion(
    mut e: *mut ::core::ffi::c_void,
    mut chunkid: uint64_t,
    mut ecid: uint8_t,
    mut version: uint32_t,
    mut oldversion: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int {
            data = matocsserv_create_packet(
                eptr,
                MATOCS_SET_VERSION as uint32_t,
                (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                    as uint32_t,
            );
            put64bit(
                &raw mut data,
                chunkid & 0xffffffffffffff as uint64_t
                    | (ecid as uint64_t) << 56 as ::core::ffi::c_int,
            );
            put32bit(&raw mut data, version);
            put32bit(&raw mut data, oldversion);
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_got_setchunkversion_status(
    mut eptr: *mut matocsserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut ecid: uint8_t = 0;
        let mut status: uint8_t = 0;
        if length != (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CSTOMA_SET_VERSION - wrong size (%u/9)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if data.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2182 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2182 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2182 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2182 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        chunkid = get64bit(&raw mut data);
        ecid = (chunkid >> 56 as ::core::ffi::c_int) as uint8_t;
        chunkid = (chunkid as ::core::ffi::c_ulong & 0xffffffffffffff as ::core::ffi::c_ulong)
            as uint64_t;
        status = get8bit(&raw mut data);
        chunk_got_setversion_status((*eptr).csid, chunkid, ecid, status);
        if status as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_NOTICE,
                b"(%s) chunk: %016lX%s set version status: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*eptr).servdesc,
                chunkid,
                matocsserv_ecid_to_str(ecid),
                mfsstrerr(status),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_send_duplicatechunk(
    mut e: *mut ::core::ffi::c_void,
    mut chunkid: uint64_t,
    mut ecid: uint8_t,
    mut version: uint32_t,
    mut oldchunkid: uint64_t,
    mut oldecid: uint8_t,
    mut oldversion: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int {
            data = matocsserv_create_packet(
                eptr,
                MATOCS_DUPLICATE as uint32_t,
                (8 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int) as uint32_t,
            );
            put64bit(
                &raw mut data,
                chunkid & 0xffffffffffffff as uint64_t
                    | (ecid as uint64_t) << 56 as ::core::ffi::c_int,
            );
            put32bit(&raw mut data, version);
            put64bit(
                &raw mut data,
                oldchunkid & 0xffffffffffffff as uint64_t
                    | (oldecid as uint64_t) << 56 as ::core::ffi::c_int,
            );
            put32bit(&raw mut data, oldversion);
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_got_duplicatechunk_status(
    mut eptr: *mut matocsserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut ecid: uint8_t = 0;
        let mut status: uint8_t = 0;
        if length != (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CSTOMA_DUPLICATE - wrong size (%u/9)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if data.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2220 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2220 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2220 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2220 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        chunkid = get64bit(&raw mut data);
        ecid = (chunkid >> 56 as ::core::ffi::c_int) as uint8_t;
        chunkid = (chunkid as ::core::ffi::c_ulong & 0xffffffffffffff as ::core::ffi::c_ulong)
            as uint64_t;
        status = get8bit(&raw mut data);
        chunk_got_duplicate_status((*eptr).csid, chunkid, ecid, status);
        if status as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_NOTICE,
                b"(%s) chunk: %016lX%s duplication status: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*eptr).servdesc,
                chunkid,
                matocsserv_ecid_to_str(ecid),
                mfsstrerr(status),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_send_truncatechunk(
    mut e: *mut ::core::ffi::c_void,
    mut chunkid: uint64_t,
    mut ecid: uint8_t,
    mut length: uint32_t,
    mut version: uint32_t,
    mut oldversion: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int {
            data = matocsserv_create_packet(
                eptr,
                MATOCS_TRUNCATE as uint32_t,
                (8 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int) as uint32_t,
            );
            put64bit(
                &raw mut data,
                chunkid & 0xffffffffffffff as uint64_t
                    | (ecid as uint64_t) << 56 as ::core::ffi::c_int,
            );
            put32bit(&raw mut data, length);
            put32bit(&raw mut data, version);
            put32bit(&raw mut data, oldversion);
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_got_truncatechunk_status(
    mut eptr: *mut matocsserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut ecid: uint8_t = 0;
        let mut status: uint8_t = 0;
        if length != (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CSTOMA_TRUNCATE - wrong size (%u/9)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if data.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2258 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2258 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2258 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2258 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        chunkid = get64bit(&raw mut data);
        ecid = (chunkid >> 56 as ::core::ffi::c_int) as uint8_t;
        chunkid = (chunkid as ::core::ffi::c_ulong & 0xffffffffffffff as ::core::ffi::c_ulong)
            as uint64_t;
        status = get8bit(&raw mut data);
        chunk_got_truncate_status((*eptr).csid, chunkid, ecid, status);
        if status as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_NOTICE,
                b"(%s) chunk: %016lX%s truncate status: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*eptr).servdesc,
                chunkid,
                matocsserv_ecid_to_str(ecid),
                mfsstrerr(status),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_send_duptruncchunk(
    mut e: *mut ::core::ffi::c_void,
    mut chunkid: uint64_t,
    mut ecid: uint8_t,
    mut version: uint32_t,
    mut oldchunkid: uint64_t,
    mut oldecid: uint8_t,
    mut oldversion: uint32_t,
    mut length: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int {
            data = matocsserv_create_packet(
                eptr,
                MATOCS_DUPTRUNC as uint32_t,
                (8 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int) as uint32_t,
            );
            put64bit(
                &raw mut data,
                chunkid & 0xffffffffffffff as uint64_t
                    | (ecid as uint64_t) << 56 as ::core::ffi::c_int,
            );
            put32bit(&raw mut data, version);
            put64bit(
                &raw mut data,
                oldchunkid & 0xffffffffffffff as uint64_t
                    | (oldecid as uint64_t) << 56 as ::core::ffi::c_int,
            );
            put32bit(&raw mut data, oldversion);
            put32bit(&raw mut data, length);
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_got_duptruncchunk_status(
    mut eptr: *mut matocsserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut ecid: uint8_t = 0;
        let mut status: uint8_t = 0;
        if length != (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CSTOMA_DUPTRUNC - wrong size (%u/9)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if data.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2298 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2298 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2298 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2298 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        chunkid = get64bit(&raw mut data);
        ecid = (chunkid >> 56 as ::core::ffi::c_int) as uint8_t;
        chunkid = (chunkid as ::core::ffi::c_ulong & 0xffffffffffffff as ::core::ffi::c_ulong)
            as uint64_t;
        status = get8bit(&raw mut data);
        chunk_got_duptrunc_status((*eptr).csid, chunkid, ecid, status);
        if status as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_NOTICE,
                b"(%s) chunk: %016lX%s duplication with truncate status: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*eptr).servdesc,
                chunkid,
                matocsserv_ecid_to_str(ecid),
                mfsstrerr(status),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_send_localsplitchunk(
    mut e: *mut ::core::ffi::c_void,
    mut chunkid: uint64_t,
    mut version: uint32_t,
    mut missingmask: uint32_t,
    mut parts: uint8_t,
    mut reason: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut pver: uint8_t = 0;
        if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int {
            pver = (if (*eptr).version
                >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                    + 25 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                    + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })) as uint32_t
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            data = matocsserv_create_packet(
                eptr,
                MATOCS_LOCALSPLIT as uint32_t,
                (8 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + pver as ::core::ffi::c_int) as uint32_t,
            );
            put64bit(&raw mut data, chunkid);
            put32bit(&raw mut data, version);
            put32bit(&raw mut data, missingmask);
            if pver as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                put8bit(&raw mut data, parts);
            }
            matocsserv_replication_begin(
                chunkid,
                version,
                eptr as *mut ::core::ffi::c_void,
                1 as uint8_t,
                &raw mut e,
                FULL_REPLICATION_WEIGHT as uint8_t,
                (LOCALPART_REPLICATION_WEIGHT * bitcount(missingmask) as ::core::ffi::c_int)
                    as uint8_t,
                REPTYPE_LOCALSPLIT as ::core::ffi::c_int as uint8_t,
                reason,
            );
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_got_localsplitchunk_status(
    mut eptr: *mut matocsserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut status: uint8_t = 0;
        if length
            != (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as uint32_t
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CSTOMA_LOCALSPLIT - wrong size (%u/13)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if data.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2342 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2342 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2342 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2342 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        chunkid = get64bit(&raw mut data);
        version = get32bit(&raw mut data);
        status = get8bit(&raw mut data);
        matocsserv_replication_end(
            chunkid,
            version,
            eptr as *mut ::core::ffi::c_void,
            (status as ::core::ffi::c_int == MFS_STATUS_OK) as ::core::ffi::c_int as uint8_t,
        );
        chunk_got_localsplit_status((*eptr).csid, chunkid, version, status);
        if status as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_NOTICE,
                b"(%s) chunk: %016lX localsplit status: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*eptr).servdesc,
                chunkid,
                mfsstrerr(status),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_send_chunkop(
    mut e: *mut ::core::ffi::c_void,
    mut chunkid: uint64_t,
    mut ecid: uint8_t,
    mut version: uint32_t,
    mut newversion: uint32_t,
    mut copychunkid: uint64_t,
    mut copyecid: uint8_t,
    mut copyversion: uint32_t,
    mut leng: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int {
            data = matocsserv_create_packet(
                eptr,
                MATOCS_CHUNKOP as uint32_t,
                (8 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int
                    + 4 as ::core::ffi::c_int) as uint32_t,
            );
            put64bit(
                &raw mut data,
                chunkid & 0xffffffffffffff as uint64_t
                    | (ecid as uint64_t) << 56 as ::core::ffi::c_int,
            );
            put32bit(&raw mut data, version);
            put32bit(&raw mut data, newversion);
            put64bit(
                &raw mut data,
                copychunkid & 0xffffffffffffff as uint64_t
                    | (copyecid as uint64_t) << 56 as ::core::ffi::c_int,
            );
            put32bit(&raw mut data, copyversion);
            put32bit(&raw mut data, leng);
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_got_chunkop_status(
    mut eptr: *mut matocsserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut ecid: uint8_t = 0;
        let mut copyecid: uint8_t = 0;
        let mut chunkid: uint64_t = 0;
        let mut copychunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut newversion: uint32_t = 0;
        let mut copyversion: uint32_t = 0;
        let mut leng: uint32_t = 0;
        let mut status: uint8_t = 0;
        if length
            != (8 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as uint32_t
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CSTOMA_CHUNKOP - wrong size (%u/33)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if data.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2389 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2389 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2389 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2389 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        chunkid = get64bit(&raw mut data);
        ecid = (chunkid >> 56 as ::core::ffi::c_int) as uint8_t;
        chunkid = (chunkid as ::core::ffi::c_ulong & 0xffffffffffffff as ::core::ffi::c_ulong)
            as uint64_t;
        version = get32bit(&raw mut data);
        newversion = get32bit(&raw mut data);
        copychunkid = get64bit(&raw mut data);
        copyecid = (copychunkid >> 56 as ::core::ffi::c_int) as uint8_t;
        copychunkid = (copychunkid as ::core::ffi::c_ulong
            & 0xffffffffffffff as ::core::ffi::c_ulong) as uint64_t;
        copyversion = get32bit(&raw mut data);
        leng = get32bit(&raw mut data);
        status = get8bit(&raw mut data);
        if newversion != version {
            chunk_got_chunkop_status((*eptr).csid, chunkid, ecid, status);
        }
        if copychunkid > 0 as uint64_t {
            chunk_got_chunkop_status((*eptr).csid, copychunkid, copyecid, status);
        }
        if status as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_NOTICE,
                b"(%s) chunkop(%016lX%s,%08X,%08X,%016lX,%08X,%u) status: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*eptr).servdesc,
                chunkid,
                matocsserv_ecid_to_str(ecid),
                version,
                newversion,
                copychunkid,
                copyversion,
                leng,
                mfsstrerr(status),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_get_version(
    mut eptr: *mut matocsserventry,
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
            ptr = matocsserv_create_packet(
                eptr,
                ANTOAN_VERSION as uint32_t,
                ((4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as size_t)
                    .wrapping_add(strlen(&raw const vstring as *const ::core::ffi::c_char))
                    as uint32_t,
            );
            put32bit(&raw mut ptr, msgid);
        } else {
            ptr = matocsserv_create_packet(
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
pub unsafe extern "C" fn matocsserv_get_config(
    mut eptr: *mut matocsserventry,
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
            ptr = matocsserv_create_packet(
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
            ptr = matocsserv_create_packet(
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
pub unsafe extern "C" fn matocsserv_get_config_file(
    mut eptr: *mut matocsserventry,
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
            ptr = matocsserv_create_packet(
                eptr,
                ANTOAN_CONFIG_FILE_CONTENT as uint32_t,
                5 as uint32_t,
            );
            put32bit(&raw mut ptr, msgid);
            put8bit(&raw mut ptr, MFS_ERROR_ENOENT as uint8_t);
        } else {
            ptr = matocsserv_create_packet(
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
pub unsafe extern "C" fn matocsserv_syslog(
    mut eptr: *mut matocsserventry,
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
unsafe extern "C" fn matocsserv_remap_ip(mut csip: uint32_t) -> uint32_t {
    unsafe {
        if csip & RemapMask == RemapSrc {
            csip &= !RemapMask;
            csip |= RemapDst;
        }
        return csip;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_register(
    mut eptr: *mut matocsserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut chunkversion: uint32_t = 0;
        let mut i: uint32_t = 0;
        let mut chunkcount: uint32_t = 0;
        let mut rversion: uint8_t = 0;
        let mut ecid: uint8_t = 0;
        let mut csid: uint16_t = 0;
        let mut us: ::core::ffi::c_double = 0.;
        let mut ts: ::core::ffi::c_double = 0.;
        if length & 1 as uint32_t == 0 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CSTOMA_REGISTER: chunkserver is too old\0".as_ptr() as *const ::core::ffi::c_char,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        } else {
            if data.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2582 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"data\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2582 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2582 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"data\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2582 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"data\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            rversion = get8bit(&raw mut data);
            if (*eptr).registered as ::core::ffi::c_int == REGISTERED as ::core::ffi::c_int
                && rversion as ::core::ffi::c_int != 63 as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"got register message from registered chunkserver !!!\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
            if rversion as ::core::ffi::c_int == 60 as ::core::ffi::c_int {
                if length != 55 as uint32_t && length != 71 as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"CSTOMA_REGISTER (BEGIN) - wrong size (%u/55|71)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        length,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
                if !AuthCode.is_null() {
                    if length == 55 as uint32_t {
                        let mut p: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
                        p = matocsserv_create_packet(
                            eptr,
                            MATOCS_MASTER_ACK as uint32_t,
                            33 as uint32_t,
                        );
                        put8bit(&raw mut p, 3 as uint8_t);
                        i = 0 as uint32_t;
                        while i < 32 as uint32_t {
                            (*eptr).passwordrnd[i as usize] = rndu8();
                            i = i.wrapping_add(1);
                        }
                        memcpy(
                            p as *mut ::core::ffi::c_void,
                            &raw mut (*eptr).passwordrnd as *mut uint8_t
                                as *const ::core::ffi::c_void,
                            32 as size_t,
                        );
                        return;
                    } else {
                        if matocsserv_check_password(
                            &raw mut (*eptr).passwordrnd as *mut uint8_t as *const uint8_t,
                            data as *const uint8_t,
                        ) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"CSTOMA_REGISTER (BEGIN) - access denied - check password\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                            return;
                        }
                        data = data.offset(16 as ::core::ffi::c_int as isize);
                    }
                }
                (*eptr).version = get32bit(&raw mut data);
                (*eptr).servip = get32bit(&raw mut data);
                (*eptr).servport = get16bit(&raw mut data);
                if ForceTimeout > 0 as uint32_t {
                    data = data.offset(2 as ::core::ffi::c_int as isize);
                } else {
                    (*eptr).timeout = get16bit(&raw mut data);
                }
                if sclass_ec_version() as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                    && (*eptr).version
                        < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })) as uint32_t
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"CSTOMA_REGISTER: chunkserver is too old - erasure coding needs chunkservers at least 4.x\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
                if sclass_ec_version() as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                    && (*eptr).version
                        < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 26 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })) as uint32_t
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"CSTOMA_REGISTER: chunkserver is too old - erasure coding 4+n needs chunkservers at least 4.26.x\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
                if (*eptr).timeout as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    (*eptr).timeout = DefaultTimeout as uint16_t;
                } else if ((*eptr).timeout as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"CSTOMA_REGISTER communication timeout too small (%hu seconds - should be at least 10 seconds)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        (*eptr).timeout as ::core::ffi::c_int,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
                csid = get16bit(&raw mut data);
                (*eptr).usedspace = get64bit(&raw mut data);
                (*eptr).totalspace = get64bit(&raw mut data);
                (*eptr).chunkscount = get32bit(&raw mut data);
                (*eptr).todelusedspace = get64bit(&raw mut data);
                (*eptr).todeltotalspace = get64bit(&raw mut data);
                (*eptr).todelchunkscount = get32bit(&raw mut data);
                if (*eptr).servip == 0 as uint32_t {
                    (*eptr).servip = (*eptr).peerip;
                }
                (*eptr).servip = matocsserv_remap_ip((*eptr).servip);
                if !(*eptr).servdesc.is_null() {
                    free((*eptr).servdesc as *mut ::core::ffi::c_void);
                }
                (*eptr).servdesc = univallocstripport((*eptr).servip, (*eptr).servport);
                if (*eptr).servip & 0xff000000 as uint32_t == 0x7f000000 as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"chunkserver connected using localhost (%s) - you cannot use localhost for communication between chunkserver and master\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        (*eptr).servdesc,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
                (*eptr).csptr = csdb_new_connection(
                    (*eptr).servip,
                    (*eptr).servport,
                    csid,
                    eptr as *mut ::core::ffi::c_void,
                ) as *mut csdbentry;
                if (*eptr).csptr.is_null() {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"can't accept chunkserver %s\0".as_ptr() as *const ::core::ffi::c_char,
                        (*eptr).servdesc,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
                us = (*eptr).usedspace as ::core::ffi::c_double
                    / (1024 as ::core::ffi::c_int
                        * 1024 as ::core::ffi::c_int
                        * 1024 as ::core::ffi::c_int)
                        as ::core::ffi::c_double;
                ts = (*eptr).totalspace as ::core::ffi::c_double
                    / (1024 as ::core::ffi::c_int
                        * 1024 as ::core::ffi::c_int
                        * 1024 as ::core::ffi::c_int)
                        as ::core::ffi::c_double;
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_INFO,
                    b"chunkserver %s register begin, usedspace: %lu (%.2lf GiB), totalspace: %lu (%.2lf GiB)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    (*eptr).servdesc,
                    (*eptr).usedspace,
                    us,
                    (*eptr).totalspace,
                    ts,
                );
                if (*eptr).version
                    >= (1 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 6 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 1 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            28 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            28 as ::core::ffi::c_int
                        })) as uint32_t
                {
                    let mut p_0: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
                    let mut mode: uint8_t = 0;
                    mode = (if (*eptr).version
                        >= (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                33 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                33 as ::core::ffi::c_int
                            })) as uint32_t
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint8_t;
                    p_0 = matocsserv_create_packet(
                        eptr,
                        MATOCS_MASTER_ACK as uint32_t,
                        (if mode as ::core::ffi::c_int != 0 {
                            17 as ::core::ffi::c_int
                        } else {
                            9 as ::core::ffi::c_int
                        }) as uint32_t,
                    );
                    put8bit(&raw mut p_0, 0 as uint8_t);
                    put32bit(&raw mut p_0, VERSHEX as uint32_t);
                    put16bit(&raw mut p_0, (*eptr).timeout);
                    put16bit(
                        &raw mut p_0,
                        csdb_get_csid((*eptr).csptr as *mut ::core::ffi::c_void),
                    );
                    if mode != 0 {
                        put64bit(&raw mut p_0, meta_get_id());
                    }
                }
                (*eptr).csid = chunk_server_connected(eptr as *mut ::core::ffi::c_void);
                return;
            } else if rversion as ::core::ffi::c_int == 61 as ::core::ffi::c_int {
                if length
                    .wrapping_sub(1 as uint32_t)
                    .wrapping_rem(12 as uint32_t)
                    != 0 as uint32_t
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"CSTOMA_REGISTER (CHUNKS) - wrong size (%u/1+N*12)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        length,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
                if (*eptr).csptr.is_null() {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"CSTOMA_REGISTER (CHUNKS) - CHUNKS packet before proper BEGIN packet\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
                (*eptr).newchunkdelay = NEWCHUNKDELAY as uint8_t;
                (*eptr).receivingchunks = ((*eptr).receivingchunks as ::core::ffi::c_int
                    | TRANSFERRING_NEW_CHUNKS) as uint8_t;
                receivingchunks =
                    (receivingchunks as ::core::ffi::c_int | TRANSFERRING_NEW_CHUNKS) as uint8_t;
                chunkcount = length
                    .wrapping_sub(1 as uint32_t)
                    .wrapping_div(12 as uint32_t);
                i = 0 as uint32_t;
                while i < chunkcount {
                    chunkid = get64bit(&raw mut data);
                    ecid = (chunkid >> 56 as ::core::ffi::c_int) as uint8_t;
                    chunkid = (chunkid as ::core::ffi::c_ulong
                        & 0xffffffffffffff as ::core::ffi::c_ulong)
                        as uint64_t;
                    chunkversion = get32bit(&raw mut data);
                    chunk_server_has_chunk((*eptr).csid, chunkid, ecid, chunkversion);
                    i = i.wrapping_add(1);
                }
                if (*eptr).version
                    >= (2 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 2 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })) as uint32_t
                {
                    let mut p_1: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
                    p_1 = matocsserv_create_packet(
                        eptr,
                        MATOCS_MASTER_ACK as uint32_t,
                        1 as uint32_t,
                    );
                    put8bit(&raw mut p_1, 0 as uint8_t);
                }
                return;
            } else if rversion as ::core::ffi::c_int == 62 as ::core::ffi::c_int {
                if length != 1 as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"CSTOMA_REGISTER (END) - wrong size (%u/1)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        length,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
                if (*eptr).csptr.is_null() {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"CSTOMA_REGISTER (END) - END packet before proper BEGIN packet\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_INFO,
                    b"chunkserver %s register end\0".as_ptr() as *const ::core::ffi::c_char,
                    (*eptr).servdesc,
                );
                (*eptr).registered = REGISTERED as ::core::ffi::c_int as uint8_t;
                chunk_server_register_end((*eptr).csid);
            } else if rversion as ::core::ffi::c_int == 63 as ::core::ffi::c_int {
                if length != 1 as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"CSTOMA_REGISTER (DISCONNECT) - wrong size (%u/1)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        length,
                    );
                    (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                    return;
                }
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_INFO,
                    b"chunkserver %s graceful disconnection\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*eptr).servdesc,
                );
                if !(*eptr).csptr.is_null() {
                    csdb_temporary_maintenance_mode((*eptr).csptr as *mut ::core::ffi::c_void);
                }
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            } else {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CSTOMA_REGISTER - register version not supported (%hhu/60..63)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    rversion as ::core::ffi::c_int,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
                return;
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_csdb_force_disconnect(
    mut e: *mut ::core::ffi::c_void,
    mut p: *mut ::core::ffi::c_void,
) -> uint8_t {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        if (*eptr).registered as ::core::ffi::c_int == WAITING as ::core::ffi::c_int
            && (*eptr).csptr == p as *mut csdbentry
        {
            (*eptr).csptr = ::core::ptr::null_mut::<csdbentry>();
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return 1 as uint8_t;
        }
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_labels(
    mut eptr: *mut matocsserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut i: uint32_t = 0;
        let mut l: uint32_t = 0;
        if length != 4 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CSTOMA_LABELS - wrong size (%u/4)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if data.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2762 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2762 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2762 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2762 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*eptr).labelmask = get32bit(&raw mut data);
        if !(*eptr).labelstr.is_null() {
            free((*eptr).labelstr as *mut ::core::ffi::c_void);
        }
        l = 0 as uint32_t;
        i = 0 as uint32_t;
        while i
            < (1 as ::core::ffi::c_int + 'Z' as ::core::ffi::c_int - 'A' as ::core::ffi::c_int)
                as uint32_t
        {
            if (*eptr).labelmask & (1 as uint32_t) << i != 0 {
                l = l.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        if l > 0 as uint32_t {
            l = l.wrapping_mul(2 as uint32_t);
        } else {
            l = 1 as uint32_t;
        }
        (*eptr).labelstr = malloc(l as size_t) as *mut ::core::ffi::c_char;
        if (*eptr).labelstr.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2779 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"eptr->labelstr\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2779 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"eptr->labelstr\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if (*eptr).labelstr
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut ::core::ffi::c_char
        {
            let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2779 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"eptr->labelstr\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2779 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"eptr->labelstr\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            abort();
        }
        l = 0 as uint32_t;
        i = 0 as uint32_t;
        while i
            < (1 as ::core::ffi::c_int + 'Z' as ::core::ffi::c_int - 'A' as ::core::ffi::c_int)
                as uint32_t
        {
            if (*eptr).labelmask & (1 as uint32_t) << i != 0 {
                if l > 0 as uint32_t {
                    let c2rust_fresh2 = l;
                    l = l.wrapping_add(1);
                    *(*eptr).labelstr.offset(c2rust_fresh2 as isize) = ',' as ::core::ffi::c_char;
                }
                let c2rust_fresh3 = l;
                l = l.wrapping_add(1);
                *(*eptr).labelstr.offset(c2rust_fresh3 as isize) =
                    ('A' as uint32_t).wrapping_add(i) as ::core::ffi::c_char;
            }
            i = i.wrapping_add(1);
        }
        *(*eptr).labelstr.offset(l as isize) = 0 as ::core::ffi::c_char;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_space(
    mut eptr: *mut matocsserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        if length != 16 as uint32_t && length != 32 as uint32_t && length != 40 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CSTOMA_SPACE - wrong size (%u/16|32|40)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if data.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2798 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2798 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2798 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2798 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*eptr).usedspace = get64bit(&raw mut data);
        (*eptr).totalspace = get64bit(&raw mut data);
        if length == 40 as uint32_t {
            (*eptr).chunkscount = get32bit(&raw mut data);
        }
        if length >= 32 as uint32_t {
            (*eptr).todelusedspace = get64bit(&raw mut data);
            (*eptr).todeltotalspace = get64bit(&raw mut data);
            if length == 40 as uint32_t {
                (*eptr).todelchunkscount = get32bit(&raw mut data);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_current_load(
    mut eptr: *mut matocsserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        if length < 4 as uint32_t || length > 6 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CSTOMA_CURRENT_LOAD - wrong size (%u/4-6)\0".as_ptr()
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
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2819 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2819 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2819 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr() as *const ::core::ffi::c_char,
                2819 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*eptr).load = get32bit(&raw mut data);
        if !(*eptr).csptr.is_null() {
            csdb_server_load((*eptr).csptr as *mut ::core::ffi::c_void, (*eptr).load);
        }
        if length >= 5 as uint32_t {
            (*eptr).hlstatus = get8bit(&raw mut data);
        }
        if length >= 6 as uint32_t {
            (*eptr).receivingchunks = get8bit(&raw mut data);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_chunk_damaged(
    mut eptr: *mut matocsserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut ecid: uint8_t = 0;
        let mut i: uint32_t = 0;
        if length.wrapping_rem(8 as uint32_t) != 0 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CSTOMA_CHUNK_DAMAGED - wrong size (%u/N*8)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length > 0 as uint32_t {
            if data.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2843 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"data\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2843 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2843 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"data\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2843 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"data\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
        }
        i = 0 as uint32_t;
        while i < length.wrapping_div(8 as uint32_t) {
            chunkid = get64bit(&raw mut data);
            ecid = (chunkid >> 56 as ::core::ffi::c_int) as uint8_t;
            chunkid = (chunkid as ::core::ffi::c_ulong & 0xffffffffffffff as ::core::ffi::c_ulong)
                as uint64_t;
            chunk_damaged((*eptr).csid, chunkid, ecid);
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_receiving_chunks_state() -> uint8_t {
    unsafe {
        return receivingchunks;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_chunks_delays() {
    unsafe {
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        receivingchunks = 0 as uint8_t;
        eptr = matocsservhead;
        while !eptr.is_null() {
            if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int {
                if (*eptr).version
                    < (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 32 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })) as uint32_t
                {
                    (*eptr).receivingchunks = 0 as uint8_t;
                    if (*eptr).lostchunkdelay as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                        (*eptr).lostchunkdelay = (*eptr).lostchunkdelay.wrapping_sub(1);
                        (*eptr).receivingchunks = ((*eptr).receivingchunks as ::core::ffi::c_int
                            | TRANSFERRING_LOST_CHUNKS)
                            as uint8_t;
                        receivingchunks = (receivingchunks as ::core::ffi::c_int
                            | TRANSFERRING_LOST_CHUNKS)
                            as uint8_t;
                    }
                    if (*eptr).newchunkdelay as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                        (*eptr).newchunkdelay = (*eptr).newchunkdelay.wrapping_sub(1);
                        (*eptr).receivingchunks = ((*eptr).receivingchunks as ::core::ffi::c_int
                            | TRANSFERRING_NEW_CHUNKS)
                            as uint8_t;
                        receivingchunks = (receivingchunks as ::core::ffi::c_int
                            | TRANSFERRING_NEW_CHUNKS)
                            as uint8_t;
                    }
                } else {
                    receivingchunks = (receivingchunks as ::core::ffi::c_int
                        | (*eptr).receivingchunks as ::core::ffi::c_int)
                        as uint8_t;
                }
            }
            eptr = (*eptr).next as *mut matocsserventry;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_nonexistent_chunks(
    mut eptr: *mut matocsserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut ecid: uint8_t = 0;
        let mut i: uint32_t = 0;
        if length.wrapping_rem(8 as uint32_t) != 0 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CSTOMA_CHUNK_DOESNT_EXIST - wrong size (%u/N*8)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length > 0 as uint32_t {
            if data.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2892 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"data\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2892 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2892 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"data\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2892 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"data\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
        }
        i = 0 as uint32_t;
        while i < length.wrapping_div(8 as uint32_t) {
            chunkid = get64bit(&raw mut data);
            ecid = (chunkid >> 56 as ::core::ffi::c_int) as uint8_t;
            chunkid = (chunkid as ::core::ffi::c_ulong & 0xffffffffffffff as ::core::ffi::c_ulong)
                as uint64_t;
            chunk_lost((*eptr).csid, chunkid, ecid, 1 as uint8_t);
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_chunks_lost(
    mut eptr: *mut matocsserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut ecid: uint8_t = 0;
        let mut i: uint32_t = 0;
        if length.wrapping_rem(8 as uint32_t) != 0 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CSTOMA_CHUNK_LOST - wrong size (%u/N*8)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length > 0 as uint32_t {
            if data.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2913 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"data\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2913 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2913 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"data\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2913 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"data\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
        }
        (*eptr).lostchunkdelay = LOSTCHUNKDELAY as uint8_t;
        (*eptr).receivingchunks =
            ((*eptr).receivingchunks as ::core::ffi::c_int | TRANSFERRING_LOST_CHUNKS) as uint8_t;
        receivingchunks =
            (receivingchunks as ::core::ffi::c_int | TRANSFERRING_LOST_CHUNKS) as uint8_t;
        i = 0 as uint32_t;
        while i < length.wrapping_div(8 as uint32_t) {
            chunkid = get64bit(&raw mut data);
            ecid = (chunkid >> 56 as ::core::ffi::c_int) as uint8_t;
            chunkid = (chunkid as ::core::ffi::c_ulong & 0xffffffffffffff as ::core::ffi::c_ulong)
                as uint64_t;
            chunk_lost((*eptr).csid, chunkid, ecid, 0 as uint8_t);
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_chunks_new(
    mut eptr: *mut matocsserventry,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        let mut chunkid: uint64_t = 0;
        let mut ecid: uint8_t = 0;
        let mut chunkversion: uint32_t = 0;
        let mut i: uint32_t = 0;
        if length.wrapping_rem(12 as uint32_t) != 0 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CSTOMA_CHUNK_NEW - wrong size (%u/N*12)\0".as_ptr() as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        if length > 0 as uint32_t {
            if data.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2940 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"data\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2940 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2940 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"data\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    2940 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"data\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
        }
        (*eptr).newchunkdelay = NEWCHUNKDELAY as uint8_t;
        (*eptr).receivingchunks =
            ((*eptr).receivingchunks as ::core::ffi::c_int | TRANSFERRING_NEW_CHUNKS) as uint8_t;
        receivingchunks =
            (receivingchunks as ::core::ffi::c_int | TRANSFERRING_NEW_CHUNKS) as uint8_t;
        i = 0 as uint32_t;
        while i < length.wrapping_div(12 as uint32_t) {
            chunkid = get64bit(&raw mut data);
            ecid = (chunkid >> 56 as ::core::ffi::c_int) as uint8_t;
            chunkid = (chunkid as ::core::ffi::c_ulong & 0xffffffffffffff as ::core::ffi::c_ulong)
                as uint64_t;
            chunkversion = get32bit(&raw mut data);
            chunk_server_has_chunk((*eptr).csid, chunkid, ecid, chunkversion);
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_error_occurred(
    mut eptr: *mut matocsserventry,
    _data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        if length != 0 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"CSTOMA_ERROR_OCCURRED - wrong size (%u/0)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                length,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        (*eptr).errorcounter = (*eptr).errorcounter.wrapping_add(1);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_reason_counters() {
    unsafe {
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        eptr = matocsservhead;
        while !eptr.is_null() {
            if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int {
                memcpy(
                    &raw mut (*eptr).lreplreadok as *mut uint32_t as *mut ::core::ffi::c_void,
                    &raw mut (*eptr).replreadok as *mut uint32_t as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<uint32_t>()
                        .wrapping_mul(REPL_REASONS as ::core::ffi::c_int as size_t),
                );
                memcpy(
                    &raw mut (*eptr).lreplreaderr as *mut uint32_t as *mut ::core::ffi::c_void,
                    &raw mut (*eptr).replreaderr as *mut uint32_t as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<uint32_t>()
                        .wrapping_mul(REPL_REASONS as ::core::ffi::c_int as size_t),
                );
                memcpy(
                    &raw mut (*eptr).lreplwriteok as *mut uint32_t as *mut ::core::ffi::c_void,
                    &raw mut (*eptr).replwriteok as *mut uint32_t as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<uint32_t>()
                        .wrapping_mul(REPL_REASONS as ::core::ffi::c_int as size_t),
                );
                memcpy(
                    &raw mut (*eptr).lreplwriteerr as *mut uint32_t as *mut ::core::ffi::c_void,
                    &raw mut (*eptr).replwriteerr as *mut uint32_t as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<uint32_t>()
                        .wrapping_mul(REPL_REASONS as ::core::ffi::c_int as size_t),
                );
                memcpy(
                    &raw mut (*eptr).ldelok as *mut uint32_t as *mut ::core::ffi::c_void,
                    &raw mut (*eptr).delok as *mut uint32_t as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<uint32_t>()
                        .wrapping_mul(OP_REASONS as ::core::ffi::c_int as size_t),
                );
                memcpy(
                    &raw mut (*eptr).ldelerr as *mut uint32_t as *mut ::core::ffi::c_void,
                    &raw mut (*eptr).delerr as *mut uint32_t as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<uint32_t>()
                        .wrapping_mul(OP_REASONS as ::core::ffi::c_int as size_t),
                );
                memset(
                    &raw mut (*eptr).replreadok as *mut uint32_t as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<uint32_t>()
                        .wrapping_mul(REPL_REASONS as ::core::ffi::c_int as size_t),
                );
                memset(
                    &raw mut (*eptr).replreaderr as *mut uint32_t as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<uint32_t>()
                        .wrapping_mul(REPL_REASONS as ::core::ffi::c_int as size_t),
                );
                memset(
                    &raw mut (*eptr).replwriteok as *mut uint32_t as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<uint32_t>()
                        .wrapping_mul(REPL_REASONS as ::core::ffi::c_int as size_t),
                );
                memset(
                    &raw mut (*eptr).replwriteerr as *mut uint32_t as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<uint32_t>()
                        .wrapping_mul(REPL_REASONS as ::core::ffi::c_int as size_t),
                );
                memset(
                    &raw mut (*eptr).delok as *mut uint32_t as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<uint32_t>()
                        .wrapping_mul(OP_REASONS as ::core::ffi::c_int as size_t),
                );
                memset(
                    &raw mut (*eptr).delerr as *mut uint32_t as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<uint32_t>()
                        .wrapping_mul(OP_REASONS as ::core::ffi::c_int as size_t),
                );
            }
            eptr = (*eptr).next as *mut matocsserventry;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_broadcast_regfirst_chunk(mut chunkid: uint64_t) {
    unsafe {
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        eptr = matocsservhead;
        while !eptr.is_null() {
            if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
                && (*eptr).registered as ::core::ffi::c_int != REGISTERED as ::core::ffi::c_int
                && (*eptr).version
                    >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                        + 30 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                        + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })) as uint32_t
            {
                data = matocsserv_create_packet(
                    eptr,
                    MATOCS_REGISTER_FIRST as uint32_t,
                    8 as uint32_t,
                );
                put64bit(&raw mut data, chunkid);
            }
            eptr = (*eptr).next as *mut matocsserventry;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_broadcast_timeout() {
    unsafe {
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        let mut data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if ForceTimeout > 0 as uint32_t {
            eptr = matocsservhead;
            while !eptr.is_null() {
                if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
                    && (*eptr).version
                        >= (4 as ::core::ffi::c_int * 0x10000 as ::core::ffi::c_int
                            + 12 as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
                            + (if 4 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                                0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })) as uint32_t
                {
                    (*eptr).timeout = ForceTimeout as uint16_t;
                    data = matocsserv_create_packet(
                        eptr,
                        ANTOAN_FORCE_TIMEOUT as uint32_t,
                        2 as uint32_t,
                    );
                    put16bit(&raw mut data, ForceTimeout as uint16_t);
                }
                eptr = (*eptr).next as *mut matocsserventry;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_get_min_cs_version() -> uint32_t {
    unsafe {
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        let mut minver: uint32_t = 0 as uint32_t;
        eptr = matocsservhead;
        while !eptr.is_null() {
            if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                && !(*eptr).csptr.is_null()
            {
                if minver == 0 as uint32_t || (*eptr).version < minver {
                    minver = (*eptr).version;
                }
            }
            eptr = (*eptr).next as *mut matocsserventry;
        }
        return minver;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_isvalid(mut e: *mut ::core::ffi::c_void) -> uint8_t {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        return (if (*eptr).mode as ::core::ffi::c_int == KILL as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_disconnection_finished(mut e: *mut ::core::ffi::c_void) {
    unsafe {
        let mut eptr: *mut matocsserventry = e as *mut matocsserventry;
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_INFO,
            b"server %s has been fully removed from data structures\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*eptr).servdesc,
        );
        if !(*eptr).servdesc.is_null() {
            free((*eptr).servdesc as *mut ::core::ffi::c_void);
        }
        free(eptr as *mut ::core::ffi::c_void);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_gotpacket(
    mut eptr: *mut matocsserventry,
    mut r#type: uint32_t,
    mut data: *const uint8_t,
    mut length: uint32_t,
) {
    unsafe {
        if r#type != CSTOMA_REGISTER as uint32_t
            && r#type != ANTOAN_NOP as uint32_t
            && (*eptr).csid as ::core::ffi::c_int == MAXCSCOUNT
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"got command type %u from unregistered chunk server\0".as_ptr()
                    as *const ::core::ffi::c_char,
                r#type,
            );
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            return;
        }
        match r#type {
            0 | 1 | 2 => {}
            10 => {
                matocsserv_get_version(eptr, data, length);
            }
            80 => {
                matocsserv_get_config(eptr, data, length);
            }
            71 => {
                matocsserv_syslog(eptr, data, length);
            }
            98 => {
                matocsserv_got_chunk_status(eptr, data, length);
            }
            100 => {
                matocsserv_register(eptr, data, length);
            }
            101 => {
                matocsserv_space(eptr, data, length);
            }
            103 => {
                matocsserv_current_load(eptr, data, length);
            }
            102 => {
                matocsserv_chunk_damaged(eptr, data, length);
            }
            105 => {
                matocsserv_chunks_lost(eptr, data, length);
            }
            107 => {
                matocsserv_chunks_new(eptr, data, length);
            }
            96 => {
                matocsserv_nonexistent_chunks(eptr, data, length);
            }
            106 => {
                matocsserv_error_occurred(eptr, data, length);
            }
            109 => {
                matocsserv_labels(eptr, data, length);
            }
            301 => {
                matocsserv_got_chunk_checksum(eptr, data, length);
            }
            111 => {
                matocsserv_got_createchunk_status(eptr, data, length);
            }
            121 => {
                matocsserv_got_deletechunk_status(eptr, data, length);
            }
            151 | 155 | 157 | 159 => {
                matocsserv_got_replicatechunk_status(eptr, data, length);
            }
            131 => {
                matocsserv_got_duplicatechunk_status(eptr, data, length);
            }
            141 => {
                matocsserv_got_setchunkversion_status(eptr, data, length);
            }
            161 => {
                matocsserv_got_truncatechunk_status(eptr, data, length);
            }
            171 => {
                matocsserv_got_duptruncchunk_status(eptr, data, length);
            }
            181 => {
                matocsserv_got_localsplitchunk_status(eptr, data, length);
            }
            _ => {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"master <-> chunkservers module: got unknown message (type:%u)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    r#type,
                );
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_read(
    mut eptr: *mut matocsserventry,
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
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    3153 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    3153 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    3153 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    3153 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                        b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        3174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        3174 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                        b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        3174 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"readbuff\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        3174 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                        b"CS(%s) packet too long (%u/%u) ; command:%u\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*eptr).servdesc,
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
                        b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        3214 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr->input_packet\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        3214 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                        b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        3214 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr->input_packet\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_1,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        3214 as ::core::ffi::c_int as ::core::ffi::c_uint,
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
                b"connection with CS(%s) has been closed by peer\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*eptr).servdesc,
            );
            (*eptr).input_end = 1 as uint8_t;
        } else if err != 0 {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"read from CS(%s) error\0".as_ptr() as *const ::core::ffi::c_char,
                (*eptr).servdesc,
            );
            (*eptr).input_end = 1 as uint8_t;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_parse(mut eptr: *mut matocsserventry) {
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
            matocsserv_gotpacket(
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
pub unsafe extern "C" fn matocsserv_write(
    mut eptr: *mut matocsserventry,
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
                        b"write to CS(%s) error\0".as_ptr() as *const ::core::ffi::c_char,
                        (*eptr).servdesc,
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
pub unsafe extern "C" fn matocsserv_desc(mut pdesc: *mut pollfd, mut ndesc: *mut uint32_t) {
    unsafe {
        let mut pos: uint32_t = *ndesc;
        let mut events: ::core::ffi::c_int = 0;
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        (*pdesc.offset(pos as isize)).fd = lsock;
        (*pdesc.offset(pos as isize)).events = POLLIN as ::core::ffi::c_short;
        lsockpdescpos = pos as int32_t;
        pos = pos.wrapping_add(1);
        eptr = matocsservhead;
        while !eptr.is_null() {
            events = 0 as ::core::ffi::c_int;
            if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                && (*eptr).input_end as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                events |= POLLIN;
            }
            if (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                && !(*eptr).outputhead.is_null()
            {
                events |= POLLOUT;
            }
            if events != 0 {
                (*pdesc.offset(pos as isize)).events = events as ::core::ffi::c_short;
                (*pdesc.offset(pos as isize)).fd = (*eptr).sock;
                (*eptr).pdescpos = pos as int32_t;
                pos = pos.wrapping_add(1);
            } else {
                (*eptr).pdescpos = -1 as ::core::ffi::c_int as int32_t;
            }
            eptr = (*eptr).next as *mut matocsserventry;
        }
        *ndesc = pos;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_disconnection_loop() {
    unsafe {
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        let mut kptr: *mut *mut matocsserventry = ::core::ptr::null_mut::<*mut matocsserventry>();
        let mut ipptr: *mut in_packetstruct = ::core::ptr::null_mut::<in_packetstruct>();
        let mut ipaptr: *mut in_packetstruct = ::core::ptr::null_mut::<in_packetstruct>();
        let mut opptr: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
        let mut opaptr: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
        kptr = &raw mut matocsservhead;
        loop {
            eptr = *kptr;
            if eptr.is_null() {
                break;
            }
            if (*eptr).mode as ::core::ffi::c_int == KILL as ::core::ffi::c_int {
                let mut us: ::core::ffi::c_double = 0.;
                let mut ts: ::core::ffi::c_double = 0.;
                us = (*eptr).usedspace as ::core::ffi::c_double
                    / (1024 as ::core::ffi::c_int
                        * 1024 as ::core::ffi::c_int
                        * 1024 as ::core::ffi::c_int)
                        as ::core::ffi::c_double;
                ts = (*eptr).totalspace as ::core::ffi::c_double
                    / (1024 as ::core::ffi::c_int
                        * 1024 as ::core::ffi::c_int
                        * 1024 as ::core::ffi::c_int)
                        as ::core::ffi::c_double;
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_INFO,
                    b"chunkserver %s disconnected, usedspace: %lu (%.2lf GiB), totalspace: %lu (%.2lf GiB)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    (*eptr).servdesc,
                    (*eptr).usedspace,
                    us,
                    (*eptr).totalspace,
                    ts,
                );
                matocsserv_replication_disconnected(eptr as *mut ::core::ffi::c_void);
                matocsserv_operation_disconnected(eptr as *mut ::core::ffi::c_void);
                if (*eptr).csid as ::core::ffi::c_int != MAXCSCOUNT {
                    chunk_server_disconnected((*eptr).csid);
                }
                csdb_lost_connection((*eptr).csptr as *mut ::core::ffi::c_void);
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
                *kptr = (*eptr).next as *mut matocsserventry;
                (*eptr).next = ::core::ptr::null_mut::<matocsserventry>();
                if (*eptr).csid as ::core::ffi::c_int == MAXCSCOUNT {
                    if !(*eptr).servdesc.is_null() {
                        free((*eptr).servdesc as *mut ::core::ffi::c_void);
                    }
                    free(eptr as *mut ::core::ffi::c_void);
                }
            } else {
                kptr = &raw mut (*eptr).next as *mut *mut matocsserventry;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_serve(mut pdesc: *mut pollfd) {
    unsafe {
        let mut now: ::core::ffi::c_double = 0.;
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        let mut ns: ::core::ffi::c_int = 0;
        static mut lastaction: ::core::ffi::c_double = 0.0f64;
        let mut timeoutadd: ::core::ffi::c_double = 0.;
        now = monotonic_seconds();
        if lastaction > 0.0f64 {
            timeoutadd = now - lastaction;
            if timeoutadd > 1.0f64 {
                eptr = matocsservhead;
                while !eptr.is_null() {
                    (*eptr).lastread += timeoutadd;
                    eptr = (*eptr).next as *mut matocsserventry;
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
                    b"Master<->CS socket: accept error\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                tcpnonblock(ns);
                tcpnodelay(ns);
                eptr = malloc(::core::mem::size_of::<matocsserventry>()) as *mut matocsserventry;
                if eptr.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        3453 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        3453 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if eptr
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut matocsserventry
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        3453 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/matocsserv.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        3453 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"eptr\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
                (*eptr).next = matocsservhead as *mut matocsserventry;
                matocsservhead = eptr;
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
                tcpgetpeer(
                    (*eptr).sock,
                    &raw mut (*eptr).peerip,
                    ::core::ptr::null_mut::<uint16_t>(),
                );
                (*eptr).servdesc = univallocstripport((*eptr).peerip, 0 as uint16_t);
                (*eptr).version = 0 as uint32_t;
                (*eptr).servip = 0 as uint32_t;
                (*eptr).servport = 0 as uint16_t;
                if ForceTimeout > 0 as uint32_t {
                    (*eptr).timeout = ForceTimeout as uint16_t;
                } else {
                    (*eptr).timeout = DefaultTimeout as uint16_t;
                }
                (*eptr).load = 0 as uint32_t;
                (*eptr).hlstatus = HLSTATUS_DEFAULT as uint8_t;
                (*eptr).usedspace = 0 as uint64_t;
                (*eptr).totalspace = 0 as uint64_t;
                (*eptr).chunkscount = 0 as uint32_t;
                (*eptr).todelusedspace = 0 as uint64_t;
                (*eptr).todeltotalspace = 0 as uint64_t;
                (*eptr).todelchunkscount = 0 as uint32_t;
                (*eptr).errorcounter = 0 as uint32_t;
                (*eptr).writecounter = 0 as uint16_t;
                (*eptr).rrepcounter = 0 as uint16_t;
                (*eptr).wrepcounter = 0 as uint16_t;
                (*eptr).delcounter = 0 as uint16_t;
                (*eptr).labelmask = 0 as uint32_t;
                (*eptr).labelstr = ::core::ptr::null_mut::<::core::ffi::c_char>();
                (*eptr).create_total_counter = 0 as uint32_t;
                (*eptr).rrep_total_counter = 0 as uint32_t;
                (*eptr).wrep_total_counter = 0 as uint32_t;
                (*eptr).del_total_counter = 0 as uint32_t;
                (*eptr).total_counter_begin = monotonic_seconds();
                (*eptr).csid = MAXCSCOUNT as uint16_t;
                (*eptr).registered = UNREGISTERED as ::core::ffi::c_int as uint8_t;
                (*eptr).lostchunkdelay = LOSTCHUNKDELAY as uint8_t;
                (*eptr).newchunkdelay = NEWCHUNKDELAY as uint8_t;
                (*eptr).receivingchunks =
                    (TRANSFERRING_NEW_CHUNKS | TRANSFERRING_LOST_CHUNKS) as uint8_t;
                memset(
                    &raw mut (*eptr).passwordrnd as *mut uint8_t as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    32 as size_t,
                );
                memset(
                    &raw mut (*eptr).lreplreadok as *mut uint32_t as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<uint32_t>()
                        .wrapping_mul(REPL_REASONS as ::core::ffi::c_int as size_t),
                );
                memset(
                    &raw mut (*eptr).lreplreaderr as *mut uint32_t as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<uint32_t>()
                        .wrapping_mul(REPL_REASONS as ::core::ffi::c_int as size_t),
                );
                memset(
                    &raw mut (*eptr).lreplwriteok as *mut uint32_t as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<uint32_t>()
                        .wrapping_mul(REPL_REASONS as ::core::ffi::c_int as size_t),
                );
                memset(
                    &raw mut (*eptr).lreplwriteerr as *mut uint32_t as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<uint32_t>()
                        .wrapping_mul(REPL_REASONS as ::core::ffi::c_int as size_t),
                );
                memset(
                    &raw mut (*eptr).ldelok as *mut uint32_t as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<uint32_t>()
                        .wrapping_mul(OP_REASONS as ::core::ffi::c_int as size_t),
                );
                memset(
                    &raw mut (*eptr).ldelerr as *mut uint32_t as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<uint32_t>()
                        .wrapping_mul(OP_REASONS as ::core::ffi::c_int as size_t),
                );
                memset(
                    &raw mut (*eptr).replreadok as *mut uint32_t as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<uint32_t>()
                        .wrapping_mul(REPL_REASONS as ::core::ffi::c_int as size_t),
                );
                memset(
                    &raw mut (*eptr).replreaderr as *mut uint32_t as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<uint32_t>()
                        .wrapping_mul(REPL_REASONS as ::core::ffi::c_int as size_t),
                );
                memset(
                    &raw mut (*eptr).replwriteok as *mut uint32_t as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<uint32_t>()
                        .wrapping_mul(REPL_REASONS as ::core::ffi::c_int as size_t),
                );
                memset(
                    &raw mut (*eptr).replwriteerr as *mut uint32_t as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<uint32_t>()
                        .wrapping_mul(REPL_REASONS as ::core::ffi::c_int as size_t),
                );
                memset(
                    &raw mut (*eptr).delok as *mut uint32_t as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<uint32_t>()
                        .wrapping_mul(OP_REASONS as ::core::ffi::c_int as size_t),
                );
                memset(
                    &raw mut (*eptr).delerr as *mut uint32_t as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<uint32_t>()
                        .wrapping_mul(OP_REASONS as ::core::ffi::c_int as size_t),
                );
                (*eptr).dist = 0 as uint32_t;
                (*eptr).first = 1 as uint8_t;
                (*eptr).corr = 0.0f64;
                (*eptr).csptr = ::core::ptr::null_mut::<csdbentry>();
            }
        }
        eptr = matocsservhead;
        while !eptr.is_null() {
            if (*eptr).pdescpos >= 0 as int32_t {
                if (*pdesc.offset((*eptr).pdescpos as isize)).revents as ::core::ffi::c_int
                    & (POLLERR | POLLIN)
                    == POLLIN
                    && (*eptr).mode as ::core::ffi::c_int != KILL as ::core::ffi::c_int
                {
                    matocsserv_read(eptr, now);
                }
                if (*pdesc.offset((*eptr).pdescpos as isize)).revents as ::core::ffi::c_int
                    & (POLLERR | POLLHUP)
                    != 0
                {
                    (*eptr).input_end = 1 as uint8_t;
                }
            }
            matocsserv_parse(eptr);
            eptr = (*eptr).next as *mut matocsserventry;
        }
        eptr = matocsservhead;
        while !eptr.is_null() {
            if (*eptr).lastwrite + 1.0f64 < now && (*eptr).outputhead.is_null() {
                matocsserv_create_packet(eptr, ANTOAN_NOP as uint32_t, 0 as uint32_t);
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
                    matocsserv_write(eptr, now);
                }
            }
            if ((*eptr).lastread + (*eptr).timeout as ::core::ffi::c_int as ::core::ffi::c_double)
                < now
            {
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            }
            if (*eptr).mode as ::core::ffi::c_int == FINISH as ::core::ffi::c_int
                && (*eptr).outputhead.is_null()
            {
                (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            }
            eptr = (*eptr).next as *mut matocsserventry;
        }
        matocsserv_disconnection_loop();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_keep_alive() {
    unsafe {
        let mut now: ::core::ffi::c_double = 0.;
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        now = monotonic_seconds();
        eptr = matocsservhead;
        while !eptr.is_null() {
            if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
                && (*eptr).input_end as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                matocsserv_read(eptr, now);
            }
            eptr = (*eptr).next as *mut matocsserventry;
        }
        eptr = matocsservhead;
        while !eptr.is_null() {
            if (*eptr).lastwrite + 2.0f64 < now && (*eptr).outputhead.is_null() {
                matocsserv_create_packet(eptr, ANTOAN_NOP as uint32_t, 0 as uint32_t);
            }
            if (*eptr).mode as ::core::ffi::c_int == DATA as ::core::ffi::c_int
                && !(*eptr).outputhead.is_null()
            {
                matocsserv_write(eptr, now);
            }
            eptr = (*eptr).next as *mut matocsserventry;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_close_lsock() {
    unsafe {
        if lsock >= 0 as ::core::ffi::c_int {
            close(lsock);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_term() {
    unsafe {
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        let mut eaptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        let mut ipptr: *mut in_packetstruct = ::core::ptr::null_mut::<in_packetstruct>();
        let mut ipaptr: *mut in_packetstruct = ::core::ptr::null_mut::<in_packetstruct>();
        let mut opptr: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
        let mut opaptr: *mut out_packetstruct = ::core::ptr::null_mut::<out_packetstruct>();
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_INFO,
            b"master <-> chunkservers module: closing %s:%s\0".as_ptr()
                as *const ::core::ffi::c_char,
            ListenHost,
            ListenPort,
        );
        tcpclose(lsock);
        eptr = matocsservhead;
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
            if !(*eptr).servdesc.is_null() {
                free((*eptr).servdesc as *mut ::core::ffi::c_void);
            }
            eaptr = eptr;
            eptr = (*eptr).next as *mut matocsserventry;
            free(eaptr as *mut ::core::ffi::c_void);
        }
        matocsservhead = ::core::ptr::null_mut::<matocsserventry>();
        matocsserv_read(::core::ptr::null_mut::<matocsserventry>(), 0.0f64);
        matocsserv_getservers_wrandom(
            ::core::ptr::null_mut::<uint16_t>(),
            ::core::ptr::null_mut::<uint16_t>(),
        );
        free(ListenHost as *mut ::core::ffi::c_void);
        free(ListenPort as *mut ::core::ffi::c_void);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_no_more_pending_jobs() -> ::core::ffi::c_int {
    unsafe {
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        eptr = matocsservhead;
        while !eptr.is_null() {
            if !(*eptr).outputhead.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            if (*eptr).rrepcounter as ::core::ffi::c_int
                | (*eptr).wrepcounter as ::core::ffi::c_int
                | (*eptr).delcounter as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            eptr = (*eptr).next as *mut matocsserventry;
        }
        return 1 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_disconnect_all() {
    unsafe {
        let mut eptr: *mut matocsserventry = ::core::ptr::null_mut::<matocsserventry>();
        eptr = matocsservhead;
        while !eptr.is_null() {
            (*eptr).mode = KILL as ::core::ffi::c_int as uint8_t;
            eptr = (*eptr).next as *mut matocsserventry;
        }
        matocsserv_disconnection_loop();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_parse_ip(
    mut ipstr: *const ::core::ffi::c_char,
    mut ipnum: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut ip: uint32_t = 0;
        let mut octet: uint32_t = 0;
        let mut i: uint32_t = 0;
        ip = 0 as uint32_t;
        while *ipstr as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || *ipstr as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
        {
            ipstr = ipstr.offset(1);
        }
        i = 0 as uint32_t;
        while i < 4 as uint32_t {
            if *ipstr as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                && *ipstr as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
            {
                octet = 0 as uint32_t;
                while *ipstr as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && *ipstr as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    octet = octet.wrapping_mul(10 as uint32_t);
                    octet = octet.wrapping_add(
                        (*ipstr as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as uint32_t,
                    );
                    ipstr = ipstr.offset(1);
                    if octet > 255 as uint32_t {
                        return 0 as uint8_t;
                    }
                }
            } else {
                return 0 as uint8_t;
            }
            if i < 3 as uint32_t {
                if *ipstr as ::core::ffi::c_int != '.' as ::core::ffi::c_int {
                    return 0 as uint8_t;
                }
                ipstr = ipstr.offset(1);
            } else if *ipstr as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                && *ipstr as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
                && *ipstr as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
                && *ipstr as ::core::ffi::c_int != '\r' as ::core::ffi::c_int
                && *ipstr as ::core::ffi::c_int != '\n' as ::core::ffi::c_int
            {
                return 0 as uint8_t;
            }
            ip = ip.wrapping_mul(256 as uint32_t);
            ip = ip.wrapping_add(octet);
            i = i.wrapping_add(1);
        }
        *ipnum = ip;
        return 1 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_reload_common() {
    unsafe {
        let mut bits: uint8_t = 0;
        let mut err: uint8_t = 0;
        let mut ipbuff: [::core::ffi::c_char; 16] = [0; 16];
        let mut srcip: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut dstip: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut srcclass: uint32_t = 0;
        let mut dstclass: uint32_t = 0;
        let mut mask: uint32_t = 0;
        let mut reservespace: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut reservespacevalue: ::core::ffi::c_double = 0.;
        ChunkServerCheck = cfg_getuint8(
            b"MATOCS_CHUNK_SERVER_CHECK_MODE\0".as_ptr() as *const ::core::ffi::c_char,
            0 as uint8_t,
        );
        DefaultTimeout = cfg_getuint32(
            b"MATOCS_TIMEOUT\0".as_ptr() as *const ::core::ffi::c_char,
            10 as uint32_t,
        );
        if DefaultTimeout > 65535 as uint32_t {
            DefaultTimeout = 65535 as uint32_t;
        } else if DefaultTimeout < 10 as uint32_t {
            DefaultTimeout = 10 as uint32_t;
        }
        ForceTimeout = cfg_getuint32(
            b"MATOCS_FORCE_TIMEOUT\0".as_ptr() as *const ::core::ffi::c_char,
            0 as uint32_t,
        );
        if ForceTimeout > 0 as uint32_t && ForceTimeout < 10 as uint32_t {
            ForceTimeout = 10 as uint32_t;
        }
        if ForceTimeout > 65535 as uint32_t {
            ForceTimeout = 65535 as uint32_t;
        }
        if !AuthCode.is_null() {
            free(AuthCode as *mut ::core::ffi::c_void);
            AuthCode = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if cfg_isdefined(b"AUTH_CODE\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
            AuthCode = cfg_getstr(
                b"AUTH_CODE\0".as_ptr() as *const ::core::ffi::c_char,
                b"mfspassword\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        reservespace = cfg_getstr(
            b"RESERVE_SPACE\0".as_ptr() as *const ::core::ffi::c_char,
            b"0\0".as_ptr() as *const ::core::ffi::c_char,
        );
        reservespacevalue = sizestrtod(reservespace, &raw mut endptr);
        if *endptr.offset(0 as isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            || *endptr.offset(0 as isize) as ::core::ffi::c_int == 'B' as ::core::ffi::c_int
                && *endptr.offset(1 as isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        {
            ReserveSpaceValue = reservespacevalue;
            ReserveSpaceMode = RESERVE_BYTES as ::core::ffi::c_int as uint8_t;
        } else if *endptr.offset(0 as isize) as ::core::ffi::c_int == '%' as ::core::ffi::c_int
            && *endptr.offset(1 as isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        {
            ReserveSpaceValue = reservespacevalue;
            ReserveSpaceMode = RESERVE_PERCENT as ::core::ffi::c_int as uint8_t;
        } else if *endptr.offset(0 as isize) as ::core::ffi::c_int == 'U' as ::core::ffi::c_int
            && *endptr.offset(1 as isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        {
            ReserveSpaceValue = reservespacevalue;
            ReserveSpaceMode = RESERVE_CHUNKSERVER_USED as ::core::ffi::c_int as uint8_t;
        } else if *endptr.offset(0 as isize) as ::core::ffi::c_int == 'C' as ::core::ffi::c_int
            && *endptr.offset(1 as isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        {
            ReserveSpaceValue = reservespacevalue;
            ReserveSpaceMode = RESERVE_CHUNKSERVER_TOTAL as ::core::ffi::c_int as uint8_t;
        } else {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"error parsing RESERVE_SPACE (\"%s\") ; error on '%c'\0".as_ptr()
                    as *const ::core::ffi::c_char,
                reservespace,
                *endptr.offset(0 as isize) as ::core::ffi::c_int,
            );
        }
        free(reservespace as *mut ::core::ffi::c_void);
        if cfg_isdefined(b"REMAP_BITS\0".as_ptr() as *const ::core::ffi::c_char) != 0
            && cfg_isdefined(b"REMAP_SOURCE_IP_CLASS\0".as_ptr() as *const ::core::ffi::c_char) != 0
            && cfg_isdefined(b"REMAP_DESTINATION_IP_CLASS\0".as_ptr() as *const ::core::ffi::c_char)
                != 0
        {
            bits = cfg_getuint8(
                b"REMAP_BITS\0".as_ptr() as *const ::core::ffi::c_char,
                24 as uint8_t,
            );
            srcip = cfg_getstr(
                b"REMAP_SOURCE_IP_CLASS\0".as_ptr() as *const ::core::ffi::c_char,
                b"192.168.1.0\0".as_ptr() as *const ::core::ffi::c_char,
            );
            dstip = cfg_getstr(
                b"REMAP_DESTINATION_IP_CLASS\0".as_ptr() as *const ::core::ffi::c_char,
                b"10.0.0.0\0".as_ptr() as *const ::core::ffi::c_char,
            );
            err = 0 as uint8_t;
            if bits as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || bits as ::core::ffi::c_int > 32 as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"wrong value for REMAP_BITS (%hhu ; shlould be between 1 and 32)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    bits as ::core::ffi::c_int,
                );
                err = (err as ::core::ffi::c_int | 1 as ::core::ffi::c_int) as uint8_t;
                mask = 0 as uint32_t;
            } else {
                mask = ((0xffffffff as ::core::ffi::c_uint)
                    << 32 as ::core::ffi::c_int - bits as ::core::ffi::c_int)
                    as uint32_t;
            }
            if matocsserv_parse_ip(srcip, &raw mut srcclass) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"error parsing ip class from REMAP_SOURCE_IP_CLASS (%s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    srcip,
                );
                err = (err as ::core::ffi::c_int | 2 as ::core::ffi::c_int) as uint8_t;
            }
            if matocsserv_parse_ip(dstip, &raw mut dstclass) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"error parsing ip class from REMAP_DESTINATION_IP_CLASS (%s)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    dstip,
                );
                err = (err as ::core::ffi::c_int | 4 as ::core::ffi::c_int) as uint8_t;
            }
            if err as ::core::ffi::c_int & 3 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && srcclass & mask != srcclass
            {
                univmakestrip(&raw mut ipbuff as *mut ::core::ffi::c_char, srcclass & mask);
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"found garbage bits at the end of REMAP_SOURCE_IP_CLASS (given: %s - should be: %s)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    srcip,
                    &raw mut ipbuff as *mut ::core::ffi::c_char,
                );
                err = (err as ::core::ffi::c_int | 8 as ::core::ffi::c_int) as uint8_t;
            }
            if err as ::core::ffi::c_int & 5 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && dstclass & mask != dstclass
            {
                univmakestrip(&raw mut ipbuff as *mut ::core::ffi::c_char, dstclass & mask);
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"found garbage bits at the end of REMAP_DESTINATION_IP_CLASS (given: %s - should be: %s)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    dstip,
                    &raw mut ipbuff as *mut ::core::ffi::c_char,
                );
                err = (err as ::core::ffi::c_int | 16 as ::core::ffi::c_int) as uint8_t;
            }
            if err as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                RemapMask = mask;
                RemapSrc = srcclass;
                RemapDst = dstclass;
            }
            free(srcip as *mut ::core::ffi::c_void);
            free(dstip as *mut ::core::ffi::c_void);
        } else {
            RemapMask = 0 as uint32_t;
            RemapSrc = 0 as uint32_t;
            RemapDst = 0 as uint32_t;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn matocsserv_reload() {
    unsafe {
        let mut oldListenHost: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut oldListenPort: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut oldlistenip: uint32_t = 0;
        let mut oldlistenport: uint16_t = 0;
        let mut newlsock: ::core::ffi::c_int = 0;
        matocsserv_reload_common();
        oldListenHost = ListenHost;
        oldListenPort = ListenPort;
        oldlistenip = listenip;
        oldlistenport = listenport;
        ListenHost = cfg_getstr(
            b"MATOCS_LISTEN_HOST\0".as_ptr() as *const ::core::ffi::c_char,
            b"*\0".as_ptr() as *const ::core::ffi::c_char,
        );
        ListenPort = cfg_getstr(
            b"MATOCS_LISTEN_PORT\0".as_ptr() as *const ::core::ffi::c_char,
            DEFAULT_MASTER_CS_PORT.as_ptr(),
        );
        if strcmp(oldListenHost, ListenHost) == 0 as ::core::ffi::c_int
            && strcmp(oldListenPort, ListenPort) == 0 as ::core::ffi::c_int
        {
            free(oldListenHost as *mut ::core::ffi::c_void);
            free(oldListenPort as *mut ::core::ffi::c_void);
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_INFO,
                b"master <-> chunkservers module: socket address hasn't changed (%s:%s)\0".as_ptr()
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
                b"master <-> chunkservers module: socket address has changed, but can't create new socket\0"
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
                b"master <-> chunkservers module: socket address has changed, but can't be resolved (%s:%s)\0"
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
                b"master <-> chunkservers module: socket address has changed, but can't listen on socket (%s:%s)\0"
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
                b"master <-> chunkservers module: can't set accept filter\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_INFO,
            b"master <-> chunkservers module: socket address has changed, now listen on %s:%s\0"
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
pub unsafe extern "C" fn matocsserv_init() -> ::core::ffi::c_int {
    unsafe {
        matocsserv_reload_common();
        ListenHost = cfg_getstr(
            b"MATOCS_LISTEN_HOST\0".as_ptr() as *const ::core::ffi::c_char,
            b"*\0".as_ptr() as *const ::core::ffi::c_char,
        );
        ListenPort = cfg_getstr(
            b"MATOCS_LISTEN_PORT\0".as_ptr() as *const ::core::ffi::c_char,
            DEFAULT_MASTER_CS_PORT.as_ptr(),
        );
        lsock = tcpsocket();
        if lsock < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"master <-> chunkservers module: can't create socket\0".as_ptr()
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
                b"master <-> chunkservers module: can't resolve %s:%s\0".as_ptr()
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
                b"master <-> chunkservers module: can't listen on %s:%s\0".as_ptr()
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
                b"master <-> chunkservers module: can't set accept filter\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_INFO,
            b"master <-> chunkservers module: listen on %s:%s\0".as_ptr()
                as *const ::core::ffi::c_char,
            ListenHost,
            ListenPort,
        );
        matocsserv_replication_init();
        matocsservhead = ::core::ptr::null_mut::<matocsserventry>();
        receivingchunks = (TRANSFERRING_NEW_CHUNKS | TRANSFERRING_LOST_CHUNKS) as uint8_t;
        main_reload_register_fname(
            Some(matocsserv_reload as unsafe extern "C" fn() -> ()),
            b"matocsserv_reload\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_destruct_register_fname(
            Some(matocsserv_term as unsafe extern "C" fn() -> ()),
            b"matocsserv_term\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_poll_register_fname(
            Some(matocsserv_desc as unsafe extern "C" fn(*mut pollfd, *mut uint32_t) -> ()),
            Some(matocsserv_serve as unsafe extern "C" fn(*mut pollfd) -> ()),
            b"matocsserv_desc\0".as_ptr() as *const ::core::ffi::c_char,
            b"matocsserv_serve\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_keepalive_register_fname(
            Some(matocsserv_keep_alive as unsafe extern "C" fn() -> ()),
            b"matocsserv_keep_alive\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_info_register_fname(
            Some(matocsserv_log_extra_info as unsafe extern "C" fn(*mut FILE) -> ()),
            b"matocsserv_log_extra_info\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_time_register_fname(
            1 as uint32_t,
            0 as uint32_t,
            Some(matocsserv_hlstatus_fix as unsafe extern "C" fn() -> ()),
            b"matocsserv_hlstatus_fix\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_time_register_fname(
            1 as uint32_t,
            0 as uint32_t,
            Some(matocsserv_calculate_space as unsafe extern "C" fn() -> ()),
            b"matocsserv_calculate_space\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_time_register_fname(
            1 as uint32_t,
            0 as uint32_t,
            Some(matocsserv_chunks_delays as unsafe extern "C" fn() -> ()),
            b"matocsserv_chunks_delays\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_time_register_fname(
            60 as uint32_t,
            0 as uint32_t,
            Some(matocsserv_reason_counters as unsafe extern "C" fn() -> ()),
            b"matocsserv_reason_counters\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_time_register_fname(
            10 as uint32_t,
            0 as uint32_t,
            Some(matocsserv_broadcast_timeout as unsafe extern "C" fn() -> ()),
            b"matocsserv_broadcast_timeout\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_eachloop_register_fname(
            Some(matocsserv_recalculate_server_counters as unsafe extern "C" fn() -> ()),
            b"matocsserv_recalculate_server_counters\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
}
