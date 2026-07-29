#![feature(core_intrinsics)]
#![allow(
    clippy::missing_safety_doc,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
pub enum pcap {}
#[macro_use]
extern crate c2rust_bitfields;
#[allow(unused_imports)]
use ::mfsnetdump;
unsafe extern "C" {
    static mut stderr: *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    static mut optarg: *mut ::core::ffi::c_char;
    unsafe fn getopt(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    unsafe fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strtok(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn pcap_lookupnet(
        _: *const ::core::ffi::c_char,
        _: *mut bpf_u_int32,
        _: *mut bpf_u_int32,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn pcap_open_live(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
    ) -> *mut pcap_t;
    unsafe fn pcap_open_offline(
        _: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
    ) -> *mut pcap_t;
    unsafe fn pcap_close(_: *mut pcap_t);
    unsafe fn pcap_loop(
        _: *mut pcap_t,
        _: ::core::ffi::c_int,
        _: pcap_handler,
        _: *mut u_char,
    ) -> ::core::ffi::c_int;
    unsafe fn pcap_setfilter(_: *mut pcap_t, _: *mut bpf_program) -> ::core::ffi::c_int;
    unsafe fn pcap_geterr(_: *mut pcap_t) -> *mut ::core::ffi::c_char;
    unsafe fn pcap_compile(
        _: *mut pcap_t,
        _: *mut bpf_program,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: bpf_u_int32,
    ) -> ::core::ffi::c_int;
    unsafe fn pcap_freecode(_: *mut bpf_program);
    unsafe fn pcap_datalink(_: *mut pcap_t) -> ::core::ffi::c_int;
    unsafe fn pcap_datalink_val_to_name(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn pcap_findalldevs(
        _: *mut *mut pcap_if_t,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn pcap_freealldevs(_: *mut pcap_if_t);
}
pub type size_t = usize;
pub type __u_char = ::core::ffi::c_uchar;
pub type __u_short = ::core::ffi::c_ushort;
pub type __u_int = ::core::ffi::c_uint;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
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
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type int32_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub type sa_family_t = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}
pub type bpf_u_int32 = u_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bpf_program {
    pub bf_len: u_int,
    pub bf_insns: *mut bpf_insn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bpf_insn {
    pub code: u_short,
    pub jt: u_char,
    pub jf: u_char,
    pub k: bpf_u_int32,
}
pub type pcap_t = pcap;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pcap_if {
    pub next: *mut pcap_if,
    pub name: *mut ::core::ffi::c_char,
    pub description: *mut ::core::ffi::c_char,
    pub addresses: *mut pcap_addr,
    pub flags: bpf_u_int32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pcap_addr {
    pub next: *mut pcap_addr,
    pub addr: *mut sockaddr,
    pub netmask: *mut sockaddr,
    pub broadaddr: *mut sockaddr,
    pub dstaddr: *mut sockaddr,
}
pub type pcap_if_t = pcap_if;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pcap_pkthdr {
    pub ts: timeval,
    pub caplen: bpf_u_int32,
    pub len: bpf_u_int32,
}
pub type pcap_handler =
    Option<unsafe extern "C" fn(*mut u_char, *const pcap_pkthdr, *const u_char) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _userdata {
    pub linktype: uint8_t,
    pub showmfsnops: uint8_t,
    pub showconnections: uint8_t,
    pub minport: uint16_t,
    pub maxport: uint16_t,
    pub maxdatainpacket: uint32_t,
}
pub type userdata = _userdata;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _mfscmd {
    pub command: uint32_t,
    pub commandstr: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _mfscommand {
    pub command: uint32_t,
    pub commandstr: *mut ::core::ffi::c_char,
    pub colorcode: uint8_t,
    pub display: uint8_t,
}
pub type mfscommand = _mfscommand;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _connection {
    pub srcip: uint32_t,
    pub dstip: uint32_t,
    pub srcport: uint16_t,
    pub dstport: uint16_t,
    pub seq: uint32_t,
    pub next: *mut _connection,
}
pub type connection = _connection;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const DLT_NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DLT_EN10MB: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DLT_RAW: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const DLT_LINUX_SLL: ::core::ffi::c_int = 113 as ::core::ffi::c_int;
pub const DLT_PKTAP: ::core::ffi::c_int = 258 as ::core::ffi::c_int;
pub const ANTOAN_NOP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ANTOAN_UNKNOWN_COMMAND: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ANTOAN_BAD_COMMAND_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ANTOAN_FORCE_TIMEOUT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const ANTOAN_GET_VERSION: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const ANTOAN_VERSION: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const ANTOMA_REGISTER: ::core::ffi::c_int = PROTO_BASE + 50 as ::core::ffi::c_int;
pub const MATOAN_METACHANGES_LOG: ::core::ffi::c_int = PROTO_BASE + 51 as ::core::ffi::c_int;
pub const MATOAN_MASTER_ACK: ::core::ffi::c_int = PROTO_BASE + 52 as ::core::ffi::c_int;
pub const MATOAN_STATE: ::core::ffi::c_int = PROTO_BASE + 53 as ::core::ffi::c_int;
pub const ANTOMA_FORCE_METADATA_SAVE: ::core::ffi::c_int = PROTO_BASE + 56 as ::core::ffi::c_int;
pub const MATOMA_METADATA_CHECKSUM: ::core::ffi::c_int = PROTO_BASE + 57 as ::core::ffi::c_int;
pub const MATOMA_FORCE_DESYNC: ::core::ffi::c_int = PROTO_BASE + 58 as ::core::ffi::c_int;
pub const ANTOMA_DOWNLOAD_START: ::core::ffi::c_int = PROTO_BASE + 60 as ::core::ffi::c_int;
pub const MATOAN_DOWNLOAD_INFO: ::core::ffi::c_int = PROTO_BASE + 61 as ::core::ffi::c_int;
pub const ANTOMA_DOWNLOAD_REQUEST: ::core::ffi::c_int = PROTO_BASE + 62 as ::core::ffi::c_int;
pub const MATOAN_DOWNLOAD_DATA: ::core::ffi::c_int = PROTO_BASE + 63 as ::core::ffi::c_int;
pub const ANTOMA_DOWNLOAD_END: ::core::ffi::c_int = PROTO_BASE + 64 as ::core::ffi::c_int;
pub const ANTOMA_STORE_METADATA: ::core::ffi::c_int = PROTO_BASE + 65 as ::core::ffi::c_int;
pub const ANTOMA_SYSLOG: ::core::ffi::c_int = PROTO_BASE + 71 as ::core::ffi::c_int;
pub const ANTOAN_GET_CONFIG: ::core::ffi::c_int = PROTO_BASE + 80 as ::core::ffi::c_int;
pub const ANTOAN_CONFIG_VALUE: ::core::ffi::c_int = PROTO_BASE + 81 as ::core::ffi::c_int;
pub const ANTOAN_GET_CONFIG_FILE: ::core::ffi::c_int = PROTO_BASE + 82 as ::core::ffi::c_int;
pub const ANTOAN_CONFIG_FILE_CONTENT: ::core::ffi::c_int = PROTO_BASE + 83 as ::core::ffi::c_int;
pub const ANTOAN_GET_CONFIG_FILE_MD5: ::core::ffi::c_int = PROTO_BASE + 84 as ::core::ffi::c_int;
pub const ANTOAN_CONFIG_FILE_MD5: ::core::ffi::c_int = PROTO_BASE + 85 as ::core::ffi::c_int;
pub const CSTOMA_CHUNK_DOESNT_EXIST: ::core::ffi::c_int = PROTO_BASE + 96 as ::core::ffi::c_int;
pub const MATOCS_CHUNK_STATUS: ::core::ffi::c_int = PROTO_BASE + 97 as ::core::ffi::c_int;
pub const CSTOMA_CHUNK_STATUS: ::core::ffi::c_int = PROTO_BASE + 98 as ::core::ffi::c_int;
pub const MATOCS_REGISTER_FIRST: ::core::ffi::c_int = PROTO_BASE + 99 as ::core::ffi::c_int;
pub const CSTOMA_REGISTER: ::core::ffi::c_int = PROTO_BASE + 100 as ::core::ffi::c_int;
pub const CSTOMA_SPACE: ::core::ffi::c_int = PROTO_BASE + 101 as ::core::ffi::c_int;
pub const CSTOMA_CHUNK_DAMAGED: ::core::ffi::c_int = PROTO_BASE + 102 as ::core::ffi::c_int;
pub const CSTOMA_CURRENT_LOAD: ::core::ffi::c_int = PROTO_BASE + 103 as ::core::ffi::c_int;
pub const MATOCS_MASTER_ACK: ::core::ffi::c_int = PROTO_BASE + 104 as ::core::ffi::c_int;
pub const CSTOMA_CHUNK_LOST: ::core::ffi::c_int = PROTO_BASE + 105 as ::core::ffi::c_int;
pub const CSTOMA_ERROR_OCCURRED: ::core::ffi::c_int = PROTO_BASE + 106 as ::core::ffi::c_int;
pub const CSTOMA_CHUNK_NEW: ::core::ffi::c_int = PROTO_BASE + 107 as ::core::ffi::c_int;
pub const CSTOMA_LABELS: ::core::ffi::c_int = PROTO_BASE + 109 as ::core::ffi::c_int;
pub const MATOCS_CREATE: ::core::ffi::c_int = PROTO_BASE + 110 as ::core::ffi::c_int;
pub const CSTOMA_CREATE: ::core::ffi::c_int = PROTO_BASE + 111 as ::core::ffi::c_int;
pub const MATOCS_DELETE: ::core::ffi::c_int = PROTO_BASE + 120 as ::core::ffi::c_int;
pub const CSTOMA_DELETE: ::core::ffi::c_int = PROTO_BASE + 121 as ::core::ffi::c_int;
pub const MATOCS_DUPLICATE: ::core::ffi::c_int = PROTO_BASE + 130 as ::core::ffi::c_int;
pub const CSTOMA_DUPLICATE: ::core::ffi::c_int = PROTO_BASE + 131 as ::core::ffi::c_int;
pub const MATOCS_SET_VERSION: ::core::ffi::c_int = PROTO_BASE + 140 as ::core::ffi::c_int;
pub const CSTOMA_SET_VERSION: ::core::ffi::c_int = PROTO_BASE + 141 as ::core::ffi::c_int;
pub const MATOCS_REPLICATE: ::core::ffi::c_int = PROTO_BASE + 150 as ::core::ffi::c_int;
pub const CSTOMA_REPLICATE: ::core::ffi::c_int = PROTO_BASE + 151 as ::core::ffi::c_int;
pub const MATOCS_CHUNKOP: ::core::ffi::c_int = PROTO_BASE + 152 as ::core::ffi::c_int;
pub const CSTOMA_CHUNKOP: ::core::ffi::c_int = PROTO_BASE + 153 as ::core::ffi::c_int;
pub const MATOCS_REPLICATE_SPLIT: ::core::ffi::c_int = PROTO_BASE + 154 as ::core::ffi::c_int;
pub const CSTOMA_REPLICATE_SPLIT: ::core::ffi::c_int = PROTO_BASE + 155 as ::core::ffi::c_int;
pub const MATOCS_REPLICATE_RECOVER: ::core::ffi::c_int = PROTO_BASE + 156 as ::core::ffi::c_int;
pub const CSTOMA_REPLICATE_RECOVER: ::core::ffi::c_int = PROTO_BASE + 157 as ::core::ffi::c_int;
pub const MATOCS_REPLICATE_JOIN: ::core::ffi::c_int = PROTO_BASE + 158 as ::core::ffi::c_int;
pub const CSTOMA_REPLICATE_JOIN: ::core::ffi::c_int = PROTO_BASE + 159 as ::core::ffi::c_int;
pub const MATOCS_TRUNCATE: ::core::ffi::c_int = PROTO_BASE + 160 as ::core::ffi::c_int;
pub const CSTOMA_TRUNCATE: ::core::ffi::c_int = PROTO_BASE + 161 as ::core::ffi::c_int;
pub const MATOCS_DUPTRUNC: ::core::ffi::c_int = PROTO_BASE + 170 as ::core::ffi::c_int;
pub const CSTOMA_DUPTRUNC: ::core::ffi::c_int = PROTO_BASE + 171 as ::core::ffi::c_int;
pub const MATOCS_LOCALSPLIT: ::core::ffi::c_int = PROTO_BASE + 180 as ::core::ffi::c_int;
pub const CSTOMA_LOCALSPLIT: ::core::ffi::c_int = PROTO_BASE + 181 as ::core::ffi::c_int;
pub const CLTOCS_READ: ::core::ffi::c_int = PROTO_BASE + 200 as ::core::ffi::c_int;
pub const CSTOCL_READ_STATUS: ::core::ffi::c_int = PROTO_BASE + 201 as ::core::ffi::c_int;
pub const CSTOCL_READ_DATA: ::core::ffi::c_int = PROTO_BASE + 202 as ::core::ffi::c_int;
pub const CLTOCS_WRITE: ::core::ffi::c_int = PROTO_BASE + 210 as ::core::ffi::c_int;
pub const CSTOCL_WRITE_STATUS: ::core::ffi::c_int = PROTO_BASE + 211 as ::core::ffi::c_int;
pub const CLTOCS_WRITE_DATA: ::core::ffi::c_int = PROTO_BASE + 212 as ::core::ffi::c_int;
pub const CLTOCS_WRITE_FINISH: ::core::ffi::c_int = PROTO_BASE + 213 as ::core::ffi::c_int;
pub const ANTOCS_GET_CHUNK_BLOCKS: ::core::ffi::c_int = PROTO_BASE + 250 as ::core::ffi::c_int;
pub const CSTOAN_CHUNK_BLOCKS: ::core::ffi::c_int = PROTO_BASE + 251 as ::core::ffi::c_int;
pub const ANTOCS_GET_CHUNK_CHECKSUM: ::core::ffi::c_int = PROTO_BASE + 300 as ::core::ffi::c_int;
pub const CSTOAN_CHUNK_CHECKSUM: ::core::ffi::c_int = PROTO_BASE + 301 as ::core::ffi::c_int;
pub const ANTOCS_GET_CHUNK_CHECKSUM_TAB: ::core::ffi::c_int =
    PROTO_BASE + 302 as ::core::ffi::c_int;
pub const CSTOAN_CHUNK_CHECKSUM_TAB: ::core::ffi::c_int = PROTO_BASE + 303 as ::core::ffi::c_int;
pub const ANTOCS_GET_CHUNK_INFO: ::core::ffi::c_int = PROTO_BASE + 304 as ::core::ffi::c_int;
pub const CSTOAN_CHUNK_INFO: ::core::ffi::c_int = PROTO_BASE + 305 as ::core::ffi::c_int;
pub const ANTOCS_CLEAR_ERRORS: ::core::ffi::c_int = PROTO_BASE + 306 as ::core::ffi::c_int;
pub const CSTOAN_CLEAR_ERRORS: ::core::ffi::c_int = PROTO_BASE + 307 as ::core::ffi::c_int;
pub const CLTOMA_SCLASS_CREATE: ::core::ffi::c_int = PROTO_BASE + 350 as ::core::ffi::c_int;
pub const MATOCL_SCLASS_CREATE: ::core::ffi::c_int = PROTO_BASE + 351 as ::core::ffi::c_int;
pub const CLTOMA_SCLASS_CHANGE: ::core::ffi::c_int = PROTO_BASE + 352 as ::core::ffi::c_int;
pub const MATOCL_SCLASS_CHANGE: ::core::ffi::c_int = PROTO_BASE + 353 as ::core::ffi::c_int;
pub const CLTOMA_SCLASS_DELETE: ::core::ffi::c_int = PROTO_BASE + 354 as ::core::ffi::c_int;
pub const MATOCL_SCLASS_DELETE: ::core::ffi::c_int = PROTO_BASE + 355 as ::core::ffi::c_int;
pub const CLTOMA_SCLASS_DUPLICATE: ::core::ffi::c_int = PROTO_BASE + 356 as ::core::ffi::c_int;
pub const MATOCL_SCLASS_DUPLICATE: ::core::ffi::c_int = PROTO_BASE + 357 as ::core::ffi::c_int;
pub const CLTOMA_SCLASS_RENAME: ::core::ffi::c_int = PROTO_BASE + 358 as ::core::ffi::c_int;
pub const MATOCL_SCLASS_RENAME: ::core::ffi::c_int = PROTO_BASE + 359 as ::core::ffi::c_int;
pub const CLTOMA_SCLASS_LIST: ::core::ffi::c_int = PROTO_BASE + 360 as ::core::ffi::c_int;
pub const MATOCL_SCLASS_LIST: ::core::ffi::c_int = PROTO_BASE + 361 as ::core::ffi::c_int;
pub const CLTOMA_PATTERN_ADD: ::core::ffi::c_int = PROTO_BASE + 370 as ::core::ffi::c_int;
pub const MATOCL_PATTERN_ADD: ::core::ffi::c_int = PROTO_BASE + 371 as ::core::ffi::c_int;
pub const CLTOMA_PATTERN_DELETE: ::core::ffi::c_int = PROTO_BASE + 372 as ::core::ffi::c_int;
pub const MATOCL_PATTERN_DELETE: ::core::ffi::c_int = PROTO_BASE + 373 as ::core::ffi::c_int;
pub const CLTOMA_PATTERN_LIST: ::core::ffi::c_int = PROTO_BASE + 374 as ::core::ffi::c_int;
pub const MATOCL_PATTERN_LIST: ::core::ffi::c_int = PROTO_BASE + 375 as ::core::ffi::c_int;
pub const CLTOMA_TRASH_LIST: ::core::ffi::c_int = PROTO_BASE + 380 as ::core::ffi::c_int;
pub const MATOCL_TRASH_LIST: ::core::ffi::c_int = PROTO_BASE + 381 as ::core::ffi::c_int;
pub const CLTOMA_TRASH_RECOVER: ::core::ffi::c_int = PROTO_BASE + 382 as ::core::ffi::c_int;
pub const MATOCL_TRASH_RECOVER: ::core::ffi::c_int = PROTO_BASE + 383 as ::core::ffi::c_int;
pub const CLTOMA_TRASH_REMOVE: ::core::ffi::c_int = PROTO_BASE + 384 as ::core::ffi::c_int;
pub const MATOCL_TRASH_REMOVE: ::core::ffi::c_int = PROTO_BASE + 385 as ::core::ffi::c_int;
pub const CLTOMA_SUSTAINED_LIST: ::core::ffi::c_int = PROTO_BASE + 388 as ::core::ffi::c_int;
pub const MATOCL_SUSTAINED_LIST: ::core::ffi::c_int = PROTO_BASE + 389 as ::core::ffi::c_int;
pub const CLTOMA_PATH_LOOKUP: ::core::ffi::c_int = PROTO_BASE + 390 as ::core::ffi::c_int;
pub const MATOCL_PATH_LOOKUP: ::core::ffi::c_int = PROTO_BASE + 391 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_REGISTER: ::core::ffi::c_int = PROTO_BASE + 400 as ::core::ffi::c_int;
pub const MATOCL_FUSE_REGISTER: ::core::ffi::c_int = PROTO_BASE + 401 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_STATFS: ::core::ffi::c_int = PROTO_BASE + 402 as ::core::ffi::c_int;
pub const MATOCL_FUSE_STATFS: ::core::ffi::c_int = PROTO_BASE + 403 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_ACCESS: ::core::ffi::c_int = PROTO_BASE + 404 as ::core::ffi::c_int;
pub const MATOCL_FUSE_ACCESS: ::core::ffi::c_int = PROTO_BASE + 405 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_LOOKUP: ::core::ffi::c_int = PROTO_BASE + 406 as ::core::ffi::c_int;
pub const MATOCL_FUSE_LOOKUP: ::core::ffi::c_int = PROTO_BASE + 407 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETATTR: ::core::ffi::c_int = PROTO_BASE + 408 as ::core::ffi::c_int;
pub const MATOCL_FUSE_GETATTR: ::core::ffi::c_int = PROTO_BASE + 409 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SETATTR: ::core::ffi::c_int = PROTO_BASE + 410 as ::core::ffi::c_int;
pub const MATOCL_FUSE_SETATTR: ::core::ffi::c_int = PROTO_BASE + 411 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_READLINK: ::core::ffi::c_int = PROTO_BASE + 412 as ::core::ffi::c_int;
pub const MATOCL_FUSE_READLINK: ::core::ffi::c_int = PROTO_BASE + 413 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SYMLINK: ::core::ffi::c_int = PROTO_BASE + 414 as ::core::ffi::c_int;
pub const MATOCL_FUSE_SYMLINK: ::core::ffi::c_int = PROTO_BASE + 415 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_MKNOD: ::core::ffi::c_int = PROTO_BASE + 416 as ::core::ffi::c_int;
pub const MATOCL_FUSE_MKNOD: ::core::ffi::c_int = PROTO_BASE + 417 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_MKDIR: ::core::ffi::c_int = PROTO_BASE + 418 as ::core::ffi::c_int;
pub const MATOCL_FUSE_MKDIR: ::core::ffi::c_int = PROTO_BASE + 419 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_UNLINK: ::core::ffi::c_int = PROTO_BASE + 420 as ::core::ffi::c_int;
pub const MATOCL_FUSE_UNLINK: ::core::ffi::c_int = PROTO_BASE + 421 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_RMDIR: ::core::ffi::c_int = PROTO_BASE + 422 as ::core::ffi::c_int;
pub const MATOCL_FUSE_RMDIR: ::core::ffi::c_int = PROTO_BASE + 423 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_RENAME: ::core::ffi::c_int = PROTO_BASE + 424 as ::core::ffi::c_int;
pub const MATOCL_FUSE_RENAME: ::core::ffi::c_int = PROTO_BASE + 425 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_LINK: ::core::ffi::c_int = PROTO_BASE + 426 as ::core::ffi::c_int;
pub const MATOCL_FUSE_LINK: ::core::ffi::c_int = PROTO_BASE + 427 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_READDIR: ::core::ffi::c_int = PROTO_BASE + 428 as ::core::ffi::c_int;
pub const MATOCL_FUSE_READDIR: ::core::ffi::c_int = PROTO_BASE + 429 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_OPEN: ::core::ffi::c_int = PROTO_BASE + 430 as ::core::ffi::c_int;
pub const MATOCL_FUSE_OPEN: ::core::ffi::c_int = PROTO_BASE + 431 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_READ_CHUNK: ::core::ffi::c_int = PROTO_BASE + 432 as ::core::ffi::c_int;
pub const MATOCL_FUSE_READ_CHUNK: ::core::ffi::c_int = PROTO_BASE + 433 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_WRITE_CHUNK: ::core::ffi::c_int = PROTO_BASE + 434 as ::core::ffi::c_int;
pub const MATOCL_FUSE_WRITE_CHUNK: ::core::ffi::c_int = PROTO_BASE + 435 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_WRITE_CHUNK_END: ::core::ffi::c_int = PROTO_BASE + 436 as ::core::ffi::c_int;
pub const MATOCL_FUSE_WRITE_CHUNK_END: ::core::ffi::c_int = PROTO_BASE + 437 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_APPEND_SLICE: ::core::ffi::c_int = PROTO_BASE + 438 as ::core::ffi::c_int;
pub const MATOCL_FUSE_APPEND_SLICE: ::core::ffi::c_int = PROTO_BASE + 439 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_CHECK: ::core::ffi::c_int = PROTO_BASE + 440 as ::core::ffi::c_int;
pub const MATOCL_FUSE_CHECK: ::core::ffi::c_int = PROTO_BASE + 441 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETTRASHRETENTION: ::core::ffi::c_int =
    PROTO_BASE + 442 as ::core::ffi::c_int;
pub const MATOCL_FUSE_GETTRASHRETENTION: ::core::ffi::c_int =
    PROTO_BASE + 443 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SETTRASHRETENTION: ::core::ffi::c_int =
    PROTO_BASE + 444 as ::core::ffi::c_int;
pub const MATOCL_FUSE_SETTRASHRETENTION: ::core::ffi::c_int =
    PROTO_BASE + 445 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETSCLASS: ::core::ffi::c_int = PROTO_BASE + 446 as ::core::ffi::c_int;
pub const MATOCL_FUSE_GETSCLASS: ::core::ffi::c_int = PROTO_BASE + 447 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SETSCLASS: ::core::ffi::c_int = PROTO_BASE + 448 as ::core::ffi::c_int;
pub const MATOCL_FUSE_SETSCLASS: ::core::ffi::c_int = PROTO_BASE + 449 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETTRASH: ::core::ffi::c_int = PROTO_BASE + 450 as ::core::ffi::c_int;
pub const MATOCL_FUSE_GETTRASH: ::core::ffi::c_int = PROTO_BASE + 451 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETDETACHEDATTR: ::core::ffi::c_int = PROTO_BASE + 452 as ::core::ffi::c_int;
pub const MATOCL_FUSE_GETDETACHEDATTR: ::core::ffi::c_int = PROTO_BASE + 453 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETTRASHPATH: ::core::ffi::c_int = PROTO_BASE + 454 as ::core::ffi::c_int;
pub const MATOCL_FUSE_GETTRASHPATH: ::core::ffi::c_int = PROTO_BASE + 455 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SETTRASHPATH: ::core::ffi::c_int = PROTO_BASE + 456 as ::core::ffi::c_int;
pub const MATOCL_FUSE_SETTRASHPATH: ::core::ffi::c_int = PROTO_BASE + 457 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_UNDEL: ::core::ffi::c_int = PROTO_BASE + 458 as ::core::ffi::c_int;
pub const MATOCL_FUSE_UNDEL: ::core::ffi::c_int = PROTO_BASE + 459 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_PURGE: ::core::ffi::c_int = PROTO_BASE + 460 as ::core::ffi::c_int;
pub const MATOCL_FUSE_PURGE: ::core::ffi::c_int = PROTO_BASE + 461 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETDIRSTATS: ::core::ffi::c_int = PROTO_BASE + 462 as ::core::ffi::c_int;
pub const MATOCL_FUSE_GETDIRSTATS: ::core::ffi::c_int = PROTO_BASE + 463 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_TRUNCATE: ::core::ffi::c_int = PROTO_BASE + 464 as ::core::ffi::c_int;
pub const MATOCL_FUSE_TRUNCATE: ::core::ffi::c_int = PROTO_BASE + 465 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_REPAIR: ::core::ffi::c_int = PROTO_BASE + 466 as ::core::ffi::c_int;
pub const MATOCL_FUSE_REPAIR: ::core::ffi::c_int = PROTO_BASE + 467 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SNAPSHOT: ::core::ffi::c_int = PROTO_BASE + 468 as ::core::ffi::c_int;
pub const MATOCL_FUSE_SNAPSHOT: ::core::ffi::c_int = PROTO_BASE + 469 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETSUSTAINED: ::core::ffi::c_int = PROTO_BASE + 470 as ::core::ffi::c_int;
pub const MATOCL_FUSE_GETSUSTAINED: ::core::ffi::c_int = PROTO_BASE + 471 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETEATTR: ::core::ffi::c_int = PROTO_BASE + 472 as ::core::ffi::c_int;
pub const MATOCL_FUSE_GETEATTR: ::core::ffi::c_int = PROTO_BASE + 473 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SETEATTR: ::core::ffi::c_int = PROTO_BASE + 474 as ::core::ffi::c_int;
pub const MATOCL_FUSE_SETEATTR: ::core::ffi::c_int = PROTO_BASE + 475 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_QUOTACONTROL: ::core::ffi::c_int = PROTO_BASE + 476 as ::core::ffi::c_int;
pub const MATOCL_FUSE_QUOTACONTROL: ::core::ffi::c_int = PROTO_BASE + 477 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETXATTR: ::core::ffi::c_int = PROTO_BASE + 478 as ::core::ffi::c_int;
pub const MATOCL_FUSE_GETXATTR: ::core::ffi::c_int = PROTO_BASE + 479 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SETXATTR: ::core::ffi::c_int = PROTO_BASE + 480 as ::core::ffi::c_int;
pub const MATOCL_FUSE_SETXATTR: ::core::ffi::c_int = PROTO_BASE + 481 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_CREATE: ::core::ffi::c_int = PROTO_BASE + 482 as ::core::ffi::c_int;
pub const MATOCL_FUSE_CREATE: ::core::ffi::c_int = PROTO_BASE + 483 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_PARENTS: ::core::ffi::c_int = PROTO_BASE + 484 as ::core::ffi::c_int;
pub const MATOCL_FUSE_PARENTS: ::core::ffi::c_int = PROTO_BASE + 485 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_PATHS: ::core::ffi::c_int = PROTO_BASE + 486 as ::core::ffi::c_int;
pub const MATOCL_FUSE_PATHS: ::core::ffi::c_int = PROTO_BASE + 487 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_GETFACL: ::core::ffi::c_int = PROTO_BASE + 488 as ::core::ffi::c_int;
pub const MATOCL_FUSE_GETFACL: ::core::ffi::c_int = PROTO_BASE + 489 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SETFACL: ::core::ffi::c_int = PROTO_BASE + 490 as ::core::ffi::c_int;
pub const MATOCL_FUSE_SETFACL: ::core::ffi::c_int = PROTO_BASE + 491 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_FLOCK: ::core::ffi::c_int = PROTO_BASE + 492 as ::core::ffi::c_int;
pub const MATOCL_FUSE_FLOCK: ::core::ffi::c_int = PROTO_BASE + 493 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_POSIX_LOCK: ::core::ffi::c_int = PROTO_BASE + 494 as ::core::ffi::c_int;
pub const MATOCL_FUSE_POSIX_LOCK: ::core::ffi::c_int = PROTO_BASE + 495 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_ARCHCTL: ::core::ffi::c_int = PROTO_BASE + 496 as ::core::ffi::c_int;
pub const MATOCL_FUSE_ARCHCTL: ::core::ffi::c_int = PROTO_BASE + 497 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_FSYNC: ::core::ffi::c_int = PROTO_BASE + 498 as ::core::ffi::c_int;
pub const MATOCL_FUSE_FSYNC: ::core::ffi::c_int = PROTO_BASE + 499 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SUSTAINED_INODES_DEPRECATED: ::core::ffi::c_int =
    PROTO_BASE + 499 as ::core::ffi::c_int;
pub const CLTOMA_CSERV_LIST: ::core::ffi::c_int = PROTO_BASE + 500 as ::core::ffi::c_int;
pub const MATOCL_CSERV_LIST: ::core::ffi::c_int = PROTO_BASE + 501 as ::core::ffi::c_int;
pub const CLTOAN_MONOTONIC_DATA: ::core::ffi::c_int = PROTO_BASE + 502 as ::core::ffi::c_int;
pub const ANTOCL_MONOTONIC_DATA: ::core::ffi::c_int = PROTO_BASE + 503 as ::core::ffi::c_int;
pub const CLTOAN_CHART: ::core::ffi::c_int = PROTO_BASE + 504 as ::core::ffi::c_int;
pub const ANTOCL_CHART: ::core::ffi::c_int = PROTO_BASE + 505 as ::core::ffi::c_int;
pub const CLTOAN_CHART_DATA: ::core::ffi::c_int = PROTO_BASE + 506 as ::core::ffi::c_int;
pub const ANTOCL_CHART_DATA: ::core::ffi::c_int = PROTO_BASE + 507 as ::core::ffi::c_int;
pub const CLTOMA_SESSION_LIST: ::core::ffi::c_int = PROTO_BASE + 508 as ::core::ffi::c_int;
pub const MATOCL_SESSION_LIST: ::core::ffi::c_int = PROTO_BASE + 509 as ::core::ffi::c_int;
pub const CLTOMA_INFO: ::core::ffi::c_int = PROTO_BASE + 510 as ::core::ffi::c_int;
pub const MATOCL_INFO: ::core::ffi::c_int = PROTO_BASE + 511 as ::core::ffi::c_int;
pub const CLTOMA_FSTEST_INFO: ::core::ffi::c_int = PROTO_BASE + 512 as ::core::ffi::c_int;
pub const MATOCL_FSTEST_INFO: ::core::ffi::c_int = PROTO_BASE + 513 as ::core::ffi::c_int;
pub const CLTOMA_CHUNKSTEST_INFO: ::core::ffi::c_int = PROTO_BASE + 514 as ::core::ffi::c_int;
pub const MATOCL_CHUNKSTEST_INFO: ::core::ffi::c_int = PROTO_BASE + 515 as ::core::ffi::c_int;
pub const CLTOMA_CHUNKS_MATRIX: ::core::ffi::c_int = PROTO_BASE + 516 as ::core::ffi::c_int;
pub const MATOCL_CHUNKS_MATRIX: ::core::ffi::c_int = PROTO_BASE + 517 as ::core::ffi::c_int;
pub const CLTOMA_QUOTA_INFO: ::core::ffi::c_int = PROTO_BASE + 518 as ::core::ffi::c_int;
pub const MATOCL_QUOTA_INFO: ::core::ffi::c_int = PROTO_BASE + 519 as ::core::ffi::c_int;
pub const CLTOMA_EXPORTS_INFO: ::core::ffi::c_int = PROTO_BASE + 520 as ::core::ffi::c_int;
pub const MATOCL_EXPORTS_INFO: ::core::ffi::c_int = PROTO_BASE + 521 as ::core::ffi::c_int;
pub const CLTOMA_MLOG_LIST: ::core::ffi::c_int = PROTO_BASE + 522 as ::core::ffi::c_int;
pub const MATOCL_MLOG_LIST: ::core::ffi::c_int = PROTO_BASE + 523 as ::core::ffi::c_int;
pub const CLTOMA_CSSERV_COMMAND: ::core::ffi::c_int = PROTO_BASE + 524 as ::core::ffi::c_int;
pub const MATOCL_CSSERV_COMMAND: ::core::ffi::c_int = PROTO_BASE + 525 as ::core::ffi::c_int;
pub const CLTOMA_SESSION_COMMAND: ::core::ffi::c_int = PROTO_BASE + 526 as ::core::ffi::c_int;
pub const MATOCL_SESSION_COMMAND: ::core::ffi::c_int = PROTO_BASE + 527 as ::core::ffi::c_int;
pub const CLTOMA_MEMORY_INFO: ::core::ffi::c_int = PROTO_BASE + 528 as ::core::ffi::c_int;
pub const MATOCL_MEMORY_INFO: ::core::ffi::c_int = PROTO_BASE + 529 as ::core::ffi::c_int;
pub const CLTOAN_MODULE_INFO: ::core::ffi::c_int = PROTO_BASE + 530 as ::core::ffi::c_int;
pub const ANTOCL_MODULE_INFO: ::core::ffi::c_int = PROTO_BASE + 531 as ::core::ffi::c_int;
pub const CLTOMA_LIST_OPEN_FILES: ::core::ffi::c_int = PROTO_BASE + 532 as ::core::ffi::c_int;
pub const MATOCL_LIST_OPEN_FILES: ::core::ffi::c_int = PROTO_BASE + 533 as ::core::ffi::c_int;
pub const CLTOMA_LIST_ACQUIRED_LOCKS: ::core::ffi::c_int = PROTO_BASE + 534 as ::core::ffi::c_int;
pub const MATOCL_LIST_ACQUIRED_LOCKS: ::core::ffi::c_int = PROTO_BASE + 535 as ::core::ffi::c_int;
pub const CLTOMA_MASS_RESOLVE_PATHS: ::core::ffi::c_int = PROTO_BASE + 536 as ::core::ffi::c_int;
pub const MATOCL_MASS_RESOLVE_PATHS: ::core::ffi::c_int = PROTO_BASE + 537 as ::core::ffi::c_int;
pub const CLTOMA_SCLASS_INFO: ::core::ffi::c_int = PROTO_BASE + 542 as ::core::ffi::c_int;
pub const MATOCL_SCLASS_INFO: ::core::ffi::c_int = PROTO_BASE + 543 as ::core::ffi::c_int;
pub const CLTOMA_MISSING_CHUNKS: ::core::ffi::c_int = PROTO_BASE + 544 as ::core::ffi::c_int;
pub const MATOCL_MISSING_CHUNKS: ::core::ffi::c_int = PROTO_BASE + 545 as ::core::ffi::c_int;
pub const CLTOMA_NODE_INFO: ::core::ffi::c_int = PROTO_BASE + 546 as ::core::ffi::c_int;
pub const MATOCL_NODE_INFO: ::core::ffi::c_int = PROTO_BASE + 547 as ::core::ffi::c_int;
pub const CLTOMA_PATTERN_INFO: ::core::ffi::c_int = PROTO_BASE + 548 as ::core::ffi::c_int;
pub const MATOCL_PATTERN_INFO: ::core::ffi::c_int = PROTO_BASE + 549 as ::core::ffi::c_int;
pub const CLTOMA_INSTANCE_NAME: ::core::ffi::c_int = PROTO_BASE + 550 as ::core::ffi::c_int;
pub const MATOCL_INSTANCE_NAME: ::core::ffi::c_int = PROTO_BASE + 551 as ::core::ffi::c_int;
pub const CLTOMA_FULL_DIRECTORY_DATA: ::core::ffi::c_int = PROTO_BASE + 552 as ::core::ffi::c_int;
pub const MATOCL_FULL_DIRECTORY_DATA: ::core::ffi::c_int = PROTO_BASE + 553 as ::core::ffi::c_int;
pub const CLTOMA_SET_ALL_NODE_ATTRIBUTES: ::core::ffi::c_int =
    PROTO_BASE + 554 as ::core::ffi::c_int;
pub const MATOCL_SET_ALL_NODE_ATTRIBUTES: ::core::ffi::c_int =
    PROTO_BASE + 555 as ::core::ffi::c_int;
pub const CLTOCS_HDD_LIST: ::core::ffi::c_int = PROTO_BASE + 600 as ::core::ffi::c_int;
pub const CSTOCL_HDD_LIST: ::core::ffi::c_int = PROTO_BASE + 601 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_SUSTAINED_INODES: ::core::ffi::c_int = PROTO_BASE + 700 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_AMTIME_INODES: ::core::ffi::c_int = PROTO_BASE + 701 as ::core::ffi::c_int;
pub const MATOCL_FUSE_CHUNK_HAS_CHANGED: ::core::ffi::c_int =
    PROTO_BASE + 702 as ::core::ffi::c_int;
pub const MATOCL_FUSE_FLENG_HAS_CHANGED: ::core::ffi::c_int =
    PROTO_BASE + 703 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_TIME_SYNC: ::core::ffi::c_int = PROTO_BASE + 704 as ::core::ffi::c_int;
pub const MATOCL_FUSE_TIME_SYNC: ::core::ffi::c_int = PROTO_BASE + 705 as ::core::ffi::c_int;
pub const MATOCL_FUSE_INVALIDATE_CHUNK_CACHE: ::core::ffi::c_int =
    PROTO_BASE + 706 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_OPDATA: ::core::ffi::c_int = PROTO_BASE + 710 as ::core::ffi::c_int;
pub const CLTOMA_FUSE_WFLAGS: ::core::ffi::c_int = PROTO_BASE + 711 as ::core::ffi::c_int;
pub const PCAP_IF_UP: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const PCAP_IF_RUNNING: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const PCAP_NETMASK_UNKNOWN: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
pub const PROTO_BASE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub static mut id: [::core::ffi::c_char; 72] = unsafe {
    ::core::mem::transmute::<[u8; 72], [::core::ffi::c_char; 72]>(
        *b"@(#) version: 4.59.2-1, build: 2106, written by Jakub Kruszona-Zawadzki\0",
    )
};
#[unsafe(no_mangle)]
pub static mut cmdtab: [_mfscmd; 273] = [
    _mfscmd {
        command: ANTOAN_NOP as uint32_t,
        commandstr: b"ANTOAN_NOP\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOAN_UNKNOWN_COMMAND as uint32_t,
        commandstr: b"ANTOAN_UNKNOWN_COMMAND\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOAN_BAD_COMMAND_SIZE as uint32_t,
        commandstr: b"ANTOAN_BAD_COMMAND_SIZE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOAN_FORCE_TIMEOUT as uint32_t,
        commandstr: b"ANTOAN_FORCE_TIMEOUT\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOAN_GET_VERSION as uint32_t,
        commandstr: b"ANTOAN_GET_VERSION\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOAN_VERSION as uint32_t,
        commandstr: b"ANTOAN_VERSION\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOMA_REGISTER as uint32_t,
        commandstr: b"ANTOMA_REGISTER\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOAN_METACHANGES_LOG as uint32_t,
        commandstr: b"MATOAN_METACHANGES_LOG\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOAN_MASTER_ACK as uint32_t,
        commandstr: b"MATOAN_MASTER_ACK\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOAN_STATE as uint32_t,
        commandstr: b"MATOAN_STATE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOMA_FORCE_METADATA_SAVE as uint32_t,
        commandstr: b"ANTOMA_FORCE_METADATA_SAVE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOMA_METADATA_CHECKSUM as uint32_t,
        commandstr: b"MATOMA_METADATA_CHECKSUM\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOMA_FORCE_DESYNC as uint32_t,
        commandstr: b"MATOMA_FORCE_DESYNC\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOMA_DOWNLOAD_START as uint32_t,
        commandstr: b"ANTOMA_DOWNLOAD_START\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOAN_DOWNLOAD_INFO as uint32_t,
        commandstr: b"MATOAN_DOWNLOAD_INFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOMA_DOWNLOAD_REQUEST as uint32_t,
        commandstr: b"ANTOMA_DOWNLOAD_REQUEST\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOAN_DOWNLOAD_DATA as uint32_t,
        commandstr: b"MATOAN_DOWNLOAD_DATA\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOMA_DOWNLOAD_END as uint32_t,
        commandstr: b"ANTOMA_DOWNLOAD_END\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOMA_STORE_METADATA as uint32_t,
        commandstr: b"ANTOMA_STORE_METADATA\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOMA_SYSLOG as uint32_t,
        commandstr: b"ANTOMA_SYSLOG\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOAN_GET_CONFIG as uint32_t,
        commandstr: b"ANTOAN_GET_CONFIG\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOAN_CONFIG_VALUE as uint32_t,
        commandstr: b"ANTOAN_CONFIG_VALUE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOAN_GET_CONFIG_FILE as uint32_t,
        commandstr: b"ANTOAN_GET_CONFIG_FILE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOAN_CONFIG_FILE_CONTENT as uint32_t,
        commandstr: b"ANTOAN_CONFIG_FILE_CONTENT\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOAN_GET_CONFIG_FILE_MD5 as uint32_t,
        commandstr: b"ANTOAN_GET_CONFIG_FILE_MD5\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOAN_CONFIG_FILE_MD5 as uint32_t,
        commandstr: b"ANTOAN_CONFIG_FILE_MD5\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOMA_CHUNK_DOESNT_EXIST as uint32_t,
        commandstr: b"CSTOMA_CHUNK_DOESNT_EXIST\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCS_CHUNK_STATUS as uint32_t,
        commandstr: b"MATOCS_CHUNK_STATUS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOMA_CHUNK_STATUS as uint32_t,
        commandstr: b"CSTOMA_CHUNK_STATUS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCS_REGISTER_FIRST as uint32_t,
        commandstr: b"MATOCS_REGISTER_FIRST\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOMA_REGISTER as uint32_t,
        commandstr: b"CSTOMA_REGISTER\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOMA_SPACE as uint32_t,
        commandstr: b"CSTOMA_SPACE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOMA_CHUNK_DAMAGED as uint32_t,
        commandstr: b"CSTOMA_CHUNK_DAMAGED\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOMA_CURRENT_LOAD as uint32_t,
        commandstr: b"CSTOMA_CURRENT_LOAD\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCS_MASTER_ACK as uint32_t,
        commandstr: b"MATOCS_MASTER_ACK\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOMA_CHUNK_LOST as uint32_t,
        commandstr: b"CSTOMA_CHUNK_LOST\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOMA_ERROR_OCCURRED as uint32_t,
        commandstr: b"CSTOMA_ERROR_OCCURRED\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOMA_CHUNK_NEW as uint32_t,
        commandstr: b"CSTOMA_CHUNK_NEW\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOMA_LABELS as uint32_t,
        commandstr: b"CSTOMA_LABELS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCS_CREATE as uint32_t,
        commandstr: b"MATOCS_CREATE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOMA_CREATE as uint32_t,
        commandstr: b"CSTOMA_CREATE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCS_DELETE as uint32_t,
        commandstr: b"MATOCS_DELETE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOMA_DELETE as uint32_t,
        commandstr: b"CSTOMA_DELETE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCS_DUPLICATE as uint32_t,
        commandstr: b"MATOCS_DUPLICATE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOMA_DUPLICATE as uint32_t,
        commandstr: b"CSTOMA_DUPLICATE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCS_SET_VERSION as uint32_t,
        commandstr: b"MATOCS_SET_VERSION\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOMA_SET_VERSION as uint32_t,
        commandstr: b"CSTOMA_SET_VERSION\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCS_REPLICATE as uint32_t,
        commandstr: b"MATOCS_REPLICATE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOMA_REPLICATE as uint32_t,
        commandstr: b"CSTOMA_REPLICATE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCS_CHUNKOP as uint32_t,
        commandstr: b"MATOCS_CHUNKOP\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOMA_CHUNKOP as uint32_t,
        commandstr: b"CSTOMA_CHUNKOP\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCS_REPLICATE_SPLIT as uint32_t,
        commandstr: b"MATOCS_REPLICATE_SPLIT\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOMA_REPLICATE_SPLIT as uint32_t,
        commandstr: b"CSTOMA_REPLICATE_SPLIT\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCS_REPLICATE_RECOVER as uint32_t,
        commandstr: b"MATOCS_REPLICATE_RECOVER\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOMA_REPLICATE_RECOVER as uint32_t,
        commandstr: b"CSTOMA_REPLICATE_RECOVER\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCS_REPLICATE_JOIN as uint32_t,
        commandstr: b"MATOCS_REPLICATE_JOIN\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOMA_REPLICATE_JOIN as uint32_t,
        commandstr: b"CSTOMA_REPLICATE_JOIN\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCS_TRUNCATE as uint32_t,
        commandstr: b"MATOCS_TRUNCATE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOMA_TRUNCATE as uint32_t,
        commandstr: b"CSTOMA_TRUNCATE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCS_DUPTRUNC as uint32_t,
        commandstr: b"MATOCS_DUPTRUNC\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOMA_DUPTRUNC as uint32_t,
        commandstr: b"CSTOMA_DUPTRUNC\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCS_LOCALSPLIT as uint32_t,
        commandstr: b"MATOCS_LOCALSPLIT\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOMA_LOCALSPLIT as uint32_t,
        commandstr: b"CSTOMA_LOCALSPLIT\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOCS_READ as uint32_t,
        commandstr: b"CLTOCS_READ\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOCL_READ_STATUS as uint32_t,
        commandstr: b"CSTOCL_READ_STATUS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOCL_READ_DATA as uint32_t,
        commandstr: b"CSTOCL_READ_DATA\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOCS_WRITE as uint32_t,
        commandstr: b"CLTOCS_WRITE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOCL_WRITE_STATUS as uint32_t,
        commandstr: b"CSTOCL_WRITE_STATUS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOCS_WRITE_DATA as uint32_t,
        commandstr: b"CLTOCS_WRITE_DATA\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOCS_WRITE_FINISH as uint32_t,
        commandstr: b"CLTOCS_WRITE_FINISH\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOCS_GET_CHUNK_BLOCKS as uint32_t,
        commandstr: b"ANTOCS_GET_CHUNK_BLOCKS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOAN_CHUNK_BLOCKS as uint32_t,
        commandstr: b"CSTOAN_CHUNK_BLOCKS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOCS_GET_CHUNK_CHECKSUM as uint32_t,
        commandstr: b"ANTOCS_GET_CHUNK_CHECKSUM\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOAN_CHUNK_CHECKSUM as uint32_t,
        commandstr: b"CSTOAN_CHUNK_CHECKSUM\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOCS_GET_CHUNK_CHECKSUM_TAB as uint32_t,
        commandstr: b"ANTOCS_GET_CHUNK_CHECKSUM_TAB\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOAN_CHUNK_CHECKSUM_TAB as uint32_t,
        commandstr: b"CSTOAN_CHUNK_CHECKSUM_TAB\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOCS_GET_CHUNK_INFO as uint32_t,
        commandstr: b"ANTOCS_GET_CHUNK_INFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOAN_CHUNK_INFO as uint32_t,
        commandstr: b"CSTOAN_CHUNK_INFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOCS_CLEAR_ERRORS as uint32_t,
        commandstr: b"ANTOCS_CLEAR_ERRORS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOAN_CLEAR_ERRORS as uint32_t,
        commandstr: b"CSTOAN_CLEAR_ERRORS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_SCLASS_CREATE as uint32_t,
        commandstr: b"CLTOMA_SCLASS_CREATE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_SCLASS_CREATE as uint32_t,
        commandstr: b"MATOCL_SCLASS_CREATE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_SCLASS_CHANGE as uint32_t,
        commandstr: b"CLTOMA_SCLASS_CHANGE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_SCLASS_CHANGE as uint32_t,
        commandstr: b"MATOCL_SCLASS_CHANGE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_SCLASS_DELETE as uint32_t,
        commandstr: b"CLTOMA_SCLASS_DELETE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_SCLASS_DELETE as uint32_t,
        commandstr: b"MATOCL_SCLASS_DELETE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_SCLASS_DUPLICATE as uint32_t,
        commandstr: b"CLTOMA_SCLASS_DUPLICATE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_SCLASS_DUPLICATE as uint32_t,
        commandstr: b"MATOCL_SCLASS_DUPLICATE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_SCLASS_RENAME as uint32_t,
        commandstr: b"CLTOMA_SCLASS_RENAME\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_SCLASS_RENAME as uint32_t,
        commandstr: b"MATOCL_SCLASS_RENAME\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_SCLASS_LIST as uint32_t,
        commandstr: b"CLTOMA_SCLASS_LIST\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_SCLASS_LIST as uint32_t,
        commandstr: b"MATOCL_SCLASS_LIST\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_PATTERN_ADD as uint32_t,
        commandstr: b"CLTOMA_PATTERN_ADD\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_PATTERN_ADD as uint32_t,
        commandstr: b"MATOCL_PATTERN_ADD\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_PATTERN_DELETE as uint32_t,
        commandstr: b"CLTOMA_PATTERN_DELETE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_PATTERN_DELETE as uint32_t,
        commandstr: b"MATOCL_PATTERN_DELETE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_PATTERN_LIST as uint32_t,
        commandstr: b"CLTOMA_PATTERN_LIST\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_PATTERN_LIST as uint32_t,
        commandstr: b"MATOCL_PATTERN_LIST\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_TRASH_LIST as uint32_t,
        commandstr: b"CLTOMA_TRASH_LIST\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_TRASH_LIST as uint32_t,
        commandstr: b"MATOCL_TRASH_LIST\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_TRASH_RECOVER as uint32_t,
        commandstr: b"CLTOMA_TRASH_RECOVER\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_TRASH_RECOVER as uint32_t,
        commandstr: b"MATOCL_TRASH_RECOVER\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_TRASH_REMOVE as uint32_t,
        commandstr: b"CLTOMA_TRASH_REMOVE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_TRASH_REMOVE as uint32_t,
        commandstr: b"MATOCL_TRASH_REMOVE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_SUSTAINED_LIST as uint32_t,
        commandstr: b"CLTOMA_SUSTAINED_LIST\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_SUSTAINED_LIST as uint32_t,
        commandstr: b"MATOCL_SUSTAINED_LIST\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_PATH_LOOKUP as uint32_t,
        commandstr: b"CLTOMA_PATH_LOOKUP\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_PATH_LOOKUP as uint32_t,
        commandstr: b"MATOCL_PATH_LOOKUP\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_REGISTER as uint32_t,
        commandstr: b"CLTOMA_FUSE_REGISTER\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_REGISTER as uint32_t,
        commandstr: b"MATOCL_FUSE_REGISTER\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_STATFS as uint32_t,
        commandstr: b"CLTOMA_FUSE_STATFS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_STATFS as uint32_t,
        commandstr: b"MATOCL_FUSE_STATFS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_ACCESS as uint32_t,
        commandstr: b"CLTOMA_FUSE_ACCESS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_ACCESS as uint32_t,
        commandstr: b"MATOCL_FUSE_ACCESS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_LOOKUP as uint32_t,
        commandstr: b"CLTOMA_FUSE_LOOKUP\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_LOOKUP as uint32_t,
        commandstr: b"MATOCL_FUSE_LOOKUP\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_GETATTR as uint32_t,
        commandstr: b"CLTOMA_FUSE_GETATTR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_GETATTR as uint32_t,
        commandstr: b"MATOCL_FUSE_GETATTR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_SETATTR as uint32_t,
        commandstr: b"CLTOMA_FUSE_SETATTR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_SETATTR as uint32_t,
        commandstr: b"MATOCL_FUSE_SETATTR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_READLINK as uint32_t,
        commandstr: b"CLTOMA_FUSE_READLINK\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_READLINK as uint32_t,
        commandstr: b"MATOCL_FUSE_READLINK\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_SYMLINK as uint32_t,
        commandstr: b"CLTOMA_FUSE_SYMLINK\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_SYMLINK as uint32_t,
        commandstr: b"MATOCL_FUSE_SYMLINK\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_MKNOD as uint32_t,
        commandstr: b"CLTOMA_FUSE_MKNOD\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_MKNOD as uint32_t,
        commandstr: b"MATOCL_FUSE_MKNOD\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_MKDIR as uint32_t,
        commandstr: b"CLTOMA_FUSE_MKDIR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_MKDIR as uint32_t,
        commandstr: b"MATOCL_FUSE_MKDIR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_UNLINK as uint32_t,
        commandstr: b"CLTOMA_FUSE_UNLINK\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_UNLINK as uint32_t,
        commandstr: b"MATOCL_FUSE_UNLINK\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_RMDIR as uint32_t,
        commandstr: b"CLTOMA_FUSE_RMDIR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_RMDIR as uint32_t,
        commandstr: b"MATOCL_FUSE_RMDIR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_RENAME as uint32_t,
        commandstr: b"CLTOMA_FUSE_RENAME\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_RENAME as uint32_t,
        commandstr: b"MATOCL_FUSE_RENAME\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_LINK as uint32_t,
        commandstr: b"CLTOMA_FUSE_LINK\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_LINK as uint32_t,
        commandstr: b"MATOCL_FUSE_LINK\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_READDIR as uint32_t,
        commandstr: b"CLTOMA_FUSE_READDIR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_READDIR as uint32_t,
        commandstr: b"MATOCL_FUSE_READDIR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_OPEN as uint32_t,
        commandstr: b"CLTOMA_FUSE_OPEN\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_OPEN as uint32_t,
        commandstr: b"MATOCL_FUSE_OPEN\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_READ_CHUNK as uint32_t,
        commandstr: b"CLTOMA_FUSE_READ_CHUNK\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_READ_CHUNK as uint32_t,
        commandstr: b"MATOCL_FUSE_READ_CHUNK\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_WRITE_CHUNK as uint32_t,
        commandstr: b"CLTOMA_FUSE_WRITE_CHUNK\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_WRITE_CHUNK as uint32_t,
        commandstr: b"MATOCL_FUSE_WRITE_CHUNK\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_WRITE_CHUNK_END as uint32_t,
        commandstr: b"CLTOMA_FUSE_WRITE_CHUNK_END\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_WRITE_CHUNK_END as uint32_t,
        commandstr: b"MATOCL_FUSE_WRITE_CHUNK_END\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_APPEND_SLICE as uint32_t,
        commandstr: b"CLTOMA_FUSE_APPEND_SLICE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_APPEND_SLICE as uint32_t,
        commandstr: b"MATOCL_FUSE_APPEND_SLICE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_CHECK as uint32_t,
        commandstr: b"CLTOMA_FUSE_CHECK\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_CHECK as uint32_t,
        commandstr: b"MATOCL_FUSE_CHECK\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_GETTRASHRETENTION as uint32_t,
        commandstr: b"CLTOMA_FUSE_GETTRASHRETENTION\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_GETTRASHRETENTION as uint32_t,
        commandstr: b"MATOCL_FUSE_GETTRASHRETENTION\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_SETTRASHRETENTION as uint32_t,
        commandstr: b"CLTOMA_FUSE_SETTRASHRETENTION\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_SETTRASHRETENTION as uint32_t,
        commandstr: b"MATOCL_FUSE_SETTRASHRETENTION\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_GETSCLASS as uint32_t,
        commandstr: b"CLTOMA_FUSE_GETSCLASS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_GETSCLASS as uint32_t,
        commandstr: b"MATOCL_FUSE_GETSCLASS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_SETSCLASS as uint32_t,
        commandstr: b"CLTOMA_FUSE_SETSCLASS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_SETSCLASS as uint32_t,
        commandstr: b"MATOCL_FUSE_SETSCLASS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_GETTRASH as uint32_t,
        commandstr: b"CLTOMA_FUSE_GETTRASH\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_GETTRASH as uint32_t,
        commandstr: b"MATOCL_FUSE_GETTRASH\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_GETDETACHEDATTR as uint32_t,
        commandstr: b"CLTOMA_FUSE_GETDETACHEDATTR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_GETDETACHEDATTR as uint32_t,
        commandstr: b"MATOCL_FUSE_GETDETACHEDATTR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_GETTRASHPATH as uint32_t,
        commandstr: b"CLTOMA_FUSE_GETTRASHPATH\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_GETTRASHPATH as uint32_t,
        commandstr: b"MATOCL_FUSE_GETTRASHPATH\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_SETTRASHPATH as uint32_t,
        commandstr: b"CLTOMA_FUSE_SETTRASHPATH\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_SETTRASHPATH as uint32_t,
        commandstr: b"MATOCL_FUSE_SETTRASHPATH\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_UNDEL as uint32_t,
        commandstr: b"CLTOMA_FUSE_UNDEL\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_UNDEL as uint32_t,
        commandstr: b"MATOCL_FUSE_UNDEL\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_PURGE as uint32_t,
        commandstr: b"CLTOMA_FUSE_PURGE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_PURGE as uint32_t,
        commandstr: b"MATOCL_FUSE_PURGE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_GETDIRSTATS as uint32_t,
        commandstr: b"CLTOMA_FUSE_GETDIRSTATS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_GETDIRSTATS as uint32_t,
        commandstr: b"MATOCL_FUSE_GETDIRSTATS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_TRUNCATE as uint32_t,
        commandstr: b"CLTOMA_FUSE_TRUNCATE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_TRUNCATE as uint32_t,
        commandstr: b"MATOCL_FUSE_TRUNCATE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_REPAIR as uint32_t,
        commandstr: b"CLTOMA_FUSE_REPAIR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_REPAIR as uint32_t,
        commandstr: b"MATOCL_FUSE_REPAIR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_SNAPSHOT as uint32_t,
        commandstr: b"CLTOMA_FUSE_SNAPSHOT\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_SNAPSHOT as uint32_t,
        commandstr: b"MATOCL_FUSE_SNAPSHOT\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_GETSUSTAINED as uint32_t,
        commandstr: b"CLTOMA_FUSE_GETSUSTAINED\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_GETSUSTAINED as uint32_t,
        commandstr: b"MATOCL_FUSE_GETSUSTAINED\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_GETEATTR as uint32_t,
        commandstr: b"CLTOMA_FUSE_GETEATTR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_GETEATTR as uint32_t,
        commandstr: b"MATOCL_FUSE_GETEATTR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_SETEATTR as uint32_t,
        commandstr: b"CLTOMA_FUSE_SETEATTR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_SETEATTR as uint32_t,
        commandstr: b"MATOCL_FUSE_SETEATTR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_QUOTACONTROL as uint32_t,
        commandstr: b"CLTOMA_FUSE_QUOTACONTROL\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_QUOTACONTROL as uint32_t,
        commandstr: b"MATOCL_FUSE_QUOTACONTROL\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_GETXATTR as uint32_t,
        commandstr: b"CLTOMA_FUSE_GETXATTR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_GETXATTR as uint32_t,
        commandstr: b"MATOCL_FUSE_GETXATTR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_SETXATTR as uint32_t,
        commandstr: b"CLTOMA_FUSE_SETXATTR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_SETXATTR as uint32_t,
        commandstr: b"MATOCL_FUSE_SETXATTR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_CREATE as uint32_t,
        commandstr: b"CLTOMA_FUSE_CREATE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_CREATE as uint32_t,
        commandstr: b"MATOCL_FUSE_CREATE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_PARENTS as uint32_t,
        commandstr: b"CLTOMA_FUSE_PARENTS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_PARENTS as uint32_t,
        commandstr: b"MATOCL_FUSE_PARENTS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_PATHS as uint32_t,
        commandstr: b"CLTOMA_FUSE_PATHS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_PATHS as uint32_t,
        commandstr: b"MATOCL_FUSE_PATHS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_GETFACL as uint32_t,
        commandstr: b"CLTOMA_FUSE_GETFACL\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_GETFACL as uint32_t,
        commandstr: b"MATOCL_FUSE_GETFACL\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_SETFACL as uint32_t,
        commandstr: b"CLTOMA_FUSE_SETFACL\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_SETFACL as uint32_t,
        commandstr: b"MATOCL_FUSE_SETFACL\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_FLOCK as uint32_t,
        commandstr: b"CLTOMA_FUSE_FLOCK\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_FLOCK as uint32_t,
        commandstr: b"MATOCL_FUSE_FLOCK\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_POSIX_LOCK as uint32_t,
        commandstr: b"CLTOMA_FUSE_POSIX_LOCK\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_POSIX_LOCK as uint32_t,
        commandstr: b"MATOCL_FUSE_POSIX_LOCK\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_ARCHCTL as uint32_t,
        commandstr: b"CLTOMA_FUSE_ARCHCTL\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_ARCHCTL as uint32_t,
        commandstr: b"MATOCL_FUSE_ARCHCTL\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_FSYNC as uint32_t,
        commandstr: b"CLTOMA_FUSE_FSYNC\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_FSYNC as uint32_t,
        commandstr: b"MATOCL_FUSE_FSYNC\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_SUSTAINED_INODES_DEPRECATED as uint32_t,
        commandstr: b"CLTOMA_FUSE_SUSTAINED_INODES_DEPRECATED\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_CSERV_LIST as uint32_t,
        commandstr: b"CLTOMA_CSERV_LIST\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_CSERV_LIST as uint32_t,
        commandstr: b"MATOCL_CSERV_LIST\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOAN_MONOTONIC_DATA as uint32_t,
        commandstr: b"CLTOAN_MONOTONIC_DATA\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOCL_MONOTONIC_DATA as uint32_t,
        commandstr: b"ANTOCL_MONOTONIC_DATA\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOAN_CHART as uint32_t,
        commandstr: b"CLTOAN_CHART\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOCL_CHART as uint32_t,
        commandstr: b"ANTOCL_CHART\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOAN_CHART_DATA as uint32_t,
        commandstr: b"CLTOAN_CHART_DATA\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOCL_CHART_DATA as uint32_t,
        commandstr: b"ANTOCL_CHART_DATA\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_SESSION_LIST as uint32_t,
        commandstr: b"CLTOMA_SESSION_LIST\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_SESSION_LIST as uint32_t,
        commandstr: b"MATOCL_SESSION_LIST\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_INFO as uint32_t,
        commandstr: b"CLTOMA_INFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_INFO as uint32_t,
        commandstr: b"MATOCL_INFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FSTEST_INFO as uint32_t,
        commandstr: b"CLTOMA_FSTEST_INFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FSTEST_INFO as uint32_t,
        commandstr: b"MATOCL_FSTEST_INFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_CHUNKSTEST_INFO as uint32_t,
        commandstr: b"CLTOMA_CHUNKSTEST_INFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_CHUNKSTEST_INFO as uint32_t,
        commandstr: b"MATOCL_CHUNKSTEST_INFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_CHUNKS_MATRIX as uint32_t,
        commandstr: b"CLTOMA_CHUNKS_MATRIX\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_CHUNKS_MATRIX as uint32_t,
        commandstr: b"MATOCL_CHUNKS_MATRIX\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_QUOTA_INFO as uint32_t,
        commandstr: b"CLTOMA_QUOTA_INFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_QUOTA_INFO as uint32_t,
        commandstr: b"MATOCL_QUOTA_INFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_EXPORTS_INFO as uint32_t,
        commandstr: b"CLTOMA_EXPORTS_INFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_EXPORTS_INFO as uint32_t,
        commandstr: b"MATOCL_EXPORTS_INFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_MLOG_LIST as uint32_t,
        commandstr: b"CLTOMA_MLOG_LIST\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_MLOG_LIST as uint32_t,
        commandstr: b"MATOCL_MLOG_LIST\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_CSSERV_COMMAND as uint32_t,
        commandstr: b"CLTOMA_CSSERV_COMMAND\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_CSSERV_COMMAND as uint32_t,
        commandstr: b"MATOCL_CSSERV_COMMAND\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_SESSION_COMMAND as uint32_t,
        commandstr: b"CLTOMA_SESSION_COMMAND\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_SESSION_COMMAND as uint32_t,
        commandstr: b"MATOCL_SESSION_COMMAND\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_MEMORY_INFO as uint32_t,
        commandstr: b"CLTOMA_MEMORY_INFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_MEMORY_INFO as uint32_t,
        commandstr: b"MATOCL_MEMORY_INFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOAN_MODULE_INFO as uint32_t,
        commandstr: b"CLTOAN_MODULE_INFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: ANTOCL_MODULE_INFO as uint32_t,
        commandstr: b"ANTOCL_MODULE_INFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_LIST_OPEN_FILES as uint32_t,
        commandstr: b"CLTOMA_LIST_OPEN_FILES\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_LIST_OPEN_FILES as uint32_t,
        commandstr: b"MATOCL_LIST_OPEN_FILES\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_LIST_ACQUIRED_LOCKS as uint32_t,
        commandstr: b"CLTOMA_LIST_ACQUIRED_LOCKS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_LIST_ACQUIRED_LOCKS as uint32_t,
        commandstr: b"MATOCL_LIST_ACQUIRED_LOCKS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_MASS_RESOLVE_PATHS as uint32_t,
        commandstr: b"CLTOMA_MASS_RESOLVE_PATHS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_MASS_RESOLVE_PATHS as uint32_t,
        commandstr: b"MATOCL_MASS_RESOLVE_PATHS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_SCLASS_INFO as uint32_t,
        commandstr: b"CLTOMA_SCLASS_INFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_SCLASS_INFO as uint32_t,
        commandstr: b"MATOCL_SCLASS_INFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_MISSING_CHUNKS as uint32_t,
        commandstr: b"CLTOMA_MISSING_CHUNKS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_MISSING_CHUNKS as uint32_t,
        commandstr: b"MATOCL_MISSING_CHUNKS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_NODE_INFO as uint32_t,
        commandstr: b"CLTOMA_NODE_INFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_NODE_INFO as uint32_t,
        commandstr: b"MATOCL_NODE_INFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_PATTERN_INFO as uint32_t,
        commandstr: b"CLTOMA_PATTERN_INFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_PATTERN_INFO as uint32_t,
        commandstr: b"MATOCL_PATTERN_INFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_INSTANCE_NAME as uint32_t,
        commandstr: b"CLTOMA_INSTANCE_NAME\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_INSTANCE_NAME as uint32_t,
        commandstr: b"MATOCL_INSTANCE_NAME\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FULL_DIRECTORY_DATA as uint32_t,
        commandstr: b"CLTOMA_FULL_DIRECTORY_DATA\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FULL_DIRECTORY_DATA as uint32_t,
        commandstr: b"MATOCL_FULL_DIRECTORY_DATA\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_SET_ALL_NODE_ATTRIBUTES as uint32_t,
        commandstr: b"CLTOMA_SET_ALL_NODE_ATTRIBUTES\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_SET_ALL_NODE_ATTRIBUTES as uint32_t,
        commandstr: b"MATOCL_SET_ALL_NODE_ATTRIBUTES\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOCS_HDD_LIST as uint32_t,
        commandstr: b"CLTOCS_HDD_LIST\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CSTOCL_HDD_LIST as uint32_t,
        commandstr: b"CSTOCL_HDD_LIST\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_SUSTAINED_INODES as uint32_t,
        commandstr: b"CLTOMA_FUSE_SUSTAINED_INODES\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_AMTIME_INODES as uint32_t,
        commandstr: b"CLTOMA_FUSE_AMTIME_INODES\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_CHUNK_HAS_CHANGED as uint32_t,
        commandstr: b"MATOCL_FUSE_CHUNK_HAS_CHANGED\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_FLENG_HAS_CHANGED as uint32_t,
        commandstr: b"MATOCL_FUSE_FLENG_HAS_CHANGED\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_TIME_SYNC as uint32_t,
        commandstr: b"CLTOMA_FUSE_TIME_SYNC\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_TIME_SYNC as uint32_t,
        commandstr: b"MATOCL_FUSE_TIME_SYNC\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: MATOCL_FUSE_INVALIDATE_CHUNK_CACHE as uint32_t,
        commandstr: b"MATOCL_FUSE_INVALIDATE_CHUNK_CACHE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_OPDATA as uint32_t,
        commandstr: b"CLTOMA_FUSE_OPDATA\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: CLTOMA_FUSE_WFLAGS as uint32_t,
        commandstr: b"CLTOMA_FUSE_WFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    _mfscmd {
        command: 0 as uint32_t,
        commandstr: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
static mut mfscmdtab: *mut mfscommand = ::core::ptr::null_mut::<mfscommand>();
static mut mfscmdtableng: uint32_t = 0 as uint32_t;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn commands_cmp(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut aa: *const mfscommand = a as *const mfscommand;
        let mut bb: *const mfscommand = b as *const mfscommand;
        return if (*aa).command > (*bb).command {
            1 as ::core::ffi::c_int
        } else if (*aa).command < (*bb).command {
            -1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn commands_convert() {
    unsafe {
        let mut i: uint32_t = 0;
        let mut ccode: uint8_t = 0;
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        mfscmdtableng = 0 as uint32_t;
        i = 0 as uint32_t;
        while !cmdtab[i as usize].commandstr.is_null() {
            mfscmdtableng = mfscmdtableng.wrapping_add(1);
            i = i.wrapping_add(1);
        }
        if mfscmdtableng == 0 as uint32_t {
            mfscmdtab = ::core::ptr::null_mut::<mfscommand>();
            return;
        }
        mfscmdtab =
            malloc(::core::mem::size_of::<mfscommand>().wrapping_mul(mfscmdtableng as size_t))
                as *mut mfscommand;
        i = 0 as uint32_t;
        while !cmdtab[i as usize].commandstr.is_null() {
            (*mfscmdtab.offset(i as isize)).command = cmdtab[i as usize].command;
            (*mfscmdtab.offset(i as isize)).commandstr = strdup(cmdtab[i as usize].commandstr);
            p = (*mfscmdtab.offset(i as isize)).commandstr;
            ccode = 0 as uint8_t;
            if *p.offset(0 as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                && *p.offset(1 as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                && *p.offset(2 as isize) as ::core::ffi::c_int == 'T' as ::core::ffi::c_int
                && *p.offset(3 as isize) as ::core::ffi::c_int == 'O' as ::core::ffi::c_int
            {
                if *p.offset(0 as isize) as ::core::ffi::c_int == 'A' as ::core::ffi::c_int
                    && *p.offset(1 as isize) as ::core::ffi::c_int == 'N' as ::core::ffi::c_int
                {
                    if *p.offset(4 as isize) as ::core::ffi::c_int == 'A' as ::core::ffi::c_int
                        && *p.offset(5 as isize) as ::core::ffi::c_int == 'N' as ::core::ffi::c_int
                    {
                        ccode = 8 as uint8_t;
                    } else if *p.offset(4 as isize) as ::core::ffi::c_int
                        == 'C' as ::core::ffi::c_int
                        && *p.offset(5 as isize) as ::core::ffi::c_int == 'L' as ::core::ffi::c_int
                    {
                        ccode = 13 as uint8_t;
                    } else if *p.offset(4 as isize) as ::core::ffi::c_int
                        == 'C' as ::core::ffi::c_int
                        && *p.offset(5 as isize) as ::core::ffi::c_int == 'S' as ::core::ffi::c_int
                    {
                        ccode = 14 as uint8_t;
                    } else if *p.offset(4 as isize) as ::core::ffi::c_int
                        == 'M' as ::core::ffi::c_int
                        && *p.offset(5 as isize) as ::core::ffi::c_int == 'A' as ::core::ffi::c_int
                    {
                        ccode = 1 as uint8_t;
                    }
                } else if *p.offset(0 as isize) as ::core::ffi::c_int == 'C' as ::core::ffi::c_int
                    && *p.offset(1 as isize) as ::core::ffi::c_int == 'L' as ::core::ffi::c_int
                {
                    if *p.offset(4 as isize) as ::core::ffi::c_int == 'A' as ::core::ffi::c_int
                        && *p.offset(5 as isize) as ::core::ffi::c_int == 'N' as ::core::ffi::c_int
                    {
                        ccode = 5 as uint8_t;
                    } else if *p.offset(4 as isize) as ::core::ffi::c_int
                        == 'C' as ::core::ffi::c_int
                        && *p.offset(5 as isize) as ::core::ffi::c_int == 'S' as ::core::ffi::c_int
                    {
                        ccode = 11 as uint8_t;
                    } else if *p.offset(4 as isize) as ::core::ffi::c_int
                        == 'M' as ::core::ffi::c_int
                        && *p.offset(5 as isize) as ::core::ffi::c_int == 'A' as ::core::ffi::c_int
                    {
                        ccode = 10 as uint8_t;
                    }
                } else if *p.offset(0 as isize) as ::core::ffi::c_int == 'C' as ::core::ffi::c_int
                    && *p.offset(1 as isize) as ::core::ffi::c_int == 'S' as ::core::ffi::c_int
                {
                    if *p.offset(4 as isize) as ::core::ffi::c_int == 'A' as ::core::ffi::c_int
                        && *p.offset(5 as isize) as ::core::ffi::c_int == 'N' as ::core::ffi::c_int
                    {
                        ccode = 6 as uint8_t;
                    } else if *p.offset(4 as isize) as ::core::ffi::c_int
                        == 'C' as ::core::ffi::c_int
                        && *p.offset(5 as isize) as ::core::ffi::c_int == 'L' as ::core::ffi::c_int
                    {
                        ccode = 3 as uint8_t;
                    } else if *p.offset(4 as isize) as ::core::ffi::c_int
                        == 'M' as ::core::ffi::c_int
                        && *p.offset(5 as isize) as ::core::ffi::c_int == 'A' as ::core::ffi::c_int
                    {
                        ccode = 4 as uint8_t;
                    }
                } else if *p.offset(0 as isize) as ::core::ffi::c_int == 'M' as ::core::ffi::c_int
                    && *p.offset(1 as isize) as ::core::ffi::c_int == 'A' as ::core::ffi::c_int
                {
                    if *p.offset(4 as isize) as ::core::ffi::c_int == 'A' as ::core::ffi::c_int
                        && *p.offset(5 as isize) as ::core::ffi::c_int == 'N' as ::core::ffi::c_int
                    {
                        ccode = 9 as uint8_t;
                    } else if *p.offset(4 as isize) as ::core::ffi::c_int
                        == 'C' as ::core::ffi::c_int
                        && *p.offset(5 as isize) as ::core::ffi::c_int == 'L' as ::core::ffi::c_int
                    {
                        ccode = 2 as uint8_t;
                    } else if *p.offset(4 as isize) as ::core::ffi::c_int
                        == 'C' as ::core::ffi::c_int
                        && *p.offset(5 as isize) as ::core::ffi::c_int == 'S' as ::core::ffi::c_int
                    {
                        ccode = 12 as uint8_t;
                    }
                }
            }
            (*mfscmdtab.offset(i as isize)).colorcode = ccode;
            (*mfscmdtab.offset(i as isize)).display = 1 as uint8_t;
            i = i.wrapping_add(1);
        }
        qsort(
            mfscmdtab as *mut ::core::ffi::c_void,
            mfscmdtableng as size_t,
            ::core::mem::size_of::<mfscommand>(),
            Some(
                commands_cmp
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn commands_find(
    mut cmd: uint32_t,
    mut color: *mut uint8_t,
    mut display: *mut uint8_t,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut first: int32_t = 0;
        let mut last: int32_t = 0;
        let mut middle: int32_t = 0;
        first = 0 as ::core::ffi::c_int as int32_t;
        last = mfscmdtableng.wrapping_sub(1 as uint32_t) as int32_t;
        middle = (first + last) / 2 as int32_t;
        while first <= last {
            if (*mfscmdtab.offset(middle as isize)).command < cmd {
                first = middle + 1 as int32_t;
            } else if (*mfscmdtab.offset(middle as isize)).command > cmd {
                last = middle - 1 as int32_t;
            } else {
                *color = (*mfscmdtab.offset(middle as isize)).colorcode;
                *display = (*mfscmdtab.offset(middle as isize)).display;
                return (*mfscmdtab.offset(middle as isize)).commandstr;
            }
            middle = (first + last) / 2 as int32_t;
        }
        *color = 0 as uint8_t;
        *display = 0 as uint8_t;
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn commands_exclude(mut opt: *const ::core::ffi::c_char) {
    unsafe {
        let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut i: uint32_t = 0;
        str = strdup(opt);
        p = strtok(str, b" ,;\0".as_ptr() as *const ::core::ffi::c_char);
        while !p.is_null() {
            i = 0 as uint32_t;
            while i < mfscmdtableng {
                if strcmp(p, (*mfscmdtab.offset(i as isize)).commandstr) == 0 as ::core::ffi::c_int
                {
                    (*mfscmdtab.offset(i as isize)).display = 0 as uint8_t;
                }
                i = i.wrapping_add(1);
            }
            p = strtok(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                b" ,;\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        free(str as *mut ::core::ffi::c_void);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn commands_onlyuse(mut opt: *const ::core::ffi::c_char) {
    unsafe {
        let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut i: uint32_t = 0;
        static mut ft: uint8_t = 1 as uint8_t;
        if ft != 0 {
            i = 0 as uint32_t;
            while i < mfscmdtableng {
                (*mfscmdtab.offset(i as isize)).display = 0 as uint8_t;
                i = i.wrapping_add(1);
            }
            ft = 0 as uint8_t;
        }
        str = strdup(opt);
        p = strtok(str, b" ,;\0".as_ptr() as *const ::core::ffi::c_char);
        while !p.is_null() {
            i = 0 as uint32_t;
            while i < mfscmdtableng {
                if strcmp(p, (*mfscmdtab.offset(i as isize)).commandstr) == 0 as ::core::ffi::c_int
                {
                    (*mfscmdtab.offset(i as isize)).display = 1 as uint8_t;
                }
                i = i.wrapping_add(1);
            }
            p = strtok(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                b" ,;\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        free(str as *mut ::core::ffi::c_void);
    }
}
pub const COLOR_ADDR_SRCHI: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"\x1B[38;5;156m\0") };
pub const COLOR_ADDR_DSTHI: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"\x1B[38;5;217m\0") };
pub const COLOR_ADDR_SRCLO: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"\x1B[38;5;194m\0") };
pub const COLOR_ADDR_DSTLO: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"\x1B[38;5;224m\0") };
pub const COLOR_CLEAR: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"\x1B(B\x1B[m\0") };
static mut color_tab: [*const ::core::ffi::c_char; 17] = [
    b"\x1B[30m\0".as_ptr() as *const ::core::ffi::c_char,
    b"\x1B[31m\0".as_ptr() as *const ::core::ffi::c_char,
    b"\x1B[32m\0".as_ptr() as *const ::core::ffi::c_char,
    b"\x1B[33m\0".as_ptr() as *const ::core::ffi::c_char,
    b"\x1B[34m\0".as_ptr() as *const ::core::ffi::c_char,
    b"\x1B[35m\0".as_ptr() as *const ::core::ffi::c_char,
    b"\x1B[36m\0".as_ptr() as *const ::core::ffi::c_char,
    b"\x1B[37m\0".as_ptr() as *const ::core::ffi::c_char,
    b"\x1B[90m\0".as_ptr() as *const ::core::ffi::c_char,
    b"\x1B[91m\0".as_ptr() as *const ::core::ffi::c_char,
    b"\x1B[92m\0".as_ptr() as *const ::core::ffi::c_char,
    b"\x1B[93m\0".as_ptr() as *const ::core::ffi::c_char,
    b"\x1B[94m\0".as_ptr() as *const ::core::ffi::c_char,
    b"\x1B[95m\0".as_ptr() as *const ::core::ffi::c_char,
    b"\x1B[96m\0".as_ptr() as *const ::core::ffi::c_char,
    b"\x1B[97m\0".as_ptr() as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hexdump(mut ptr: *const uint8_t, mut len: uint32_t) {
    unsafe {
        let mut eol: uint8_t = 0;
        let mut i: uint32_t = 0;
        eol = 0 as uint8_t;
        i = 0 as uint32_t;
        while i < len {
            eol = 1 as uint8_t;
            if i & 0x1f as uint32_t == 0 as uint32_t {
                printf(
                    b"\x1B[38;5;159m\t0x%05X:\x1B[38;5;228m\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    i,
                );
            }
            printf(
                b" %02X\0".as_ptr() as *const ::core::ffi::c_char,
                *ptr.offset(i as isize) as ::core::ffi::c_int,
            );
            if i & 0x1f as uint32_t == 0x1f as uint32_t {
                printf(b"\x1B(B\x1B[m\n\0".as_ptr() as *const ::core::ffi::c_char);
                eol = 0 as uint8_t;
            }
            i = i.wrapping_add(1);
        }
        if eol != 0 {
            printf(b"\x1B(B\x1B[m\n\0".as_ptr() as *const ::core::ffi::c_char);
        }
    }
}
static mut connhead: *mut connection = ::core::ptr::null_mut::<connection>();
#[inline]
unsafe extern "C" fn packet_find(
    mut srcip: uint32_t,
    mut srcport: uint16_t,
    mut dstip: uint32_t,
    mut dstport: uint16_t,
) -> *mut *mut connection {
    unsafe {
        let mut c: *mut connection = ::core::ptr::null_mut::<connection>();
        let mut cp: *mut *mut connection = ::core::ptr::null_mut::<*mut connection>();
        cp = &raw mut connhead;
        loop {
            c = *cp;
            if c.is_null() {
                break;
            }
            if (*c).srcip == srcip
                && (*c).dstip == dstip
                && (*c).srcport as ::core::ffi::c_int == srcport as ::core::ffi::c_int
                && (*c).dstport as ::core::ffi::c_int == dstport as ::core::ffi::c_int
            {
                return cp;
            }
            cp = &raw mut (*c).next as *mut *mut connection;
        }
        return ::core::ptr::null_mut::<*mut connection>();
    }
}
#[inline]
unsafe extern "C" fn print_info(
    mut ts: *const timeval,
    mut ip: *const uint8_t,
    mut srcport: uint16_t,
    mut dstport: uint16_t,
) {
    unsafe {
        printf(
            b"\x1B[38;5;231m%ld.%06u : \0".as_ptr() as *const ::core::ffi::c_char,
            (*ts).tv_sec,
            (*ts).tv_usec as ::core::ffi::c_uint,
        );
        printf(
            b"%s%3u.%3u.%3u.%3u : %5hu\x1B(B\x1B[m -> %s%3u.%3u.%3u.%3u : %5hu\x1B(B\x1B[m \0"
                .as_ptr() as *const ::core::ffi::c_char,
            if (srcport as ::core::ffi::c_int) < 49152 as ::core::ffi::c_int {
                COLOR_ADDR_SRCLO.as_ptr()
            } else {
                COLOR_ADDR_SRCHI.as_ptr()
            },
            *ip.offset(12 as isize) as ::core::ffi::c_int,
            *ip.offset(13 as isize) as ::core::ffi::c_int,
            *ip.offset(14 as isize) as ::core::ffi::c_int,
            *ip.offset(15 as isize) as ::core::ffi::c_int,
            srcport as ::core::ffi::c_int,
            if (dstport as ::core::ffi::c_int) < 49152 as ::core::ffi::c_int {
                COLOR_ADDR_DSTLO.as_ptr()
            } else {
                COLOR_ADDR_DSTHI.as_ptr()
            },
            *ip.offset(16 as isize) as ::core::ffi::c_int,
            *ip.offset(17 as isize) as ::core::ffi::c_int,
            *ip.offset(18 as isize) as ::core::ffi::c_int,
            *ip.offset(19 as isize) as ::core::ffi::c_int,
            dstport as ::core::ffi::c_int,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_packet(
    mut args: *mut u_char,
    mut header: *const pcap_pkthdr,
    mut packet: *const u_char,
) {
    unsafe {
        let mut ud: *mut userdata = args as *mut userdata;
        let mut ip: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut tcp: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut payload: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut iplen: uint32_t = 0;
        let mut iphdrlen: uint32_t = 0;
        let mut tcplen: uint32_t = 0;
        let mut tcphdrlen: uint32_t = 0;
        let mut payloadlen: uint32_t = 0;
        let mut seqno: uint32_t = 0;
        let mut skip: uint32_t = 0;
        let mut srcip: uint32_t = 0;
        let mut dstip: uint32_t = 0;
        let mut srcport: uint16_t = 0;
        let mut dstport: uint16_t = 0;
        let mut mfscmd: uint32_t = 0;
        let mut mfslen: uint32_t = 0;
        let mut ccode: uint8_t = 0;
        let mut display: uint8_t = 0;
        let mut commandstr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut c: *mut connection = ::core::ptr::null_mut::<connection>();
        let mut cp: *mut *mut connection = ::core::ptr::null_mut::<*mut connection>();
        let mut bytesskip: uint32_t = 0;
        let mut linktype: uint16_t = 0;
        bytesskip = 0 as uint32_t;
        linktype = (*ud).linktype as uint16_t;
        loop {
            if linktype as ::core::ffi::c_int == DLT_EN10MB {
                bytesskip = bytesskip.wrapping_add(14 as uint32_t);
                break;
            } else if linktype as ::core::ffi::c_int == DLT_NULL {
                bytesskip = bytesskip.wrapping_add(4 as uint32_t);
                break;
            } else {
                if linktype as ::core::ffi::c_int == DLT_RAW {
                    break;
                }
                if linktype as ::core::ffi::c_int == DLT_LINUX_SLL {
                    bytesskip = bytesskip.wrapping_add(16 as uint32_t);
                    break;
                } else if linktype as ::core::ffi::c_int == DLT_PKTAP {
                    linktype = (*packet.offset(bytesskip.wrapping_add(8 as uint32_t) as isize)
                        as ::core::ffi::c_uint)
                        .wrapping_add((256 as ::core::ffi::c_uint).wrapping_mul(
                            *packet.offset(bytesskip.wrapping_add(9 as uint32_t) as isize)
                                as ::core::ffi::c_uint,
                        )) as uint16_t;
                    bytesskip = (bytesskip as ::core::ffi::c_uint).wrapping_add(
                        (*packet.offset(bytesskip as isize) as ::core::ffi::c_uint).wrapping_add(
                            (256 as ::core::ffi::c_uint).wrapping_mul(
                                *packet.offset(bytesskip.wrapping_add(1 as uint32_t) as isize)
                                    as ::core::ffi::c_uint,
                            ),
                        ),
                    ) as uint32_t;
                } else {
                    return;
                }
            }
        }
        if bytesskip >= (*header).caplen as uint32_t {
            return;
        }
        ip = packet.offset(bytesskip as isize) as *const uint8_t;
        iplen = ((*header).caplen as uint32_t).wrapping_sub(bytesskip);
        if *ip.offset(0 as isize) as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
            != 0x40 as ::core::ffi::c_int
        {
            return;
        }
        if iplen < 20 as uint32_t {
            return;
        }
        if *ip.offset(9 as isize) as ::core::ffi::c_int != 6 as ::core::ffi::c_int {
            return;
        }
        iphdrlen = (4 as ::core::ffi::c_int
            * (*ip.offset(0 as isize) as ::core::ffi::c_int & 0xf as ::core::ffi::c_int))
            as uint32_t;
        if iplen < iphdrlen {
            return;
        }
        tcp = ip.offset(iphdrlen as isize);
        tcplen = iplen.wrapping_sub(iphdrlen);
        if tcplen < 20 as uint32_t {
            return;
        }
        tcphdrlen = (4 as ::core::ffi::c_int
            * (*tcp.offset(12 as isize) as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
            as uint32_t;
        if tcplen < tcphdrlen {
            return;
        }
        srcip = (*ip.offset(12 as isize) as ::core::ffi::c_uint)
            .wrapping_mul(256 as ::core::ffi::c_uint)
            .wrapping_add(*ip.offset(13 as isize) as ::core::ffi::c_uint)
            .wrapping_mul(256 as ::core::ffi::c_uint)
            .wrapping_add(*ip.offset(14 as isize) as ::core::ffi::c_uint)
            .wrapping_mul(256 as ::core::ffi::c_uint)
            .wrapping_add(*ip.offset(15 as isize) as ::core::ffi::c_uint)
            as uint32_t;
        dstip = (*ip.offset(16 as isize) as ::core::ffi::c_uint)
            .wrapping_mul(256 as ::core::ffi::c_uint)
            .wrapping_add(*ip.offset(17 as isize) as ::core::ffi::c_uint)
            .wrapping_mul(256 as ::core::ffi::c_uint)
            .wrapping_add(*ip.offset(18 as isize) as ::core::ffi::c_uint)
            .wrapping_mul(256 as ::core::ffi::c_uint)
            .wrapping_add(*ip.offset(19 as isize) as ::core::ffi::c_uint)
            as uint32_t;
        srcport = (*tcp.offset(0 as isize) as ::core::ffi::c_uint)
            .wrapping_mul(256 as ::core::ffi::c_uint)
            .wrapping_add(*tcp.offset(1 as isize) as ::core::ffi::c_uint)
            as uint16_t;
        dstport = (*tcp.offset(2 as isize) as ::core::ffi::c_uint)
            .wrapping_mul(256 as ::core::ffi::c_uint)
            .wrapping_add(*tcp.offset(3 as isize) as ::core::ffi::c_uint)
            as uint16_t;
        if ((srcport as ::core::ffi::c_int) < (*ud).minport as ::core::ffi::c_int
            || srcport as ::core::ffi::c_int > (*ud).maxport as ::core::ffi::c_int)
            && ((dstport as ::core::ffi::c_int) < (*ud).minport as ::core::ffi::c_int
                || dstport as ::core::ffi::c_int > (*ud).maxport as ::core::ffi::c_int)
        {
            return;
        }
        seqno = (*tcp.offset(4 as isize) as ::core::ffi::c_uint)
            .wrapping_mul(256 as ::core::ffi::c_uint)
            .wrapping_add(*tcp.offset(5 as isize) as ::core::ffi::c_uint)
            .wrapping_mul(256 as ::core::ffi::c_uint)
            .wrapping_add(*tcp.offset(6 as isize) as ::core::ffi::c_uint)
            .wrapping_mul(256 as ::core::ffi::c_uint)
            .wrapping_add(*tcp.offset(7 as isize) as ::core::ffi::c_uint)
            as uint32_t;
        cp = packet_find(srcip, srcport, dstip, dstport);
        if *tcp.offset(13 as isize) as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int != 0 {
            if (*ud).showconnections != 0 {
                print_info(&raw const (*header).ts, ip, srcport, dstport);
                printf(
                    b"\x1B[38;5;116m... new connection ...\x1B(B\x1B[m\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        }
        if *tcp.offset(13 as isize) as ::core::ffi::c_int & 0x5 as ::core::ffi::c_int != 0 {
            if (*ud).showconnections != 0 {
                print_info(&raw const (*header).ts, ip, srcport, dstport);
                printf(
                    b"\x1B[38;5;100m... close connection ...\x1B(B\x1B[m\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            if !cp.is_null() {
                c = *cp;
                *cp = (*c).next as *mut connection;
                free(c as *mut ::core::ffi::c_void);
            }
            return;
        }
        payload = tcp.offset(tcphdrlen as isize);
        payloadlen = tcplen.wrapping_sub(tcphdrlen);
        if !cp.is_null() {
            c = *cp;
            if (*c).seq > seqno {
                skip = (*c).seq.wrapping_sub(seqno);
                print_info(&raw const (*header).ts, ip, srcport, dstport);
                printf(
                    b"\x1B[38;5;180m... data in packet ...\x1B(B\x1B[m\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            } else {
                skip = 0 as uint32_t;
            }
            if skip < payloadlen {
                payload = payload.offset(skip as isize);
                payloadlen = payloadlen.wrapping_sub(skip);
            } else {
                return;
            }
        }
        while payloadlen >= 8 as uint32_t {
            mfscmd = (*payload.offset(0 as isize) as ::core::ffi::c_uint)
                .wrapping_mul(256 as ::core::ffi::c_uint)
                .wrapping_add(*payload.offset(1 as isize) as ::core::ffi::c_uint)
                .wrapping_mul(256 as ::core::ffi::c_uint)
                .wrapping_add(*payload.offset(2 as isize) as ::core::ffi::c_uint)
                .wrapping_mul(256 as ::core::ffi::c_uint)
                .wrapping_add(*payload.offset(3 as isize) as ::core::ffi::c_uint)
                as uint32_t;
            mfslen = (*payload.offset(4 as isize) as ::core::ffi::c_uint)
                .wrapping_mul(256 as ::core::ffi::c_uint)
                .wrapping_add(*payload.offset(5 as isize) as ::core::ffi::c_uint)
                .wrapping_mul(256 as ::core::ffi::c_uint)
                .wrapping_add(*payload.offset(6 as isize) as ::core::ffi::c_uint)
                .wrapping_mul(256 as ::core::ffi::c_uint)
                .wrapping_add(*payload.offset(7 as isize) as ::core::ffi::c_uint)
                as uint32_t;
            commandstr = commands_find(mfscmd, &raw mut ccode, &raw mut display);
            if (*ud).showmfsnops as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && mfscmd == ANTOAN_NOP as uint32_t
            {
                display = 0 as uint8_t;
            }
            if !commandstr.is_null() && mfslen <= 100000000 as uint32_t {
                if display != 0 {
                    print_info(&raw const (*header).ts, ip, srcport, dstport);
                    if ccode as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
                        && ccode as ::core::ffi::c_int <= 15 as ::core::ffi::c_int
                    {
                        printf(
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            color_tab[ccode as usize],
                        );
                    }
                    printf(b"%s\0".as_ptr() as *const ::core::ffi::c_char, commandstr);
                    if ccode != 0 {
                        printf(COLOR_CLEAR.as_ptr());
                    }
                    printf(b" (%u)\n\0".as_ptr() as *const ::core::ffi::c_char, mfslen);
                    if payloadlen.wrapping_sub(8 as uint32_t) <= (*ud).maxdatainpacket {
                        if mfslen < payloadlen.wrapping_sub(8 as uint32_t) {
                            hexdump(payload.offset(8 as ::core::ffi::c_int as isize), mfslen);
                        } else {
                            hexdump(
                                payload.offset(8 as ::core::ffi::c_int as isize),
                                payloadlen.wrapping_sub(8 as uint32_t),
                            );
                        }
                    } else if mfslen < (*ud).maxdatainpacket {
                        hexdump(payload.offset(8 as ::core::ffi::c_int as isize), mfslen);
                    } else {
                        hexdump(
                            payload.offset(8 as ::core::ffi::c_int as isize),
                            (*ud).maxdatainpacket,
                        );
                        printf(b"\t(...)\n\0".as_ptr() as *const ::core::ffi::c_char);
                    }
                }
                if mfslen.wrapping_add(8 as uint32_t) == payloadlen {
                    if !cp.is_null() {
                        c = *cp;
                        *cp = (*c).next as *mut connection;
                        free(c as *mut ::core::ffi::c_void);
                    }
                    payloadlen = 0 as uint32_t;
                    payload = ::core::ptr::null::<uint8_t>();
                } else if mfslen.wrapping_add(8 as uint32_t) < payloadlen {
                    payloadlen = payloadlen.wrapping_sub(mfslen.wrapping_add(8 as uint32_t));
                    payload = payload.offset(mfslen.wrapping_add(8 as uint32_t) as isize);
                    seqno = seqno.wrapping_add(mfslen.wrapping_add(8 as uint32_t));
                } else {
                    if !cp.is_null() {
                        c = *cp;
                        (*c).seq = seqno.wrapping_add(mfslen).wrapping_add(8 as uint32_t);
                    } else {
                        c = malloc(::core::mem::size_of::<connection>()) as *mut connection;
                        (*c).srcip = srcip;
                        (*c).dstip = dstip;
                        (*c).srcport = srcport;
                        (*c).dstport = dstport;
                        (*c).seq = seqno.wrapping_add(mfslen).wrapping_add(8 as uint32_t);
                        (*c).next = connhead as *mut _connection;
                        connhead = c;
                    }
                    payloadlen = 0 as uint32_t;
                    payload = ::core::ptr::null::<uint8_t>();
                }
            } else {
                print_info(&raw const (*header).ts, ip, srcport, dstport);
                printf(
                    b"\x1B[38;5;199m... not mfs packet (%u:%u) ...\x1B(B\x1B[m\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    mfscmd,
                    mfslen,
                );
                if !cp.is_null() {
                    c = *cp;
                    *cp = (*c).next as *mut connection;
                    free(c as *mut ::core::ffi::c_void);
                }
                return;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn usage(mut appname: *const ::core::ffi::c_char) {
    unsafe {
        fprintf(
            stderr,
            b"usage: %s -l | %s [-xyn] [-r pcap_file] [-i interface] [-p portrange] [-f pcap_filter] [-c packet_count] [-s max_bytes_to_show] [-e commands] [-o commands]\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            appname,
            appname,
        );
        fprintf(
            stderr,
            b"\t-e: do not display this commands\n\t-o: when present only this commands will be displayed\n\t-x: ignore maintenance packets like 'CSTOMA_SPACE' or 'CLTOMA_FUSE_TIME_SYNC'\n\t-y: do not show SYN/FIN packets\n\t-n: show NOP packets\n\t-p: show only packets on given port range (default: 9419-9422)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
pub const ARGLIST: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"s:p:i:f:c:e:o:r:hxyln?\0")
};
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ch: ::core::ffi::c_int = 0;
        let mut errbuf: [::core::ffi::c_char; 256] = [0; 256];
        let mut dev: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut filter: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut pcapfile: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut optaux: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut packetcnt: int32_t = 0;
        let mut devnet: bpf_u_int32 = 0;
        let mut devmask: bpf_u_int32 = 0;
        let mut alldevsp: *mut pcap_if_t = ::core::ptr::null_mut::<pcap_if_t>();
        let mut devit: *mut pcap_if_t = ::core::ptr::null_mut::<pcap_if_t>();
        let mut handle: *mut pcap_t = ::core::ptr::null_mut::<pcap_t>();
        let mut datalink: ::core::ffi::c_int = 0;
        let mut fp: bpf_program = bpf_program {
            bf_len: 0,
            bf_insns: ::core::ptr::null_mut::<bpf_insn>(),
        };
        let mut udm: userdata = userdata {
            linktype: 0,
            showmfsnops: 0,
            showconnections: 0,
            minport: 0,
            maxport: 0,
            maxdatainpacket: 0,
        };
        let mut ok: uint8_t = 1 as uint8_t;
        commands_convert();
        dev = ::core::ptr::null_mut::<::core::ffi::c_char>();
        filter = ::core::ptr::null_mut::<::core::ffi::c_char>();
        pcapfile = ::core::ptr::null_mut::<::core::ffi::c_char>();
        packetcnt = -1 as ::core::ffi::c_int as int32_t;
        udm.maxdatainpacket = 128 as uint32_t;
        udm.showconnections = 1 as uint8_t;
        udm.minport = 9419 as uint16_t;
        udm.maxport = 9422 as uint16_t;
        udm.showmfsnops = 0 as uint8_t;
        '_err: {
            loop {
                ch = getopt(argc, argv, ARGLIST.as_ptr());
                if ch == -1 as ::core::ffi::c_int {
                    break;
                }
                match ch {
                    115 => {
                        udm.maxdatainpacket = strtoul(
                            optarg,
                            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                            0 as ::core::ffi::c_int,
                        ) as uint32_t;
                    }
                    112 => {
                        if *optarg.offset(0 as isize) as ::core::ffi::c_int
                            == '*' as ::core::ffi::c_int
                        {
                            udm.minport = 0 as uint16_t;
                            udm.maxport = 65535 as uint16_t;
                        } else {
                            udm.minport = strtoul(optarg, &raw mut optaux, 10 as ::core::ffi::c_int)
                                as uint16_t;
                            while *optaux as ::core::ffi::c_int == ' ' as ::core::ffi::c_int {
                                optaux = optaux.offset(1);
                            }
                            if *optaux as ::core::ffi::c_int == '-' as ::core::ffi::c_int {
                                optaux = optaux.offset(1);
                                while *optaux as ::core::ffi::c_int == ' ' as ::core::ffi::c_int {
                                    optaux = optaux.offset(1);
                                }
                                udm.maxport = strtoul(
                                    optaux,
                                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                                    10 as ::core::ffi::c_int,
                                ) as uint16_t;
                            } else {
                                udm.maxport = udm.minport;
                            }
                        }
                    }
                    105 => {
                        if !dev.is_null() {
                            free(dev as *mut ::core::ffi::c_void);
                        }
                        dev = strdup(optarg);
                    }
                    102 => {
                        if !filter.is_null() {
                            free(filter as *mut ::core::ffi::c_void);
                        }
                        filter = strdup(optarg);
                    }
                    99 => {
                        packetcnt = strtol(
                            optarg,
                            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                            0 as ::core::ffi::c_int,
                        ) as int32_t;
                    }
                    101 => {
                        commands_exclude(optarg);
                    }
                    111 => {
                        commands_onlyuse(optarg);
                    }
                    120 => {
                        commands_exclude(
                            b"ANTOMA_REGISTER,MATOAN_STATE,CSTOMA_SPACE,CLTOMA_FUSE_SUSTAINED_INODES,CSTOMA_CURRENT_LOAD,MATOCS_MANAGER_OFFSET_TIME,CLTOMA_FUSE_TIME_SYNC,MATOCL_FUSE_TIME_SYNC\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                    121 => {
                        udm.showconnections = 0 as uint8_t;
                    }
                    110 => {
                        udm.showmfsnops = 1 as uint8_t;
                    }
                    114 => {
                        if !pcapfile.is_null() {
                            free(pcapfile as *mut ::core::ffi::c_void);
                        }
                        pcapfile = strdup(optarg);
                    }
                    108 => {
                        if pcap_findalldevs(
                            &raw mut alldevsp,
                            &raw mut errbuf as *mut ::core::ffi::c_char,
                        ) < 0 as ::core::ffi::c_int
                        {
                            fprintf(
                                stderr,
                                b"Couldn't find default device: %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut errbuf as *mut ::core::ffi::c_char,
                            );
                            break '_err;
                        } else {
                            ok = 0 as uint8_t;
                            if alldevsp.is_null() {
                                printf(b"Network device list is empty\n\0".as_ptr()
                                    as *const ::core::ffi::c_char);
                                break '_err;
                            } else {
                                devit = alldevsp;
                                while !devit.is_null() {
                                    if !(*devit).addresses.is_null()
                                        && (*devit).flags
                                            & (PCAP_IF_UP | PCAP_IF_RUNNING) as bpf_u_int32
                                            == (PCAP_IF_UP | PCAP_IF_RUNNING) as bpf_u_int32
                                    {
                                        printf(
                                            b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                                            (*devit).name,
                                        );
                                    }
                                    devit = (*devit).next as *mut pcap_if_t;
                                }
                                pcap_freealldevs(alldevsp);
                                break '_err;
                            }
                        }
                    }
                    _ => {
                        usage(*argv.offset(0 as isize));
                        break '_err;
                    }
                }
            }
            if !dev.is_null() && !pcapfile.is_null() {
                fprintf(
                    stderr,
                    b"Options '-i' and '-r' are mutually exclusive\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            } else {
                if dev.is_null() && pcapfile.is_null() {
                    if pcap_lookupnet(
                        b"any\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut devnet,
                        &raw mut devmask,
                        &raw mut errbuf as *mut ::core::ffi::c_char,
                    ) < 0 as ::core::ffi::c_int
                    {
                        if pcap_findalldevs(
                            &raw mut alldevsp,
                            &raw mut errbuf as *mut ::core::ffi::c_char,
                        ) < 0 as ::core::ffi::c_int
                        {
                            fprintf(
                                stderr,
                                b"Couldn't find default device: %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                &raw mut errbuf as *mut ::core::ffi::c_char,
                            );
                            break '_err;
                        } else if alldevsp.is_null() {
                            fprintf(
                                stderr,
                                b"Couldn't find default device (empty devices list)\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            break '_err;
                        } else {
                            dev = strdup((*alldevsp).name);
                            pcap_freealldevs(alldevsp);
                        }
                    } else {
                        dev = strdup(b"any\0".as_ptr() as *const ::core::ffi::c_char);
                    }
                    if dev.is_null() {
                        fprintf(
                            stderr,
                            b"Couldn't find default device: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            &raw mut errbuf as *mut ::core::ffi::c_char,
                        );
                        break '_err;
                    }
                }
                if !pcapfile.is_null() {
                    handle =
                        pcap_open_offline(pcapfile, &raw mut errbuf as *mut ::core::ffi::c_char);
                    if handle.is_null() {
                        fprintf(
                            stderr,
                            b"Couldn't open pcap file %s: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            pcapfile,
                            &raw mut errbuf as *mut ::core::ffi::c_char,
                        );
                        break '_err;
                    } else {
                        devnet = PCAP_NETMASK_UNKNOWN as bpf_u_int32;
                    }
                } else {
                    if pcap_lookupnet(
                        dev,
                        &raw mut devnet,
                        &raw mut devmask,
                        &raw mut errbuf as *mut ::core::ffi::c_char,
                    ) < 0 as ::core::ffi::c_int
                    {
                        printf(
                            b"Device: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                            dev,
                        );
                        fprintf(
                            stderr,
                            b"Couldn't get netmask for device %s: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            dev,
                            &raw mut errbuf as *mut ::core::ffi::c_char,
                        );
                        devnet = 0 as bpf_u_int32;
                        devmask = 0 as bpf_u_int32;
                    } else {
                        printf(
                            b"Device: %s (%u.%u.%u.%u/%u.%u.%u.%u)\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            dev,
                            devnet & 0xff as bpf_u_int32,
                            devnet >> 8 as ::core::ffi::c_int & 0xff as bpf_u_int32,
                            devnet >> 16 as ::core::ffi::c_int & 0xff as bpf_u_int32,
                            devnet >> 24 as ::core::ffi::c_int & 0xff as bpf_u_int32,
                            devmask & 0xff as bpf_u_int32,
                            devmask >> 8 as ::core::ffi::c_int & 0xff as bpf_u_int32,
                            devmask >> 16 as ::core::ffi::c_int & 0xff as bpf_u_int32,
                            devmask >> 24 as ::core::ffi::c_int & 0xff as bpf_u_int32,
                        );
                    }
                    handle = pcap_open_live(
                        dev,
                        100000 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                        1000 as ::core::ffi::c_int,
                        &raw mut errbuf as *mut ::core::ffi::c_char,
                    );
                    if handle.is_null() {
                        fprintf(
                            stderr,
                            b"Couldn't open device %s: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            dev,
                            &raw mut errbuf as *mut ::core::ffi::c_char,
                        );
                        break '_err;
                    }
                }
                datalink = pcap_datalink(handle);
                if false
                    || datalink == DLT_EN10MB
                    || datalink == DLT_NULL
                    || datalink == DLT_RAW
                    || datalink == DLT_LINUX_SLL
                    || datalink == DLT_PKTAP
                {
                    udm.linktype = datalink as uint8_t;
                    if !filter.is_null() {
                        if pcap_compile(
                            handle,
                            &raw mut fp,
                            filter,
                            0 as ::core::ffi::c_int,
                            devnet,
                        ) < 0 as ::core::ffi::c_int
                        {
                            fprintf(
                                stderr,
                                b"Couldn't parse filter %s: %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                filter,
                                pcap_geterr(handle),
                            );
                            break '_err;
                        } else if pcap_setfilter(handle, &raw mut fp) < 0 as ::core::ffi::c_int {
                            fprintf(
                                stderr,
                                b"Couldn't install filter %s: %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                filter,
                                pcap_geterr(handle),
                            );
                            break '_err;
                        }
                    }
                    pcap_loop(
                        handle,
                        packetcnt as ::core::ffi::c_int,
                        Some(
                            parse_packet
                                as unsafe extern "C" fn(
                                    *mut u_char,
                                    *const pcap_pkthdr,
                                    *const u_char,
                                ) -> (),
                        ),
                        &raw mut udm as *mut ::core::ffi::c_void as *mut u_char,
                    );
                    pcap_freecode(&raw mut fp);
                    pcap_close(handle);
                    printf(b"\nCapture complete.\n\0".as_ptr() as *const ::core::ffi::c_char);
                    ok = 0 as uint8_t;
                } else {
                    fprintf(
                        stderr,
                        b"device '%s' uses unsupported datalink type: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        dev,
                        pcap_datalink_val_to_name(datalink),
                    );
                }
            }
        }
        if !pcapfile.is_null() {
            free(pcapfile as *mut ::core::ffi::c_void);
        }
        if !filter.is_null() {
            free(filter as *mut ::core::ffi::c_void);
        }
        if !dev.is_null() {
            free(dev as *mut ::core::ffi::c_void);
        }
        return ok as ::core::ffi::c_int;
    }
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ) as i32)
    }
}
