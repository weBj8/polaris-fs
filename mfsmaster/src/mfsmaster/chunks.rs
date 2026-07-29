pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
pub enum _bio {}
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn strtod(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    unsafe fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
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
    unsafe fn fputc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
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
    unsafe fn main_msectime_register_fname(
        mseconds: uint32_t,
        offset: uint32_t,
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn main_time_register_fname(
        seconds: uint32_t,
        offset: uint32_t,
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn main_msectime_change(
        x: *mut ::core::ffi::c_void,
        mseconds: uint32_t,
        offset: uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn main_time() -> uint32_t;
    unsafe fn main_start_time() -> uint32_t;
    unsafe fn cfg_isdefined(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn cfg_getstr(
        name: *const ::core::ffi::c_char,
        def: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn cfg_getuint8(name: *const ::core::ffi::c_char, def: uint8_t) -> uint8_t;
    unsafe fn cfg_getuint32(name: *const ::core::ffi::c_char, def: uint32_t) -> uint32_t;
    unsafe fn cfg_getdouble(
        name: *const ::core::ffi::c_char,
        def: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    unsafe fn bio_read(b: *mut bio, dst: *mut ::core::ffi::c_void, len: uint64_t) -> int64_t;
    unsafe fn bio_write(b: *mut bio, src: *const ::core::ffi::c_void, len: uint64_t) -> int64_t;
    unsafe fn meta_version_inc() -> uint64_t;
    unsafe fn sclass_get_name(sclassid: uint8_t) -> *const uint8_t;
    unsafe fn sclass_get_labels_mode(sclassid: uint16_t, sm: *mut storagemode) -> uint8_t;
    unsafe fn sclass_get_create_storagemode(sclassid: uint16_t) -> *mut storagemode;
    unsafe fn sclass_get_keeparch_storagemode(
        sclassid: uint16_t,
        flags: uint8_t,
    ) -> *mut storagemode;
    unsafe fn sclass_calc_goal_equivalent(sm: *mut storagemode) -> uint8_t;
    unsafe fn sclass_get_joining_priority(sclassid: uint16_t) -> uint64_t;
    unsafe fn matocsserv_server_matches_labelexpr(
        e: *mut ::core::ffi::c_void,
        labelexpr: *const uint8_t,
    ) -> uint8_t;
    unsafe fn matocsserv_server_get_labelmask(e: *mut ::core::ffi::c_void) -> uint32_t;
    unsafe fn matocsserv_server_get_ip(e: *mut ::core::ffi::c_void) -> uint32_t;
    unsafe fn matocsserv_recalculate_storagemode_scounts(sm: *mut storagemode);
    unsafe fn matocsserv_servers_count() -> uint16_t;
    unsafe fn matocsserv_almostfull_servers() -> uint16_t;
    unsafe fn matocsserv_replallowed_servers() -> uint16_t;
    unsafe fn matocsserv_receiving_chunks_state() -> uint8_t;
    unsafe fn matocsserv_getservers_replpossible(csids: *mut uint16_t) -> uint16_t;
    unsafe fn matocsserv_getservers_replallowed(csids: *mut uint16_t) -> uint16_t;
    unsafe fn matocsserv_getservers_test(
        stdcscnt: *mut uint16_t,
        stdcsids: *mut uint16_t,
        olcscnt: *mut uint16_t,
        olcsids: *mut uint16_t,
        allcscnt: *mut uint16_t,
        allcsids: *mut uint16_t,
    );
    unsafe fn matocsserv_getservers_ordered(csids: *mut uint16_t) -> uint16_t;
    unsafe fn matocsserv_getservers_wrandom(
        csids: *mut uint16_t,
        overloaded: *mut uint16_t,
    ) -> uint16_t;
    unsafe fn matocsserv_useservers_wrandom(servers: *mut *mut ::core::ffi::c_void, cnt: uint16_t);
    unsafe fn matocsserv_getservers_lessrepl(
        csids: *mut uint16_t,
        replimit: ::core::ffi::c_double,
        highpriority: uint8_t,
        allservflag: *mut uint8_t,
    ) -> uint16_t;
    unsafe fn matocsserv_get_server_groups(
        csids: *mut uint16_t,
        replimit: ::core::ffi::c_double,
        counters: *mut uint16_t,
    );
    unsafe fn matocsserv_getstrip(e: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_char;
    unsafe fn matocsserv_get_csdata(
        e: *mut ::core::ffi::c_void,
        clientip: uint32_t,
        servip: *mut uint32_t,
        servport: *mut uint16_t,
        servver: *mut uint32_t,
        servlabelmask: *mut uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn matocsserv_write_counters(e: *mut ::core::ffi::c_void, x: uint8_t);
    unsafe fn matocsserv_has_avail_space(e: *mut ::core::ffi::c_void) -> uint8_t;
    unsafe fn matocsserv_get_usage(e: *mut ::core::ffi::c_void) -> ::core::ffi::c_double;
    unsafe fn matocsserv_replication_write_counter(
        e: *mut ::core::ffi::c_void,
        now: uint32_t,
    ) -> ::core::ffi::c_double;
    unsafe fn matocsserv_replication_read_counter(
        e: *mut ::core::ffi::c_void,
        now: uint32_t,
    ) -> ::core::ffi::c_double;
    unsafe fn matocsserv_deletion_counter(e: *mut ::core::ffi::c_void) -> uint16_t;
    unsafe fn matocsserv_broadcast_chunk_status(chunkid: uint64_t);
    unsafe fn matocsserv_send_replicatechunk(
        e: *mut ::core::ffi::c_void,
        chunkid: uint64_t,
        ecid: uint8_t,
        version: uint32_t,
        src: *mut ::core::ffi::c_void,
        reason: uint8_t,
    ) -> ::core::ffi::c_int;
    unsafe fn matocsserv_send_replicatechunk_split(
        e: *mut ::core::ffi::c_void,
        chunkid: uint64_t,
        ecid: uint8_t,
        version: uint32_t,
        src: *mut ::core::ffi::c_void,
        srcecid: uint8_t,
        partno: uint8_t,
        parts: uint8_t,
        reason: uint8_t,
    ) -> ::core::ffi::c_int;
    unsafe fn matocsserv_send_replicatechunk_recover(
        e: *mut ::core::ffi::c_void,
        chunkid: uint64_t,
        ecid: uint8_t,
        version: uint32_t,
        parts: uint8_t,
        survivors: *mut *mut ::core::ffi::c_void,
        survivorecids: *mut uint8_t,
        reason: uint8_t,
    ) -> ::core::ffi::c_int;
    unsafe fn matocsserv_send_replicatechunk_join(
        e: *mut ::core::ffi::c_void,
        chunkid: uint64_t,
        ecid: uint8_t,
        version: uint32_t,
        parts: uint8_t,
        survivors: *mut *mut ::core::ffi::c_void,
        survivorecids: *mut uint8_t,
        reason: uint8_t,
    ) -> ::core::ffi::c_int;
    unsafe fn matocsserv_send_deletechunk(
        e: *mut ::core::ffi::c_void,
        chunkid: uint64_t,
        ecid: uint8_t,
        version: uint32_t,
        reason: uint8_t,
    ) -> ::core::ffi::c_int;
    unsafe fn matocsserv_send_createchunk(
        e: *mut ::core::ffi::c_void,
        chunkid: uint64_t,
        ecid: uint8_t,
        version: uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn matocsserv_send_setchunkversion(
        e: *mut ::core::ffi::c_void,
        chunkid: uint64_t,
        ecid: uint8_t,
        version: uint32_t,
        oldversion: uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn matocsserv_send_duplicatechunk(
        e: *mut ::core::ffi::c_void,
        chunkid: uint64_t,
        ecid: uint8_t,
        version: uint32_t,
        oldchunkid: uint64_t,
        oldecid: uint8_t,
        oldversion: uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn matocsserv_send_truncatechunk(
        e: *mut ::core::ffi::c_void,
        chunkid: uint64_t,
        ecid: uint8_t,
        length: uint32_t,
        version: uint32_t,
        oldversion: uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn matocsserv_send_duptruncchunk(
        e: *mut ::core::ffi::c_void,
        chunkid: uint64_t,
        ecid: uint8_t,
        version: uint32_t,
        oldchunkid: uint64_t,
        oldecid: uint8_t,
        oldversion: uint32_t,
        length: uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn matocsserv_send_localsplitchunk(
        e: *mut ::core::ffi::c_void,
        chunkid: uint64_t,
        version: uint32_t,
        missingmask: uint32_t,
        parts: uint8_t,
        reason: uint8_t,
    ) -> ::core::ffi::c_int;
    unsafe fn matocsserv_can_split_chunks(
        e: *mut ::core::ffi::c_void,
        ecmode: uint8_t,
    ) -> ::core::ffi::c_int;
    unsafe fn matocsserv_broadcast_regfirst_chunk(chunkid: uint64_t);
    unsafe fn matocsserv_isvalid(e: *mut ::core::ffi::c_void) -> uint8_t;
    unsafe fn matocsserv_disconnection_finished(e: *mut ::core::ffi::c_void);
    unsafe fn matoclserv_chunk_unlocked(chunkid: uint64_t, cptr: *mut ::core::ffi::c_void);
    unsafe fn matoclserv_chunk_status(chunkid: uint64_t, status: uint8_t);
    unsafe fn matoclserv_fuse_invalidate_chunk_cache();
    unsafe fn changelog(format: *const ::core::ffi::c_char, ...);
    unsafe fn csdb_have_all_servers() -> uint8_t;
    unsafe fn csdb_stop_chunk_jobs() -> uint8_t;
    unsafe fn rndu32() -> uint32_t;
    unsafe fn rndu32_ranged(range: uint32_t) -> uint32_t;
    unsafe fn topology_get_rackid(ip: uint32_t) -> uint32_t;
    unsafe fn topology_distance(ip1: uint32_t, ip2: uint32_t) -> uint8_t;
    unsafe fn chunk_delay_protect(chunkid: uint64_t);
    unsafe fn chunk_delay_is_protected(chunkid: uint64_t) -> uint8_t;
    unsafe fn chunk_delay_init();
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn mmap(
        __addr: *mut ::core::ffi::c_void,
        __len: size_t,
        __prot: ::core::ffi::c_int,
        __flags: ::core::ffi::c_int,
        __fd: ::core::ffi::c_int,
        __offset: __off64_t,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn munmap(__addr: *mut ::core::ffi::c_void, __len: size_t) -> ::core::ffi::c_int;
    unsafe fn monotonic_useconds() -> uint64_t;
}
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type size_t = usize;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
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
pub type bio = _bio;
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
pub type chunkfloop = ::core::ffi::c_uint;
pub const CHUNK_FLOOP_OK: chunkfloop = 7;
pub const CHUNK_FLOOP_UNDERGOAL: chunkfloop = 6;
pub const CHUNK_FLOOP_MISSING_PARTIALEC: chunkfloop = 5;
pub const CHUNK_FLOOP_MISSING_WRONGVERSION: chunkfloop = 4;
pub const CHUNK_FLOOP_MISSING_INVALID: chunkfloop = 3;
pub const CHUNK_FLOOP_MISSING_NOCOPY: chunkfloop = 2;
pub const CHUNK_FLOOP_DELETED: chunkfloop = 1;
pub const CHUNK_FLOOP_NOTFOUND: chunkfloop = 0;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct chunk {
    pub chunkid: uint64_t,
    #[bitfield(name = "needverincrease", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "allowreadzeros", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "version", ty = "::core::ffi::c_uint", bits = "2..=31")]
    pub needverincrease_allowreadzeros_version: [u8; 4],
    pub sclassid: uint8_t,
    #[bitfield(name = "storage_mode", ty = "::core::ffi::c_uint", bits = "0..=3")]
    #[bitfield(name = "all_gequiv", ty = "::core::ffi::c_uint", bits = "4..=7")]
    #[bitfield(name = "reg_gequiv", ty = "::core::ffi::c_uint", bits = "8..=11")]
    #[bitfield(name = "unused", ty = "::core::ffi::c_uint", bits = "12..=15")]
    #[bitfield(name = "ondangerlist", ty = "::core::ffi::c_uint", bits = "16..=16")]
    #[bitfield(name = "interrupted", ty = "::core::ffi::c_uint", bits = "17..=17")]
    #[bitfield(name = "writeinprogress", ty = "::core::ffi::c_uint", bits = "18..=18")]
    #[bitfield(name = "flags", ty = "::core::ffi::c_uint", bits = "19..=20")]
    #[bitfield(name = "operation", ty = "::core::ffi::c_uint", bits = "21..=23")]
    pub storage_mode_all_gequiv_reg_gequiv_unused_ondangerlist_interrupted_writeinprogress_flags_operation:
        [u8; 3],
    pub fhead: uint32_t,
    pub lockedto: uint32_t,
    pub slisthead: *mut slist,
    pub next: *mut chunk,
}
pub type slist = _slist;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _slist {
    pub csid: uint16_t,
    pub valid: uint8_t,
    pub ecid: uint8_t,
    pub version: uint32_t,
    pub next: *mut _slist,
}
pub type csdata = _csdata;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _csdata {
    pub ptr: *mut ::core::ffi::c_void,
    pub opchunks: *mut csopchunk,
    pub valid: uint8_t,
    #[bitfield(name = "registered", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "mfr_state", ty = "::core::ffi::c_uint", bits = "1..=3")]
    pub registered_mfr_state: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub next: uint32_t,
    pub prev: uint32_t,
}
pub type csopchunk = _csopchunk;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _csopchunk {
    pub chunkid: uint64_t,
    pub status: uint8_t,
    pub next: *mut _csopchunk,
}
pub const TDWVER: C2Rust_Unnamed_4 = 7;
pub const WVER: C2Rust_Unnamed_4 = 4;
pub const DEL: C2Rust_Unnamed_4 = 1;
pub const INVALID: C2Rust_Unnamed_4 = 0;
pub type flist = _flist;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _flist {
    #[bitfield(name = "fcount", ty = "::core::ffi::c_uint", bits = "0..=23")]
    pub fcount: [u8; 3],
    pub sclassid: uint8_t,
    pub nexti: uint32_t,
}
pub const NONE: C2Rust_Unnamed_3 = 0;
pub type chunk_bucket = _chunk_bucket;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _chunk_bucket {
    pub bucket: [chunk; 250000],
    pub firstfree: uint32_t,
    pub next: *mut _chunk_bucket,
}
pub const BUSY: C2Rust_Unnamed_4 = 2;
pub type slist_bucket = _slist_bucket;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _slist_bucket {
    pub bucket: [slist; 625000],
    pub firstfree: uint32_t,
    pub next: *mut _slist_bucket,
}
pub const DUPLICATE: C2Rust_Unnamed_3 = 3;
pub const VALID: C2Rust_Unnamed_4 = 3;
pub const TDVALID: C2Rust_Unnamed_4 = 6;
pub type io_ready_chunk = _io_ready_chunk;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _io_ready_chunk {
    pub c: *mut chunk,
    pub next: *mut _io_ready_chunk,
}
pub type io_ready_chunk_bucket = _io_ready_chunk_bucket;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _io_ready_chunk_bucket {
    pub bucket: [io_ready_chunk; 625000],
    pub firstfree: uint32_t,
    pub next: *mut _io_ready_chunk_bucket,
}
pub const TDBUSY: C2Rust_Unnamed_4 = 5;
pub const JOBS_CHUNK: C2Rust_Unnamed_2 = 4;
pub const REBALANCE_NOT_DONE_OR_NOT_NEEDED: C2Rust_Unnamed_5 = 93;
pub const REBALANCE_DONE: C2Rust_Unnamed_5 = 92;
pub type loop_info = _loop_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _loop_info {
    pub fixed: uint32_t,
    pub forcekeep: uint32_t,
    pub delete_invalid: uint32_t,
    pub delete_no_longer_needed: uint32_t,
    pub delete_wrong_version: uint32_t,
    pub delete_duplicated_ecpart: uint32_t,
    pub delete_excess_ecpart: uint32_t,
    pub delete_excess_copy: uint32_t,
    pub delete_diskclean_ecpart: uint32_t,
    pub delete_diskclean_copy: uint32_t,
    pub replicate_dupserver_ecpart: uint32_t,
    pub replicate_needed_ecpart: uint32_t,
    pub replicate_needed_copy: uint32_t,
    pub replicate_wronglabels_ecpart: uint32_t,
    pub replicate_wronglabels_copy: uint32_t,
    pub split_copy_into_ecparts: uint32_t,
    pub join_ecparts_into_copy: uint32_t,
    pub recover_ecpart: uint32_t,
    pub calculate_ecchksum: uint32_t,
    pub locked_unused: uint32_t,
    pub locked_used: uint32_t,
    pub replicate_rebalance: uint32_t,
}
pub const REPL_EC_REBALANCE: C2Rust_Unnamed = 8;
pub const REPL_COPY_REBALANCE: C2Rust_Unnamed = 4;
pub const SIMPLE: C2Rust_Unnamed_1 = 0;
pub type replock = _replock;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _replock {
    pub chunkid: uint64_t,
    pub lockedto: uint32_t,
    pub next: *mut _replock,
}
pub type replock_bucket = _replock_bucket;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _replock_bucket {
    pub bucket: [replock; 416666],
    pub firstfree: uint32_t,
    pub next: *mut _replock_bucket,
}
pub const REPLICATE: C2Rust_Unnamed_3 = 6;
pub const JOIN: C2Rust_Unnamed_1 = 3;
pub const RECOVER: C2Rust_Unnamed_1 = 2;
pub const SPLIT: C2Rust_Unnamed_1 = 1;
pub const REBALANCE_BLOCKED_BY_TOO_MANY_FAILS: C2Rust_Unnamed_5 = 91;
pub const REBALANCE_BLOCKED_BY_PRIORITY_QUEUES: C2Rust_Unnamed_5 = 90;
pub type chq_element = _chq_element;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _chq_element {
    pub c: *mut chunk,
    pub priority: uint8_t,
    pub hnext: *mut _chq_element,
    pub hprev: *mut *mut _chq_element,
    pub qnext: *mut _chq_element,
    pub qprev: *mut *mut _chq_element,
}
pub const CANT_REBALANCE_ON_EXTRA_CALL: C2Rust_Unnamed_5 = 89;
pub const CANT_REPLICATE_UNDERGOAL_COPY: C2Rust_Unnamed_5 = 88;
pub const CANT_REPLICATE_WRONG_LABELED_COPY: C2Rust_Unnamed_5 = 87;
pub type chunk_queue_bucket = _chunk_queue_bucket;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _chunk_queue_bucket {
    pub bucket: [chq_element; 208333],
    pub firstfree: uint32_t,
    pub next: *mut _chunk_queue_bucket,
}
pub const REPLICATED_UNDERGOAL_COPY: C2Rust_Unnamed_5 = 86;
pub const REPL_COPY_WRONGLABEL: C2Rust_Unnamed = 3;
pub const REPL_COPY_UNDERGOAL: C2Rust_Unnamed = 2;
pub const REPL_COPY_ENDANGERED: C2Rust_Unnamed = 1;
pub const REPL_COPY_IO: C2Rust_Unnamed = 0;
pub const REPLICATED_WRONG_LABELED_COPY_GOOD: C2Rust_Unnamed_5 = 85;
pub const REPLICATED_WRONG_LABELED_COPY_BUSY: C2Rust_Unnamed_5 = 84;
pub const REPLICATED_WRONG_LABELED_COPY_UNMATCHED: C2Rust_Unnamed_5 = 83;
pub const DELETED_MFR_COPY_TO_MAKE_SPACE: C2Rust_Unnamed_5 = 82;
pub const OP_DEL_OVERGOAL: C2Rust_Unnamed_0 = 3;
pub const CANT_DELETE_EXTRA_COPIES: C2Rust_Unnamed_5 = 81;
pub const DELETED_EXTRA_COPIES: C2Rust_Unnamed_5 = 80;
pub const REPLICATED_WRONG_LABELED_PARTS: C2Rust_Unnamed_5 = 79;
pub const REPLICATED_WRONG_LABELED_CHKSUM_PART: C2Rust_Unnamed_5 = 77;
pub const REPLICATED_WRONG_LABELED_DATA_PART: C2Rust_Unnamed_5 = 76;
pub const CANT_REPLICATE_WRONG_LABELED_PART: C2Rust_Unnamed_5 = 78;
pub const ERROR_REPLICATING_WRONG_LABELED_CHKSUM_PART: C2Rust_Unnamed_5 = 75;
pub const REPL_EC_WRONGLABEL: C2Rust_Unnamed = 7;
pub const ERROR_REPLICATING_WRONG_LABELED_DATA_PART: C2Rust_Unnamed_5 = 74;
pub const DELETED_EXTRA_EC_PARTS: C2Rust_Unnamed_5 = 72;
pub const CANT_DELETE_EXTRA_EC_PARTS: C2Rust_Unnamed_5 = 73;
pub const DELETED_PARTS_IN_COPY_MODE: C2Rust_Unnamed_5 = 70;
pub const CANT_DELETE_PARTS_IN_COPY_MODE: C2Rust_Unnamed_5 = 71;
pub const DELETED_COPIES_IN_EC_MODE: C2Rust_Unnamed_5 = 68;
pub const CANT_DELETE_COPIES_IN_EC_MODE: C2Rust_Unnamed_5 = 69;
pub const CANT_CREATE_COPY_FROM_PARTS: C2Rust_Unnamed_5 = 67;
pub const CREATED_COPY_FROM_EC8_PARTS: C2Rust_Unnamed_5 = 61;
pub const CREATED_COPY_FROM_EC4_PARTS: C2Rust_Unnamed_5 = 62;
pub const CREATED_LABELED_COPY_FROM_EC8_PARTS: C2Rust_Unnamed_5 = 59;
pub const CREATED_LABELED_COPY_FROM_EC4_PARTS: C2Rust_Unnamed_5 = 60;
pub const ERROR_CREATING_COPY_FROM_EC8_PARTS: C2Rust_Unnamed_5 = 65;
pub const JOIN_EC_GENERIC: C2Rust_Unnamed = 14;
pub const JOIN_EC_NOSERVERS: C2Rust_Unnamed = 13;
pub const JOIN_EC_CHANGE: C2Rust_Unnamed = 12;
pub const JOIN_EC_IO: C2Rust_Unnamed = 11;
pub const ERROR_CREATING_COPY_FROM_EC4_PARTS: C2Rust_Unnamed_5 = 66;
pub const ERROR_CREATING_LABELED_COPY_FROM_EC8_PARTS: C2Rust_Unnamed_5 = 63;
pub const ERROR_CREATING_LABELED_COPY_FROM_EC4_PARTS: C2Rust_Unnamed_5 = 64;
pub const CANT_CREATE_PART_FROM_COPY: C2Rust_Unnamed_5 = 58;
pub const CREATED_MISSING_EC4_PART_FROM_COPY: C2Rust_Unnamed_5 = 55;
pub const CREATED_MISSING_EC8_PART_FROM_COPY: C2Rust_Unnamed_5 = 54;
pub const ERROR_CREATING_MISSING_EC4_PART_FROM_COPY: C2Rust_Unnamed_5 = 57;
pub const SPLIT_EC_GENERIC: C2Rust_Unnamed = 15;
pub const ERROR_CREATING_MISSING_EC8_PART_FROM_COPY: C2Rust_Unnamed_5 = 56;
pub const CANT_CREATE_MISSING_EC4_CHKSUM_PART: C2Rust_Unnamed_5 = 53;
pub const CANT_CREATE_MISSING_EC8_CHKSUM_PART: C2Rust_Unnamed_5 = 48;
pub const CREATED_MISSING_EC4_CHKSUM_PART: C2Rust_Unnamed_5 = 51;
pub const CREATED_MISSING_EC8_CHKSUM_PART: C2Rust_Unnamed_5 = 46;
pub const ERROR_CREATING_MISSING_EC4_CHKSUM_PART: C2Rust_Unnamed_5 = 52;
pub const REPL_EC_UNDERGOAL: C2Rust_Unnamed = 6;
pub const REPL_EC_ENDANGERED: C2Rust_Unnamed = 5;
pub const ERROR_REPLICATING_MISSING_EC4_CHKSUM_PART_FROM_MFR: C2Rust_Unnamed_5 = 49;
pub const ERROR_CREATING_MISSING_EC8_CHKSUM_PART: C2Rust_Unnamed_5 = 47;
pub const ERROR_REPLICATING_MISSING_EC8_CHKSUM_PART_FROM_MFR: C2Rust_Unnamed_5 = 44;
pub const CREATED_MISSING_PARTS_FROM_COPY: C2Rust_Unnamed_5 = 43;
pub const LOCALSPLIT: C2Rust_Unnamed_3 = 7;
pub const ERROR_CREATING_MISSING_PARTS_FROM_COPY: C2Rust_Unnamed_5 = 42;
pub const LOCALSPLIT_TO_EC4: C2Rust_Unnamed = 9;
pub const LOCALSPLIT_TO_EC8: C2Rust_Unnamed = 10;
pub const CANT_RECOVER_MISSING_EC4_DATA_PART: C2Rust_Unnamed_5 = 41;
pub const RECOVERED_MISSING_EC4_DATA_PART: C2Rust_Unnamed_5 = 39;
pub const ERROR_RECOVERING_MISSING_EC4_DATA_PART: C2Rust_Unnamed_5 = 40;
pub const RECOVER_IO: C2Rust_Unnamed = 16;
pub const ERROR_REPLICATING_MISSING_EC4_DATA_PART_FROM_MFR: C2Rust_Unnamed_5 = 37;
pub const CANT_RECOVER_MISSING_EC8_DATA_PART: C2Rust_Unnamed_5 = 36;
pub const RECOVERED_MISSING_EC8_DATA_PART: C2Rust_Unnamed_5 = 34;
pub const ERROR_RECOVERING_MISSING_EC8_DATA_PART: C2Rust_Unnamed_5 = 35;
pub const ERROR_REPLICATING_MISSING_EC8_DATA_PART_FROM_MFR: C2Rust_Unnamed_5 = 32;
pub const REPLICATED_MFR_CHKSUM_PART: C2Rust_Unnamed_5 = 27;
pub const CANT_REPLICATE_MFR_CHKSUM_PART: C2Rust_Unnamed_5 = 31;
pub const REPLICATED_MFR_DATA_PART: C2Rust_Unnamed_5 = 26;
pub const CANT_REPLICATE_MFR_DATA_PART: C2Rust_Unnamed_5 = 29;
pub const ERROR_REPLICATING_MFR_CHKSUM_PART: C2Rust_Unnamed_5 = 30;
pub const ERROR_REPLICATING_MFR_DATA_PART: C2Rust_Unnamed_5 = 28;
pub const CANT_FIND_VALID_REPLICATION_SOURCE: C2Rust_Unnamed_5 = 25;
pub const FOUND_DUPLICATED_EC_PARTS: C2Rust_Unnamed_5 = 21;
pub const CANT_FIX_PARTS_ON_THE_SAME_SERVER: C2Rust_Unnamed_5 = 20;
pub const REPLICATED_DUPLICATED_PART: C2Rust_Unnamed_5 = 18;
pub const ERROR_REPLICATING_DUPLICATED_PART: C2Rust_Unnamed_5 = 19;
pub const DELETED_DUPLICATED_PART: C2Rust_Unnamed_5 = 17;
pub const DELETED_PART_ON_THE_SAME_SERVER: C2Rust_Unnamed_5 = 16;
pub const CANT_DELETE_MFR_PART_TO_MAKE_SPACE: C2Rust_Unnamed_5 = 24;
pub const DELETED_MFR_PART_TO_MAKE_SPACE: C2Rust_Unnamed_5 = 23;
pub const BLOCKED_BY_CHUNKSERVER_IN_MAINTENANCE_MODE: C2Rust_Unnamed_5 = 15;
pub const BLOCKED_BY_HIGHER_PRIORITY_QUEUE: C2Rust_Unnamed_5 = 14;
pub const DELETED_UNUSED_CHUNK: C2Rust_Unnamed_5 = 13;
pub const OP_DEL_NOTUSED: C2Rust_Unnamed_0 = 2;
pub const UNEXPECTED_BUSY_COPY_OR_PART: C2Rust_Unnamed_5 = 12;
pub const CHUNK_IS_BEING_MODIFIED: C2Rust_Unnamed_5 = 11;
pub const FOUND_NOT_FINISHED_REPLICATION: C2Rust_Unnamed_5 = 10;
pub const DELETED_SOME_INVALID_COPIES_OR_PARTS: C2Rust_Unnamed_5 = 9;
pub const DELETED_INVALID_COPY_OR_PART_TO_MAKE_SPACE: C2Rust_Unnamed_5 = 8;
pub const OP_DEL_INVALID: C2Rust_Unnamed_0 = 1;
pub const NO_COPIES_AND_PARTS: C2Rust_Unnamed_5 = 7;
pub const NO_VALID_COPIES_AND_PARTS: C2Rust_Unnamed_5 = 6;
pub const FIXED_EC4_VERSION: C2Rust_Unnamed_5 = 5;
pub const FIXED_EC8_VERSION: C2Rust_Unnamed_5 = 4;
pub const FIXED_COPY_VERSION: C2Rust_Unnamed_5 = 3;
pub const DELETED_UNEXPECTED_BUSY_COPY_OR_PART: C2Rust_Unnamed_5 = 2;
pub const REPL_IN_PROGRESS: C2Rust_Unnamed_6 = 3;
pub const INTERNAL_ERROR: C2Rust_Unnamed_5 = 0;
pub const DISCONNECTED_CHUNKSERVER: C2Rust_Unnamed_5 = 1;
pub const SET_VERSION: C2Rust_Unnamed_3 = 2;
pub const TRUNCATE: C2Rust_Unnamed_3 = 4;
pub type discserv = _discserv;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _discserv {
    pub csid: uint16_t,
    pub next: *mut _discserv,
}
pub const JOBS_TERM: C2Rust_Unnamed_2 = 3;
pub const JOBS_EVERYTICK: C2Rust_Unnamed_2 = 2;
pub const JOBS_EVERYLOOP: C2Rust_Unnamed_2 = 1;
pub const JOBS_INIT: C2Rust_Unnamed_2 = 0;
pub const CREATE: C2Rust_Unnamed_3 = 1;
pub const DUPTRUNC: C2Rust_Unnamed_3 = 5;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct locsort {
    pub ip: uint32_t,
    pub port: uint16_t,
    pub csver: uint32_t,
    pub labelmask: uint32_t,
    pub dist: uint32_t,
    pub rnd: uint32_t,
}
pub const WAS_IN_PROGRESS: C2Rust_Unnamed_6 = 4;
pub const CAN_BE_REMOVED: C2Rust_Unnamed_6 = 2;
pub const UNKNOWN_SOFT: C2Rust_Unnamed_6 = 1;
pub const UNKNOWN_HARD: C2Rust_Unnamed_6 = 0;
pub const CBF_NO: canbefulfilled = 0;
pub const CBF_NOSPACE: canbefulfilled = 1;
pub const CBF_OVERLOADED: canbefulfilled = 2;
pub const CBF_YES: canbefulfilled = 3;
pub const CBF_ECKEEP: canbefulfilled = 4;
pub const JOB_EXIT_REASONS: C2Rust_Unnamed_5 = 94;
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const REPL_REASONS: C2Rust_Unnamed = 17;
pub type C2Rust_Unnamed_0 = ::core::ffi::c_uint;
pub const OP_REASONS: C2Rust_Unnamed_0 = 4;
pub const OP_GENERIC_IO: C2Rust_Unnamed_0 = 0;
pub type C2Rust_Unnamed_1 = ::core::ffi::c_uint;
pub type C2Rust_Unnamed_2 = ::core::ffi::c_uint;
pub type C2Rust_Unnamed_3 = ::core::ffi::c_uint;
pub type C2Rust_Unnamed_4 = ::core::ffi::c_uint;
pub type C2Rust_Unnamed_5 = ::core::ffi::c_uint;
pub const CANT_FIND_EC4_ENCODE_MATRIX: C2Rust_Unnamed_5 = 50;
pub const CANT_FIND_EC8_ENCODE_MATRIX: C2Rust_Unnamed_5 = 45;
pub const CANT_FIND_EC4_DECODE_MATRIX: C2Rust_Unnamed_5 = 38;
pub const CANT_FIND_EC8_DECODE_MATRIX: C2Rust_Unnamed_5 = 33;
pub const DELETED_INVALID_PART_TO_MAKE_SPACE: C2Rust_Unnamed_5 = 22;
pub type C2Rust_Unnamed_6 = ::core::ffi::c_uint;
pub type canbefulfilled = ::core::ffi::c_uint;
pub const CHUNKHASH_MOVEFACTOR: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MAXSCLASS: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_CHUNKLOST: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const MFS_ERROR_LOCKED: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const MFS_ERROR_NOCHUNKSERVERS: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const MFS_ERROR_NOCHUNK: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const MFS_ERROR_CHUNKBUSY: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const MFS_ERROR_NOTDONE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MFS_ERROR_WRONGVERSION: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const MFS_ERROR_CHUNKEXIST: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const MFS_ERROR_NOSPACE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const MFS_ERROR_MISMATCH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MFS_ERROR_ACTIVE: ::core::ffi::c_int = 42 as ::core::ffi::c_int;
pub const MFS_ERROR_CSNOTPRESENT: ::core::ffi::c_int = 43 as ::core::ffi::c_int;
pub const MFS_ERROR_EAGAIN: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
pub const MFS_ERROR_ETIMEDOUT: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const UNIQ_MASK_IP: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
    << 1 as ::core::ffi::c_int + 'Z' as ::core::ffi::c_int - 'A' as ::core::ffi::c_int;
pub const UNIQ_MASK_RACK: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
    << 2 as ::core::ffi::c_int + 'Z' as ::core::ffi::c_int - 'A' as ::core::ffi::c_int;
pub const CHECK_VALID: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CHECK_MARKEDFORREMOVAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CHECK_WRONGVERSION: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CHECK_WV_AND_MFR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CHECK_INVALID: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LABELS_MODE_LOOSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LABELS_MODE_STD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LABELS_MODE_STRICT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFRSTATUS_VALIDATING: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFRSTATUS_INPROGRESS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFRSTATUS_READY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TRANSFERRING_LOST_CHUNKS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TRANSFERRING_NEW_CHUNKS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CHUNKSERVERS_DISCONNECTING: ::core::ffi::c_int = TRANSFERRING_LOST_CHUNKS;
pub const CHUNKSERVERS_CONNECTING: ::core::ffi::c_int = TRANSFERRING_NEW_CHUNKS;
pub const MAXCSCOUNT: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
pub const CHUNK_OP_DELETE_TRY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CHUNK_OP_REPLICATE_TRY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CHUNK_OP_CREATE_TRY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CHUNK_OP_CHANGE_TRY: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CHUNK_OP_SPLIT_TRY: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CHUNK_OP_DELETE_OK: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const CHUNK_OP_REPLICATE_OK: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const CHUNK_OP_CREATE_OK: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const CHUNK_OP_CHANGE_OK: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const CHUNK_OP_SPLIT_OK: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const CHUNK_OP_DELETE_ERR: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const CHUNK_OP_REPLICATE_ERR: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const CHUNK_OP_CREATE_ERR: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const CHUNK_OP_CHANGE_ERR: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const CHUNK_OP_SPLIT_ERR: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const CHUNK_STATS_CNT: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const CSSTATE_LIMIT_REACHED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CSSTATE_OVERLOADED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CSSTATE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TOPOLOGY_DIST_SAME_RACKID: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TOPOLOGY_DIST_MAX: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
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
unsafe extern "C" fn put24bit(mut ptr: *mut *mut uint8_t, mut val: uint32_t) {
    unsafe {
        val = val.swap_bytes() as uint32_t;
        memcpy(
            *ptr as *mut ::core::ffi::c_void,
            (&raw mut val as *mut uint8_t).offset(1 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_void,
            3 as size_t,
        );
        *ptr = (*ptr).offset(3 as ::core::ffi::c_int as isize);
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
unsafe extern "C" fn get24bit(mut ptr: *mut *const uint8_t) -> uint32_t {
    unsafe {
        let mut t32: uint32_t = 0;
        memset(
            &raw mut t32 as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            1 as size_t,
        );
        memcpy(
            (&raw mut t32 as *mut uint8_t).offset(1 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_void,
            *ptr as *const ::core::ffi::c_void,
            3 as size_t,
        );
        *ptr = (*ptr).offset(3 as ::core::ffi::c_int as isize);
        return t32.swap_bytes();
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
#[inline]
unsafe extern "C" fn hash32(mut key: uint32_t) -> uint32_t {
    unsafe {
        key = (!key).wrapping_add(key << 15 as ::core::ffi::c_int);
        key = key ^ key >> 12 as ::core::ffi::c_int;
        key = key.wrapping_add(key << 2 as ::core::ffi::c_int);
        key = key ^ key >> 4 as ::core::ffi::c_int;
        key = key.wrapping_mul(2057 as uint32_t);
        key = key ^ key >> 16 as ::core::ffi::c_int;
        return key;
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
pub const PROT_READ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PROT_WRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MAP_PRIVATE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MAP_ANONYMOUS: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const MAP_ANON: ::core::ffi::c_int = MAP_ANONYMOUS;
pub const MINLOOPTIME: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const MAXLOOPTIME: ::core::ffi::c_int = 7200 as ::core::ffi::c_int;
pub const MAXCPS: ::core::ffi::c_int = 10000000 as ::core::ffi::c_int;
pub const MINCPS: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
pub const HASHTAB_LOBITS: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const HASHTAB_HISIZE: ::core::ffi::c_uint = 0x80000000 as ::core::ffi::c_uint >> HASHTAB_LOBITS;
pub const HASHTAB_LOSIZE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << HASHTAB_LOBITS;
pub const HASHTAB_MASK: ::core::ffi::c_int = HASHTAB_LOSIZE - 1 as ::core::ffi::c_int;
static mut opstr: [*const ::core::ffi::c_char; 9] = [
    b"NONE\0".as_ptr() as *const ::core::ffi::c_char,
    b"CREATE\0".as_ptr() as *const ::core::ffi::c_char,
    b"SET_VERSION\0".as_ptr() as *const ::core::ffi::c_char,
    b"DUPLICATE\0".as_ptr() as *const ::core::ffi::c_char,
    b"TRUNCATE\0".as_ptr() as *const ::core::ffi::c_char,
    b"DUPLICATE+TRUNCATE\0".as_ptr() as *const ::core::ffi::c_char,
    b"REPLICATE\0".as_ptr() as *const ::core::ffi::c_char,
    b"LOCALSPLIT\0".as_ptr() as *const ::core::ffi::c_char,
    b"???\0".as_ptr() as *const ::core::ffi::c_char,
];
unsafe extern "C" fn op_to_str(mut op: uint8_t) -> *const ::core::ffi::c_char {
    unsafe {
        if (op as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
            return opstr[op as usize];
        } else {
            return opstr[8 as usize];
        };
    }
}
static mut validstr: [*const ::core::ffi::c_char; 9] = [
    b"INVALID\0".as_ptr() as *const ::core::ffi::c_char,
    b"DEL\0".as_ptr() as *const ::core::ffi::c_char,
    b"BUSY\0".as_ptr() as *const ::core::ffi::c_char,
    b"VALID\0".as_ptr() as *const ::core::ffi::c_char,
    b"WVER\0".as_ptr() as *const ::core::ffi::c_char,
    b"TDBUSY\0".as_ptr() as *const ::core::ffi::c_char,
    b"TDVALID\0".as_ptr() as *const ::core::ffi::c_char,
    b"TDWVER\0".as_ptr() as *const ::core::ffi::c_char,
    b"???\0".as_ptr() as *const ::core::ffi::c_char,
];
unsafe extern "C" fn valid_to_str(mut valid: uint8_t) -> *const ::core::ffi::c_char {
    unsafe {
        if (valid as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
            return validstr[valid as usize];
        } else {
            return validstr[8 as usize];
        };
    }
}
static mut job_exit_reason_str: [*const ::core::ffi::c_char; 95] = [
    b"INTERNAL_ERROR\0".as_ptr() as *const ::core::ffi::c_char,
    b"DISCONNECTED_CHUNKSERVER\0".as_ptr() as *const ::core::ffi::c_char,
    b"DELETED_UNEXPECTED_BUSY_COPY_OR_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"FIXED_COPY_VERSION\0".as_ptr() as *const ::core::ffi::c_char,
    b"FIXED_EC8_VERSION\0".as_ptr() as *const ::core::ffi::c_char,
    b"FIXED_EC4_VERSION\0".as_ptr() as *const ::core::ffi::c_char,
    b"NO_VALID_COPIES_AND_PARTS\0".as_ptr() as *const ::core::ffi::c_char,
    b"NO_COPIES_AND_PARTS\0".as_ptr() as *const ::core::ffi::c_char,
    b"DELETED_INVALID_COPY_OR_PART_TO_MAKE_SPACE\0".as_ptr() as *const ::core::ffi::c_char,
    b"DELETED_SOME_INVALID_COPIES_OR_PARTS\0".as_ptr() as *const ::core::ffi::c_char,
    b"FOUND_NOT_FINISHED_REPLICATION\0".as_ptr() as *const ::core::ffi::c_char,
    b"CHUNK_IS_BEING_MODIFIED\0".as_ptr() as *const ::core::ffi::c_char,
    b"UNEXPECTED_BUSY_COPY_OR_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"DELETED_UNUSED_CHUNK\0".as_ptr() as *const ::core::ffi::c_char,
    b"BLOCKED_BY_HIGHER_PRIORITY_QUEUE\0".as_ptr() as *const ::core::ffi::c_char,
    b"BLOCKED_BY_CHUNKSERVER_IN_MAINTENANCE_MODE\0".as_ptr() as *const ::core::ffi::c_char,
    b"DELETED_PART_ON_THE_SAME_SERVER\0".as_ptr() as *const ::core::ffi::c_char,
    b"DELETED_DUPLICATED_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"REPLICATED_DUPLICATED_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"ERROR_REPLICATING_DUPLICATED_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"CANT_FIX_PARTS_ON_THE_SAME_SERVER\0".as_ptr() as *const ::core::ffi::c_char,
    b"FOUND_DUPLICATED_EC_PARTS\0".as_ptr() as *const ::core::ffi::c_char,
    b"DELETED_INVALID_PART_TO_MAKE_SPACE\0".as_ptr() as *const ::core::ffi::c_char,
    b"DELETED_MFR_PART_TO_MAKE_SPACE\0".as_ptr() as *const ::core::ffi::c_char,
    b"CANT_DELETE_MFR_PART_TO_MAKE_SPACE\0".as_ptr() as *const ::core::ffi::c_char,
    b"CANT_FIND_VALID_REPLICATION_SOURCE\0".as_ptr() as *const ::core::ffi::c_char,
    b"REPLICATED_MFR_DATA_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"REPLICATED_MFR_CHKSUM_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"ERROR_REPLICATING_MFR_DATA_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"CANT_REPLICATE_MFR_DATA_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"ERROR_REPLICATE_MFR_CHKSUM_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"CANT_REPLICATE_MFR_CHKSUM_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"ERROR_REPLICATING_MISSING_EC8_DATA_PART_FROM_MFR\0".as_ptr() as *const ::core::ffi::c_char,
    b"CANT_FIND_EC8_DECODE_MATRIX\0".as_ptr() as *const ::core::ffi::c_char,
    b"RECOVERED_MISSING_EC8_DATA_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"ERROR_RECOVERING_MISSING_EC8_DATA_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"CANT_RECOVER_MISSING_EC8_DATA_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"ERROR_REPLICATING_MISSING_EC4_DATA_PART_FROM_MFR\0".as_ptr() as *const ::core::ffi::c_char,
    b"CANT_FIND_EC4_DECODE_MATRIX\0".as_ptr() as *const ::core::ffi::c_char,
    b"RECOVERED_MISSING_EC4_DATA_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"ERROR_RECOVERING_MISSING_EC4_DATA_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"CANT_RECOVER_MISSING_EC4_DATA_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"ERROR_CREATING_MISSING_PARTS_FROM_COPY\0".as_ptr() as *const ::core::ffi::c_char,
    b"CREATED_MISSING_PARTS_FROM_COPY\0".as_ptr() as *const ::core::ffi::c_char,
    b"ERROR_REPLICATING_MISSING_EC8_CHKSUM_PART_FROM_MFR\0".as_ptr() as *const ::core::ffi::c_char,
    b"CANT_FIND_EC8_ENCODE_MATRIX\0".as_ptr() as *const ::core::ffi::c_char,
    b"CREATED_MISSING_EC8_CHKSUM_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"ERROR_CREATING_MISSING_EC8_CHKSUM_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"CANT_CREATE_MISSING_EC8_CHKSUM_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"ERROR_REPLICATING_MISSING_EC4_CHKSUM_PART_FROM_MFR\0".as_ptr() as *const ::core::ffi::c_char,
    b"CANT_FIND_EC4_ENCODE_MATRIX\0".as_ptr() as *const ::core::ffi::c_char,
    b"CREATED_MISSING_EC4_CHKSUM_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"ERROR_CREATING_MISSING_EC4_CHKSUM_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"CANT_CREATE_MISSING_EC4_CHKSUM_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"CREATED_MISSING_EC8_PART_FROM_COPY\0".as_ptr() as *const ::core::ffi::c_char,
    b"CREATED_MISSING_EC4_PART_FROM_COPY\0".as_ptr() as *const ::core::ffi::c_char,
    b"ERROR_CREATING_MISSING_EC8_PART_FROM_COPY\0".as_ptr() as *const ::core::ffi::c_char,
    b"ERROR_CREATING_MISSING_EC4_PART_FROM_COPY\0".as_ptr() as *const ::core::ffi::c_char,
    b"CANT_CREATE_PART_FROM_COPY\0".as_ptr() as *const ::core::ffi::c_char,
    b"CREATED_LABELED_COPY_FROM_EC8_PARTS\0".as_ptr() as *const ::core::ffi::c_char,
    b"CREATED_LABELED_COPY_FROM_EC4_PARTS\0".as_ptr() as *const ::core::ffi::c_char,
    b"CREATED_COPY_FROM_EC8_PARTS\0".as_ptr() as *const ::core::ffi::c_char,
    b"CREATED_COPY_FROM_EC4_PARTS\0".as_ptr() as *const ::core::ffi::c_char,
    b"ERROR_CREATING_LABELED_COPY_FROM_EC8_PARTS\0".as_ptr() as *const ::core::ffi::c_char,
    b"ERROR_CREATING_LABELED_COPY_FROM_EC4_PARTS\0".as_ptr() as *const ::core::ffi::c_char,
    b"ERROR_CREATING_COPY_FROM_EC8_PARTS\0".as_ptr() as *const ::core::ffi::c_char,
    b"ERROR_CREATING_COPY_FROM_EC4_PARTS\0".as_ptr() as *const ::core::ffi::c_char,
    b"CANT_CREATE_COPY_FROM_PARTS\0".as_ptr() as *const ::core::ffi::c_char,
    b"DELETED_COPIES_IN_EC_MODE\0".as_ptr() as *const ::core::ffi::c_char,
    b"CANT_DELETE_COPIES_IN_EC_MODE\0".as_ptr() as *const ::core::ffi::c_char,
    b"DELETED_PARTS_IN_COPY_MODE\0".as_ptr() as *const ::core::ffi::c_char,
    b"CANT_DELETE_PARTS_IN_COPY_MODE\0".as_ptr() as *const ::core::ffi::c_char,
    b"DELETED_EXTRA_EC_PARTS\0".as_ptr() as *const ::core::ffi::c_char,
    b"CANT_DELETE_EXTRA_EC_PARTS\0".as_ptr() as *const ::core::ffi::c_char,
    b"ERROR_REPLICATING_WRONG_LABELED_DATA_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"ERROR_REPLICATING_WRONG_LABELED_CHKSUM_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"REPLICATED_WRONG_LABELED_DATA_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"REPLICATED_WRONG_LABELED_CHKSUM_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"CANT_REPLICATE_WRONG_LABELED_PART\0".as_ptr() as *const ::core::ffi::c_char,
    b"REPLICATED_WRONG_LABELED_PARTS\0".as_ptr() as *const ::core::ffi::c_char,
    b"DELETED_EXTRA_COPIES\0".as_ptr() as *const ::core::ffi::c_char,
    b"CANT_DELETE_EXTRA_COPIES\0".as_ptr() as *const ::core::ffi::c_char,
    b"DELETED_MFR_COPY_TO_MAKE_SPACE\0".as_ptr() as *const ::core::ffi::c_char,
    b"REPLICATED_WRONG_LABELED_COPY_UNMATCHED\0".as_ptr() as *const ::core::ffi::c_char,
    b"REPLICATED_WRONG_LABELED_COPY_BUSY\0".as_ptr() as *const ::core::ffi::c_char,
    b"REPLICATED_WRONG_LABELED_COPY_GOOD\0".as_ptr() as *const ::core::ffi::c_char,
    b"REPLICATED_UNDERGOAL_COPY\0".as_ptr() as *const ::core::ffi::c_char,
    b"CANT_REPLICATE_WRONG_LABELED_COPY\0".as_ptr() as *const ::core::ffi::c_char,
    b"CANT_REPLICATE_UNDERGOAL_COPY\0".as_ptr() as *const ::core::ffi::c_char,
    b"CANT_REBALANCE_ON_EXTRA_CALL\0".as_ptr() as *const ::core::ffi::c_char,
    b"REBALANCE_BLOCKED_BY_PRIORITY_QUEUES\0".as_ptr() as *const ::core::ffi::c_char,
    b"REBALANCE_BLOCKED_BY_TOO_MANY_FAILS\0".as_ptr() as *const ::core::ffi::c_char,
    b"REBALANCE_DONE\0".as_ptr() as *const ::core::ffi::c_char,
    b"REBALANCE_NOT_DONE_OR_NOT_NEEDED\0".as_ptr() as *const ::core::ffi::c_char,
    b"???\0".as_ptr() as *const ::core::ffi::c_char,
];
unsafe extern "C" fn job_exit_reason_to_str(mut reason: uint8_t) -> *const ::core::ffi::c_char {
    unsafe {
        if (reason as ::core::ffi::c_int) < JOB_EXIT_REASONS as ::core::ffi::c_int {
            return job_exit_reason_str[reason as usize];
        } else {
            return job_exit_reason_str[JOB_EXIT_REASONS as ::core::ffi::c_int as usize];
        };
    }
}
static mut job_exit_reasons_last: [[uint32_t; 94]; 256] = [[0; 94]; 256];
static mut job_exit_reasons: [[uint32_t; 94]; 256] = [[0; 94]; 256];
unsafe extern "C" fn chunk_job_exit_counters_shift() {
    unsafe {
        let mut i: uint32_t = 0;
        let mut sc: uint32_t = 0;
        sc = 0 as uint32_t;
        while sc < MAXSCLASS as uint32_t {
            i = 0 as uint32_t;
            while i < JOB_EXIT_REASONS as ::core::ffi::c_int as uint32_t {
                job_exit_reasons_last[sc as usize][i as usize] =
                    job_exit_reasons[sc as usize][i as usize];
                job_exit_reasons[sc as usize][i as usize] = 0 as uint32_t;
                i = i.wrapping_add(1);
            }
            sc = sc.wrapping_add(1);
        }
    }
}
pub const DANGER_PRIORITIES: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const CHUNK_PRIORITY_HASHSIZE: ::core::ffi::c_int = 0x1000000 as ::core::ffi::c_int;
pub const CHUNK_PRIORITY_HASHMASK: ::core::ffi::c_int = 0xffffff as ::core::ffi::c_int;
pub const CHUNK_PRIORITY_IOREADY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CHUNK_PRIORITY_ONECOPY_HIGHGOAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CHUNK_PRIORITY_ONECOPY_ANY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CHUNK_PRIORITY_ONEREGCOPY_PLUSMFR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CHUNK_PRIORITY_MARKEDFORREMOVAL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CHUNK_PRIORITY_UNFINISHEDEC: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const CHUNK_PRIORITY_UNDERGOAL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const CHUNK_PRIORITY_OVERGOAL: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const CHUNK_PRIORITY_WRONGLABELS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
static mut job_call_last: [[uint32_t; 10]; 256] = [[0; 10]; 256];
static mut job_call: [[uint32_t; 10]; 256] = [[0; 10]; 256];
static mut job_nocall_last: [[uint32_t; 10]; 256] = [[0; 10]; 256];
static mut job_nocall: [[uint32_t; 10]; 256] = [[0; 10]; 256];
unsafe extern "C" fn chunk_job_call_counters_shift() {
    unsafe {
        let mut i: uint32_t = 0;
        let mut sc: uint32_t = 0;
        sc = 0 as uint32_t;
        while sc < MAXSCLASS as uint32_t {
            i = 0 as uint32_t;
            while i <= DANGER_PRIORITIES as uint32_t {
                job_call_last[sc as usize][i as usize] = job_call[sc as usize][i as usize];
                job_call[sc as usize][i as usize] = 0 as uint32_t;
                job_nocall_last[sc as usize][i as usize] = job_nocall[sc as usize][i as usize];
                job_nocall[sc as usize][i as usize] = 0 as uint32_t;
                i = i.wrapping_add(1);
            }
            sc = sc.wrapping_add(1);
        }
    }
}
pub const REPLOCKHASHSIZE: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const REPLOCKHASHMASK: ::core::ffi::c_int = 0xffff as ::core::ffi::c_int;
pub const REPLOCKTIMEOUT: ::core::ffi::c_int = 120 as ::core::ffi::c_int;
static mut replock_hash: [*mut replock; 65536] = [::core::ptr::null_mut::<replock>(); 65536];
#[inline]
unsafe extern "C" fn replock_getusage(mut allocated: *mut uint64_t, mut used: *mut uint64_t) {
    unsafe {
        *allocated = replock_allocated;
        *used = replock_used;
    }
}
#[inline]
unsafe extern "C" fn replock_free_all() {
    unsafe {
        let mut srb: *mut replock_bucket = ::core::ptr::null_mut::<replock_bucket>();
        let mut nsrb: *mut replock_bucket = ::core::ptr::null_mut::<replock_bucket>();
        srb = replock_buckets_head;
        while !srb.is_null() {
            nsrb = (*srb).next as *mut replock_bucket;
            munmap(
                srb as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<replock_bucket>(),
            );
            srb = nsrb;
        }
        replock_buckets_head = ::core::ptr::null_mut::<replock_bucket>();
        replock_free_head = NULL;
        replock_allocated = 0 as uint64_t;
        replock_used = 0 as uint64_t;
    }
}
#[inline]
unsafe extern "C" fn replock_malloc() -> *mut replock {
    unsafe {
        let mut srb: *mut replock_bucket = ::core::ptr::null_mut::<replock_bucket>();
        let mut ret: *mut replock = ::core::ptr::null_mut::<replock>();
        if !replock_free_head.is_null() {
            ret = replock_free_head as *mut replock;
            replock_free_head = *(ret as *mut *mut ::core::ffi::c_void);
            replock_used = (replock_used as ::core::ffi::c_ulong)
                .wrapping_add(::core::mem::size_of::<replock>() as ::core::ffi::c_ulong)
                as uint64_t;
            return ret;
        }
        if replock_buckets_head.is_null()
            || (*replock_buckets_head).firstfree as usize
                == (10000000 as ::core::ffi::c_int as usize)
                    .wrapping_div(::core::mem::size_of::<replock>())
        {
            srb = mmap(
                NULL,
                ::core::mem::size_of::<replock_bucket>(),
                PROT_READ | PROT_WRITE,
                MAP_ANON | MAP_PRIVATE,
                -1 as ::core::ffi::c_int,
                0 as __off64_t,
            ) as *mut replock_bucket;
            if srb.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    424 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    424 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if srb
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut replock_bucket
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    424 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    424 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            (*srb).next = replock_buckets_head as *mut _replock_bucket;
            (*srb).firstfree = 0 as uint32_t;
            replock_buckets_head = srb;
            replock_allocated = (replock_allocated as ::core::ffi::c_ulong)
                .wrapping_add(::core::mem::size_of::<replock_bucket>() as ::core::ffi::c_ulong)
                as uint64_t;
        }
        ret = (&raw mut (*replock_buckets_head).bucket as *mut replock)
            .offset((*replock_buckets_head).firstfree as isize);
        (*replock_buckets_head).firstfree = (*replock_buckets_head).firstfree.wrapping_add(1);
        replock_used = (replock_used as ::core::ffi::c_ulong)
            .wrapping_add(::core::mem::size_of::<replock>() as ::core::ffi::c_ulong)
            as uint64_t;
        return ret;
    }
}
static mut replock_used: uint64_t = 0 as uint64_t;
static mut replock_buckets_head: *mut replock_bucket = ::core::ptr::null_mut::<replock_bucket>();
static mut replock_allocated: uint64_t = 0 as uint64_t;
static mut replock_free_head: *mut ::core::ffi::c_void = NULL;
#[inline]
unsafe extern "C" fn replock_free(mut p: *mut replock) {
    unsafe {
        *(p as *mut *mut ::core::ffi::c_void) = replock_free_head;
        replock_free_head = p as *mut ::core::ffi::c_void;
        replock_used = (replock_used as ::core::ffi::c_ulong)
            .wrapping_sub(::core::mem::size_of::<replock>() as ::core::ffi::c_ulong)
            as uint64_t;
    }
}
#[inline]
unsafe extern "C" fn chunk_replock_repstart(mut chunkid: uint64_t, mut now: uint32_t) {
    unsafe {
        let mut rl: *mut replock = ::core::ptr::null_mut::<replock>();
        rl = replock_hash[(chunkid & REPLOCKHASHMASK as uint64_t) as usize];
        while !rl.is_null() {
            if (*rl).chunkid == chunkid {
                (*rl).lockedto = now.wrapping_add(REPLOCKTIMEOUT as uint32_t);
                return;
            }
            rl = (*rl).next as *mut replock;
        }
        rl = replock_malloc();
        (*rl).chunkid = chunkid;
        (*rl).lockedto = now.wrapping_add(REPLOCKTIMEOUT as uint32_t);
        (*rl).next =
            replock_hash[(chunkid & REPLOCKHASHMASK as uint64_t) as usize] as *mut _replock;
        replock_hash[(chunkid & REPLOCKHASHMASK as uint64_t) as usize] = rl;
    }
}
#[inline]
unsafe extern "C" fn chunk_replock_repend(mut chunkid: uint64_t) {
    unsafe {
        let mut rl: *mut replock = ::core::ptr::null_mut::<replock>();
        let mut rlp: *mut *mut replock = ::core::ptr::null_mut::<*mut replock>();
        rlp = (&raw mut replock_hash as *mut *mut replock)
            .offset((chunkid & REPLOCKHASHMASK as uint64_t) as isize);
        loop {
            rl = *rlp;
            if rl.is_null() {
                break;
            }
            if (*rl).chunkid == chunkid {
                *rlp = (*rl).next as *mut replock;
                replock_free(rl);
                return;
            }
            rlp = &raw mut (*rl).next as *mut *mut replock;
        }
    }
}
#[inline]
unsafe extern "C" fn chunk_replock_test(mut chunkid: uint64_t, mut now: uint32_t) -> uint8_t {
    unsafe {
        let mut rl: *mut replock = ::core::ptr::null_mut::<replock>();
        rl = replock_hash[(chunkid & REPLOCKHASHMASK as uint64_t) as usize];
        while !rl.is_null() {
            if (*rl).chunkid == chunkid && (*rl).lockedto >= now {
                return 1 as uint8_t;
            }
            rl = (*rl).next as *mut replock;
        }
        return 0 as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn chunk_replock_init() {
    unsafe {
        let mut hash: uint32_t = 0;
        hash = 0 as uint32_t;
        while hash < REPLOCKHASHSIZE as uint32_t {
            replock_hash[hash as usize] = ::core::ptr::null_mut::<replock>();
            hash = hash.wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C" fn chunk_replock_cleanall() {
    unsafe {
        replock_free_all();
        chunk_replock_init();
    }
}
static mut discservers: *mut discserv = ::core::ptr::null_mut::<discserv>();
static mut discservers_next: *mut discserv = ::core::ptr::null_mut::<discserv>();
pub const FLISTNULLINDX: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FLISTONEFILEINDX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FLISTFIRSTINDX: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const FLISTMAXFCOUNT: ::core::ffi::c_int = 0xffffff as ::core::ffi::c_int;
static mut flistmem: *mut *mut flist = ::core::ptr::null_mut::<*mut flist>();
static mut flistfirstfree: uint32_t = FLISTFIRSTINDX as uint32_t;
static mut flistfreehead: uint32_t = FLISTNULLINDX as uint32_t;
#[inline]
unsafe extern "C" fn flist_init() {
    unsafe {
        let mut i: uint32_t = 0;
        if flistmem.is_null() {
            flistmem = malloc(
                (65536 as ::core::ffi::c_int as size_t)
                    .wrapping_mul(::core::mem::size_of::<*mut flist>()),
            ) as *mut *mut flist;
            i = 0 as uint32_t;
            while i < 65536 as uint32_t {
                *flistmem.offset(i as isize) = ::core::ptr::null_mut::<flist>();
                i = i.wrapping_add(1);
            }
        }
        flistfirstfree = FLISTFIRSTINDX as uint32_t;
        flistfreehead = FLISTNULLINDX as uint32_t;
    }
}
#[inline]
unsafe extern "C" fn flist_cleanup(mut full: uint8_t) {
    unsafe {
        let mut i: uint32_t = 0;
        i = 0 as uint32_t;
        while i < 65536 as uint32_t {
            if !(*flistmem.offset(i as isize)).is_null() {
                free(*flistmem.offset(i as isize) as *mut ::core::ffi::c_void);
                *flistmem.offset(i as isize) = ::core::ptr::null_mut::<flist>();
            }
            i = i.wrapping_add(1);
        }
        flistfirstfree = FLISTFIRSTINDX as uint32_t;
        flistfreehead = FLISTNULLINDX as uint32_t;
        if full != 0 {
            free(flistmem as *mut ::core::ffi::c_void);
            flistmem = ::core::ptr::null_mut::<*mut flist>();
        }
    }
}
#[inline]
unsafe extern "C" fn flist_get(mut indx: uint32_t) -> *mut flist {
    unsafe {
        return (*flistmem.offset((indx >> 16 as ::core::ffi::c_int) as isize))
            .offset((indx & 0xffff as uint32_t) as isize);
    }
}
#[inline]
unsafe extern "C" fn flist_alloc() -> uint32_t {
    unsafe {
        let mut indx: uint32_t = 0;
        let mut fl: *mut flist = ::core::ptr::null_mut::<flist>();
        if flistfreehead == FLISTNULLINDX as uint32_t {
            indx = flistfirstfree;
            flistfirstfree = flistfirstfree.wrapping_add(1);
            if (*flistmem.offset((indx >> 16 as ::core::ffi::c_int) as isize)).is_null() {
                *flistmem.offset((indx >> 16 as ::core::ffi::c_int) as isize) = malloc(
                    (65536 as ::core::ffi::c_int as size_t)
                        .wrapping_mul(::core::mem::size_of::<flist>()),
                )
                    as *mut flist;
            }
        } else {
            indx = flistfreehead;
            fl = flist_get(indx);
            flistfreehead = (*fl).nexti;
        }
        return indx;
    }
}
#[inline]
unsafe extern "C" fn flist_free(mut indx: uint32_t) {
    unsafe {
        let mut fl: *mut flist = ::core::ptr::null_mut::<flist>();
        fl = flist_get(indx);
        (*fl).nexti = flistfreehead;
        flistfreehead = indx;
    }
}
pub const FLAG_ARCH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FLAG_TRASH: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FLAG_MASK: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const STORAGE_MODE_COPIES: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STORAGE_MODE_EC8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STORAGE_MODE_EC4: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut chunkhashtab: [*mut *mut chunk; 128] = [::core::ptr::null_mut::<*mut chunk>(); 128];
static mut chunkrehashpos: uint32_t = 0;
static mut chunkhashsize: uint32_t = 0;
static mut chunkhashelem: uint32_t = 0;
static mut nextchunkid: uint64_t = 1 as uint64_t;
pub const LOCKTIMEOUT: ::core::ffi::c_int = 120 as ::core::ffi::c_int;
pub const UNUSED_DELETE_TIMEOUT: ::core::ffi::c_int =
    86400 as ::core::ffi::c_int * 7 as ::core::ffi::c_int;
static mut cstab: *mut csdata = ::core::ptr::null_mut::<csdata>();
static mut csfreehead: uint32_t = MAXCSCOUNT as uint32_t;
static mut csfreetail: uint32_t = MAXCSCOUNT as uint32_t;
static mut csusedhead: uint32_t = MAXCSCOUNT as uint32_t;
static mut opsinprogress: uint32_t = 0 as uint32_t;
static mut csregisterinprogress: uint16_t = 0 as uint16_t;
static mut chq_hash: *mut *mut chq_element = ::core::ptr::null_mut::<*mut chq_element>();
static mut chq_queue_head: [*mut chq_element; 9] = [::core::ptr::null_mut::<chq_element>(); 9];
static mut chq_queue_tail: [*mut *mut chq_element; 9] =
    [::core::ptr::null_mut::<*mut chq_element>(); 9];
static mut chq_queue_elements: [uint32_t; 9] = [0; 9];
static mut chq_elements: uint32_t = 0;
static mut chq_queue_last_append_count: [uint32_t; 9] = [0; 9];
static mut chq_queue_current_append_count: [uint32_t; 9] = [0; 9];
static mut chq_queue_last_pop_count: [uint32_t; 9] = [0; 9];
static mut chq_queue_current_pop_count: [uint32_t; 9] = [0; 9];
static mut chq_queue_last_remove_count: [uint32_t; 9] = [0; 9];
static mut chq_queue_current_remove_count: [uint32_t; 9] = [0; 9];
static mut pristr: [*const ::core::ffi::c_char; 9] = [
    b"IOREADY\0".as_ptr() as *const ::core::ffi::c_char,
    b"ONECOPY_HIGHGOAL\0".as_ptr() as *const ::core::ffi::c_char,
    b"ONECOPY_ANY\0".as_ptr() as *const ::core::ffi::c_char,
    b"ONEREGCOPY_PLUSMFR\0".as_ptr() as *const ::core::ffi::c_char,
    b"MARKEDFORREMOVAL\0".as_ptr() as *const ::core::ffi::c_char,
    b"UNFINISHEDEC\0".as_ptr() as *const ::core::ffi::c_char,
    b"UNDERGOAL\0".as_ptr() as *const ::core::ffi::c_char,
    b"OVERGOAL\0".as_ptr() as *const ::core::ffi::c_char,
    b"WRONGLABELS\0".as_ptr() as *const ::core::ffi::c_char,
];
static mut io_ready_chunk_hash: [*mut io_ready_chunk; 256] =
    [::core::ptr::null_mut::<io_ready_chunk>(); 256];
static mut JobsTimerMilliSeconds: uint32_t = 0;
static mut TicksPerSecond: uint32_t = 0;
static mut MaxFailsPerClass: uint32_t = 0;
static mut FailClassCounterResetCalls: uint32_t = 0;
static mut MaxRebalanceFails: uint32_t = 0;
static mut FailRebalanceCounterResetCalls: uint32_t = 0;
static mut rebalance_fails: [uint16_t; 256] = [0; 256];
static mut jobs_timer: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut ReplicationsDelayInit: uint32_t = 60 as uint32_t;
static mut DangerMinLeng: uint32_t = 10000 as uint32_t;
static mut DangerMaxLeng: uint32_t = 1000000 as uint32_t;
static mut MaxWriteRepl: [::core::ffi::c_double; 5] = [0.; 5];
static mut MaxReadRepl: [::core::ffi::c_double; 5] = [0.; 5];
static mut MaxDelSoftLimit: uint32_t = 0;
static mut MaxDelHardLimit: uint32_t = 0;
static mut TmpMaxDelFrac: ::core::ffi::c_double = 0.;
static mut TmpMaxDel: uint32_t = 0;
static mut ReplicationsRespectTopology: uint8_t = 0;
static mut CreationsRespectTopology: uint32_t = 0;
static mut LoopTimeMin: uint32_t = 0;
static mut HashCPTMax: uint32_t = 0;
static mut AcceptableDifference: ::core::ffi::c_double = 0.;
static mut DoNotUseSameIP: uint8_t = 0;
static mut DoNotUseSameRack: uint8_t = 0;
static mut chunksinfo: loop_info = _loop_info {
    fixed: 0 as uint32_t,
    forcekeep: 0 as uint32_t,
    delete_invalid: 0 as uint32_t,
    delete_no_longer_needed: 0 as uint32_t,
    delete_wrong_version: 0 as uint32_t,
    delete_duplicated_ecpart: 0 as uint32_t,
    delete_excess_ecpart: 0 as uint32_t,
    delete_excess_copy: 0 as uint32_t,
    delete_diskclean_ecpart: 0 as uint32_t,
    delete_diskclean_copy: 0 as uint32_t,
    replicate_dupserver_ecpart: 0 as uint32_t,
    replicate_needed_ecpart: 0 as uint32_t,
    replicate_needed_copy: 0 as uint32_t,
    replicate_wronglabels_ecpart: 0 as uint32_t,
    replicate_wronglabels_copy: 0 as uint32_t,
    split_copy_into_ecparts: 0 as uint32_t,
    join_ecparts_into_copy: 0 as uint32_t,
    recover_ecpart: 0 as uint32_t,
    calculate_ecchksum: 0 as uint32_t,
    locked_unused: 0 as uint32_t,
    locked_used: 0 as uint32_t,
    replicate_rebalance: 0 as uint32_t,
};
static mut chunksinfo_loopstart: uint32_t = 0 as uint32_t;
static mut chunksinfo_loopend: uint32_t = 0 as uint32_t;
static mut lastchunkid: uint64_t = 0 as uint64_t;
static mut lastchunkptr: *mut chunk = ::core::ptr::null_mut::<chunk>();
static mut chunks: uint32_t = 0;
static mut last_rebalance: uint32_t = 0 as uint32_t;
static mut allchunkcopycounts: *mut *mut uint64_t = ::core::ptr::null_mut::<*mut uint64_t>();
static mut regchunkcopycounts: *mut *mut uint64_t = ::core::ptr::null_mut::<*mut uint64_t>();
static mut allchunkec8counts: *mut *mut uint64_t = ::core::ptr::null_mut::<*mut uint64_t>();
static mut regchunkec8counts: *mut *mut uint64_t = ::core::ptr::null_mut::<*mut uint64_t>();
static mut allchunkec4counts: *mut *mut uint64_t = ::core::ptr::null_mut::<*mut uint64_t>();
static mut regchunkec4counts: *mut *mut uint64_t = ::core::ptr::null_mut::<*mut uint64_t>();
static mut stats_chunkops: [uint32_t; 15] = [0; 15];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_advanced_match(
    mut sm: *const storagemode,
    mut servcnt: uint32_t,
    mut servers: *const uint16_t,
) -> *mut int32_t {
    unsafe {
        let mut i: uint32_t = 0;
        let mut l: uint32_t = 0;
        let mut x: uint32_t = 0;
        let mut v: uint32_t = 0;
        let mut sid: uint32_t = 0;
        let mut sids: uint32_t = 0;
        let mut gr: uint32_t = 0;
        let mut t: int32_t = 0;
        static mut imatching: *mut int32_t = ::core::ptr::null_mut::<int32_t>();
        static mut matching: *mut int32_t = ::core::ptr::null_mut::<int32_t>();
        static mut augment: *mut int32_t = ::core::ptr::null_mut::<int32_t>();
        static mut visited: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        static mut queue: *mut int32_t = ::core::ptr::null_mut::<int32_t>();
        static mut group: *mut int32_t = ::core::ptr::null_mut::<int32_t>();
        static mut grnode: *mut int32_t = ::core::ptr::null_mut::<int32_t>();
        static mut sidval: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        static mut sidpos: *mut int32_t = ::core::ptr::null_mut::<int32_t>();
        static mut tablength: uint32_t = 0 as uint32_t;
        static mut stablength: uint32_t = 0 as uint32_t;
        let mut qfu: uint32_t = 0;
        let mut qff: uint32_t = 0;
        if ((*sm).labelscnt as uint32_t).wrapping_add(servcnt) > tablength {
            tablength = (100 as uint32_t).wrapping_add(
                (2 as uint32_t).wrapping_mul(((*sm).labelscnt as uint32_t).wrapping_add(servcnt)),
            );
            if !imatching.is_null() {
                free(imatching as *mut ::core::ffi::c_void);
            }
            if !matching.is_null() {
                free(matching as *mut ::core::ffi::c_void);
            }
            if !augment.is_null() {
                free(augment as *mut ::core::ffi::c_void);
            }
            if !visited.is_null() {
                free(visited as *mut ::core::ffi::c_void);
            }
            if !queue.is_null() {
                free(queue as *mut ::core::ffi::c_void);
            }
            imatching = malloc(::core::mem::size_of::<int32_t>().wrapping_mul(tablength as size_t))
                as *mut int32_t;
            if imatching.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    885 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"imatching\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    885 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"imatching\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if imatching
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut int32_t
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    885 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"imatching\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    885 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"imatching\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            matching = malloc(::core::mem::size_of::<int32_t>().wrapping_mul(tablength as size_t))
                as *mut int32_t;
            if matching.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    887 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"matching\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    887 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"matching\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if matching
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut int32_t
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    887 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"matching\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    887 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"matching\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
            augment = malloc(::core::mem::size_of::<int32_t>().wrapping_mul(tablength as size_t))
                as *mut int32_t;
            if augment.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    889 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"augment\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    889 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"augment\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if augment
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut int32_t
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    889 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"augment\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    889 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"augment\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_1,
                );
                abort();
            }
            visited = malloc(::core::mem::size_of::<uint8_t>().wrapping_mul(tablength as size_t))
                as *mut uint8_t;
            if visited.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    891 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"visited\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    891 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"visited\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if visited
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint8_t
            {
                let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    891 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"visited\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    891 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"visited\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_2,
                );
                abort();
            }
            queue = malloc(::core::mem::size_of::<int32_t>().wrapping_mul(tablength as size_t))
                as *mut int32_t;
            if queue.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    893 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"queue\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    893 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"queue\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if queue
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut int32_t
            {
                let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    893 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"queue\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    893 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"queue\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_3,
                );
                abort();
            }
        }
        if servcnt > stablength {
            stablength = (100 as uint32_t).wrapping_add((2 as uint32_t).wrapping_mul(servcnt));
            if !sidval.is_null() {
                free(sidval as *mut ::core::ffi::c_void);
            }
            if !sidpos.is_null() {
                free(sidpos as *mut ::core::ffi::c_void);
            }
            if !grnode.is_null() {
                free(grnode as *mut ::core::ffi::c_void);
            }
            if !group.is_null() {
                free(group as *mut ::core::ffi::c_void);
            }
            sidval = malloc(::core::mem::size_of::<uint32_t>().wrapping_mul(stablength as size_t))
                as *mut uint32_t;
            if sidval.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    914 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"sidval\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    914 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"sidval\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if sidval
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint32_t
            {
                let mut _mfs_errorstring_4: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    914 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"sidval\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    914 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"sidval\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_4,
                );
                abort();
            }
            sidpos = malloc(::core::mem::size_of::<int32_t>().wrapping_mul(stablength as size_t))
                as *mut int32_t;
            if sidpos.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    916 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"sidpos\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    916 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"sidpos\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if sidpos
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut int32_t
            {
                let mut _mfs_errorstring_5: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    916 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"sidpos\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_5,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    916 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"sidpos\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_5,
                );
                abort();
            }
            grnode = malloc(::core::mem::size_of::<int32_t>().wrapping_mul(stablength as size_t))
                as *mut int32_t;
            if grnode.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    918 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"grnode\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    918 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"grnode\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if grnode
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut int32_t
            {
                let mut _mfs_errorstring_6: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    918 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"grnode\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_6,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    918 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"grnode\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_6,
                );
                abort();
            }
            group = malloc(::core::mem::size_of::<int32_t>().wrapping_mul(stablength as size_t))
                as *mut int32_t;
            if group.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    920 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"group\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    920 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"group\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if group
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut int32_t
            {
                let mut _mfs_errorstring_7: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    920 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"group\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_7,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    920 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"group\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_7,
                );
                abort();
            }
        }
        i = 0 as uint32_t;
        while i < servcnt.wrapping_add((*sm).labelscnt as uint32_t) {
            *imatching.offset(i as isize) = -1 as ::core::ffi::c_int as int32_t;
            *matching.offset(i as isize) = -1 as ::core::ffi::c_int as int32_t;
            *augment.offset(i as isize) = -1 as ::core::ffi::c_int as int32_t;
            i = i.wrapping_add(1);
        }
        if servcnt == 0 as uint32_t
            || (*sm).labelscnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            return matching;
        }
        sids = 0 as uint32_t;
        i = 0 as uint32_t;
        while i < servcnt {
            if DoNotUseSameIP != 0 {
                sid = matocsserv_server_get_ip(
                    (*cstab.offset(*servers.offset(i as isize) as isize)).ptr,
                );
            } else if DoNotUseSameRack != 0 {
                sid = topology_get_rackid(matocsserv_server_get_ip(
                    (*cstab.offset(*servers.offset(i as isize) as isize)).ptr,
                ));
            } else if (*sm).uniqmask & UNIQ_MASK_IP as uint32_t != 0 {
                sid = matocsserv_server_get_ip(
                    (*cstab.offset(*servers.offset(i as isize) as isize)).ptr,
                );
            } else if (*sm).uniqmask & UNIQ_MASK_RACK as uint32_t != 0 {
                sid = topology_get_rackid(matocsserv_server_get_ip(
                    (*cstab.offset(*servers.offset(i as isize) as isize)).ptr,
                ));
            } else {
                sid = matocsserv_server_get_labelmask(
                    (*cstab.offset(*servers.offset(i as isize) as isize)).ptr,
                ) & (*sm).uniqmask;
            }
            if sid == 0 as uint32_t {
                *group.offset(i as isize) = i as int32_t;
            } else {
                l = 0 as uint32_t;
                while l < sids && sid != *sidval.offset(l as isize) {
                    l = l.wrapping_add(1);
                }
                if l == sids {
                    *sidval.offset(l as isize) = sid;
                    *sidpos.offset(l as isize) = i as int32_t;
                    sids = sids.wrapping_add(1);
                }
                *group.offset(i as isize) = *sidpos.offset(l as isize);
            }
            i = i.wrapping_add(1);
        }
        l = 0 as uint32_t;
        while l < (*sm).labelscnt as uint32_t {
            if *imatching.offset(l as isize) == -1 as int32_t {
                i = 0 as uint32_t;
                while i < servcnt.wrapping_add((*sm).labelscnt as uint32_t) {
                    *visited.offset(i as isize) = 0 as uint8_t;
                    i = i.wrapping_add(1);
                }
                *visited.offset(l as isize) = 1 as uint8_t;
                *augment.offset(l as isize) = -1 as ::core::ffi::c_int as int32_t;
                qff = 0 as uint32_t;
                qfu = qff;
                let c2rust_fresh6 = qff;
                qff = qff.wrapping_add(1);
                *queue.offset(c2rust_fresh6 as isize) = l as int32_t;
                qff = qff.wrapping_rem(tablength);
                while qfu != qff {
                    let c2rust_fresh7 = qfu;
                    qfu = qfu.wrapping_add(1);
                    x = *queue.offset(c2rust_fresh7 as isize) as uint32_t;
                    qfu = qfu.wrapping_rem(tablength);
                    if x < (*sm).labelscnt as uint32_t {
                        v = 0 as uint32_t;
                        while v < servcnt {
                            gr = *group.offset(v as isize) as uint32_t;
                            if *visited
                                .offset(((*sm).labelscnt as uint32_t).wrapping_add(gr) as isize)
                                as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                if matocsserv_server_matches_labelexpr(
                                    (*cstab.offset(*servers.offset(v as isize) as isize)).ptr,
                                    &raw const *(&raw const (*sm).labelexpr
                                        as *const [uint8_t; 128])
                                        .offset(x as isize)
                                        as *const uint8_t,
                                ) != 0
                                {
                                    *visited.offset(
                                        ((*sm).labelscnt as uint32_t).wrapping_add(gr) as isize,
                                    ) = 1 as uint8_t;
                                    *augment.offset(
                                        ((*sm).labelscnt as uint32_t).wrapping_add(gr) as isize,
                                    ) = x as int32_t;
                                    *grnode.offset(gr as isize) = v as int32_t;
                                    let c2rust_fresh8 = qff;
                                    qff = qff.wrapping_add(1);
                                    *queue.offset(c2rust_fresh8 as isize) =
                                        ((*sm).labelscnt as uint32_t).wrapping_add(gr) as int32_t;
                                    qff = qff.wrapping_rem(tablength);
                                }
                            }
                            v = v.wrapping_add(1);
                        }
                    } else if *imatching.offset(x as isize) >= 0 as int32_t {
                        *augment.offset(*imatching.offset(x as isize) as isize) = x as int32_t;
                        *visited.offset(*imatching.offset(x as isize) as isize) = 1 as uint8_t;
                        let c2rust_fresh9 = qff;
                        qff = qff.wrapping_add(1);
                        *queue.offset(c2rust_fresh9 as isize) = *imatching.offset(x as isize);
                        qff = qff.wrapping_rem(tablength);
                    } else {
                        while *augment.offset(x as isize) >= 0 as int32_t {
                            if x >= (*sm).labelscnt as uint32_t {
                                *imatching.offset(x as isize) = *augment.offset(x as isize);
                                *matching.offset(*augment.offset(x as isize) as isize) = *grnode
                                    .offset(x.wrapping_sub((*sm).labelscnt as uint32_t) as isize)
                                    + (*sm).labelscnt as int32_t;
                            }
                            x = *augment.offset(x as isize) as uint32_t;
                        }
                        break;
                    }
                }
            }
            l = l.wrapping_add(1);
        }
        i = 0 as uint32_t;
        while i < (*sm).labelscnt as uint32_t {
            t = *matching.offset(i as isize);
            if t >= 0 as int32_t {
                *matching.offset(t as isize) = i as int32_t;
            }
            i = i.wrapping_add(1);
        }
        return matching;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_extend_match(
    mut sm: *const storagemode,
    mut servcnt: uint32_t,
    mut matching: *mut int32_t,
) -> uint16_t {
    unsafe {
        let mut i: uint32_t = 0;
        let mut j: uint32_t = 0;
        let mut goodlabels: uint16_t = 0;
        goodlabels = 0 as uint16_t;
        i = 0 as uint32_t;
        while i < (*sm).labelscnt as uint32_t {
            if *matching.offset(i as isize) < 0 as int32_t {
                j = 0 as uint32_t;
                while j < servcnt {
                    if *matching.offset(((*sm).labelscnt as uint32_t).wrapping_add(j) as isize)
                        < 0 as int32_t
                    {
                        *matching.offset(i as isize) =
                            ((*sm).labelscnt as uint32_t).wrapping_add(j) as int32_t;
                        *matching.offset(((*sm).labelscnt as uint32_t).wrapping_add(j) as isize) =
                            i as int32_t;
                        break;
                    } else {
                        j = j.wrapping_add(1);
                    }
                }
            } else {
                goodlabels = goodlabels.wrapping_add(1);
            }
            if *matching.offset(i as isize) < 0 as int32_t {
                break;
            }
            i = i.wrapping_add(1);
        }
        return goodlabels;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_stats(mut chunkops: *mut uint32_t) {
    unsafe {
        let mut i: uint32_t = 0;
        i = 0 as uint32_t;
        while i < CHUNK_STATS_CNT as uint32_t {
            *chunkops.offset(i as isize) = stats_chunkops[i as usize];
            stats_chunkops[i as usize] = 0 as uint32_t;
            i = i.wrapping_add(1);
        }
    }
}
static mut slist_free_head: *mut ::core::ffi::c_void = NULL;
static mut slist_used: uint64_t = 0 as uint64_t;
#[inline]
unsafe extern "C" fn slist_getusage(mut allocated: *mut uint64_t, mut used: *mut uint64_t) {
    unsafe {
        *allocated = slist_allocated;
        *used = slist_used;
    }
}
#[inline]
unsafe extern "C" fn slist_malloc() -> *mut slist {
    unsafe {
        let mut srb: *mut slist_bucket = ::core::ptr::null_mut::<slist_bucket>();
        let mut ret: *mut slist = ::core::ptr::null_mut::<slist>();
        if !slist_free_head.is_null() {
            ret = slist_free_head as *mut slist;
            slist_free_head = *(ret as *mut *mut ::core::ffi::c_void);
            slist_used = (slist_used as ::core::ffi::c_ulong)
                .wrapping_add(::core::mem::size_of::<slist>() as ::core::ffi::c_ulong)
                as uint64_t;
            return ret;
        }
        if slist_buckets_head.is_null()
            || (*slist_buckets_head).firstfree as usize
                == (10000000 as ::core::ffi::c_int as usize)
                    .wrapping_div(::core::mem::size_of::<slist>())
        {
            srb = mmap(
                NULL,
                ::core::mem::size_of::<slist_bucket>(),
                PROT_READ | PROT_WRITE,
                MAP_ANON | MAP_PRIVATE,
                -1 as ::core::ffi::c_int,
                0 as __off64_t,
            ) as *mut slist_bucket;
            if srb.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1060 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1060 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if srb
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut slist_bucket
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1060 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1060 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            (*srb).next = slist_buckets_head as *mut _slist_bucket;
            (*srb).firstfree = 0 as uint32_t;
            slist_buckets_head = srb;
            slist_allocated = (slist_allocated as ::core::ffi::c_ulong)
                .wrapping_add(::core::mem::size_of::<slist_bucket>() as ::core::ffi::c_ulong)
                as uint64_t;
        }
        ret = (&raw mut (*slist_buckets_head).bucket as *mut slist)
            .offset((*slist_buckets_head).firstfree as isize);
        (*slist_buckets_head).firstfree = (*slist_buckets_head).firstfree.wrapping_add(1);
        slist_used = (slist_used as ::core::ffi::c_ulong)
            .wrapping_add(::core::mem::size_of::<slist>() as ::core::ffi::c_ulong)
            as uint64_t;
        return ret;
    }
}
#[inline]
unsafe extern "C" fn slist_free_all() {
    unsafe {
        let mut srb: *mut slist_bucket = ::core::ptr::null_mut::<slist_bucket>();
        let mut nsrb: *mut slist_bucket = ::core::ptr::null_mut::<slist_bucket>();
        srb = slist_buckets_head;
        while !srb.is_null() {
            nsrb = (*srb).next as *mut slist_bucket;
            munmap(
                srb as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<slist_bucket>(),
            );
            srb = nsrb;
        }
        slist_buckets_head = ::core::ptr::null_mut::<slist_bucket>();
        slist_free_head = NULL;
        slist_allocated = 0 as uint64_t;
        slist_used = 0 as uint64_t;
    }
}
static mut slist_buckets_head: *mut slist_bucket = ::core::ptr::null_mut::<slist_bucket>();
static mut slist_allocated: uint64_t = 0 as uint64_t;
#[inline]
unsafe extern "C" fn slist_free(mut p: *mut slist) {
    unsafe {
        *(p as *mut *mut ::core::ffi::c_void) = slist_free_head;
        slist_free_head = p as *mut ::core::ffi::c_void;
        slist_used = (slist_used as ::core::ffi::c_ulong)
            .wrapping_sub(::core::mem::size_of::<slist>() as ::core::ffi::c_ulong)
            as uint64_t;
    }
}
#[inline]
unsafe extern "C" fn chunk_free_all() {
    unsafe {
        let mut srb: *mut chunk_bucket = ::core::ptr::null_mut::<chunk_bucket>();
        let mut nsrb: *mut chunk_bucket = ::core::ptr::null_mut::<chunk_bucket>();
        srb = chunk_buckets_head;
        while !srb.is_null() {
            nsrb = (*srb).next as *mut chunk_bucket;
            munmap(
                srb as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<chunk_bucket>(),
            );
            srb = nsrb;
        }
        chunk_buckets_head = ::core::ptr::null_mut::<chunk_bucket>();
        chunk_free_head = NULL;
        chunk_allocated = 0 as uint64_t;
        chunk_used = 0 as uint64_t;
    }
}
#[inline]
unsafe extern "C" fn chunk_malloc() -> *mut chunk {
    unsafe {
        let mut srb: *mut chunk_bucket = ::core::ptr::null_mut::<chunk_bucket>();
        let mut ret: *mut chunk = ::core::ptr::null_mut::<chunk>();
        if !chunk_free_head.is_null() {
            ret = chunk_free_head as *mut chunk;
            chunk_free_head = *(ret as *mut *mut ::core::ffi::c_void);
            chunk_used = (chunk_used as ::core::ffi::c_ulong)
                .wrapping_add(::core::mem::size_of::<chunk>() as ::core::ffi::c_ulong)
                as uint64_t;
            return ret;
        }
        if chunk_buckets_head.is_null()
            || (*chunk_buckets_head).firstfree as usize
                == (10000000 as ::core::ffi::c_int as usize)
                    .wrapping_div(::core::mem::size_of::<chunk>())
        {
            srb = mmap(
                NULL,
                ::core::mem::size_of::<chunk_bucket>(),
                PROT_READ | PROT_WRITE,
                MAP_ANON | MAP_PRIVATE,
                -1 as ::core::ffi::c_int,
                0 as __off64_t,
            ) as *mut chunk_bucket;
            if srb.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1062 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1062 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if srb
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut chunk_bucket
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1062 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1062 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            (*srb).next = chunk_buckets_head as *mut _chunk_bucket;
            (*srb).firstfree = 0 as uint32_t;
            chunk_buckets_head = srb;
            chunk_allocated = (chunk_allocated as ::core::ffi::c_ulong)
                .wrapping_add(::core::mem::size_of::<chunk_bucket>() as ::core::ffi::c_ulong)
                as uint64_t;
        }
        ret = (&raw mut (*chunk_buckets_head).bucket as *mut chunk)
            .offset((*chunk_buckets_head).firstfree as isize);
        (*chunk_buckets_head).firstfree = (*chunk_buckets_head).firstfree.wrapping_add(1);
        chunk_used = (chunk_used as ::core::ffi::c_ulong)
            .wrapping_add(::core::mem::size_of::<chunk>() as ::core::ffi::c_ulong)
            as uint64_t;
        return ret;
    }
}
#[inline]
unsafe extern "C" fn chunk_getusage(mut allocated: *mut uint64_t, mut used: *mut uint64_t) {
    unsafe {
        *allocated = chunk_allocated;
        *used = chunk_used;
    }
}
#[inline]
unsafe extern "C" fn chunk_free(mut p: *mut chunk) {
    unsafe {
        *(p as *mut *mut ::core::ffi::c_void) = chunk_free_head;
        chunk_free_head = p as *mut ::core::ffi::c_void;
        chunk_used = (chunk_used as ::core::ffi::c_ulong)
            .wrapping_sub(::core::mem::size_of::<chunk>() as ::core::ffi::c_ulong)
            as uint64_t;
    }
}
static mut chunk_used: uint64_t = 0 as uint64_t;
static mut chunk_buckets_head: *mut chunk_bucket = ::core::ptr::null_mut::<chunk_bucket>();
static mut chunk_allocated: uint64_t = 0 as uint64_t;
static mut chunk_free_head: *mut ::core::ffi::c_void = NULL;
static mut chunk_queue_buckets_head: *mut chunk_queue_bucket =
    ::core::ptr::null_mut::<chunk_queue_bucket>();
#[inline]
unsafe extern "C" fn chunk_queue_getusage(mut allocated: *mut uint64_t, mut used: *mut uint64_t) {
    unsafe {
        *allocated = chunk_queue_allocated;
        *used = chunk_queue_used;
    }
}
static mut chunk_queue_free_head: *mut ::core::ffi::c_void = NULL;
#[inline]
unsafe extern "C" fn chunk_queue_malloc() -> *mut chq_element {
    unsafe {
        let mut srb: *mut chunk_queue_bucket = ::core::ptr::null_mut::<chunk_queue_bucket>();
        let mut ret: *mut chq_element = ::core::ptr::null_mut::<chq_element>();
        if !chunk_queue_free_head.is_null() {
            ret = chunk_queue_free_head as *mut chq_element;
            chunk_queue_free_head = *(ret as *mut *mut ::core::ffi::c_void);
            chunk_queue_used = (chunk_queue_used as ::core::ffi::c_ulong)
                .wrapping_add(::core::mem::size_of::<chq_element>() as ::core::ffi::c_ulong)
                as uint64_t;
            return ret;
        }
        if chunk_queue_buckets_head.is_null()
            || (*chunk_queue_buckets_head).firstfree as usize
                == (10000000 as ::core::ffi::c_int as usize)
                    .wrapping_div(::core::mem::size_of::<chq_element>())
        {
            srb = mmap(
                NULL,
                ::core::mem::size_of::<chunk_queue_bucket>(),
                PROT_READ | PROT_WRITE,
                MAP_ANON | MAP_PRIVATE,
                -1 as ::core::ffi::c_int,
                0 as __off64_t,
            ) as *mut chunk_queue_bucket;
            if srb.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1064 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1064 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if srb
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut chunk_queue_bucket
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1064 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1064 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            (*srb).next = chunk_queue_buckets_head as *mut _chunk_queue_bucket;
            (*srb).firstfree = 0 as uint32_t;
            chunk_queue_buckets_head = srb;
            chunk_queue_allocated = (chunk_queue_allocated as ::core::ffi::c_ulong)
                .wrapping_add(::core::mem::size_of::<chunk_queue_bucket>() as ::core::ffi::c_ulong)
                as uint64_t;
        }
        ret = (&raw mut (*chunk_queue_buckets_head).bucket as *mut chq_element)
            .offset((*chunk_queue_buckets_head).firstfree as isize);
        (*chunk_queue_buckets_head).firstfree =
            (*chunk_queue_buckets_head).firstfree.wrapping_add(1);
        chunk_queue_used = (chunk_queue_used as ::core::ffi::c_ulong)
            .wrapping_add(::core::mem::size_of::<chq_element>() as ::core::ffi::c_ulong)
            as uint64_t;
        return ret;
    }
}
static mut chunk_queue_allocated: uint64_t = 0 as uint64_t;
#[inline]
unsafe extern "C" fn chunk_queue_free_all() {
    unsafe {
        let mut srb: *mut chunk_queue_bucket = ::core::ptr::null_mut::<chunk_queue_bucket>();
        let mut nsrb: *mut chunk_queue_bucket = ::core::ptr::null_mut::<chunk_queue_bucket>();
        srb = chunk_queue_buckets_head;
        while !srb.is_null() {
            nsrb = (*srb).next as *mut chunk_queue_bucket;
            munmap(
                srb as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<chunk_queue_bucket>(),
            );
            srb = nsrb;
        }
        chunk_queue_buckets_head = ::core::ptr::null_mut::<chunk_queue_bucket>();
        chunk_queue_free_head = NULL;
        chunk_queue_allocated = 0 as uint64_t;
        chunk_queue_used = 0 as uint64_t;
    }
}
static mut chunk_queue_used: uint64_t = 0 as uint64_t;
#[inline]
unsafe extern "C" fn chunk_queue_free(mut p: *mut chq_element) {
    unsafe {
        *(p as *mut *mut ::core::ffi::c_void) = chunk_queue_free_head;
        chunk_queue_free_head = p as *mut ::core::ffi::c_void;
        chunk_queue_used = (chunk_queue_used as ::core::ffi::c_ulong)
            .wrapping_sub(::core::mem::size_of::<chq_element>() as ::core::ffi::c_ulong)
            as uint64_t;
    }
}
#[inline]
unsafe extern "C" fn chunk_hash_init() {
    unsafe {
        let mut i: uint16_t = 0;
        chunkhashsize = 0 as uint32_t;
        chunkhashelem = 0 as uint32_t;
        chunkrehashpos = 0 as uint32_t;
        i = 0 as uint16_t;
        while (i as ::core::ffi::c_uint) < HASHTAB_HISIZE {
            chunkhashtab[i as usize] = ::core::ptr::null_mut::<*mut chunk>();
            i = i.wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C" fn chunk_hash_cleanup() {
    unsafe {
        let mut i: uint16_t = 0;
        chunkhashelem = 0 as uint32_t;
        chunkhashsize = 0 as uint32_t;
        chunkrehashpos = 0 as uint32_t;
        i = 0 as uint16_t;
        while (i as ::core::ffi::c_uint) < HASHTAB_HISIZE {
            if !chunkhashtab[i as usize].is_null() {
                munmap(
                    chunkhashtab[i as usize] as *mut ::core::ffi::c_void,
                    ::core::mem::size_of::<*mut chunk>().wrapping_mul(HASHTAB_LOSIZE as size_t),
                );
            }
            chunkhashtab[i as usize] = ::core::ptr::null_mut::<*mut chunk>();
            i = i.wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C" fn chunk_hash_rehash() {
    unsafe {
        let mut i: uint16_t = 0;
        chunkrehashpos = chunkhashsize;
        chunkhashsize = chunkhashsize.wrapping_mul(2 as uint32_t);
        i = (chunkhashsize >> HASHTAB_LOBITS).wrapping_div(2 as uint32_t) as uint16_t;
        while (i as uint32_t) < chunkhashsize >> HASHTAB_LOBITS {
            chunkhashtab[i as usize] = mmap(
                NULL,
                ::core::mem::size_of::<*mut chunk>().wrapping_mul(HASHTAB_LOSIZE as size_t),
                PROT_READ | PROT_WRITE,
                MAP_ANON | MAP_PRIVATE,
                -1 as ::core::ffi::c_int,
                0 as __off64_t,
            ) as *mut *mut chunk;
            if chunkhashtab[i as usize].is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1120 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"chunkhashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1120 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"chunkhashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if chunkhashtab[i as usize]
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut *mut chunk
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1120 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"chunkhashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1120 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"chunkhashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            i = i.wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C" fn chunk_hash_move() {
    unsafe {
        let mut hash: uint32_t = 0;
        let mut mask: uint32_t = 0;
        let mut moved: uint32_t = 0 as uint32_t;
        let mut chptr: *mut *mut chunk = ::core::ptr::null_mut::<*mut chunk>();
        let mut chptralt: *mut *mut chunk = ::core::ptr::null_mut::<*mut chunk>();
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        mask = chunkhashsize.wrapping_sub(1 as uint32_t);
        loop {
            if chunkrehashpos >= chunkhashsize {
                chunkrehashpos = chunkhashsize;
                return;
            }
            chptr = chunkhashtab[(chunkrehashpos
                .wrapping_sub(chunkhashsize.wrapping_div(2 as uint32_t))
                >> HASHTAB_LOBITS) as usize]
                .offset((chunkrehashpos & HASHTAB_MASK as uint32_t) as isize);
            chptralt = chunkhashtab[(chunkrehashpos >> HASHTAB_LOBITS) as usize]
                .offset((chunkrehashpos & HASHTAB_MASK as uint32_t) as isize);
            *chptralt = ::core::ptr::null_mut::<chunk>();
            loop {
                c = *chptr;
                if c.is_null() {
                    break;
                }
                hash = hash32((*c).chunkid as uint32_t) & mask;
                if hash == chunkrehashpos {
                    *chptralt = c;
                    *chptr = (*c).next as *mut chunk;
                    chptralt = &raw mut (*c).next as *mut *mut chunk;
                    (*c).next = ::core::ptr::null_mut::<chunk>();
                } else {
                    chptr = &raw mut (*c).next as *mut *mut chunk;
                }
                moved = moved.wrapping_add(1);
            }
            chunkrehashpos = chunkrehashpos.wrapping_add(1);
            if moved >= CHUNKHASH_MOVEFACTOR as uint32_t {
                break;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn chunk_hash_find(mut chunkid: uint64_t) -> *mut chunk {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut hash: uint32_t = 0;
        if chunkhashsize == 0 as uint32_t {
            return ::core::ptr::null_mut::<chunk>();
        }
        hash = hash32(chunkid as uint32_t) & chunkhashsize.wrapping_sub(1 as uint32_t);
        if chunkrehashpos < chunkhashsize {
            chunk_hash_move();
            if hash >= chunkrehashpos {
                hash = hash.wrapping_sub(chunkhashsize.wrapping_div(2 as uint32_t));
            }
        }
        c = *chunkhashtab[(hash >> HASHTAB_LOBITS) as usize]
            .offset((hash & HASHTAB_MASK as uint32_t) as isize);
        while !c.is_null() {
            if (*c).chunkid == chunkid {
                return c;
            }
            c = (*c).next as *mut chunk;
        }
        return ::core::ptr::null_mut::<chunk>();
    }
}
#[inline]
unsafe extern "C" fn chunk_hash_delete(mut c: *mut chunk) {
    unsafe {
        let mut chptr: *mut *mut chunk = ::core::ptr::null_mut::<*mut chunk>();
        let mut cit: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut hash: uint32_t = 0;
        if chunkhashsize == 0 as uint32_t {
            return;
        }
        hash = hash32((*c).chunkid as uint32_t) & chunkhashsize.wrapping_sub(1 as uint32_t);
        if chunkrehashpos < chunkhashsize {
            chunk_hash_move();
            if hash >= chunkrehashpos {
                hash = hash.wrapping_sub(chunkhashsize.wrapping_div(2 as uint32_t));
            }
        }
        chptr = chunkhashtab[(hash >> HASHTAB_LOBITS) as usize]
            .offset((hash & HASHTAB_MASK as uint32_t) as isize);
        loop {
            cit = *chptr;
            if cit.is_null() {
                break;
            }
            if cit == c {
                *chptr = (*c).next as *mut chunk;
                chunkhashelem = chunkhashelem.wrapping_sub(1);
                return;
            }
            chptr = &raw mut (*cit).next as *mut *mut chunk;
        }
    }
}
#[inline]
unsafe extern "C" fn chunk_hash_add(mut c: *mut chunk) {
    unsafe {
        let mut i: uint16_t = 0;
        let mut hash: uint32_t = 0;
        if chunkhashsize == 0 as uint32_t {
            chunkhashsize = HASHTAB_LOSIZE as uint32_t;
            chunkrehashpos = chunkhashsize;
            chunkhashelem = 0 as uint32_t;
            i = 0 as uint16_t;
            while (i as uint32_t) < chunkhashsize >> HASHTAB_LOBITS {
                chunkhashtab[i as usize] = mmap(
                    NULL,
                    ::core::mem::size_of::<*mut chunk>().wrapping_mul(HASHTAB_LOSIZE as size_t),
                    PROT_READ | PROT_WRITE,
                    MAP_ANON | MAP_PRIVATE,
                    -1 as ::core::ffi::c_int,
                    0 as __off64_t,
                ) as *mut *mut chunk;
                if chunkhashtab[i as usize].is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1215 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"chunkhashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1215 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"chunkhashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if chunkhashtab[i as usize]
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut *mut chunk
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1215 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"chunkhashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1215 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"chunkhashtab[i]\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
                memset(
                    chunkhashtab[i as usize] as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<*mut chunk>(),
                );
                if (*chunkhashtab[i as usize].offset(0 as isize)).is_null() {
                    memset(
                        chunkhashtab[i as usize] as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<*mut chunk>().wrapping_mul(HASHTAB_LOSIZE as size_t),
                    );
                } else {
                    hash = 0 as uint32_t;
                    while hash < HASHTAB_LOSIZE as uint32_t {
                        *chunkhashtab[i as usize].offset(hash as isize) =
                            ::core::ptr::null_mut::<chunk>();
                        hash = hash.wrapping_add(1);
                    }
                }
                i = i.wrapping_add(1);
            }
        }
        hash = hash32((*c).chunkid as uint32_t) & chunkhashsize.wrapping_sub(1 as uint32_t);
        if chunkrehashpos < chunkhashsize {
            chunk_hash_move();
            if hash >= chunkrehashpos {
                hash = hash.wrapping_sub(chunkhashsize.wrapping_div(2 as uint32_t));
            }
            (*c).next = *chunkhashtab[(hash >> HASHTAB_LOBITS) as usize]
                .offset((hash & HASHTAB_MASK as uint32_t) as isize)
                as *mut chunk;
            *chunkhashtab[(hash >> HASHTAB_LOBITS) as usize]
                .offset((hash & HASHTAB_MASK as uint32_t) as isize) = c;
            chunkhashelem = chunkhashelem.wrapping_add(1);
        } else {
            (*c).next = *chunkhashtab[(hash >> HASHTAB_LOBITS) as usize]
                .offset((hash & HASHTAB_MASK as uint32_t) as isize)
                as *mut chunk;
            *chunkhashtab[(hash >> HASHTAB_LOBITS) as usize]
                .offset((hash & HASHTAB_MASK as uint32_t) as isize) = c;
            chunkhashelem = chunkhashelem.wrapping_add(1);
            if chunkhashelem > chunkhashsize
                && chunkhashsize >> HASHTAB_LOBITS < HASHTAB_HISIZE as uint32_t
            {
                chunk_hash_rehash();
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_new(mut chunkid: uint64_t) -> *mut chunk {
    unsafe {
        let mut newchunk: *mut chunk = ::core::ptr::null_mut::<chunk>();
        newchunk = chunk_malloc();
        chunks = chunks.wrapping_add(1);
        *(*allchunkcopycounts.offset(0 as isize)).offset(0 as isize) =
            (*(*allchunkcopycounts.offset(0 as isize)).offset(0 as isize)).wrapping_add(1);
        *(*regchunkcopycounts.offset(0 as isize)).offset(0 as isize) =
            (*(*regchunkcopycounts.offset(0 as isize)).offset(0 as isize)).wrapping_add(1);
        (*newchunk).chunkid = chunkid;
        (*newchunk).set_version(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*newchunk).sclassid = 0 as uint8_t;
        (*newchunk).lockedto = 0 as uint32_t;
        (*newchunk)
            .set_storage_mode(STORAGE_MODE_COPIES as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*newchunk).set_all_gequiv(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*newchunk).set_reg_gequiv(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*newchunk).set_needverincrease(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*newchunk).set_allowreadzeros(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*newchunk).set_ondangerlist(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*newchunk).set_interrupted(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*newchunk).set_writeinprogress(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*newchunk).set_flags(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*newchunk).set_operation(
            NONE as ::core::ffi::c_int as ::core::ffi::c_uint as ::core::ffi::c_uint,
        );
        (*newchunk).slisthead = ::core::ptr::null_mut::<slist>();
        (*newchunk).fhead = FLISTNULLINDX as uint32_t;
        lastchunkid = chunkid;
        lastchunkptr = newchunk;
        chunk_hash_add(newchunk);
        return newchunk;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_find(mut chunkid: uint64_t) -> *mut chunk {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        if lastchunkid == chunkid {
            return lastchunkptr;
        }
        c = chunk_hash_find(chunkid);
        if !c.is_null() {
            lastchunkid = chunkid;
            lastchunkptr = c;
        }
        return c;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_delete(mut c: *mut chunk) {
    unsafe {
        let mut indx: uint32_t = 0;
        indx = ((*c).sclassid as uint32_t).wrapping_add(
            (MAXSCLASS * ((*c).flags() as ::core::ffi::c_int & FLAG_MASK)) as uint32_t,
        );
        if lastchunkptr == c {
            lastchunkid = 0 as uint64_t;
            lastchunkptr = ::core::ptr::null_mut::<chunk>();
        }
        chunks = chunks.wrapping_sub(1);
        if (*c).storage_mode() as ::core::ffi::c_int == STORAGE_MODE_COPIES {
            *(*allchunkcopycounts.offset(indx as isize)).offset(0 as isize) =
                (*(*allchunkcopycounts.offset(indx as isize)).offset(0 as isize)).wrapping_sub(1);
            *(*regchunkcopycounts.offset(indx as isize)).offset(0 as isize) =
                (*(*regchunkcopycounts.offset(indx as isize)).offset(0 as isize)).wrapping_sub(1);
        } else if (*c).storage_mode() as ::core::ffi::c_int == STORAGE_MODE_EC8 {
            *(*allchunkec8counts.offset(indx as isize)).offset(0 as isize) =
                (*(*allchunkec8counts.offset(indx as isize)).offset(0 as isize)).wrapping_sub(1);
            *(*regchunkec8counts.offset(indx as isize)).offset(0 as isize) =
                (*(*regchunkec8counts.offset(indx as isize)).offset(0 as isize)).wrapping_sub(1);
        } else if (*c).storage_mode() as ::core::ffi::c_int == STORAGE_MODE_EC4 {
            *(*allchunkec4counts.offset(indx as isize)).offset(0 as isize) =
                (*(*allchunkec4counts.offset(indx as isize)).offset(0 as isize)).wrapping_sub(1);
            *(*regchunkec4counts.offset(indx as isize)).offset(0 as isize) =
                (*(*regchunkec4counts.offset(indx as isize)).offset(0 as isize)).wrapping_sub(1);
        }
        chunk_hash_delete(c);
        chunk_free(c);
    }
}
unsafe extern "C" fn chunk_ecid_to_str(mut ecid: uint8_t) -> *const ::core::ffi::c_char {
    unsafe {
        let mut ecid8names: [*const ::core::ffi::c_char; 17] = [
            b"DE0\0".as_ptr() as *const ::core::ffi::c_char,
            b"DE1\0".as_ptr() as *const ::core::ffi::c_char,
            b"DE2\0".as_ptr() as *const ::core::ffi::c_char,
            b"DE3\0".as_ptr() as *const ::core::ffi::c_char,
            b"DE4\0".as_ptr() as *const ::core::ffi::c_char,
            b"DE5\0".as_ptr() as *const ::core::ffi::c_char,
            b"DE6\0".as_ptr() as *const ::core::ffi::c_char,
            b"DE7\0".as_ptr() as *const ::core::ffi::c_char,
            b"CE0\0".as_ptr() as *const ::core::ffi::c_char,
            b"CE1\0".as_ptr() as *const ::core::ffi::c_char,
            b"CE2\0".as_ptr() as *const ::core::ffi::c_char,
            b"CE3\0".as_ptr() as *const ::core::ffi::c_char,
            b"CE4\0".as_ptr() as *const ::core::ffi::c_char,
            b"CE5\0".as_ptr() as *const ::core::ffi::c_char,
            b"CE6\0".as_ptr() as *const ::core::ffi::c_char,
            b"CE7\0".as_ptr() as *const ::core::ffi::c_char,
            b"CE8\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        let mut ecid4names: [*const ::core::ffi::c_char; 13] = [
            b"DF0\0".as_ptr() as *const ::core::ffi::c_char,
            b"DF1\0".as_ptr() as *const ::core::ffi::c_char,
            b"DF2\0".as_ptr() as *const ::core::ffi::c_char,
            b"DF3\0".as_ptr() as *const ::core::ffi::c_char,
            b"CF0\0".as_ptr() as *const ::core::ffi::c_char,
            b"CF1\0".as_ptr() as *const ::core::ffi::c_char,
            b"CF2\0".as_ptr() as *const ::core::ffi::c_char,
            b"CF3\0".as_ptr() as *const ::core::ffi::c_char,
            b"CF4\0".as_ptr() as *const ::core::ffi::c_char,
            b"CF5\0".as_ptr() as *const ::core::ffi::c_char,
            b"CF6\0".as_ptr() as *const ::core::ffi::c_char,
            b"CF7\0".as_ptr() as *const ::core::ffi::c_char,
            b"CF8\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        if ecid as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int != 0 {
            if (ecid as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int) < 17 as ::core::ffi::c_int
            {
                return ecid8names
                    [(ecid as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int) as usize];
            }
        } else if ecid as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int != 0 {
            if (ecid as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) < 13 as ::core::ffi::c_int {
                return ecid4names
                    [(ecid as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as usize];
            }
        } else if ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return b"COPY\0".as_ptr() as *const ::core::ffi::c_char;
        }
        return b"???\0".as_ptr() as *const ::core::ffi::c_char;
    }
}
#[inline]
unsafe extern "C" fn chunk_check_ecid(mut ecid: uint8_t) -> ::core::ffi::c_int {
    unsafe {
        if ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                && ecid as ::core::ffi::c_int <= 0x30 as ::core::ffi::c_int
            || ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                && ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        return -1 as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn chunk_state_change(
    mut oldsclassid: uint8_t,
    mut newsclassid: uint8_t,
    mut oldflags: uint8_t,
    mut newflags: uint8_t,
    mut old_storage_mode: uint8_t,
    mut new_storage_mode: uint8_t,
    mut old_all_ge: uint8_t,
    mut new_all_ge: uint8_t,
    mut old_reg_ge: uint8_t,
    mut new_reg_ge: uint8_t,
) {
    unsafe {
        let mut oldindx: uint32_t = 0;
        let mut newindx: uint32_t = 0;
        oldindx = (oldsclassid as uint32_t)
            .wrapping_add((MAXSCLASS * (oldflags as ::core::ffi::c_int & FLAG_MASK)) as uint32_t);
        newindx = (newsclassid as uint32_t)
            .wrapping_add((MAXSCLASS * (newflags as ::core::ffi::c_int & FLAG_MASK)) as uint32_t);
        if old_all_ge as ::core::ffi::c_int > 9 as ::core::ffi::c_int {
            old_all_ge = 10 as uint8_t;
        }
        if new_all_ge as ::core::ffi::c_int > 9 as ::core::ffi::c_int {
            new_all_ge = 10 as uint8_t;
        }
        if old_reg_ge as ::core::ffi::c_int > 9 as ::core::ffi::c_int {
            old_reg_ge = 10 as uint8_t;
        }
        if new_reg_ge as ::core::ffi::c_int > 9 as ::core::ffi::c_int {
            new_reg_ge = 10 as uint8_t;
        }
        if old_storage_mode as ::core::ffi::c_int == STORAGE_MODE_COPIES {
            *(*allchunkcopycounts.offset(oldindx as isize)).offset(old_all_ge as isize) =
                (*(*allchunkcopycounts.offset(oldindx as isize)).offset(old_all_ge as isize))
                    .wrapping_sub(1);
            *(*regchunkcopycounts.offset(oldindx as isize)).offset(old_reg_ge as isize) =
                (*(*regchunkcopycounts.offset(oldindx as isize)).offset(old_reg_ge as isize))
                    .wrapping_sub(1);
        } else if old_storage_mode as ::core::ffi::c_int == STORAGE_MODE_EC8 {
            *(*allchunkec8counts.offset(oldindx as isize)).offset(old_all_ge as isize) =
                (*(*allchunkec8counts.offset(oldindx as isize)).offset(old_all_ge as isize))
                    .wrapping_sub(1);
            *(*regchunkec8counts.offset(oldindx as isize)).offset(old_reg_ge as isize) =
                (*(*regchunkec8counts.offset(oldindx as isize)).offset(old_reg_ge as isize))
                    .wrapping_sub(1);
        } else if old_storage_mode as ::core::ffi::c_int == STORAGE_MODE_EC4 {
            *(*allchunkec4counts.offset(oldindx as isize)).offset(old_all_ge as isize) =
                (*(*allchunkec4counts.offset(oldindx as isize)).offset(old_all_ge as isize))
                    .wrapping_sub(1);
            *(*regchunkec4counts.offset(oldindx as isize)).offset(old_reg_ge as isize) =
                (*(*regchunkec4counts.offset(oldindx as isize)).offset(old_reg_ge as isize))
                    .wrapping_sub(1);
        }
        if new_storage_mode as ::core::ffi::c_int == STORAGE_MODE_COPIES {
            *(*allchunkcopycounts.offset(newindx as isize)).offset(new_all_ge as isize) =
                (*(*allchunkcopycounts.offset(newindx as isize)).offset(new_all_ge as isize))
                    .wrapping_add(1);
            *(*regchunkcopycounts.offset(newindx as isize)).offset(new_reg_ge as isize) =
                (*(*regchunkcopycounts.offset(newindx as isize)).offset(new_reg_ge as isize))
                    .wrapping_add(1);
        } else if new_storage_mode as ::core::ffi::c_int == STORAGE_MODE_EC8 {
            *(*allchunkec8counts.offset(newindx as isize)).offset(new_all_ge as isize) =
                (*(*allchunkec8counts.offset(newindx as isize)).offset(new_all_ge as isize))
                    .wrapping_add(1);
            *(*regchunkec8counts.offset(newindx as isize)).offset(new_reg_ge as isize) =
                (*(*regchunkec8counts.offset(newindx as isize)).offset(new_reg_ge as isize))
                    .wrapping_add(1);
        } else if new_storage_mode as ::core::ffi::c_int == STORAGE_MODE_EC4 {
            *(*allchunkec4counts.offset(newindx as isize)).offset(new_all_ge as isize) =
                (*(*allchunkec4counts.offset(newindx as isize)).offset(new_all_ge as isize))
                    .wrapping_add(1);
            *(*regchunkec4counts.offset(newindx as isize)).offset(new_reg_ge as isize) =
                (*(*regchunkec4counts.offset(newindx as isize)).offset(new_reg_ge as isize))
                    .wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C" fn chunk_state_set_counters(
    mut c: *mut chunk,
    mut storage_mode: uint8_t,
    mut all_ge: uint8_t,
    mut reg_ge: uint8_t,
) {
    unsafe {
        chunk_state_change(
            (*c).sclassid,
            (*c).sclassid,
            (*c).flags() as uint8_t,
            (*c).flags() as uint8_t,
            (*c).storage_mode() as uint8_t,
            storage_mode,
            (*c).all_gequiv() as uint8_t,
            all_ge,
            (*c).reg_gequiv() as uint8_t,
            reg_ge,
        );
        (*c).set_storage_mode(storage_mode as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*c).set_all_gequiv(all_ge as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*c).set_reg_gequiv(reg_ge as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
}
#[inline]
unsafe extern "C" fn chunk_state_set_flags(mut c: *mut chunk, mut new_flags: uint8_t) {
    unsafe {
        chunk_state_change(
            (*c).sclassid,
            (*c).sclassid,
            (*c).flags() as uint8_t,
            new_flags,
            (*c).storage_mode() as uint8_t,
            (*c).storage_mode() as uint8_t,
            (*c).all_gequiv() as uint8_t,
            (*c).all_gequiv() as uint8_t,
            (*c).reg_gequiv() as uint8_t,
            (*c).reg_gequiv() as uint8_t,
        );
        (*c).set_flags(new_flags as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
}
#[inline]
unsafe extern "C" fn chunk_state_set_sclass(mut c: *mut chunk, mut new_sclassid: uint8_t) {
    unsafe {
        chunk_state_change(
            (*c).sclassid,
            new_sclassid,
            (*c).flags() as uint8_t,
            (*c).flags() as uint8_t,
            (*c).storage_mode() as uint8_t,
            (*c).storage_mode() as uint8_t,
            (*c).all_gequiv() as uint8_t,
            (*c).all_gequiv() as uint8_t,
            (*c).reg_gequiv() as uint8_t,
            (*c).reg_gequiv() as uint8_t,
        );
        (*c).sclassid = new_sclassid;
    }
}
#[inline]
unsafe extern "C" fn chunk_calc_ecge(
    mut mask8: uint32_t,
    mut mask4: uint32_t,
    mut ec8uniqserv: uint8_t,
    mut ec4uniqserv: uint8_t,
    mut smode: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut ge: uint32_t = 0;
        let mut bcnt: uint32_t = 0;
        ge = 0 as uint32_t;
        *smode = STORAGE_MODE_COPIES as uint8_t;
        if mask8 != 0 {
            bcnt = bitcount(mask8) as uint32_t;
            if (ec8uniqserv as uint32_t) < bcnt {
                bcnt = ec8uniqserv as uint32_t;
            }
            if bcnt > 7 as uint32_t {
                ge = bcnt.wrapping_sub(7 as uint32_t);
                *smode = STORAGE_MODE_EC8 as uint8_t;
            }
        }
        if mask4 != 0 {
            bcnt = bitcount(mask4) as uint32_t;
            if (ec4uniqserv as uint32_t) < bcnt {
                bcnt = ec4uniqserv as uint32_t;
            }
            if bcnt > 3 as uint32_t && bcnt.wrapping_sub(3 as uint32_t) > ge {
                ge = bcnt.wrapping_sub(3 as uint32_t);
                *smode = STORAGE_MODE_EC4 as uint8_t;
            }
        }
        return ge as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn chunk_state_fix(mut c: *mut chunk) {
    unsafe {
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut allmask8: uint32_t = 0;
        let mut allmask4: uint32_t = 0;
        let mut regmask8: uint32_t = 0;
        let mut regmask4: uint32_t = 0;
        let mut regc: uint8_t = 0;
        let mut allc: uint8_t = 0;
        let mut regecge: uint8_t = 0;
        let mut allecge: uint8_t = 0;
        let mut lregec4csid: int32_t = 0;
        let mut lregec8csid: int32_t = 0;
        let mut lallec4csid: int32_t = 0;
        let mut lallec8csid: int32_t = 0;
        let mut regec4uniqserv: uint8_t = 0;
        let mut regec8uniqserv: uint8_t = 0;
        let mut allec4uniqserv: uint8_t = 0;
        let mut allec8uniqserv: uint8_t = 0;
        let mut storage_mode: uint8_t = 0;
        allmask8 = 0 as uint32_t;
        allmask4 = 0 as uint32_t;
        regmask8 = 0 as uint32_t;
        regmask4 = 0 as uint32_t;
        regc = 0 as uint8_t;
        allc = 0 as uint8_t;
        lregec4csid = -1 as ::core::ffi::c_int as int32_t;
        lregec8csid = -1 as ::core::ffi::c_int as int32_t;
        lallec4csid = -1 as ::core::ffi::c_int as int32_t;
        lallec8csid = -1 as ::core::ffi::c_int as int32_t;
        regec4uniqserv = 0 as uint8_t;
        regec8uniqserv = 0 as uint8_t;
        allec4uniqserv = 0 as uint8_t;
        allec8uniqserv = 0 as uint8_t;
        s = (*c).slisthead;
        while !s.is_null() {
            if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                || (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int
                || (*s).valid as ::core::ffi::c_int == BUSY as ::core::ffi::c_int
                || (*s).valid as ::core::ffi::c_int == TDBUSY as ::core::ffi::c_int
            {
                if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    allc = allc.wrapping_add(1);
                } else if (*s).ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
                {
                    allmask4 = (allmask4 as ::core::ffi::c_uint
                        | (1 as ::core::ffi::c_uint)
                            << ((*s).ecid as ::core::ffi::c_int & 0xf as ::core::ffi::c_int))
                        as uint32_t;
                    if (*s).csid as int32_t != lallec4csid {
                        allec4uniqserv = allec4uniqserv.wrapping_add(1);
                    }
                    lallec4csid = (*s).csid as int32_t;
                } else if (*s).ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int <= 0x30 as ::core::ffi::c_int
                {
                    allmask8 = (allmask8 as ::core::ffi::c_uint
                        | (1 as ::core::ffi::c_uint)
                            << ((*s).ecid as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
                        as uint32_t;
                    if (*s).csid as int32_t != lallec8csid {
                        allec8uniqserv = allec8uniqserv.wrapping_add(1);
                    }
                    lallec8csid = (*s).csid as int32_t;
                }
                if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                    || (*s).valid as ::core::ffi::c_int == BUSY as ::core::ffi::c_int
                {
                    if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        regc = regc.wrapping_add(1);
                    } else if (*s).ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
                    {
                        regmask4 = (regmask4 as ::core::ffi::c_uint
                            | (1 as ::core::ffi::c_uint)
                                << ((*s).ecid as ::core::ffi::c_int & 0xf as ::core::ffi::c_int))
                            as uint32_t;
                        if (*s).csid as int32_t != lregec4csid {
                            regec4uniqserv = regec4uniqserv.wrapping_add(1);
                        }
                        lregec4csid = (*s).csid as int32_t;
                    } else if (*s).ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x30 as ::core::ffi::c_int
                    {
                        regmask8 = (regmask8 as ::core::ffi::c_uint
                            | (1 as ::core::ffi::c_uint)
                                << ((*s).ecid as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
                            as uint32_t;
                        if (*s).csid as int32_t != lregec8csid {
                            regec8uniqserv = regec8uniqserv.wrapping_add(1);
                        }
                        lregec8csid = (*s).csid as int32_t;
                    }
                }
            }
            s = (*s).next as *mut slist;
        }
        regecge = chunk_calc_ecge(
            regmask8,
            regmask4,
            regec8uniqserv,
            regec4uniqserv,
            &raw mut storage_mode,
        );
        allecge = chunk_calc_ecge(
            allmask8,
            allmask4,
            allec8uniqserv,
            allec4uniqserv,
            &raw mut storage_mode,
        );
        if allc as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && allecge as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            if allmask8 > 0 as uint32_t && allmask4 > 0 as uint32_t {
                if bitcount(allmask8) as ::core::ffi::c_int
                    > bitcount(allmask4) as ::core::ffi::c_int
                {
                    storage_mode = STORAGE_MODE_EC8 as uint8_t;
                } else {
                    storage_mode = STORAGE_MODE_EC4 as uint8_t;
                }
            } else if allmask8 > 0 as uint32_t {
                storage_mode = STORAGE_MODE_EC8 as uint8_t;
            } else if allmask4 > 0 as uint32_t {
                storage_mode = STORAGE_MODE_EC4 as uint8_t;
            } else {
                storage_mode = STORAGE_MODE_COPIES as uint8_t;
            }
        } else if allc as ::core::ffi::c_int >= allecge as ::core::ffi::c_int {
            storage_mode = STORAGE_MODE_COPIES as uint8_t;
        }
        if storage_mode as ::core::ffi::c_int == STORAGE_MODE_COPIES {
            allecge = allc;
            regecge = regc;
        }
        if allecge as ::core::ffi::c_int > 15 as ::core::ffi::c_int {
            allecge = 15 as uint8_t;
        }
        if regecge as ::core::ffi::c_int > 15 as ::core::ffi::c_int {
            regecge = 15 as uint8_t;
        }
        if storage_mode as ::core::ffi::c_int != (*c).storage_mode() as ::core::ffi::c_int
            || allecge as ::core::ffi::c_int != (*c).all_gequiv() as ::core::ffi::c_int
            || regecge as ::core::ffi::c_int != (*c).reg_gequiv() as ::core::ffi::c_int
        {
            chunk_state_set_counters(c, storage_mode, allecge, regecge);
        }
    }
}
#[inline]
unsafe extern "C" fn chunk_set_op(mut c: *mut chunk, mut op: uint8_t) {
    unsafe {
        (*c).set_operation(op as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_count() -> uint32_t {
    unsafe {
        return chunks;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_sclass_inc_counters(
    mut sclassid: uint8_t,
    mut flags: uint8_t,
    mut gequiv: uint8_t,
    mut counters: *mut uint64_t,
) {
    unsafe {
        let mut indx: uint16_t = (sclassid as uint16_t as ::core::ffi::c_int
            + MAXSCLASS
                * (flags as ::core::ffi::c_int & FLAG_MASK) as uint16_t as ::core::ffi::c_int)
            as uint16_t;
        let mut i: uint32_t = 0;
        i = 0 as uint32_t;
        while i < 11 as uint32_t {
            if i < gequiv as uint32_t {
                *counters.offset(0 as isize) = (*counters.offset(0 as isize))
                    .wrapping_add(*(*regchunkcopycounts.offset(indx as isize)).offset(i as isize));
                *counters.offset(1 as isize) = (*counters.offset(1 as isize)).wrapping_add(
                    (*(*regchunkec8counts.offset(indx as isize)).offset(i as isize)).wrapping_add(
                        *(*regchunkec4counts.offset(indx as isize)).offset(i as isize),
                    ),
                );
            } else if i > gequiv as uint32_t {
                *counters.offset(4 as isize) = (*counters.offset(4 as isize))
                    .wrapping_add(*(*regchunkcopycounts.offset(indx as isize)).offset(i as isize));
                *counters.offset(5 as isize) = (*counters.offset(5 as isize)).wrapping_add(
                    (*(*regchunkec8counts.offset(indx as isize)).offset(i as isize)).wrapping_add(
                        *(*regchunkec4counts.offset(indx as isize)).offset(i as isize),
                    ),
                );
            } else {
                *counters.offset(2 as isize) = (*counters.offset(2 as isize))
                    .wrapping_add(*(*regchunkcopycounts.offset(indx as isize)).offset(i as isize));
                *counters.offset(3 as isize) = (*counters.offset(3 as isize)).wrapping_add(
                    (*(*regchunkec8counts.offset(indx as isize)).offset(i as isize)).wrapping_add(
                        *(*regchunkec4counts.offset(indx as isize)).offset(i as isize),
                    ),
                );
            }
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_sclass_has_chunks(mut sclassid: uint8_t) -> uint8_t {
    unsafe {
        let mut indx: uint16_t = 0;
        let mut i: uint32_t = 0;
        let mut gr: uint8_t = 0;
        gr = 0 as uint8_t;
        while (gr as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
            indx = (sclassid as uint16_t as ::core::ffi::c_int
                + MAXSCLASS * gr as uint16_t as ::core::ffi::c_int) as uint16_t;
            i = 0 as uint32_t;
            while i < 11 as uint32_t {
                if *(*allchunkcopycounts.offset(indx as isize)).offset(i as isize)
                    | *(*allchunkec8counts.offset(indx as isize)).offset(i as isize)
                    | *(*allchunkec4counts.offset(indx as isize)).offset(i as isize)
                    != 0
                {
                    return 1 as uint8_t;
                }
                i = i.wrapping_add(1);
            }
            gr = gr.wrapping_add(1);
        }
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_info(
    mut allchunks: *mut uint32_t,
    mut copychunks: *mut uint32_t,
    mut ec8chunks: *mut uint32_t,
    mut ec4chunks: *mut uint32_t,
    mut copies: *mut uint64_t,
    mut ec8parts: *mut uint64_t,
    mut ec4parts: *mut uint64_t,
    mut hypotheticalcopies: *mut uint64_t,
) {
    unsafe {
        let mut i: uint32_t = 0;
        let mut j: uint32_t = 0;
        let mut avc: uint32_t = 0;
        let mut aec8: uint32_t = 0;
        let mut aec4: uint32_t = 0;
        *allchunks = chunks;
        *copychunks = 0 as uint32_t;
        *ec8chunks = 0 as uint32_t;
        *ec4chunks = 0 as uint32_t;
        *copies = 0 as uint64_t;
        *ec8parts = 0 as uint64_t;
        *ec4parts = 0 as uint64_t;
        *hypotheticalcopies = 0 as uint64_t;
        i = 1 as uint32_t;
        while i <= 10 as uint32_t {
            avc = 0 as uint32_t;
            aec8 = 0 as uint32_t;
            aec4 = 0 as uint32_t;
            j = 0 as uint32_t;
            while j < (MAXSCLASS * 4 as ::core::ffi::c_int) as uint32_t {
                avc = (avc as uint64_t)
                    .wrapping_add(*(*allchunkcopycounts.offset(j as isize)).offset(i as isize))
                    as uint32_t;
                aec8 = (aec8 as uint64_t)
                    .wrapping_add(*(*allchunkec8counts.offset(j as isize)).offset(i as isize))
                    as uint32_t;
                aec4 = (aec4 as uint64_t)
                    .wrapping_add(*(*allchunkec4counts.offset(j as isize)).offset(i as isize))
                    as uint32_t;
                j = j.wrapping_add(1);
            }
            *copychunks = (*copychunks).wrapping_add(avc);
            *ec8chunks = (*ec8chunks).wrapping_add(aec8);
            *ec4chunks = (*ec4chunks).wrapping_add(aec4);
            *copies = (*copies).wrapping_add(avc.wrapping_mul(i) as uint64_t);
            *ec8parts = (*ec8parts)
                .wrapping_add(aec8.wrapping_mul(i.wrapping_add(7 as uint32_t)) as uint64_t);
            *ec4parts = (*ec4parts)
                .wrapping_add(aec4.wrapping_mul(i.wrapping_add(3 as uint32_t)) as uint64_t);
            *hypotheticalcopies =
                (*hypotheticalcopies).wrapping_add(
                    aec8.wrapping_add(aec4).wrapping_add(avc).wrapping_mul(i) as uint64_t,
                );
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_chart_data(
    mut copychunks: *mut uint64_t,
    mut ec8chunks: *mut uint64_t,
    mut ec4chunks: *mut uint64_t,
    mut regendangered: *mut uint64_t,
    mut regundergoal: *mut uint64_t,
    mut allendangered: *mut uint64_t,
    mut allundergoal: *mut uint64_t,
) {
    unsafe {
        let mut i: uint32_t = 0;
        let mut j: uint32_t = 0;
        let mut gequiv: uint8_t = 0;
        let mut repdisabled: uint8_t = 0;
        *copychunks = 0 as uint64_t;
        *ec8chunks = 0 as uint64_t;
        *ec4chunks = 0 as uint64_t;
        *regendangered = 0 as uint64_t;
        *regundergoal = 0 as uint64_t;
        *allendangered = 0 as uint64_t;
        *allundergoal = 0 as uint64_t;
        i = 0 as uint32_t;
        while i < (MAXSCLASS * 4 as ::core::ffi::c_int) as uint32_t {
            gequiv = sclass_calc_goal_equivalent(sclass_get_keeparch_storagemode(
                i.wrapping_rem(MAXSCLASS as uint32_t) as uint16_t,
                i.wrapping_div(MAXSCLASS as uint32_t) as uint8_t,
            ));
            j = 1 as uint32_t;
            while j <= 10 as uint32_t {
                *copychunks = (*copychunks)
                    .wrapping_add(*(*allchunkcopycounts.offset(i as isize)).offset(j as isize));
                *ec8chunks = (*ec8chunks)
                    .wrapping_add(*(*allchunkec8counts.offset(i as isize)).offset(j as isize));
                *ec4chunks = (*ec4chunks)
                    .wrapping_add(*(*allchunkec4counts.offset(i as isize)).offset(j as isize));
                if j < gequiv as uint32_t {
                    if j == 1 as uint32_t {
                        *regendangered = (*regendangered).wrapping_add(
                            (*(*regchunkcopycounts.offset(i as isize)).offset(j as isize))
                                .wrapping_add(
                                    *(*regchunkec8counts.offset(i as isize)).offset(j as isize),
                                )
                                .wrapping_add(
                                    *(*regchunkec4counts.offset(i as isize)).offset(j as isize),
                                ),
                        );
                        *allendangered = (*allendangered).wrapping_add(
                            (*(*allchunkcopycounts.offset(i as isize)).offset(j as isize))
                                .wrapping_add(
                                    *(*allchunkec8counts.offset(i as isize)).offset(j as isize),
                                )
                                .wrapping_add(
                                    *(*allchunkec4counts.offset(i as isize)).offset(j as isize),
                                ),
                        );
                    } else {
                        *regundergoal = (*regundergoal).wrapping_add(
                            (*(*regchunkcopycounts.offset(i as isize)).offset(j as isize))
                                .wrapping_add(
                                    *(*regchunkec8counts.offset(i as isize)).offset(j as isize),
                                )
                                .wrapping_add(
                                    *(*regchunkec4counts.offset(i as isize)).offset(j as isize),
                                ),
                        );
                        *allundergoal = (*allundergoal).wrapping_add(
                            (*(*allchunkcopycounts.offset(i as isize)).offset(j as isize))
                                .wrapping_add(
                                    *(*allchunkec8counts.offset(i as isize)).offset(j as isize),
                                )
                                .wrapping_add(
                                    *(*allchunkec4counts.offset(i as isize)).offset(j as isize),
                                ),
                        );
                    }
                }
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        repdisabled = 0 as uint8_t;
        if matocsserv_servers_count() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || chunkrehashpos == 0 as uint32_t
        {
            repdisabled = 1 as uint8_t;
        }
        if chunk_counters_in_progress() != 0 {
            repdisabled = 1 as uint8_t;
        }
        if main_start_time().wrapping_add(ReplicationsDelayInit) > main_time() {
            repdisabled = 1 as uint8_t;
        }
        if repdisabled != 0 {
            *regendangered = 0xffffffffffffffff as ::core::ffi::c_ulong as uint64_t;
            *regundergoal = 0xffffffffffffffff as ::core::ffi::c_ulong as uint64_t;
            *allendangered = 0xffffffffffffffff as ::core::ffi::c_ulong as uint64_t;
            *allundergoal = 0xffffffffffffffff as ::core::ffi::c_ulong as uint64_t;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_counters_in_progress() -> uint8_t {
    unsafe {
        return ((if !discservers.is_null() || !discservers_next.is_null() {
            CHUNKSERVERS_DISCONNECTING
        } else {
            0 as ::core::ffi::c_int
        }) | (if csregisterinprogress as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            CHUNKSERVERS_CONNECTING
        } else {
            0 as ::core::ffi::c_int
        }) | matocsserv_receiving_chunks_state() as ::core::ffi::c_int
            & (TRANSFERRING_LOST_CHUNKS | TRANSFERRING_NEW_CHUNKS)) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_store_chunkcounters(
    mut buff: *mut uint8_t,
    mut matrixid: uint8_t,
    mut classid: int16_t,
) {
    unsafe {
        let mut i: uint32_t = 0;
        let mut j: uint32_t = 0;
        let mut gequiv: uint8_t = 0;
        let mut counts: [[uint64_t; 11]; 11] = [[0; 11]; 11];
        let mut sm: *mut storagemode = ::core::ptr::null_mut::<storagemode>();
        i = 0 as uint32_t;
        while i <= 10 as uint32_t {
            j = 0 as uint32_t;
            while j <= 10 as uint32_t {
                counts[i as usize][j as usize] = 0 as uint64_t;
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        if matrixid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            i = 0 as uint32_t;
            while i < (MAXSCLASS * 4 as ::core::ffi::c_int) as uint32_t {
                if (classid as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                    || classid as ::core::ffi::c_int > 255 as ::core::ffi::c_int
                    || classid as uint8_t as uint32_t == i.wrapping_rem(MAXSCLASS as uint32_t)
                {
                    sm = sclass_get_keeparch_storagemode(
                        i.wrapping_rem(MAXSCLASS as uint32_t) as uint16_t,
                        i.wrapping_div(MAXSCLASS as uint32_t) as uint8_t,
                    );
                    gequiv = sclass_calc_goal_equivalent(sm);
                    if gequiv as ::core::ffi::c_int > 10 as ::core::ffi::c_int {
                        gequiv = 10 as uint8_t;
                    }
                    j = 0 as uint32_t;
                    while j <= 10 as uint32_t {
                        counts[gequiv as usize][j as usize] = counts[gequiv as usize][j as usize]
                            .wrapping_add(
                                (*(*allchunkcopycounts.offset(i as isize)).offset(j as isize))
                                    .wrapping_add(
                                        *(*allchunkec8counts.offset(i as isize)).offset(j as isize),
                                    )
                                    .wrapping_add(
                                        *(*allchunkec4counts.offset(i as isize)).offset(j as isize),
                                    ),
                            );
                        j = j.wrapping_add(1);
                    }
                }
                i = i.wrapping_add(1);
            }
        } else if matrixid as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            i = 0 as uint32_t;
            while i < (MAXSCLASS * 4 as ::core::ffi::c_int) as uint32_t {
                if (classid as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                    || classid as ::core::ffi::c_int > 255 as ::core::ffi::c_int
                    || classid as uint8_t as uint32_t == i.wrapping_rem(MAXSCLASS as uint32_t)
                {
                    sm = sclass_get_keeparch_storagemode(
                        i.wrapping_rem(MAXSCLASS as uint32_t) as uint16_t,
                        i.wrapping_div(MAXSCLASS as uint32_t) as uint8_t,
                    );
                    gequiv = sclass_calc_goal_equivalent(sm);
                    if gequiv as ::core::ffi::c_int > 10 as ::core::ffi::c_int {
                        gequiv = 10 as uint8_t;
                    }
                    j = 0 as uint32_t;
                    while j <= 10 as uint32_t {
                        counts[gequiv as usize][j as usize] = counts[gequiv as usize][j as usize]
                            .wrapping_add(
                                (*(*regchunkcopycounts.offset(i as isize)).offset(j as isize))
                                    .wrapping_add(
                                        *(*regchunkec8counts.offset(i as isize)).offset(j as isize),
                                    )
                                    .wrapping_add(
                                        *(*regchunkec4counts.offset(i as isize)).offset(j as isize),
                                    ),
                            );
                        j = j.wrapping_add(1);
                    }
                }
                i = i.wrapping_add(1);
            }
        } else if matrixid as ::core::ffi::c_int >= 2 as ::core::ffi::c_int
            && matrixid as ::core::ffi::c_int <= 7 as ::core::ffi::c_int
        {
            let mut srccounters: *mut *mut uint64_t = ::core::ptr::null_mut::<*mut uint64_t>();
            match matrixid as ::core::ffi::c_int {
                2 => {
                    srccounters = allchunkcopycounts;
                }
                3 => {
                    srccounters = regchunkcopycounts;
                }
                4 => {
                    srccounters = allchunkec8counts;
                }
                5 => {
                    srccounters = regchunkec8counts;
                }
                6 => {
                    srccounters = allchunkec4counts;
                }
                _ => {
                    srccounters = regchunkec4counts;
                }
            }
            i = 0 as uint32_t;
            while i < (MAXSCLASS * 4 as ::core::ffi::c_int) as uint32_t {
                if (classid as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                    || classid as ::core::ffi::c_int > 255 as ::core::ffi::c_int
                    || classid as uint8_t as uint32_t == i.wrapping_rem(MAXSCLASS as uint32_t)
                {
                    sm = sclass_get_keeparch_storagemode(
                        i.wrapping_rem(MAXSCLASS as uint32_t) as uint16_t,
                        i.wrapping_div(MAXSCLASS as uint32_t) as uint8_t,
                    );
                    gequiv = sclass_calc_goal_equivalent(sm);
                    if gequiv as ::core::ffi::c_int > 10 as ::core::ffi::c_int {
                        gequiv = 10 as uint8_t;
                    }
                    j = 0 as uint32_t;
                    while j <= 10 as uint32_t {
                        counts[gequiv as usize][j as usize] = counts[gequiv as usize][j as usize]
                            .wrapping_add(*(*srccounters.offset(i as isize)).offset(j as isize));
                        j = j.wrapping_add(1);
                    }
                }
                i = i.wrapping_add(1);
            }
        }
        i = 0 as uint32_t;
        while i <= 10 as uint32_t {
            j = 0 as uint32_t;
            while j <= 10 as uint32_t {
                put32bit(&raw mut buff, counts[i as usize][j as usize] as uint32_t);
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C" fn chunk_priority_next(mut priority: uint8_t) -> *mut chunk {
    unsafe {
        let mut ce: *mut chq_element = ::core::ptr::null_mut::<chq_element>();
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        ce = chq_queue_head[priority as usize];
        if ce.is_null() {
            return ::core::ptr::null_mut::<chunk>();
        }
        c = (*ce).c;
        chq_queue_head[priority as usize] = (*ce).qnext as *mut chq_element;
        if (*ce).qnext.is_null() {
            chq_queue_tail[priority as usize] = (&raw mut chq_queue_head as *mut *mut chq_element)
                .offset(priority as ::core::ffi::c_int as isize);
        } else {
            (*(*ce).qnext).qprev = (&raw mut chq_queue_head as *mut *mut chq_element)
                .offset(priority as ::core::ffi::c_int as isize)
                as *mut *mut _chq_element;
        }
        *(*ce).hprev = (*ce).hnext;
        if !(*ce).hnext.is_null() {
            (*(*ce).hnext).hprev = (*ce).hprev;
        }
        chq_queue_elements[priority as usize] =
            chq_queue_elements[priority as usize].wrapping_sub(1);
        chq_elements = chq_elements.wrapping_sub(1);
        chq_queue_current_pop_count[priority as usize] =
            chq_queue_current_pop_count[priority as usize].wrapping_add(1);
        chunk_queue_free(ce);
        (*c).set_ondangerlist(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        return c;
    }
}
#[inline]
unsafe extern "C" fn chunk_priority_enqueue(mut priority: uint8_t, mut c: *mut chunk) {
    unsafe {
        let mut ce: *mut chq_element = ::core::ptr::null_mut::<chq_element>();
        let mut hash: uint32_t = ((*c).chunkid & CHUNK_PRIORITY_HASHMASK as uint64_t) as uint32_t;
        let mut i: uint8_t = 0;
        if priority as ::core::ffi::c_int >= DANGER_PRIORITIES {
            return;
        }
        if (*c).ondangerlist() != 0 {
            ce = *chq_hash.offset(hash as isize);
            while !ce.is_null() {
                if (*ce).c == c {
                    if (*ce).priority as ::core::ffi::c_int > priority as ::core::ffi::c_int {
                        *(*ce).qprev = (*ce).qnext;
                        if (*ce).qnext.is_null() {
                            chq_queue_tail[(*ce).priority as usize] =
                                (*ce).qprev as *mut *mut chq_element;
                        } else {
                            (*(*ce).qnext).qprev = (*ce).qprev;
                        }
                        *chq_queue_tail[priority as usize] = ce;
                        (*ce).qnext = ::core::ptr::null_mut::<_chq_element>();
                        (*ce).qprev = chq_queue_tail[priority as usize] as *mut *mut _chq_element;
                        chq_queue_tail[priority as usize] =
                            &raw mut (*ce).qnext as *mut *mut chq_element;
                        chq_queue_elements[(*ce).priority as usize] =
                            chq_queue_elements[(*ce).priority as usize].wrapping_sub(1);
                        chq_queue_elements[priority as usize] =
                            chq_queue_elements[priority as usize].wrapping_add(1);
                        (*ce).priority = priority;
                    }
                    return;
                }
                ce = (*ce).hnext as *mut chq_element;
            }
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"internal structure error: chunk (0x%016lX) has 'ondanger' bit set but it is not on danger list\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                (*c).chunkid,
            );
            (*c).set_ondangerlist(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        if chq_elements >= DangerMaxLeng && chq_queue_elements[priority as usize] > DangerMinLeng {
            let mut cx: *mut chunk = ::core::ptr::null_mut::<chunk>();
            cx = ::core::ptr::null_mut::<chunk>();
            i = (DANGER_PRIORITIES - 1 as ::core::ffi::c_int) as uint8_t;
            while i as ::core::ffi::c_int > priority as ::core::ffi::c_int && cx.is_null() {
                if chq_queue_elements[i as usize] > DangerMinLeng {
                    cx = chunk_priority_next(i);
                }
                i = i.wrapping_sub(1);
            }
            if cx.is_null() {
                return;
            }
        }
        chq_elements = chq_elements.wrapping_add(1);
        chq_queue_elements[priority as usize] =
            chq_queue_elements[priority as usize].wrapping_add(1);
        chq_queue_current_append_count[priority as usize] =
            chq_queue_current_append_count[priority as usize].wrapping_add(1);
        ce = chunk_queue_malloc();
        (*ce).c = c;
        (*ce).priority = priority;
        (*ce).hnext = *chq_hash.offset(hash as isize) as *mut _chq_element;
        (*ce).hprev = chq_hash.offset(hash as isize) as *mut *mut _chq_element;
        if !(*ce).hnext.is_null() {
            (*(*ce).hnext).hprev = &raw mut (*ce).hnext;
        }
        *chq_hash.offset(hash as isize) = ce;
        *chq_queue_tail[priority as usize] = ce;
        (*ce).qnext = ::core::ptr::null_mut::<_chq_element>();
        (*ce).qprev = chq_queue_tail[priority as usize] as *mut *mut _chq_element;
        chq_queue_tail[priority as usize] = &raw mut (*ce).qnext as *mut *mut chq_element;
        (*c).set_ondangerlist(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
}
#[inline]
unsafe extern "C" fn chunk_priority_remove(mut c: *mut chunk) {
    unsafe {
        let mut ce: *mut chq_element = ::core::ptr::null_mut::<chq_element>();
        let mut hash: uint32_t = ((*c).chunkid & CHUNK_PRIORITY_HASHMASK as uint64_t) as uint32_t;
        ce = *chq_hash.offset(hash as isize);
        while !ce.is_null() {
            if (*ce).c == c {
                *(*ce).qprev = (*ce).qnext;
                if (*ce).qnext.is_null() {
                    chq_queue_tail[(*ce).priority as usize] = (*ce).qprev as *mut *mut chq_element;
                } else {
                    (*(*ce).qnext).qprev = (*ce).qprev;
                }
                *(*ce).hprev = (*ce).hnext;
                if !(*ce).hnext.is_null() {
                    (*(*ce).hnext).hprev = (*ce).hprev;
                }
                (*c).set_ondangerlist(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                chq_queue_elements[(*ce).priority as usize] =
                    chq_queue_elements[(*ce).priority as usize].wrapping_sub(1);
                chq_elements = chq_elements.wrapping_sub(1);
                chq_queue_current_remove_count[(*ce).priority as usize] =
                    chq_queue_current_remove_count[(*ce).priority as usize].wrapping_add(1);
                chunk_queue_free(ce);
                return;
            }
            ce = (*ce).hnext as *mut chq_element;
        }
    }
}
#[inline]
unsafe extern "C" fn chunk_priority_not_empty(mut priority: uint8_t) -> uint8_t {
    unsafe {
        if (priority as ::core::ffi::c_int) < DANGER_PRIORITIES {
            return (if !chq_queue_head[priority as usize].is_null() {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
        } else {
            return 0 as uint8_t;
        };
    }
}
#[inline]
unsafe extern "C" fn chunk_priority_get_elements(mut priority: uint8_t) -> uint32_t {
    unsafe {
        if (priority as ::core::ffi::c_int) < DANGER_PRIORITIES {
            return chq_queue_elements[priority as usize];
        } else {
            return 0 as uint32_t;
        };
    }
}
#[inline]
unsafe extern "C" fn chunk_priority_should_block(mut priority: uint8_t) -> uint8_t {
    unsafe {
        if priority as ::core::ffi::c_int == CHUNK_PRIORITY_UNFINISHEDEC
            || priority as ::core::ffi::c_int == CHUNK_PRIORITY_OVERGOAL
        {
            return (if chq_queue_elements[priority as usize] >= 1000 as uint32_t {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
        } else {
            return chunk_priority_not_empty(priority);
        };
    }
}
#[inline]
unsafe extern "C" fn chunk_priority_is_empty(mut priority: uint8_t) -> uint8_t {
    unsafe {
        return (1 as ::core::ffi::c_int - chunk_priority_not_empty(priority) as ::core::ffi::c_int)
            as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn chunk_priority_cleanall() {
    unsafe {
        let mut ce: *mut chq_element = ::core::ptr::null_mut::<chq_element>();
        let mut i: uint32_t = 0;
        i = 0 as uint32_t;
        while i < DANGER_PRIORITIES as uint32_t {
            chq_queue_head[i as usize] = ::core::ptr::null_mut::<chq_element>();
            chq_queue_tail[i as usize] =
                (&raw mut chq_queue_head as *mut *mut chq_element).offset(i as isize);
            chq_queue_elements[i as usize] = 0 as uint32_t;
            chq_queue_last_append_count[i as usize] = 0 as uint32_t;
            chq_queue_current_append_count[i as usize] = 0 as uint32_t;
            chq_queue_last_pop_count[i as usize] = 0 as uint32_t;
            chq_queue_current_pop_count[i as usize] = 0 as uint32_t;
            chq_queue_last_remove_count[i as usize] = 0 as uint32_t;
            chq_queue_current_remove_count[i as usize] = 0 as uint32_t;
            i = i.wrapping_add(1);
        }
        i = 0 as uint32_t;
        while i < CHUNK_PRIORITY_HASHSIZE as uint32_t {
            ce = *chq_hash.offset(i as isize);
            while !ce.is_null() {
                (*(*ce).c).set_ondangerlist(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                ce = (*ce).hnext as *mut chq_element;
            }
            *chq_hash.offset(i as isize) = ::core::ptr::null_mut::<chq_element>();
            i = i.wrapping_add(1);
        }
        chq_elements = 0 as uint32_t;
        chunk_queue_free_all();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_queue_counters_shift() {
    unsafe {
        let mut i: uint32_t = 0;
        i = 0 as uint32_t;
        while i < DANGER_PRIORITIES as uint32_t {
            chq_queue_last_append_count[i as usize] = chq_queue_current_append_count[i as usize];
            chq_queue_current_append_count[i as usize] = 0 as uint32_t;
            chq_queue_last_pop_count[i as usize] = chq_queue_current_pop_count[i as usize];
            chq_queue_current_pop_count[i as usize] = 0 as uint32_t;
            chq_queue_last_remove_count[i as usize] = chq_queue_current_remove_count[i as usize];
            chq_queue_current_remove_count[i as usize] = 0 as uint32_t;
            i = i.wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C" fn chunk_new_copy(mut c: *mut chunk, mut s: *mut slist) {
    unsafe {
        let mut si: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut sp: *mut *mut slist = ::core::ptr::null_mut::<*mut slist>();
        sp = &raw mut (*c).slisthead;
        loop {
            si = *sp;
            if si.is_null() {
                break;
            }
            if ((*si).csid as ::core::ffi::c_int) < (*s).csid as ::core::ffi::c_int {
                sp = &raw mut (*si).next as *mut *mut slist;
            } else {
                if !((*si).csid as ::core::ffi::c_int == (*s).csid as ::core::ffi::c_int
                    && ((*si).ecid as ::core::ffi::c_int) < (*s).ecid as ::core::ffi::c_int)
                {
                    break;
                }
                sp = &raw mut (*si).next as *mut *mut slist;
            }
        }
        (*s).next = *sp as *mut _slist;
        *sp = s;
    }
}
#[inline]
unsafe extern "C" fn chunk_replallowed_servers(
    mut cnt: *mut uint16_t,
    mut mode: uint8_t,
) -> *mut uint16_t {
    unsafe {
        static mut aservers: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
        static mut aservcnt: uint16_t = 0;
        if mode as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            if aservers.is_null() {
                aservers =
                    malloc(::core::mem::size_of::<uint16_t>().wrapping_mul(MAXCSCOUNT as size_t))
                        as *mut uint16_t;
                if aservers.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1944 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"aservers\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1944 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"aservers\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if aservers
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut uint16_t
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1944 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"aservers\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1944 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"aservers\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
            }
            aservcnt = matocsserv_getservers_replallowed(aservers as *mut uint16_t);
        } else if mode as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            if !aservers.is_null() {
                free(aservers as *mut ::core::ffi::c_void);
                aservers = ::core::ptr::null_mut::<uint16_t>();
            }
        }
        if !cnt.is_null() {
            *cnt = aservcnt;
        }
        return aservers;
    }
}
#[inline]
unsafe extern "C" fn chunk_get_labels_mode_for_ec(
    mut sm: *mut storagemode,
    mut sclassid: uint8_t,
) -> uint8_t {
    unsafe {
        let mut labels_mode: uint8_t = 0;
        if (*sm).has_labels as ::core::ffi::c_int != 0 && (*sm).labelscnt as ::core::ffi::c_int != 0
        {
            labels_mode = sclass_get_labels_mode(sclassid as uint16_t, sm);
            if labels_mode as ::core::ffi::c_int != LABELS_MODE_LOOSE {
                if ((*sm).valid_ec_counters as ::core::ffi::c_int)
                    < (*sm).labelscnt as ::core::ffi::c_int
                {
                    matocsserv_recalculate_storagemode_scounts(sm);
                }
            }
        } else {
            labels_mode = LABELS_MODE_LOOSE as uint8_t;
        }
        return labels_mode;
    }
}
#[inline]
unsafe extern "C" fn chunk_check_forcekeep_condidiotns_for_ec(
    mut sm: *mut storagemode,
    mut labels_mode: uint8_t,
    mut has_copies: uint8_t,
    mut has_ec4parts: uint8_t,
    mut has_ec8parts: uint8_t,
    mut replallowed: uint16_t,
    mut allvalid: uint16_t,
) -> uint8_t {
    unsafe {
        let mut usekeep: uint8_t = 0;
        let mut ec_strict_mode: uint8_t = 0;
        let mut ec_data_parts: uint8_t = 0;
        let mut ec_chksum_parts: uint8_t = 0;
        usekeep = 0 as uint8_t;
        ec_strict_mode = 0 as uint8_t;
        ec_data_parts = ((*sm).ec_data_chksum_parts as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int) as uint8_t;
        ec_chksum_parts = ((*sm).ec_data_chksum_parts as ::core::ffi::c_int
            & 0xf as ::core::ffi::c_int) as uint8_t;
        if ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int
            && has_ec4parts as ::core::ffi::c_int != 0
        {
            usekeep = 1 as uint8_t;
        } else if ec_data_parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int
            && has_ec8parts as ::core::ffi::c_int != 0
        {
            usekeep = 1 as uint8_t;
        } else if ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int
            || ec_data_parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int
        {
            match labels_mode as ::core::ffi::c_int {
                LABELS_MODE_LOOSE => {
                    if has_copies != 0 {
                        if (replallowed as ::core::ffi::c_int)
                            < ec_data_parts as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * ec_chksum_parts as ::core::ffi::c_int
                        {
                            usekeep = 2 as uint8_t;
                        }
                    } else if (allvalid as ::core::ffi::c_int)
                        < ec_data_parts as ::core::ffi::c_int
                            + ec_chksum_parts as ::core::ffi::c_int
                    {
                        usekeep = 3 as uint8_t;
                    }
                }
                LABELS_MODE_STD => {
                    if has_copies != 0 {
                        if ((*sm).replallowed as ::core::ffi::c_int)
                            < ec_data_parts as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * ec_chksum_parts as ::core::ffi::c_int
                        {
                            usekeep = 2 as uint8_t;
                        }
                        if (*sm).labelscnt as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                            if ((*sm).data_replallowed as ::core::ffi::c_int
                                + (*sm).both_replallowed as ::core::ffi::c_int)
                                < ec_data_parts as ::core::ffi::c_int
                                    + ec_chksum_parts as ::core::ffi::c_int
                            {
                                usekeep = 2 as uint8_t;
                            }
                            if ((*sm).chksum_replallowed as ::core::ffi::c_int
                                + (*sm).both_replallowed as ::core::ffi::c_int)
                                < 2 as ::core::ffi::c_int * ec_chksum_parts as ::core::ffi::c_int
                            {
                                usekeep = 2 as uint8_t;
                            }
                        }
                    } else {
                        if (allvalid as ::core::ffi::c_int)
                            < ec_data_parts as ::core::ffi::c_int
                                + ec_chksum_parts as ::core::ffi::c_int
                        {
                            usekeep = 3 as uint8_t;
                        }
                        if (*sm).labelscnt as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                            if (*sm).replallowed as ::core::ffi::c_int
                                >= ec_data_parts as ::core::ffi::c_int
                                    + ec_chksum_parts as ::core::ffi::c_int
                            {
                                ec_strict_mode = 1 as uint8_t;
                            }
                        } else if (*sm).replallowed as ::core::ffi::c_int
                            >= ec_data_parts as ::core::ffi::c_int
                                + ec_chksum_parts as ::core::ffi::c_int
                            && (*sm).data_replallowed as ::core::ffi::c_int
                                + (*sm).both_replallowed as ::core::ffi::c_int
                                >= ec_data_parts as ::core::ffi::c_int
                            && (*sm).chksum_replallowed as ::core::ffi::c_int
                                + (*sm).both_replallowed as ::core::ffi::c_int
                                >= ec_chksum_parts as ::core::ffi::c_int
                        {
                            ec_strict_mode = 1 as uint8_t;
                        }
                    }
                }
                LABELS_MODE_STRICT => {
                    if has_copies != 0 {
                        if ((*sm).replallowed as ::core::ffi::c_int)
                            < ec_data_parts as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * ec_chksum_parts as ::core::ffi::c_int
                        {
                            usekeep = 2 as uint8_t;
                        }
                        if (*sm).labelscnt as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                            if ((*sm).data_replallowed as ::core::ffi::c_int
                                + (*sm).both_replallowed as ::core::ffi::c_int)
                                < ec_data_parts as ::core::ffi::c_int
                                    + ec_chksum_parts as ::core::ffi::c_int
                            {
                                usekeep = 2 as uint8_t;
                            }
                            if ((*sm).chksum_replallowed as ::core::ffi::c_int
                                + (*sm).both_replallowed as ::core::ffi::c_int)
                                < 2 as ::core::ffi::c_int * ec_chksum_parts as ::core::ffi::c_int
                            {
                                usekeep = 2 as uint8_t;
                            }
                        }
                    } else {
                        if ((*sm).allvalid as ::core::ffi::c_int)
                            < ec_data_parts as ::core::ffi::c_int
                                + ec_chksum_parts as ::core::ffi::c_int
                        {
                            usekeep = 3 as uint8_t;
                        }
                        if (*sm).labelscnt as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                            if ((*sm).data_allvalid as ::core::ffi::c_int
                                + (*sm).both_allvalid as ::core::ffi::c_int)
                                < ec_data_parts as ::core::ffi::c_int
                            {
                                usekeep = 3 as uint8_t;
                            }
                            if ((*sm).chksum_allvalid as ::core::ffi::c_int
                                + (*sm).both_allvalid as ::core::ffi::c_int)
                                < ec_chksum_parts as ::core::ffi::c_int
                            {
                                usekeep = 3 as uint8_t;
                            }
                        }
                    }
                    ec_strict_mode = 1 as uint8_t;
                }
                _ => {}
            }
        } else {
            usekeep = 1 as uint8_t;
        }
        return (((ec_strict_mode as ::core::ffi::c_int) << 4 as ::core::ffi::c_int)
            + usekeep as ::core::ffi::c_int) as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn chunk_calculate_endanger_priority(
    mut c: *mut chunk,
    mut checklabels: uint8_t,
) -> uint8_t {
    unsafe {
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut goal: uint8_t = 0;
        let mut ec_data_parts: uint8_t = 0;
        let mut ec_chksum_parts: uint8_t = 0;
        let mut j: uint8_t = 0;
        let mut m: uint32_t = 0;
        static mut servers: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
        let mut aservers: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
        let mut aservcnt: uint16_t = 0;
        let mut scount: uint16_t = 0;
        let mut sm: *mut storagemode = ::core::ptr::null_mut::<storagemode>();
        let mut labels_mode: uint8_t = 0;
        let mut vcmask8: uint32_t = 0;
        let mut vcmask4: uint32_t = 0;
        let mut ecmask8: uint32_t = 0;
        let mut ecmask4: uint32_t = 0;
        let mut partmask: uint32_t = 0;
        let mut lcsid4: int32_t = 0;
        let mut lcsid8: int32_t = 0;
        let mut extra_chunks: uint8_t = 0;
        let mut wrong_repairable_labels: uint8_t = 0;
        let mut redundant_ec_parts: uint8_t = 0;
        let mut parts_on_the_same_server: uint8_t = 0;
        let mut server_count_with_parts8: uint8_t = 0;
        let mut server_count_with_parts4: uint8_t = 0;
        let mut hasvc: uint8_t = 0;
        let mut hasec8parts: uint8_t = 0;
        let mut hasec4parts: uint8_t = 0;
        if c.is_null() {
            if checklabels as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if servers.is_null() {
                    servers = malloc(
                        ::core::mem::size_of::<uint16_t>().wrapping_mul(MAXCSCOUNT as size_t),
                    ) as *mut uint16_t;
                    if servers.is_null() {
                        fprintf(
                            stderr,
                            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            2126 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"servers\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            2126 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"servers\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        abort();
                    } else if servers
                        == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                            -1 as ::core::ffi::c_int as usize,
                        ) as *mut uint16_t
                    {
                        let mut _mfs_errorstring: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            2126 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"servers\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            2126 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"servers\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring,
                        );
                        abort();
                    }
                }
            } else if !servers.is_null() {
                free(servers as *mut ::core::ffi::c_void);
                servers = ::core::ptr::null_mut::<uint16_t>();
            }
            return 0xff as uint8_t;
        }
        extra_chunks = 0 as uint8_t;
        parts_on_the_same_server = 0 as uint8_t;
        wrong_repairable_labels = 0 as uint8_t;
        redundant_ec_parts = 0 as uint8_t;
        server_count_with_parts8 = 0 as uint8_t;
        server_count_with_parts4 = 0 as uint8_t;
        scount = matocsserv_servers_count();
        aservers = chunk_replallowed_servers(&raw mut aservcnt, 0 as uint8_t);
        hasvc = 0 as uint8_t;
        hasec4parts = 0 as uint8_t;
        hasec8parts = 0 as uint8_t;
        s = (*c).slisthead;
        while !s.is_null() {
            if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int {
                if (*s).ecid as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int != 0 {
                    hasec8parts = 1 as uint8_t;
                } else if (*s).ecid as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int != 0 {
                    hasec4parts = 1 as uint8_t;
                } else {
                    hasvc = 1 as uint8_t;
                }
            }
            s = (*s).next as *mut slist;
        }
        sm = sclass_get_keeparch_storagemode((*c).sclassid as uint16_t, (*c).flags() as uint8_t);
        if (*sm).ec_data_chksum_parts != 0 {
            let mut usekeep: uint8_t = 0;
            labels_mode = chunk_get_labels_mode_for_ec(sm, (*c).sclassid);
            usekeep = (chunk_check_forcekeep_condidiotns_for_ec(
                sm,
                labels_mode,
                hasvc,
                hasec4parts,
                hasec8parts,
                aservcnt,
                scount,
            ) as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int) as uint8_t;
            if usekeep != 0 {
                sm = sclass_get_keeparch_storagemode((*c).sclassid as uint16_t, 0 as uint8_t);
            }
        }
        if (*sm).ec_data_chksum_parts != 0 {
            ec_data_parts = ((*sm).ec_data_chksum_parts as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int) as uint8_t;
            ec_chksum_parts = ((*sm).ec_data_chksum_parts as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int) as uint8_t;
            goal = (ec_chksum_parts as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t;
        } else {
            ec_data_parts = 0 as uint8_t;
            ec_chksum_parts = 0 as uint8_t;
            goal = (*sm).labelscnt;
        }
        if ec_data_parts != 0 {
            if (aservcnt as ::core::ffi::c_int) < ec_data_parts as ::core::ffi::c_int {
                return DANGER_PRIORITIES as uint8_t;
            }
            if goal as ::core::ffi::c_int
                + (ec_data_parts as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                > aservcnt as ::core::ffi::c_int
            {
                goal = (aservcnt as ::core::ffi::c_int
                    - (ec_data_parts as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                    as uint8_t;
            }
        } else if goal as ::core::ffi::c_int > aservcnt as ::core::ffi::c_int {
            goal = aservcnt as uint8_t;
        }
        vcmask8 = 0 as uint32_t;
        vcmask4 = 0 as uint32_t;
        ecmask8 = 0 as uint32_t;
        ecmask4 = 0 as uint32_t;
        lcsid4 = -1 as ::core::ffi::c_int as int32_t;
        lcsid8 = -1 as ::core::ffi::c_int as int32_t;
        s = (*c).slisthead;
        while !s.is_null() {
            if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int {
                if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && ec_data_parts as ::core::ffi::c_int != 0
                    || (*s).ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
                        && ec_data_parts as ::core::ffi::c_int != 4 as ::core::ffi::c_int
                    || (*s).ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x30 as ::core::ffi::c_int
                        && ec_data_parts as ::core::ffi::c_int != 8 as ::core::ffi::c_int
                {
                    extra_chunks = 1 as uint8_t;
                }
                if (*s).ecid as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    if (*s).ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
                    {
                        partmask = ((1 as ::core::ffi::c_uint)
                            << ((*s).ecid as ::core::ffi::c_int & 0xf as ::core::ffi::c_int))
                            as uint32_t;
                        vcmask4 |= partmask;
                        if partmask & ecmask4 != 0 {
                            redundant_ec_parts = 1 as uint8_t;
                        } else if lcsid4 != (*s).csid as int32_t {
                            server_count_with_parts4 = server_count_with_parts4.wrapping_add(1);
                        }
                        ecmask4 |= partmask;
                        if lcsid4 == (*s).csid as int32_t {
                            parts_on_the_same_server = 1 as uint8_t;
                        }
                        lcsid4 = (*s).csid as int32_t;
                    } else if (*s).ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x30 as ::core::ffi::c_int
                    {
                        partmask = ((1 as ::core::ffi::c_uint)
                            << ((*s).ecid as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
                            as uint32_t;
                        vcmask8 |= partmask;
                        if partmask & ecmask8 != 0 {
                            redundant_ec_parts = 1 as uint8_t;
                        } else if lcsid8 != (*s).csid as int32_t {
                            server_count_with_parts8 = server_count_with_parts8.wrapping_add(1);
                        }
                        ecmask8 |= partmask;
                        if lcsid8 == (*s).csid as int32_t {
                            parts_on_the_same_server = 1 as uint8_t;
                        }
                        lcsid8 = (*s).csid as int32_t;
                    }
                }
            }
            s = (*s).next as *mut slist;
        }
        if extra_chunks as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && parts_on_the_same_server as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && checklabels as ::core::ffi::c_int != 0
            && (*c).reg_gequiv() as ::core::ffi::c_int >= goal as ::core::ffi::c_int
            && ((*sm).has_labels as ::core::ffi::c_int != 0
                || DoNotUseSameIP as ::core::ffi::c_int != 0
                || DoNotUseSameRack as ::core::ffi::c_int != 0)
        {
            if ec_data_parts != 0 {
                let mut checkindex: uint8_t = 0;
                let mut need_l0: uint8_t = 0;
                let mut need_l1: uint8_t = 0;
                let mut minecid: uint8_t = 0;
                let mut maxecid: uint8_t = 0;
                let mut ecidmask: uint8_t = 0;
                let mut eciddata: uint8_t = 0;
                if ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                    minecid = 0x20 as uint8_t;
                    maxecid = 0x30 as uint8_t;
                    ecidmask = 0x1f as uint8_t;
                    eciddata = 8 as uint8_t;
                } else {
                    minecid = 0x10 as uint8_t;
                    maxecid = 0x1c as uint8_t;
                    ecidmask = 0xf as uint8_t;
                    eciddata = 4 as uint8_t;
                }
                if (*sm).labelscnt as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    checkindex = 0 as uint8_t;
                } else {
                    checkindex = 1 as uint8_t;
                }
                need_l0 = 0 as uint8_t;
                need_l1 = 0 as uint8_t;
                s = (*c).slisthead;
                while !s.is_null() {
                    if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int >= minecid as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= maxecid as ::core::ffi::c_int
                    {
                        if ((*s).ecid as ::core::ffi::c_int & ecidmask as ::core::ffi::c_int)
                            < eciddata as ::core::ffi::c_int
                        {
                            if matocsserv_server_matches_labelexpr(
                                (*cstab.offset((*s).csid as isize)).ptr,
                                &raw mut *(&raw mut (*sm).labelexpr as *mut [uint8_t; 128])
                                    .offset(0 as isize)
                                    as *mut uint8_t
                                    as *const uint8_t,
                            ) as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                need_l0 = need_l0.wrapping_add(1);
                            }
                        } else if matocsserv_server_matches_labelexpr(
                            (*cstab.offset((*s).csid as isize)).ptr,
                            &raw mut *(&raw mut (*sm).labelexpr as *mut [uint8_t; 128])
                                .offset(checkindex as isize)
                                as *mut uint8_t as *const uint8_t,
                        ) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            if checkindex as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                                need_l0 = need_l0.wrapping_add(1);
                            } else {
                                need_l1 = need_l1.wrapping_add(1);
                            }
                        }
                    }
                    s = (*s).next as *mut slist;
                }
                if need_l0 as ::core::ffi::c_int | need_l1 as ::core::ffi::c_int != 0 {
                    m = 0 as uint32_t;
                    while m < aservcnt as uint32_t
                        && wrong_repairable_labels as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        s = (*c).slisthead;
                        while !s.is_null() {
                            if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                                && (*s).ecid as ::core::ffi::c_int >= minecid as ::core::ffi::c_int
                                && (*s).ecid as ::core::ffi::c_int <= maxecid as ::core::ffi::c_int
                                && (*s).csid as ::core::ffi::c_int
                                    == *aservers.offset(m as isize) as ::core::ffi::c_int
                            {
                                break;
                            }
                            s = (*s).next as *mut slist;
                        }
                        if s.is_null() {
                            if need_l0 != 0 {
                                if matocsserv_server_matches_labelexpr(
                                    (*cstab.offset(*aservers.offset(m as isize) as isize)).ptr,
                                    &raw mut *(&raw mut (*sm).labelexpr as *mut [uint8_t; 128])
                                        .offset(0 as isize)
                                        as *mut uint8_t
                                        as *const uint8_t,
                                ) != 0
                                {
                                    wrong_repairable_labels = 1 as uint8_t;
                                }
                            }
                            if need_l1 != 0 {
                                if matocsserv_server_matches_labelexpr(
                                    (*cstab.offset(*aservers.offset(m as isize) as isize)).ptr,
                                    &raw mut *(&raw mut (*sm).labelexpr as *mut [uint8_t; 128])
                                        .offset(1 as isize)
                                        as *mut uint8_t
                                        as *const uint8_t,
                                ) != 0
                                {
                                    wrong_repairable_labels = 1 as uint8_t;
                                }
                            }
                        }
                        m = m.wrapping_add(1);
                    }
                }
            } else {
                let mut servcnt: uint32_t = 0;
                let mut matching: *mut int32_t = ::core::ptr::null_mut::<int32_t>();
                let mut vcms: uint32_t = 0;
                let mut scms: uint32_t = 0;
                servcnt = 0 as uint32_t;
                s = (*c).slisthead;
                while !s.is_null() {
                    if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        let c2rust_fresh29 = servcnt;
                        servcnt = servcnt.wrapping_add(1);
                        *servers.offset(c2rust_fresh29 as isize) = (*s).csid;
                    }
                    s = (*s).next as *mut slist;
                }
                matching = do_advanced_match(sm, servcnt, servers);
                vcms = 0 as uint32_t;
                j = 0 as uint8_t;
                while (j as ::core::ffi::c_int) < (*sm).labelscnt as ::core::ffi::c_int {
                    if *matching.offset(j as isize) >= 0 as int32_t {
                        vcms = vcms.wrapping_add(1);
                    }
                    j = j.wrapping_add(1);
                }
                if vcms <= (*sm).labelscnt as uint32_t {
                    scms = (*sm).matching_servers as uint32_t;
                    if vcms < scms {
                        wrong_repairable_labels = 1 as uint8_t;
                    } else {
                        m = 0 as uint32_t;
                        while m < aservcnt as uint32_t {
                            s = (*c).slisthead;
                            while !s.is_null() {
                                if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                                    && (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                    && (*s).csid as ::core::ffi::c_int
                                        == *aservers.offset(m as isize) as ::core::ffi::c_int
                                {
                                    break;
                                }
                                s = (*s).next as *mut slist;
                            }
                            if s.is_null() {
                                let c2rust_fresh30 = servcnt;
                                servcnt = servcnt.wrapping_add(1);
                                *servers.offset(c2rust_fresh30 as isize) =
                                    *aservers.offset(m as isize);
                            }
                            m = m.wrapping_add(1);
                        }
                        if servcnt != aservcnt as uint32_t {
                            matching = do_advanced_match(sm, servcnt, servers);
                            scms = 0 as uint32_t;
                            j = 0 as uint8_t;
                            while (j as ::core::ffi::c_int) < (*sm).labelscnt as ::core::ffi::c_int
                            {
                                if *matching.offset(j as isize) >= 0 as int32_t {
                                    scms = scms.wrapping_add(1);
                                }
                                j = j.wrapping_add(1);
                            }
                            if vcms < scms {
                                wrong_repairable_labels = 1 as uint8_t;
                            }
                        }
                    }
                }
            }
        }
        if (*c).all_gequiv() as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            if (*c).all_gequiv() as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                && goal as ::core::ffi::c_int > 2 as ::core::ffi::c_int
            {
                return CHUNK_PRIORITY_ONECOPY_HIGHGOAL as uint8_t;
            } else if (*c).all_gequiv() as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                && goal as ::core::ffi::c_int == 2 as ::core::ffi::c_int
            {
                return CHUNK_PRIORITY_ONECOPY_ANY as uint8_t;
            } else if (*c).reg_gequiv() as ::core::ffi::c_int <= 1 as ::core::ffi::c_int
                && (*c).all_gequiv() as ::core::ffi::c_int > (*c).reg_gequiv() as ::core::ffi::c_int
            {
                return CHUNK_PRIORITY_ONEREGCOPY_PLUSMFR as uint8_t;
            } else if (*c).all_gequiv() as ::core::ffi::c_int
                > (*c).reg_gequiv() as ::core::ffi::c_int
                && ((*c).reg_gequiv() as ::core::ffi::c_int) < goal as ::core::ffi::c_int
            {
                return CHUNK_PRIORITY_MARKEDFORREMOVAL as uint8_t;
            } else if (hasec8parts as ::core::ffi::c_int != 0
                || hasec4parts as ::core::ffi::c_int != 0)
                && hasvc as ::core::ffi::c_int != 0
            {
                return CHUNK_PRIORITY_UNFINISHEDEC as uint8_t;
            } else if hasec8parts as ::core::ffi::c_int != 0
                && hasec4parts as ::core::ffi::c_int != 0
            {
                return CHUNK_PRIORITY_UNFINISHEDEC as uint8_t;
            } else if ((*c).reg_gequiv() as ::core::ffi::c_int) < goal as ::core::ffi::c_int {
                return CHUNK_PRIORITY_UNDERGOAL as uint8_t;
            } else if ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int
                && (hasec4parts as ::core::ffi::c_int != 0 || hasvc as ::core::ffi::c_int != 0)
            {
                return CHUNK_PRIORITY_UNDERGOAL as uint8_t;
            } else if ec_data_parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int
                && (hasec8parts as ::core::ffi::c_int != 0 || hasvc as ::core::ffi::c_int != 0)
            {
                return CHUNK_PRIORITY_UNDERGOAL as uint8_t;
            } else if ec_data_parts as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && (hasec4parts as ::core::ffi::c_int != 0
                    || hasec8parts as ::core::ffi::c_int != 0)
            {
                return CHUNK_PRIORITY_UNDERGOAL as uint8_t;
            } else if ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int
                && (server_count_with_parts8 as ::core::ffi::c_int)
                    < goal as ::core::ffi::c_int + 7 as ::core::ffi::c_int
            {
                return CHUNK_PRIORITY_UNDERGOAL as uint8_t;
            } else if ec_data_parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int
                && (server_count_with_parts4 as ::core::ffi::c_int)
                    < goal as ::core::ffi::c_int + 3 as ::core::ffi::c_int
            {
                return CHUNK_PRIORITY_UNDERGOAL as uint8_t;
            } else if vcmask8 & 0xff as uint32_t != 0xff as uint32_t
                && parts_on_the_same_server as ::core::ffi::c_int != 0
            {
                return CHUNK_PRIORITY_UNDERGOAL as uint8_t;
            } else if vcmask4 & 0xf as uint32_t != 0xf as uint32_t
                && parts_on_the_same_server as ::core::ffi::c_int != 0
            {
                return CHUNK_PRIORITY_UNDERGOAL as uint8_t;
            } else if (*c).reg_gequiv() as ::core::ffi::c_int > goal as ::core::ffi::c_int
                || redundant_ec_parts as ::core::ffi::c_int != 0
                || parts_on_the_same_server as ::core::ffi::c_int != 0
            {
                return CHUNK_PRIORITY_OVERGOAL as uint8_t;
            } else if wrong_repairable_labels != 0 {
                return CHUNK_PRIORITY_WRONGLABELS as uint8_t;
            }
        }
        return DANGER_PRIORITIES as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn chunk_priority_queue_check(mut c: *mut chunk, mut checklabels: uint8_t) {
    unsafe {
        let mut j: uint8_t = 0;
        if (*c).sclassid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*c).fhead == FLISTNULLINDX as uint32_t
            || (*c).lockedto >= main_time().wrapping_add(3600 as uint32_t)
        {
            return;
        }
        j = chunk_calculate_endanger_priority(c, checklabels);
        if (j as ::core::ffi::c_int) < DANGER_PRIORITIES {
            chunk_priority_enqueue(j, c);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_addopchunk(mut csid: uint16_t, mut chunkid: uint64_t) {
    unsafe {
        let mut csop: *mut csopchunk = ::core::ptr::null_mut::<csopchunk>();
        csop = malloc(::core::mem::size_of::<csopchunk>()) as *mut csopchunk;
        (*csop).chunkid = chunkid;
        (*csop).status = MFS_ERROR_MISMATCH as uint8_t;
        (*csop).next = (*cstab.offset(csid as isize)).opchunks as *mut _csopchunk;
        (*cstab.offset(csid as isize)).opchunks = csop;
        opsinprogress = opsinprogress.wrapping_add(1);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_statusopchunk(
    mut csid: uint16_t,
    mut chunkid: uint64_t,
    mut status: uint8_t,
) {
    unsafe {
        let mut csop: *mut csopchunk = ::core::ptr::null_mut::<csopchunk>();
        csop = (*cstab.offset(csid as isize)).opchunks;
        while !csop.is_null() {
            if (*csop).chunkid == chunkid {
                (*csop).status = status;
                return;
            }
            csop = (*csop).next as *mut csopchunk;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_delopchunk(mut csid: uint16_t, mut chunkid: uint64_t) -> uint8_t {
    unsafe {
        let mut csopp: *mut *mut csopchunk = ::core::ptr::null_mut::<*mut csopchunk>();
        let mut csop: *mut csopchunk = ::core::ptr::null_mut::<csopchunk>();
        let mut status: uint8_t = 0;
        status = MFS_ERROR_MISMATCH as uint8_t;
        csopp = &raw mut (*cstab.offset(csid as isize)).opchunks;
        loop {
            csop = *csopp;
            if csop.is_null() {
                break;
            }
            if (*csop).chunkid == chunkid {
                status = (*csop).status;
                *csopp = (*csop).next as *mut csopchunk;
                free(csop as *mut ::core::ffi::c_void);
                if opsinprogress > 0 as uint32_t {
                    opsinprogress = opsinprogress.wrapping_sub(1);
                }
            } else {
                csopp = &raw mut (*csop).next as *mut *mut csopchunk;
            }
        }
        return status;
    }
}
#[inline]
unsafe extern "C" fn chunk_creation_servers(
    mut csids: *mut uint16_t,
    mut sm: *mut storagemode,
    mut labels_mode: uint8_t,
    mut olflag: *mut uint8_t,
    mut clientip: uint32_t,
) -> uint16_t {
    unsafe {
        let mut tmpcsids: [uint16_t; 10000] = [0; 10000];
        let mut matching: *mut int32_t = ::core::ptr::null_mut::<int32_t>();
        let mut servcount: uint16_t = 0;
        let mut goodlabelscount: uint16_t = 0;
        let mut overloaded: uint16_t = 0;
        let mut i: int32_t = 0;
        let mut j: int32_t = 0;
        let mut cpos: uint16_t = 0;
        let mut fpos: uint16_t = 0;
        let mut dist: uint32_t = 0;
        let mut x: int32_t = 0;
        servcount = matocsserv_getservers_wrandom(csids, &raw mut overloaded);
        if servcount as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            *olflag = (if overloaded as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            return 0 as uint16_t;
        }
        if (servcount as ::core::ffi::c_int) < (*sm).labelscnt as ::core::ffi::c_int
            && servcount as ::core::ffi::c_int + overloaded as ::core::ffi::c_int
                >= (*sm).labelscnt as ::core::ffi::c_int
        {
            *olflag = 1 as uint8_t;
            return 0 as uint16_t;
        } else {
            *olflag = 0 as uint8_t;
        }
        if CreationsRespectTopology > 0 as uint32_t {
            cpos = 0 as uint16_t;
            fpos = MAXCSCOUNT as uint16_t;
            i = 0 as ::core::ffi::c_int as int32_t;
            while i < servcount as int32_t {
                dist = topology_distance(
                    matocsserv_server_get_ip(
                        (*cstab.offset(*csids.offset(i as isize) as isize)).ptr,
                    ),
                    clientip,
                ) as uint32_t;
                if dist < CreationsRespectTopology {
                    let c2rust_fresh31 = cpos;
                    cpos = cpos.wrapping_add(1);
                    tmpcsids[c2rust_fresh31 as usize] = *csids.offset(i as isize);
                } else {
                    fpos = fpos.wrapping_sub(1);
                    tmpcsids[fpos as usize] = *csids.offset(i as isize);
                }
                i += 1;
            }
            if cpos as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                && fpos as ::core::ffi::c_int != MAXCSCOUNT
            {
                i = 0 as ::core::ffi::c_int as int32_t;
                while i < cpos as int32_t {
                    *csids.offset(i as isize) = tmpcsids[i as usize];
                    i += 1;
                }
                i = fpos as int32_t;
                j = (servcount as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int32_t;
                while i < MAXCSCOUNT as int32_t {
                    *csids.offset(j as isize) = tmpcsids[i as usize];
                    i += 1;
                    j -= 1;
                }
            }
        }
        if (*sm).has_labels as ::core::ffi::c_int != 0
            || DoNotUseSameIP as ::core::ffi::c_int != 0
            || DoNotUseSameRack as ::core::ffi::c_int != 0
        {
            matching = do_advanced_match(sm, servcount as uint32_t, csids as *const uint16_t);
            if labels_mode as ::core::ffi::c_int != LABELS_MODE_STRICT {
                goodlabelscount = do_extend_match(sm, servcount as uint32_t, matching);
                if labels_mode as ::core::ffi::c_int == LABELS_MODE_STD {
                    if (goodlabelscount as ::core::ffi::c_int)
                        < (*sm).labelscnt as ::core::ffi::c_int
                        && goodlabelscount as ::core::ffi::c_int + overloaded as ::core::ffi::c_int
                            >= (*sm).labelscnt as ::core::ffi::c_int
                    {
                        *olflag = 1 as uint8_t;
                        return 0 as uint16_t;
                    }
                }
            }
            i = 0 as ::core::ffi::c_int as int32_t;
            j = (servcount as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int32_t;
            while i <= j {
                while i <= j
                    && *matching.offset(((*sm).labelscnt as int32_t + i) as isize) >= 0 as int32_t
                {
                    i += 1;
                }
                while i <= j
                    && *matching.offset(((*sm).labelscnt as int32_t + j) as isize) < 0 as int32_t
                {
                    j -= 1;
                }
                if i < j {
                    x = *matching.offset(((*sm).labelscnt as int32_t + i) as isize);
                    *matching.offset(((*sm).labelscnt as int32_t + i) as isize) =
                        *matching.offset(((*sm).labelscnt as int32_t + j) as isize);
                    *matching.offset(((*sm).labelscnt as int32_t + j) as isize) = x;
                    x = *csids.offset(i as isize) as int32_t;
                    *csids.offset(i as isize) = *csids.offset(j as isize);
                    *csids.offset(j as isize) = x as uint16_t;
                }
            }
            if labels_mode as ::core::ffi::c_int == LABELS_MODE_STRICT {
                if i < (*sm).labelscnt as int32_t
                    && i + overloaded as int32_t >= (*sm).labelscnt as int32_t
                {
                    *olflag = 1 as uint8_t;
                    return 0 as uint16_t;
                }
                return i as uint16_t;
            }
        }
        return servcount;
    }
}
static mut io_ready_chunk_used: uint64_t = 0 as uint64_t;
#[inline]
unsafe extern "C" fn io_ready_chunk_free_all() {
    unsafe {
        let mut srb: *mut io_ready_chunk_bucket = ::core::ptr::null_mut::<io_ready_chunk_bucket>();
        let mut nsrb: *mut io_ready_chunk_bucket = ::core::ptr::null_mut::<io_ready_chunk_bucket>();
        srb = io_ready_chunk_buckets_head;
        while !srb.is_null() {
            nsrb = (*srb).next as *mut io_ready_chunk_bucket;
            munmap(
                srb as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<io_ready_chunk_bucket>(),
            );
            srb = nsrb;
        }
        io_ready_chunk_buckets_head = ::core::ptr::null_mut::<io_ready_chunk_bucket>();
        io_ready_chunk_free_head = NULL;
        io_ready_chunk_allocated = 0 as uint64_t;
        io_ready_chunk_used = 0 as uint64_t;
    }
}
#[inline]
unsafe extern "C" fn io_ready_chunk_free(mut p: *mut io_ready_chunk) {
    unsafe {
        *(p as *mut *mut ::core::ffi::c_void) = io_ready_chunk_free_head;
        io_ready_chunk_free_head = p as *mut ::core::ffi::c_void;
        io_ready_chunk_used = (io_ready_chunk_used as ::core::ffi::c_ulong)
            .wrapping_sub(::core::mem::size_of::<io_ready_chunk>() as ::core::ffi::c_ulong)
            as uint64_t;
    }
}
static mut io_ready_chunk_allocated: uint64_t = 0 as uint64_t;
#[inline]
unsafe extern "C" fn io_ready_chunk_malloc() -> *mut io_ready_chunk {
    unsafe {
        let mut srb: *mut io_ready_chunk_bucket = ::core::ptr::null_mut::<io_ready_chunk_bucket>();
        let mut ret: *mut io_ready_chunk = ::core::ptr::null_mut::<io_ready_chunk>();
        if !io_ready_chunk_free_head.is_null() {
            ret = io_ready_chunk_free_head as *mut io_ready_chunk;
            io_ready_chunk_free_head = *(ret as *mut *mut ::core::ffi::c_void);
            io_ready_chunk_used = (io_ready_chunk_used as ::core::ffi::c_ulong)
                .wrapping_add(::core::mem::size_of::<io_ready_chunk>() as ::core::ffi::c_ulong)
                as uint64_t;
            return ret;
        }
        if io_ready_chunk_buckets_head.is_null()
            || (*io_ready_chunk_buckets_head).firstfree as usize
                == (10000000 as ::core::ffi::c_int as usize)
                    .wrapping_div(::core::mem::size_of::<io_ready_chunk>())
        {
            srb = mmap(
                NULL,
                ::core::mem::size_of::<io_ready_chunk_bucket>(),
                PROT_READ | PROT_WRITE,
                MAP_ANON | MAP_PRIVATE,
                -1 as ::core::ffi::c_int,
                0 as __off64_t,
            ) as *mut io_ready_chunk_bucket;
            if srb.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2560 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2560 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if srb
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut io_ready_chunk_bucket
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2560 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2560 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"srb\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            (*srb).next = io_ready_chunk_buckets_head as *mut _io_ready_chunk_bucket;
            (*srb).firstfree = 0 as uint32_t;
            io_ready_chunk_buckets_head = srb;
            io_ready_chunk_allocated = (io_ready_chunk_allocated as ::core::ffi::c_ulong)
                .wrapping_add(
                    ::core::mem::size_of::<io_ready_chunk_bucket>() as ::core::ffi::c_ulong
                ) as uint64_t;
        }
        ret = (&raw mut (*io_ready_chunk_buckets_head).bucket as *mut io_ready_chunk)
            .offset((*io_ready_chunk_buckets_head).firstfree as isize);
        (*io_ready_chunk_buckets_head).firstfree =
            (*io_ready_chunk_buckets_head).firstfree.wrapping_add(1);
        io_ready_chunk_used = (io_ready_chunk_used as ::core::ffi::c_ulong)
            .wrapping_add(::core::mem::size_of::<io_ready_chunk>() as ::core::ffi::c_ulong)
            as uint64_t;
        return ret;
    }
}
static mut io_ready_chunk_free_head: *mut ::core::ffi::c_void = NULL;
#[inline]
unsafe extern "C" fn io_ready_chunk_getusage(
    mut allocated: *mut uint64_t,
    mut used: *mut uint64_t,
) {
    unsafe {
        *allocated = io_ready_chunk_allocated;
        *used = io_ready_chunk_used;
    }
}
static mut io_ready_chunk_buckets_head: *mut io_ready_chunk_bucket =
    ::core::ptr::null_mut::<io_ready_chunk_bucket>();
#[inline]
unsafe extern "C" fn chunk_io_ready_begin(mut c: *mut chunk) {
    unsafe {
        let mut rch: *mut io_ready_chunk = ::core::ptr::null_mut::<io_ready_chunk>();
        let mut hash: uint32_t = 0;
        hash = ((*c).chunkid & 0xff as uint64_t) as uint32_t;
        rch = io_ready_chunk_hash[hash as usize];
        while !rch.is_null() {
            if (*rch).c == c {
                return;
            }
            rch = (*rch).next as *mut io_ready_chunk;
        }
        rch = io_ready_chunk_malloc();
        (*rch).c = c;
        (*rch).next = io_ready_chunk_hash[hash as usize] as *mut _io_ready_chunk;
        io_ready_chunk_hash[hash as usize] = rch;
    }
}
#[inline]
unsafe extern "C" fn chunk_io_ready_end(mut c: *mut chunk) {
    unsafe {
        let mut rch: *mut io_ready_chunk = ::core::ptr::null_mut::<io_ready_chunk>();
        let mut rchp: *mut *mut io_ready_chunk = ::core::ptr::null_mut::<*mut io_ready_chunk>();
        let mut hash: uint32_t = 0;
        hash = ((*c).chunkid & 0xff as uint64_t) as uint32_t;
        rchp = (&raw mut io_ready_chunk_hash as *mut *mut io_ready_chunk).offset(hash as isize);
        loop {
            rch = *rchp;
            if rch.is_null() {
                break;
            }
            if (*rch).c == c {
                *rchp = (*rch).next as *mut io_ready_chunk;
                io_ready_chunk_free(rch);
            } else {
                rchp = &raw mut (*rch).next as *mut *mut io_ready_chunk;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn chunk_not_ready_for_io(mut c: *mut chunk) -> uint8_t {
    unsafe {
        let mut rch: *mut io_ready_chunk = ::core::ptr::null_mut::<io_ready_chunk>();
        let mut hash: uint32_t = 0;
        hash = ((*c).chunkid & 0xff as uint64_t) as uint32_t;
        rch = io_ready_chunk_hash[hash as usize];
        while !rch.is_null() {
            if (*rch).c == c {
                return 1 as uint8_t;
            }
            rch = (*rch).next as *mut io_ready_chunk;
        }
        return 0 as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn chunk_io_ready_check(mut c: *mut chunk) {
    unsafe {
        if chunk_not_ready_for_io(c) != 0 {
            chunk_do_fast_job(c, main_time(), 2 as uint8_t);
            if (*c).operation() as ::core::ffi::c_int == NONE as ::core::ffi::c_int
                && (*c).ondangerlist() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                chunk_io_ready_end(c);
            }
        } else if (*c).ondangerlist() != 0 {
            chunk_do_fast_job(c, main_time(), 1 as uint8_t);
        }
    }
}
#[inline]
unsafe extern "C" fn chunk_io_ready_init() {
    unsafe {
        let mut hash: uint32_t = 0;
        hash = 0 as uint32_t;
        while hash < 256 as uint32_t {
            io_ready_chunk_hash[hash as usize] = ::core::ptr::null_mut::<io_ready_chunk>();
            hash = hash.wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C" fn chunk_io_ready_cleanall() {
    unsafe {
        io_ready_chunk_free_all();
        chunk_io_ready_init();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_emergency_increase_version(mut c: *mut chunk) {
    unsafe {
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut i: uint32_t = 0;
        let mut fix: uint8_t = 0;
        i = 0 as uint32_t;
        fix = 0 as uint8_t;
        s = (*c).slisthead;
        while !s.is_null() {
            if (*s).valid as ::core::ffi::c_int != INVALID as ::core::ffi::c_int
                && (*s).valid as ::core::ffi::c_int != DEL as ::core::ffi::c_int
                && (*s).valid as ::core::ffi::c_int != WVER as ::core::ffi::c_int
                && (*s).valid as ::core::ffi::c_int != TDWVER as ::core::ffi::c_int
            {
                if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    if (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int
                        || (*s).valid as ::core::ffi::c_int == TDBUSY as ::core::ffi::c_int
                    {
                        (*s).valid = TDBUSY as ::core::ffi::c_int as uint8_t;
                    } else {
                        (*s).valid = BUSY as ::core::ffi::c_int as uint8_t;
                    }
                    (*s).version = ((*c).version() as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                        as uint32_t;
                    stats_chunkops[CHUNK_OP_CHANGE_TRY as usize] =
                        stats_chunkops[CHUNK_OP_CHANGE_TRY as usize].wrapping_add(1);
                    matocsserv_send_setchunkversion(
                        (*cstab.offset((*s).csid as isize)).ptr,
                        (*c).chunkid,
                        0 as uint8_t,
                        ((*c).version() as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            as uint32_t,
                        (*c).version() as uint32_t,
                    );
                    chunk_addopchunk((*s).csid, (*c).chunkid);
                    i = i.wrapping_add(1);
                } else {
                    if (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int
                        || (*s).valid as ::core::ffi::c_int == TDBUSY as ::core::ffi::c_int
                    {
                        (*s).valid = TDWVER as ::core::ffi::c_int as uint8_t;
                    } else {
                        (*s).valid = WVER as ::core::ffi::c_int as uint8_t;
                    }
                    fix = 1 as uint8_t;
                }
            }
            s = (*s).next as *mut slist;
        }
        if fix != 0 {
            chunk_state_fix(c);
        }
        if i > 0 as uint32_t {
            (*c).set_interrupted(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            chunk_set_op(c, SET_VERSION as ::core::ffi::c_int as uint8_t);
            (*c).set_version((*c).version() + 1 as ::core::ffi::c_uint);
            (*c).set_allowreadzeros(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            changelog(
                b"%u|SETVERSION(%lu,%u)\0".as_ptr() as *const ::core::ffi::c_char,
                main_time(),
                (*c).chunkid,
                (*c).version() as ::core::ffi::c_int,
            );
        } else {
            matoclserv_chunk_status((*c).chunkid, MFS_ERROR_CHUNKLOST as uint8_t);
        };
    }
}
#[inline]
unsafe extern "C" fn chunk_remove_disconnected_chunks(mut c: *mut chunk) -> ::core::ffi::c_int {
    unsafe {
        let mut opfinished: uint8_t = 0;
        let mut validcopies: uint8_t = 0;
        let mut disc: uint8_t = 0;
        let mut verfixed: uint8_t = 0;
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut st: *mut *mut slist = ::core::ptr::null_mut::<*mut slist>();
        let mut now: uint32_t = 0;
        if discservers.is_null() && discservers_next.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        disc = 0 as uint8_t;
        st = &raw mut (*c).slisthead;
        while !(*st).is_null() {
            s = *st;
            if (*cstab.offset((*s).csid as isize)).valid == 0 {
                if (*c).writeinprogress() as ::core::ffi::c_int != 0
                    && (*s).valid as ::core::ffi::c_int != INVALID as ::core::ffi::c_int
                    && (*s).valid as ::core::ffi::c_int != DEL as ::core::ffi::c_int
                    && (*s).valid as ::core::ffi::c_int != WVER as ::core::ffi::c_int
                    && (*s).valid as ::core::ffi::c_int != TDWVER as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    matocsserv_write_counters(
                        (*cstab.offset((*s).csid as isize)).ptr,
                        0 as uint8_t,
                    );
                }
                (*c).set_needverincrease(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                *st = (*s).next as *mut slist;
                slist_free(s);
                disc = 1 as uint8_t;
            } else {
                st = &raw mut (*s).next as *mut *mut slist;
            }
        }
        if disc as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if (*c).lockedto < main_time()
            && (*c).operation() as ::core::ffi::c_int == NONE as ::core::ffi::c_int
            && (*c).slisthead.is_null()
            && (*c).fhead == FLISTNULLINDX as uint32_t
            && chunk_counters_in_progress() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && csdb_have_all_servers() as ::core::ffi::c_int != 0
        {
            changelog(
                b"%u|CHUNKDEL(%lu,%u)\0".as_ptr() as *const ::core::ffi::c_char,
                main_time(),
                (*c).chunkid,
                (*c).version() as ::core::ffi::c_int,
            );
            if (*c).ondangerlist() != 0 {
                chunk_priority_remove(c);
            }
            chunk_delete(c);
            return 1 as ::core::ffi::c_int;
        }
        now = main_time();
        if (*c).operation() as ::core::ffi::c_int != NONE as ::core::ffi::c_int {
            validcopies = 0 as uint8_t;
            opfinished = 1 as uint8_t;
            s = (*c).slisthead;
            while !s.is_null() {
                if (*s).valid as ::core::ffi::c_int == BUSY as ::core::ffi::c_int
                    || (*s).valid as ::core::ffi::c_int == TDBUSY as ::core::ffi::c_int
                {
                    opfinished = 0 as uint8_t;
                }
                if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && ((*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                        || (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int)
                {
                    validcopies = 1 as uint8_t;
                }
                s = (*s).next as *mut slist;
            }
            if opfinished as ::core::ffi::c_int != 0
                && validcopies as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && ((*c).operation() as ::core::ffi::c_int == SET_VERSION as ::core::ffi::c_int
                    || (*c).operation() as ::core::ffi::c_int == TRUNCATE as ::core::ffi::c_int)
            {
                verfixed = 0 as uint8_t;
                s = (*c).slisthead;
                while !s.is_null() {
                    if (*s).version.wrapping_add(1 as uint32_t) == (*c).version() as uint32_t
                        && (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        if (*s).valid as ::core::ffi::c_int == TDWVER as ::core::ffi::c_int {
                            verfixed = 1 as uint8_t;
                            (*s).valid = TDVALID as ::core::ffi::c_int as uint8_t;
                        } else if (*s).valid as ::core::ffi::c_int == WVER as ::core::ffi::c_int {
                            verfixed = 1 as uint8_t;
                            (*s).valid = VALID as ::core::ffi::c_int as uint8_t;
                        }
                    }
                    s = (*s).next as *mut slist;
                }
                if verfixed != 0 {
                    (*c).set_version((*c).version() - 1 as ::core::ffi::c_uint);
                    (*c).set_allowreadzeros(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    changelog(
                        b"%u|SETVERSION(%lu,%u)\0".as_ptr() as *const ::core::ffi::c_char,
                        main_time(),
                        (*c).chunkid,
                        (*c).version() as ::core::ffi::c_int,
                    );
                }
            }
            if opfinished != 0 {
                let mut nospace: uint8_t = 0;
                let mut status: uint8_t = 0;
                nospace = 1 as uint8_t;
                s = (*c).slisthead;
                while !s.is_null() {
                    status = chunk_delopchunk((*s).csid, (*c).chunkid);
                    if status as ::core::ffi::c_int != MFS_ERROR_MISMATCH
                        && status as ::core::ffi::c_int != MFS_ERROR_NOSPACE
                    {
                        nospace = 0 as uint8_t;
                    }
                    s = (*s).next as *mut slist;
                }
                if (*c).operation() as ::core::ffi::c_int == REPLICATE as ::core::ffi::c_int
                    || (*c).operation() as ::core::ffi::c_int == LOCALSPLIT as ::core::ffi::c_int
                {
                    chunk_set_op(c, NONE as ::core::ffi::c_int as uint8_t);
                    chunk_replock_repend((*c).chunkid);
                    chunk_io_ready_check(c);
                    if (*c).operation() as ::core::ffi::c_int == NONE as ::core::ffi::c_int {
                        matoclserv_chunk_status((*c).chunkid, MFS_STATUS_OK as uint8_t);
                    }
                    if (*c).lockedto < now
                        && chunk_replock_test((*c).chunkid, now) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                    {
                        matoclserv_chunk_unlocked((*c).chunkid, c as *mut ::core::ffi::c_void);
                    }
                } else if validcopies != 0 {
                    chunk_emergency_increase_version(c);
                } else {
                    chunk_set_op(c, NONE as ::core::ffi::c_int as uint8_t);
                    if nospace != 0 {
                        matoclserv_chunk_status((*c).chunkid, MFS_ERROR_NOSPACE as uint8_t);
                    } else {
                        matoclserv_chunk_status((*c).chunkid, MFS_ERROR_NOTDONE as uint8_t);
                    }
                }
            } else if (*c).operation() as ::core::ffi::c_int != REPLICATE as ::core::ffi::c_int
                && (*c).operation() as ::core::ffi::c_int != LOCALSPLIT as ::core::ffi::c_int
            {
                (*c).set_interrupted(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        }
        chunk_state_fix(c);
        chunk_priority_queue_check(c, 1 as uint8_t);
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_mr_increase_version(mut chunkid: uint64_t) -> ::core::ffi::c_int {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        c = chunk_find(chunkid);
        if c.is_null() {
            return MFS_ERROR_NOCHUNK;
        }
        (*c).set_version((*c).version() + 1 as ::core::ffi::c_uint);
        meta_version_inc();
        return MFS_STATUS_OK;
    }
}
#[inline]
unsafe extern "C" fn chunk_recalc_sclassid(mut c: *mut chunk) {
    unsafe {
        let mut sclassid: uint8_t = 0;
        let mut pri: uint64_t = 0;
        let mut bestpri: uint64_t = 0;
        let mut findx: uint32_t = 0;
        let mut fl: *mut flist = ::core::ptr::null_mut::<flist>();
        if (*c).fhead >= FLISTFIRSTINDX as uint32_t {
            sclassid = 0 as uint8_t;
            bestpri = 0 as uint64_t;
            findx = (*c).fhead;
            while findx != FLISTNULLINDX as uint32_t {
                fl = flist_get(findx);
                findx = (*fl).nexti;
                pri = sclass_get_joining_priority((*fl).sclassid as uint16_t);
                if pri > bestpri {
                    sclassid = (*fl).sclassid;
                    bestpri = pri;
                }
            }
            if sclassid as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            } else {
                fprintf(
                    stderr,
                    b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2839 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"sclassid>0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"wrong labels set\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - failed assertion '%s' : %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2839 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"sclassid>0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"wrong labels set\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            };
            if sclassid as ::core::ffi::c_int != (*c).sclassid as ::core::ffi::c_int {
                chunk_state_set_sclass(c, sclassid);
                chunk_priority_queue_check(c, 1 as uint8_t);
            }
        }
    }
}
#[inline]
unsafe extern "C" fn chunk_compact_and_find_sclassid(mut c: *mut chunk) -> uint16_t {
    unsafe {
        let mut sclassid: uint8_t = 0;
        let mut pri: uint64_t = 0;
        let mut bestpri: uint64_t = 0;
        let mut findx: uint32_t = 0;
        let mut fl: *mut flist = ::core::ptr::null_mut::<flist>();
        if (*c).fhead < FLISTFIRSTINDX as uint32_t {
            if (*c).fhead == FLISTNULLINDX as uint32_t {
                return 0 as uint16_t;
            } else {
                return (*c).sclassid as uint16_t;
            }
        }
        fl = flist_get((*c).fhead);
        if (*fl).nexti == FLISTNULLINDX as uint32_t {
            sclassid = (*fl).sclassid;
            findx = (*fl).fcount() as uint32_t;
            if findx < FLISTFIRSTINDX as uint32_t {
                flist_free((*c).fhead);
                (*c).fhead = findx;
            }
            return sclassid as uint16_t;
        } else {
            sclassid = 0 as uint8_t;
            bestpri = 0 as uint64_t;
            findx = (*c).fhead;
            while findx != FLISTNULLINDX as uint32_t {
                fl = flist_get(findx);
                findx = (*fl).nexti;
                pri = sclass_get_joining_priority((*fl).sclassid as uint16_t);
                if pri > bestpri {
                    sclassid = (*fl).sclassid;
                    bestpri = pri;
                }
            }
            if sclassid as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            } else {
                fprintf(
                    stderr,
                    b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2883 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"sclassid>0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"wrong labels set\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - failed assertion '%s' : %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2883 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"sclassid>0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"wrong labels set\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            };
            return sclassid as uint16_t;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_change_file(
    mut chunkid: uint64_t,
    mut prevsclassid: uint8_t,
    mut newsclassid: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut new_calculated_sclassid: uint16_t = 0;
        let mut fcount: uint32_t = 0;
        let mut findx: uint32_t = 0;
        let mut findxptr: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut flags: uint8_t = 0;
        let mut fl: *mut flist = ::core::ptr::null_mut::<flist>();
        if prevsclassid as ::core::ffi::c_int == newsclassid as ::core::ffi::c_int {
            return MFS_STATUS_OK;
        }
        c = chunk_find(chunkid);
        if c.is_null() {
            return MFS_ERROR_NOCHUNK;
        }
        if (*c).fhead == FLISTNULLINDX as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"serious structure inconsistency: (chunkid:%016lX)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*c).chunkid,
            );
            return MFS_ERROR_CHUNKLOST;
        }
        if (*c).fhead == FLISTONEFILEINDX as uint32_t {
            new_calculated_sclassid = newsclassid as uint16_t;
        } else {
            if (*c).fhead < FLISTFIRSTINDX as uint32_t {
                if prevsclassid as ::core::ffi::c_int == (*c).sclassid as ::core::ffi::c_int {
                } else {
                    fprintf(
                        stderr,
                        b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        2911 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"prevsclassid==c->sclassid\0".as_ptr() as *const ::core::ffi::c_char,
                        b"wrong chunk sclassid\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - failed assertion '%s' : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        2911 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"prevsclassid==c->sclassid\0".as_ptr() as *const ::core::ffi::c_char,
                        b"wrong chunk sclassid\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                };
                fcount = (*c).fhead;
                (*c).fhead = flist_alloc();
                fl = flist_get((*c).fhead);
                (*fl).set_fcount(fcount.wrapping_sub(1 as uint32_t) as ::core::ffi::c_uint
                    as ::core::ffi::c_uint);
                (*fl).sclassid = (*c).sclassid;
                (*fl).nexti = flist_alloc();
                fl = flist_get((*fl).nexti);
                (*fl).set_fcount(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                (*fl).sclassid = newsclassid;
                (*fl).nexti = FLISTNULLINDX as uint32_t;
            } else {
                findxptr = &raw mut (*c).fhead;
                flags = 0 as uint8_t;
                while flags as ::core::ffi::c_int != 3 as ::core::ffi::c_int && {
                    findx = *findxptr;
                    findx != FLISTNULLINDX as uint32_t
                } {
                    fl = flist_get(findx);
                    if (*fl).sclassid as ::core::ffi::c_int == prevsclassid as ::core::ffi::c_int
                        && flags as ::core::ffi::c_int & 2 as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                    {
                        if (*fl).fcount() as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            (*fl).set_fcount((*fl).fcount() - 1 as ::core::ffi::c_uint);
                            findxptr = &raw mut (*fl).nexti;
                        } else {
                            *findxptr = (*fl).nexti;
                            flist_free(findx);
                        }
                        flags = (flags as ::core::ffi::c_int | 2 as ::core::ffi::c_int) as uint8_t;
                    } else {
                        findxptr = &raw mut (*fl).nexti;
                        if (*fl).sclassid as ::core::ffi::c_int == newsclassid as ::core::ffi::c_int
                            && ((*fl).fcount() as ::core::ffi::c_int) < FLISTMAXFCOUNT
                            && flags as ::core::ffi::c_int & 1 as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                        {
                            (*fl).set_fcount((*fl).fcount() + 1 as ::core::ffi::c_uint);
                            flags =
                                (flags as ::core::ffi::c_int | 1 as ::core::ffi::c_int) as uint8_t;
                        }
                    }
                }
                if flags as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0 {
                } else {
                    fprintf(
                        stderr,
                        b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        2942 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"flags&2\0".as_ptr() as *const ::core::ffi::c_char,
                        b"prevsclassid not found\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - failed assertion '%s' : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        2942 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"flags&2\0".as_ptr() as *const ::core::ffi::c_char,
                        b"prevsclassid not found\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                };
                if flags as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    findx = flist_alloc();
                    fl = flist_get(findx);
                    (*fl).nexti = (*c).fhead;
                    (*c).fhead = findx;
                    (*fl).sclassid = newsclassid;
                    (*fl).set_fcount(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
            }
            new_calculated_sclassid = chunk_compact_and_find_sclassid(c);
        }
        if new_calculated_sclassid as ::core::ffi::c_int != (*c).sclassid as ::core::ffi::c_int {
            chunk_state_set_sclass(c, new_calculated_sclassid as uint8_t);
            chunk_priority_queue_check(c, 1 as uint8_t);
        } else {
            chunk_priority_queue_check(c, 0 as uint8_t);
        }
        return MFS_STATUS_OK;
    }
}
#[inline]
unsafe extern "C" fn chunk_delete_file_int(
    mut c: *mut chunk,
    mut sclassid: uint8_t,
    mut delete_timeout: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut new_calculated_sclassid: uint16_t = 0;
        let mut findx: uint32_t = 0;
        let mut findxptr: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut flags: uint8_t = 0;
        let mut fl: *mut flist = ::core::ptr::null_mut::<flist>();
        if (*c).fhead == FLISTNULLINDX as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"serious structure inconsistency: (chunkid:%016lX)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*c).chunkid,
            );
            return MFS_ERROR_CHUNKLOST;
        }
        if sclassid as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        } else {
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                2976 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"sclassid>0\0".as_ptr() as *const ::core::ffi::c_char,
                b"wrong storage class id\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s' : %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                2976 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"sclassid>0\0".as_ptr() as *const ::core::ffi::c_char,
                b"wrong storage class id\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        if (*c).fhead == FLISTONEFILEINDX as uint32_t {
            if sclassid as ::core::ffi::c_int == (*c).sclassid as ::core::ffi::c_int {
            } else {
                fprintf(
                    stderr,
                    b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2978 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"sclassid==c->sclassid\0".as_ptr() as *const ::core::ffi::c_char,
                    b"wrong chunk sclassid\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - failed assertion '%s' : %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2978 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"sclassid==c->sclassid\0".as_ptr() as *const ::core::ffi::c_char,
                    b"wrong chunk sclassid\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            };
            new_calculated_sclassid = 0 as uint16_t;
            (*c).fhead = FLISTNULLINDX as uint32_t;
        } else if (*c).fhead < FLISTFIRSTINDX as uint32_t {
            if sclassid as ::core::ffi::c_int == (*c).sclassid as ::core::ffi::c_int {
            } else {
                fprintf(
                    stderr,
                    b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2982 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"sclassid==c->sclassid\0".as_ptr() as *const ::core::ffi::c_char,
                    b"wrong chunk sclassid\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - failed assertion '%s' : %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    2982 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"sclassid==c->sclassid\0".as_ptr() as *const ::core::ffi::c_char,
                    b"wrong chunk sclassid\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            };
            (*c).fhead = (*c).fhead.wrapping_sub(1);
            new_calculated_sclassid = (*c).sclassid as uint16_t;
        } else {
            findxptr = &raw mut (*c).fhead;
            flags = 0 as uint8_t;
            while flags as ::core::ffi::c_int == 0 as ::core::ffi::c_int && {
                findx = *findxptr;
                findx != FLISTNULLINDX as uint32_t
            } {
                fl = flist_get(findx);
                if (*fl).sclassid as ::core::ffi::c_int == sclassid as ::core::ffi::c_int {
                    if (*fl).fcount() as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        (*fl).set_fcount((*fl).fcount() - 1 as ::core::ffi::c_uint);
                        findxptr = &raw mut (*fl).nexti;
                    } else {
                        *findxptr = (*fl).nexti;
                        flist_free(findx);
                    }
                    flags = 1 as uint8_t;
                } else {
                    findxptr = &raw mut (*fl).nexti;
                }
            }
            if flags as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            } else {
                fprintf(
                    stderr,
                    b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    3003 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"flags==1\0".as_ptr() as *const ::core::ffi::c_char,
                    b"sclassid not found\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - failed assertion '%s' : %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    3003 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"flags==1\0".as_ptr() as *const ::core::ffi::c_char,
                    b"sclassid not found\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            };
            new_calculated_sclassid = chunk_compact_and_find_sclassid(c);
        }
        if new_calculated_sclassid as ::core::ffi::c_int != (*c).sclassid as ::core::ffi::c_int {
            chunk_state_set_sclass(c, new_calculated_sclassid as uint8_t);
        }
        if (*c).fhead == FLISTNULLINDX as uint32_t && delete_timeout > 0 as uint32_t {
            (*c).lockedto = main_time().wrapping_add(delete_timeout);
        }
        return MFS_STATUS_OK;
    }
}
#[inline]
unsafe extern "C" fn chunk_add_file_int(
    mut c: *mut chunk,
    mut sclassid: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut new_calculated_sclassid: uint16_t = 0;
        let mut findx: uint32_t = 0;
        let mut findxptr: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut flags: uint8_t = 0;
        let mut fl: *mut flist = ::core::ptr::null_mut::<flist>();
        if sclassid as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        } else {
            fprintf(
                stderr,
                b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                3021 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"sclassid>0\0".as_ptr() as *const ::core::ffi::c_char,
                b"wrong labels set\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - failed assertion '%s' : %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                3021 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"sclassid>0\0".as_ptr() as *const ::core::ffi::c_char,
                b"wrong labels set\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        };
        if (*c).fhead == FLISTNULLINDX as uint32_t {
            new_calculated_sclassid = sclassid as uint16_t;
            (*c).fhead = FLISTONEFILEINDX as uint32_t;
        } else if (*c).fhead < (FLISTFIRSTINDX - 1 as ::core::ffi::c_int) as uint32_t {
            if sclassid as ::core::ffi::c_int == (*c).sclassid as ::core::ffi::c_int {
                (*c).fhead = (*c).fhead.wrapping_add(1);
                new_calculated_sclassid = sclassid as uint16_t;
            } else {
                findx = flist_alloc();
                fl = flist_get(findx);
                (*fl).sclassid = (*c).sclassid;
                (*fl).set_fcount((*c).fhead as ::core::ffi::c_uint as ::core::ffi::c_uint);
                (*c).fhead = findx;
                (*fl).nexti = flist_alloc();
                fl = flist_get((*fl).nexti);
                (*fl).sclassid = sclassid;
                (*fl).set_fcount(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                (*fl).nexti = FLISTNULLINDX as uint32_t;
                new_calculated_sclassid = chunk_compact_and_find_sclassid(c);
            }
        } else {
            if (*c).fhead == (FLISTFIRSTINDX - 1 as ::core::ffi::c_int) as uint32_t {
                findx = flist_alloc();
                fl = flist_get(findx);
                (*fl).nexti = FLISTNULLINDX as uint32_t;
                (*fl).sclassid = (*c).sclassid;
                (*fl).set_fcount((*c).fhead as ::core::ffi::c_uint as ::core::ffi::c_uint);
                (*c).fhead = findx;
            }
            findxptr = &raw mut (*c).fhead;
            flags = 0 as uint8_t;
            while flags as ::core::ffi::c_int == 0 as ::core::ffi::c_int && {
                findx = *findxptr;
                findx != FLISTNULLINDX as uint32_t
            } {
                fl = flist_get(findx);
                findxptr = &raw mut (*fl).nexti;
                if (*fl).sclassid as ::core::ffi::c_int == sclassid as ::core::ffi::c_int
                    && ((*fl).fcount() as ::core::ffi::c_int) < FLISTMAXFCOUNT
                {
                    (*fl).set_fcount((*fl).fcount() + 1 as ::core::ffi::c_uint);
                    flags = 1 as uint8_t;
                }
            }
            if flags as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                findx = flist_alloc();
                fl = flist_get(findx);
                (*fl).nexti = (*c).fhead;
                (*c).fhead = findx;
                (*fl).sclassid = sclassid;
                (*fl).set_fcount(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
            new_calculated_sclassid = chunk_compact_and_find_sclassid(c);
        }
        if new_calculated_sclassid as ::core::ffi::c_int != (*c).sclassid as ::core::ffi::c_int {
            chunk_state_set_sclass(c, new_calculated_sclassid as uint8_t);
        }
        return MFS_STATUS_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_delete_file(
    mut chunkid: uint64_t,
    mut sclassid: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        c = chunk_find(chunkid);
        if c.is_null() {
            return MFS_ERROR_NOCHUNK;
        }
        return chunk_delete_file_int(c, sclassid, 0 as uint32_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_add_file(
    mut chunkid: uint64_t,
    mut sclassid: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        c = chunk_find(chunkid);
        if c.is_null() {
            return MFS_ERROR_NOCHUNK;
        }
        return chunk_add_file_int(c, sclassid);
    }
}
#[inline]
unsafe extern "C" fn chunk_write_counters(mut c: *mut chunk, mut x: uint8_t) {
    unsafe {
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        if x != 0 {
            if (*c).writeinprogress() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*c).set_writeinprogress(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            } else {
                return;
            }
        } else if (*c).writeinprogress() != 0 {
            (*c).set_writeinprogress(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        } else {
            return;
        }
        s = (*c).slisthead;
        while !s.is_null() {
            if (*s).valid as ::core::ffi::c_int != INVALID as ::core::ffi::c_int
                && (*s).valid as ::core::ffi::c_int != DEL as ::core::ffi::c_int
                && (*s).valid as ::core::ffi::c_int != WVER as ::core::ffi::c_int
                && (*s).valid as ::core::ffi::c_int != TDWVER as ::core::ffi::c_int
                && (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                matocsserv_write_counters((*cstab.offset((*s).csid as isize)).ptr, x);
            }
            s = (*s).next as *mut slist;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_locked_or_busy(
    mut cptr: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: *mut chunk = cptr as *mut chunk;
        return if (*c).lockedto < main_time()
            && (*c).operation() as ::core::ffi::c_int == NONE as ::core::ffi::c_int
        {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_get_storage_status(
    mut chunkid: uint64_t,
    mut fcopies: *mut uint8_t,
    mut ec8parts: *mut uint8_t,
    mut ec4parts: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut cnt: uint8_t = 0;
        let mut c8: uint8_t = 0;
        let mut c4: uint8_t = 0;
        let mut ecmask8: uint32_t = 0;
        let mut ecmask4: uint32_t = 0;
        c = chunk_find(chunkid);
        if c.is_null() {
            return MFS_ERROR_NOCHUNK as uint8_t;
        }
        cnt = 0 as uint8_t;
        ecmask8 = 0 as uint32_t;
        ecmask4 = 0 as uint32_t;
        s = (*c).slisthead;
        while !s.is_null() {
            if (*cstab.offset((*s).csid as isize)).valid as ::core::ffi::c_int != 0
                && (*s).valid as ::core::ffi::c_int != DEL as ::core::ffi::c_int
                && (*s).valid as ::core::ffi::c_int != WVER as ::core::ffi::c_int
                && (*s).valid as ::core::ffi::c_int != TDWVER as ::core::ffi::c_int
                && (*s).valid as ::core::ffi::c_int != INVALID as ::core::ffi::c_int
            {
                if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    if (cnt as ::core::ffi::c_int) < 255 as ::core::ffi::c_int {
                        cnt = cnt.wrapping_add(1);
                    }
                } else if (*s).ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
                {
                    ecmask4 = (ecmask4 as ::core::ffi::c_uint
                        | (1 as ::core::ffi::c_uint)
                            << ((*s).ecid as ::core::ffi::c_int & 0xf as ::core::ffi::c_int))
                        as uint32_t;
                } else if (*s).ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int <= 0x30 as ::core::ffi::c_int
                {
                    ecmask8 = (ecmask8 as ::core::ffi::c_uint
                        | (1 as ::core::ffi::c_uint)
                            << ((*s).ecid as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
                        as uint32_t;
                }
            }
            s = (*s).next as *mut slist;
        }
        *fcopies = cnt;
        c8 = 0 as uint8_t;
        c4 = 0 as uint8_t;
        if ecmask8 != 0 {
            c8 = bitcount(ecmask8);
        }
        if ecmask4 != 0 {
            c4 = bitcount(ecmask4);
        }
        *ec8parts = c8;
        *ec4parts = c4;
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_get_archflag(
    mut chunkid: uint64_t,
    mut archflag: *mut uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        c = chunk_find(chunkid);
        if c.is_null() {
            return MFS_ERROR_NOCHUNK;
        }
        *archflag = ((*c).flags() as ::core::ffi::c_int & FLAG_ARCH) as uint8_t;
        return MFS_STATUS_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_set_archflag(
    mut chunkid: uint64_t,
    mut archflag: uint8_t,
    mut archflagchanged: *mut uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut newflags: uint8_t = 0;
        c = chunk_find(chunkid);
        if c.is_null() {
            return MFS_ERROR_NOCHUNK;
        }
        newflags = ((*c).flags() as ::core::ffi::c_int & FLAG_TRASH
            | (if archflag as ::core::ffi::c_int != 0 {
                FLAG_ARCH
            } else {
                0 as ::core::ffi::c_int
            })) as uint8_t;
        if newflags as ::core::ffi::c_int != (*c).flags() as ::core::ffi::c_int {
            chunk_state_set_flags(c, newflags);
            chunk_priority_queue_check(c, 1 as uint8_t);
            *archflagchanged = (*archflagchanged).wrapping_add(1);
        }
        return MFS_STATUS_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_set_trashflag(
    mut chunkid: uint64_t,
    mut trashflag: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut newflags: uint8_t = 0;
        c = chunk_find(chunkid);
        if c.is_null() {
            return MFS_ERROR_NOCHUNK;
        }
        if trashflag as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            && (*c).fhead > FLISTONEFILEINDX as uint32_t
        {
            return MFS_ERROR_NOTDONE;
        }
        newflags = ((*c).flags() as ::core::ffi::c_int & FLAG_ARCH
            | (if trashflag as ::core::ffi::c_int != 0 {
                FLAG_TRASH
            } else {
                0 as ::core::ffi::c_int
            })) as uint8_t;
        if newflags as ::core::ffi::c_int != (*c).flags() as ::core::ffi::c_int {
            chunk_state_set_flags(c, newflags);
            chunk_priority_queue_check(c, 1 as uint8_t);
        }
        return MFS_STATUS_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_set_autoarch(
    mut chunkid: uint64_t,
    mut archreftime: uint32_t,
    mut archflagchanged: *mut uint32_t,
    mut intrash: uint8_t,
    mut trashflagchanged: *mut uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut newflags: uint8_t = 0;
        c = chunk_find(chunkid);
        if c.is_null() {
            return MFS_ERROR_NOCHUNK;
        }
        newflags = (*c).flags() as uint8_t;
        if archreftime > (*c).lockedto
            && (*c).flags() as ::core::ffi::c_int & FLAG_ARCH == 0 as ::core::ffi::c_int
        {
            newflags = ((*c).flags() as ::core::ffi::c_int & FLAG_TRASH | FLAG_ARCH) as uint8_t;
            *archflagchanged = (*archflagchanged).wrapping_add(1);
        }
        if (intrash as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*c).fhead > FLISTONEFILEINDX as uint32_t)
            && (*c).flags() as ::core::ffi::c_int & FLAG_TRASH != 0
        {
            newflags = (newflags as ::core::ffi::c_int & FLAG_ARCH) as uint8_t;
            *trashflagchanged = (*trashflagchanged).wrapping_add(1);
        }
        if newflags as ::core::ffi::c_int != (*c).flags() as ::core::ffi::c_int {
            chunk_state_set_flags(c, newflags);
            chunk_priority_queue_check(c, 1 as uint8_t);
        }
        return MFS_STATUS_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_fileloop_task(
    mut chunkid: uint64_t,
    mut sclassid: uint8_t,
    mut aftereof: uint8_t,
    mut archreftime: uint32_t,
    mut archflagchanged: *mut uint32_t,
    mut intrash: uint8_t,
    mut trashflagchanged: *mut uint32_t,
) -> chunkfloop {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut newflags: uint8_t = 0;
        c = chunk_find(chunkid);
        if c.is_null() {
            return CHUNK_FLOOP_NOTFOUND;
        }
        if (*c).all_gequiv() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && aftereof as ::core::ffi::c_int != 0
            && (*c).lockedto < main_time()
            && (*c).operation() as ::core::ffi::c_int == NONE as ::core::ffi::c_int
        {
            chunk_delete_file_int(c, sclassid, 0 as uint32_t);
            return CHUNK_FLOOP_DELETED;
        }
        newflags = (*c).flags() as uint8_t;
        if archreftime > (*c).lockedto
            && (*c).flags() as ::core::ffi::c_int & FLAG_ARCH == 0 as ::core::ffi::c_int
        {
            newflags = (newflags as ::core::ffi::c_int & FLAG_TRASH | FLAG_ARCH) as uint8_t;
            *archflagchanged = (*archflagchanged).wrapping_add(1);
        }
        if (intrash as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*c).fhead > FLISTONEFILEINDX as uint32_t)
            && (*c).flags() as ::core::ffi::c_int & FLAG_TRASH != 0
        {
            newflags = (newflags as ::core::ffi::c_int & FLAG_ARCH) as uint8_t;
            *trashflagchanged = (*trashflagchanged).wrapping_add(1);
        }
        if newflags as ::core::ffi::c_int != (*c).flags() as ::core::ffi::c_int {
            chunk_state_set_flags(c, newflags);
            chunk_priority_queue_check(c, 1 as uint8_t);
        }
        if (*c).all_gequiv() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut wv: uint8_t = 0;
            let mut r#in: uint8_t = 0;
            let mut wvec4mask: uint32_t = 0;
            let mut wvec8mask: uint32_t = 0;
            let mut inec4mask: uint32_t = 0;
            let mut inec8mask: uint32_t = 0;
            let mut ec4mask: uint32_t = 0;
            let mut ec8mask: uint32_t = 0;
            wv = 0 as uint8_t;
            r#in = 0 as uint8_t;
            wvec4mask = 0 as uint32_t;
            wvec8mask = 0 as uint32_t;
            inec4mask = 0 as uint32_t;
            inec8mask = 0 as uint32_t;
            ec4mask = 0 as uint32_t;
            ec8mask = 0 as uint32_t;
            s = (*c).slisthead;
            while !s.is_null() {
                if (*s).valid as ::core::ffi::c_int == WVER as ::core::ffi::c_int
                    || (*s).valid as ::core::ffi::c_int == TDWVER as ::core::ffi::c_int
                {
                    if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        wv = 1 as uint8_t;
                    } else if (*s).ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
                    {
                        wvec4mask = (wvec4mask as ::core::ffi::c_uint
                            | (1 as ::core::ffi::c_uint)
                                << ((*s).ecid as ::core::ffi::c_int & 0xf as ::core::ffi::c_int))
                            as uint32_t;
                    } else if (*s).ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x30 as ::core::ffi::c_int
                    {
                        wvec8mask = (wvec8mask as ::core::ffi::c_uint
                            | (1 as ::core::ffi::c_uint)
                                << ((*s).ecid as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
                            as uint32_t;
                    }
                } else if (*s).valid as ::core::ffi::c_int == INVALID as ::core::ffi::c_int {
                    if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        r#in = 1 as uint8_t;
                    } else if (*s).ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
                    {
                        inec4mask = (inec4mask as ::core::ffi::c_uint
                            | (1 as ::core::ffi::c_uint)
                                << ((*s).ecid as ::core::ffi::c_int & 0xf as ::core::ffi::c_int))
                            as uint32_t;
                    } else if (*s).ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x30 as ::core::ffi::c_int
                    {
                        inec8mask = (inec8mask as ::core::ffi::c_uint
                            | (1 as ::core::ffi::c_uint)
                                << ((*s).ecid as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
                            as uint32_t;
                    }
                } else if (*s).valid as ::core::ffi::c_int != DEL as ::core::ffi::c_int {
                    if (*s).ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
                    {
                        ec4mask = (ec4mask as ::core::ffi::c_uint
                            | (1 as ::core::ffi::c_uint)
                                << ((*s).ecid as ::core::ffi::c_int & 0xf as ::core::ffi::c_int))
                            as uint32_t;
                    } else if (*s).ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x30 as ::core::ffi::c_int
                    {
                        ec8mask = (ec8mask as ::core::ffi::c_uint
                            | (1 as ::core::ffi::c_uint)
                                << ((*s).ecid as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
                            as uint32_t;
                    }
                }
                s = (*s).next as *mut slist;
            }
            if wv as ::core::ffi::c_int != 0
                || bitcount(wvec4mask | ec4mask) as ::core::ffi::c_int >= 4 as ::core::ffi::c_int
                || bitcount(wvec8mask | ec8mask) as ::core::ffi::c_int >= 8 as ::core::ffi::c_int
            {
                return CHUNK_FLOOP_MISSING_WRONGVERSION;
            }
            if r#in as ::core::ffi::c_int != 0
                || bitcount(wvec4mask | inec4mask | ec4mask) as ::core::ffi::c_int
                    >= 4 as ::core::ffi::c_int
                || bitcount(wvec8mask | inec8mask | ec8mask) as ::core::ffi::c_int
                    >= 8 as ::core::ffi::c_int
            {
                return CHUNK_FLOOP_MISSING_INVALID;
            }
            if ec4mask | ec8mask != 0 {
                return CHUNK_FLOOP_MISSING_PARTIALEC;
            }
            return CHUNK_FLOOP_MISSING_NOCOPY;
        }
        if ((*c).all_gequiv() as ::core::ffi::c_int)
            < sclass_calc_goal_equivalent(sclass_get_keeparch_storagemode(
                (*c).sclassid as uint16_t,
                (*c).flags() as uint8_t,
            )) as ::core::ffi::c_int
        {
            return CHUNK_FLOOP_UNDERGOAL;
        }
        return CHUNK_FLOOP_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_remove_from_missing_log(mut chunkid: uint64_t) -> uint8_t {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        c = chunk_find(chunkid);
        if c.is_null() {
            return 1 as uint8_t;
        }
        if (*c).all_gequiv() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return 0 as uint8_t;
        }
        return 1 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_read_check(
    mut ts: uint32_t,
    mut chunkid: uint64_t,
    mut allow_recover: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut ecmask8: uint32_t = 0;
        let mut ecmask4: uint32_t = 0;
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        c = chunk_find(chunkid);
        if chunk_remove_disconnected_chunks(c) != 0 {
            c = ::core::ptr::null_mut::<chunk>();
        }
        if c.is_null() {
            return MFS_ERROR_NOCHUNK;
        }
        if (*c).lockedto >= ts {
            return MFS_ERROR_LOCKED;
        }
        if (*c).operation() as ::core::ffi::c_int != NONE as ::core::ffi::c_int {
            return MFS_ERROR_CHUNKBUSY;
        }
        ecmask4 = 0 as uint32_t;
        ecmask8 = 0 as uint32_t;
        s = (*c).slisthead;
        while !s.is_null() {
            if (*s).valid as ::core::ffi::c_int == BUSY as ::core::ffi::c_int
                || (*s).valid as ::core::ffi::c_int == TDBUSY as ::core::ffi::c_int
            {
                return MFS_ERROR_EAGAIN;
            }
            if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                || (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int
            {
                if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    return MFS_STATUS_OK;
                } else if (*s).ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
                {
                    ecmask4 = (ecmask4 as ::core::ffi::c_uint
                        | (1 as ::core::ffi::c_uint)
                            << ((*s).ecid as ::core::ffi::c_int & 0xf as ::core::ffi::c_int))
                        as uint32_t;
                } else if (*s).ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int <= 0x30 as ::core::ffi::c_int
                {
                    ecmask8 = (ecmask8 as ::core::ffi::c_uint
                        | (1 as ::core::ffi::c_uint)
                            << ((*s).ecid as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
                        as uint32_t;
                }
            }
            s = (*s).next as *mut slist;
        }
        if ecmask8 & 0xff as uint32_t == 0xff as uint32_t
            || ecmask4 & 0xf as uint32_t == 0xf as uint32_t
        {
            return MFS_STATUS_OK;
        }
        if allow_recover as ::core::ffi::c_int != 0
            && (bitcount(ecmask8) as ::core::ffi::c_int >= 8 as ::core::ffi::c_int
                || bitcount(ecmask4) as ::core::ffi::c_int >= 4 as ::core::ffi::c_int)
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_NOTICE,
                b"chunk 0x%016lX: data parts missing for read operation - trying to recover\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                chunkid,
            );
            chunk_do_fast_job(c, ts, 2 as uint8_t);
            if (*c).operation() as ::core::ffi::c_int != NONE as ::core::ffi::c_int {
                chunk_io_ready_begin(c);
                return MFS_ERROR_CHUNKBUSY;
            }
        }
        if csregisterinprogress != 0 {
            matocsserv_broadcast_regfirst_chunk(chunkid);
        }
        return MFS_STATUS_OK;
    }
}
#[inline]
unsafe extern "C" fn chunk_can_be_fixed(
    mut c: *mut chunk,
    mut sm: *mut storagemode,
) -> ::core::ffi::c_int {
    unsafe {
        static mut servers: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
        static mut rcsids: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
        let mut matching: *mut int32_t = ::core::ptr::null_mut::<int32_t>();
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut servcnt: uint16_t = 0;
        let mut vcservcnt: uint16_t = 0;
        let mut availservers: uint16_t = 0;
        let mut i: uint16_t = 0;
        if c.is_null() && sm.is_null() {
            if !rcsids.is_null() {
                free(rcsids as *mut ::core::ffi::c_void);
                rcsids = ::core::ptr::null_mut::<uint16_t>();
            }
            if !servers.is_null() {
                free(servers as *mut ::core::ffi::c_void);
                servers = ::core::ptr::null_mut::<uint16_t>();
            }
            return 0 as ::core::ffi::c_int;
        }
        if servers.is_null() {
            servers = malloc(::core::mem::size_of::<uint16_t>().wrapping_mul(MAXCSCOUNT as size_t))
                as *mut uint16_t;
            if servers.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    3408 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"servers\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    3408 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"servers\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if servers
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint16_t
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    3408 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"servers\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    3408 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"servers\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
        }
        if rcsids.is_null() {
            rcsids = malloc(::core::mem::size_of::<uint16_t>().wrapping_mul(MAXCSCOUNT as size_t))
                as *mut uint16_t;
            if rcsids.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    3412 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"rcsids\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    3412 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"rcsids\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if rcsids
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint16_t
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    3412 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"rcsids\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    3412 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"rcsids\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
        }
        availservers = matocsserv_getservers_replpossible(rcsids as *mut uint16_t);
        servcnt = 0 as uint16_t;
        s = (*c).slisthead;
        while !s.is_null() {
            if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                && (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                let c2rust_fresh4 = servcnt;
                servcnt = servcnt.wrapping_add(1);
                *servers.offset(c2rust_fresh4 as isize) = (*s).csid;
            }
            s = (*s).next as *mut slist;
        }
        vcservcnt = servcnt;
        if ((*sm).has_labels as ::core::ffi::c_int != 0
            || DoNotUseSameIP as ::core::ffi::c_int != 0
            || DoNotUseSameRack as ::core::ffi::c_int != 0)
            && sclass_get_labels_mode((*c).sclassid as uint16_t, sm) as ::core::ffi::c_int
                == LABELS_MODE_STRICT
        {
            i = 0 as uint16_t;
            while (i as ::core::ffi::c_int) < availservers as ::core::ffi::c_int {
                s = (*c).slisthead;
                while !s.is_null()
                    && ((*s).csid as ::core::ffi::c_int
                        != *rcsids.offset(i as isize) as ::core::ffi::c_int
                        || (*s).ecid as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                {
                    s = (*s).next as *mut slist;
                }
                if s.is_null() {
                    let c2rust_fresh5 = servcnt;
                    servcnt = servcnt.wrapping_add(1);
                    *servers.offset(c2rust_fresh5 as isize) = *rcsids.offset(i as isize);
                }
                i = i.wrapping_add(1);
            }
            matching = do_advanced_match(sm, servcnt as uint32_t, servers);
            i = 0 as uint16_t;
            while (i as ::core::ffi::c_int) < (*sm).labelscnt as ::core::ffi::c_int {
                let mut servpos: int32_t = 0;
                if *matching.offset(i as isize) < (*sm).labelscnt as int32_t {
                    servpos = -1 as ::core::ffi::c_int as int32_t;
                } else {
                    servpos = *matching.offset(i as isize) - (*sm).labelscnt as int32_t;
                }
                if servpos >= vcservcnt as int32_t {
                    return 1 as ::core::ffi::c_int;
                }
                i = i.wrapping_add(1);
            }
            return 0 as ::core::ffi::c_int;
        }
        return if (*sm).labelscnt as ::core::ffi::c_int <= availservers as ::core::ffi::c_int
            || (vcservcnt as ::core::ffi::c_int) < availservers as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
}
#[inline]
unsafe extern "C" fn chunk_prepare_to_modify(
    mut c: *mut chunk,
    mut ts: uint32_t,
    mut cschanges: uint8_t,
    mut csalldata: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut vc: uint16_t = 0;
        let mut sm: *mut storagemode = ::core::ptr::null_mut::<storagemode>();
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut gequiv: uint8_t = 0;
        if (*c).operation() as ::core::ffi::c_int != NONE as ::core::ffi::c_int {
            return MFS_ERROR_CHUNKBUSY;
        }
        vc = 0 as uint16_t;
        s = (*c).slisthead;
        while !s.is_null() {
            if (*s).valid as ::core::ffi::c_int == BUSY as ::core::ffi::c_int
                || (*s).valid as ::core::ffi::c_int == TDBUSY as ::core::ffi::c_int
            {
                return MFS_ERROR_EAGAIN;
            }
            if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && ((*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                    || (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int)
            {
                vc = vc.wrapping_add(1);
            }
            s = (*s).next as *mut slist;
        }
        sm = sclass_get_keeparch_storagemode((*c).sclassid as uint16_t, (*c).flags() as uint8_t);
        if (*sm).ec_data_chksum_parts as ::core::ffi::c_int != 0
            && (*c).flags() as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            chunk_state_set_flags(c, 0 as uint8_t);
            changelog(
                b"%u|CHUNKFLAGSCLR(%lu)\0".as_ptr() as *const ::core::ffi::c_char,
                ts,
                (*c).chunkid,
            );
            sm =
                sclass_get_keeparch_storagemode((*c).sclassid as uint16_t, (*c).flags() as uint8_t);
        }
        gequiv = sclass_calc_goal_equivalent(sm);
        if vc as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (*c).all_gequiv() as ::core::ffi::c_int > 0 as ::core::ffi::c_int
        {
            chunk_do_fast_job(c, ts, 2 as uint8_t);
            if (*c).operation() as ::core::ffi::c_int != NONE as ::core::ffi::c_int {
                chunk_io_ready_begin(c);
                return MFS_ERROR_CHUNKBUSY;
            }
            vc = 0 as uint16_t;
            s = (*c).slisthead;
            while !s.is_null() {
                if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && ((*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                        || (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int)
                {
                    vc = vc.wrapping_add(1);
                }
                s = (*s).next as *mut slist;
            }
        }
        if cschanges != 0 {
            if (vc as ::core::ffi::c_int) < gequiv as ::core::ffi::c_int {
                if csregisterinprogress != 0 {
                    matocsserv_broadcast_regfirst_chunk((*c).chunkid);
                }
                return MFS_ERROR_EAGAIN;
            }
        } else if vc as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && (vc as ::core::ffi::c_int) < gequiv as ::core::ffi::c_int
        {
            chunk_do_fast_job(c, ts, 2 as uint8_t);
            if (*c).operation() as ::core::ffi::c_int != NONE as ::core::ffi::c_int {
                chunk_io_ready_begin(c);
                return MFS_ERROR_CHUNKBUSY;
            }
            vc = 0 as uint16_t;
            s = (*c).slisthead;
            while !s.is_null() {
                if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && ((*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                        || (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int)
                {
                    vc = vc.wrapping_add(1);
                }
                s = (*s).next as *mut slist;
            }
        }
        if vc as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && (vc as ::core::ffi::c_int) < gequiv as ::core::ffi::c_int
        {
            if chunk_can_be_fixed(c, sm) != 0 {
                return MFS_ERROR_EAGAIN;
            }
        }
        if vc as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if (*c).all_gequiv() as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                if sclass_get_labels_mode((*c).sclassid as uint16_t, sm) as ::core::ffi::c_int
                    == LABELS_MODE_STRICT
                    && (*sclass_get_keeparch_storagemode((*c).sclassid as uint16_t, 0 as uint8_t))
                        .matching_servers as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    && csalldata as ::core::ffi::c_int != 0
                {
                    return MFS_ERROR_NOSPACE;
                }
                return MFS_ERROR_EAGAIN;
            } else if csalldata != 0 {
                return MFS_ERROR_CHUNKLOST;
            } else {
                return MFS_ERROR_CSNOTPRESENT;
            }
        }
        return MFS_STATUS_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_univ_multi_modify(
    mut ts: uint32_t,
    mut mr: uint8_t,
    mut continueop: uint8_t,
    mut nchunkid: *mut uint64_t,
    mut ochunkid: uint64_t,
    mut sclassid: uint8_t,
    mut opflag: *mut uint8_t,
    mut clientip: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut csids: [uint16_t; 10000] = [0; 10000];
        static mut chosen: *mut *mut ::core::ffi::c_void =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
        static mut chosenleng: uint32_t = 0 as uint32_t;
        let mut servcount: uint16_t = 0 as uint16_t;
        let mut rvcexists: uint8_t = 0;
        let mut status: ::core::ffi::c_int = 0;
        let mut overloaded: uint8_t = 0;
        let mut fix: uint8_t = 0;
        let mut os: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut i: uint32_t = 0;
        let mut oc: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut csstable: uint8_t = 0;
        let mut csalldata: uint8_t = 0;
        let mut cschanges: uint8_t = 0;
        let mut labels_mode: uint8_t = 0;
        let mut sm: *mut storagemode = ::core::ptr::null_mut::<storagemode>();
        let mut allvc: uint8_t = 0;
        if ts > main_start_time().wrapping_add(5 as uint32_t)
            && csregisterinprogress as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            csstable = 1 as uint8_t;
        } else {
            csstable = 0 as uint8_t;
        }
        cschanges = (if csstable as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || matocsserv_receiving_chunks_state() as ::core::ffi::c_int & TRANSFERRING_NEW_CHUNKS
                != 0
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
        if chunk_counters_in_progress() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && csdb_have_all_servers() as ::core::ffi::c_int != 0
        {
            csalldata = 1 as uint8_t;
        } else {
            csalldata = 0 as uint8_t;
        }
        sm = sclass_get_create_storagemode(sclassid as uint16_t);
        if ochunkid == 0 as uint64_t {
            if mr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                labels_mode = sclass_get_labels_mode(sclassid as uint16_t, sm);
                servcount = chunk_creation_servers(
                    &raw mut csids as *mut uint16_t,
                    sm,
                    labels_mode,
                    &raw mut overloaded,
                    clientip,
                );
                if servcount as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    if overloaded as ::core::ffi::c_int != 0
                        || csalldata as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        return MFS_ERROR_EAGAIN;
                    } else {
                        let mut scount: uint16_t = 0;
                        scount = matocsserv_servers_count();
                        if scount as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                            && csstable as ::core::ffi::c_int != 0
                        {
                            return MFS_ERROR_NOSPACE;
                        } else {
                            return MFS_ERROR_NOCHUNKSERVERS;
                        }
                    }
                }
                let c2rust_fresh0 = nextchunkid;
                nextchunkid = nextchunkid.wrapping_add(1);
                c = chunk_new(c2rust_fresh0);
                (*c).set_version(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                (*c).set_interrupted(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                chunk_set_op(c, CREATE as ::core::ffi::c_int as uint8_t);
                chunk_add_file_int(c, sclassid);
                if (servcount as ::core::ffi::c_int) < (*sm).labelscnt as ::core::ffi::c_int {
                    allvc = servcount as uint8_t;
                } else {
                    allvc = (*sm).labelscnt;
                }
                if allvc as uint32_t > chosenleng {
                    chosenleng =
                        (allvc as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as uint32_t;
                    chosen = malloc(
                        ::core::mem::size_of::<*mut ::core::ffi::c_void>()
                            .wrapping_mul(chosenleng as size_t),
                    ) as *mut *mut ::core::ffi::c_void;
                    if chosen.is_null() {
                        fprintf(
                            stderr,
                            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            3606 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"chosen\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            3606 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"chosen\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        abort();
                    } else if chosen
                        == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                            -1 as ::core::ffi::c_int as usize,
                        ) as *mut *mut ::core::ffi::c_void
                    {
                        let mut _mfs_errorstring: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            3606 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"chosen\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            3606 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"chosen\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring,
                        );
                        abort();
                    }
                }
                i = 0 as uint32_t;
                while i < allvc as uint32_t {
                    s = slist_malloc();
                    (*s).csid = csids[i as usize];
                    (*s).ecid = 0 as uint8_t;
                    (*s).valid = BUSY as ::core::ffi::c_int as uint8_t;
                    (*s).version = (*c).version() as uint32_t;
                    chunk_new_copy(c, s);
                    *chosen.offset(i as isize) = (*cstab.offset((*s).csid as isize)).ptr;
                    stats_chunkops[CHUNK_OP_CREATE_TRY as usize] =
                        stats_chunkops[CHUNK_OP_CREATE_TRY as usize].wrapping_add(1);
                    matocsserv_send_createchunk(
                        (*cstab.offset((*s).csid as isize)).ptr,
                        (*c).chunkid,
                        0 as uint8_t,
                        (*c).version() as uint32_t,
                    );
                    chunk_addopchunk((*s).csid, (*c).chunkid);
                    i = i.wrapping_add(1);
                }
                matocsserv_useservers_wrandom(
                    chosen as *mut *mut ::core::ffi::c_void,
                    allvc as uint16_t,
                );
                if allvc as ::core::ffi::c_int > 15 as ::core::ffi::c_int {
                    allvc = 15 as uint8_t;
                }
                chunk_state_set_counters(c, STORAGE_MODE_COPIES as uint8_t, allvc, allvc);
                *opflag = 1 as uint8_t;
                *nchunkid = (*c).chunkid;
            } else {
                if *nchunkid != nextchunkid {
                    return MFS_ERROR_MISMATCH;
                }
                let c2rust_fresh1 = nextchunkid;
                nextchunkid = nextchunkid.wrapping_add(1);
                c = chunk_new(c2rust_fresh1);
                (*c).set_version(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                chunk_add_file_int(c, sclassid);
            }
        } else {
            c = ::core::ptr::null_mut::<chunk>();
            oc = chunk_find(ochunkid);
            if !oc.is_null() && mr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if chunk_remove_disconnected_chunks(oc) != 0 {
                    oc = ::core::ptr::null_mut::<chunk>();
                }
            }
            if oc.is_null() {
                return MFS_ERROR_NOCHUNK;
            }
            if mr as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && ((*oc).lockedto >= ts
                    || chunk_replock_test(ochunkid, ts) as ::core::ffi::c_int != 0)
                && continueop as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                return MFS_ERROR_LOCKED;
            }
            if (*oc).fhead == FLISTONEFILEINDX as uint32_t {
                c = oc;
                if mr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    *nchunkid = ochunkid;
                    status = chunk_prepare_to_modify(c, ts, cschanges, csalldata);
                    if status != MFS_STATUS_OK {
                        return status;
                    }
                    rvcexists = 0 as uint8_t;
                    if (*c).needverincrease() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        s = (*c).slisthead;
                        while !s.is_null()
                            && rvcexists as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        {
                            if (*s).ecid as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                                && ((*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                                    || (*s).valid as ::core::ffi::c_int
                                        == TDVALID as ::core::ffi::c_int)
                            {
                                rvcexists = 1 as uint8_t;
                            }
                            s = (*s).next as *mut slist;
                        }
                    }
                    if (*c).needverincrease() as ::core::ffi::c_int != 0
                        || rvcexists as ::core::ffi::c_int != 0
                    {
                        fix = 0 as uint8_t;
                        s = (*c).slisthead;
                        while !s.is_null() {
                            if (*s).valid as ::core::ffi::c_int != INVALID as ::core::ffi::c_int
                                && (*s).valid as ::core::ffi::c_int != DEL as ::core::ffi::c_int
                                && (*s).valid as ::core::ffi::c_int != WVER as ::core::ffi::c_int
                                && (*s).valid as ::core::ffi::c_int != TDWVER as ::core::ffi::c_int
                            {
                                if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                                    if (*s).valid as ::core::ffi::c_int
                                        == TDVALID as ::core::ffi::c_int
                                    {
                                        (*s).valid = TDBUSY as ::core::ffi::c_int as uint8_t;
                                    } else {
                                        (*s).valid = BUSY as ::core::ffi::c_int as uint8_t;
                                    }
                                    (*s).version = ((*c).version() as ::core::ffi::c_int
                                        + 1 as ::core::ffi::c_int)
                                        as uint32_t;
                                    stats_chunkops[CHUNK_OP_CHANGE_TRY as usize] = stats_chunkops
                                        [CHUNK_OP_CHANGE_TRY as usize]
                                        .wrapping_add(1);
                                    matocsserv_send_setchunkversion(
                                        (*cstab.offset((*s).csid as isize)).ptr,
                                        ochunkid,
                                        0 as uint8_t,
                                        ((*c).version() as ::core::ffi::c_int
                                            + 1 as ::core::ffi::c_int)
                                            as uint32_t,
                                        (*c).version() as uint32_t,
                                    );
                                    chunk_addopchunk((*s).csid, (*c).chunkid);
                                } else {
                                    if (*s).valid as ::core::ffi::c_int
                                        == TDVALID as ::core::ffi::c_int
                                    {
                                        (*s).valid = TDWVER as ::core::ffi::c_int as uint8_t;
                                    } else {
                                        (*s).valid = WVER as ::core::ffi::c_int as uint8_t;
                                    }
                                    fix = 1 as uint8_t;
                                }
                            }
                            s = (*s).next as *mut slist;
                        }
                        if fix != 0 {
                            chunk_state_fix(c);
                        }
                        (*c).set_interrupted(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        chunk_set_op(c, SET_VERSION as ::core::ffi::c_int as uint8_t);
                        (*c).set_version((*c).version() + 1 as ::core::ffi::c_uint);
                        *opflag = 1 as uint8_t;
                    } else {
                        *opflag = 0 as uint8_t;
                    }
                } else {
                    if *nchunkid != ochunkid {
                        return MFS_ERROR_MISMATCH;
                    }
                    if *opflag != 0 {
                        (*c).set_version((*c).version() + 1 as ::core::ffi::c_uint);
                    }
                }
            } else {
                if (*oc).fhead == FLISTNULLINDX as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"serious structure inconsistency: (chunkid:%016lX)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ochunkid,
                    );
                    return MFS_ERROR_CHUNKLOST;
                }
                if mr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    status = chunk_prepare_to_modify(oc, ts, cschanges, csalldata);
                    if status != MFS_STATUS_OK {
                        return status;
                    }
                    let c2rust_fresh2 = nextchunkid;
                    nextchunkid = nextchunkid.wrapping_add(1);
                    c = chunk_new(c2rust_fresh2);
                    (*c).set_version(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    (*c).set_interrupted(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    chunk_delete_file_int(oc, sclassid, 0 as uint32_t);
                    chunk_add_file_int(c, sclassid);
                    allvc = 0 as uint8_t;
                    os = (*oc).slisthead;
                    while !os.is_null() {
                        if (*os).valid as ::core::ffi::c_int != INVALID as ::core::ffi::c_int
                            && (*os).valid as ::core::ffi::c_int != DEL as ::core::ffi::c_int
                            && (*os).valid as ::core::ffi::c_int != WVER as ::core::ffi::c_int
                            && (*os).valid as ::core::ffi::c_int != TDWVER as ::core::ffi::c_int
                        {
                            if (*os).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                                chunk_set_op(c, DUPLICATE as ::core::ffi::c_int as uint8_t);
                                s = slist_malloc();
                                (*s).csid = (*os).csid;
                                (*s).ecid = (*os).ecid;
                                (*s).valid = BUSY as ::core::ffi::c_int as uint8_t;
                                (*s).ecid = 0 as uint8_t;
                                (*s).version = (*c).version() as uint32_t;
                                chunk_new_copy(c, s);
                                allvc = allvc.wrapping_add(1);
                                stats_chunkops[CHUNK_OP_CHANGE_TRY as usize] =
                                    stats_chunkops[CHUNK_OP_CHANGE_TRY as usize].wrapping_add(1);
                                matocsserv_send_duplicatechunk(
                                    (*cstab.offset((*s).csid as isize)).ptr,
                                    (*c).chunkid,
                                    0 as uint8_t,
                                    (*c).version() as uint32_t,
                                    (*oc).chunkid,
                                    0 as uint8_t,
                                    (*oc).version() as uint32_t,
                                );
                                chunk_addopchunk((*s).csid, (*c).chunkid);
                            }
                        }
                        os = (*os).next as *mut slist;
                    }
                    if allvc as ::core::ffi::c_int > 15 as ::core::ffi::c_int {
                        allvc = 15 as uint8_t;
                    }
                    chunk_state_set_counters(c, STORAGE_MODE_COPIES as uint8_t, allvc, allvc);
                    *nchunkid = (*c).chunkid;
                    *opflag = 1 as uint8_t;
                } else {
                    if *nchunkid != nextchunkid {
                        return MFS_ERROR_MISMATCH;
                    }
                    let c2rust_fresh3 = nextchunkid;
                    nextchunkid = nextchunkid.wrapping_add(1);
                    c = chunk_new(c2rust_fresh3);
                    (*c).set_version(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    chunk_delete_file_int(oc, sclassid, 0 as uint32_t);
                    chunk_add_file_int(c, sclassid);
                    *nchunkid = (*c).chunkid;
                }
            }
        }
        (*c).lockedto = ts.wrapping_add(LOCKTIMEOUT as uint32_t);
        chunk_write_counters(c, 1 as uint8_t);
        return MFS_STATUS_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_multi_modify(
    mut continueop: uint8_t,
    mut nchunkid: *mut uint64_t,
    mut ochunkid: uint64_t,
    mut sclassid: uint8_t,
    mut opflag: *mut uint8_t,
    mut clientip: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        return chunk_univ_multi_modify(
            main_time(),
            0 as uint8_t,
            continueop,
            nchunkid,
            ochunkid,
            sclassid,
            opflag,
            clientip,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_mr_multi_modify(
    mut ts: uint32_t,
    mut nchunkid: *mut uint64_t,
    mut ochunkid: uint64_t,
    mut sclassid: uint8_t,
    mut opflag: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        return chunk_univ_multi_modify(
            ts,
            1 as uint8_t,
            0 as uint8_t,
            nchunkid,
            ochunkid,
            sclassid,
            &raw mut opflag,
            0 as uint32_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_univ_multi_truncate(
    mut ts: uint32_t,
    mut mr: uint8_t,
    mut nchunkid: *mut uint64_t,
    mut ochunkid: uint64_t,
    mut length: uint32_t,
    mut sclassid: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut os: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut oc: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut csstable: uint8_t = 0;
        let mut csalldata: uint8_t = 0;
        let mut cschanges: uint8_t = 0;
        let mut allvc: uint8_t = 0;
        let mut status: ::core::ffi::c_int = 0;
        let mut fix: uint8_t = 0;
        if ts > main_start_time().wrapping_add(5 as uint32_t)
            && csregisterinprogress as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            csstable = 1 as uint8_t;
        } else {
            csstable = 0 as uint8_t;
        }
        cschanges = (if csstable as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || matocsserv_receiving_chunks_state() as ::core::ffi::c_int & TRANSFERRING_NEW_CHUNKS
                != 0
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
        if chunk_counters_in_progress() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && csdb_have_all_servers() as ::core::ffi::c_int != 0
        {
            csalldata = 1 as uint8_t;
        } else {
            csalldata = 0 as uint8_t;
        }
        c = ::core::ptr::null_mut::<chunk>();
        oc = chunk_find(ochunkid);
        if !oc.is_null() && mr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if chunk_remove_disconnected_chunks(oc) != 0 {
                oc = ::core::ptr::null_mut::<chunk>();
            }
        }
        if oc.is_null() {
            return MFS_ERROR_NOCHUNK;
        }
        if mr as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && ((*oc).lockedto >= ts || chunk_replock_test(ochunkid, ts) as ::core::ffi::c_int != 0)
        {
            return MFS_ERROR_LOCKED;
        }
        if (*oc).fhead == FLISTONEFILEINDX as uint32_t {
            c = oc;
            if mr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                *nchunkid = ochunkid;
                status = chunk_prepare_to_modify(c, ts, cschanges, csalldata);
                if status != MFS_STATUS_OK {
                    return status;
                }
                fix = 0 as uint8_t;
                s = (*c).slisthead;
                while !s.is_null() {
                    if (*s).valid as ::core::ffi::c_int != INVALID as ::core::ffi::c_int
                        && (*s).valid as ::core::ffi::c_int != DEL as ::core::ffi::c_int
                        && (*s).valid as ::core::ffi::c_int != WVER as ::core::ffi::c_int
                        && (*s).valid as ::core::ffi::c_int != TDWVER as ::core::ffi::c_int
                    {
                        if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            if (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int {
                                (*s).valid = TDBUSY as ::core::ffi::c_int as uint8_t;
                            } else {
                                (*s).valid = BUSY as ::core::ffi::c_int as uint8_t;
                            }
                            (*s).version = ((*c).version() as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int)
                                as uint32_t;
                            stats_chunkops[CHUNK_OP_CHANGE_TRY as usize] =
                                stats_chunkops[CHUNK_OP_CHANGE_TRY as usize].wrapping_add(1);
                            matocsserv_send_truncatechunk(
                                (*cstab.offset((*s).csid as isize)).ptr,
                                ochunkid,
                                0 as uint8_t,
                                length,
                                ((*c).version() as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as uint32_t,
                                (*c).version() as uint32_t,
                            );
                            chunk_addopchunk((*s).csid, (*c).chunkid);
                        } else {
                            if (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int {
                                (*s).valid = TDWVER as ::core::ffi::c_int as uint8_t;
                            } else {
                                (*s).valid = WVER as ::core::ffi::c_int as uint8_t;
                            }
                            fix = 1 as uint8_t;
                        }
                    }
                    s = (*s).next as *mut slist;
                }
                if fix != 0 {
                    chunk_state_fix(c);
                }
                (*c).set_interrupted(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                chunk_set_op(c, TRUNCATE as ::core::ffi::c_int as uint8_t);
                (*c).set_version((*c).version() + 1 as ::core::ffi::c_uint);
            } else {
                if *nchunkid != ochunkid {
                    return MFS_ERROR_MISMATCH;
                }
                (*c).set_version((*c).version() + 1 as ::core::ffi::c_uint);
            }
        } else {
            if (*oc).fhead == FLISTNULLINDX as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"serious structure inconsistency: (chunkid:%016lX)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ochunkid,
                );
                return MFS_ERROR_CHUNKLOST;
            }
            if mr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                status = chunk_prepare_to_modify(oc, ts, cschanges, csalldata);
                if status != MFS_STATUS_OK {
                    return status;
                }
                let c2rust_fresh32 = nextchunkid;
                nextchunkid = nextchunkid.wrapping_add(1);
                c = chunk_new(c2rust_fresh32);
                (*c).set_version(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                (*c).set_interrupted(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                chunk_delete_file_int(oc, sclassid, 0 as uint32_t);
                chunk_add_file_int(c, sclassid);
                allvc = 0 as uint8_t;
                os = (*oc).slisthead;
                while !os.is_null() {
                    if (*os).valid as ::core::ffi::c_int != INVALID as ::core::ffi::c_int
                        && (*os).valid as ::core::ffi::c_int != DEL as ::core::ffi::c_int
                        && (*os).valid as ::core::ffi::c_int != WVER as ::core::ffi::c_int
                        && (*os).valid as ::core::ffi::c_int != TDWVER as ::core::ffi::c_int
                    {
                        if (*os).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            chunk_set_op(c, DUPTRUNC as ::core::ffi::c_int as uint8_t);
                            s = slist_malloc();
                            (*s).csid = (*os).csid;
                            (*s).ecid = (*os).ecid;
                            (*s).valid = BUSY as ::core::ffi::c_int as uint8_t;
                            (*s).version = (*c).version() as uint32_t;
                            chunk_new_copy(c, s);
                            allvc = allvc.wrapping_add(1);
                            stats_chunkops[CHUNK_OP_CHANGE_TRY as usize] =
                                stats_chunkops[CHUNK_OP_CHANGE_TRY as usize].wrapping_add(1);
                            matocsserv_send_duptruncchunk(
                                (*cstab.offset((*s).csid as isize)).ptr,
                                (*c).chunkid,
                                0 as uint8_t,
                                (*c).version() as uint32_t,
                                (*oc).chunkid,
                                0 as uint8_t,
                                (*oc).version() as uint32_t,
                                length,
                            );
                            chunk_addopchunk((*s).csid, (*c).chunkid);
                        }
                    }
                    os = (*os).next as *mut slist;
                }
                if allvc as ::core::ffi::c_int > 15 as ::core::ffi::c_int {
                    allvc = 15 as uint8_t;
                }
                chunk_state_set_counters(c, STORAGE_MODE_COPIES as uint8_t, allvc, allvc);
                *nchunkid = (*c).chunkid;
            } else {
                if *nchunkid != nextchunkid {
                    return MFS_ERROR_MISMATCH;
                }
                let c2rust_fresh33 = nextchunkid;
                nextchunkid = nextchunkid.wrapping_add(1);
                c = chunk_new(c2rust_fresh33);
                (*c).set_version(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                chunk_delete_file_int(oc, sclassid, 0 as uint32_t);
                chunk_add_file_int(c, sclassid);
                *nchunkid = (*c).chunkid;
            }
        }
        (*c).lockedto = ts.wrapping_add(LOCKTIMEOUT as uint32_t);
        return MFS_STATUS_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_multi_truncate(
    mut nchunkid: *mut uint64_t,
    mut ochunkid: uint64_t,
    mut length: uint32_t,
    mut sclassid: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        return chunk_univ_multi_truncate(
            main_time(),
            0 as uint8_t,
            nchunkid,
            ochunkid,
            length,
            sclassid,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_mr_multi_truncate(
    mut ts: uint32_t,
    mut nchunkid: *mut uint64_t,
    mut ochunkid: uint64_t,
    mut sclassid: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        return chunk_univ_multi_truncate(
            ts,
            1 as uint8_t,
            nchunkid,
            ochunkid,
            0 as uint32_t,
            sclassid,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_repair(
    mut sclassid: uint8_t,
    mut ochunkid: uint64_t,
    mut flags: uint8_t,
    mut nversion: *mut uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut bestversion: uint32_t = 0;
        let mut bestecversion8: uint32_t = 0;
        let mut bestecversion4: uint32_t = 0;
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut mask8: uint32_t = 0;
        let mut mask4: uint32_t = 0;
        let mut now: uint32_t = 0;
        *nversion = 0 as uint32_t;
        if ochunkid == 0 as uint64_t {
            return 0 as ::core::ffi::c_int;
        }
        c = chunk_find(ochunkid);
        if c.is_null() {
            return 1 as ::core::ffi::c_int;
        }
        now = main_time();
        if (*c).lockedto >= now || chunk_replock_test(ochunkid, now) as ::core::ffi::c_int != 0 {
            return 0 as ::core::ffi::c_int;
        }
        if (*c).all_gequiv() as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        chunk_write_counters(c, 0 as uint8_t);
        mask8 = 0 as uint32_t;
        mask4 = 0 as uint32_t;
        bestversion = 0 as uint32_t;
        bestecversion8 = 0 as uint32_t;
        bestecversion4 = 0 as uint32_t;
        s = (*c).slisthead;
        while !s.is_null() {
            if (*cstab.offset((*s).csid as isize)).valid != 0 {
                if (*s).valid as ::core::ffi::c_int == WVER as ::core::ffi::c_int
                    || (*s).valid as ::core::ffi::c_int == TDWVER as ::core::ffi::c_int
                {
                    if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        if (*s).version >= bestversion {
                            bestversion = (*s).version;
                        }
                    } else if (*s).ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
                    {
                        mask4 = (mask4 as ::core::ffi::c_uint
                            | (1 as ::core::ffi::c_uint)
                                << ((*s).ecid as ::core::ffi::c_int & 0xf as ::core::ffi::c_int))
                            as uint32_t;
                        if (*s).version >= bestecversion4 {
                            bestecversion4 = (*s).version;
                        }
                    } else if (*s).ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x30 as ::core::ffi::c_int
                    {
                        mask8 = (mask8 as ::core::ffi::c_uint
                            | (1 as ::core::ffi::c_uint)
                                << ((*s).ecid as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
                            as uint32_t;
                        if (*s).version >= bestecversion8 {
                            bestecversion8 = (*s).version;
                        }
                    }
                } else if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                    || (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int
                    || (*s).valid as ::core::ffi::c_int == BUSY as ::core::ffi::c_int
                    || (*s).valid as ::core::ffi::c_int == TDBUSY as ::core::ffi::c_int
                {
                    if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        return 0 as ::core::ffi::c_int;
                    } else if (*s).ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
                    {
                        mask4 = (mask4 as ::core::ffi::c_uint
                            | (1 as ::core::ffi::c_uint)
                                << ((*s).ecid as ::core::ffi::c_int & 0xf as ::core::ffi::c_int))
                            as uint32_t;
                        if (*s).version >= bestecversion4 {
                            bestecversion4 = (*s).version;
                        }
                    } else if (*s).ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x30 as ::core::ffi::c_int
                    {
                        mask8 = (mask8 as ::core::ffi::c_uint
                            | (1 as ::core::ffi::c_uint)
                                << ((*s).ecid as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
                            as uint32_t;
                        if (*s).version >= bestecversion8 {
                            bestecversion8 = (*s).version;
                        }
                    }
                }
            }
            s = (*s).next as *mut slist;
        }
        if bestversion == 0 as uint32_t {
            let mut useec: uint8_t = 0;
            useec = 0 as uint8_t;
            if bitcount(mask8) as ::core::ffi::c_int >= 8 as ::core::ffi::c_int
                && bitcount(mask4) as ::core::ffi::c_int >= 4 as ::core::ffi::c_int
            {
                if bestecversion8 > bestecversion4 {
                    useec = 8 as uint8_t;
                } else {
                    useec = 4 as uint8_t;
                }
            } else if bitcount(mask8) as ::core::ffi::c_int >= 8 as ::core::ffi::c_int {
                useec = 8 as uint8_t;
            } else if bitcount(mask4) as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                useec = 4 as uint8_t;
            } else if flags as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                chunk_delete_file_int(c, sclassid, 0 as uint32_t);
                return 1 as ::core::ffi::c_int;
            } else {
                (*c).set_allowreadzeros(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                *nversion = ((*c).version() | 0x80000000 as ::core::ffi::c_uint) as uint32_t;
                return 1 as ::core::ffi::c_int;
            }
            if useec as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                (*c).set_version(bestecversion8 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                s = (*c).slisthead;
                while !s.is_null() {
                    's_354: {
                        if (*cstab.offset((*s).csid as isize)).valid as ::core::ffi::c_int != 0
                            && (*s).ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                            && (*s).ecid as ::core::ffi::c_int <= 0x30 as ::core::ffi::c_int
                        {
                            if (*s).valid as ::core::ffi::c_int == WVER as ::core::ffi::c_int {
                                (*s).valid = BUSY as ::core::ffi::c_int as uint8_t;
                            } else if (*s).valid as ::core::ffi::c_int
                                == TDWVER as ::core::ffi::c_int
                            {
                                (*s).valid = TDBUSY as ::core::ffi::c_int as uint8_t;
                            } else {
                                break 's_354;
                            }
                            (*s).version = bestecversion8;
                            stats_chunkops[CHUNK_OP_CHANGE_TRY as usize] =
                                stats_chunkops[CHUNK_OP_CHANGE_TRY as usize].wrapping_add(1);
                            matocsserv_send_setchunkversion(
                                (*cstab.offset((*s).csid as isize)).ptr,
                                ochunkid,
                                (*s).ecid,
                                bestecversion8,
                                0 as uint32_t,
                            );
                        }
                        if (((*s).ecid as ::core::ffi::c_int) < 0x20 as ::core::ffi::c_int
                            || (*s).ecid as ::core::ffi::c_int > 0x30 as ::core::ffi::c_int)
                            && ((*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                                || (*s).valid as ::core::ffi::c_int
                                    == TDVALID as ::core::ffi::c_int)
                        {
                            (*s).valid = INVALID as ::core::ffi::c_int as uint8_t;
                        }
                    }
                    s = (*s).next as *mut slist;
                }
                *nversion = bestecversion8;
            } else {
                (*c).set_version(bestecversion4 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                s = (*c).slisthead;
                while !s.is_null() {
                    's_433: {
                        if (*cstab.offset((*s).csid as isize)).valid as ::core::ffi::c_int != 0
                            && (*s).ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                            && (*s).ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
                        {
                            if (*s).valid as ::core::ffi::c_int == WVER as ::core::ffi::c_int {
                                (*s).valid = BUSY as ::core::ffi::c_int as uint8_t;
                            } else if (*s).valid as ::core::ffi::c_int
                                == TDWVER as ::core::ffi::c_int
                            {
                                (*s).valid = TDBUSY as ::core::ffi::c_int as uint8_t;
                            } else {
                                break 's_433;
                            }
                            (*s).version = bestecversion4;
                            stats_chunkops[CHUNK_OP_CHANGE_TRY as usize] =
                                stats_chunkops[CHUNK_OP_CHANGE_TRY as usize].wrapping_add(1);
                            matocsserv_send_setchunkversion(
                                (*cstab.offset((*s).csid as isize)).ptr,
                                ochunkid,
                                (*s).ecid,
                                bestecversion4,
                                0 as uint32_t,
                            );
                        }
                        if (((*s).ecid as ::core::ffi::c_int) < 0x10 as ::core::ffi::c_int
                            || (*s).ecid as ::core::ffi::c_int > 0x1c as ::core::ffi::c_int)
                            && ((*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                                || (*s).valid as ::core::ffi::c_int
                                    == TDVALID as ::core::ffi::c_int)
                        {
                            (*s).valid = INVALID as ::core::ffi::c_int as uint8_t;
                        }
                    }
                    s = (*s).next as *mut slist;
                }
                *nversion = bestecversion4;
            }
            (*c).set_interrupted(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            chunk_set_op(c, SET_VERSION as ::core::ffi::c_int as uint8_t);
            chunk_state_fix(c);
        } else {
            (*c).set_version(bestversion as ::core::ffi::c_uint as ::core::ffi::c_uint);
            s = (*c).slisthead;
            while !s.is_null() {
                if (*s).version == bestversion
                    && (*cstab.offset((*s).csid as isize)).valid as ::core::ffi::c_int != 0
                    && (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    if (*s).valid as ::core::ffi::c_int == WVER as ::core::ffi::c_int {
                        (*s).valid = VALID as ::core::ffi::c_int as uint8_t;
                    } else if (*s).valid as ::core::ffi::c_int == TDWVER as ::core::ffi::c_int {
                        (*s).valid = TDVALID as ::core::ffi::c_int as uint8_t;
                    }
                }
                if (*s).ecid as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                    && ((*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                        || (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int)
                {
                    (*s).valid = INVALID as ::core::ffi::c_int as uint8_t;
                }
                s = (*s).next as *mut slist;
            }
            *nversion = bestversion;
            (*c).set_needverincrease(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            chunk_state_fix(c);
        }
        return 1 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_mr_set_version(
    mut chunkid: uint64_t,
    mut version: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        c = chunk_find(chunkid);
        if c.is_null() {
            return MFS_ERROR_NOCHUNK;
        }
        (*c).set_allowreadzeros(
            (if version & 0x80000000 as uint32_t != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint as ::core::ffi::c_uint,
        );
        (*c).set_version(
            (version & 0x3fffffff as uint32_t) as ::core::ffi::c_uint as ::core::ffi::c_uint,
        );
        meta_version_inc();
        return MFS_STATUS_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_locsort_cmp(
    mut aa: *const ::core::ffi::c_void,
    mut bb: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        let mut a: *const locsort = aa as *const locsort;
        let mut b: *const locsort = bb as *const locsort;
        if (*a).dist < (*b).dist {
            return -1 as ::core::ffi::c_int;
        } else if (*a).dist > (*b).dist {
            return 1 as ::core::ffi::c_int;
        } else if (*a).rnd < (*b).rnd {
            return -1 as ::core::ffi::c_int;
        } else if (*a).rnd > (*b).rnd {
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_get_version_and_csdata(
    mut mode: uint8_t,
    mut chunkid: uint64_t,
    mut clientip: uint32_t,
    mut version: *mut uint32_t,
    mut count: *mut uint8_t,
    mut cs_data: *mut uint8_t,
    mut split: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut i: uint8_t = 0;
        let mut cnt: uint8_t = 0;
        let mut dmask: uint8_t = 0;
        let mut dmask4: uint8_t = 0;
        let mut dmask8: uint8_t = 0;
        let mut minecid: uint8_t = 0;
        let mut maxecid: uint8_t = 0;
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut lstab: [locsort; 100] = [locsort {
            ip: 0,
            port: 0,
            csver: 0,
            labelmask: 0,
            dist: 0,
            rnd: 0,
        }; 100];
        c = chunk_find(chunkid);
        if c.is_null() {
            return MFS_ERROR_NOCHUNK as uint8_t;
        }
        *version = (*c).version() as uint32_t;
        cnt = 0 as uint8_t;
        dmask4 = 0 as uint8_t;
        dmask8 = 0 as uint8_t;
        s = (*c).slisthead;
        while !s.is_null() {
            if (*s).valid as ::core::ffi::c_int != INVALID as ::core::ffi::c_int
                && (*s).valid as ::core::ffi::c_int != DEL as ::core::ffi::c_int
                && (*s).valid as ::core::ffi::c_int != WVER as ::core::ffi::c_int
                && (*s).valid as ::core::ffi::c_int != TDWVER as ::core::ffi::c_int
                && (*cstab.offset((*s).csid as isize)).valid as ::core::ffi::c_int != 0
            {
                if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    if (cnt as ::core::ffi::c_int) < 100 as ::core::ffi::c_int
                        && matocsserv_get_csdata(
                            (*cstab.offset((*s).csid as isize)).ptr,
                            clientip,
                            &raw mut (*(&raw mut lstab as *mut locsort).offset(cnt as isize)).ip,
                            &raw mut (*(&raw mut lstab as *mut locsort).offset(cnt as isize)).port,
                            &raw mut (*(&raw mut lstab as *mut locsort).offset(cnt as isize)).csver,
                            &raw mut (*(&raw mut lstab as *mut locsort).offset(cnt as isize))
                                .labelmask,
                        ) == 0 as ::core::ffi::c_int
                    {
                        lstab[cnt as usize].dist =
                            topology_distance(lstab[cnt as usize].ip, clientip) as uint32_t;
                        lstab[cnt as usize].rnd = rndu32();
                        cnt = cnt.wrapping_add(1);
                    }
                } else if (*s).ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int <= 0x13 as ::core::ffi::c_int
                {
                    dmask4 = (dmask4 as ::core::ffi::c_uint
                        | (1 as ::core::ffi::c_uint)
                            << ((*s).ecid as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int))
                        as uint8_t;
                } else if (*s).ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int <= 0x27 as ::core::ffi::c_int
                {
                    dmask8 = (dmask8 as ::core::ffi::c_uint
                        | (1 as ::core::ffi::c_uint)
                            << ((*s).ecid as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int))
                        as uint8_t;
                }
            }
            s = (*s).next as *mut slist;
        }
        *split = 0 as uint8_t;
        if cnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            dmask = 0 as uint8_t;
            minecid = 0 as uint8_t;
            maxecid = 0 as uint8_t;
            if dmask8 as ::core::ffi::c_int == 0xff as ::core::ffi::c_int {
                minecid = 0x20 as uint8_t;
                maxecid = 0x27 as uint8_t;
                cnt = 8 as uint8_t;
            } else if dmask4 as ::core::ffi::c_int == 0xf as ::core::ffi::c_int {
                minecid = 0x10 as uint8_t;
                maxecid = 0x13 as uint8_t;
                cnt = 4 as uint8_t;
            } else {
                *count = 0 as uint8_t;
                if (*c).all_gequiv() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && chunk_counters_in_progress() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && csdb_have_all_servers() as ::core::ffi::c_int != 0
                {
                    if (*c).allowreadzeros() != 0 {
                        *version = 0 as uint32_t;
                        return MFS_STATUS_OK as uint8_t;
                    }
                    return MFS_ERROR_CHUNKLOST as uint8_t;
                } else {
                    return MFS_STATUS_OK as uint8_t;
                }
            }
            s = (*c).slisthead;
            while !s.is_null() {
                if (*s).ecid as ::core::ffi::c_int >= minecid as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int <= maxecid as ::core::ffi::c_int
                    && ((*s).valid as ::core::ffi::c_int != INVALID as ::core::ffi::c_int
                        && (*s).valid as ::core::ffi::c_int != DEL as ::core::ffi::c_int
                        && (*s).valid as ::core::ffi::c_int != WVER as ::core::ffi::c_int
                        && (*s).valid as ::core::ffi::c_int != TDWVER as ::core::ffi::c_int
                        && (*cstab.offset((*s).csid as isize)).valid as ::core::ffi::c_int != 0)
                {
                    i = ((*s).ecid as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int) as uint8_t;
                    if dmask as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << i as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        matocsserv_get_csdata(
                            (*cstab.offset((*s).csid as isize)).ptr,
                            clientip,
                            &raw mut (*(&raw mut lstab as *mut locsort).offset(i as isize)).ip,
                            &raw mut (*(&raw mut lstab as *mut locsort).offset(i as isize)).port,
                            &raw mut (*(&raw mut lstab as *mut locsort).offset(i as isize)).csver,
                            &raw mut (*(&raw mut lstab as *mut locsort).offset(i as isize))
                                .labelmask,
                        );
                        lstab[i as usize].dist = i as uint32_t;
                        lstab[i as usize].rnd = 0 as uint32_t;
                        dmask = (dmask as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << i as ::core::ffi::c_int)
                            as uint8_t;
                    }
                }
                s = (*s).next as *mut slist;
            }
            *split = cnt;
        }
        qsort(
            &raw mut lstab as *mut locsort as *mut ::core::ffi::c_void,
            cnt as size_t,
            ::core::mem::size_of::<locsort>(),
            Some(
                chunk_locsort_cmp
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        );
        wptr = cs_data as *mut uint8_t;
        i = 0 as uint8_t;
        while (i as ::core::ffi::c_int) < cnt as ::core::ffi::c_int {
            put32bit(&raw mut wptr, lstab[i as usize].ip);
            put16bit(&raw mut wptr, lstab[i as usize].port);
            if mode as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                put32bit(&raw mut wptr, lstab[i as usize].csver);
            }
            if mode as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                put32bit(&raw mut wptr, lstab[i as usize].labelmask);
            }
            i = i.wrapping_add(1);
        }
        *count = cnt;
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_get_eights_copies(
    mut chunkid: uint64_t,
    mut count: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut cnt: uint8_t = 0;
        c = chunk_find(chunkid);
        if c.is_null() {
            return MFS_ERROR_NOCHUNK as uint8_t;
        }
        cnt = 0 as uint8_t;
        s = (*c).slisthead;
        while !s.is_null() && (cnt as ::core::ffi::c_int) < 100 as ::core::ffi::c_int {
            if (*cstab.offset((*s).csid as isize)).valid as ::core::ffi::c_int != 0
                && (*s).valid as ::core::ffi::c_int != DEL as ::core::ffi::c_int
            {
                if matocsserv_isvalid((*cstab.offset((*s).csid as isize)).ptr) != 0 {
                    if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        cnt = (cnt as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as uint8_t;
                    } else if (*s).ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
                    {
                        cnt = (cnt as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint8_t;
                    } else if (*s).ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x30 as ::core::ffi::c_int
                    {
                        cnt = cnt.wrapping_add(1);
                    }
                }
            }
            s = (*s).next as *mut slist;
        }
        *count = cnt;
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_get_version(
    mut chunkid: uint64_t,
    mut version: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        c = chunk_find(chunkid);
        if c.is_null() {
            *version = 0 as uint32_t;
            return MFS_ERROR_NOCHUNK as uint8_t;
        }
        *version = (*c).version() as uint32_t;
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_get_version_and_copies(
    mut mode: uint8_t,
    mut chunkid: uint64_t,
    mut clientip: uint32_t,
    mut version: *mut uint32_t,
    mut chunkmtime: *mut uint32_t,
    mut count: *mut uint8_t,
    mut cs_data: *mut uint8_t,
) -> uint8_t {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut cnt: uint8_t = 0;
        let mut ip: uint32_t = 0;
        let mut port: uint16_t = 0;
        let mut wptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        c = chunk_find(chunkid);
        if c.is_null() {
            return MFS_ERROR_NOCHUNK as uint8_t;
        }
        *version = (*c).version() as uint32_t;
        if (*c).allowreadzeros() != 0 {
            *version =
                (*version as ::core::ffi::c_uint | 0x80000000 as ::core::ffi::c_uint) as uint32_t;
        }
        *chunkmtime = (*c).lockedto;
        cnt = 0 as uint8_t;
        wptr = cs_data as *mut uint8_t;
        s = (*c).slisthead;
        while !s.is_null() && (cnt as ::core::ffi::c_int) < 100 as ::core::ffi::c_int {
            if (*cstab.offset((*s).csid as isize)).valid as ::core::ffi::c_int != 0
                && (*s).valid as ::core::ffi::c_int != DEL as ::core::ffi::c_int
            {
                if matocsserv_get_csdata(
                    (*cstab.offset((*s).csid as isize)).ptr,
                    clientip,
                    &raw mut ip,
                    &raw mut port,
                    ::core::ptr::null_mut::<uint32_t>(),
                    ::core::ptr::null_mut::<uint32_t>(),
                ) == 0 as ::core::ffi::c_int
                {
                    if mode as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
                        || (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        put32bit(&raw mut wptr, ip);
                        put16bit(&raw mut wptr, port);
                        if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                            || (*s).valid as ::core::ffi::c_int == BUSY as ::core::ffi::c_int
                        {
                            put8bit(&raw mut wptr, CHECK_VALID as uint8_t);
                        } else if (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int
                            || (*s).valid as ::core::ffi::c_int == TDBUSY as ::core::ffi::c_int
                        {
                            put8bit(&raw mut wptr, CHECK_MARKEDFORREMOVAL as uint8_t);
                        } else if (*s).valid as ::core::ffi::c_int == WVER as ::core::ffi::c_int {
                            put8bit(&raw mut wptr, CHECK_WRONGVERSION as uint8_t);
                        } else if (*s).valid as ::core::ffi::c_int == TDWVER as ::core::ffi::c_int {
                            put8bit(&raw mut wptr, CHECK_WV_AND_MFR as uint8_t);
                        } else {
                            put8bit(&raw mut wptr, CHECK_INVALID as uint8_t);
                        }
                        if mode as ::core::ffi::c_int >= 1 as ::core::ffi::c_int {
                            put8bit(&raw mut wptr, (*s).ecid);
                        }
                        cnt = cnt.wrapping_add(1);
                    }
                }
            }
            s = (*s).next as *mut slist;
        }
        *count = cnt;
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_mr_nextchunkid(mut nchunkid: uint64_t) -> ::core::ffi::c_int {
    unsafe {
        if nchunkid > nextchunkid {
            nextchunkid = nchunkid;
            meta_version_inc();
            return MFS_STATUS_OK;
        } else {
            return MFS_ERROR_MISMATCH;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_mr_chunkadd(
    mut ts: uint32_t,
    mut chunkid: uint64_t,
    mut version: uint32_t,
    mut lockedto: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        c = chunk_find(chunkid);
        if !c.is_null() {
            return MFS_ERROR_CHUNKEXIST;
        }
        if chunkid > nextchunkid.wrapping_add(1000000000 as uint64_t) {
            return MFS_ERROR_MISMATCH;
        }
        if lockedto > 0 as uint32_t && lockedto < ts {
            return MFS_ERROR_MISMATCH;
        }
        if chunkid >= nextchunkid {
            nextchunkid = chunkid.wrapping_add(1 as uint64_t);
        }
        c = chunk_new(chunkid);
        (*c).set_version(version as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*c).lockedto = lockedto;
        meta_version_inc();
        return MFS_STATUS_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_mr_chunkdel(
    mut ts: uint32_t,
    mut chunkid: uint64_t,
    mut version: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        c = chunk_find(chunkid);
        if c.is_null() {
            return MFS_ERROR_NOCHUNK;
        }
        if (*c).version() as uint32_t != version {
            return MFS_ERROR_WRONGVERSION;
        }
        if (*c).fhead != FLISTNULLINDX as uint32_t {
            return MFS_ERROR_ACTIVE;
        }
        if !(*c).slisthead.is_null() {
            return MFS_ERROR_CHUNKBUSY;
        }
        if (*c).lockedto >= ts {
            return MFS_ERROR_LOCKED;
        }
        chunk_delete(c);
        meta_version_inc();
        return MFS_STATUS_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_mr_flagsclr(
    mut ts: uint32_t,
    mut chunkid: uint64_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        c = chunk_find(chunkid);
        if c.is_null() {
            return MFS_ERROR_NOCHUNK;
        }
        if (*c).lockedto >= ts {
            return MFS_ERROR_LOCKED;
        }
        chunk_state_set_flags(c, 0 as uint8_t);
        meta_version_inc();
        return MFS_STATUS_OK;
    }
}
#[inline]
unsafe extern "C" fn chunk_mfr_state_check(mut c: *mut chunk) {
    unsafe {
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut gequiv: uint8_t = 0;
        gequiv = sclass_calc_goal_equivalent(sclass_get_keeparch_storagemode(
            (*c).sclassid as uint16_t,
            (*c).flags() as uint8_t,
        ));
        if (*c).all_gequiv() as ::core::ffi::c_int >= gequiv as ::core::ffi::c_int
            && ((*c).reg_gequiv() as ::core::ffi::c_int) < gequiv as ::core::ffi::c_int
        {
            s = (*c).slisthead;
            while !s.is_null() {
                if (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int
                    || (*s).valid as ::core::ffi::c_int == TDBUSY as ::core::ffi::c_int
                {
                    (*cstab.offset((*s).csid as isize)).set_mfr_state(
                        REPL_IN_PROGRESS as ::core::ffi::c_int as ::core::ffi::c_uint
                            as ::core::ffi::c_uint,
                    );
                }
                s = (*s).next as *mut slist;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_server_has_chunk(
    mut csid: uint16_t,
    mut chunkid: uint64_t,
    mut ecid: uint8_t,
    mut version: uint32_t,
) {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut fix: uint8_t = 0;
        static mut loglastts: uint32_t = 0 as uint32_t;
        static mut ilogcount: uint32_t = 0 as uint32_t;
        static mut clogcount: uint32_t = 0 as uint32_t;
        if chunk_check_ecid(ecid) < 0 as ::core::ffi::c_int {
            return;
        }
        c = chunk_find(chunkid);
        if !c.is_null() {
            if chunk_remove_disconnected_chunks(c) != 0 {
                c = ::core::ptr::null_mut::<chunk>();
            }
        }
        if c.is_null() {
            if loglastts.wrapping_add(60 as uint32_t) < main_time() {
                ilogcount = 0 as uint32_t;
                clogcount = 0 as uint32_t;
                loglastts = main_time();
            }
            if chunkid == 0 as uint64_t
                || chunkid > nextchunkid.wrapping_add(1000000000 as uint64_t)
            {
                if ilogcount < 10 as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"chunkserver (%s) has nonexistent chunk (%016lX_%08X), id looks wrong - just ignore it\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        matocsserv_getstrip((*cstab.offset(csid as isize)).ptr),
                        chunkid,
                        version & 0x7fffffff as uint32_t,
                    );
                } else if ilogcount == 10 as uint32_t {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"there are more nonexistent chunks to ignore - stop logging\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                ilogcount = ilogcount.wrapping_add(1);
                return;
            }
            if clogcount < 10 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"chunkserver (%s) has nonexistent chunk (%016lX_%08X), so create it for future deletion\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    matocsserv_getstrip((*cstab.offset(csid as isize)).ptr),
                    chunkid,
                    version & 0x7fffffff as uint32_t,
                );
            } else if clogcount == 10 as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"there are more nonexistent chunks to create - stop logging\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            clogcount = clogcount.wrapping_add(1);
            if chunkid >= nextchunkid {
                nextchunkid = chunkid.wrapping_add(1 as uint64_t);
            }
            c = chunk_new(chunkid);
            (*c).set_version(
                (version & 0x7fffffff as uint32_t) as ::core::ffi::c_uint as ::core::ffi::c_uint,
            );
            (*c).lockedto = main_time().wrapping_add(UNUSED_DELETE_TIMEOUT as uint32_t);
            changelog(
                b"%u|CHUNKADD(%lu,%u,%u)\0".as_ptr() as *const ::core::ffi::c_char,
                main_time(),
                (*c).chunkid,
                (*c).version() as ::core::ffi::c_int,
                (*c).lockedto,
            );
        }
        s = (*c).slisthead;
        while !s.is_null() {
            if (*s).csid as ::core::ffi::c_int == csid as ::core::ffi::c_int
                && (*s).ecid as ::core::ffi::c_int == ecid as ::core::ffi::c_int
            {
                let mut nextvalid: uint8_t = 0;
                if (*s).valid as ::core::ffi::c_int == INVALID as ::core::ffi::c_int
                    || (*s).valid as ::core::ffi::c_int == DEL as ::core::ffi::c_int
                    || (*s).valid as ::core::ffi::c_int == WVER as ::core::ffi::c_int
                    || (*s).valid as ::core::ffi::c_int == TDWVER as ::core::ffi::c_int
                {
                    return;
                }
                if (*s).valid as ::core::ffi::c_int == BUSY as ::core::ffi::c_int
                    || (*s).valid as ::core::ffi::c_int == TDBUSY as ::core::ffi::c_int
                {
                    if version & 0x80000000 as uint32_t != 0 {
                        nextvalid = TDBUSY as ::core::ffi::c_int as uint8_t;
                    } else {
                        nextvalid = BUSY as ::core::ffi::c_int as uint8_t;
                    }
                } else if version & 0x80000000 as uint32_t != 0 {
                    nextvalid = TDVALID as ::core::ffi::c_int as uint8_t;
                } else {
                    nextvalid = VALID as ::core::ffi::c_int as uint8_t;
                }
                if (*s).valid as ::core::ffi::c_int == nextvalid as ::core::ffi::c_int {
                    return;
                }
                (*s).valid = nextvalid;
                chunk_state_fix(c);
                if version & 0x80000000 as uint32_t != 0 {
                    chunk_mfr_state_check(c);
                }
                return;
            }
            s = (*s).next as *mut slist;
        }
        fix = 0 as uint8_t;
        s = slist_malloc();
        (*s).csid = csid;
        (*s).ecid = ecid;
        if (*c).version() as uint32_t != version & 0x7fffffff as uint32_t {
            if version & 0x80000000 as uint32_t != 0 {
                (*s).valid = TDWVER as ::core::ffi::c_int as uint8_t;
            } else {
                (*s).valid = WVER as ::core::ffi::c_int as uint8_t;
            }
            (*s).version = version & 0x7fffffff as uint32_t;
        } else {
            if (*c).writeinprogress() as ::core::ffi::c_int != 0
                && ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                matocsserv_write_counters((*cstab.offset(csid as isize)).ptr, 1 as uint8_t);
            }
            if version & 0x80000000 as uint32_t != 0 {
                (*s).valid = TDVALID as ::core::ffi::c_int as uint8_t;
                (*s).version = (*c).version() as uint32_t;
                if ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && (*c).storage_mode() as ::core::ffi::c_int == STORAGE_MODE_COPIES
                    && ((*c).all_gequiv() as ::core::ffi::c_int) < 15 as ::core::ffi::c_int
                {
                    chunk_state_set_counters(
                        c,
                        STORAGE_MODE_COPIES as uint8_t,
                        ((*c).all_gequiv() as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            as uint8_t,
                        (*c).reg_gequiv() as uint8_t,
                    );
                } else {
                    fix = 1 as uint8_t;
                }
            } else {
                (*s).valid = VALID as ::core::ffi::c_int as uint8_t;
                (*s).version = (*c).version() as uint32_t;
                if ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && (*c).storage_mode() as ::core::ffi::c_int == STORAGE_MODE_COPIES
                    && ((*c).all_gequiv() as ::core::ffi::c_int) < 15 as ::core::ffi::c_int
                    && ((*c).reg_gequiv() as ::core::ffi::c_int) < 15 as ::core::ffi::c_int
                {
                    chunk_state_set_counters(
                        c,
                        STORAGE_MODE_COPIES as uint8_t,
                        ((*c).all_gequiv() as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            as uint8_t,
                        ((*c).reg_gequiv() as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            as uint8_t,
                    );
                } else {
                    fix = 1 as uint8_t;
                }
            }
        }
        chunk_new_copy(c, s);
        (*c).set_needverincrease(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        if fix != 0 {
            chunk_state_fix(c);
        }
        if version & 0x80000000 as uint32_t != 0 {
            chunk_mfr_state_check(c);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_damaged(
    mut csid: uint16_t,
    mut chunkid: uint64_t,
    mut ecid: uint8_t,
) {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        if chunk_check_ecid(ecid) < 0 as ::core::ffi::c_int {
            return;
        }
        c = chunk_find(chunkid);
        if c.is_null() {
            if chunkid > nextchunkid.wrapping_add(1000000000 as uint64_t) {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"chunkserver has nonexistent chunk (%016lX), id looks wrong - just ignore it\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    chunkid,
                );
                return;
            }
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"chunkserver has nonexistent chunk (%016lX), so create it for future deletion\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                chunkid,
            );
            if chunkid >= nextchunkid {
                nextchunkid = chunkid.wrapping_add(1 as uint64_t);
            }
            c = chunk_new(chunkid);
            (*c).set_version(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            changelog(
                b"%u|CHUNKADD(%lu,%u,%u)\0".as_ptr() as *const ::core::ffi::c_char,
                main_time(),
                (*c).chunkid,
                (*c).version() as ::core::ffi::c_int,
                (*c).lockedto,
            );
        }
        s = (*c).slisthead;
        while !s.is_null() {
            if (*s).csid as ::core::ffi::c_int == csid as ::core::ffi::c_int
                && (*s).ecid as ::core::ffi::c_int == ecid as ::core::ffi::c_int
            {
                if (*c).writeinprogress() as ::core::ffi::c_int != 0
                    && (*s).valid as ::core::ffi::c_int != INVALID as ::core::ffi::c_int
                    && (*s).valid as ::core::ffi::c_int != DEL as ::core::ffi::c_int
                    && (*s).valid as ::core::ffi::c_int != WVER as ::core::ffi::c_int
                    && (*s).valid as ::core::ffi::c_int != TDWVER as ::core::ffi::c_int
                    && ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    matocsserv_write_counters((*cstab.offset(csid as isize)).ptr, 0 as uint8_t);
                }
                (*s).valid = INVALID as ::core::ffi::c_int as uint8_t;
                (*s).version = 0 as uint32_t;
                (*c).set_needverincrease(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                chunk_state_fix(c);
                chunk_priority_queue_check(c, 1 as uint8_t);
                chunk_mfr_state_check(c);
                return;
            }
            s = (*s).next as *mut slist;
        }
        s = slist_malloc();
        (*s).csid = csid;
        (*s).ecid = ecid;
        (*s).valid = INVALID as ::core::ffi::c_int as uint8_t;
        (*s).version = 0 as uint32_t;
        chunk_new_copy(c, s);
        (*c).set_needverincrease(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_lost(
    mut csid: uint16_t,
    mut chunkid: uint64_t,
    mut ecid: uint8_t,
    mut report: uint8_t,
) {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut sptr: *mut *mut slist = ::core::ptr::null_mut::<*mut slist>();
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        if chunk_check_ecid(ecid) < 0 as ::core::ffi::c_int {
            return;
        }
        c = chunk_find(chunkid);
        if c.is_null() {
            return;
        }
        sptr = &raw mut (*c).slisthead;
        loop {
            s = *sptr;
            if s.is_null() {
                break;
            }
            if (*s).csid as ::core::ffi::c_int == csid as ::core::ffi::c_int
                && (*s).ecid as ::core::ffi::c_int == ecid as ::core::ffi::c_int
            {
                if report != 0 {
                    if ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"chunkserver (%s) reported nonexistent chunk copy (chunkid: %016lX)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            matocsserv_getstrip((*cstab.offset(csid as isize)).ptr),
                            chunkid,
                        );
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"chunkserver (%s) reported nonexistent chunk part (chunkid: %016lX ; ecid: %s)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            matocsserv_getstrip((*cstab.offset(csid as isize)).ptr),
                            chunkid,
                            chunk_ecid_to_str(ecid),
                        );
                    }
                }
                if (*c).writeinprogress() as ::core::ffi::c_int != 0
                    && (*s).valid as ::core::ffi::c_int != INVALID as ::core::ffi::c_int
                    && (*s).valid as ::core::ffi::c_int != DEL as ::core::ffi::c_int
                    && (*s).valid as ::core::ffi::c_int != WVER as ::core::ffi::c_int
                    && (*s).valid as ::core::ffi::c_int != TDWVER as ::core::ffi::c_int
                    && ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    matocsserv_write_counters((*cstab.offset(csid as isize)).ptr, 0 as uint8_t);
                }
                (*c).set_needverincrease(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                *sptr = (*s).next as *mut slist;
                slist_free(s);
                chunk_state_fix(c);
            } else {
                sptr = &raw mut (*s).next as *mut *mut slist;
            }
        }
        if (*c).lockedto < main_time()
            && (*c).operation() as ::core::ffi::c_int == NONE as ::core::ffi::c_int
            && (*c).slisthead.is_null()
            && (*c).fhead == FLISTNULLINDX as uint32_t
            && chunk_counters_in_progress() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && csdb_have_all_servers() as ::core::ffi::c_int != 0
        {
            changelog(
                b"%u|CHUNKDEL(%lu,%u)\0".as_ptr() as *const ::core::ffi::c_char,
                main_time(),
                (*c).chunkid,
                (*c).version() as ::core::ffi::c_int,
            );
            if (*c).ondangerlist() != 0 {
                chunk_priority_remove(c);
            }
            chunk_delete(c);
        } else {
            chunk_priority_queue_check(c, 1 as uint8_t);
            chunk_mfr_state_check(c);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_get_mfrstatus(mut csid: uint16_t) -> uint8_t {
    unsafe {
        if (csid as ::core::ffi::c_int) < MAXCSCOUNT {
            match (*cstab.offset(csid as isize)).mfr_state() as ::core::ffi::c_int {
                0 | 1 => return MFRSTATUS_VALIDATING as uint8_t,
                2 => return MFRSTATUS_READY as uint8_t,
                3 | 4 => return MFRSTATUS_INPROGRESS as uint8_t,
                _ => {}
            }
        }
        return MFRSTATUS_VALIDATING as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn chunk_server_remove_csid(mut csid: uint16_t) {
    unsafe {
        if (*cstab.offset(csid as isize)).prev < MAXCSCOUNT as uint32_t {
            (*cstab.offset((*cstab.offset(csid as isize)).prev as isize)).next =
                (*cstab.offset(csid as isize)).next;
        } else {
            csusedhead = (*cstab.offset(csid as isize)).next;
        }
        if (*cstab.offset(csid as isize)).next < MAXCSCOUNT as uint32_t {
            (*cstab.offset((*cstab.offset(csid as isize)).next as isize)).prev =
                (*cstab.offset(csid as isize)).prev;
        }
        (*cstab.offset(csid as isize)).next = MAXCSCOUNT as uint32_t;
        (*cstab.offset(csid as isize)).prev = csfreetail;
        if csfreetail < MAXCSCOUNT as uint32_t {
            (*cstab.offset(csfreetail as isize)).next = csid as uint32_t;
        } else {
            csfreehead = csid as uint32_t;
        }
        csfreetail = csid as uint32_t;
    }
}
#[inline]
unsafe extern "C" fn chunk_server_new_csid() -> uint16_t {
    unsafe {
        let mut csid: uint16_t = 0;
        csid = csfreehead as uint16_t;
        csfreehead = (*cstab.offset(csid as isize)).next;
        (*cstab.offset(csfreehead as isize)).prev = MAXCSCOUNT as uint32_t;
        if csusedhead < MAXCSCOUNT as uint32_t {
            (*cstab.offset(csusedhead as isize)).prev = csid as uint32_t;
        }
        (*cstab.offset(csid as isize)).next = csusedhead;
        csusedhead = csid as uint32_t;
        return csid;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_server_connected(mut ptr: *mut ::core::ffi::c_void) -> uint16_t {
    unsafe {
        let mut csid: uint16_t = 0;
        csid = chunk_server_new_csid();
        (*cstab.offset(csid as isize)).ptr = ptr;
        (*cstab.offset(csid as isize)).opchunks = ::core::ptr::null_mut::<csopchunk>();
        (*cstab.offset(csid as isize)).valid = 1 as uint8_t;
        (*cstab.offset(csid as isize))
            .set_registered(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*cstab.offset(csid as isize)).set_mfr_state(
            UNKNOWN_HARD as ::core::ffi::c_int as ::core::ffi::c_uint as ::core::ffi::c_uint,
        );
        csregisterinprogress =
            (csregisterinprogress as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint16_t;
        return csid;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_server_register_end(mut csid: uint16_t) {
    unsafe {
        if (*cstab.offset(csid as isize)).registered() as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            && (*cstab.offset(csid as isize)).valid as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        {
            (*cstab.offset(csid as isize))
                .set_registered(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            csregisterinprogress =
                (csregisterinprogress as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as uint16_t;
        }
        if csregisterinprogress as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            matoclserv_fuse_invalidate_chunk_cache();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_server_disconnected(mut csid: uint16_t) {
    unsafe {
        let mut ds: *mut discserv = ::core::ptr::null_mut::<discserv>();
        let mut csop: *mut csopchunk = ::core::ptr::null_mut::<csopchunk>();
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        ds = malloc(::core::mem::size_of::<discserv>()) as *mut discserv;
        (*ds).csid = csid;
        (*ds).next = discservers_next as *mut _discserv;
        discservers_next = ds;
        (*cstab.offset(csid as isize)).valid = 0 as uint8_t;
        if (*cstab.offset(csid as isize)).registered() as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            csregisterinprogress =
                (csregisterinprogress as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as uint16_t;
        }
        csop = (*cstab.offset(csid as isize)).opchunks;
        while !csop.is_null() {
            c = chunk_find((*csop).chunkid);
            if !c.is_null() {
                chunk_remove_disconnected_chunks(c);
            }
            csop = (*csop).next as *mut csopchunk;
            if opsinprogress > 0 as uint32_t {
                opsinprogress = opsinprogress.wrapping_sub(1);
            }
        }
        (*cstab.offset(csid as isize)).opchunks = ::core::ptr::null_mut::<csopchunk>();
        csid = csusedhead as uint16_t;
        while (csid as ::core::ffi::c_int) < MAXCSCOUNT {
            (*cstab.offset(csid as isize)).set_mfr_state(
                UNKNOWN_HARD as ::core::ffi::c_int as ::core::ffi::c_uint as ::core::ffi::c_uint,
            );
            csid = (*cstab.offset(csid as isize)).next as uint16_t;
        }
        matoclserv_fuse_invalidate_chunk_cache();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_server_disconnection_loop() {
    unsafe {
        let mut i: uint32_t = 0;
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut cn: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut ds: *mut discserv = ::core::ptr::null_mut::<discserv>();
        let mut startutime: uint64_t = 0;
        let mut currutime: uint64_t = 0;
        static mut discserverspos: uint32_t = 0 as uint32_t;
        if !discservers.is_null() {
            startutime = monotonic_useconds();
            currutime = startutime;
            while startutime
                .wrapping_add(JobsTimerMilliSeconds.wrapping_mul(200 as uint32_t) as uint64_t)
                > currutime
            {
                i = 0 as uint32_t;
                while i < 100 as uint32_t {
                    if discserverspos < chunkrehashpos {
                        c = *chunkhashtab[(discserverspos >> HASHTAB_LOBITS) as usize]
                            .offset((discserverspos & HASHTAB_MASK as uint32_t) as isize);
                        while !c.is_null() {
                            cn = (*c).next as *mut chunk;
                            chunk_remove_disconnected_chunks(c);
                            c = cn;
                        }
                        discserverspos = discserverspos.wrapping_add(1);
                    } else {
                        while !discservers.is_null() {
                            ds = discservers;
                            discservers = (*ds).next as *mut discserv;
                            chunk_server_remove_csid((*ds).csid);
                            matocsserv_disconnection_finished(
                                (*cstab.offset((*ds).csid as isize)).ptr,
                            );
                            free(ds as *mut ::core::ffi::c_void);
                        }
                        return;
                    }
                    i = i.wrapping_add(1);
                }
                currutime = monotonic_useconds();
            }
        } else if !discservers_next.is_null() {
            discservers = discservers_next;
            discservers_next = ::core::ptr::null_mut::<discserv>();
            discserverspos = 0 as uint32_t;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_got_delete_status(
    mut csid: uint16_t,
    mut chunkid: uint64_t,
    mut ecid: uint8_t,
    mut status: uint8_t,
) {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut st: *mut *mut slist = ::core::ptr::null_mut::<*mut slist>();
        let mut fix: uint8_t = 0;
        if chunk_check_ecid(ecid) < 0 as ::core::ffi::c_int {
            return;
        }
        if status as ::core::ffi::c_int == MFS_ERROR_NOCHUNK {
            status = MFS_STATUS_OK as uint8_t;
        }
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            stats_chunkops[CHUNK_OP_DELETE_OK as usize] =
                stats_chunkops[CHUNK_OP_DELETE_OK as usize].wrapping_add(1);
        } else {
            stats_chunkops[CHUNK_OP_DELETE_ERR as usize] =
                stats_chunkops[CHUNK_OP_DELETE_ERR as usize].wrapping_add(1);
        }
        c = chunk_find(chunkid);
        if c.is_null() {
            return;
        }
        fix = 0 as uint8_t;
        st = &raw mut (*c).slisthead;
        while !(*st).is_null() {
            s = *st;
            if (*s).csid as ::core::ffi::c_int == csid as ::core::ffi::c_int
                && (*s).ecid as ::core::ffi::c_int == ecid as ::core::ffi::c_int
            {
                if status as ::core::ffi::c_int == MFS_STATUS_OK {
                    if (*s).valid as ::core::ffi::c_int != DEL as ::core::ffi::c_int {
                        fix = 1 as uint8_t;
                        if (*c).writeinprogress() as ::core::ffi::c_int != 0
                            && (*s).valid as ::core::ffi::c_int != INVALID as ::core::ffi::c_int
                            && (*s).valid as ::core::ffi::c_int != DEL as ::core::ffi::c_int
                            && (*s).valid as ::core::ffi::c_int != WVER as ::core::ffi::c_int
                            && (*s).valid as ::core::ffi::c_int != TDWVER as ::core::ffi::c_int
                            && ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        {
                            matocsserv_write_counters(
                                (*cstab.offset((*s).csid as isize)).ptr,
                                0 as uint8_t,
                            );
                        }
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"chunk %016lX_%08X - got unexpected delete status from %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            chunkid,
                            (*c).version() as ::core::ffi::c_int,
                            matocsserv_getstrip((*cstab.offset(csid as isize)).ptr),
                        );
                    }
                    *st = (*s).next as *mut slist;
                    slist_free(s);
                } else {
                    if (*s).valid as ::core::ffi::c_int == DEL as ::core::ffi::c_int {
                        (*s).valid = INVALID as ::core::ffi::c_int as uint8_t;
                    }
                    st = &raw mut (*s).next as *mut *mut slist;
                }
            } else {
                st = &raw mut (*s).next as *mut *mut slist;
            }
        }
        if fix != 0 {
            chunk_state_fix(c);
        }
        if (*c).lockedto < main_time()
            && (*c).operation() as ::core::ffi::c_int == NONE as ::core::ffi::c_int
            && (*c).slisthead.is_null()
            && (*c).fhead == FLISTNULLINDX as uint32_t
            && chunk_counters_in_progress() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && csdb_have_all_servers() as ::core::ffi::c_int != 0
        {
            changelog(
                b"%u|CHUNKDEL(%lu,%u)\0".as_ptr() as *const ::core::ffi::c_char,
                main_time(),
                (*c).chunkid,
                (*c).version() as ::core::ffi::c_int,
            );
            if (*c).ondangerlist() != 0 {
                chunk_priority_remove(c);
            }
            chunk_delete(c);
            return;
        }
        if (*c).operation() as ::core::ffi::c_int == NONE as ::core::ffi::c_int {
            chunk_priority_queue_check(c, 1 as uint8_t);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_got_replicate_status(
    mut csid: uint16_t,
    mut chunkid: uint64_t,
    mut ecid: uint8_t,
    mut version: uint32_t,
    mut status: uint8_t,
) {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut st: *mut *mut slist = ::core::ptr::null_mut::<*mut slist>();
        let mut fix: uint8_t = 0;
        let mut finished: uint8_t = 0;
        let mut now: uint32_t = 0;
        if chunk_check_ecid(ecid) < 0 as ::core::ffi::c_int {
            return;
        }
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            stats_chunkops[CHUNK_OP_REPLICATE_OK as usize] =
                stats_chunkops[CHUNK_OP_REPLICATE_OK as usize].wrapping_add(1);
        } else {
            stats_chunkops[CHUNK_OP_REPLICATE_ERR as usize] =
                stats_chunkops[CHUNK_OP_REPLICATE_ERR as usize].wrapping_add(1);
        }
        c = chunk_find(chunkid);
        if c.is_null() {
            return;
        }
        now = main_time();
        if (*c).operation() as ::core::ffi::c_int == REPLICATE as ::core::ffi::c_int {
            fix = 0 as uint8_t;
            finished = 1 as uint8_t;
            if status as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                st = &raw mut (*c).slisthead;
                while !(*st).is_null() {
                    s = *st;
                    if (*s).csid as ::core::ffi::c_int == csid as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int == ecid as ::core::ffi::c_int
                        && (*s).valid as ::core::ffi::c_int == BUSY as ::core::ffi::c_int
                    {
                        fix = 1 as uint8_t;
                        *st = (*s).next as *mut slist;
                        chunk_delopchunk((*s).csid, (*c).chunkid);
                        slist_free(s);
                    } else {
                        if (*s).valid as ::core::ffi::c_int == BUSY as ::core::ffi::c_int {
                            finished = 0 as uint8_t;
                        }
                        st = &raw mut (*s).next as *mut *mut slist;
                    }
                }
            }
            if fix as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                finished = 1 as uint8_t;
                s = (*c).slisthead;
                while !s.is_null() {
                    if (*s).csid as ::core::ffi::c_int == csid as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int == ecid as ::core::ffi::c_int
                    {
                        if (*s).valid as ::core::ffi::c_int != BUSY as ::core::ffi::c_int {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"chunk %016lX_%08X - got replication status from server not set as busy (server: %s)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                chunkid,
                                version,
                                matocsserv_getstrip((*cstab.offset(csid as isize)).ptr),
                            );
                        }
                        if status as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                            || version != (*c).version() as uint32_t
                        {
                            fix = 1 as uint8_t;
                            if (*c).writeinprogress() as ::core::ffi::c_int != 0
                                && (*s).valid as ::core::ffi::c_int != INVALID as ::core::ffi::c_int
                                && (*s).valid as ::core::ffi::c_int != DEL as ::core::ffi::c_int
                                && (*s).valid as ::core::ffi::c_int != WVER as ::core::ffi::c_int
                                && (*s).valid as ::core::ffi::c_int != TDWVER as ::core::ffi::c_int
                                && ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            {
                                matocsserv_write_counters(
                                    (*cstab.offset((*s).csid as isize)).ptr,
                                    0 as uint8_t,
                                );
                            }
                            (*s).valid = INVALID as ::core::ffi::c_int as uint8_t;
                            (*s).version = 0 as uint32_t;
                        } else if (*s).valid as ::core::ffi::c_int == BUSY as ::core::ffi::c_int
                            || (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                        {
                            (*s).valid = VALID as ::core::ffi::c_int as uint8_t;
                        }
                        chunk_delopchunk((*s).csid, (*c).chunkid);
                    } else if (*s).valid as ::core::ffi::c_int == BUSY as ::core::ffi::c_int {
                        finished = 0 as uint8_t;
                    }
                    s = (*s).next as *mut slist;
                }
            }
            if fix != 0 {
                chunk_state_fix(c);
            }
            if finished != 0 {
                chunk_set_op(c, NONE as ::core::ffi::c_int as uint8_t);
                chunk_replock_repend((*c).chunkid);
                if status as ::core::ffi::c_int != MFS_ERROR_ETIMEDOUT {
                    chunk_io_ready_check(c);
                }
                if (*c).operation() as ::core::ffi::c_int == NONE as ::core::ffi::c_int
                    && (*c).ondangerlist() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    matoclserv_chunk_status((*c).chunkid, MFS_STATUS_OK as uint8_t);
                }
                if (*c).lockedto < now
                    && chunk_replock_test((*c).chunkid, now) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                {
                    matoclserv_chunk_unlocked((*c).chunkid, c as *mut ::core::ffi::c_void);
                }
            }
        } else {
            if status as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                chunk_priority_queue_check(c, 1 as uint8_t);
                return;
            }
            fix = 0 as uint8_t;
            s = (*c).slisthead;
            while !s.is_null() {
                if (*s).csid as ::core::ffi::c_int == csid as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int == ecid as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"chunk %016lX_%08X - got replication status from server which had had that chunk before (server: %s)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        chunkid,
                        version,
                        matocsserv_getstrip((*cstab.offset(csid as isize)).ptr),
                    );
                    if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                        && version != (*c).version() as uint32_t
                    {
                        fix = 1 as uint8_t;
                        (*s).valid = INVALID as ::core::ffi::c_int as uint8_t;
                        (*s).version = version;
                        if (*c).writeinprogress() as ::core::ffi::c_int != 0
                            && ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        {
                            matocsserv_write_counters(
                                (*cstab.offset((*s).csid as isize)).ptr,
                                0 as uint8_t,
                            );
                        }
                    }
                    if fix != 0 {
                        chunk_state_fix(c);
                    }
                    chunk_priority_queue_check(c, 1 as uint8_t);
                    return;
                }
                s = (*s).next as *mut slist;
            }
            s = slist_malloc();
            (*s).csid = csid;
            (*s).ecid = ecid;
            if (*c).lockedto >= main_time() || version != (*c).version() as uint32_t {
                (*s).valid = INVALID as ::core::ffi::c_int as uint8_t;
            } else {
                chunk_write_counters(c, 0 as uint8_t);
                fix = 1 as uint8_t;
                (*s).valid = VALID as ::core::ffi::c_int as uint8_t;
            }
            (*s).version = version;
            chunk_new_copy(c, s);
            if fix != 0 {
                chunk_state_fix(c);
            }
        }
        if (*c).operation() as ::core::ffi::c_int == NONE as ::core::ffi::c_int {
            chunk_priority_queue_check(c, 1 as uint8_t);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_operation_status(
    mut chunkid: uint64_t,
    mut ecid: uint8_t,
    mut status: uint8_t,
    mut csid: uint16_t,
    mut operation: uint8_t,
) {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut opfinished: uint8_t = 0;
        let mut validcopies: uint8_t = 0;
        let mut verfixed: uint8_t = 0;
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut st: *mut *mut slist = ::core::ptr::null_mut::<*mut slist>();
        let mut fix: uint8_t = 0;
        let mut mask4: uint32_t = 0;
        let mut mask8: uint32_t = 0;
        let mut now: uint32_t = 0;
        if chunk_check_ecid(ecid) < 0 as ::core::ffi::c_int {
            return;
        }
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            if operation as ::core::ffi::c_int == CREATE as ::core::ffi::c_int {
                stats_chunkops[CHUNK_OP_CREATE_OK as usize] =
                    stats_chunkops[CHUNK_OP_CREATE_OK as usize].wrapping_add(1);
            } else {
                stats_chunkops[CHUNK_OP_CHANGE_OK as usize] =
                    stats_chunkops[CHUNK_OP_CHANGE_OK as usize].wrapping_add(1);
            }
        } else if operation as ::core::ffi::c_int == CREATE as ::core::ffi::c_int {
            stats_chunkops[CHUNK_OP_CREATE_ERR as usize] =
                stats_chunkops[CHUNK_OP_CREATE_ERR as usize].wrapping_add(1);
        } else {
            stats_chunkops[CHUNK_OP_CHANGE_ERR as usize] =
                stats_chunkops[CHUNK_OP_CHANGE_ERR as usize].wrapping_add(1);
        }
        c = chunk_find(chunkid);
        if c.is_null() {
            return;
        }
        if chunk_remove_disconnected_chunks(c) != 0 {
            return;
        }
        if (*c).operation() as ::core::ffi::c_int != operation as ::core::ffi::c_int
            && operation as ::core::ffi::c_int != NONE as ::core::ffi::c_int
        {
            let mut eop: uint8_t = 0;
            let mut sop: uint8_t = 0;
            eop = (*c).operation() as uint8_t;
            sop = operation;
            if eop as ::core::ffi::c_int > 8 as ::core::ffi::c_int {
                eop = 8 as uint8_t;
            }
            if sop as ::core::ffi::c_int > 8 as ::core::ffi::c_int {
                sop = 8 as uint8_t;
            }
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"chunk %016lX_%08X - got unexpected status (expected: %s ; got: %s) from %s\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                chunkid,
                (*c).version() as ::core::ffi::c_int,
                op_to_str(eop),
                op_to_str(sop),
                matocsserv_getstrip((*cstab.offset(csid as isize)).ptr),
            );
        }
        now = main_time();
        fix = 0 as uint8_t;
        validcopies = 0 as uint8_t;
        opfinished = 1 as uint8_t;
        mask4 = 0 as uint32_t;
        mask8 = 0 as uint32_t;
        st = &raw mut (*c).slisthead;
        loop {
            s = *st;
            if s.is_null() {
                break;
            }
            if (*s).csid as ::core::ffi::c_int == csid as ::core::ffi::c_int
                && (*s).ecid as ::core::ffi::c_int == ecid as ::core::ffi::c_int
            {
                if status as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    fix = 1 as uint8_t;
                    if status as ::core::ffi::c_int == MFS_ERROR_NOTDONE {
                        if (*c).operation() as ::core::ffi::c_int
                            == SET_VERSION as ::core::ffi::c_int
                            || (*c).operation() as ::core::ffi::c_int
                                == TRUNCATE as ::core::ffi::c_int
                        {
                            if (*s).valid as ::core::ffi::c_int == TDBUSY as ::core::ffi::c_int
                                || (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int
                            {
                                (*s).valid = TDWVER as ::core::ffi::c_int as uint8_t;
                            } else {
                                (*s).valid = WVER as ::core::ffi::c_int as uint8_t;
                            }
                            (*s).version = (*s).version.wrapping_sub(1);
                        } else if (*c).operation() as ::core::ffi::c_int
                            == CREATE as ::core::ffi::c_int
                            || (*c).operation() as ::core::ffi::c_int
                                == DUPLICATE as ::core::ffi::c_int
                            || (*c).operation() as ::core::ffi::c_int
                                == DUPTRUNC as ::core::ffi::c_int
                        {
                            if (*c).writeinprogress() as ::core::ffi::c_int != 0
                                && (*s).valid as ::core::ffi::c_int != INVALID as ::core::ffi::c_int
                                && (*s).valid as ::core::ffi::c_int != DEL as ::core::ffi::c_int
                                && (*s).valid as ::core::ffi::c_int != WVER as ::core::ffi::c_int
                                && (*s).valid as ::core::ffi::c_int != TDWVER as ::core::ffi::c_int
                                && ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            {
                                matocsserv_write_counters(
                                    (*cstab.offset((*s).csid as isize)).ptr,
                                    0 as uint8_t,
                                );
                            }
                            *st = (*s).next as *mut slist;
                            slist_free(s);
                            continue;
                        }
                    } else {
                        if (*c).writeinprogress() as ::core::ffi::c_int != 0
                            && (*s).valid as ::core::ffi::c_int != INVALID as ::core::ffi::c_int
                            && (*s).valid as ::core::ffi::c_int != DEL as ::core::ffi::c_int
                            && (*s).valid as ::core::ffi::c_int != WVER as ::core::ffi::c_int
                            && (*s).valid as ::core::ffi::c_int != TDWVER as ::core::ffi::c_int
                            && ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        {
                            matocsserv_write_counters(
                                (*cstab.offset((*s).csid as isize)).ptr,
                                0 as uint8_t,
                            );
                        }
                        (*c).set_interrupted(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        (*s).valid = INVALID as ::core::ffi::c_int as uint8_t;
                        (*s).version = 0 as uint32_t;
                    }
                } else if (*s).valid as ::core::ffi::c_int == TDBUSY as ::core::ffi::c_int
                    || (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int
                {
                    (*s).valid = TDVALID as ::core::ffi::c_int as uint8_t;
                } else {
                    (*s).valid = VALID as ::core::ffi::c_int as uint8_t;
                }
                chunk_statusopchunk((*s).csid, (*c).chunkid, status);
            }
            if (*s).valid as ::core::ffi::c_int == BUSY as ::core::ffi::c_int
                || (*s).valid as ::core::ffi::c_int == TDBUSY as ::core::ffi::c_int
            {
                opfinished = 0 as uint8_t;
            }
            if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                || (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int
            {
                if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    validcopies = 1 as uint8_t;
                } else if (*s).ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
                {
                    mask4 = (mask4 as ::core::ffi::c_uint
                        | (1 as ::core::ffi::c_uint)
                            << ((*s).ecid as ::core::ffi::c_int & 0xf as ::core::ffi::c_int))
                        as uint32_t;
                } else if (*s).ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int <= 0x30 as ::core::ffi::c_int
                {
                    mask8 = (mask8 as ::core::ffi::c_uint
                        | (1 as ::core::ffi::c_uint)
                            << ((*s).ecid as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
                        as uint32_t;
                }
            }
            st = &raw mut (*s).next as *mut *mut slist;
        }
        if opfinished as ::core::ffi::c_int != 0
            && validcopies as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && ((*c).operation() as ::core::ffi::c_int == SET_VERSION as ::core::ffi::c_int
                || (*c).operation() as ::core::ffi::c_int == TRUNCATE as ::core::ffi::c_int)
        {
            verfixed = 0 as uint8_t;
            s = (*c).slisthead;
            while !s.is_null() {
                if (*s).version.wrapping_add(1 as uint32_t) == (*c).version() as uint32_t
                    && (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    if (*s).valid as ::core::ffi::c_int == TDWVER as ::core::ffi::c_int {
                        verfixed = 1 as uint8_t;
                        (*s).valid = TDVALID as ::core::ffi::c_int as uint8_t;
                        fix = 1 as uint8_t;
                    } else if (*s).valid as ::core::ffi::c_int == WVER as ::core::ffi::c_int {
                        verfixed = 1 as uint8_t;
                        (*s).valid = VALID as ::core::ffi::c_int as uint8_t;
                        fix = 1 as uint8_t;
                    }
                }
                s = (*s).next as *mut slist;
            }
            if verfixed != 0 {
                (*c).set_version((*c).version() - 1 as ::core::ffi::c_uint);
                (*c).set_allowreadzeros(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                changelog(
                    b"%u|SETVERSION(%lu,%u)\0".as_ptr() as *const ::core::ffi::c_char,
                    now,
                    (*c).chunkid,
                    (*c).version() as ::core::ffi::c_int,
                );
            }
        }
        if fix != 0 {
            chunk_state_fix(c);
        }
        if opfinished != 0 {
            let mut nospace: uint8_t = 0;
            nospace = 1 as uint8_t;
            s = (*c).slisthead;
            while !s.is_null() {
                status = chunk_delopchunk((*s).csid, chunkid);
                if status as ::core::ffi::c_int != MFS_ERROR_MISMATCH
                    && status as ::core::ffi::c_int != MFS_ERROR_NOSPACE
                {
                    nospace = 0 as uint8_t;
                }
                s = (*s).next as *mut slist;
            }
            if validcopies as ::core::ffi::c_int != 0
                && ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || bitcount(mask4) as ::core::ffi::c_int >= 4 as ::core::ffi::c_int
                    && ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                    && ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
                || bitcount(mask8) as ::core::ffi::c_int >= 8 as ::core::ffi::c_int
                    && ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                    && ecid as ::core::ffi::c_int <= 0x30 as ::core::ffi::c_int
            {
                if (*c).interrupted() != 0 {
                    chunk_emergency_increase_version(c);
                } else {
                    chunk_set_op(c, NONE as ::core::ffi::c_int as uint8_t);
                    (*c).set_needverincrease(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    matoclserv_chunk_status((*c).chunkid, MFS_STATUS_OK as uint8_t);
                    if (*c).lockedto < now
                        && chunk_replock_test((*c).chunkid, now) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                    {
                        matoclserv_chunk_unlocked((*c).chunkid, c as *mut ::core::ffi::c_void);
                    }
                }
            } else {
                chunk_set_op(c, NONE as ::core::ffi::c_int as uint8_t);
                if nospace != 0 {
                    matoclserv_chunk_status((*c).chunkid, MFS_ERROR_NOSPACE as uint8_t);
                } else {
                    matoclserv_chunk_status((*c).chunkid, MFS_ERROR_NOTDONE as uint8_t);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_got_chunkop_status(
    mut csid: uint16_t,
    mut chunkid: uint64_t,
    mut ecid: uint8_t,
    mut status: uint8_t,
) {
    unsafe {
        chunk_operation_status(
            chunkid,
            ecid,
            status,
            csid,
            NONE as ::core::ffi::c_int as uint8_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_got_create_status(
    mut csid: uint16_t,
    mut chunkid: uint64_t,
    mut ecid: uint8_t,
    mut status: uint8_t,
) {
    unsafe {
        chunk_operation_status(
            chunkid,
            ecid,
            status,
            csid,
            CREATE as ::core::ffi::c_int as uint8_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_got_duplicate_status(
    mut csid: uint16_t,
    mut chunkid: uint64_t,
    mut ecid: uint8_t,
    mut status: uint8_t,
) {
    unsafe {
        chunk_operation_status(
            chunkid,
            ecid,
            status,
            csid,
            DUPLICATE as ::core::ffi::c_int as uint8_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_got_setversion_status(
    mut csid: uint16_t,
    mut chunkid: uint64_t,
    mut ecid: uint8_t,
    mut status: uint8_t,
) {
    unsafe {
        chunk_operation_status(
            chunkid,
            ecid,
            status,
            csid,
            SET_VERSION as ::core::ffi::c_int as uint8_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_got_truncate_status(
    mut csid: uint16_t,
    mut chunkid: uint64_t,
    mut ecid: uint8_t,
    mut status: uint8_t,
) {
    unsafe {
        chunk_operation_status(
            chunkid,
            ecid,
            status,
            csid,
            TRUNCATE as ::core::ffi::c_int as uint8_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_got_duptrunc_status(
    mut csid: uint16_t,
    mut chunkid: uint64_t,
    mut ecid: uint8_t,
    mut status: uint8_t,
) {
    unsafe {
        chunk_operation_status(
            chunkid,
            ecid,
            status,
            csid,
            DUPTRUNC as ::core::ffi::c_int as uint8_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_got_localsplit_status(
    mut csid: uint16_t,
    mut chunkid: uint64_t,
    mut version: uint32_t,
    mut status: uint8_t,
) {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut fix: uint8_t = 0;
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut now: uint32_t = 0;
        if status as ::core::ffi::c_int == MFS_STATUS_OK {
            stats_chunkops[CHUNK_OP_SPLIT_OK as usize] =
                stats_chunkops[CHUNK_OP_SPLIT_OK as usize].wrapping_add(1);
        } else {
            stats_chunkops[CHUNK_OP_SPLIT_ERR as usize] =
                stats_chunkops[CHUNK_OP_SPLIT_ERR as usize].wrapping_add(1);
        }
        c = chunk_find(chunkid);
        if c.is_null() {
            return;
        }
        if (*c).operation() as ::core::ffi::c_int != LOCALSPLIT as ::core::ffi::c_int {
            let mut eop: uint8_t = 0;
            eop = (*c).operation() as uint8_t;
            if eop as ::core::ffi::c_int > 8 as ::core::ffi::c_int {
                eop = 8 as uint8_t;
            }
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"chunk %016lX_%08X - got unexpected status (expected: %s ; got: LOCALSPLIT) from %s\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                chunkid,
                (*c).version() as ::core::ffi::c_int,
                op_to_str(eop),
                matocsserv_getstrip((*cstab.offset(csid as isize)).ptr),
            );
            return;
        }
        if chunk_remove_disconnected_chunks(c) != 0 {
            return;
        }
        now = main_time();
        fix = 0 as uint8_t;
        s = (*c).slisthead;
        while !s.is_null() {
            if (*s).csid as ::core::ffi::c_int == csid as ::core::ffi::c_int
                && (*s).ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                && ((*s).valid as ::core::ffi::c_int == BUSY as ::core::ffi::c_int
                    || (*s).valid as ::core::ffi::c_int == TDBUSY as ::core::ffi::c_int)
            {
                if status as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                    || version != (*c).version() as uint32_t
                {
                    fix = 1 as uint8_t;
                    (*s).valid = INVALID as ::core::ffi::c_int as uint8_t;
                    (*s).version = 0 as uint32_t;
                } else if (*s).valid as ::core::ffi::c_int == TDBUSY as ::core::ffi::c_int
                    || (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int
                {
                    (*s).valid = TDVALID as ::core::ffi::c_int as uint8_t;
                } else {
                    (*s).valid = VALID as ::core::ffi::c_int as uint8_t;
                }
            }
            s = (*s).next as *mut slist;
        }
        chunk_delopchunk(csid, chunkid);
        if fix != 0 {
            chunk_state_fix(c);
        }
        chunk_set_op(c, NONE as ::core::ffi::c_int as uint8_t);
        chunk_replock_repend((*c).chunkid);
        chunk_io_ready_check(c);
        if (*c).operation() as ::core::ffi::c_int == NONE as ::core::ffi::c_int
            && (*c).ondangerlist() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            matoclserv_chunk_status((*c).chunkid, MFS_STATUS_OK as uint8_t);
        }
        if (*c).lockedto < now
            && chunk_replock_test((*c).chunkid, now) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            matoclserv_chunk_unlocked((*c).chunkid, c as *mut ::core::ffi::c_void);
        }
        if (*c).operation() as ::core::ffi::c_int == NONE as ::core::ffi::c_int {
            chunk_priority_queue_check(c, 1 as uint8_t);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_got_status_data(
    mut chunkid: uint64_t,
    mut servdesc: *const ::core::ffi::c_char,
    mut csid: uint16_t,
    mut parts: uint8_t,
    mut ecid: *mut uint8_t,
    mut version: *mut uint32_t,
    mut damaged: *mut uint8_t,
    mut blocks: *mut uint16_t,
    mut fixmode: uint8_t,
) {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut si: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut sp: *mut *mut slist = ::core::ptr::null_mut::<*mut slist>();
        let mut slisthead: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut i: uint8_t = 0;
        let mut error: uint8_t = 0;
        let mut msgbuff: [::core::ffi::c_char; 1024] = [0; 1024];
        let mut buff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut leng: int32_t = 0;
        c = chunk_find(chunkid);
        if c.is_null() {
            return;
        }
        if (*c).operation() as ::core::ffi::c_int != NONE as ::core::ffi::c_int {
            return;
        }
        slisthead = ::core::ptr::null_mut::<slist>();
        i = 0 as uint8_t;
        while (i as ::core::ffi::c_int) < parts as ::core::ffi::c_int {
            s = slist_malloc();
            (*s).csid = csid;
            (*s).ecid = *ecid.offset(i as isize);
            (*s).version = *version.offset(i as isize) & 0x7fffffff as uint32_t;
            if *damaged.offset(i as isize) != 0 {
                (*s).valid = INVALID as ::core::ffi::c_int as uint8_t;
            } else if *version.offset(i as isize) & 0x80000000 as uint32_t != 0 {
                if (*s).version != (*c).version() as uint32_t {
                    (*s).valid = TDWVER as ::core::ffi::c_int as uint8_t;
                } else {
                    (*s).valid = TDVALID as ::core::ffi::c_int as uint8_t;
                }
            } else if (*s).version != (*c).version() as uint32_t {
                (*s).valid = WVER as ::core::ffi::c_int as uint8_t;
            } else {
                (*s).valid = VALID as ::core::ffi::c_int as uint8_t;
            }
            sp = &raw mut slisthead;
            loop {
                si = *sp;
                if si.is_null() {
                    break;
                }
                if ((*si).ecid as ::core::ffi::c_int) >= (*s).ecid as ::core::ffi::c_int {
                    break;
                }
                sp = &raw mut (*si).next as *mut *mut slist;
            }
            (*s).next = *sp as *mut _slist;
            *sp = s;
            i = i.wrapping_add(1);
        }
        si = slisthead;
        error = 0 as uint8_t;
        s = (*c).slisthead;
        while !s.is_null() && error as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if (*s).csid as ::core::ffi::c_int == csid as ::core::ffi::c_int {
                if (*s).valid as ::core::ffi::c_int == DEL as ::core::ffi::c_int {
                    if !si.is_null()
                        && (*s).ecid as ::core::ffi::c_int == (*si).ecid as ::core::ffi::c_int
                    {
                        si = (*si).next as *mut slist;
                    }
                } else if si.is_null() {
                    error = 1 as uint8_t;
                } else if (*s).ecid as ::core::ffi::c_int != (*si).ecid as ::core::ffi::c_int {
                    error = 1 as uint8_t;
                } else if (*s).version != (*si).version {
                    error = 2 as uint8_t;
                } else if (*s).valid as ::core::ffi::c_int == BUSY as ::core::ffi::c_int
                    || (*s).valid as ::core::ffi::c_int == TDBUSY as ::core::ffi::c_int
                {
                    error = 3 as uint8_t;
                } else if (*s).valid as ::core::ffi::c_int != (*si).valid as ::core::ffi::c_int {
                    error = 4 as uint8_t;
                } else {
                    si = (*si).next as *mut slist;
                }
            }
            s = (*s).next as *mut slist;
        }
        if !si.is_null() {
            error = 1 as uint8_t;
        }
        if error != 0 {
            leng = 0 as ::core::ffi::c_int as int32_t;
            buff = &raw mut msgbuff as *mut ::core::ffi::c_char;
            if leng < 1024 as int32_t {
                leng = (leng as ::core::ffi::c_int
                    + snprintf(
                        buff.offset(leng as isize),
                        (1024 as int32_t - leng) as size_t,
                        b"chunk %016lX_%08X: copies/parts on server %s mismatch ; master data:\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        chunkid,
                        (*c).version() as ::core::ffi::c_int,
                        servdesc,
                    )) as int32_t;
            }
            i = 0 as uint8_t;
            s = (*c).slisthead;
            while !s.is_null() {
                if (*s).csid as ::core::ffi::c_int == csid as ::core::ffi::c_int {
                    if leng < 1024 as int32_t {
                        leng = (leng as ::core::ffi::c_int
                            + snprintf(
                                buff.offset(leng as isize),
                                (1024 as int32_t - leng) as size_t,
                                b"%c%s:%08X:%s\0".as_ptr() as *const ::core::ffi::c_char,
                                if i as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                                    ' ' as ::core::ffi::c_int
                                } else {
                                    ',' as ::core::ffi::c_int
                                },
                                chunk_ecid_to_str((*s).ecid),
                                (*s).version,
                                valid_to_str((*s).valid),
                            )) as int32_t;
                    }
                    i = 1 as uint8_t;
                }
                s = (*s).next as *mut slist;
            }
            if leng < 1024 as int32_t {
                leng = (leng as ::core::ffi::c_int
                    + snprintf(
                        buff.offset(leng as isize),
                        (1024 as int32_t - leng) as size_t,
                        b" ; chunkserver data:\0".as_ptr() as *const ::core::ffi::c_char,
                    )) as int32_t;
            }
            i = 0 as uint8_t;
            s = slisthead;
            while !s.is_null() {
                if leng < 1024 as int32_t {
                    leng = (leng as ::core::ffi::c_int
                        + snprintf(
                            buff.offset(leng as isize),
                            (1024 as int32_t - leng) as size_t,
                            b"%c%s:%08X:%s\0".as_ptr() as *const ::core::ffi::c_char,
                            if i as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                                ' ' as ::core::ffi::c_int
                            } else {
                                ',' as ::core::ffi::c_int
                            },
                            chunk_ecid_to_str((*s).ecid),
                            (*s).version,
                            valid_to_str((*s).valid),
                        )) as int32_t;
                }
                i = 1 as uint8_t;
                s = (*s).next as *mut slist;
            }
            if leng < 1024 as int32_t {
                *buff.offset(leng as isize) = '\0' as ::core::ffi::c_char;
            } else {
                *buff.offset(1023 as isize) = '\0' as ::core::ffi::c_char;
            }
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut msgbuff as *mut ::core::ffi::c_char,
            );
            if fixmode != 0 {
                sp = &raw mut (*c).slisthead;
                loop {
                    si = *sp;
                    if si.is_null() {
                        break;
                    }
                    if ((*si).csid as ::core::ffi::c_int) < csid as ::core::ffi::c_int {
                        sp = &raw mut (*si).next as *mut *mut slist;
                    } else {
                        if (*si).csid as ::core::ffi::c_int != csid as ::core::ffi::c_int {
                            break;
                        }
                        *sp = (*si).next as *mut slist;
                        slist_free(si);
                    }
                }
                if !slisthead.is_null() {
                    *sp = slisthead;
                    sp = &raw mut slisthead;
                    loop {
                        s = *sp;
                        if s.is_null() {
                            break;
                        }
                        sp = &raw mut (*s).next as *mut *mut slist;
                    }
                    *sp = si;
                    slisthead = ::core::ptr::null_mut::<slist>();
                }
            }
        }
        si = slisthead;
        while !si.is_null() {
            s = (*si).next as *mut slist;
            slist_free(si);
            si = s;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_no_more_pending_jobs() -> uint8_t {
    unsafe {
        return (if opsinprogress == 0 as uint32_t {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_store_info(mut buff: *mut uint8_t) -> uint32_t {
    unsafe {
        if !buff.is_null() {
            put32bit(&raw mut buff, chunksinfo_loopstart);
            put32bit(&raw mut buff, chunksinfo_loopend);
            put32bit(&raw mut buff, chunksinfo.fixed);
            put32bit(&raw mut buff, chunksinfo.forcekeep);
            put32bit(&raw mut buff, chunksinfo.delete_invalid);
            put32bit(&raw mut buff, chunksinfo.delete_no_longer_needed);
            put32bit(&raw mut buff, chunksinfo.delete_wrong_version);
            put32bit(&raw mut buff, chunksinfo.delete_duplicated_ecpart);
            put32bit(&raw mut buff, chunksinfo.delete_excess_ecpart);
            put32bit(&raw mut buff, chunksinfo.delete_excess_copy);
            put32bit(&raw mut buff, chunksinfo.delete_diskclean_ecpart);
            put32bit(&raw mut buff, chunksinfo.delete_diskclean_copy);
            put32bit(&raw mut buff, chunksinfo.replicate_dupserver_ecpart);
            put32bit(&raw mut buff, chunksinfo.replicate_needed_ecpart);
            put32bit(&raw mut buff, chunksinfo.replicate_needed_copy);
            put32bit(&raw mut buff, chunksinfo.replicate_wronglabels_ecpart);
            put32bit(&raw mut buff, chunksinfo.replicate_wronglabels_copy);
            put32bit(&raw mut buff, chunksinfo.split_copy_into_ecparts);
            put32bit(&raw mut buff, chunksinfo.join_ecparts_into_copy);
            put32bit(&raw mut buff, chunksinfo.recover_ecpart);
            put32bit(&raw mut buff, chunksinfo.calculate_ecchksum);
            put32bit(&raw mut buff, chunksinfo.locked_unused);
            put32bit(&raw mut buff, chunksinfo.locked_used);
            put32bit(&raw mut buff, chunksinfo.replicate_rebalance);
        }
        return 96 as uint32_t;
    }
}
#[inline]
unsafe extern "C" fn chunk_mindist(
    mut csid: uint16_t,
    mut ip: *mut uint32_t,
    mut ipcnt: uint8_t,
) -> uint8_t {
    unsafe {
        let mut mindist: uint8_t = 0;
        let mut dist: uint8_t = 0;
        let mut k: uint8_t = 0;
        let mut sip: uint32_t = 0;
        mindist = TOPOLOGY_DIST_MAX as uint8_t;
        sip = matocsserv_server_get_ip((*cstab.offset(csid as isize)).ptr);
        k = 0 as uint8_t;
        while (k as ::core::ffi::c_int) < ipcnt as ::core::ffi::c_int
            && mindist as ::core::ffi::c_int > 0 as ::core::ffi::c_int
        {
            dist = topology_distance(sip, *ip.offset(k as isize));
            if (dist as ::core::ffi::c_int) < mindist as ::core::ffi::c_int {
                mindist = dist;
            }
            k = k.wrapping_add(1);
        }
        return mindist;
    }
}
#[inline]
unsafe extern "C" fn chunk_rack_sort(
    mut servers: *mut uint16_t,
    mut servcount: uint16_t,
    mut ip: *mut uint32_t,
    mut ipcnt: uint8_t,
) {
    unsafe {
        let mut i: int16_t = 0;
        let mut j: int16_t = 0;
        let mut csid: uint16_t = 0;
        let mut mindist: uint8_t = 0;
        if servcount as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || ipcnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            return;
        }
        i = 0 as int16_t;
        j = (servcount as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int16_t;
        while i as ::core::ffi::c_int <= j as ::core::ffi::c_int {
            while i as ::core::ffi::c_int <= j as ::core::ffi::c_int {
                mindist = chunk_mindist(*servers.offset(i as isize), ip, ipcnt);
                if mindist as ::core::ffi::c_int != TOPOLOGY_DIST_SAME_RACKID {
                    break;
                }
                i += 1;
            }
            while i as ::core::ffi::c_int <= j as ::core::ffi::c_int {
                mindist = chunk_mindist(*servers.offset(j as isize), ip, ipcnt);
                if mindist as ::core::ffi::c_int == TOPOLOGY_DIST_SAME_RACKID {
                    break;
                }
                j -= 1;
            }
            if (i as ::core::ffi::c_int) < j as ::core::ffi::c_int {
                csid = *servers.offset(i as isize);
                *servers.offset(i as isize) = *servers.offset(j as isize);
                *servers.offset(j as isize) = csid;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn chunk_get_undergoal_replicate_srccsid(
    mut c: *mut chunk,
    mut dstcsid: uint16_t,
    mut now: uint32_t,
    mut repl_limit_read: ::core::ffi::c_double,
    mut rgvc: uint32_t,
    mut rgtdc: uint32_t,
) -> uint16_t {
    unsafe {
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut r: uint32_t = 0 as uint32_t;
        let mut srccsid: uint16_t = MAXCSCOUNT as uint16_t;
        if ReplicationsRespectTopology != 0 {
            let mut min_dist: uint32_t = 0xffffffff as uint32_t;
            let mut tdcflag: uint8_t = 0 as uint8_t;
            let mut dist: uint32_t = 0;
            let mut ip: uint32_t = 0;
            let mut cuip: uint32_t = 0;
            let mut cnt: uint32_t = 0;
            let mut frnd: uint32_t = 0;
            let mut rnd: uint32_t = 0;
            frnd = rndu32_ranged(3628800 as uint32_t);
            rnd = frnd;
            cuip = matocsserv_server_get_ip((*cstab.offset(dstcsid as isize)).ptr);
            cnt = 0 as uint32_t;
            s = (*c).slisthead;
            while !s.is_null() {
                if matocsserv_replication_read_counter((*cstab.offset((*s).csid as isize)).ptr, now)
                    < repl_limit_read
                    && ((*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                        || tdcflag as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            && (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int)
                {
                    ip = matocsserv_server_get_ip((*cstab.offset((*s).csid as isize)).ptr);
                    dist = topology_distance(ip, cuip) as uint32_t;
                    if tdcflag as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        && (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                        || dist < min_dist
                    {
                        if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int {
                            tdcflag = 1 as uint8_t;
                        }
                        min_dist = dist;
                        srccsid = (*s).csid;
                        cnt = 1 as uint32_t;
                        rnd = frnd;
                    } else if dist == min_dist {
                        cnt = cnt.wrapping_add(1);
                        if cnt <= 10 as uint32_t {
                            if rnd.wrapping_rem(cnt) == 0 as uint32_t {
                                srccsid = (*s).csid;
                            }
                            rnd = rnd.wrapping_div(cnt);
                        } else if rndu32_ranged(cnt) == 0 as uint32_t {
                            srccsid = (*s).csid;
                        }
                    }
                }
                s = (*s).next as *mut slist;
            }
        } else if rgvc > 0 as uint32_t {
            r = (1 as uint32_t).wrapping_add(rndu32_ranged(rgvc));
            s = (*c).slisthead;
            while !s.is_null() && r > 0 as uint32_t {
                if matocsserv_replication_read_counter((*cstab.offset((*s).csid as isize)).ptr, now)
                    < repl_limit_read
                    && (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    r = r.wrapping_sub(1);
                    srccsid = (*s).csid;
                }
                s = (*s).next as *mut slist;
            }
        } else {
            r = (1 as uint32_t).wrapping_add(rndu32_ranged(rgtdc));
            s = (*c).slisthead;
            while !s.is_null() && r > 0 as uint32_t {
                if matocsserv_replication_read_counter((*cstab.offset((*s).csid as isize)).ptr, now)
                    < repl_limit_read
                    && (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    r = r.wrapping_sub(1);
                    srccsid = (*s).csid;
                }
                s = (*s).next as *mut slist;
            }
        }
        return srccsid;
    }
}
#[inline]
unsafe extern "C" fn chunk_replicate(
    mut repication_mode: uint8_t,
    mut now: uint32_t,
    mut c: *mut chunk,
    mut ec_data_parts: uint8_t,
    mut ecid: uint8_t,
    mut src: uint16_t,
    mut dst: uint16_t,
    mut user: *mut ::core::ffi::c_void,
    mut survivorscsid: *mut uint16_t,
    mut survivorsecid: *mut uint8_t,
    mut reason: uint8_t,
) -> uint8_t {
    unsafe {
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut res: ::core::ffi::c_int = 0;
        let mut i: uint8_t = 0;
        let mut part: uint8_t = 0;
        let mut survivorsptrs: [*mut ::core::ffi::c_void; 8] =
            [::core::ptr::null_mut::<::core::ffi::c_void>(); 8];
        if chunk_check_ecid(ecid) < 0 as ::core::ffi::c_int {
            return 1 as uint8_t;
        }
        if ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
            && ecid as ::core::ffi::c_int <= 0x30 as ::core::ffi::c_int
        {
            part = (ecid as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int) as uint8_t;
        } else if ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
            && ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
        {
            part = (ecid as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as uint8_t;
        } else {
            part = 0 as uint8_t;
        }
        s = (*c).slisthead;
        while !s.is_null() {
            if (*s).csid as ::core::ffi::c_int == dst as ::core::ffi::c_int
                && (*s).ecid as ::core::ffi::c_int == ecid as ::core::ffi::c_int
            {
                if ecid as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"can't replicate chunk %016lX_%08X part: %s : found duplicate\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*c).chunkid,
                        (*c).version() as ::core::ffi::c_int,
                        chunk_ecid_to_str(ecid),
                    );
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"can't replicate chunk %016lX_%08X : found duplicate\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*c).chunkid,
                        (*c).version() as ::core::ffi::c_int,
                    );
                }
                return 1 as uint8_t;
            }
            s = (*s).next as *mut slist;
        }
        chunk_delay_protect((*c).chunkid);
        stats_chunkops[CHUNK_OP_REPLICATE_TRY as usize] =
            stats_chunkops[CHUNK_OP_REPLICATE_TRY as usize].wrapping_add(1);
        match repication_mode as ::core::ffi::c_int {
            0 => {
                res = matocsserv_send_replicatechunk(
                    (*cstab.offset(dst as isize)).ptr,
                    (*c).chunkid,
                    ecid,
                    (*c).version() as uint32_t,
                    (*cstab.offset(src as isize)).ptr,
                    reason,
                );
            }
            1 => {
                res = matocsserv_send_replicatechunk_split(
                    (*cstab.offset(dst as isize)).ptr,
                    (*c).chunkid,
                    ecid,
                    (*c).version() as uint32_t,
                    (*cstab.offset(src as isize)).ptr,
                    0 as uint8_t,
                    part,
                    ec_data_parts,
                    reason,
                );
            }
            2 => {
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int) < ec_data_parts as ::core::ffi::c_int {
                    survivorsptrs[i as usize] =
                        (*cstab.offset(*survivorscsid.offset(i as isize) as isize)).ptr;
                    i = i.wrapping_add(1);
                }
                res = matocsserv_send_replicatechunk_recover(
                    (*cstab.offset(dst as isize)).ptr,
                    (*c).chunkid,
                    ecid,
                    (*c).version() as uint32_t,
                    ec_data_parts,
                    &raw mut survivorsptrs as *mut *mut ::core::ffi::c_void,
                    survivorsecid,
                    reason,
                );
            }
            3 => {
                i = 0 as uint8_t;
                while (i as ::core::ffi::c_int) < ec_data_parts as ::core::ffi::c_int {
                    survivorsptrs[i as usize] =
                        (*cstab.offset(*survivorscsid.offset(i as isize) as isize)).ptr;
                    i = i.wrapping_add(1);
                }
                res = matocsserv_send_replicatechunk_join(
                    (*cstab.offset(dst as isize)).ptr,
                    (*c).chunkid,
                    ecid,
                    (*c).version() as uint32_t,
                    ec_data_parts,
                    &raw mut survivorsptrs as *mut *mut ::core::ffi::c_void,
                    survivorsecid,
                    reason,
                );
            }
            _ => {
                res = -1 as ::core::ffi::c_int;
            }
        }
        if res < 0 as ::core::ffi::c_int {
            let mut rmodestr: *const ::core::ffi::c_char =
                if repication_mode as ::core::ffi::c_int == SIMPLE as ::core::ffi::c_int {
                    b"SIMPLE\0".as_ptr() as *const ::core::ffi::c_char
                } else if repication_mode as ::core::ffi::c_int == SPLIT as ::core::ffi::c_int {
                    b"SPLIT\0".as_ptr() as *const ::core::ffi::c_char
                } else if repication_mode as ::core::ffi::c_int == RECOVER as ::core::ffi::c_int {
                    b"RECOVER\0".as_ptr() as *const ::core::ffi::c_char
                } else if repication_mode as ::core::ffi::c_int == JOIN as ::core::ffi::c_int {
                    b"JOIN\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"???\0".as_ptr() as *const ::core::ffi::c_char
                };
            if ecid as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"chunk %016lX_%08X part: %s : error sending replicate (%s) command\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*c).chunkid,
                    (*c).version() as ::core::ffi::c_int,
                    chunk_ecid_to_str(ecid),
                    rmodestr,
                );
            } else {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"chunk %016lX_%08X : error sending replicate (%s) command\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*c).chunkid,
                    (*c).version() as ::core::ffi::c_int,
                    rmodestr,
                );
            }
            return 1 as uint8_t;
        }
        chunk_addopchunk(dst, (*c).chunkid);
        chunk_set_op(c, REPLICATE as ::core::ffi::c_int as uint8_t);
        chunk_replock_repstart((*c).chunkid, now);
        s = slist_malloc();
        (*s).csid = dst;
        (*s).ecid = ecid;
        (*s).valid = BUSY as ::core::ffi::c_int as uint8_t;
        (*s).version = (*c).version() as uint32_t;
        chunk_new_copy(c, s);
        return 0 as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn chunk_undergoal_replicate(
    mut c: *mut chunk,
    mut dstcsid: uint16_t,
    mut now: uint32_t,
    mut repl_limit_read: ::core::ffi::c_double,
    mut extrajob: uint8_t,
    mut chunk_priority: uint16_t,
    mut inforec: *mut loop_info,
    mut rgvc: uint32_t,
    mut rgtdc: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut srccsid: uint16_t = 0;
        srccsid =
            chunk_get_undergoal_replicate_srccsid(c, dstcsid, now, repl_limit_read, rgvc, rgtdc);
        if srccsid as ::core::ffi::c_int == MAXCSCOUNT {
            return -1 as ::core::ffi::c_int;
        }
        if chunk_replicate(
            SIMPLE as ::core::ffi::c_int as uint8_t,
            now,
            c,
            0 as uint8_t,
            0 as uint8_t,
            srccsid,
            dstcsid,
            NULL,
            ::core::ptr::null_mut::<uint16_t>(),
            ::core::ptr::null_mut::<uint8_t>(),
            (if chunk_priority as ::core::ffi::c_int == CHUNK_PRIORITY_IOREADY {
                REPL_COPY_IO as ::core::ffi::c_int
            } else {
                (if chunk_priority as ::core::ffi::c_int <= CHUNK_PRIORITY_ONEREGCOPY_PLUSMFR {
                    REPL_COPY_ENDANGERED as ::core::ffi::c_int
                } else {
                    (if chunk_priority as ::core::ffi::c_int <= CHUNK_PRIORITY_UNDERGOAL {
                        REPL_COPY_UNDERGOAL as ::core::ffi::c_int
                    } else {
                        REPL_COPY_WRONGLABEL as ::core::ffi::c_int
                    })
                })
            }) as uint8_t,
        ) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        if extrajob as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if chunk_priority as ::core::ffi::c_int <= CHUNK_PRIORITY_UNDERGOAL {
                (*inforec).replicate_needed_copy = (*inforec).replicate_needed_copy.wrapping_add(1);
            } else if chunk_priority as ::core::ffi::c_int == CHUNK_PRIORITY_WRONGLABELS {
                (*inforec).replicate_wronglabels_copy =
                    (*inforec).replicate_wronglabels_copy.wrapping_add(1);
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn chunk_check_label_for_ec_part(
    mut servid: uint16_t,
    mut sm: *mut storagemode,
    mut chksumflag: uint8_t,
    mut ec_both_labels_limit: *mut int32_t,
) -> uint8_t {
    unsafe {
        if (*sm).labelscnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return 1 as uint8_t;
        }
        if (*sm).ec_data_chksum_parts != 0 {
            if (*sm).labelscnt as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                return (if matocsserv_server_matches_labelexpr(
                    (*cstab.offset(servid as isize)).ptr,
                    &raw mut *(&raw mut (*sm).labelexpr as *mut [uint8_t; 128]).offset(0 as isize)
                        as *mut uint8_t as *const uint8_t,
                ) as ::core::ffi::c_int
                    != 0
                {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as uint8_t;
            } else if (*sm).labelscnt as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                let mut datamatch: uint8_t = 0;
                let mut chksummatch: uint8_t = 0;
                datamatch = matocsserv_server_matches_labelexpr(
                    (*cstab.offset(servid as isize)).ptr,
                    &raw mut *(&raw mut (*sm).labelexpr as *mut [uint8_t; 128]).offset(0 as isize)
                        as *mut uint8_t as *const uint8_t,
                );
                chksummatch = matocsserv_server_matches_labelexpr(
                    (*cstab.offset(servid as isize)).ptr,
                    &raw mut *(&raw mut (*sm).labelexpr as *mut [uint8_t; 128]).offset(1 as isize)
                        as *mut uint8_t as *const uint8_t,
                );
                if datamatch as ::core::ffi::c_int != 0
                    && chksummatch as ::core::ffi::c_int != 0
                    && *ec_both_labels_limit > 0 as int32_t
                {
                    *ec_both_labels_limit -= 1;
                    return 1 as uint8_t;
                } else if datamatch as ::core::ffi::c_int != 0
                    && chksumflag as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    return 1 as uint8_t;
                } else if chksummatch as ::core::ffi::c_int != 0
                    && chksumflag as ::core::ffi::c_int != 0
                {
                    return 1 as uint8_t;
                }
            } else {
                return 1 as uint8_t;
            }
        } else {
            let mut i: uint8_t = 0;
            i = 0 as uint8_t;
            while (i as ::core::ffi::c_int) < (*sm).labelscnt as ::core::ffi::c_int {
                if matocsserv_server_matches_labelexpr(
                    (*cstab.offset(servid as isize)).ptr,
                    &raw mut *(&raw mut (*sm).labelexpr as *mut [uint8_t; 128]).offset(i as isize)
                        as *mut uint8_t as *const uint8_t,
                ) != 0
                {
                    return 1 as uint8_t;
                }
                i = i.wrapping_add(1);
            }
        }
        return 0 as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn chunk_get_available_servers_for_parts(
    mut servers: *mut uint16_t,
    mut c: *mut chunk,
    mut sm: *mut storagemode,
    mut strict_mode: uint8_t,
    mut data_recovery_mode: uint8_t,
    mut ec_both_labels_limit: *mut int32_t,
    mut minecid: uint8_t,
    mut maxecid: uint8_t,
    mut chksumflag: uint8_t,
    mut srcservcount: uint16_t,
    mut srcsids: *mut uint16_t,
) -> uint16_t {
    unsafe {
        let mut servcnt: uint16_t = 0;
        let mut extraservcnt: uint16_t = 0;
        let mut i: uint16_t = 0;
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        servcnt = 0 as uint16_t;
        extraservcnt = 0 as uint16_t;
        i = 0 as uint16_t;
        while (i as ::core::ffi::c_int) < srcservcount as ::core::ffi::c_int {
            s = (*c).slisthead;
            while !s.is_null()
                && ((*s).csid as ::core::ffi::c_int
                    != *srcsids.offset(i as isize) as ::core::ffi::c_int
                    || ((*s).ecid as ::core::ffi::c_int) < minecid as ::core::ffi::c_int
                    || (*s).ecid as ::core::ffi::c_int > maxecid as ::core::ffi::c_int)
            {
                s = (*s).next as *mut slist;
            }
            if s.is_null() {
                if chunk_check_label_for_ec_part(
                    *srcsids.offset(i as isize),
                    sm,
                    chksumflag,
                    ec_both_labels_limit,
                ) != 0
                {
                    let c2rust_fresh25 = servcnt;
                    servcnt = servcnt.wrapping_add(1);
                    *servers.offset(c2rust_fresh25 as isize) = *srcsids.offset(i as isize);
                } else {
                    extraservcnt = extraservcnt.wrapping_add(1);
                    *servers.offset((MAXCSCOUNT - extraservcnt as ::core::ffi::c_int) as isize) =
                        *srcsids.offset(i as isize);
                }
            }
            i = i.wrapping_add(1);
        }
        if strict_mode as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            i = 1 as uint16_t;
            while i as ::core::ffi::c_int <= extraservcnt as ::core::ffi::c_int {
                let c2rust_fresh26 = servcnt;
                servcnt = servcnt.wrapping_add(1);
                *servers.offset(c2rust_fresh26 as isize) =
                    *servers.offset((MAXCSCOUNT - i as ::core::ffi::c_int) as isize);
                i = i.wrapping_add(1);
            }
        }
        if servcnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && data_recovery_mode as ::core::ffi::c_int != 0
        {
            extraservcnt = 0 as uint16_t;
            i = 0 as uint16_t;
            while (i as ::core::ffi::c_int) < srcservcount as ::core::ffi::c_int {
                if chunk_check_label_for_ec_part(
                    *srcsids.offset(i as isize),
                    sm,
                    chksumflag,
                    ec_both_labels_limit,
                ) != 0
                {
                    let c2rust_fresh27 = servcnt;
                    servcnt = servcnt.wrapping_add(1);
                    *servers.offset(c2rust_fresh27 as isize) = *srcsids.offset(i as isize);
                } else {
                    extraservcnt = extraservcnt.wrapping_add(1);
                    *servers.offset((MAXCSCOUNT - extraservcnt as ::core::ffi::c_int) as isize) =
                        *srcsids.offset(i as isize);
                }
                i = i.wrapping_add(1);
            }
            if strict_mode as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && sclass_get_labels_mode((*c).sclassid as uint16_t, sm) as ::core::ffi::c_int
                    == LABELS_MODE_STD
            {
                i = 1 as uint16_t;
                while i as ::core::ffi::c_int <= extraservcnt as ::core::ffi::c_int {
                    let c2rust_fresh28 = servcnt;
                    servcnt = servcnt.wrapping_add(1);
                    *servers.offset(c2rust_fresh28 as isize) =
                        *servers.offset((MAXCSCOUNT - i as ::core::ffi::c_int) as isize);
                    i = i.wrapping_add(1);
                }
            }
        }
        return servcnt;
    }
}
#[inline]
unsafe extern "C" fn chunk_fix_wrong_version_ec(
    mut c: *mut chunk,
    mut gequiv: uint8_t,
    mut ecidmask: uint8_t,
    mut minecid: uint8_t,
    mut maxecid: uint8_t,
) -> uint8_t {
    unsafe {
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut bestversionmask: uint32_t = 0;
        let mut bestversion: uint32_t = 0;
        bestversion = 0 as uint32_t;
        s = (*c).slisthead;
        while !s.is_null() {
            if (*s).ecid as ::core::ffi::c_int >= minecid as ::core::ffi::c_int
                && (*s).ecid as ::core::ffi::c_int <= maxecid as ::core::ffi::c_int
                && ((*s).valid as ::core::ffi::c_int == WVER as ::core::ffi::c_int
                    || (*s).valid as ::core::ffi::c_int == TDWVER as ::core::ffi::c_int)
            {
                if (*s).version >= bestversion {
                    bestversion = (*s).version;
                }
            }
            s = (*s).next as *mut slist;
        }
        if bestversion > 0 as uint32_t
            && (bestversion.wrapping_add(1 as uint32_t) == (*c).version() as uint32_t
                || ((*c).version() as uint32_t).wrapping_add(1 as uint32_t) == bestversion)
        {
            bestversionmask = 0 as uint32_t;
            s = (*c).slisthead;
            while !s.is_null() {
                if (*s).ecid as ::core::ffi::c_int >= minecid as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int <= maxecid as ::core::ffi::c_int
                    && ((*s).valid as ::core::ffi::c_int == WVER as ::core::ffi::c_int
                        || (*s).valid as ::core::ffi::c_int == TDWVER as ::core::ffi::c_int)
                    && (*s).version == bestversion
                {
                    bestversionmask = (bestversionmask as ::core::ffi::c_uint
                        | (1 as ::core::ffi::c_uint)
                            << ((*s).ecid as ::core::ffi::c_int & ecidmask as ::core::ffi::c_int))
                        as uint32_t;
                }
                s = (*s).next as *mut slist;
            }
            if bitcount(bestversionmask) as ::core::ffi::c_int >= gequiv as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"chunk %016lX has only ec parts with wrong version - fixing it\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*c).chunkid,
                );
                (*c).set_version(bestversion as ::core::ffi::c_uint as ::core::ffi::c_uint);
                s = (*c).slisthead;
                while !s.is_null() {
                    if (*s).ecid as ::core::ffi::c_int >= minecid as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= maxecid as ::core::ffi::c_int
                        && (*s).version == bestversion
                        && (*cstab.offset((*s).csid as isize)).valid as ::core::ffi::c_int != 0
                    {
                        if (*s).valid as ::core::ffi::c_int == WVER as ::core::ffi::c_int {
                            (*s).valid = VALID as ::core::ffi::c_int as uint8_t;
                        } else if (*s).valid as ::core::ffi::c_int == TDWVER as ::core::ffi::c_int {
                            (*s).valid = TDVALID as ::core::ffi::c_int as uint8_t;
                        }
                    }
                    if (*s).valid as ::core::ffi::c_int == WVER as ::core::ffi::c_int
                        || (*s).valid as ::core::ffi::c_int == TDWVER as ::core::ffi::c_int
                    {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_INFO,
                            b"chunk %016lX_%08X - wrong versioned EC part on (%s - ver:%08X)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            (*c).chunkid,
                            (*c).version() as ::core::ffi::c_int,
                            matocsserv_getstrip((*cstab.offset((*s).csid as isize)).ptr),
                            (*s).version,
                        );
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_INFO,
                            b"chunk %016lX_%08X - valid EC part on (%s - ver:%08X)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*c).chunkid,
                            (*c).version() as ::core::ffi::c_int,
                            matocsserv_getstrip((*cstab.offset((*s).csid as isize)).ptr),
                            (*s).version,
                        );
                    }
                    s = (*s).next as *mut slist;
                }
                (*c).set_allowreadzeros(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                changelog(
                    b"%u|SETVERSION(%lu,%u)\0".as_ptr() as *const ::core::ffi::c_char,
                    main_time(),
                    (*c).chunkid,
                    (*c).version() as ::core::ffi::c_int,
                );
                return 1 as uint8_t;
            }
        }
        return 0 as uint8_t;
    }
}
#[inline]
unsafe extern "C" fn chunk_find_ec_survivors(
    mut c: *mut chunk,
    mut ec_data_parts: uint8_t,
    mut ecidmask: uint8_t,
    mut minecid: uint8_t,
    mut maxecid: uint8_t,
    mut now: uint32_t,
    mut repl_limit_read: ::core::ffi::c_double,
    mut eccsid: *mut uint16_t,
    mut survivorsecid: *mut uint8_t,
    mut survivorscsid: *mut uint16_t,
) -> uint32_t {
    unsafe {
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut survmap: [uint8_t; 32] = [0; 32];
        let mut readysurvmask: uint32_t = 0 as uint32_t;
        let mut i: uint32_t = 0;
        let mut j: uint32_t = 0;
        let mut mask: uint32_t = 0;
        let mut survivorsmask: uint32_t = 0 as uint32_t;
        s = (*c).slisthead;
        while !s.is_null() {
            if ((*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                || (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int)
                && (*s).ecid as ::core::ffi::c_int >= minecid as ::core::ffi::c_int
                && (*s).ecid as ::core::ffi::c_int <= maxecid as ::core::ffi::c_int
                && matocsserv_replication_read_counter((*cstab.offset((*s).csid as isize)).ptr, now)
                    < repl_limit_read
            {
                readysurvmask = (readysurvmask as ::core::ffi::c_uint
                    | (1 as ::core::ffi::c_uint)
                        << ((*s).ecid as ::core::ffi::c_int & ecidmask as ::core::ffi::c_int))
                    as uint32_t;
            }
            s = (*s).next as *mut slist;
        }
        if bitcount(readysurvmask) as ::core::ffi::c_int >= ec_data_parts as ::core::ffi::c_int {
            i = 0 as uint32_t;
            mask = 1 as uint32_t;
            j = 0 as uint32_t;
            while i < 32 as uint32_t {
                if readysurvmask & mask != 0 && j < ec_data_parts as uint32_t {
                    survmap[i as usize] = j as uint8_t;
                    j = j.wrapping_add(1);
                } else {
                    survmap[i as usize] = 0xff as uint8_t;
                }
                i = i.wrapping_add(1);
                mask <<= 1 as ::core::ffi::c_int;
            }
            i = 0 as uint32_t;
            while i < 32 as uint32_t {
                *eccsid.offset(i as isize) = 0 as uint16_t;
                i = i.wrapping_add(1);
            }
            s = (*c).slisthead;
            while !s.is_null() {
                if ((*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                    || (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int)
                    && (*s).ecid as ::core::ffi::c_int >= minecid as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int <= maxecid as ::core::ffi::c_int
                    && matocsserv_replication_read_counter(
                        (*cstab.offset((*s).csid as isize)).ptr,
                        now,
                    ) < repl_limit_read
                {
                    let mut ecindx: uint8_t = ((*s).ecid as ::core::ffi::c_int
                        & ecidmask as ::core::ffi::c_int)
                        as uint8_t;
                    if (survmap[ecindx as usize] as ::core::ffi::c_int)
                        < ec_data_parts as ::core::ffi::c_int
                    {
                        *survivorscsid.offset(survmap[ecindx as usize] as isize) = (*s).csid;
                        *survivorsecid.offset(survmap[ecindx as usize] as isize) = (*s).ecid;
                        survivorsmask = (survivorsmask as ::core::ffi::c_uint
                            | (1 as ::core::ffi::c_uint) << ecindx as ::core::ffi::c_int)
                            as uint32_t;
                    }
                    *eccsid.offset(ecindx as isize) = (*s).csid;
                }
                s = (*s).next as *mut slist;
            }
        }
        return survivorsmask;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_do_jobs(
    mut c: *mut chunk,
    mut mode: uint8_t,
    mut now: uint32_t,
    mut extrajob: uint8_t,
) {
    unsafe {
        let mut s: *mut slist = ::core::ptr::null_mut::<slist>();
        let mut sf: *mut slist = ::core::ptr::null_mut::<slist>();
        static mut dcsids: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
        static mut dservcount: uint16_t = 0;
        static mut rcsids: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
        let mut scount: uint16_t = 0;
        let mut fullservers: uint16_t = 0;
        let mut replservers: uint16_t = 0;
        let mut rservcount: uint16_t = 0;
        let mut rservallflag: uint8_t = 0;
        let mut srccsid: uint16_t = 0;
        let mut dstcsid: uint16_t = 0;
        let mut preferedsrccsid: uint16_t = 0;
        let mut survivorsecid4: [uint8_t; 4] = [0; 4];
        let mut survivorscsid4: [uint16_t; 4] = [0; 4];
        let mut eccsid4: [uint16_t; 32] = [0; 32];
        let mut survivorsmask4: uint32_t = 0;
        let mut survivorsecid8: [uint8_t; 8] = [0; 8];
        let mut survivorscsid8: [uint16_t; 8] = [0; 8];
        let mut eccsid8: [uint16_t; 32] = [0; 32];
        let mut survivorsmask8: uint32_t = 0;
        let mut chunk_priority: uint8_t = 0;
        let mut enqueue: uint8_t = 0;
        let mut repl_limit_class: uint8_t = 0;
        let mut repl_limit_read: ::core::ffi::c_double = 0.;
        let mut repl_limit_write: ::core::ffi::c_double = 0.;
        let mut repl_read_counter: ::core::ffi::c_double = 0.;
        let mut i: uint16_t = 0;
        let mut j: uint16_t = 0;
        let mut k: uint16_t = 0;
        let mut done: uint8_t = 0;
        let mut vc: uint16_t = 0;
        let mut tdc: uint16_t = 0;
        let mut ivc: uint16_t = 0;
        let mut bc: uint16_t = 0;
        let mut tdb: uint16_t = 0;
        let mut dc: uint16_t = 0;
        let mut wvc: uint16_t = 0;
        let mut tdw: uint16_t = 0;
        let mut vcmask4: uint32_t = 0;
        let mut tdcmask4: uint32_t = 0;
        let mut ivcmask4: uint32_t = 0;
        let mut bcmask4: uint32_t = 0;
        let mut tdbmask4: uint32_t = 0;
        let mut dcmask4: uint32_t = 0;
        let mut wvcmask4: uint32_t = 0;
        let mut tdwmask4: uint32_t = 0;
        let mut vcmask8: uint32_t = 0;
        let mut tdcmask8: uint32_t = 0;
        let mut ivcmask8: uint32_t = 0;
        let mut bcmask8: uint32_t = 0;
        let mut tdbmask8: uint32_t = 0;
        let mut dcmask8: uint32_t = 0;
        let mut wvcmask8: uint32_t = 0;
        let mut tdwmask8: uint32_t = 0;
        let mut overmask4: uint32_t = 0;
        let mut overmask8: uint32_t = 0;
        let mut tdovermask4: uint32_t = 0;
        let mut tdovermask8: uint32_t = 0;
        let mut mfrmask4: uint32_t = 0;
        let mut mfrmask8: uint32_t = 0;
        let mut mask: uint32_t = 0;
        let mut wlmask: uint32_t = 0;
        let mut ltotalec4csid: int32_t = 0;
        let mut ltotalec8csid: int32_t = 0;
        let mut ldataec4csid: int32_t = 0;
        let mut ldataec8csid: int32_t = 0;
        let mut lchksumec4csid: int32_t = 0;
        let mut lchksumec8csid: int32_t = 0;
        let mut lregec4csid: int32_t = 0;
        let mut lregec8csid: int32_t = 0;
        let mut lallec4csid: int32_t = 0;
        let mut lallec8csid: int32_t = 0;
        let mut regec4uniqserv: uint16_t = 0;
        let mut regec8uniqserv: uint16_t = 0;
        let mut allec4uniqserv: uint16_t = 0;
        let mut allec8uniqserv: uint16_t = 0;
        let mut totalec4uniqserv: uint16_t = 0;
        let mut totalec8uniqserv: uint16_t = 0;
        let mut dataec4uniqserv: uint16_t = 0;
        let mut dataec8uniqserv: uint16_t = 0;
        let mut chksumec4uniqserv: uint16_t = 0;
        let mut chksumec8uniqserv: uint16_t = 0;
        let mut validec4uniqserv: uint16_t = 0;
        let mut validec8uniqserv: uint16_t = 0;
        let mut validdataec4uniqserv: uint16_t = 0;
        let mut validdataec8uniqserv: uint16_t = 0;
        let mut validchksumec4uniqserv: uint16_t = 0;
        let mut validchksumec8uniqserv: uint16_t = 0;
        let mut allecgoalequiv: uint32_t = 0;
        let mut regularecgoalequiv: uint32_t = 0;
        let mut validecgoalequiv: uint32_t = 0;
        let mut storage_mode: uint8_t = 0;
        let mut labels_mode: uint8_t = 0;
        let mut ec_strict_mode: uint8_t = 0;
        let mut dataec_both_labels_limit: int32_t = 0;
        let mut chksumec_both_labels_limit: int32_t = 0;
        let mut goal: uint8_t = 0;
        let mut ec_data_parts: uint8_t = 0;
        let mut ec_chksum_parts: uint8_t = 0;
        let mut minecid: uint8_t = 0;
        let mut maxecid: uint8_t = 0;
        let mut ecidmask: uint8_t = 0;
        let mut parts_on_the_same_server: uint8_t = 0;
        let mut lecid: uint8_t = 0;
        let mut can_delete_invalid_chunks: uint8_t = 0;
        let mut lcsid: int32_t = 0;
        let mut lcsid4: int32_t = 0;
        let mut lcsid8: int32_t = 0;
        let mut repecid: uint8_t = 0;
        let mut usekeep: uint8_t = 0;
        let mut dontdelete: uint8_t = 0;
        let mut overloaded: uint8_t = 0;
        let mut maxdiff: ::core::ffi::c_double = 0.;
        static mut inforec: loop_info = loop_info {
            fixed: 0,
            forcekeep: 0,
            delete_invalid: 0,
            delete_no_longer_needed: 0,
            delete_wrong_version: 0,
            delete_duplicated_ecpart: 0,
            delete_excess_ecpart: 0,
            delete_excess_copy: 0,
            delete_diskclean_ecpart: 0,
            delete_diskclean_copy: 0,
            replicate_dupserver_ecpart: 0,
            replicate_needed_ecpart: 0,
            replicate_needed_copy: 0,
            replicate_wronglabels_ecpart: 0,
            replicate_wronglabels_copy: 0,
            split_copy_into_ecparts: 0,
            join_ecparts_into_copy: 0,
            recover_ecpart: 0,
            calculate_ecchksum: 0,
            locked_unused: 0,
            locked_used: 0,
            replicate_rebalance: 0,
        };
        static mut delnotdone: uint32_t = 0;
        static mut deldone: uint32_t = 0;
        static mut prevtodeletecount: uint32_t = 0;
        static mut delloopcnt: uint32_t = 0;
        let mut sm: *mut storagemode = ::core::ptr::null_mut::<storagemode>();
        static mut servers: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
        static mut ecids: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut servcnt: uint32_t = 0;
        let mut extraservcnt: uint32_t = 0;
        let mut matching: *mut int32_t = ::core::ptr::null_mut::<int32_t>();
        let mut vrip: [uint32_t; 255] = [0; 255];
        let mut vripcnt: uint8_t = 0;
        if servers.is_null() {
            servers = malloc(::core::mem::size_of::<uint16_t>().wrapping_mul(MAXCSCOUNT as size_t))
                as *mut uint16_t;
            if servers.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    5856 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"servers\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    5856 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"servers\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if servers
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint16_t
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    5856 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"servers\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    5856 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"servers\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
        }
        if ecids.is_null() {
            ecids = malloc(::core::mem::size_of::<uint8_t>().wrapping_mul(MAXCSCOUNT as size_t))
                as *mut uint8_t;
            if ecids.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    5860 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ecids\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    5860 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ecids\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if ecids
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint8_t
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    5860 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ecids\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    5860 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ecids\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
        }
        if rcsids.is_null() {
            rcsids = malloc(::core::mem::size_of::<uint16_t>().wrapping_mul(MAXCSCOUNT as size_t))
                as *mut uint16_t;
            if rcsids.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    5864 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"rcsids\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    5864 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"rcsids\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if rcsids
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint16_t
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    5864 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"rcsids\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    5864 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"rcsids\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_1,
                );
                abort();
            }
        }
        if dcsids.is_null() {
            dcsids = malloc(::core::mem::size_of::<uint16_t>().wrapping_mul(MAXCSCOUNT as size_t))
                as *mut uint16_t;
            if dcsids.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    5868 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"dcsids\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    5868 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"dcsids\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if dcsids
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint16_t
            {
                let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    5868 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"dcsids\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    5868 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"dcsids\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_2,
                );
                abort();
            }
        }
        if c.is_null() {
            if mode as ::core::ffi::c_int == JOBS_INIT as ::core::ffi::c_int {
                delnotdone = 0 as uint32_t;
                deldone = 0 as uint32_t;
                prevtodeletecount = 0 as uint32_t;
                delloopcnt = 0 as uint32_t;
                memset(
                    &raw mut inforec as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<loop_info>(),
                );
                dservcount = 0 as uint16_t;
            } else if mode as ::core::ffi::c_int == JOBS_EVERYLOOP as ::core::ffi::c_int {
                delloopcnt = delloopcnt.wrapping_add(1);
                if delloopcnt >= 16 as uint32_t {
                    let mut todeletecount: uint32_t = deldone.wrapping_add(delnotdone);
                    delloopcnt = 0 as uint32_t;
                    if delnotdone > deldone && todeletecount > prevtodeletecount {
                        TmpMaxDelFrac *= 1.5f64;
                        if TmpMaxDelFrac > MaxDelHardLimit as ::core::ffi::c_double {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_WARNING,
                                b"DEL_LIMIT hard limit (%u per server) reached\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                MaxDelHardLimit,
                            );
                            TmpMaxDelFrac = MaxDelHardLimit as ::core::ffi::c_double;
                        }
                        TmpMaxDel = TmpMaxDelFrac as uint32_t;
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_NOTICE,
                            b"DEL_LIMIT temporary increased to: %u per server\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            TmpMaxDel,
                        );
                    }
                    if todeletecount < prevtodeletecount
                        && TmpMaxDelFrac > MaxDelSoftLimit as ::core::ffi::c_double
                    {
                        TmpMaxDelFrac /= 1.5f64;
                        if TmpMaxDelFrac < MaxDelSoftLimit as ::core::ffi::c_double {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_INFO,
                                b"DEL_LIMIT back to soft limit (%u per server)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                MaxDelSoftLimit,
                            );
                            TmpMaxDelFrac = MaxDelSoftLimit as ::core::ffi::c_double;
                        }
                        TmpMaxDel = TmpMaxDelFrac as uint32_t;
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_NOTICE,
                            b"DEL_LIMIT decreased back to: %u per server\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            TmpMaxDel,
                        );
                    }
                    prevtodeletecount = todeletecount;
                    delnotdone = 0 as uint32_t;
                    deldone = 0 as uint32_t;
                }
                chunksinfo = inforec;
                memset(
                    &raw mut inforec as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<loop_info>(),
                );
                chunksinfo_loopstart = chunksinfo_loopend;
                chunksinfo_loopend = now;
            } else if mode as ::core::ffi::c_int == JOBS_EVERYTICK as ::core::ffi::c_int {
                dservcount = 0 as uint16_t;
            } else if mode as ::core::ffi::c_int == JOBS_TERM as ::core::ffi::c_int {
                if !servers.is_null() {
                    free(servers as *mut ::core::ffi::c_void);
                }
                if !ecids.is_null() {
                    free(ecids as *mut ::core::ffi::c_void);
                }
                if !rcsids.is_null() {
                    free(rcsids as *mut ::core::ffi::c_void);
                }
                if !dcsids.is_null() {
                    free(dcsids as *mut ::core::ffi::c_void);
                }
            }
            return;
        }
        scount = matocsserv_servers_count();
        fullservers = matocsserv_almostfull_servers();
        replservers = matocsserv_replallowed_servers();
        if chunk_remove_disconnected_chunks(c) != 0 {
            job_exit_reasons[(*c).sclassid as usize]
                [DISCONNECTED_CHUNKSERVER as ::core::ffi::c_int as usize] = job_exit_reasons
                [(*c).sclassid as usize][DISCONNECTED_CHUNKSERVER as ::core::ffi::c_int as usize]
                .wrapping_add(1);
            return;
        }
        if (*c).lockedto < now {
            chunk_write_counters(c, 0 as uint8_t);
        }
        vc = 0 as uint16_t;
        tdc = 0 as uint16_t;
        ivc = 0 as uint16_t;
        bc = 0 as uint16_t;
        tdb = 0 as uint16_t;
        dc = 0 as uint16_t;
        wvc = 0 as uint16_t;
        tdw = 0 as uint16_t;
        vcmask4 = 0 as uint32_t;
        tdcmask4 = 0 as uint32_t;
        ivcmask4 = 0 as uint32_t;
        bcmask4 = 0 as uint32_t;
        tdbmask4 = 0 as uint32_t;
        dcmask4 = 0 as uint32_t;
        wvcmask4 = 0 as uint32_t;
        tdwmask4 = 0 as uint32_t;
        overmask4 = 0 as uint32_t;
        tdovermask4 = 0 as uint32_t;
        vcmask8 = 0 as uint32_t;
        tdcmask8 = 0 as uint32_t;
        ivcmask8 = 0 as uint32_t;
        bcmask8 = 0 as uint32_t;
        tdbmask8 = 0 as uint32_t;
        dcmask8 = 0 as uint32_t;
        wvcmask8 = 0 as uint32_t;
        tdwmask8 = 0 as uint32_t;
        overmask8 = 0 as uint32_t;
        tdovermask8 = 0 as uint32_t;
        parts_on_the_same_server = 0 as uint8_t;
        lcsid4 = -1 as ::core::ffi::c_int as int32_t;
        lcsid8 = -1 as ::core::ffi::c_int as int32_t;
        lregec4csid = -1 as ::core::ffi::c_int as int32_t;
        lregec8csid = -1 as ::core::ffi::c_int as int32_t;
        lallec4csid = -1 as ::core::ffi::c_int as int32_t;
        lallec8csid = -1 as ::core::ffi::c_int as int32_t;
        ltotalec4csid = -1 as ::core::ffi::c_int as int32_t;
        ltotalec8csid = -1 as ::core::ffi::c_int as int32_t;
        ldataec4csid = -1 as ::core::ffi::c_int as int32_t;
        ldataec8csid = -1 as ::core::ffi::c_int as int32_t;
        lchksumec4csid = -1 as ::core::ffi::c_int as int32_t;
        lchksumec8csid = -1 as ::core::ffi::c_int as int32_t;
        regec4uniqserv = 0 as uint16_t;
        regec8uniqserv = 0 as uint16_t;
        allec4uniqserv = 0 as uint16_t;
        allec8uniqserv = 0 as uint16_t;
        totalec4uniqserv = 0 as uint16_t;
        totalec8uniqserv = 0 as uint16_t;
        dataec4uniqserv = 0 as uint16_t;
        dataec8uniqserv = 0 as uint16_t;
        chksumec4uniqserv = 0 as uint16_t;
        chksumec8uniqserv = 0 as uint16_t;
        s = (*c).slisthead;
        while !s.is_null() {
            if (*s).ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                && (*s).ecid as ::core::ffi::c_int <= 0x30 as ::core::ffi::c_int
            {
                mask = ((1 as ::core::ffi::c_uint)
                    << ((*s).ecid as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
                    as uint32_t;
            } else if (*s).ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                && (*s).ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
            {
                mask = ((1 as ::core::ffi::c_uint)
                    << ((*s).ecid as ::core::ffi::c_int & 0xf as ::core::ffi::c_int))
                    as uint32_t;
            } else {
                mask = 0 as uint32_t;
            }
            if (*s).ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                && (*s).ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
            {
                if (*s).csid as int32_t != ltotalec4csid {
                    totalec4uniqserv = totalec4uniqserv.wrapping_add(1);
                }
                ltotalec4csid = (*s).csid as int32_t;
                if ((*s).ecid as ::core::ffi::c_int) < 0x14 as ::core::ffi::c_int {
                    if (*s).csid as int32_t != ldataec4csid {
                        dataec4uniqserv = dataec4uniqserv.wrapping_add(1);
                    }
                    ldataec4csid = (*s).csid as int32_t;
                } else {
                    if (*s).csid as int32_t != lchksumec4csid {
                        chksumec4uniqserv = chksumec4uniqserv.wrapping_add(1);
                    }
                    lchksumec4csid = (*s).csid as int32_t;
                }
            } else if (*s).ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                && (*s).ecid as ::core::ffi::c_int <= 0x30 as ::core::ffi::c_int
            {
                if (*s).csid as int32_t != ltotalec8csid {
                    totalec8uniqserv = totalec8uniqserv.wrapping_add(1);
                }
                ltotalec8csid = (*s).csid as int32_t;
                if ((*s).ecid as ::core::ffi::c_int) < 0x28 as ::core::ffi::c_int {
                    if (*s).csid as int32_t != ldataec8csid {
                        dataec8uniqserv = dataec8uniqserv.wrapping_add(1);
                    }
                    ldataec8csid = (*s).csid as int32_t;
                } else {
                    if (*s).csid as int32_t != lchksumec8csid {
                        chksumec8uniqserv = chksumec8uniqserv.wrapping_add(1);
                    }
                    lchksumec8csid = (*s).csid as int32_t;
                }
            }
            if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                || (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int
                || (*s).valid as ::core::ffi::c_int == BUSY as ::core::ffi::c_int
                || (*s).valid as ::core::ffi::c_int == TDBUSY as ::core::ffi::c_int
            {
                if (*s).ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
                {
                    if (*s).csid as int32_t != lallec4csid {
                        allec4uniqserv = allec4uniqserv.wrapping_add(1);
                    }
                    lallec4csid = (*s).csid as int32_t;
                } else if (*s).ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int <= 0x30 as ::core::ffi::c_int
                {
                    if (*s).csid as int32_t != lallec8csid {
                        allec8uniqserv = allec8uniqserv.wrapping_add(1);
                    }
                    lallec8csid = (*s).csid as int32_t;
                }
                if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                    || (*s).valid as ::core::ffi::c_int == BUSY as ::core::ffi::c_int
                {
                    if (*s).ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
                    {
                        if (*s).csid as int32_t != lregec4csid {
                            regec4uniqserv = regec4uniqserv.wrapping_add(1);
                        }
                        lregec4csid = (*s).csid as int32_t;
                    } else if (*s).ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x30 as ::core::ffi::c_int
                    {
                        if (*s).csid as int32_t != lregec8csid {
                            regec8uniqserv = regec8uniqserv.wrapping_add(1);
                        }
                        lregec8csid = (*s).csid as int32_t;
                    }
                }
            }
            match (*s).valid as ::core::ffi::c_int {
                0 => {
                    if (*s).ecid as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int != 0 {
                        ivcmask8 |= mask;
                    } else if (*s).ecid as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int != 0 {
                        ivcmask4 |= mask;
                    } else {
                        ivc = ivc.wrapping_add(1);
                    }
                }
                6 => {
                    if (*s).ecid as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int != 0 {
                        if (vcmask8 | tdcmask8) & mask != 0 {
                            tdovermask8 |= mask;
                        }
                        tdcmask8 |= mask;
                    } else if (*s).ecid as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int != 0 {
                        if (vcmask4 | tdcmask4) & mask != 0 {
                            tdovermask4 |= mask;
                        }
                        tdcmask4 |= mask;
                    } else {
                        tdc = tdc.wrapping_add(1);
                    }
                }
                3 => {
                    if (*s).ecid as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int != 0 {
                        if (vcmask8 | tdcmask8) & mask != 0 {
                            tdovermask8 |= mask;
                        }
                        if vcmask8 & mask != 0 {
                            overmask8 |= mask;
                        }
                        vcmask8 |= mask;
                        if lcsid8 == (*s).csid as int32_t {
                            parts_on_the_same_server = 1 as uint8_t;
                        }
                        lcsid8 = (*s).csid as int32_t;
                    } else if (*s).ecid as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int != 0 {
                        if (vcmask4 | tdcmask4) & mask != 0 {
                            tdovermask4 |= mask;
                        }
                        if vcmask4 & mask != 0 {
                            overmask4 |= mask;
                        }
                        vcmask4 |= mask;
                        if lcsid4 == (*s).csid as int32_t {
                            parts_on_the_same_server = 1 as uint8_t;
                        }
                        lcsid4 = (*s).csid as int32_t;
                    } else {
                        vc = vc.wrapping_add(1);
                    }
                }
                5 => {
                    if (*s).ecid as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int != 0 {
                        tdbmask8 |= mask;
                    } else if (*s).ecid as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int != 0 {
                        tdbmask4 |= mask;
                    } else {
                        tdb = tdb.wrapping_add(1);
                    }
                }
                2 => {
                    if (*s).ecid as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int != 0 {
                        bcmask8 |= mask;
                    } else if (*s).ecid as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int != 0 {
                        bcmask4 |= mask;
                    } else {
                        bc = bc.wrapping_add(1);
                    }
                }
                1 => {
                    if (*s).ecid as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int != 0 {
                        dcmask8 |= mask;
                    } else if (*s).ecid as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int != 0 {
                        dcmask4 |= mask;
                    } else {
                        dc = dc.wrapping_add(1);
                    }
                }
                4 => {
                    if (*s).ecid as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int != 0 {
                        wvcmask8 |= mask;
                    } else if (*s).ecid as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int != 0 {
                        wvcmask4 |= mask;
                    } else {
                        wvc = wvc.wrapping_add(1);
                    }
                }
                7 => {
                    if (*s).ecid as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int != 0 {
                        tdwmask8 |= mask;
                    } else if (*s).ecid as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int != 0 {
                        tdwmask4 |= mask;
                    } else {
                        tdw = tdw.wrapping_add(1);
                    }
                }
                _ => {}
            }
            s = (*s).next as *mut slist;
        }
        mfrmask8 = (vcmask8 ^ tdcmask8) & tdcmask8;
        mfrmask4 = (vcmask4 ^ tdcmask4) & tdcmask4;
        ltotalec4csid = -1 as ::core::ffi::c_int as int32_t;
        ltotalec8csid = -1 as ::core::ffi::c_int as int32_t;
        ldataec4csid = -1 as ::core::ffi::c_int as int32_t;
        ldataec8csid = -1 as ::core::ffi::c_int as int32_t;
        lchksumec4csid = -1 as ::core::ffi::c_int as int32_t;
        lchksumec8csid = -1 as ::core::ffi::c_int as int32_t;
        validec4uniqserv = 0 as uint16_t;
        validec8uniqserv = 0 as uint16_t;
        validdataec4uniqserv = 0 as uint16_t;
        validdataec8uniqserv = 0 as uint16_t;
        validchksumec4uniqserv = 0 as uint16_t;
        validchksumec8uniqserv = 0 as uint16_t;
        s = (*c).slisthead;
        while !s.is_null() {
            if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                || (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int
            {
                if (*s).ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
                {
                    if (*s).csid as int32_t != ltotalec4csid {
                        validec4uniqserv = validec4uniqserv.wrapping_add(1);
                    }
                    ltotalec4csid = (*s).csid as int32_t;
                    if ((*s).ecid as ::core::ffi::c_int) < 0x14 as ::core::ffi::c_int {
                        if (*s).csid as int32_t != ldataec4csid {
                            validdataec4uniqserv = validdataec4uniqserv.wrapping_add(1);
                        }
                        ldataec4csid = (*s).csid as int32_t;
                    } else {
                        if (*s).csid as int32_t != lchksumec4csid {
                            validchksumec4uniqserv = validchksumec4uniqserv.wrapping_add(1);
                        }
                        lchksumec4csid = (*s).csid as int32_t;
                    }
                } else if (*s).ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int <= 0x30 as ::core::ffi::c_int
                {
                    if (*s).csid as int32_t != ltotalec8csid {
                        validec8uniqserv = validec8uniqserv.wrapping_add(1);
                    }
                    ltotalec8csid = (*s).csid as int32_t;
                    if ((*s).ecid as ::core::ffi::c_int) < 0x28 as ::core::ffi::c_int {
                        if (*s).csid as int32_t != ldataec8csid {
                            validdataec8uniqserv = validdataec8uniqserv.wrapping_add(1);
                        }
                        ldataec8csid = (*s).csid as int32_t;
                    } else {
                        if (*s).csid as int32_t != lchksumec8csid {
                            validchksumec8uniqserv = validchksumec8uniqserv.wrapping_add(1);
                        }
                        lchksumec8csid = (*s).csid as int32_t;
                    }
                }
            }
            s = (*s).next as *mut slist;
        }
        regularecgoalequiv = chunk_calc_ecge(
            vcmask8 | bcmask8,
            vcmask4 | bcmask4,
            regec8uniqserv as uint8_t,
            regec4uniqserv as uint8_t,
            &raw mut storage_mode,
        ) as uint32_t;
        allecgoalequiv = chunk_calc_ecge(
            vcmask8 | tdcmask8 | bcmask8 | tdbmask8,
            vcmask4 | tdcmask4 | bcmask4 | tdbmask4,
            allec8uniqserv as uint8_t,
            allec4uniqserv as uint8_t,
            &raw mut storage_mode,
        ) as uint32_t;
        if (vc as ::core::ffi::c_int
            + tdc as ::core::ffi::c_int
            + bc as ::core::ffi::c_int
            + tdb as ::core::ffi::c_int) as uint32_t
            >= allecgoalequiv
        {
            storage_mode = STORAGE_MODE_COPIES as uint8_t;
        }
        if (tdb as ::core::ffi::c_int | bc as ::core::ffi::c_int) as uint32_t
            | tdbmask8
            | bcmask8
            | tdbmask4
            | bcmask4
            == 0 as uint32_t
            && (*c).operation() as ::core::ffi::c_int != NONE as ::core::ffi::c_int
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"chunk %016lX_%08X: chunk in the middle of the operation %s, but no chunk server is busy - finish operation\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                (*c).chunkid,
                (*c).version() as ::core::ffi::c_int,
                op_to_str((*c).operation() as uint8_t),
            );
            chunk_set_op(c, NONE as ::core::ffi::c_int as uint8_t);
            inforec.fixed = inforec.fixed.wrapping_add(1);
        }
        usekeep = 0 as uint8_t;
        ec_strict_mode = 0 as uint8_t;
        dataec_both_labels_limit = 0 as ::core::ffi::c_int as int32_t;
        chksumec_both_labels_limit = 0 as ::core::ffi::c_int as int32_t;
        sm = sclass_get_keeparch_storagemode((*c).sclassid as uint16_t, (*c).flags() as uint8_t);
        if (*sm).ec_data_chksum_parts != 0 {
            labels_mode = chunk_get_labels_mode_for_ec(sm, (*c).sclassid);
            usekeep = chunk_check_forcekeep_condidiotns_for_ec(
                sm,
                labels_mode,
                (vc as ::core::ffi::c_int > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    as uint8_t,
                (vcmask4 != 0 as uint32_t) as ::core::ffi::c_int as uint8_t,
                (vcmask8 != 0 as uint32_t) as ::core::ffi::c_int as uint8_t,
                replservers,
                scount,
            );
            ec_strict_mode = (usekeep as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as uint8_t;
            usekeep = (usekeep as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as uint8_t;
            ec_data_parts = ((*sm).ec_data_chksum_parts as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int) as uint8_t;
            ec_chksum_parts = ((*sm).ec_data_chksum_parts as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int) as uint8_t;
            if usekeep as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && (*sm).labelscnt as ::core::ffi::c_int == 2 as ::core::ffi::c_int
            {
                let mut ec_data_parts_on_both: uint16_t = 0;
                let mut ec_chksum_parts_on_both: uint16_t = 0;
                ec_data_parts_on_both = 0 as uint16_t;
                ec_chksum_parts_on_both = 0 as uint16_t;
                s = (*c).slisthead;
                while !s.is_null() {
                    if (*s).ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x13 as ::core::ffi::c_int
                        || (*s).ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                            && (*s).ecid as ::core::ffi::c_int <= 0x27 as ::core::ffi::c_int
                    {
                        if matocsserv_server_matches_labelexpr(
                            (*cstab.offset((*s).csid as isize)).ptr,
                            &raw mut *(&raw mut (*sm).labelexpr as *mut [uint8_t; 128])
                                .offset(0 as isize) as *mut uint8_t
                                as *const uint8_t,
                        ) as ::core::ffi::c_int
                            != 0
                            && matocsserv_server_matches_labelexpr(
                                (*cstab.offset((*s).csid as isize)).ptr,
                                &raw mut *(&raw mut (*sm).labelexpr as *mut [uint8_t; 128])
                                    .offset(1 as isize)
                                    as *mut uint8_t
                                    as *const uint8_t,
                            ) as ::core::ffi::c_int
                                != 0
                        {
                            ec_data_parts_on_both = ec_data_parts_on_both.wrapping_add(1);
                        }
                    }
                    if (*s).ecid as ::core::ffi::c_int >= 0x14 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
                        || (*s).ecid as ::core::ffi::c_int >= 0x28 as ::core::ffi::c_int
                            && (*s).ecid as ::core::ffi::c_int >= 0x30 as ::core::ffi::c_int
                    {
                        if matocsserv_server_matches_labelexpr(
                            (*cstab.offset((*s).csid as isize)).ptr,
                            &raw mut *(&raw mut (*sm).labelexpr as *mut [uint8_t; 128])
                                .offset(0 as isize) as *mut uint8_t
                                as *const uint8_t,
                        ) as ::core::ffi::c_int
                            != 0
                            && matocsserv_server_matches_labelexpr(
                                (*cstab.offset((*s).csid as isize)).ptr,
                                &raw mut *(&raw mut (*sm).labelexpr as *mut [uint8_t; 128])
                                    .offset(1 as isize)
                                    as *mut uint8_t
                                    as *const uint8_t,
                            ) as ::core::ffi::c_int
                                != 0
                        {
                            ec_chksum_parts_on_both = ec_chksum_parts_on_both.wrapping_add(1);
                        }
                    }
                    s = (*s).next as *mut slist;
                }
                dataec_both_labels_limit = ((*sm).chksum_allvalid as ::core::ffi::c_int
                    + (*sm).both_allvalid as ::core::ffi::c_int
                    - 2 as ::core::ffi::c_int * ec_chksum_parts as ::core::ffi::c_int
                    - ec_data_parts_on_both as ::core::ffi::c_int)
                    as int32_t;
                chksumec_both_labels_limit = ((*sm).data_allvalid as ::core::ffi::c_int
                    + (*sm).both_allvalid as ::core::ffi::c_int
                    - (ec_data_parts as ::core::ffi::c_int + ec_chksum_parts as ::core::ffi::c_int)
                    - ec_chksum_parts_on_both as ::core::ffi::c_int)
                    as int32_t;
            }
            if usekeep != 0 {
                if usekeep as ::core::ffi::c_int == 2 as ::core::ffi::c_int
                    && inforec.forcekeep < 10 as uint32_t
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"chunk %016lX_%08X: not enough servers to safely convert to EC format (%u servers needed) - using KEEP mode\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        (*c).chunkid,
                        (*c).version() as ::core::ffi::c_int,
                        ec_chksum_parts as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            + ec_data_parts as ::core::ffi::c_int,
                    );
                }
                if usekeep as ::core::ffi::c_int == 3 as ::core::ffi::c_int
                    && inforec.forcekeep < 10 as uint32_t
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"chunk %016lX_%08X: not enough servers to maintain EC format (%u servers needed) - using KEEP mode\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        (*c).chunkid,
                        (*c).version() as ::core::ffi::c_int,
                        ec_chksum_parts as ::core::ffi::c_int
                            + ec_data_parts as ::core::ffi::c_int,
                    );
                }
                if usekeep as ::core::ffi::c_int >= 2 as ::core::ffi::c_int
                    && inforec.forcekeep == 10 as uint32_t
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"there are more chunks that cannot be converted to EC or has to be converted back to copy format - no more messages in this loop - change definition of storage classes or add more servers\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                sm = sclass_get_keeparch_storagemode((*c).sclassid as uint16_t, 0 as uint8_t);
                inforec.forcekeep = inforec.forcekeep.wrapping_add(1);
            }
        }
        if (*sm).ec_data_chksum_parts != 0 {
            ec_data_parts = ((*sm).ec_data_chksum_parts as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int) as uint8_t;
            ec_chksum_parts = ((*sm).ec_data_chksum_parts as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int) as uint8_t;
            goal = (ec_chksum_parts as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t;
        } else {
            ec_data_parts = 0 as uint8_t;
            ec_chksum_parts = 0 as uint8_t;
            goal = (*sm).labelscnt;
        }
        if ec_data_parts != 0 {
            if (scount as ::core::ffi::c_int) < ec_data_parts as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"storage class %hhu : internal error - KEEP mode with EC defined\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*c).sclassid as ::core::ffi::c_int,
                );
                job_exit_reasons[(*c).sclassid as usize]
                    [INTERNAL_ERROR as ::core::ffi::c_int as usize] = job_exit_reasons
                    [(*c).sclassid as usize][INTERNAL_ERROR as ::core::ffi::c_int as usize]
                    .wrapping_add(1);
                return;
            }
            if goal as ::core::ffi::c_int
                + (ec_data_parts as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                > scount as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"reducing redundancy level due to lack of servers (servcnt:%u ; goal:%u->%d)\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    scount as ::core::ffi::c_int,
                    goal as ::core::ffi::c_int,
                    scount as ::core::ffi::c_int
                        - (ec_data_parts as ::core::ffi::c_int - 1 as ::core::ffi::c_int),
                );
                goal = (scount as ::core::ffi::c_int
                    - (ec_data_parts as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                    as uint8_t;
            }
        } else if goal as ::core::ffi::c_int > scount as ::core::ffi::c_int {
            goal = scount as uint8_t;
        }
        if ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
            minecid = 0x20 as uint8_t;
            maxecid = 0x30 as uint8_t;
            ecidmask = 0x1f as uint8_t;
        } else if ec_data_parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            minecid = 0x10 as uint8_t;
            maxecid = 0x1c as uint8_t;
            ecidmask = 0xf as uint8_t;
        } else if ec_data_parts as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            minecid = 0 as uint8_t;
            maxecid = 0 as uint8_t;
            ecidmask = 0 as uint8_t;
        } else {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"wrong EC mode (%u) - ignoring chunk\0".as_ptr() as *const ::core::ffi::c_char,
                ec_data_parts as ::core::ffi::c_int,
            );
            job_exit_reasons[(*c).sclassid as usize]
                [INTERNAL_ERROR as ::core::ffi::c_int as usize] = job_exit_reasons
                [(*c).sclassid as usize][INTERNAL_ERROR as ::core::ffi::c_int as usize]
                .wrapping_add(1);
            return;
        }
        if ((vc as ::core::ffi::c_int + bc as ::core::ffi::c_int) as uint32_t)
            .wrapping_add(regularecgoalequiv)
            < goal as uint32_t
            && (tdc as ::core::ffi::c_int | tdb as ::core::ffi::c_int) as uint32_t
                | tdcmask8
                | tdbmask8
                | tdcmask4
                | tdbmask4
                != 0 as uint32_t
        {
            s = (*c).slisthead;
            while !s.is_null() {
                if (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int
                    || (*s).valid as ::core::ffi::c_int == TDBUSY as ::core::ffi::c_int
                {
                    (*cstab.offset((*s).csid as isize)).set_mfr_state(
                        REPL_IN_PROGRESS as ::core::ffi::c_int as ::core::ffi::c_uint
                            as ::core::ffi::c_uint,
                    );
                }
                s = (*s).next as *mut slist;
            }
        }
        validecgoalequiv = chunk_calc_ecge(
            vcmask8 | tdcmask8,
            vcmask4 | tdcmask4,
            0xff as uint8_t,
            0xff as uint8_t,
            &raw mut storage_mode,
        ) as uint32_t;
        if (*c).lockedto < now
            && chunk_replock_test((*c).chunkid, now) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            if (tdb as ::core::ffi::c_int | bc as ::core::ffi::c_int) as uint32_t
                | tdbmask8
                | bcmask8
                | tdbmask4
                | bcmask4
                != 0 as uint32_t
                && (*c).operation() as ::core::ffi::c_int == NONE as ::core::ffi::c_int
            {
                if ((tdc as ::core::ffi::c_int + vc as ::core::ffi::c_int) as uint32_t)
                    .wrapping_add(validecgoalequiv)
                    > 0 as uint32_t
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"chunk %016lX_%08X: unexpected BUSY copies - fixing\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*c).chunkid,
                        (*c).version() as ::core::ffi::c_int,
                    );
                    s = (*c).slisthead;
                    while !s.is_null() {
                        if (*s).valid as ::core::ffi::c_int == BUSY as ::core::ffi::c_int {
                            (*s).valid = INVALID as ::core::ffi::c_int as uint8_t;
                            (*s).version = 0 as uint32_t;
                        } else if (*s).valid as ::core::ffi::c_int == TDBUSY as ::core::ffi::c_int {
                            (*s).valid = INVALID as ::core::ffi::c_int as uint8_t;
                            (*s).version = 0 as uint32_t;
                        }
                        s = (*s).next as *mut slist;
                    }
                    inforec.fixed = inforec.fixed.wrapping_add(1);
                    job_exit_reasons[(*c).sclassid as usize]
                        [DELETED_UNEXPECTED_BUSY_COPY_OR_PART as ::core::ffi::c_int as usize] =
                        job_exit_reasons[(*c).sclassid as usize]
                            [DELETED_UNEXPECTED_BUSY_COPY_OR_PART as ::core::ffi::c_int as usize]
                            .wrapping_add(1);
                    return;
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"chunk %016lX_%08X: unexpected BUSY copies - can't fix\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*c).chunkid,
                        (*c).version() as ::core::ffi::c_int,
                    );
                }
            }
            if ((tdc as ::core::ffi::c_int
                + vc as ::core::ffi::c_int
                + tdb as ::core::ffi::c_int
                + bc as ::core::ffi::c_int) as uint32_t)
                .wrapping_add(allecgoalequiv)
                == 0 as uint32_t
                && (wvc as ::core::ffi::c_int | tdw as ::core::ffi::c_int) as uint32_t
                    | wvcmask8
                    | tdwmask8
                    | wvcmask4
                    | tdwmask4
                    != 0 as uint32_t
                && (*c).fhead > FLISTNULLINDX as uint32_t
            {
                let mut bestversion: uint32_t = 0;
                bestversion = 0 as uint32_t;
                if tdw as ::core::ffi::c_int + wvc as ::core::ffi::c_int
                    >= goal as ::core::ffi::c_int
                {
                    s = (*c).slisthead;
                    while !s.is_null() {
                        if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            && ((*s).valid as ::core::ffi::c_int == WVER as ::core::ffi::c_int
                                || (*s).valid as ::core::ffi::c_int == TDWVER as ::core::ffi::c_int)
                        {
                            if (*s).version >= bestversion {
                                bestversion = (*s).version;
                            }
                        }
                        s = (*s).next as *mut slist;
                    }
                    if bestversion > 0 as uint32_t
                        && (bestversion.wrapping_add(1 as uint32_t) == (*c).version() as uint32_t
                            || ((*c).version() as uint32_t).wrapping_add(1 as uint32_t)
                                == bestversion)
                    {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"chunk %016lX has only copies (%u) with wrong version - fixing it\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            (*c).chunkid,
                            wvc as ::core::ffi::c_int + tdw as ::core::ffi::c_int,
                        );
                        (*c).set_version(bestversion as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        s = (*c).slisthead;
                        while !s.is_null() {
                            if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                && (*s).version == bestversion
                                && (*cstab.offset((*s).csid as isize)).valid as ::core::ffi::c_int
                                    != 0
                            {
                                if (*s).valid as ::core::ffi::c_int == WVER as ::core::ffi::c_int {
                                    (*s).valid = VALID as ::core::ffi::c_int as uint8_t;
                                } else if (*s).valid as ::core::ffi::c_int
                                    == TDWVER as ::core::ffi::c_int
                                {
                                    (*s).valid = TDVALID as ::core::ffi::c_int as uint8_t;
                                }
                            }
                            if (*s).valid as ::core::ffi::c_int == WVER as ::core::ffi::c_int
                                || (*s).valid as ::core::ffi::c_int == TDWVER as ::core::ffi::c_int
                            {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_INFO,
                                    b"chunk %016lX_%08X - wrong versioned copy on (%s - ver:%08X)\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    (*c).chunkid,
                                    (*c).version() as ::core::ffi::c_int,
                                    matocsserv_getstrip((*cstab.offset((*s).csid as isize)).ptr),
                                    (*s).version,
                                );
                            } else {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_INFO,
                                    b"chunk %016lX_%08X - valid copy on (%s - ver:%08X)\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    (*c).chunkid,
                                    (*c).version() as ::core::ffi::c_int,
                                    matocsserv_getstrip((*cstab.offset((*s).csid as isize)).ptr),
                                    (*s).version,
                                );
                            }
                            s = (*s).next as *mut slist;
                        }
                        (*c).set_needverincrease(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        (*c).set_allowreadzeros(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        changelog(
                            b"%u|SETVERSION(%lu,%u)\0".as_ptr() as *const ::core::ffi::c_char,
                            main_time(),
                            (*c).chunkid,
                            (*c).version() as ::core::ffi::c_int,
                        );
                        job_exit_reasons[(*c).sclassid as usize]
                            [FIXED_COPY_VERSION as ::core::ffi::c_int as usize] = job_exit_reasons
                            [(*c).sclassid as usize]
                            [FIXED_COPY_VERSION as ::core::ffi::c_int as usize]
                            .wrapping_add(1);
                        return;
                    }
                } else if bitcount(wvcmask8 | tdwmask8) as ::core::ffi::c_int
                    >= 7 as ::core::ffi::c_int + goal as ::core::ffi::c_int
                {
                    if chunk_fix_wrong_version_ec(
                        c,
                        (7 as ::core::ffi::c_int + goal as ::core::ffi::c_int) as uint8_t,
                        0x1f as uint8_t,
                        0x20 as uint8_t,
                        0x30 as uint8_t,
                    ) != 0
                    {
                        job_exit_reasons[(*c).sclassid as usize]
                            [FIXED_EC8_VERSION as ::core::ffi::c_int as usize] = job_exit_reasons
                            [(*c).sclassid as usize]
                            [FIXED_EC8_VERSION as ::core::ffi::c_int as usize]
                            .wrapping_add(1);
                        return;
                    }
                } else if bitcount(wvcmask4 | tdwmask4) as ::core::ffi::c_int
                    >= 3 as ::core::ffi::c_int + goal as ::core::ffi::c_int
                {
                    if chunk_fix_wrong_version_ec(
                        c,
                        (7 as ::core::ffi::c_int + goal as ::core::ffi::c_int) as uint8_t,
                        0xf as uint8_t,
                        0x10 as uint8_t,
                        0x1c as uint8_t,
                    ) != 0
                    {
                        job_exit_reasons[(*c).sclassid as usize]
                            [FIXED_EC4_VERSION as ::core::ffi::c_int as usize] = job_exit_reasons
                            [(*c).sclassid as usize]
                            [FIXED_EC4_VERSION as ::core::ffi::c_int as usize]
                            .wrapping_add(1);
                        return;
                    }
                }
            }
            if ((tdc as ::core::ffi::c_int
                + vc as ::core::ffi::c_int
                + tdb as ::core::ffi::c_int
                + bc as ::core::ffi::c_int) as uint32_t)
                .wrapping_add(allecgoalequiv)
                == 0 as uint32_t
                && (wvc as ::core::ffi::c_int
                    | tdw as ::core::ffi::c_int
                    | ivc as ::core::ffi::c_int) as uint32_t
                    | wvcmask8
                    | tdwmask8
                    | ivcmask8
                    | wvcmask4
                    | tdwmask4
                    | ivcmask4
                    != 0 as uint32_t
                && (*c).fhead > FLISTNULLINDX as uint32_t
            {
                if (wvc as ::core::ffi::c_int | tdw as ::core::ffi::c_int) as uint32_t
                    | wvcmask8
                    | tdwmask8
                    | wvcmask4
                    | tdwmask4
                    == 0 as uint32_t
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"chunk %016lX_%08X has only invalid copies (%u) - please repair it manually\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        (*c).chunkid,
                        (*c).version() as ::core::ffi::c_int,
                        ivc as ::core::ffi::c_int,
                    );
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"chunk %016lX_%08X has only copies with wrong versions (%u) - please repair it manually\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        (*c).chunkid,
                        (*c).version() as ::core::ffi::c_int,
                        wvc as ::core::ffi::c_int + tdw as ::core::ffi::c_int,
                    );
                }
                s = (*c).slisthead;
                while !s.is_null() {
                    if (*s).valid as ::core::ffi::c_int == INVALID as ::core::ffi::c_int {
                        if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_INFO,
                                b"chunk %016lX_%08X - invalid copy on (%s - ver:%08X)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                (*c).chunkid,
                                (*c).version() as ::core::ffi::c_int,
                                matocsserv_getstrip((*cstab.offset((*s).csid as isize)).ptr),
                                (*s).version,
                            );
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_INFO,
                                b"chunk %016lX_%08X - invalid part %s on (%s - ver:%08X)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                (*c).chunkid,
                                (*c).version() as ::core::ffi::c_int,
                                chunk_ecid_to_str((*s).ecid),
                                matocsserv_getstrip((*cstab.offset((*s).csid as isize)).ptr),
                                (*s).version,
                            );
                        }
                    } else if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_INFO,
                            b"chunk %016lX_%08X - copy with wrong version on (%s - ver:%08X)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            (*c).chunkid,
                            (*c).version() as ::core::ffi::c_int,
                            matocsserv_getstrip((*cstab.offset((*s).csid as isize)).ptr),
                            (*s).version,
                        );
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_INFO,
                            b"chunk %016lX_%08X - part %s with wrong version on (%s - ver:%08X)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            (*c).chunkid,
                            (*c).version() as ::core::ffi::c_int,
                            chunk_ecid_to_str((*s).ecid),
                            matocsserv_getstrip((*cstab.offset((*s).csid as isize)).ptr),
                            (*s).version,
                        );
                    }
                    s = (*s).next as *mut slist;
                }
                job_exit_reasons[(*c).sclassid as usize]
                    [NO_VALID_COPIES_AND_PARTS as ::core::ffi::c_int as usize] = job_exit_reasons
                    [(*c).sclassid as usize]
                    [NO_VALID_COPIES_AND_PARTS as ::core::ffi::c_int as usize]
                    .wrapping_add(1);
                return;
            }
            if (*c).slisthead.is_null() && (*c).fhead > FLISTNULLINDX as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"chunk %016lX_%08X: there are no copies\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*c).chunkid,
                    (*c).version() as ::core::ffi::c_int,
                );
                job_exit_reasons[(*c).sclassid as usize]
                    [NO_COPIES_AND_PARTS as ::core::ffi::c_int as usize] = job_exit_reasons
                    [(*c).sclassid as usize]
                    [NO_COPIES_AND_PARTS as ::core::ffi::c_int as usize]
                    .wrapping_add(1);
                return;
            }
        }
        dontdelete = chunk_delay_is_protected((*c).chunkid);
        if dontdelete as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            can_delete_invalid_chunks = 0 as uint8_t;
            if (*c).fhead == FLISTNULLINDX as uint32_t
                && (*c).lockedto < now
                && chunk_replock_test((*c).chunkid, now) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                can_delete_invalid_chunks = 1 as uint8_t;
            } else if regularecgoalequiv >= goal as uint32_t
                || vc as ::core::ffi::c_int + bc as ::core::ffi::c_int >= goal as ::core::ffi::c_int
            {
                can_delete_invalid_chunks = 1 as uint8_t;
            } else if regularecgoalequiv > 0 as uint32_t
                || vc as ::core::ffi::c_int + bc as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            {
                if ec_strict_mode as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    if totalec4uniqserv as ::core::ffi::c_int >= replservers as ::core::ffi::c_int
                        || totalec8uniqserv as ::core::ffi::c_int
                            >= replservers as ::core::ffi::c_int
                        || vc as ::core::ffi::c_int
                            + tdc as ::core::ffi::c_int
                            + bc as ::core::ffi::c_int
                            + tdb as ::core::ffi::c_int
                            + ivc as ::core::ffi::c_int
                            + wvc as ::core::ffi::c_int
                            + tdw as ::core::ffi::c_int
                            >= replservers as ::core::ffi::c_int
                    {
                        can_delete_invalid_chunks = 2 as uint8_t;
                    }
                    if extrajob as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        && vc as ::core::ffi::c_int
                            + tdc as ::core::ffi::c_int
                            + bc as ::core::ffi::c_int
                            + tdb as ::core::ffi::c_int
                            + ivc as ::core::ffi::c_int
                            + wvc as ::core::ffi::c_int
                            + tdw as ::core::ffi::c_int
                            + dc as ::core::ffi::c_int
                            >= replservers as ::core::ffi::c_int
                    {
                        can_delete_invalid_chunks = 2 as uint8_t;
                    }
                } else if (*sm).labelscnt as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    if totalec4uniqserv as ::core::ffi::c_int
                        >= (*sm).replallowed as ::core::ffi::c_int
                        || totalec8uniqserv as ::core::ffi::c_int
                            >= (*sm).replallowed as ::core::ffi::c_int
                    {
                        can_delete_invalid_chunks = 2 as uint8_t;
                    }
                } else if (*sm).labelscnt as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                    if dataec4uniqserv as ::core::ffi::c_int
                        >= (*sm).data_replallowed as ::core::ffi::c_int
                            + (*sm).both_replallowed as ::core::ffi::c_int
                        || chksumec4uniqserv as ::core::ffi::c_int
                            >= (*sm).chksum_replallowed as ::core::ffi::c_int
                                + (*sm).both_replallowed as ::core::ffi::c_int
                    {
                        can_delete_invalid_chunks = 2 as uint8_t;
                    }
                    if dataec8uniqserv as ::core::ffi::c_int
                        >= (*sm).data_replallowed as ::core::ffi::c_int
                            + (*sm).both_replallowed as ::core::ffi::c_int
                        || chksumec8uniqserv as ::core::ffi::c_int
                            >= (*sm).chksum_replallowed as ::core::ffi::c_int
                                + (*sm).both_replallowed as ::core::ffi::c_int
                    {
                        can_delete_invalid_chunks = 2 as uint8_t;
                    }
                }
            }
            if can_delete_invalid_chunks != 0 {
                let mut actions: uint8_t = 0 as uint8_t;
                s = (*c).slisthead;
                while !s.is_null() {
                    if (matocsserv_deletion_counter((*cstab.offset((*s).csid as isize)).ptr)
                        as uint32_t)
                        < TmpMaxDel
                    {
                        if (*s).valid as ::core::ffi::c_int == WVER as ::core::ffi::c_int
                            || (*s).valid as ::core::ffi::c_int == TDWVER as ::core::ffi::c_int
                            || (*s).valid as ::core::ffi::c_int == INVALID as ::core::ffi::c_int
                            || extrajob as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                && (*s).valid as ::core::ffi::c_int == DEL as ::core::ffi::c_int
                        {
                            if matocsserv_send_deletechunk(
                                (*cstab.offset((*s).csid as isize)).ptr,
                                (*c).chunkid,
                                (*s).ecid,
                                0 as uint32_t,
                                OP_DEL_INVALID as ::core::ffi::c_int as uint8_t,
                            ) < 0 as ::core::ffi::c_int
                            {
                                if (*s).valid as ::core::ffi::c_int != DEL as ::core::ffi::c_int {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_WARNING,
                                        b"chunk %016lX_%08X: can't delete chunk (delete command already sent)\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        (*c).chunkid,
                                        (*c).version() as ::core::ffi::c_int,
                                    );
                                }
                            } else {
                                if (*s).valid as ::core::ffi::c_int == DEL as ::core::ffi::c_int {
                                    mfs_log(
                                        MFSLOG_SYSLOG,
                                        MFSLOG_WARNING,
                                        b"chunk %016lX_%08X: chunk hasn't been deleted since previous loop - retry\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        (*c).chunkid,
                                        (*c).version() as ::core::ffi::c_int,
                                    );
                                }
                                inforec.delete_invalid = inforec.delete_invalid.wrapping_add(1);
                                (*s).valid = DEL as ::core::ffi::c_int as uint8_t;
                                stats_chunkops[CHUNK_OP_DELETE_TRY as usize] =
                                    stats_chunkops[CHUNK_OP_DELETE_TRY as usize].wrapping_add(1);
                                if extrajob as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                                    deldone = deldone.wrapping_add(1);
                                }
                                if can_delete_invalid_chunks as ::core::ffi::c_int
                                    == 2 as ::core::ffi::c_int
                                {
                                    job_exit_reasons[(*c).sclassid as usize]
                                        [DELETED_INVALID_COPY_OR_PART_TO_MAKE_SPACE
                                            as ::core::ffi::c_int
                                            as usize] = job_exit_reasons[(*c).sclassid as usize]
                                        [DELETED_INVALID_COPY_OR_PART_TO_MAKE_SPACE
                                            as ::core::ffi::c_int
                                            as usize]
                                        .wrapping_add(1);
                                    return;
                                }
                                actions = 1 as uint8_t;
                            }
                        }
                    } else if (*s).valid as ::core::ffi::c_int == WVER as ::core::ffi::c_int
                        || (*s).valid as ::core::ffi::c_int == TDWVER as ::core::ffi::c_int
                        || (*s).valid as ::core::ffi::c_int == INVALID as ::core::ffi::c_int
                    {
                        if extrajob as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            delnotdone = delnotdone.wrapping_add(1);
                        }
                    }
                    s = (*s).next as *mut slist;
                }
                if actions != 0 {
                    job_exit_reasons[(*c).sclassid as usize]
                        [DELETED_SOME_INVALID_COPIES_OR_PARTS as ::core::ffi::c_int as usize] =
                        job_exit_reasons[(*c).sclassid as usize]
                            [DELETED_SOME_INVALID_COPIES_OR_PARTS as ::core::ffi::c_int as usize]
                            .wrapping_add(1);
                    return;
                }
            }
        }
        if extrajob as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            if ((*c).operation() as ::core::ffi::c_int == REPLICATE as ::core::ffi::c_int
                || (*c).operation() as ::core::ffi::c_int == LOCALSPLIT as ::core::ffi::c_int)
                && chunk_replock_test((*c).chunkid, now) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
            {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"chunk %016lX_%08X: chunk hasn't been replicated since previous loop - cancel\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    (*c).chunkid,
                    (*c).version() as ::core::ffi::c_int,
                );
                s = (*c).slisthead;
                while !s.is_null() {
                    if (*s).valid as ::core::ffi::c_int == TDBUSY as ::core::ffi::c_int
                        || (*s).valid as ::core::ffi::c_int == BUSY as ::core::ffi::c_int
                    {
                        (*s).valid = INVALID as ::core::ffi::c_int as uint8_t;
                        (*s).version = 0 as uint32_t;
                        chunk_delopchunk((*s).csid, (*c).chunkid);
                    }
                    s = (*s).next as *mut slist;
                }
                chunk_set_op(c, NONE as ::core::ffi::c_int as uint8_t);
                chunk_replock_repend((*c).chunkid);
                matoclserv_chunk_unlocked((*c).chunkid, c as *mut ::core::ffi::c_void);
                inforec.fixed = inforec.fixed.wrapping_add(1);
                job_exit_reasons[(*c).sclassid as usize]
                    [FOUND_NOT_FINISHED_REPLICATION as ::core::ffi::c_int as usize] =
                    job_exit_reasons[(*c).sclassid as usize]
                        [FOUND_NOT_FINISHED_REPLICATION as ::core::ffi::c_int as usize]
                        .wrapping_add(1);
                return;
            }
        }
        if (*c).operation() as ::core::ffi::c_int != NONE as ::core::ffi::c_int
            || (*c).lockedto >= now
            || chunk_replock_test((*c).chunkid, now) as ::core::ffi::c_int != 0
        {
            if extrajob as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if (*c).fhead == FLISTNULLINDX as uint32_t {
                    inforec.locked_unused = inforec.locked_unused.wrapping_add(1);
                } else {
                    inforec.locked_used = inforec.locked_used.wrapping_add(1);
                    if goal as uint32_t
                        > ((vc as ::core::ffi::c_int + bc as ::core::ffi::c_int) as uint32_t)
                            .wrapping_add(regularecgoalequiv)
                        && ((vc as ::core::ffi::c_int
                            + tdc as ::core::ffi::c_int
                            + bc as ::core::ffi::c_int
                            + tdb as ::core::ffi::c_int) as uint32_t)
                            .wrapping_add(allecgoalequiv)
                            > 0 as uint32_t
                    {
                        if (*c).operation() as ::core::ffi::c_int != NONE as ::core::ffi::c_int {
                            if (*c).operation() as ::core::ffi::c_int
                                != REPLICATE as ::core::ffi::c_int
                                && (*c).operation() as ::core::ffi::c_int
                                    != LOCALSPLIT as ::core::ffi::c_int
                            {
                                mfs_log(
                                    MFSLOG_SYSLOG,
                                    MFSLOG_NOTICE,
                                    b"chunk %016lX_%08X: can't replicate chunk - operation %s in progress\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    (*c).chunkid,
                                    (*c).version() as ::core::ffi::c_int,
                                    op_to_str((*c).operation() as uint8_t),
                                );
                            }
                        } else if (*c).lockedto <= now.wrapping_add(LOCKTIMEOUT as uint32_t) {
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_NOTICE,
                                b"chunk %016lX_%08X: can't replicate chunk - chunk is being modified (locked for next %u second%s)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                (*c).chunkid,
                                (*c).version() as ::core::ffi::c_int,
                                (1 as uint32_t)
                                    .wrapping_add((*c).lockedto)
                                    .wrapping_sub(now),
                                if (*c).lockedto == now {
                                    b"\0".as_ptr() as *const ::core::ffi::c_char
                                } else {
                                    b"s\0".as_ptr() as *const ::core::ffi::c_char
                                },
                            );
                        }
                    }
                }
            }
            job_exit_reasons[(*c).sclassid as usize]
                [CHUNK_IS_BEING_MODIFIED as ::core::ffi::c_int as usize] = job_exit_reasons
                [(*c).sclassid as usize][CHUNK_IS_BEING_MODIFIED as ::core::ffi::c_int as usize]
                .wrapping_add(1);
            return;
        }
        if (bc as ::core::ffi::c_int | tdb as ::core::ffi::c_int) as uint32_t
            | bcmask8
            | tdbmask8
            | bcmask4
            | tdbmask4
            != 0 as uint32_t
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"chunk %016lX_%08X has unexpected BUSY copies\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*c).chunkid,
                (*c).version() as ::core::ffi::c_int,
            );
            job_exit_reasons[(*c).sclassid as usize]
                [UNEXPECTED_BUSY_COPY_OR_PART as ::core::ffi::c_int as usize] = job_exit_reasons
                [(*c).sclassid as usize]
                [UNEXPECTED_BUSY_COPY_OR_PART as ::core::ffi::c_int as usize]
                .wrapping_add(1);
            return;
        }
        if extrajob as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (*c).fhead == FLISTNULLINDX as uint32_t
        {
            s = (*c).slisthead;
            while !s.is_null() {
                if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                    || (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int
                {
                    if (matocsserv_deletion_counter((*cstab.offset((*s).csid as isize)).ptr)
                        as uint32_t)
                        < TmpMaxDel
                    {
                        if matocsserv_send_deletechunk(
                            (*cstab.offset((*s).csid as isize)).ptr,
                            (*c).chunkid,
                            (*s).ecid,
                            (*c).version() as uint32_t,
                            OP_DEL_NOTUSED as ::core::ffi::c_int as uint8_t,
                        ) >= 0 as ::core::ffi::c_int
                        {
                            (*s).valid = DEL as ::core::ffi::c_int as uint8_t;
                            stats_chunkops[CHUNK_OP_DELETE_TRY as usize] =
                                stats_chunkops[CHUNK_OP_DELETE_TRY as usize].wrapping_add(1);
                            inforec.delete_no_longer_needed =
                                inforec.delete_no_longer_needed.wrapping_add(1);
                            deldone = deldone.wrapping_add(1);
                        } else {
                            delnotdone = delnotdone.wrapping_add(1);
                        }
                    } else {
                        delnotdone = delnotdone.wrapping_add(1);
                    }
                }
                s = (*s).next as *mut slist;
            }
            job_exit_reasons[(*c).sclassid as usize]
                [DELETED_UNUSED_CHUNK as ::core::ffi::c_int as usize] = job_exit_reasons
                [(*c).sclassid as usize][DELETED_UNUSED_CHUNK as ::core::ffi::c_int as usize]
                .wrapping_add(1);
            return;
        }
        if (extrajob as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
            chunk_priority = chunk_calculate_endanger_priority(c, 1 as uint8_t);
        } else {
            chunk_priority = 0 as uint8_t;
        }
        if extrajob as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            i = 0 as uint16_t;
            while (i as ::core::ffi::c_int) < chunk_priority as ::core::ffi::c_int {
                if chunk_priority_should_block(i as uint8_t) != 0 {
                    if (chunk_priority as ::core::ffi::c_int) < DANGER_PRIORITIES {
                        chunk_priority_enqueue(chunk_priority, c);
                    }
                    job_exit_reasons[(*c).sclassid as usize]
                        [BLOCKED_BY_HIGHER_PRIORITY_QUEUE as ::core::ffi::c_int as usize] =
                        job_exit_reasons[(*c).sclassid as usize]
                            [BLOCKED_BY_HIGHER_PRIORITY_QUEUE as ::core::ffi::c_int as usize]
                            .wrapping_add(1);
                    return;
                }
                i = i.wrapping_add(1);
            }
        }
        if (extrajob as ::core::ffi::c_int) < 2 as ::core::ffi::c_int
            && csdb_stop_chunk_jobs() as ::core::ffi::c_int != 0
        {
            if (chunk_priority as ::core::ffi::c_int) < DANGER_PRIORITIES {
                chunk_priority_enqueue(chunk_priority, c);
            }
            job_exit_reasons[(*c).sclassid as usize]
                [BLOCKED_BY_CHUNKSERVER_IN_MAINTENANCE_MODE as ::core::ffi::c_int as usize] =
                job_exit_reasons[(*c).sclassid as usize]
                    [BLOCKED_BY_CHUNKSERVER_IN_MAINTENANCE_MODE as ::core::ffi::c_int as usize]
                    .wrapping_add(1);
            return;
        }
        if chunk_priority as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            repl_limit_class = (1 as ::core::ffi::c_int
                - (if chunk_priority as ::core::ffi::c_int <= CHUNK_PRIORITY_ONECOPY_ANY {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint8_t;
        } else {
            repl_limit_class = 4 as uint8_t;
        }
        repl_limit_read = MaxReadRepl[repl_limit_class as usize];
        repl_limit_write = MaxWriteRepl[repl_limit_class as usize];
        if dontdelete as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && regularecgoalequiv < goal as uint32_t
            && (ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int
                && tdcmask8 != 0 as uint32_t
                || ec_data_parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int
                    && tdcmask4 != 0 as uint32_t)
            && chunk_priority_is_empty(CHUNK_PRIORITY_ONECOPY_HIGHGOAL as uint8_t)
                as ::core::ffi::c_int
                != 0
            && chunk_priority_is_empty(CHUNK_PRIORITY_ONECOPY_ANY as uint8_t) as ::core::ffi::c_int
                != 0
        {
            let mut can_delete_mark_for_removal: uint8_t = 0;
            let mut vcmask: uint32_t = 0;
            can_delete_mark_for_removal = 0 as uint8_t;
            if ec_strict_mode as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if ec_data_parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int
                    && validec4uniqserv as ::core::ffi::c_int >= replservers as ::core::ffi::c_int
                    || ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int
                        && validec8uniqserv as ::core::ffi::c_int
                            >= replservers as ::core::ffi::c_int
                {
                    can_delete_mark_for_removal = 1 as uint8_t;
                }
            } else if (*sm).labelscnt as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                if ec_data_parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int
                    && validec4uniqserv as ::core::ffi::c_int
                        >= (*sm).replallowed as ::core::ffi::c_int
                    || ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int
                        && validec8uniqserv as ::core::ffi::c_int
                            >= (*sm).replallowed as ::core::ffi::c_int
                {
                    can_delete_mark_for_removal = 1 as uint8_t;
                }
            } else if (*sm).labelscnt as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                if ec_data_parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int
                    && (validdataec4uniqserv as ::core::ffi::c_int
                        >= (*sm).data_replallowed as ::core::ffi::c_int
                            + (*sm).both_replallowed as ::core::ffi::c_int
                        || validchksumec4uniqserv as ::core::ffi::c_int
                            >= (*sm).chksum_replallowed as ::core::ffi::c_int
                                + (*sm).both_replallowed as ::core::ffi::c_int)
                {
                    can_delete_mark_for_removal = 1 as uint8_t;
                }
                if ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int
                    && (validdataec8uniqserv as ::core::ffi::c_int
                        >= (*sm).data_replallowed as ::core::ffi::c_int
                            + (*sm).both_replallowed as ::core::ffi::c_int
                        || validchksumec8uniqserv as ::core::ffi::c_int
                            >= (*sm).chksum_replallowed as ::core::ffi::c_int
                                + (*sm).both_replallowed as ::core::ffi::c_int)
                {
                    can_delete_mark_for_removal = 1 as uint8_t;
                }
            }
            if can_delete_mark_for_removal != 0 {
                vcmask = if ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                    tdovermask8
                } else {
                    tdovermask4
                };
                s = (*c).slisthead;
                while !s.is_null() {
                    if (*s).ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x30 as ::core::ffi::c_int
                    {
                        mask = ((1 as ::core::ffi::c_uint)
                            << ((*s).ecid as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
                            as uint32_t;
                    } else if (*s).ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
                    {
                        mask = ((1 as ::core::ffi::c_uint)
                            << ((*s).ecid as ::core::ffi::c_int & 0xf as ::core::ffi::c_int))
                            as uint32_t;
                    } else {
                        mask = 0 as uint32_t;
                    }
                    if (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int >= minecid as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= maxecid as ::core::ffi::c_int
                        && mask & vcmask != 0
                    {
                        if (matocsserv_deletion_counter((*cstab.offset((*s).csid as isize)).ptr)
                            as uint32_t)
                            < TmpMaxDel
                        {
                            if matocsserv_send_deletechunk(
                                (*cstab.offset((*s).csid as isize)).ptr,
                                (*c).chunkid,
                                (*s).ecid,
                                (*c).version() as uint32_t,
                                OP_DEL_OVERGOAL as ::core::ffi::c_int as uint8_t,
                            ) >= 0 as ::core::ffi::c_int
                            {
                                (*s).valid = DEL as ::core::ffi::c_int as uint8_t;
                                stats_chunkops[CHUNK_OP_DELETE_TRY as usize] =
                                    stats_chunkops[CHUNK_OP_DELETE_TRY as usize].wrapping_add(1);
                                inforec.delete_diskclean_ecpart =
                                    inforec.delete_diskclean_ecpart.wrapping_add(1);
                                job_exit_reasons[(*c).sclassid as usize]
                                    [DELETED_MFR_PART_TO_MAKE_SPACE as ::core::ffi::c_int
                                        as usize] = job_exit_reasons[(*c).sclassid as usize]
                                    [DELETED_MFR_PART_TO_MAKE_SPACE as ::core::ffi::c_int as usize]
                                    .wrapping_add(1);
                                return;
                            }
                        }
                    }
                    s = (*s).next as *mut slist;
                }
                chunk_priority_enqueue(chunk_priority, c);
                job_exit_reasons[(*c).sclassid as usize]
                    [CANT_DELETE_MFR_PART_TO_MAKE_SPACE as ::core::ffi::c_int as usize] =
                    job_exit_reasons[(*c).sclassid as usize]
                        [CANT_DELETE_MFR_PART_TO_MAKE_SPACE as ::core::ffi::c_int as usize]
                        .wrapping_add(1);
                return;
            }
        }
        if ec_data_parts as ::core::ffi::c_int != 0
            && parts_on_the_same_server as ::core::ffi::c_int != 0
        {
            let mut checkindex: uint8_t = 0;
            let mut overmask: uint32_t = 0;
            lcsid = -1 as ::core::ffi::c_int as int32_t;
            lecid = 0 as uint8_t;
            s = (*c).slisthead;
            while !s.is_null() {
                if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int >= minecid as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int <= maxecid as ::core::ffi::c_int
                {
                    if lcsid == (*s).csid as int32_t {
                        break;
                    }
                    lecid = (*s).ecid;
                    lcsid = (*s).csid as int32_t;
                }
                s = (*s).next as *mut slist;
            }
            sf = ::core::ptr::null_mut::<slist>();
            s = (*c).slisthead;
            while !s.is_null() {
                if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int
                        >= minecid as ::core::ffi::c_int
                            + (ec_data_parts as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            + goal as ::core::ffi::c_int
                {
                    if (*s).csid as int32_t == lcsid {
                        break;
                    }
                    if sf.is_null() {
                        sf = s;
                    }
                }
                s = (*s).next as *mut slist;
            }
            if s.is_null() && !sf.is_null() {
                s = sf;
            }
            if !s.is_null() {
                if (matocsserv_deletion_counter((*cstab.offset((*s).csid as isize)).ptr)
                    as uint32_t)
                    < TmpMaxDel
                {
                    if matocsserv_send_deletechunk(
                        (*cstab.offset((*s).csid as isize)).ptr,
                        (*c).chunkid,
                        (*s).ecid,
                        (*c).version() as uint32_t,
                        OP_DEL_OVERGOAL as ::core::ffi::c_int as uint8_t,
                    ) >= 0 as ::core::ffi::c_int
                    {
                        (*s).valid = DEL as ::core::ffi::c_int as uint8_t;
                        stats_chunkops[CHUNK_OP_DELETE_TRY as usize] =
                            stats_chunkops[CHUNK_OP_DELETE_TRY as usize].wrapping_add(1);
                        inforec.delete_excess_ecpart = inforec.delete_excess_ecpart.wrapping_add(1);
                        deldone = deldone.wrapping_add(1);
                    } else {
                        delnotdone = delnotdone.wrapping_add(1);
                        chunk_priority_enqueue(chunk_priority, c);
                    }
                } else {
                    delnotdone = delnotdone.wrapping_add(1);
                    chunk_priority_enqueue(chunk_priority, c);
                }
                job_exit_reasons[(*c).sclassid as usize]
                    [DELETED_PART_ON_THE_SAME_SERVER as ::core::ffi::c_int as usize] =
                    job_exit_reasons[(*c).sclassid as usize]
                        [DELETED_PART_ON_THE_SAME_SERVER as ::core::ffi::c_int as usize]
                        .wrapping_add(1);
                return;
            }
            overmask = if ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                overmask8
            } else {
                overmask4
            };
            if overmask > 0 as uint32_t {
                sf = ::core::ptr::null_mut::<slist>();
                s = (*c).slisthead;
                while !s.is_null() {
                    if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int >= minecid as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= maxecid as ::core::ffi::c_int
                        && overmask
                            & ((1 as ::core::ffi::c_int)
                                << ((*s).ecid as ::core::ffi::c_int
                                    & ecidmask as ::core::ffi::c_int))
                                as uint32_t
                            != 0
                    {
                        if (*s).csid as int32_t == lcsid {
                            break;
                        }
                        if sf.is_null() {
                            sf = s;
                        }
                    }
                    s = (*s).next as *mut slist;
                }
                if s.is_null() && !sf.is_null() {
                    s = sf;
                }
                if !s.is_null() {
                    if (matocsserv_deletion_counter((*cstab.offset((*s).csid as isize)).ptr)
                        as uint32_t)
                        < TmpMaxDel
                    {
                        if matocsserv_send_deletechunk(
                            (*cstab.offset((*s).csid as isize)).ptr,
                            (*c).chunkid,
                            (*s).ecid,
                            (*c).version() as uint32_t,
                            OP_DEL_OVERGOAL as ::core::ffi::c_int as uint8_t,
                        ) >= 0 as ::core::ffi::c_int
                        {
                            (*s).valid = DEL as ::core::ffi::c_int as uint8_t;
                            stats_chunkops[CHUNK_OP_DELETE_TRY as usize] =
                                stats_chunkops[CHUNK_OP_DELETE_TRY as usize].wrapping_add(1);
                            inforec.delete_excess_ecpart =
                                inforec.delete_excess_ecpart.wrapping_add(1);
                            deldone = deldone.wrapping_add(1);
                        } else {
                            delnotdone = delnotdone.wrapping_add(1);
                            chunk_priority_enqueue(chunk_priority, c);
                        }
                    } else {
                        delnotdone = delnotdone.wrapping_add(1);
                        chunk_priority_enqueue(chunk_priority, c);
                    }
                    job_exit_reasons[(*c).sclassid as usize]
                        [DELETED_DUPLICATED_PART as ::core::ffi::c_int as usize] = job_exit_reasons
                        [(*c).sclassid as usize]
                        [DELETED_DUPLICATED_PART as ::core::ffi::c_int as usize]
                        .wrapping_add(1);
                    return;
                }
            }
            if (*sm).labelscnt as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                if (*sm).labelscnt as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    checkindex = 0 as uint8_t;
                } else {
                    checkindex = 1 as uint8_t;
                }
                s = (*c).slisthead;
                while !s.is_null() {
                    if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int >= minecid as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= maxecid as ::core::ffi::c_int
                        && (*s).csid as int32_t == lcsid
                    {
                        if ((*s).ecid as ::core::ffi::c_int & ecidmask as ::core::ffi::c_int)
                            < ec_data_parts as ::core::ffi::c_int
                        {
                            if matocsserv_server_matches_labelexpr(
                                (*cstab.offset((*s).csid as isize)).ptr,
                                &raw mut *(&raw mut (*sm).labelexpr as *mut [uint8_t; 128])
                                    .offset(0 as isize)
                                    as *mut uint8_t
                                    as *const uint8_t,
                            ) as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                lecid = (*s).ecid;
                                break;
                            }
                        } else if matocsserv_server_matches_labelexpr(
                            (*cstab.offset((*s).csid as isize)).ptr,
                            &raw mut *(&raw mut (*sm).labelexpr as *mut [uint8_t; 128])
                                .offset(checkindex as isize)
                                as *mut uint8_t as *const uint8_t,
                        ) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            lecid = (*s).ecid;
                            break;
                        }
                    }
                    s = (*s).next as *mut slist;
                }
            }
            done = 0 as uint8_t;
            if matocsserv_replication_read_counter((*cstab.offset(lcsid as isize)).ptr, now)
                < repl_limit_read
            {
                let mut clcsid: uint16_t = 0;
                let mut wlcsid: uint16_t = 0;
                let mut clfound: uint8_t = 0;
                let mut wlfound: uint8_t = 0;
                let mut chksumflag: uint8_t = 0;
                chksumflag = (if (lecid as ::core::ffi::c_int & ecidmask as ::core::ffi::c_int)
                    < ec_data_parts as ::core::ffi::c_int
                {
                    0 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) as uint8_t;
                rservcount = matocsserv_getservers_lessrepl(
                    rcsids as *mut uint16_t,
                    repl_limit_write,
                    (if chunk_priority as ::core::ffi::c_int <= CHUNK_PRIORITY_ONECOPY_ANY {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint8_t,
                    &raw mut rservallflag,
                );
                clfound = 0 as uint8_t;
                wlfound = 0 as uint8_t;
                clcsid = 0 as uint16_t;
                wlcsid = 0 as uint16_t;
                i = 0 as uint16_t;
                while (i as ::core::ffi::c_int) < rservcount as ::core::ffi::c_int
                    && clfound as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    s = (*c).slisthead;
                    while !s.is_null()
                        && ((*s).csid as ::core::ffi::c_int
                            != *rcsids.offset(i as isize) as ::core::ffi::c_int
                            || ((*s).ecid as ::core::ffi::c_int) < minecid as ::core::ffi::c_int
                            || (*s).ecid as ::core::ffi::c_int
                                >= minecid as ::core::ffi::c_int
                                    + (ec_data_parts as ::core::ffi::c_int
                                        - 1 as ::core::ffi::c_int)
                                    + goal as ::core::ffi::c_int)
                    {
                        s = (*s).next as *mut slist;
                    }
                    if s.is_null() {
                        if chunk_check_label_for_ec_part(
                            *rcsids.offset(i as isize),
                            sm,
                            chksumflag,
                            if chksumflag as ::core::ffi::c_int != 0 {
                                &raw mut chksumec_both_labels_limit
                            } else {
                                &raw mut dataec_both_labels_limit
                            },
                        ) != 0
                        {
                            clcsid = *rcsids.offset(i as isize);
                            clfound = 1 as uint8_t;
                        } else if wlfound as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            wlcsid = *rcsids.offset(i as isize);
                            wlfound = 1 as uint8_t;
                        }
                    }
                    i = i.wrapping_add(1);
                }
                if clfound as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && wlfound as ::core::ffi::c_int != 0
                    && ec_strict_mode as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    clcsid = wlcsid;
                    clfound = 1 as uint8_t;
                }
                if clfound != 0 {
                    if chunk_replicate(
                        SIMPLE as ::core::ffi::c_int as uint8_t,
                        now,
                        c,
                        ec_data_parts,
                        lecid,
                        lcsid as uint16_t,
                        clcsid,
                        NULL,
                        ::core::ptr::null_mut::<uint16_t>(),
                        ::core::ptr::null_mut::<uint8_t>(),
                        (if extrajob as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                            RECOVER_IO as ::core::ffi::c_int
                        } else {
                            (if regularecgoalequiv == 1 as uint32_t {
                                REPL_EC_ENDANGERED as ::core::ffi::c_int
                            } else {
                                REPL_EC_UNDERGOAL as ::core::ffi::c_int
                            })
                        }) as uint8_t,
                    ) as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int
                    {
                        job_exit_reasons[(*c).sclassid as usize]
                            [ERROR_REPLICATING_DUPLICATED_PART as ::core::ffi::c_int as usize] =
                            job_exit_reasons[(*c).sclassid as usize]
                                [ERROR_REPLICATING_DUPLICATED_PART as ::core::ffi::c_int as usize]
                                .wrapping_add(1);
                        return;
                    }
                    done = 1 as uint8_t;
                    inforec.replicate_dupserver_ecpart =
                        inforec.replicate_dupserver_ecpart.wrapping_add(1);
                } else if rservallflag as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    chunk_priority_enqueue(chunk_priority, c);
                }
            } else {
                chunk_priority_enqueue(chunk_priority, c);
            }
            if done != 0 {
                job_exit_reasons[(*c).sclassid as usize]
                    [REPLICATED_DUPLICATED_PART as ::core::ffi::c_int as usize] = job_exit_reasons
                    [(*c).sclassid as usize]
                    [REPLICATED_DUPLICATED_PART as ::core::ffi::c_int as usize]
                    .wrapping_add(1);
            } else {
                job_exit_reasons[(*c).sclassid as usize]
                    [CANT_FIX_PARTS_ON_THE_SAME_SERVER as ::core::ffi::c_int as usize] =
                    job_exit_reasons[(*c).sclassid as usize]
                        [CANT_FIX_PARTS_ON_THE_SAME_SERVER as ::core::ffi::c_int as usize]
                        .wrapping_add(1);
            }
            return;
        }
        if ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int
            && overmask8 > 0 as uint32_t
            || ec_data_parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int
                && overmask4 > 0 as uint32_t
        {
            let mut overmask_0: uint32_t = 0;
            overmask_0 = if ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                overmask8
            } else {
                overmask4
            };
            enqueue = 0 as uint8_t;
            j = 0 as uint16_t;
            mask = 1 as uint32_t;
            while (j as ::core::ffi::c_int)
                < ec_data_parts as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + goal as ::core::ffi::c_int
            {
                if overmask_0 & mask != 0 {
                    let mut overcnt: uint8_t = 0;
                    let mut prevdone: uint8_t = 0;
                    overcnt = 0 as uint8_t;
                    s = (*c).slisthead;
                    while !s.is_null() {
                        if (*s).ecid as ::core::ffi::c_int >= minecid as ::core::ffi::c_int
                            && (*s).ecid as ::core::ffi::c_int <= maxecid as ::core::ffi::c_int
                            && (*s).ecid as ::core::ffi::c_int & ecidmask as ::core::ffi::c_int
                                == j as ::core::ffi::c_int
                            && (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                        {
                            overcnt = overcnt.wrapping_add(1);
                        }
                        s = (*s).next as *mut slist;
                    }
                    if overcnt as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        if dservcount as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            dservcount = matocsserv_getservers_ordered(dcsids as *mut uint16_t);
                        }
                        delnotdone = delnotdone.wrapping_add(
                            (overcnt as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as uint32_t,
                        );
                        prevdone = 1 as uint8_t;
                        if (*sm).labelscnt != 0 {
                            servcnt = 0 as uint32_t;
                            extraservcnt = 0 as uint32_t;
                            i = 0 as uint16_t;
                            while (i as ::core::ffi::c_int) < dservcount as ::core::ffi::c_int {
                                s = (*c).slisthead;
                                while !s.is_null() {
                                    if (*s).csid as ::core::ffi::c_int
                                        == *dcsids.offset(
                                            (dservcount as ::core::ffi::c_int
                                                - 1 as ::core::ffi::c_int
                                                - i as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as ::core::ffi::c_int
                                        && (*s).ecid as ::core::ffi::c_int
                                            == minecid as ::core::ffi::c_int
                                                + j as ::core::ffi::c_int
                                        && (*s).valid as ::core::ffi::c_int
                                            == VALID as ::core::ffi::c_int
                                    {
                                        if matocsserv_server_matches_labelexpr(
                                            (*cstab.offset((*s).csid as isize)).ptr,
                                            &raw mut *(&raw mut (*sm).labelexpr
                                                as *mut [uint8_t; 128])
                                                .offset(
                                                    (if (*sm).labelscnt as ::core::ffi::c_int
                                                        == 1 as ::core::ffi::c_int
                                                    {
                                                        0 as ::core::ffi::c_int
                                                    } else if (j as ::core::ffi::c_int)
                                                        < ec_data_parts as ::core::ffi::c_int
                                                    {
                                                        0 as ::core::ffi::c_int
                                                    } else {
                                                        1 as ::core::ffi::c_int
                                                    })
                                                        as isize,
                                                )
                                                as *mut uint8_t
                                                as *const uint8_t,
                                        ) != 0
                                        {
                                            extraservcnt = extraservcnt.wrapping_add(1);
                                            *servers.offset(
                                                (MAXCSCOUNT as uint32_t).wrapping_sub(extraservcnt)
                                                    as isize,
                                            ) = (*s).csid;
                                        } else {
                                            let c2rust_fresh10 = servcnt;
                                            servcnt = servcnt.wrapping_add(1);
                                            *servers.offset(c2rust_fresh10 as isize) = (*s).csid;
                                        }
                                        break;
                                    } else {
                                        s = (*s).next as *mut slist;
                                    }
                                }
                                i = i.wrapping_add(1);
                            }
                            i = 1 as uint16_t;
                            while i as uint32_t <= extraservcnt {
                                let c2rust_fresh11 = servcnt;
                                servcnt = servcnt.wrapping_add(1);
                                *servers.offset(c2rust_fresh11 as isize) = *servers
                                    .offset((MAXCSCOUNT - i as ::core::ffi::c_int) as isize);
                                i = i.wrapping_add(1);
                            }
                            i = 0 as uint16_t;
                            while (i as uint32_t) < servcnt
                                && overcnt as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                                && prevdone as ::core::ffi::c_int != 0
                            {
                                s = (*c).slisthead;
                                while !s.is_null()
                                    && ((*s).csid as ::core::ffi::c_int
                                        != *servers.offset(i as isize) as ::core::ffi::c_int
                                        || (*s).ecid as ::core::ffi::c_int
                                            != minecid as ::core::ffi::c_int
                                                + j as ::core::ffi::c_int)
                                {
                                    s = (*s).next as *mut slist;
                                }
                                if !s.is_null()
                                    && (*s).valid as ::core::ffi::c_int
                                        == VALID as ::core::ffi::c_int
                                {
                                    if (matocsserv_deletion_counter(
                                        (*cstab.offset((*s).csid as isize)).ptr,
                                    ) as uint32_t)
                                        < TmpMaxDel
                                    {
                                        if matocsserv_send_deletechunk(
                                            (*cstab.offset((*s).csid as isize)).ptr,
                                            (*c).chunkid,
                                            (*s).ecid,
                                            (*c).version() as uint32_t,
                                            OP_DEL_OVERGOAL as ::core::ffi::c_int as uint8_t,
                                        ) >= 0 as ::core::ffi::c_int
                                        {
                                            (*s).valid = DEL as ::core::ffi::c_int as uint8_t;
                                            stats_chunkops[CHUNK_OP_DELETE_TRY as usize] =
                                                stats_chunkops[CHUNK_OP_DELETE_TRY as usize]
                                                    .wrapping_add(1);
                                            inforec.delete_duplicated_ecpart =
                                                inforec.delete_duplicated_ecpart.wrapping_add(1);
                                            deldone = deldone.wrapping_add(1);
                                            delnotdone = delnotdone.wrapping_sub(1);
                                            overcnt = overcnt.wrapping_sub(1);
                                        } else {
                                            prevdone = 0 as uint8_t;
                                        }
                                    } else {
                                        prevdone = 0 as uint8_t;
                                    }
                                }
                                i = i.wrapping_add(1);
                            }
                        } else {
                            i = 0 as uint16_t;
                            while (i as ::core::ffi::c_int) < dservcount as ::core::ffi::c_int
                                && overcnt as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                                && prevdone as ::core::ffi::c_int != 0
                            {
                                s = (*c).slisthead;
                                while !s.is_null()
                                    && ((*s).csid as ::core::ffi::c_int
                                        != *dcsids.offset(
                                            (dservcount as ::core::ffi::c_int
                                                - 1 as ::core::ffi::c_int
                                                - i as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as ::core::ffi::c_int
                                        || (*s).ecid as ::core::ffi::c_int
                                            != minecid as ::core::ffi::c_int
                                                + j as ::core::ffi::c_int)
                                {
                                    s = (*s).next as *mut slist;
                                }
                                if !s.is_null()
                                    && (*s).valid as ::core::ffi::c_int
                                        == VALID as ::core::ffi::c_int
                                {
                                    if (matocsserv_deletion_counter(
                                        (*cstab.offset((*s).csid as isize)).ptr,
                                    ) as uint32_t)
                                        < TmpMaxDel
                                    {
                                        if matocsserv_send_deletechunk(
                                            (*cstab.offset((*s).csid as isize)).ptr,
                                            (*c).chunkid,
                                            (*s).ecid,
                                            (*c).version() as uint32_t,
                                            OP_DEL_OVERGOAL as ::core::ffi::c_int as uint8_t,
                                        ) >= 0 as ::core::ffi::c_int
                                        {
                                            (*s).valid = DEL as ::core::ffi::c_int as uint8_t;
                                            stats_chunkops[CHUNK_OP_DELETE_TRY as usize] =
                                                stats_chunkops[CHUNK_OP_DELETE_TRY as usize]
                                                    .wrapping_add(1);
                                            inforec.delete_duplicated_ecpart =
                                                inforec.delete_duplicated_ecpart.wrapping_add(1);
                                            deldone = deldone.wrapping_add(1);
                                            delnotdone = delnotdone.wrapping_sub(1);
                                            overcnt = overcnt.wrapping_sub(1);
                                        } else {
                                            prevdone = 0 as uint8_t;
                                        }
                                    } else {
                                        prevdone = 0 as uint8_t;
                                    }
                                }
                                i = i.wrapping_add(1);
                            }
                        }
                        if prevdone as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            enqueue = 1 as uint8_t;
                        }
                    }
                }
                j = j.wrapping_add(1);
                mask <<= 1 as ::core::ffi::c_int;
            }
            if enqueue != 0 {
                if regularecgoalequiv == goal as uint32_t {
                    chunk_priority_enqueue(CHUNK_PRIORITY_OVERGOAL as uint8_t, c);
                } else {
                    chunk_priority_enqueue(chunk_priority, c);
                }
            }
            job_exit_reasons[(*c).sclassid as usize]
                [FOUND_DUPLICATED_EC_PARTS as ::core::ffi::c_int as usize] = job_exit_reasons
                [(*c).sclassid as usize][FOUND_DUPLICATED_EC_PARTS as ::core::ffi::c_int as usize]
                .wrapping_add(1);
            return;
        }
        survivorsmask4 = 0 as uint32_t;
        survivorsmask8 = 0 as uint32_t;
        if validecgoalequiv > 0 as uint32_t {
            if vcmask8 | tdcmask8 != 0 as uint32_t {
                survivorsmask8 = chunk_find_ec_survivors(
                    c,
                    8 as uint8_t,
                    0x1f as uint8_t,
                    0x20 as uint8_t,
                    0x30 as uint8_t,
                    now,
                    repl_limit_read,
                    &raw mut eccsid8 as *mut uint16_t,
                    &raw mut survivorsecid8 as *mut uint8_t,
                    &raw mut survivorscsid8 as *mut uint16_t,
                );
            }
            if vcmask4 | tdcmask4 != 0 as uint32_t {
                survivorsmask4 = chunk_find_ec_survivors(
                    c,
                    4 as uint8_t,
                    0xf as uint8_t,
                    0x10 as uint8_t,
                    0x1c as uint8_t,
                    now,
                    repl_limit_read,
                    &raw mut eccsid4 as *mut uint16_t,
                    &raw mut survivorsecid4 as *mut uint8_t,
                    &raw mut survivorscsid4 as *mut uint16_t,
                );
            }
        }
        preferedsrccsid = MAXCSCOUNT as uint16_t;
        if vc as ::core::ffi::c_int + tdc as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            s = (*c).slisthead;
            while !s.is_null() {
                if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && matocsserv_replication_read_counter(
                        (*cstab.offset((*s).csid as isize)).ptr,
                        now,
                    ) < repl_limit_read
                {
                    if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int {
                        preferedsrccsid = (*s).csid;
                        break;
                    } else if (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int {
                        if preferedsrccsid as ::core::ffi::c_int == MAXCSCOUNT {
                            preferedsrccsid = (*s).csid;
                        }
                    }
                }
                s = (*s).next as *mut slist;
            }
        }
        if preferedsrccsid as ::core::ffi::c_int == MAXCSCOUNT
            && survivorsmask8 == 0 as uint32_t
            && survivorsmask4 == 0 as uint32_t
        {
            if (chunk_priority as ::core::ffi::c_int) < DANGER_PRIORITIES {
                chunk_priority_enqueue(chunk_priority, c);
            }
            job_exit_reasons[(*c).sclassid as usize]
                [CANT_FIND_VALID_REPLICATION_SOURCE as ::core::ffi::c_int as usize] =
                job_exit_reasons[(*c).sclassid as usize]
                    [CANT_FIND_VALID_REPLICATION_SOURCE as ::core::ffi::c_int as usize]
                    .wrapping_add(1);
            return;
        }
        if vc as ::core::ffi::c_int + tdc as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int
                && mfrmask8 != 0
                && survivorsmask8 & mfrmask8 != 0
                || ec_data_parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int
                    && mfrmask4 != 0
                    && survivorsmask4 & mfrmask4 != 0)
        {
            let mut mfrmask: uint32_t = 0;
            let mut eccsid: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
            let mut survivorsmask: uint32_t = 0;
            if ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                mfrmask = mfrmask8;
                eccsid = &raw mut eccsid8 as *mut uint16_t;
                survivorsmask = survivorsmask8;
            } else {
                mfrmask = mfrmask4;
                eccsid = &raw mut eccsid4 as *mut uint16_t;
                survivorsmask = survivorsmask4;
            }
            rservcount = matocsserv_getservers_lessrepl(
                rcsids as *mut uint16_t,
                repl_limit_write,
                (if chunk_priority as ::core::ffi::c_int <= CHUNK_PRIORITY_ONECOPY_ANY {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as uint8_t,
                &raw mut rservallflag,
            );
            if ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int
                && mfrmask8 & 0xff as uint32_t != 0
                || ec_data_parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int
                    && mfrmask4 & 0xf as uint32_t != 0
            {
                done = 0 as uint8_t;
                servcnt = chunk_get_available_servers_for_parts(
                    servers,
                    c,
                    sm,
                    ec_strict_mode,
                    0 as uint8_t,
                    &raw mut dataec_both_labels_limit,
                    minecid,
                    (minecid as ::core::ffi::c_int
                        + (ec_data_parts as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                        + goal as ::core::ffi::c_int) as uint8_t,
                    0 as uint8_t,
                    rservcount,
                    rcsids,
                ) as uint32_t;
                j = 0 as uint16_t;
                i = 0 as uint16_t;
                mask = 1 as uint32_t;
                while (i as ::core::ffi::c_int) < ec_data_parts as ::core::ffi::c_int
                    && (j as uint32_t) < servcnt
                {
                    if mask & survivorsmask & mfrmask != 0 {
                        if chunk_replicate(
                            SIMPLE as ::core::ffi::c_int as uint8_t,
                            now,
                            c,
                            ec_data_parts,
                            (i as ::core::ffi::c_int + minecid as ::core::ffi::c_int) as uint8_t,
                            *eccsid.offset(i as isize),
                            *servers.offset(j as isize),
                            NULL,
                            ::core::ptr::null_mut::<uint16_t>(),
                            ::core::ptr::null_mut::<uint8_t>(),
                            (if extrajob as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                                RECOVER_IO as ::core::ffi::c_int
                            } else {
                                (if regularecgoalequiv == 1 as uint32_t {
                                    REPL_EC_ENDANGERED as ::core::ffi::c_int
                                } else {
                                    REPL_EC_UNDERGOAL as ::core::ffi::c_int
                                })
                            }) as uint8_t,
                        ) as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int
                        {
                            job_exit_reasons[(*c).sclassid as usize]
                                [ERROR_REPLICATING_MFR_DATA_PART as ::core::ffi::c_int as usize] =
                                job_exit_reasons[(*c).sclassid as usize]
                                    [ERROR_REPLICATING_MFR_DATA_PART as ::core::ffi::c_int
                                        as usize]
                                    .wrapping_add(1);
                            return;
                        }
                        done = 1 as uint8_t;
                        inforec.replicate_needed_ecpart =
                            inforec.replicate_needed_ecpart.wrapping_add(1);
                        j = j.wrapping_add(1);
                    }
                    i = i.wrapping_add(1);
                    mask <<= 1 as ::core::ffi::c_int;
                }
            } else {
                done = 2 as uint8_t;
                servcnt = chunk_get_available_servers_for_parts(
                    servers,
                    c,
                    sm,
                    ec_strict_mode,
                    0 as uint8_t,
                    &raw mut chksumec_both_labels_limit,
                    minecid,
                    (minecid as ::core::ffi::c_int
                        + (ec_data_parts as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                        + goal as ::core::ffi::c_int) as uint8_t,
                    1 as uint8_t,
                    rservcount,
                    rcsids,
                ) as uint32_t;
                j = 0 as uint16_t;
                i = ec_data_parts as uint16_t;
                mask =
                    ((1 as ::core::ffi::c_uint) << ec_data_parts as ::core::ffi::c_int) as uint32_t;
                while (i as ::core::ffi::c_int)
                    < ec_data_parts as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                        + goal as ::core::ffi::c_int
                    && (j as uint32_t) < servcnt
                {
                    if mask & survivorsmask & mfrmask != 0 {
                        if chunk_replicate(
                            SIMPLE as ::core::ffi::c_int as uint8_t,
                            now,
                            c,
                            ec_data_parts,
                            (i as ::core::ffi::c_int + minecid as ::core::ffi::c_int) as uint8_t,
                            *eccsid.offset(i as isize),
                            *servers.offset(j as isize),
                            NULL,
                            ::core::ptr::null_mut::<uint16_t>(),
                            ::core::ptr::null_mut::<uint8_t>(),
                            (if extrajob as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                                RECOVER_IO as ::core::ffi::c_int
                            } else {
                                (if regularecgoalequiv == 1 as uint32_t {
                                    REPL_EC_ENDANGERED as ::core::ffi::c_int
                                } else {
                                    REPL_EC_UNDERGOAL as ::core::ffi::c_int
                                })
                            }) as uint8_t,
                        ) as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int
                        {
                            job_exit_reasons[(*c).sclassid as usize]
                                [ERROR_REPLICATING_MFR_CHKSUM_PART as ::core::ffi::c_int
                                    as usize] = job_exit_reasons[(*c).sclassid as usize]
                                [ERROR_REPLICATING_MFR_CHKSUM_PART as ::core::ffi::c_int as usize]
                                .wrapping_add(1);
                            return;
                        }
                        done = 3 as uint8_t;
                        inforec.replicate_needed_ecpart =
                            inforec.replicate_needed_ecpart.wrapping_add(1);
                        j = j.wrapping_add(1);
                    }
                    i = i.wrapping_add(1);
                    mask <<= 1 as ::core::ffi::c_int;
                }
            }
            if j as uint32_t == servcnt
                && rservallflag as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                chunk_priority_enqueue(chunk_priority, c);
            }
            match done as ::core::ffi::c_int {
                0 => {
                    job_exit_reasons[(*c).sclassid as usize]
                        [CANT_REPLICATE_MFR_DATA_PART as ::core::ffi::c_int as usize] =
                        job_exit_reasons[(*c).sclassid as usize]
                            [CANT_REPLICATE_MFR_DATA_PART as ::core::ffi::c_int as usize]
                            .wrapping_add(1);
                }
                1 => {
                    job_exit_reasons[(*c).sclassid as usize]
                        [REPLICATED_MFR_DATA_PART as ::core::ffi::c_int as usize] =
                        job_exit_reasons[(*c).sclassid as usize]
                            [REPLICATED_MFR_DATA_PART as ::core::ffi::c_int as usize]
                            .wrapping_add(1);
                }
                2 => {
                    job_exit_reasons[(*c).sclassid as usize]
                        [CANT_REPLICATE_MFR_CHKSUM_PART as ::core::ffi::c_int as usize] =
                        job_exit_reasons[(*c).sclassid as usize]
                            [CANT_REPLICATE_MFR_CHKSUM_PART as ::core::ffi::c_int as usize]
                            .wrapping_add(1);
                }
                3 => {
                    job_exit_reasons[(*c).sclassid as usize]
                        [REPLICATED_MFR_CHKSUM_PART as ::core::ffi::c_int as usize] =
                        job_exit_reasons[(*c).sclassid as usize]
                            [REPLICATED_MFR_CHKSUM_PART as ::core::ffi::c_int as usize]
                            .wrapping_add(1);
                }
                _ => {}
            }
            return;
        }
        if vc as ::core::ffi::c_int + tdc as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (vcmask8 | tdcmask8) & 0xff as uint32_t != 0xff as uint32_t
            && bitcount(survivorsmask8) as ::core::ffi::c_int >= 8 as ::core::ffi::c_int
        {
            rservcount = matocsserv_getservers_lessrepl(
                rcsids as *mut uint16_t,
                repl_limit_write,
                (if chunk_priority as ::core::ffi::c_int <= CHUNK_PRIORITY_ONECOPY_ANY {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as uint8_t,
                &raw mut rservallflag,
            );
            servcnt = chunk_get_available_servers_for_parts(
                servers,
                c,
                sm,
                ec_strict_mode,
                1 as uint8_t,
                &raw mut dataec_both_labels_limit,
                0x20 as uint8_t,
                (0x26 as ::core::ffi::c_int + goal as ::core::ffi::c_int) as uint8_t,
                0 as uint8_t,
                rservcount,
                rcsids,
            ) as uint32_t;
            j = 0 as uint16_t;
            done = 0 as uint8_t;
            i = 0 as uint16_t;
            mask = 1 as uint32_t;
            while (i as ::core::ffi::c_int) < 8 as ::core::ffi::c_int && (j as uint32_t) < servcnt {
                if mask & vcmask8 == 0 as uint32_t {
                    if mask & survivorsmask8 != 0 {
                        if chunk_replicate(
                            SIMPLE as ::core::ffi::c_int as uint8_t,
                            now,
                            c,
                            8 as uint8_t,
                            (i as ::core::ffi::c_int + 0x20 as ::core::ffi::c_int) as uint8_t,
                            eccsid8[i as usize],
                            *servers.offset(j as isize),
                            NULL,
                            ::core::ptr::null_mut::<uint16_t>(),
                            ::core::ptr::null_mut::<uint8_t>(),
                            (if extrajob as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                                RECOVER_IO as ::core::ffi::c_int
                            } else {
                                (if regularecgoalequiv == 1 as uint32_t {
                                    REPL_EC_ENDANGERED as ::core::ffi::c_int
                                } else {
                                    REPL_EC_UNDERGOAL as ::core::ffi::c_int
                                })
                            }) as uint8_t,
                        ) as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int
                        {
                            job_exit_reasons[(*c).sclassid as usize]
                                [ERROR_REPLICATING_MISSING_EC8_DATA_PART_FROM_MFR
                                    as ::core::ffi::c_int
                                    as usize] = job_exit_reasons[(*c).sclassid as usize]
                                [ERROR_REPLICATING_MISSING_EC8_DATA_PART_FROM_MFR
                                    as ::core::ffi::c_int
                                    as usize]
                                .wrapping_add(1);
                            return;
                        }
                        done = 1 as uint8_t;
                        inforec.replicate_needed_ecpart =
                            inforec.replicate_needed_ecpart.wrapping_add(1);
                    } else {
                        if chunk_replicate(
                            RECOVER as ::core::ffi::c_int as uint8_t,
                            now,
                            c,
                            8 as uint8_t,
                            (i as ::core::ffi::c_int + 0x20 as ::core::ffi::c_int) as uint8_t,
                            0 as uint16_t,
                            *servers.offset(j as isize),
                            NULL,
                            &raw mut survivorscsid8 as *mut uint16_t,
                            &raw mut survivorsecid8 as *mut uint8_t,
                            (if extrajob as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                                RECOVER_IO as ::core::ffi::c_int
                            } else {
                                (if regularecgoalequiv == 1 as uint32_t {
                                    REPL_EC_ENDANGERED as ::core::ffi::c_int
                                } else {
                                    REPL_EC_UNDERGOAL as ::core::ffi::c_int
                                })
                            }) as uint8_t,
                        ) as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int
                        {
                            job_exit_reasons[(*c).sclassid as usize]
                                [ERROR_RECOVERING_MISSING_EC8_DATA_PART as ::core::ffi::c_int
                                    as usize] = job_exit_reasons[(*c).sclassid as usize]
                                [ERROR_RECOVERING_MISSING_EC8_DATA_PART as ::core::ffi::c_int
                                    as usize]
                                .wrapping_add(1);
                            return;
                        }
                        done = 1 as uint8_t;
                        inforec.recover_ecpart = inforec.recover_ecpart.wrapping_add(1);
                    }
                    j = j.wrapping_add(1);
                }
                i = i.wrapping_add(1);
                mask <<= 1 as ::core::ffi::c_int;
            }
            if j as uint32_t == servcnt
                && rservallflag as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                chunk_priority_enqueue(chunk_priority, c);
            }
            if done != 0 {
                job_exit_reasons[(*c).sclassid as usize]
                    [RECOVERED_MISSING_EC8_DATA_PART as ::core::ffi::c_int as usize] =
                    job_exit_reasons[(*c).sclassid as usize]
                        [RECOVERED_MISSING_EC8_DATA_PART as ::core::ffi::c_int as usize]
                        .wrapping_add(1);
            } else {
                job_exit_reasons[(*c).sclassid as usize]
                    [CANT_RECOVER_MISSING_EC8_DATA_PART as ::core::ffi::c_int as usize] =
                    job_exit_reasons[(*c).sclassid as usize]
                        [CANT_RECOVER_MISSING_EC8_DATA_PART as ::core::ffi::c_int as usize]
                        .wrapping_add(1);
            }
            return;
        }
        if vc as ::core::ffi::c_int + tdc as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (vcmask4 | tdcmask4) & 0xf as uint32_t != 0xf as uint32_t
            && bitcount(survivorsmask4) as ::core::ffi::c_int >= 4 as ::core::ffi::c_int
        {
            rservcount = matocsserv_getservers_lessrepl(
                rcsids as *mut uint16_t,
                repl_limit_write,
                (if chunk_priority as ::core::ffi::c_int <= CHUNK_PRIORITY_ONECOPY_ANY {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as uint8_t,
                &raw mut rservallflag,
            );
            servcnt = chunk_get_available_servers_for_parts(
                servers,
                c,
                sm,
                ec_strict_mode,
                1 as uint8_t,
                &raw mut dataec_both_labels_limit,
                0x10 as uint8_t,
                (0x12 as ::core::ffi::c_int + goal as ::core::ffi::c_int) as uint8_t,
                0 as uint8_t,
                rservcount,
                rcsids,
            ) as uint32_t;
            j = 0 as uint16_t;
            done = 0 as uint8_t;
            i = 0 as uint16_t;
            mask = 1 as uint32_t;
            while (i as ::core::ffi::c_int) < 4 as ::core::ffi::c_int && (j as uint32_t) < servcnt {
                if mask & vcmask4 == 0 as uint32_t {
                    if mask & survivorsmask4 != 0 {
                        if chunk_replicate(
                            SIMPLE as ::core::ffi::c_int as uint8_t,
                            now,
                            c,
                            4 as uint8_t,
                            (i as ::core::ffi::c_int + 0x10 as ::core::ffi::c_int) as uint8_t,
                            eccsid4[i as usize],
                            *servers.offset(j as isize),
                            NULL,
                            ::core::ptr::null_mut::<uint16_t>(),
                            ::core::ptr::null_mut::<uint8_t>(),
                            (if extrajob as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                                RECOVER_IO as ::core::ffi::c_int
                            } else {
                                (if regularecgoalequiv == 1 as uint32_t {
                                    REPL_EC_ENDANGERED as ::core::ffi::c_int
                                } else {
                                    REPL_EC_UNDERGOAL as ::core::ffi::c_int
                                })
                            }) as uint8_t,
                        ) as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int
                        {
                            job_exit_reasons[(*c).sclassid as usize]
                                [ERROR_REPLICATING_MISSING_EC4_DATA_PART_FROM_MFR
                                    as ::core::ffi::c_int
                                    as usize] = job_exit_reasons[(*c).sclassid as usize]
                                [ERROR_REPLICATING_MISSING_EC4_DATA_PART_FROM_MFR
                                    as ::core::ffi::c_int
                                    as usize]
                                .wrapping_add(1);
                            return;
                        }
                        done = 1 as uint8_t;
                        inforec.replicate_needed_ecpart =
                            inforec.replicate_needed_ecpart.wrapping_add(1);
                    } else {
                        if chunk_replicate(
                            RECOVER as ::core::ffi::c_int as uint8_t,
                            now,
                            c,
                            4 as uint8_t,
                            (i as ::core::ffi::c_int + 0x10 as ::core::ffi::c_int) as uint8_t,
                            0 as uint16_t,
                            *servers.offset(j as isize),
                            NULL,
                            &raw mut survivorscsid4 as *mut uint16_t,
                            &raw mut survivorsecid4 as *mut uint8_t,
                            (if extrajob as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                                RECOVER_IO as ::core::ffi::c_int
                            } else {
                                (if regularecgoalequiv == 1 as uint32_t {
                                    REPL_EC_ENDANGERED as ::core::ffi::c_int
                                } else {
                                    REPL_EC_UNDERGOAL as ::core::ffi::c_int
                                })
                            }) as uint8_t,
                        ) as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int
                        {
                            job_exit_reasons[(*c).sclassid as usize]
                                [ERROR_RECOVERING_MISSING_EC4_DATA_PART as ::core::ffi::c_int
                                    as usize] = job_exit_reasons[(*c).sclassid as usize]
                                [ERROR_RECOVERING_MISSING_EC4_DATA_PART as ::core::ffi::c_int
                                    as usize]
                                .wrapping_add(1);
                            return;
                        }
                        done = 1 as uint8_t;
                        inforec.recover_ecpart = inforec.recover_ecpart.wrapping_add(1);
                    }
                    j = j.wrapping_add(1);
                }
                i = i.wrapping_add(1);
                mask <<= 1 as ::core::ffi::c_int;
            }
            if j as uint32_t == servcnt
                && rservallflag as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                chunk_priority_enqueue(chunk_priority, c);
            }
            if done != 0 {
                job_exit_reasons[(*c).sclassid as usize]
                    [RECOVERED_MISSING_EC4_DATA_PART as ::core::ffi::c_int as usize] =
                    job_exit_reasons[(*c).sclassid as usize]
                        [RECOVERED_MISSING_EC4_DATA_PART as ::core::ffi::c_int as usize]
                        .wrapping_add(1);
            } else {
                job_exit_reasons[(*c).sclassid as usize]
                    [CANT_RECOVER_MISSING_EC4_DATA_PART as ::core::ffi::c_int as usize] =
                    job_exit_reasons[(*c).sclassid as usize]
                        [CANT_RECOVER_MISSING_EC4_DATA_PART as ::core::ffi::c_int as usize]
                        .wrapping_add(1);
            }
            return;
        }
        if ec_data_parts as ::core::ffi::c_int != 0
            && regularecgoalequiv < goal as uint32_t
            && preferedsrccsid as ::core::ffi::c_int != MAXCSCOUNT
            && matocsserv_can_split_chunks(
                (*cstab.offset(preferedsrccsid as isize)).ptr,
                ec_data_parts,
            ) != 0
        {
            let mut missingmask: uint32_t = 0 as uint32_t;
            i = 0 as uint16_t;
            mask = 0x1 as uint32_t;
            while (i as ::core::ffi::c_int)
                < ec_data_parts as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + goal as ::core::ffi::c_int
            {
                if ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                    if mask & vcmask8 == 0 as uint32_t && mask & survivorsmask8 == 0 as uint32_t {
                        missingmask |= mask;
                    }
                } else if ec_data_parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
                    if mask & vcmask4 == 0 as uint32_t && mask & survivorsmask4 == 0 as uint32_t {
                        missingmask |= mask;
                    }
                }
                i = i.wrapping_add(1);
                mask <<= 1 as ::core::ffi::c_int;
            }
            s = (*c).slisthead;
            while !s.is_null() {
                if (*s).csid as ::core::ffi::c_int == preferedsrccsid as ::core::ffi::c_int {
                    if ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int >= 0x20 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x30 as ::core::ffi::c_int
                    {
                        mask = ((1 as ::core::ffi::c_uint)
                            << ((*s).ecid as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
                            as uint32_t;
                        missingmask &= !mask;
                    } else if ec_data_parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int >= 0x10 as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= 0x1c as ::core::ffi::c_int
                    {
                        mask = ((1 as ::core::ffi::c_uint)
                            << ((*s).ecid as ::core::ffi::c_int & 0xf as ::core::ffi::c_int))
                            as uint32_t;
                        missingmask &= !mask;
                    }
                }
                s = (*s).next as *mut slist;
            }
            if missingmask != 0 as uint32_t {
                if matocsserv_send_localsplitchunk(
                    (*cstab.offset(preferedsrccsid as isize)).ptr,
                    (*c).chunkid,
                    (*c).version() as uint32_t,
                    missingmask,
                    ec_data_parts,
                    (if ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                        LOCALSPLIT_TO_EC8 as ::core::ffi::c_int
                    } else {
                        LOCALSPLIT_TO_EC4 as ::core::ffi::c_int
                    }) as uint8_t,
                ) < 0 as ::core::ffi::c_int
                {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"chunk %016lX_%08X : error sending localsplit command\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*c).chunkid,
                        (*c).version() as ::core::ffi::c_int,
                    );
                    job_exit_reasons[(*c).sclassid as usize]
                        [ERROR_CREATING_MISSING_PARTS_FROM_COPY as ::core::ffi::c_int as usize] =
                        job_exit_reasons[(*c).sclassid as usize]
                            [ERROR_CREATING_MISSING_PARTS_FROM_COPY as ::core::ffi::c_int as usize]
                            .wrapping_add(1);
                } else {
                    stats_chunkops[CHUNK_OP_SPLIT_TRY as usize] =
                        stats_chunkops[CHUNK_OP_SPLIT_TRY as usize].wrapping_add(1);
                    chunk_addopchunk(preferedsrccsid, (*c).chunkid);
                    chunk_set_op(c, LOCALSPLIT as ::core::ffi::c_int as uint8_t);
                    chunk_replock_repstart((*c).chunkid, now);
                    i = 0 as uint16_t;
                    mask = 0x1 as uint32_t;
                    while (i as ::core::ffi::c_int)
                        < ec_data_parts as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                            + goal as ::core::ffi::c_int
                    {
                        if mask & missingmask != 0 {
                            s = slist_malloc();
                            (*s).csid = preferedsrccsid;
                            (*s).ecid = (minecid as ::core::ffi::c_int + i as ::core::ffi::c_int)
                                as uint8_t;
                            (*s).valid = BUSY as ::core::ffi::c_int as uint8_t;
                            (*s).version = (*c).version() as uint32_t;
                            chunk_new_copy(c, s);
                        }
                        i = i.wrapping_add(1);
                        mask <<= 1 as ::core::ffi::c_int;
                    }
                    job_exit_reasons[(*c).sclassid as usize]
                        [CREATED_MISSING_PARTS_FROM_COPY as ::core::ffi::c_int as usize] =
                        job_exit_reasons[(*c).sclassid as usize]
                            [CREATED_MISSING_PARTS_FROM_COPY as ::core::ffi::c_int as usize]
                            .wrapping_add(1);
                    inforec.split_copy_into_ecparts =
                        inforec.split_copy_into_ecparts.wrapping_add(1);
                }
                return;
            }
        }
        if regularecgoalequiv < goal as uint32_t
            && (ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int
                && (vcmask8 | tdcmask8) & 0xff as uint32_t == 0xff as uint32_t
                && bitcount(survivorsmask8) as ::core::ffi::c_int >= 8 as ::core::ffi::c_int
                || ec_data_parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int
                    && (vcmask4 | tdcmask4) & 0xf as uint32_t == 0xf as uint32_t
                    && bitcount(survivorsmask4) as ::core::ffi::c_int >= 4 as ::core::ffi::c_int)
        {
            rservcount = matocsserv_getservers_lessrepl(
                rcsids as *mut uint16_t,
                repl_limit_write,
                (if chunk_priority as ::core::ffi::c_int <= CHUNK_PRIORITY_ONECOPY_ANY {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as uint8_t,
                &raw mut rservallflag,
            );
            servcnt = chunk_get_available_servers_for_parts(
                servers,
                c,
                sm,
                ec_strict_mode,
                0 as uint8_t,
                &raw mut chksumec_both_labels_limit,
                minecid,
                (minecid as ::core::ffi::c_int
                    + (ec_data_parts as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    + goal as ::core::ffi::c_int) as uint8_t,
                1 as uint8_t,
                rservcount,
                rcsids,
            ) as uint32_t;
            j = 0 as uint16_t;
            done = 0 as uint8_t;
            i = ec_data_parts as uint16_t;
            mask = ((1 as ::core::ffi::c_uint) << ec_data_parts as ::core::ffi::c_int) as uint32_t;
            while (i as ::core::ffi::c_int)
                < ec_data_parts as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + goal as ::core::ffi::c_int
                && (j as uint32_t) < servcnt
            {
                if ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                    if mask & vcmask8 == 0 as uint32_t {
                        if mask & survivorsmask8 != 0 {
                            if chunk_replicate(
                                SIMPLE as ::core::ffi::c_int as uint8_t,
                                now,
                                c,
                                ec_data_parts,
                                (i as ::core::ffi::c_int + 0x20 as ::core::ffi::c_int) as uint8_t,
                                eccsid8[i as usize],
                                *servers.offset(j as isize),
                                NULL,
                                ::core::ptr::null_mut::<uint16_t>(),
                                ::core::ptr::null_mut::<uint8_t>(),
                                (if regularecgoalequiv == 1 as uint32_t {
                                    REPL_EC_ENDANGERED as ::core::ffi::c_int
                                } else {
                                    REPL_EC_UNDERGOAL as ::core::ffi::c_int
                                }) as uint8_t,
                            ) as ::core::ffi::c_int
                                != 0 as ::core::ffi::c_int
                            {
                                job_exit_reasons[(*c).sclassid as usize]
                                    [ERROR_REPLICATING_MISSING_EC8_CHKSUM_PART_FROM_MFR
                                        as ::core::ffi::c_int
                                        as usize] = job_exit_reasons[(*c).sclassid as usize]
                                    [ERROR_REPLICATING_MISSING_EC8_CHKSUM_PART_FROM_MFR
                                        as ::core::ffi::c_int
                                        as usize]
                                    .wrapping_add(1);
                                return;
                            }
                            done = 1 as uint8_t;
                            inforec.replicate_needed_ecpart =
                                inforec.replicate_needed_ecpart.wrapping_add(1);
                        } else {
                            if chunk_replicate(
                                RECOVER as ::core::ffi::c_int as uint8_t,
                                now,
                                c,
                                ec_data_parts,
                                (i as ::core::ffi::c_int + 0x20 as ::core::ffi::c_int) as uint8_t,
                                0 as uint16_t,
                                *servers.offset(j as isize),
                                NULL,
                                &raw mut survivorscsid8 as *mut uint16_t,
                                &raw mut survivorsecid8 as *mut uint8_t,
                                (if regularecgoalequiv == 1 as uint32_t {
                                    REPL_EC_ENDANGERED as ::core::ffi::c_int
                                } else {
                                    REPL_EC_UNDERGOAL as ::core::ffi::c_int
                                }) as uint8_t,
                            ) as ::core::ffi::c_int
                                != 0 as ::core::ffi::c_int
                            {
                                job_exit_reasons[(*c).sclassid as usize]
                                    [ERROR_CREATING_MISSING_EC8_CHKSUM_PART as ::core::ffi::c_int
                                        as usize] = job_exit_reasons[(*c).sclassid as usize]
                                    [ERROR_CREATING_MISSING_EC8_CHKSUM_PART as ::core::ffi::c_int
                                        as usize]
                                    .wrapping_add(1);
                                return;
                            }
                            done = 1 as uint8_t;
                            inforec.calculate_ecchksum = inforec.calculate_ecchksum.wrapping_add(1);
                        }
                        j = j.wrapping_add(1);
                    }
                } else if mask & vcmask4 == 0 as uint32_t {
                    if mask & survivorsmask4 != 0 {
                        if chunk_replicate(
                            SIMPLE as ::core::ffi::c_int as uint8_t,
                            now,
                            c,
                            ec_data_parts,
                            (i as ::core::ffi::c_int + 0x10 as ::core::ffi::c_int) as uint8_t,
                            eccsid4[i as usize],
                            *servers.offset(j as isize),
                            NULL,
                            ::core::ptr::null_mut::<uint16_t>(),
                            ::core::ptr::null_mut::<uint8_t>(),
                            (if regularecgoalequiv == 1 as uint32_t {
                                REPL_EC_ENDANGERED as ::core::ffi::c_int
                            } else {
                                REPL_EC_UNDERGOAL as ::core::ffi::c_int
                            }) as uint8_t,
                        ) as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int
                        {
                            job_exit_reasons[(*c).sclassid as usize]
                                [ERROR_REPLICATING_MISSING_EC4_CHKSUM_PART_FROM_MFR
                                    as ::core::ffi::c_int
                                    as usize] = job_exit_reasons[(*c).sclassid as usize]
                                [ERROR_REPLICATING_MISSING_EC4_CHKSUM_PART_FROM_MFR
                                    as ::core::ffi::c_int
                                    as usize]
                                .wrapping_add(1);
                            return;
                        }
                        done = 2 as uint8_t;
                        inforec.replicate_needed_ecpart =
                            inforec.replicate_needed_ecpart.wrapping_add(1);
                    } else {
                        if chunk_replicate(
                            RECOVER as ::core::ffi::c_int as uint8_t,
                            now,
                            c,
                            ec_data_parts,
                            (i as ::core::ffi::c_int + 0x10 as ::core::ffi::c_int) as uint8_t,
                            0 as uint16_t,
                            *servers.offset(j as isize),
                            NULL,
                            &raw mut survivorscsid4 as *mut uint16_t,
                            &raw mut survivorsecid4 as *mut uint8_t,
                            (if regularecgoalequiv == 1 as uint32_t {
                                REPL_EC_ENDANGERED as ::core::ffi::c_int
                            } else {
                                REPL_EC_UNDERGOAL as ::core::ffi::c_int
                            }) as uint8_t,
                        ) as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int
                        {
                            job_exit_reasons[(*c).sclassid as usize]
                                [ERROR_CREATING_MISSING_EC4_CHKSUM_PART as ::core::ffi::c_int
                                    as usize] = job_exit_reasons[(*c).sclassid as usize]
                                [ERROR_CREATING_MISSING_EC4_CHKSUM_PART as ::core::ffi::c_int
                                    as usize]
                                .wrapping_add(1);
                            return;
                        }
                        done = 2 as uint8_t;
                        inforec.calculate_ecchksum = inforec.calculate_ecchksum.wrapping_add(1);
                    }
                    j = j.wrapping_add(1);
                }
                i = i.wrapping_add(1);
                mask <<= 1 as ::core::ffi::c_int;
            }
            if j as uint32_t == servcnt
                && rservallflag as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                chunk_priority_enqueue(chunk_priority, c);
            }
            match done as ::core::ffi::c_int {
                1 => {
                    job_exit_reasons[(*c).sclassid as usize]
                        [CREATED_MISSING_EC8_CHKSUM_PART as ::core::ffi::c_int as usize] =
                        job_exit_reasons[(*c).sclassid as usize]
                            [CREATED_MISSING_EC8_CHKSUM_PART as ::core::ffi::c_int as usize]
                            .wrapping_add(1);
                }
                2 => {
                    job_exit_reasons[(*c).sclassid as usize]
                        [CREATED_MISSING_EC4_CHKSUM_PART as ::core::ffi::c_int as usize] =
                        job_exit_reasons[(*c).sclassid as usize]
                            [CREATED_MISSING_EC4_CHKSUM_PART as ::core::ffi::c_int as usize]
                            .wrapping_add(1);
                }
                _ => {
                    if ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                        job_exit_reasons[(*c).sclassid as usize]
                            [CANT_CREATE_MISSING_EC8_CHKSUM_PART as ::core::ffi::c_int as usize] =
                            job_exit_reasons[(*c).sclassid as usize]
                                [CANT_CREATE_MISSING_EC8_CHKSUM_PART as ::core::ffi::c_int
                                    as usize]
                                .wrapping_add(1);
                    } else {
                        job_exit_reasons[(*c).sclassid as usize]
                            [CANT_CREATE_MISSING_EC4_CHKSUM_PART as ::core::ffi::c_int as usize] =
                            job_exit_reasons[(*c).sclassid as usize]
                                [CANT_CREATE_MISSING_EC4_CHKSUM_PART as ::core::ffi::c_int
                                    as usize]
                                .wrapping_add(1);
                    }
                }
            }
            return;
        }
        if (ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int
            && vcmask8 & 0xff as uint32_t != 0xff as uint32_t
            || ec_data_parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int
                && vcmask4 & 0xf as uint32_t != 0xf as uint32_t)
            && preferedsrccsid as ::core::ffi::c_int != MAXCSCOUNT
        {
            rservcount = matocsserv_getservers_lessrepl(
                rcsids as *mut uint16_t,
                repl_limit_write,
                (if chunk_priority as ::core::ffi::c_int <= CHUNK_PRIORITY_ONECOPY_ANY {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as uint8_t,
                &raw mut rservallflag,
            );
            servcnt = chunk_get_available_servers_for_parts(
                servers,
                c,
                sm,
                ec_strict_mode,
                0 as uint8_t,
                &raw mut dataec_both_labels_limit,
                minecid,
                (minecid as ::core::ffi::c_int
                    + (ec_data_parts as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    + goal as ::core::ffi::c_int) as uint8_t,
                0 as uint8_t,
                rservcount,
                rcsids,
            ) as uint32_t;
            j = 0 as uint16_t;
            done = 0 as uint8_t;
            i = 0 as uint16_t;
            mask = 1 as uint32_t;
            while (i as ::core::ffi::c_int) < ec_data_parts as ::core::ffi::c_int
                && (j as uint32_t) < servcnt
            {
                if ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                    if mask & vcmask8 == 0 as uint32_t {
                        if chunk_replicate(
                            SPLIT as ::core::ffi::c_int as uint8_t,
                            now,
                            c,
                            ec_data_parts,
                            (i as ::core::ffi::c_int + 0x20 as ::core::ffi::c_int) as uint8_t,
                            preferedsrccsid,
                            *servers.offset(j as isize),
                            NULL,
                            ::core::ptr::null_mut::<uint16_t>(),
                            ::core::ptr::null_mut::<uint8_t>(),
                            SPLIT_EC_GENERIC as ::core::ffi::c_int as uint8_t,
                        ) as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int
                        {
                            job_exit_reasons[(*c).sclassid as usize]
                                [ERROR_CREATING_MISSING_EC8_PART_FROM_COPY as ::core::ffi::c_int
                                    as usize] = job_exit_reasons[(*c).sclassid as usize]
                                [ERROR_CREATING_MISSING_EC8_PART_FROM_COPY as ::core::ffi::c_int
                                    as usize]
                                .wrapping_add(1);
                            return;
                        }
                        done = 1 as uint8_t;
                        inforec.split_copy_into_ecparts =
                            inforec.split_copy_into_ecparts.wrapping_add(1);
                        j = j.wrapping_add(1);
                    }
                } else if mask & vcmask4 == 0 as uint32_t {
                    if chunk_replicate(
                        SPLIT as ::core::ffi::c_int as uint8_t,
                        now,
                        c,
                        ec_data_parts,
                        (i as ::core::ffi::c_int + 0x10 as ::core::ffi::c_int) as uint8_t,
                        preferedsrccsid,
                        *servers.offset(j as isize),
                        NULL,
                        ::core::ptr::null_mut::<uint16_t>(),
                        ::core::ptr::null_mut::<uint8_t>(),
                        SPLIT_EC_GENERIC as ::core::ffi::c_int as uint8_t,
                    ) as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int
                    {
                        job_exit_reasons[(*c).sclassid as usize]
                            [ERROR_CREATING_MISSING_EC4_PART_FROM_COPY as ::core::ffi::c_int
                                as usize] = job_exit_reasons[(*c).sclassid as usize]
                            [ERROR_CREATING_MISSING_EC4_PART_FROM_COPY as ::core::ffi::c_int
                                as usize]
                            .wrapping_add(1);
                        return;
                    }
                    done = 2 as uint8_t;
                    inforec.split_copy_into_ecparts =
                        inforec.split_copy_into_ecparts.wrapping_add(1);
                    j = j.wrapping_add(1);
                }
                i = i.wrapping_add(1);
                mask <<= 1 as ::core::ffi::c_int;
            }
            if j as uint32_t == servcnt
                && rservallflag as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                chunk_priority_enqueue(chunk_priority, c);
            }
            match done as ::core::ffi::c_int {
                1 => {
                    job_exit_reasons[(*c).sclassid as usize]
                        [CREATED_MISSING_EC8_PART_FROM_COPY as ::core::ffi::c_int as usize] =
                        job_exit_reasons[(*c).sclassid as usize]
                            [CREATED_MISSING_EC8_PART_FROM_COPY as ::core::ffi::c_int as usize]
                            .wrapping_add(1);
                }
                2 => {
                    job_exit_reasons[(*c).sclassid as usize]
                        [CREATED_MISSING_EC4_PART_FROM_COPY as ::core::ffi::c_int as usize] =
                        job_exit_reasons[(*c).sclassid as usize]
                            [CREATED_MISSING_EC4_PART_FROM_COPY as ::core::ffi::c_int as usize]
                            .wrapping_add(1);
                }
                _ => {
                    job_exit_reasons[(*c).sclassid as usize]
                        [CANT_CREATE_PART_FROM_COPY as ::core::ffi::c_int as usize] =
                        job_exit_reasons[(*c).sclassid as usize]
                            [CANT_CREATE_PART_FROM_COPY as ::core::ffi::c_int as usize]
                            .wrapping_add(1);
                }
            }
            return;
        }
        if ec_data_parts as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && vc as ::core::ffi::c_int + tdc as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (survivorsmask8 == 0xff as uint32_t || survivorsmask4 == 0xf as uint32_t)
        {
            rservcount = matocsserv_getservers_lessrepl(
                rcsids as *mut uint16_t,
                repl_limit_write,
                (if chunk_priority as ::core::ffi::c_int <= CHUNK_PRIORITY_ONECOPY_ANY {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as uint8_t,
                &raw mut rservallflag,
            );
            done = 0 as uint8_t;
            if rservcount as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                if (*sm).has_labels as ::core::ffi::c_int != 0
                    || DoNotUseSameIP as ::core::ffi::c_int != 0
                    || DoNotUseSameRack as ::core::ffi::c_int != 0
                {
                    let mut reps: uint32_t = 0 as uint32_t;
                    servcnt = 0 as uint32_t;
                    i = 0 as uint16_t;
                    while (i as ::core::ffi::c_int) < rservcount as ::core::ffi::c_int {
                        s = (*c).slisthead;
                        while !s.is_null()
                            && ((*s).csid as ::core::ffi::c_int
                                != *rcsids.offset(i as isize) as ::core::ffi::c_int
                                || (*s).ecid as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                        {
                            s = (*s).next as *mut slist;
                        }
                        if s.is_null() {
                            let c2rust_fresh12 = servcnt;
                            servcnt = servcnt.wrapping_add(1);
                            *servers.offset(c2rust_fresh12 as isize) = *rcsids.offset(i as isize);
                        }
                        i = i.wrapping_add(1);
                    }
                    matching = do_advanced_match(sm, servcnt, servers);
                    i = 0 as uint16_t;
                    while (i as uint32_t) < servcnt {
                        if *matching.offset(
                            (i as ::core::ffi::c_int + (*sm).labelscnt as ::core::ffi::c_int)
                                as isize,
                        ) >= 0 as int32_t
                        {
                            if survivorsmask4 == 0xf as uint32_t {
                                if chunk_replicate(
                                    JOIN as ::core::ffi::c_int as uint8_t,
                                    now,
                                    c,
                                    4 as uint8_t,
                                    0 as uint8_t,
                                    0 as uint16_t,
                                    *servers.offset(i as isize),
                                    NULL,
                                    &raw mut survivorscsid4 as *mut uint16_t,
                                    &raw mut survivorsecid4 as *mut uint8_t,
                                    (if extrajob as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                                        JOIN_EC_IO as ::core::ffi::c_int
                                    } else {
                                        (if usekeep as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                                        {
                                            JOIN_EC_CHANGE as ::core::ffi::c_int
                                        } else {
                                            (if usekeep as ::core::ffi::c_int
                                                == 2 as ::core::ffi::c_int
                                            {
                                                JOIN_EC_NOSERVERS as ::core::ffi::c_int
                                            } else {
                                                JOIN_EC_GENERIC as ::core::ffi::c_int
                                            })
                                        })
                                    }) as uint8_t,
                                ) as ::core::ffi::c_int
                                    != 0 as ::core::ffi::c_int
                                {
                                    job_exit_reasons[(*c).sclassid as usize]
                                        [ERROR_CREATING_LABELED_COPY_FROM_EC4_PARTS
                                            as ::core::ffi::c_int
                                            as usize] = job_exit_reasons[(*c).sclassid as usize]
                                        [ERROR_CREATING_LABELED_COPY_FROM_EC4_PARTS
                                            as ::core::ffi::c_int
                                            as usize]
                                        .wrapping_add(1);
                                    return;
                                }
                                done = 1 as uint8_t;
                            } else {
                                if chunk_replicate(
                                    JOIN as ::core::ffi::c_int as uint8_t,
                                    now,
                                    c,
                                    8 as uint8_t,
                                    0 as uint8_t,
                                    0 as uint16_t,
                                    *servers.offset(i as isize),
                                    NULL,
                                    &raw mut survivorscsid8 as *mut uint16_t,
                                    &raw mut survivorsecid8 as *mut uint8_t,
                                    (if extrajob as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                                        JOIN_EC_IO as ::core::ffi::c_int
                                    } else {
                                        (if usekeep as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                                        {
                                            JOIN_EC_CHANGE as ::core::ffi::c_int
                                        } else {
                                            (if usekeep as ::core::ffi::c_int
                                                == 2 as ::core::ffi::c_int
                                            {
                                                JOIN_EC_NOSERVERS as ::core::ffi::c_int
                                            } else {
                                                JOIN_EC_GENERIC as ::core::ffi::c_int
                                            })
                                        })
                                    }) as uint8_t,
                                ) as ::core::ffi::c_int
                                    != 0 as ::core::ffi::c_int
                                {
                                    job_exit_reasons[(*c).sclassid as usize]
                                        [ERROR_CREATING_LABELED_COPY_FROM_EC8_PARTS
                                            as ::core::ffi::c_int
                                            as usize] = job_exit_reasons[(*c).sclassid as usize]
                                        [ERROR_CREATING_LABELED_COPY_FROM_EC8_PARTS
                                            as ::core::ffi::c_int
                                            as usize]
                                        .wrapping_add(1);
                                    return;
                                }
                                done = 2 as uint8_t;
                            }
                            inforec.join_ecparts_into_copy =
                                inforec.join_ecparts_into_copy.wrapping_add(1);
                            reps = reps.wrapping_add(1);
                        }
                        i = i.wrapping_add(1);
                    }
                    if reps < (*sm).labelscnt as uint32_t {
                        chunk_priority_enqueue(CHUNK_PRIORITY_UNDERGOAL as uint8_t, c);
                    }
                } else {
                    let mut reps_0: uint32_t = 0 as uint32_t;
                    i = 0 as uint16_t;
                    while (i as ::core::ffi::c_int) < rservcount as ::core::ffi::c_int
                        && reps_0 < goal as uint32_t
                    {
                        s = (*c).slisthead;
                        while !s.is_null()
                            && ((*s).csid as ::core::ffi::c_int
                                != *rcsids.offset(i as isize) as ::core::ffi::c_int
                                || (*s).ecid as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                        {
                            s = (*s).next as *mut slist;
                        }
                        if s.is_null() {
                            if survivorsmask4 == 0xf as uint32_t {
                                if chunk_replicate(
                                    JOIN as ::core::ffi::c_int as uint8_t,
                                    now,
                                    c,
                                    4 as uint8_t,
                                    0 as uint8_t,
                                    0 as uint16_t,
                                    *rcsids.offset(i as isize),
                                    NULL,
                                    &raw mut survivorscsid4 as *mut uint16_t,
                                    &raw mut survivorsecid4 as *mut uint8_t,
                                    (if extrajob as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                                        JOIN_EC_IO as ::core::ffi::c_int
                                    } else {
                                        (if usekeep as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                                        {
                                            JOIN_EC_CHANGE as ::core::ffi::c_int
                                        } else {
                                            (if usekeep as ::core::ffi::c_int
                                                == 2 as ::core::ffi::c_int
                                            {
                                                JOIN_EC_NOSERVERS as ::core::ffi::c_int
                                            } else {
                                                JOIN_EC_GENERIC as ::core::ffi::c_int
                                            })
                                        })
                                    }) as uint8_t,
                                ) as ::core::ffi::c_int
                                    != 0 as ::core::ffi::c_int
                                {
                                    job_exit_reasons[(*c).sclassid as usize]
                                        [ERROR_CREATING_COPY_FROM_EC4_PARTS as ::core::ffi::c_int
                                            as usize] = job_exit_reasons[(*c).sclassid as usize]
                                        [ERROR_CREATING_COPY_FROM_EC4_PARTS as ::core::ffi::c_int
                                            as usize]
                                        .wrapping_add(1);
                                    return;
                                }
                                done = 3 as uint8_t;
                            } else {
                                if chunk_replicate(
                                    JOIN as ::core::ffi::c_int as uint8_t,
                                    now,
                                    c,
                                    8 as uint8_t,
                                    0 as uint8_t,
                                    0 as uint16_t,
                                    *rcsids.offset(i as isize),
                                    NULL,
                                    &raw mut survivorscsid8 as *mut uint16_t,
                                    &raw mut survivorsecid8 as *mut uint8_t,
                                    (if extrajob as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                                        JOIN_EC_IO as ::core::ffi::c_int
                                    } else {
                                        (if usekeep as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                                        {
                                            JOIN_EC_CHANGE as ::core::ffi::c_int
                                        } else {
                                            (if usekeep as ::core::ffi::c_int
                                                == 2 as ::core::ffi::c_int
                                            {
                                                JOIN_EC_NOSERVERS as ::core::ffi::c_int
                                            } else {
                                                JOIN_EC_GENERIC as ::core::ffi::c_int
                                            })
                                        })
                                    }) as uint8_t,
                                ) as ::core::ffi::c_int
                                    != 0 as ::core::ffi::c_int
                                {
                                    job_exit_reasons[(*c).sclassid as usize]
                                        [ERROR_CREATING_COPY_FROM_EC8_PARTS as ::core::ffi::c_int
                                            as usize] = job_exit_reasons[(*c).sclassid as usize]
                                        [ERROR_CREATING_COPY_FROM_EC8_PARTS as ::core::ffi::c_int
                                            as usize]
                                        .wrapping_add(1);
                                    return;
                                }
                                done = 4 as uint8_t;
                            }
                            inforec.join_ecparts_into_copy =
                                inforec.join_ecparts_into_copy.wrapping_add(1);
                            reps_0 = reps_0.wrapping_add(1);
                        }
                        i = i.wrapping_add(1);
                    }
                    if reps_0 < goal as uint32_t {
                        chunk_priority_enqueue(CHUNK_PRIORITY_UNDERGOAL as uint8_t, c);
                    }
                }
            } else {
                chunk_priority_enqueue(CHUNK_PRIORITY_UNDERGOAL as uint8_t, c);
            }
            match done as ::core::ffi::c_int {
                1 => {
                    job_exit_reasons[(*c).sclassid as usize]
                        [CREATED_LABELED_COPY_FROM_EC4_PARTS as ::core::ffi::c_int as usize] =
                        job_exit_reasons[(*c).sclassid as usize]
                            [CREATED_LABELED_COPY_FROM_EC4_PARTS as ::core::ffi::c_int as usize]
                            .wrapping_add(1);
                }
                2 => {
                    job_exit_reasons[(*c).sclassid as usize]
                        [CREATED_LABELED_COPY_FROM_EC8_PARTS as ::core::ffi::c_int as usize] =
                        job_exit_reasons[(*c).sclassid as usize]
                            [CREATED_LABELED_COPY_FROM_EC8_PARTS as ::core::ffi::c_int as usize]
                            .wrapping_add(1);
                }
                3 => {
                    job_exit_reasons[(*c).sclassid as usize]
                        [CREATED_COPY_FROM_EC4_PARTS as ::core::ffi::c_int as usize] =
                        job_exit_reasons[(*c).sclassid as usize]
                            [CREATED_COPY_FROM_EC4_PARTS as ::core::ffi::c_int as usize]
                            .wrapping_add(1);
                }
                4 => {
                    job_exit_reasons[(*c).sclassid as usize]
                        [CREATED_COPY_FROM_EC8_PARTS as ::core::ffi::c_int as usize] =
                        job_exit_reasons[(*c).sclassid as usize]
                            [CREATED_COPY_FROM_EC8_PARTS as ::core::ffi::c_int as usize]
                            .wrapping_add(1);
                }
                _ => {
                    job_exit_reasons[(*c).sclassid as usize]
                        [CANT_CREATE_COPY_FROM_PARTS as ::core::ffi::c_int as usize] =
                        job_exit_reasons[(*c).sclassid as usize]
                            [CANT_CREATE_COPY_FROM_PARTS as ::core::ffi::c_int as usize]
                            .wrapping_add(1);
                }
            }
            return;
        }
        if dontdelete as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && ec_data_parts as ::core::ffi::c_int != 0
            && vc as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && regularecgoalequiv >= goal as uint32_t
        {
            enqueue = 0 as uint8_t;
            s = (*c).slisthead;
            while !s.is_null() {
                if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    if (matocsserv_deletion_counter((*cstab.offset((*s).csid as isize)).ptr)
                        as uint32_t)
                        < TmpMaxDel
                    {
                        if matocsserv_send_deletechunk(
                            (*cstab.offset((*s).csid as isize)).ptr,
                            (*c).chunkid,
                            (*s).ecid,
                            (*c).version() as uint32_t,
                            OP_DEL_OVERGOAL as ::core::ffi::c_int as uint8_t,
                        ) >= 0 as ::core::ffi::c_int
                        {
                            (*s).valid = DEL as ::core::ffi::c_int as uint8_t;
                            stats_chunkops[CHUNK_OP_DELETE_TRY as usize] =
                                stats_chunkops[CHUNK_OP_DELETE_TRY as usize].wrapping_add(1);
                            inforec.delete_excess_copy = inforec.delete_excess_copy.wrapping_add(1);
                            deldone = deldone.wrapping_add(1);
                        } else {
                            enqueue = 1 as uint8_t;
                            delnotdone = delnotdone.wrapping_add(1);
                        }
                    } else {
                        enqueue = 1 as uint8_t;
                        delnotdone = delnotdone.wrapping_add(1);
                    }
                }
                s = (*s).next as *mut slist;
            }
            if enqueue != 0 {
                chunk_priority_enqueue(chunk_priority, c);
                job_exit_reasons[(*c).sclassid as usize]
                    [CANT_DELETE_COPIES_IN_EC_MODE as ::core::ffi::c_int as usize] =
                    job_exit_reasons[(*c).sclassid as usize]
                        [CANT_DELETE_COPIES_IN_EC_MODE as ::core::ffi::c_int as usize]
                        .wrapping_add(1);
            } else {
                job_exit_reasons[(*c).sclassid as usize]
                    [DELETED_COPIES_IN_EC_MODE as ::core::ffi::c_int as usize] = job_exit_reasons
                    [(*c).sclassid as usize]
                    [DELETED_COPIES_IN_EC_MODE as ::core::ffi::c_int as usize]
                    .wrapping_add(1);
            }
            return;
        }
        if dontdelete as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && ec_data_parts as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && vcmask8 | vcmask4 != 0 as uint32_t
            && vc as ::core::ffi::c_int >= goal as ::core::ffi::c_int
        {
            enqueue = 0 as uint8_t;
            s = (*c).slisthead;
            while !s.is_null() {
                if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    if (matocsserv_deletion_counter((*cstab.offset((*s).csid as isize)).ptr)
                        as uint32_t)
                        < TmpMaxDel
                    {
                        if matocsserv_send_deletechunk(
                            (*cstab.offset((*s).csid as isize)).ptr,
                            (*c).chunkid,
                            (*s).ecid,
                            (*c).version() as uint32_t,
                            OP_DEL_OVERGOAL as ::core::ffi::c_int as uint8_t,
                        ) >= 0 as ::core::ffi::c_int
                        {
                            (*s).valid = DEL as ::core::ffi::c_int as uint8_t;
                            stats_chunkops[CHUNK_OP_DELETE_TRY as usize] =
                                stats_chunkops[CHUNK_OP_DELETE_TRY as usize].wrapping_add(1);
                            inforec.delete_excess_ecpart =
                                inforec.delete_excess_ecpart.wrapping_add(1);
                            deldone = deldone.wrapping_add(1);
                        } else {
                            enqueue = 1 as uint8_t;
                            delnotdone = delnotdone.wrapping_add(1);
                        }
                    } else {
                        enqueue = 1 as uint8_t;
                        delnotdone = delnotdone.wrapping_add(1);
                    }
                }
                s = (*s).next as *mut slist;
            }
            if enqueue != 0 {
                chunk_priority_enqueue(chunk_priority, c);
                job_exit_reasons[(*c).sclassid as usize]
                    [CANT_DELETE_PARTS_IN_COPY_MODE as ::core::ffi::c_int as usize] =
                    job_exit_reasons[(*c).sclassid as usize]
                        [CANT_DELETE_PARTS_IN_COPY_MODE as ::core::ffi::c_int as usize]
                        .wrapping_add(1);
            } else {
                job_exit_reasons[(*c).sclassid as usize]
                    [DELETED_PARTS_IN_COPY_MODE as ::core::ffi::c_int as usize] = job_exit_reasons
                    [(*c).sclassid as usize]
                    [DELETED_PARTS_IN_COPY_MODE as ::core::ffi::c_int as usize]
                    .wrapping_add(1);
            }
            return;
        }
        if ec_data_parts as ::core::ffi::c_int != 0 && regularecgoalequiv > goal as uint32_t {
            enqueue = 0 as uint8_t;
            s = (*c).slisthead;
            while !s.is_null() {
                if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int
                        >= minecid as ::core::ffi::c_int
                            + (ec_data_parts as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            + goal as ::core::ffi::c_int
                {
                    if (matocsserv_deletion_counter((*cstab.offset((*s).csid as isize)).ptr)
                        as uint32_t)
                        < TmpMaxDel
                    {
                        if matocsserv_send_deletechunk(
                            (*cstab.offset((*s).csid as isize)).ptr,
                            (*c).chunkid,
                            (*s).ecid,
                            0 as uint32_t,
                            OP_DEL_OVERGOAL as ::core::ffi::c_int as uint8_t,
                        ) >= 0 as ::core::ffi::c_int
                        {
                            (*s).valid = DEL as ::core::ffi::c_int as uint8_t;
                            stats_chunkops[CHUNK_OP_DELETE_TRY as usize] =
                                stats_chunkops[CHUNK_OP_DELETE_TRY as usize].wrapping_add(1);
                            inforec.delete_excess_ecpart =
                                inforec.delete_excess_ecpart.wrapping_add(1);
                            deldone = deldone.wrapping_add(1);
                        } else {
                            enqueue = 1 as uint8_t;
                            delnotdone = delnotdone.wrapping_add(1);
                        }
                    } else {
                        enqueue = 1 as uint8_t;
                        delnotdone = delnotdone.wrapping_add(1);
                    }
                }
                s = (*s).next as *mut slist;
            }
            if enqueue != 0 {
                chunk_priority_enqueue(CHUNK_PRIORITY_OVERGOAL as uint8_t, c);
                job_exit_reasons[(*c).sclassid as usize]
                    [CANT_DELETE_EXTRA_EC_PARTS as ::core::ffi::c_int as usize] = job_exit_reasons
                    [(*c).sclassid as usize]
                    [CANT_DELETE_EXTRA_EC_PARTS as ::core::ffi::c_int as usize]
                    .wrapping_add(1);
            } else {
                job_exit_reasons[(*c).sclassid as usize]
                    [DELETED_EXTRA_EC_PARTS as ::core::ffi::c_int as usize] = job_exit_reasons
                    [(*c).sclassid as usize]
                    [DELETED_EXTRA_EC_PARTS as ::core::ffi::c_int as usize]
                    .wrapping_add(1);
            }
            return;
        }
        if ec_data_parts as ::core::ffi::c_int != 0
            && chunk_priority as ::core::ffi::c_int == CHUNK_PRIORITY_WRONGLABELS
        {
            wlmask = 0 as uint32_t;
            if (*sm).labelscnt as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                let mut checkindex_0: uint8_t = 0;
                if (*sm).labelscnt as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    checkindex_0 = 0 as uint8_t;
                } else {
                    checkindex_0 = 1 as uint8_t;
                }
                s = (*c).slisthead;
                while !s.is_null() {
                    if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int >= minecid as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= maxecid as ::core::ffi::c_int
                    {
                        if ((*s).ecid as ::core::ffi::c_int & ecidmask as ::core::ffi::c_int)
                            < ec_data_parts as ::core::ffi::c_int
                        {
                            if matocsserv_server_matches_labelexpr(
                                (*cstab.offset((*s).csid as isize)).ptr,
                                &raw mut *(&raw mut (*sm).labelexpr as *mut [uint8_t; 128])
                                    .offset(0 as isize)
                                    as *mut uint8_t
                                    as *const uint8_t,
                            ) as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                wlmask = (wlmask as ::core::ffi::c_uint
                                    | (1 as ::core::ffi::c_uint)
                                        << ((*s).ecid as ::core::ffi::c_int
                                            & ecidmask as ::core::ffi::c_int))
                                    as uint32_t;
                            }
                        } else if matocsserv_server_matches_labelexpr(
                            (*cstab.offset((*s).csid as isize)).ptr,
                            &raw mut *(&raw mut (*sm).labelexpr as *mut [uint8_t; 128])
                                .offset(checkindex_0 as isize)
                                as *mut uint8_t as *const uint8_t,
                        ) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            wlmask = (wlmask as ::core::ffi::c_uint
                                | (1 as ::core::ffi::c_uint)
                                    << ((*s).ecid as ::core::ffi::c_int
                                        & ecidmask as ::core::ffi::c_int))
                                as uint32_t;
                        }
                    }
                    s = (*s).next as *mut slist;
                }
                if wlmask != 0 as uint32_t {
                    done = 0 as uint8_t;
                    rservcount = matocsserv_getservers_lessrepl(
                        rcsids as *mut uint16_t,
                        repl_limit_write,
                        (if chunk_priority as ::core::ffi::c_int <= CHUNK_PRIORITY_ONECOPY_ANY {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }) as uint8_t,
                        &raw mut rservallflag,
                    );
                    if ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int
                        && wlmask & 0xff as uint32_t != 0
                        || ec_data_parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int
                            && wlmask & 0xf as uint32_t != 0
                    {
                        servcnt = 0 as uint32_t;
                        i = 0 as uint16_t;
                        while (i as ::core::ffi::c_int) < rservcount as ::core::ffi::c_int {
                            s = (*c).slisthead;
                            while !s.is_null()
                                && ((*s).csid as ::core::ffi::c_int
                                    != *rcsids.offset(i as isize) as ::core::ffi::c_int
                                    || ((*s).ecid as ::core::ffi::c_int)
                                        < minecid as ::core::ffi::c_int
                                    || (*s).ecid as ::core::ffi::c_int
                                        > maxecid as ::core::ffi::c_int)
                            {
                                s = (*s).next as *mut slist;
                            }
                            if s.is_null() {
                                if matocsserv_server_matches_labelexpr(
                                    (*cstab.offset(*rcsids.offset(i as isize) as isize)).ptr,
                                    &raw mut *(&raw mut (*sm).labelexpr as *mut [uint8_t; 128])
                                        .offset(0 as isize)
                                        as *mut uint8_t
                                        as *const uint8_t,
                                ) != 0
                                {
                                    let c2rust_fresh13 = servcnt;
                                    servcnt = servcnt.wrapping_add(1);
                                    *servers.offset(c2rust_fresh13 as isize) =
                                        *rcsids.offset(i as isize);
                                }
                            }
                            i = i.wrapping_add(1);
                        }
                        j = 0 as uint16_t;
                        i = 0 as uint16_t;
                        mask = 1 as uint32_t;
                        while (i as ::core::ffi::c_int) < ec_data_parts as ::core::ffi::c_int
                            && (j as uint32_t) < servcnt
                        {
                            if wlmask & mask != 0 {
                                srccsid = MAXCSCOUNT as uint16_t;
                                s = (*c).slisthead;
                                while !s.is_null() && srccsid as ::core::ffi::c_int == MAXCSCOUNT {
                                    if matocsserv_replication_read_counter(
                                        (*cstab.offset((*s).csid as isize)).ptr,
                                        now,
                                    ) < repl_limit_read
                                        && (*s).valid as ::core::ffi::c_int
                                            == VALID as ::core::ffi::c_int
                                        && (*s).ecid as ::core::ffi::c_int
                                            == minecid as ::core::ffi::c_int
                                                + i as ::core::ffi::c_int
                                    {
                                        srccsid = (*s).csid;
                                    }
                                    s = (*s).next as *mut slist;
                                }
                                if srccsid as ::core::ffi::c_int != MAXCSCOUNT {
                                    if chunk_replicate(
                                        SIMPLE as ::core::ffi::c_int as uint8_t,
                                        now,
                                        c,
                                        ec_data_parts,
                                        (i as ::core::ffi::c_int + minecid as ::core::ffi::c_int)
                                            as uint8_t,
                                        srccsid,
                                        *servers.offset(j as isize),
                                        NULL,
                                        ::core::ptr::null_mut::<uint16_t>(),
                                        ::core::ptr::null_mut::<uint8_t>(),
                                        REPL_EC_WRONGLABEL as ::core::ffi::c_int as uint8_t,
                                    ) as ::core::ffi::c_int
                                        != 0 as ::core::ffi::c_int
                                    {
                                        job_exit_reasons[(*c).sclassid as usize]
                                            [ERROR_REPLICATING_WRONG_LABELED_DATA_PART
                                                as ::core::ffi::c_int
                                                as usize] = job_exit_reasons
                                            [(*c).sclassid as usize]
                                            [ERROR_REPLICATING_WRONG_LABELED_DATA_PART
                                                as ::core::ffi::c_int
                                                as usize]
                                            .wrapping_add(1);
                                        return;
                                    }
                                    done = (done as ::core::ffi::c_int | 1 as ::core::ffi::c_int)
                                        as uint8_t;
                                    inforec.replicate_wronglabels_ecpart =
                                        inforec.replicate_wronglabels_ecpart.wrapping_add(1);
                                    j = j.wrapping_add(1);
                                }
                            }
                            i = i.wrapping_add(1);
                            mask <<= 1 as ::core::ffi::c_int;
                        }
                    }
                    if ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int
                        && wlmask & 0xffffff00 as uint32_t != 0
                        || ec_data_parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int
                            && wlmask & 0xfffffff0 as uint32_t != 0
                    {
                        servcnt = 0 as uint32_t;
                        i = 0 as uint16_t;
                        while (i as ::core::ffi::c_int) < rservcount as ::core::ffi::c_int {
                            s = (*c).slisthead;
                            while !s.is_null()
                                && ((*s).csid as ::core::ffi::c_int
                                    != *rcsids.offset(i as isize) as ::core::ffi::c_int
                                    || ((*s).ecid as ::core::ffi::c_int)
                                        < minecid as ::core::ffi::c_int
                                    || (*s).ecid as ::core::ffi::c_int
                                        > maxecid as ::core::ffi::c_int)
                            {
                                s = (*s).next as *mut slist;
                            }
                            if s.is_null() {
                                if matocsserv_server_matches_labelexpr(
                                    (*cstab.offset(*rcsids.offset(i as isize) as isize)).ptr,
                                    &raw mut *(&raw mut (*sm).labelexpr as *mut [uint8_t; 128])
                                        .offset(
                                            (if (*sm).labelscnt as ::core::ffi::c_int
                                                == 1 as ::core::ffi::c_int
                                            {
                                                0 as ::core::ffi::c_int
                                            } else {
                                                1 as ::core::ffi::c_int
                                            }) as isize,
                                        ) as *mut uint8_t
                                        as *const uint8_t,
                                ) != 0
                                {
                                    let c2rust_fresh14 = servcnt;
                                    servcnt = servcnt.wrapping_add(1);
                                    *servers.offset(c2rust_fresh14 as isize) =
                                        *rcsids.offset(i as isize);
                                }
                            }
                            i = i.wrapping_add(1);
                        }
                        j = 0 as uint16_t;
                        i = ec_data_parts as uint16_t;
                        mask = ((1 as ::core::ffi::c_uint) << ec_data_parts as ::core::ffi::c_int)
                            as uint32_t;
                        while (i as ::core::ffi::c_int)
                            < ec_data_parts as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                + goal as ::core::ffi::c_int
                            && (j as uint32_t) < servcnt
                        {
                            if wlmask & mask != 0 {
                                srccsid = MAXCSCOUNT as uint16_t;
                                s = (*c).slisthead;
                                while !s.is_null() && srccsid as ::core::ffi::c_int == MAXCSCOUNT {
                                    if matocsserv_replication_read_counter(
                                        (*cstab.offset((*s).csid as isize)).ptr,
                                        now,
                                    ) < repl_limit_read
                                        && (*s).valid as ::core::ffi::c_int
                                            == VALID as ::core::ffi::c_int
                                        && (*s).ecid as ::core::ffi::c_int
                                            == minecid as ::core::ffi::c_int
                                                + i as ::core::ffi::c_int
                                    {
                                        srccsid = (*s).csid;
                                    }
                                    s = (*s).next as *mut slist;
                                }
                                if srccsid as ::core::ffi::c_int != MAXCSCOUNT {
                                    if chunk_replicate(
                                        SIMPLE as ::core::ffi::c_int as uint8_t,
                                        now,
                                        c,
                                        ec_data_parts,
                                        (i as ::core::ffi::c_int + minecid as ::core::ffi::c_int)
                                            as uint8_t,
                                        srccsid,
                                        *servers.offset(j as isize),
                                        NULL,
                                        ::core::ptr::null_mut::<uint16_t>(),
                                        ::core::ptr::null_mut::<uint8_t>(),
                                        REPL_EC_WRONGLABEL as ::core::ffi::c_int as uint8_t,
                                    ) as ::core::ffi::c_int
                                        != 0 as ::core::ffi::c_int
                                    {
                                        job_exit_reasons[(*c).sclassid as usize]
                                            [ERROR_REPLICATING_WRONG_LABELED_CHKSUM_PART
                                                as ::core::ffi::c_int
                                                as usize] = job_exit_reasons
                                            [(*c).sclassid as usize]
                                            [ERROR_REPLICATING_WRONG_LABELED_CHKSUM_PART
                                                as ::core::ffi::c_int
                                                as usize]
                                            .wrapping_add(1);
                                        return;
                                    }
                                    done = (done as ::core::ffi::c_int | 2 as ::core::ffi::c_int)
                                        as uint8_t;
                                    inforec.replicate_wronglabels_ecpart =
                                        inforec.replicate_wronglabels_ecpart.wrapping_add(1);
                                    j = j.wrapping_add(1);
                                }
                            }
                            i = i.wrapping_add(1);
                            mask <<= 1 as ::core::ffi::c_int;
                        }
                    }
                    chunk_priority_enqueue(chunk_priority, c);
                    match done as ::core::ffi::c_int {
                        0 => {
                            job_exit_reasons[(*c).sclassid as usize]
                                [CANT_REPLICATE_WRONG_LABELED_PART as ::core::ffi::c_int
                                    as usize] = job_exit_reasons[(*c).sclassid as usize]
                                [CANT_REPLICATE_WRONG_LABELED_PART as ::core::ffi::c_int as usize]
                                .wrapping_add(1);
                        }
                        1 => {
                            job_exit_reasons[(*c).sclassid as usize]
                                [REPLICATED_WRONG_LABELED_DATA_PART as ::core::ffi::c_int
                                    as usize] = job_exit_reasons[(*c).sclassid as usize]
                                [REPLICATED_WRONG_LABELED_DATA_PART as ::core::ffi::c_int as usize]
                                .wrapping_add(1);
                        }
                        2 => {
                            job_exit_reasons[(*c).sclassid as usize]
                                [REPLICATED_WRONG_LABELED_CHKSUM_PART as ::core::ffi::c_int
                                    as usize] = job_exit_reasons[(*c).sclassid as usize]
                                [REPLICATED_WRONG_LABELED_CHKSUM_PART as ::core::ffi::c_int
                                    as usize]
                                .wrapping_add(1);
                        }
                        _ => {
                            job_exit_reasons[(*c).sclassid as usize]
                                [REPLICATED_WRONG_LABELED_PARTS as ::core::ffi::c_int as usize] =
                                job_exit_reasons[(*c).sclassid as usize]
                                    [REPLICATED_WRONG_LABELED_PARTS as ::core::ffi::c_int as usize]
                                    .wrapping_add(1);
                        }
                    }
                    return;
                }
            }
        }
        if ec_data_parts as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && vc as ::core::ffi::c_int > goal as ::core::ffi::c_int
        {
            let mut prevdone_0: uint8_t = 0;
            let mut delcnt: uint8_t = 0;
            if dservcount as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                dservcount = matocsserv_getservers_ordered(dcsids as *mut uint16_t);
            }
            delnotdone = delnotdone
                .wrapping_add((vc as ::core::ffi::c_int - goal as ::core::ffi::c_int) as uint32_t);
            prevdone_0 = 1 as uint8_t;
            if (*sm).has_labels as ::core::ffi::c_int != 0
                || DoNotUseSameIP as ::core::ffi::c_int != 0
                || DoNotUseSameRack as ::core::ffi::c_int != 0
            {
                servcnt = 0 as uint32_t;
                i = 0 as uint16_t;
                while (i as ::core::ffi::c_int) < dservcount as ::core::ffi::c_int {
                    s = (*c).slisthead;
                    while !s.is_null() {
                        if (*s).csid as ::core::ffi::c_int
                            == *dcsids.offset(i as isize) as ::core::ffi::c_int
                            && (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            && (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                        {
                            let c2rust_fresh15 = servcnt;
                            servcnt = servcnt.wrapping_add(1);
                            *servers.offset(c2rust_fresh15 as isize) = (*s).csid;
                            break;
                        } else {
                            s = (*s).next as *mut slist;
                        }
                    }
                    i = i.wrapping_add(1);
                }
                matching = do_advanced_match(sm, servcnt, servers);
                do_extend_match(sm, servcnt, matching);
                delcnt = (vc as ::core::ffi::c_int - goal as ::core::ffi::c_int) as uint8_t;
                i = 0 as uint16_t;
                while (i as uint32_t) < servcnt
                    && delcnt as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                    && prevdone_0 as ::core::ffi::c_int != 0
                {
                    if *matching.offset(
                        (i as ::core::ffi::c_int + (*sm).labelscnt as ::core::ffi::c_int) as isize,
                    ) < 0 as int32_t
                    {
                        s = (*c).slisthead;
                        while !s.is_null()
                            && ((*s).csid as ::core::ffi::c_int
                                != *servers.offset(i as isize) as ::core::ffi::c_int
                                || (*s).ecid as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                        {
                            s = (*s).next as *mut slist;
                        }
                        if !s.is_null()
                            && (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                        {
                            if (matocsserv_deletion_counter((*cstab.offset((*s).csid as isize)).ptr)
                                as uint32_t)
                                < TmpMaxDel
                            {
                                if matocsserv_send_deletechunk(
                                    (*cstab.offset((*s).csid as isize)).ptr,
                                    (*c).chunkid,
                                    (*s).ecid,
                                    (*c).version() as uint32_t,
                                    OP_DEL_OVERGOAL as ::core::ffi::c_int as uint8_t,
                                ) >= 0 as ::core::ffi::c_int
                                {
                                    (*s).valid = DEL as ::core::ffi::c_int as uint8_t;
                                    stats_chunkops[CHUNK_OP_DELETE_TRY as usize] = stats_chunkops
                                        [CHUNK_OP_DELETE_TRY as usize]
                                        .wrapping_add(1);
                                    inforec.delete_excess_copy =
                                        inforec.delete_excess_copy.wrapping_add(1);
                                    deldone = deldone.wrapping_add(1);
                                    delnotdone = delnotdone.wrapping_sub(1);
                                    delcnt = delcnt.wrapping_sub(1);
                                } else {
                                    prevdone_0 = 0 as uint8_t;
                                    chunk_priority_enqueue(CHUNK_PRIORITY_OVERGOAL as uint8_t, c);
                                }
                            } else {
                                prevdone_0 = 0 as uint8_t;
                                chunk_priority_enqueue(CHUNK_PRIORITY_OVERGOAL as uint8_t, c);
                            }
                        }
                    }
                    i = i.wrapping_add(1);
                }
            } else {
                delcnt = (vc as ::core::ffi::c_int - goal as ::core::ffi::c_int) as uint8_t;
                i = 0 as uint16_t;
                while (i as ::core::ffi::c_int) < dservcount as ::core::ffi::c_int
                    && delcnt as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                    && prevdone_0 as ::core::ffi::c_int != 0
                {
                    s = (*c).slisthead;
                    while !s.is_null()
                        && ((*s).csid as ::core::ffi::c_int
                            != *dcsids.offset(
                                (dservcount as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int
                                    - i as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int
                            || (*s).ecid as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                    {
                        s = (*s).next as *mut slist;
                    }
                    if !s.is_null()
                        && (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                    {
                        if (matocsserv_deletion_counter((*cstab.offset((*s).csid as isize)).ptr)
                            as uint32_t)
                            < TmpMaxDel
                        {
                            if matocsserv_send_deletechunk(
                                (*cstab.offset((*s).csid as isize)).ptr,
                                (*c).chunkid,
                                (*s).ecid,
                                (*c).version() as uint32_t,
                                OP_DEL_OVERGOAL as ::core::ffi::c_int as uint8_t,
                            ) >= 0 as ::core::ffi::c_int
                            {
                                (*c).set_needverincrease(
                                    1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                );
                                (*s).valid = DEL as ::core::ffi::c_int as uint8_t;
                                stats_chunkops[CHUNK_OP_DELETE_TRY as usize] =
                                    stats_chunkops[CHUNK_OP_DELETE_TRY as usize].wrapping_add(1);
                                inforec.delete_excess_copy =
                                    inforec.delete_excess_copy.wrapping_add(1);
                                deldone = deldone.wrapping_add(1);
                                delnotdone = delnotdone.wrapping_sub(1);
                                delcnt = delcnt.wrapping_sub(1);
                            } else {
                                prevdone_0 = 0 as uint8_t;
                                chunk_priority_enqueue(CHUNK_PRIORITY_OVERGOAL as uint8_t, c);
                            }
                        } else {
                            prevdone_0 = 0 as uint8_t;
                            chunk_priority_enqueue(CHUNK_PRIORITY_OVERGOAL as uint8_t, c);
                        }
                    }
                    i = i.wrapping_add(1);
                }
            }
            if prevdone_0 != 0 {
                job_exit_reasons[(*c).sclassid as usize]
                    [DELETED_EXTRA_COPIES as ::core::ffi::c_int as usize] = job_exit_reasons
                    [(*c).sclassid as usize]
                    [DELETED_EXTRA_COPIES as ::core::ffi::c_int as usize]
                    .wrapping_add(1);
            } else {
                job_exit_reasons[(*c).sclassid as usize]
                    [CANT_DELETE_EXTRA_COPIES as ::core::ffi::c_int as usize] = job_exit_reasons
                    [(*c).sclassid as usize]
                    [CANT_DELETE_EXTRA_COPIES as ::core::ffi::c_int as usize]
                    .wrapping_add(1);
            }
            return;
        }
        if ec_data_parts as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && vc as ::core::ffi::c_int + tdc as ::core::ffi::c_int >= scount as ::core::ffi::c_int
            && (vc as ::core::ffi::c_int) < goal as ::core::ffi::c_int
            && tdc as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && vc as ::core::ffi::c_int + tdc as ::core::ffi::c_int > 1 as ::core::ffi::c_int
            && chunk_priority_is_empty(CHUNK_PRIORITY_ONECOPY_HIGHGOAL as uint8_t)
                as ::core::ffi::c_int
                != 0
            && chunk_priority_is_empty(CHUNK_PRIORITY_ONECOPY_ANY as uint8_t) as ::core::ffi::c_int
                != 0
        {
            let mut tdcr: uint8_t = 0 as uint8_t;
            s = (*c).slisthead;
            while !s.is_null() {
                if (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    if matocsserv_has_avail_space((*cstab.offset((*s).csid as isize)).ptr) != 0 {
                        tdcr = tdcr.wrapping_add(1);
                    }
                }
                s = (*s).next as *mut slist;
            }
            if vc as ::core::ffi::c_int + tdcr as ::core::ffi::c_int >= scount as ::core::ffi::c_int
            {
                s = (*c).slisthead;
                while !s.is_null() {
                    if (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        if matocsserv_has_avail_space((*cstab.offset((*s).csid as isize)).ptr)
                            as ::core::ffi::c_int
                            != 0
                            && (matocsserv_deletion_counter((*cstab.offset((*s).csid as isize)).ptr)
                                as uint32_t)
                                < TmpMaxDel
                        {
                            if matocsserv_send_deletechunk(
                                (*cstab.offset((*s).csid as isize)).ptr,
                                (*c).chunkid,
                                (*s).ecid,
                                (*c).version() as uint32_t,
                                OP_DEL_OVERGOAL as ::core::ffi::c_int as uint8_t,
                            ) >= 0 as ::core::ffi::c_int
                            {
                                (*c).set_needverincrease(
                                    1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                );
                                (*s).valid = DEL as ::core::ffi::c_int as uint8_t;
                                stats_chunkops[CHUNK_OP_DELETE_TRY as usize] =
                                    stats_chunkops[CHUNK_OP_DELETE_TRY as usize].wrapping_add(1);
                                inforec.delete_diskclean_copy =
                                    inforec.delete_diskclean_copy.wrapping_add(1);
                                job_exit_reasons[(*c).sclassid as usize]
                                    [DELETED_MFR_COPY_TO_MAKE_SPACE as ::core::ffi::c_int
                                        as usize] = job_exit_reasons[(*c).sclassid as usize]
                                    [DELETED_MFR_COPY_TO_MAKE_SPACE as ::core::ffi::c_int as usize]
                                    .wrapping_add(1);
                                return;
                            }
                        }
                    }
                    s = (*s).next as *mut slist;
                }
            }
        }
        if ec_data_parts as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && (chunk_priority as ::core::ffi::c_int == CHUNK_PRIORITY_WRONGLABELS
                || goal as ::core::ffi::c_int > vc as ::core::ffi::c_int)
            && vc as ::core::ffi::c_int + tdc as ::core::ffi::c_int > 0 as ::core::ffi::c_int
        {
            let mut rgvc: uint32_t = 0;
            let mut rgtdc: uint32_t = 0;
            let mut servmaxpos: [uint16_t; 4] = [0; 4];
            let mut canbefixed: uint8_t = 0;
            canbefixed = 0 as uint8_t;
            matocsserv_get_server_groups(
                rcsids as *mut uint16_t,
                repl_limit_write,
                &raw mut servmaxpos as *mut uint16_t,
            );
            if if chunk_priority as ::core::ffi::c_int <= CHUNK_PRIORITY_ONECOPY_ANY {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } != 0
            {
                rservcount = servmaxpos[CSSTATE_OVERLOADED as usize];
            } else {
                rservcount = servmaxpos[CSSTATE_OK as usize];
            }
            rgvc = 0 as uint32_t;
            rgtdc = 0 as uint32_t;
            vripcnt = 0 as uint8_t;
            s = (*c).slisthead;
            while !s.is_null() {
                if (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && matocsserv_replication_read_counter(
                        (*cstab.offset((*s).csid as isize)).ptr,
                        now,
                    ) < repl_limit_read
                {
                    if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int {
                        if (vripcnt as ::core::ffi::c_int) < 255 as ::core::ffi::c_int
                            && ReplicationsRespectTopology as ::core::ffi::c_int
                                > 1 as ::core::ffi::c_int
                        {
                            let c2rust_fresh16 = vripcnt;
                            vripcnt = vripcnt.wrapping_add(1);
                            vrip[c2rust_fresh16 as usize] =
                                matocsserv_server_get_ip((*cstab.offset((*s).csid as isize)).ptr);
                        }
                        rgvc = rgvc.wrapping_add(1);
                    } else if (*s).valid as ::core::ffi::c_int == TDVALID as ::core::ffi::c_int {
                        if (vripcnt as ::core::ffi::c_int) < 255 as ::core::ffi::c_int
                            && ReplicationsRespectTopology as ::core::ffi::c_int
                                > 1 as ::core::ffi::c_int
                        {
                            let c2rust_fresh17 = vripcnt;
                            vripcnt = vripcnt.wrapping_add(1);
                            vrip[c2rust_fresh17 as usize] =
                                matocsserv_server_get_ip((*cstab.offset((*s).csid as isize)).ptr);
                        }
                        rgtdc = rgtdc.wrapping_add(1);
                    }
                }
                s = (*s).next as *mut slist;
            }
            if ReplicationsRespectTopology as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                chunk_rack_sort(
                    rcsids as *mut uint16_t,
                    rservcount,
                    &raw mut vrip as *mut uint32_t,
                    vripcnt,
                );
            }
            if rgvc.wrapping_add(rgtdc) > 0 as uint32_t
                && rservcount as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            {
                if (*sm).has_labels as ::core::ffi::c_int != 0
                    || DoNotUseSameIP as ::core::ffi::c_int != 0
                    || DoNotUseSameRack as ::core::ffi::c_int != 0
                {
                    let mut maxstdservers: uint16_t = 0;
                    let mut dstservcnt: uint16_t = 0;
                    servcnt = 0 as uint32_t;
                    s = (*c).slisthead;
                    while !s.is_null() {
                        if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                            && (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        {
                            let c2rust_fresh18 = servcnt;
                            servcnt = servcnt.wrapping_add(1);
                            *servers.offset(c2rust_fresh18 as isize) = (*s).csid;
                        }
                        s = (*s).next as *mut slist;
                    }
                    dstservcnt = servcnt as uint16_t;
                    i = 0 as uint16_t;
                    while (i as ::core::ffi::c_int) < rservcount as ::core::ffi::c_int {
                        s = (*c).slisthead;
                        while !s.is_null()
                            && ((*s).csid as ::core::ffi::c_int
                                != *rcsids.offset(i as isize) as ::core::ffi::c_int
                                || (*s).ecid as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                        {
                            s = (*s).next as *mut slist;
                        }
                        if s.is_null() {
                            let c2rust_fresh19 = servcnt;
                            servcnt = servcnt.wrapping_add(1);
                            *servers.offset(c2rust_fresh19 as isize) = *rcsids.offset(i as isize);
                        }
                        i = i.wrapping_add(1);
                    }
                    maxstdservers = servcnt as uint16_t;
                    i = rservcount;
                    while (i as ::core::ffi::c_int)
                        < servmaxpos[CSSTATE_OVERLOADED as usize] as ::core::ffi::c_int
                    {
                        s = (*c).slisthead;
                        while !s.is_null()
                            && ((*s).csid as ::core::ffi::c_int
                                != *rcsids.offset(i as isize) as ::core::ffi::c_int
                                || (*s).ecid as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                        {
                            s = (*s).next as *mut slist;
                        }
                        if s.is_null() {
                            let c2rust_fresh20 = servcnt;
                            servcnt = servcnt.wrapping_add(1);
                            *servers.offset(c2rust_fresh20 as isize) = *rcsids.offset(i as isize);
                        }
                        i = i.wrapping_add(1);
                    }
                    i = servmaxpos[CSSTATE_OVERLOADED as usize];
                    while (i as ::core::ffi::c_int)
                        < servmaxpos[CSSTATE_LIMIT_REACHED as usize] as ::core::ffi::c_int
                    {
                        s = (*c).slisthead;
                        while !s.is_null()
                            && (*s).csid as ::core::ffi::c_int
                                != *rcsids.offset(i as isize) as ::core::ffi::c_int
                        {
                            s = (*s).next as *mut slist;
                        }
                        if s.is_null() {
                            let c2rust_fresh21 = servcnt;
                            servcnt = servcnt.wrapping_add(1);
                            *servers.offset(c2rust_fresh21 as isize) = *rcsids.offset(i as isize);
                        }
                        i = i.wrapping_add(1);
                    }
                    matching = do_advanced_match(sm, servcnt, servers);
                    labels_mode = sclass_get_labels_mode((*c).sclassid as uint16_t, sm);
                    i = 0 as uint16_t;
                    while (i as ::core::ffi::c_int) < (*sm).labelscnt as ::core::ffi::c_int {
                        let mut servpos: int32_t = 0;
                        if *matching.offset(i as isize) < (*sm).labelscnt as int32_t {
                            servpos = -1 as ::core::ffi::c_int as int32_t;
                        } else {
                            servpos = *matching.offset(i as isize) - (*sm).labelscnt as int32_t;
                        }
                        if servpos < 0 as int32_t {
                            if labels_mode as ::core::ffi::c_int != LABELS_MODE_STRICT {
                                canbefixed = 1 as uint8_t;
                                j = dstservcnt;
                                while (j as ::core::ffi::c_int)
                                    < maxstdservers as ::core::ffi::c_int
                                {
                                    if *matching.offset(
                                        (j as ::core::ffi::c_int
                                            + (*sm).labelscnt as ::core::ffi::c_int)
                                            as isize,
                                    ) < 0 as int32_t
                                    {
                                        if chunk_undergoal_replicate(
                                            c,
                                            *servers.offset(j as isize),
                                            now,
                                            repl_limit_read,
                                            extrajob,
                                            chunk_priority as uint16_t,
                                            &raw mut inforec,
                                            rgvc,
                                            rgtdc,
                                        ) >= 0 as ::core::ffi::c_int
                                        {
                                            job_exit_reasons[(*c).sclassid as usize]
                                                [REPLICATED_WRONG_LABELED_COPY_UNMATCHED
                                                    as ::core::ffi::c_int
                                                    as usize] = job_exit_reasons
                                                [(*c).sclassid as usize]
                                                [REPLICATED_WRONG_LABELED_COPY_UNMATCHED
                                                    as ::core::ffi::c_int
                                                    as usize]
                                                .wrapping_add(1);
                                            return;
                                        }
                                    }
                                    j = j.wrapping_add(1);
                                }
                            }
                        } else if servpos >= maxstdservers as int32_t {
                            canbefixed = 1 as uint8_t;
                            if labels_mode as ::core::ffi::c_int == LABELS_MODE_LOOSE {
                                j = dstservcnt;
                                while (j as ::core::ffi::c_int)
                                    < maxstdservers as ::core::ffi::c_int
                                {
                                    if *matching.offset(
                                        (j as ::core::ffi::c_int
                                            + (*sm).labelscnt as ::core::ffi::c_int)
                                            as isize,
                                    ) < 0 as int32_t
                                    {
                                        if chunk_undergoal_replicate(
                                            c,
                                            *servers.offset(j as isize),
                                            now,
                                            repl_limit_read,
                                            extrajob,
                                            chunk_priority as uint16_t,
                                            &raw mut inforec,
                                            rgvc,
                                            rgtdc,
                                        ) >= 0 as ::core::ffi::c_int
                                        {
                                            job_exit_reasons[(*c).sclassid as usize]
                                                [REPLICATED_WRONG_LABELED_COPY_BUSY
                                                    as ::core::ffi::c_int
                                                    as usize] = job_exit_reasons
                                                [(*c).sclassid as usize]
                                                [REPLICATED_WRONG_LABELED_COPY_BUSY
                                                    as ::core::ffi::c_int
                                                    as usize]
                                                .wrapping_add(1);
                                            return;
                                        }
                                    }
                                    j = j.wrapping_add(1);
                                }
                            }
                        } else if servpos >= dstservcnt as int32_t {
                            canbefixed = 1 as uint8_t;
                            if chunk_undergoal_replicate(
                                c,
                                *servers.offset(servpos as isize),
                                now,
                                repl_limit_read,
                                extrajob,
                                chunk_priority as uint16_t,
                                &raw mut inforec,
                                rgvc,
                                rgtdc,
                            ) >= 0 as ::core::ffi::c_int
                            {
                                job_exit_reasons[(*c).sclassid as usize]
                                    [REPLICATED_WRONG_LABELED_COPY_GOOD as ::core::ffi::c_int
                                        as usize] = job_exit_reasons[(*c).sclassid as usize]
                                    [REPLICATED_WRONG_LABELED_COPY_GOOD as ::core::ffi::c_int
                                        as usize]
                                    .wrapping_add(1);
                                return;
                            }
                        }
                        i = i.wrapping_add(1);
                    }
                } else if goal as ::core::ffi::c_int
                    <= servmaxpos[CSSTATE_LIMIT_REACHED as usize] as ::core::ffi::c_int
                    || (vc as ::core::ffi::c_int)
                        < servmaxpos[CSSTATE_LIMIT_REACHED as usize] as ::core::ffi::c_int
                {
                    canbefixed = 1 as uint8_t;
                    i = 0 as uint16_t;
                    while (i as ::core::ffi::c_int) < rservcount as ::core::ffi::c_int {
                        s = (*c).slisthead;
                        while !s.is_null()
                            && ((*s).csid as ::core::ffi::c_int
                                != *rcsids.offset(i as isize) as ::core::ffi::c_int
                                || (*s).ecid as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                        {
                            s = (*s).next as *mut slist;
                        }
                        if s.is_null() {
                            if chunk_undergoal_replicate(
                                c,
                                *rcsids.offset(i as isize),
                                now,
                                repl_limit_read,
                                extrajob,
                                chunk_priority as uint16_t,
                                &raw mut inforec,
                                rgvc,
                                rgtdc,
                            ) >= 0 as ::core::ffi::c_int
                            {
                                job_exit_reasons[(*c).sclassid as usize]
                                    [REPLICATED_UNDERGOAL_COPY as ::core::ffi::c_int as usize] =
                                    job_exit_reasons[(*c).sclassid as usize]
                                        [REPLICATED_UNDERGOAL_COPY as ::core::ffi::c_int as usize]
                                        .wrapping_add(1);
                                return;
                            }
                        }
                        i = i.wrapping_add(1);
                    }
                }
            }
            if canbefixed != 0 {
                chunk_priority_enqueue(chunk_priority, c);
            }
            if chunk_priority as ::core::ffi::c_int == CHUNK_PRIORITY_WRONGLABELS {
                job_exit_reasons[(*c).sclassid as usize]
                    [CANT_REPLICATE_WRONG_LABELED_COPY as ::core::ffi::c_int as usize] =
                    job_exit_reasons[(*c).sclassid as usize]
                        [CANT_REPLICATE_WRONG_LABELED_COPY as ::core::ffi::c_int as usize]
                        .wrapping_add(1);
            } else {
                job_exit_reasons[(*c).sclassid as usize]
                    [CANT_REPLICATE_UNDERGOAL_COPY as ::core::ffi::c_int as usize] =
                    job_exit_reasons[(*c).sclassid as usize]
                        [CANT_REPLICATE_UNDERGOAL_COPY as ::core::ffi::c_int as usize]
                        .wrapping_add(1);
            }
            return;
        }
        if extrajob != 0 {
            job_exit_reasons[(*c).sclassid as usize]
                [CANT_REBALANCE_ON_EXTRA_CALL as ::core::ffi::c_int as usize] = job_exit_reasons
                [(*c).sclassid as usize]
                [CANT_REBALANCE_ON_EXTRA_CALL as ::core::ffi::c_int as usize]
                .wrapping_add(1);
            return;
        }
        if fullservers as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut queues_empty: uint8_t = 0;
            queues_empty = 1 as uint8_t;
            i = 0 as uint16_t;
            while (i as ::core::ffi::c_int) < DANGER_PRIORITIES {
                if chunk_priority_not_empty(i as uint8_t) != 0 {
                    if if i as ::core::ffi::c_int != CHUNK_PRIORITY_OVERGOAL {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    } != 0
                    {
                        job_exit_reasons[(*c).sclassid as usize]
                            [REBALANCE_BLOCKED_BY_PRIORITY_QUEUES as ::core::ffi::c_int as usize] =
                            job_exit_reasons[(*c).sclassid as usize]
                                [REBALANCE_BLOCKED_BY_PRIORITY_QUEUES as ::core::ffi::c_int
                                    as usize]
                                .wrapping_add(1);
                        return;
                    } else {
                        queues_empty = 0 as uint8_t;
                    }
                }
                i = i.wrapping_add(1);
            }
            if queues_empty != 0 {
                if (*c).ondangerlist() != 0 {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"chunk %016lX_%08X: fixing 'ondangerlist' flag\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*c).chunkid,
                        (*c).version() as ::core::ffi::c_int,
                    );
                    (*c).set_ondangerlist(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
            }
        }
        if rebalance_fails[(*c).sclassid as usize] as uint32_t >= MaxRebalanceFails
            && MaxRebalanceFails > 0 as uint32_t
            && FailRebalanceCounterResetCalls > 0 as uint32_t
        {
            job_exit_reasons[(*c).sclassid as usize]
                [REBALANCE_BLOCKED_BY_TOO_MANY_FAILS as ::core::ffi::c_int as usize] =
                job_exit_reasons[(*c).sclassid as usize]
                    [REBALANCE_BLOCKED_BY_TOO_MANY_FAILS as ::core::ffi::c_int as usize]
                    .wrapping_add(1);
            return;
        }
        srccsid = MAXCSCOUNT as uint16_t;
        dstcsid = MAXCSCOUNT as uint16_t;
        repecid = 0 as uint8_t;
        maxdiff = 0.0f64;
        matching = ::core::ptr::null_mut::<int32_t>();
        labels_mode = LABELS_MODE_LOOSE as uint8_t;
        servcnt = 0 as uint32_t;
        if ec_data_parts as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
            if goal as uint32_t == regularecgoalequiv
                && overmask8 == 0 as uint32_t
                && vc as ::core::ffi::c_int + tdc as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && vcmask4 | tdcmask4 == 0 as uint32_t
            {
                s = (*c).slisthead;
                while !s.is_null() {
                    if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int >= minecid as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= maxecid as ::core::ffi::c_int
                    {
                        *ecids.offset(servcnt as isize) = (*s).ecid;
                        *servers.offset(servcnt as isize) = (*s).csid;
                        servcnt = servcnt.wrapping_add(1);
                    }
                    s = (*s).next as *mut slist;
                }
                extraservcnt = servcnt;
                s = (*c).slisthead;
                while !s.is_null() {
                    if (*s).valid as ::core::ffi::c_int != VALID as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int >= minecid as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= maxecid as ::core::ffi::c_int
                    {
                        let c2rust_fresh22 = extraservcnt;
                        extraservcnt = extraservcnt.wrapping_add(1);
                        *servers.offset(c2rust_fresh22 as isize) = (*s).csid;
                    }
                    s = (*s).next as *mut slist;
                }
            }
        } else if ec_data_parts as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            if goal as uint32_t == regularecgoalequiv
                && overmask4 == 0 as uint32_t
                && vc as ::core::ffi::c_int + tdc as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && vcmask8 | tdcmask8 == 0 as uint32_t
            {
                s = (*c).slisthead;
                while !s.is_null() {
                    if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int >= minecid as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= maxecid as ::core::ffi::c_int
                    {
                        *ecids.offset(servcnt as isize) = (*s).ecid;
                        *servers.offset(servcnt as isize) = (*s).csid;
                        servcnt = servcnt.wrapping_add(1);
                    }
                    s = (*s).next as *mut slist;
                }
                extraservcnt = servcnt;
                s = (*c).slisthead;
                while !s.is_null() {
                    if (*s).valid as ::core::ffi::c_int != VALID as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int >= minecid as ::core::ffi::c_int
                        && (*s).ecid as ::core::ffi::c_int <= maxecid as ::core::ffi::c_int
                    {
                        let c2rust_fresh23 = extraservcnt;
                        extraservcnt = extraservcnt.wrapping_add(1);
                        *servers.offset(c2rust_fresh23 as isize) = (*s).csid;
                    }
                    s = (*s).next as *mut slist;
                }
            }
        } else if goal as ::core::ffi::c_int == vc as ::core::ffi::c_int
            && vc as ::core::ffi::c_int + tdc as ::core::ffi::c_int > 0 as ::core::ffi::c_int
        {
            s = (*c).slisthead;
            while !s.is_null() {
                if (*s).valid as ::core::ffi::c_int == VALID as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    *ecids.offset(servcnt as isize) = 0 as uint8_t;
                    *servers.offset(servcnt as isize) = (*s).csid;
                    servcnt = servcnt.wrapping_add(1);
                }
                s = (*s).next as *mut slist;
            }
            extraservcnt = servcnt;
            s = (*c).slisthead;
            while !s.is_null() {
                if (*s).valid as ::core::ffi::c_int != VALID as ::core::ffi::c_int
                    && (*s).ecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    let c2rust_fresh24 = extraservcnt;
                    extraservcnt = extraservcnt.wrapping_add(1);
                    *servers.offset(c2rust_fresh24 as isize) = (*s).csid;
                }
                s = (*s).next as *mut slist;
            }
            if (*sm).has_labels != 0 {
                matching = do_advanced_match(sm, servcnt, servers);
                labels_mode = sclass_get_labels_mode((*c).sclassid as uint16_t, sm);
            }
        }
        overloaded = 0 as uint8_t;
        if servcnt > 0 as uint32_t {
            if dservcount as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                dservcount = matocsserv_getservers_ordered(dcsids as *mut uint16_t);
            }
            i = 0 as uint16_t;
            while (i as uint32_t) < servcnt {
                let mut lexpr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
                let mut anyunmatched: uint8_t = 0;
                let mut srcusage: ::core::ffi::c_double = 0.;
                let mut dstusage: ::core::ffi::c_double = 0.;
                let mut lclass: uint8_t = 0;
                anyunmatched = 0 as uint8_t;
                lexpr = ::core::ptr::null_mut::<uint8_t>();
                if ec_data_parts as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    if (*sm).has_labels != 0 {
                        if *matching.offset(
                            ((*sm).labelscnt as ::core::ffi::c_int + i as ::core::ffi::c_int)
                                as isize,
                        ) >= 0 as int32_t
                        {
                            lexpr = &raw mut *(&raw mut (*sm).labelexpr as *mut [uint8_t; 128])
                                .offset(*matching.offset(
                                    ((*sm).labelscnt as ::core::ffi::c_int
                                        + i as ::core::ffi::c_int)
                                        as isize,
                                ) as isize) as *mut uint8_t;
                        } else if labels_mode as ::core::ffi::c_int != LABELS_MODE_LOOSE {
                            anyunmatched = 1 as uint8_t;
                        }
                    }
                } else if (*sm).labelscnt != 0 {
                    lexpr = &raw mut *(&raw mut (*sm).labelexpr as *mut [uint8_t; 128]).offset(
                        (if (*sm).labelscnt as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                            0 as ::core::ffi::c_int
                        } else if (*ecids.offset(i as isize) as ::core::ffi::c_int
                            & ecidmask as ::core::ffi::c_int)
                            < ec_data_parts as ::core::ffi::c_int
                        {
                            0 as ::core::ffi::c_int
                        } else {
                            1 as ::core::ffi::c_int
                        }) as isize,
                    ) as *mut uint8_t;
                }
                srcusage =
                    matocsserv_get_usage((*cstab.offset(*servers.offset(i as isize) as isize)).ptr);
                repl_read_counter = matocsserv_replication_read_counter(
                    (*cstab.offset(*servers.offset(i as isize) as isize)).ptr,
                    now,
                );
                j = 0 as uint16_t;
                while (j as ::core::ffi::c_int) < dservcount as ::core::ffi::c_int {
                    dstusage = matocsserv_get_usage(
                        (*cstab.offset(*dcsids.offset(j as isize) as isize)).ptr,
                    );
                    if srcusage - dstusage < maxdiff {
                        break;
                    }
                    if srcusage - dstusage <= AcceptableDifference
                        && last_rebalance as ::core::ffi::c_double + 0.01f64 / (srcusage - dstusage)
                            >= now as ::core::ffi::c_double
                    {
                        break;
                    }
                    k = 0 as uint16_t;
                    while (k as uint32_t) < extraservcnt {
                        if *servers.offset(k as isize) as ::core::ffi::c_int
                            == *dcsids.offset(j as isize) as ::core::ffi::c_int
                        {
                            break;
                        }
                        if ec_data_parts as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            if DoNotUseSameIP != 0 {
                                if matocsserv_server_get_ip(
                                    (*cstab.offset(*servers.offset(k as isize) as isize)).ptr,
                                ) == matocsserv_server_get_ip(
                                    (*cstab.offset(*dcsids.offset(j as isize) as isize)).ptr,
                                ) {
                                    break;
                                }
                            } else if DoNotUseSameRack != 0 {
                                if topology_get_rackid(matocsserv_server_get_ip(
                                    (*cstab.offset(*servers.offset(k as isize) as isize)).ptr,
                                )) == topology_get_rackid(matocsserv_server_get_ip(
                                    (*cstab.offset(*dcsids.offset(j as isize) as isize)).ptr,
                                )) {
                                    break;
                                }
                            } else if (*sm).uniqmask & UNIQ_MASK_IP as uint32_t != 0 {
                                if matocsserv_server_get_ip(
                                    (*cstab.offset(*servers.offset(k as isize) as isize)).ptr,
                                ) == matocsserv_server_get_ip(
                                    (*cstab.offset(*dcsids.offset(j as isize) as isize)).ptr,
                                ) {
                                    break;
                                }
                            } else if (*sm).uniqmask & UNIQ_MASK_RACK as uint32_t != 0 {
                                if topology_get_rackid(matocsserv_server_get_ip(
                                    (*cstab.offset(*servers.offset(k as isize) as isize)).ptr,
                                )) == topology_get_rackid(matocsserv_server_get_ip(
                                    (*cstab.offset(*dcsids.offset(j as isize) as isize)).ptr,
                                )) {
                                    break;
                                }
                            } else if (*sm).uniqmask != 0 {
                                if matocsserv_server_get_labelmask(
                                    (*cstab.offset(*servers.offset(k as isize) as isize)).ptr,
                                ) & (*sm).uniqmask
                                    == matocsserv_server_get_labelmask(
                                        (*cstab.offset(*dcsids.offset(j as isize) as isize)).ptr,
                                    ) & (*sm).uniqmask
                                {
                                    break;
                                }
                            }
                        }
                        k = k.wrapping_add(1);
                    }
                    's_8270: {
                        if (k as uint32_t) >= extraservcnt {
                            if anyunmatched != 0 {
                                k = 0 as uint16_t;
                                while (k as ::core::ffi::c_int)
                                    < (*sm).labelscnt as ::core::ffi::c_int
                                {
                                    if *matching.offset(k as isize) < 0 as int32_t {
                                        if matocsserv_server_matches_labelexpr(
                                            (*cstab.offset(*dcsids.offset(j as isize) as isize))
                                                .ptr,
                                            &raw mut *(&raw mut (*sm).labelexpr
                                                as *mut [uint8_t; 128])
                                                .offset(k as isize)
                                                as *mut uint8_t
                                                as *const uint8_t,
                                        ) != 0
                                        {
                                            break;
                                        }
                                    }
                                    k = k.wrapping_add(1);
                                }
                                if k as ::core::ffi::c_int == (*sm).labelscnt as ::core::ffi::c_int
                                {
                                    break 's_8270;
                                }
                            }
                            if lexpr.is_null()
                                || matocsserv_server_matches_labelexpr(
                                    (*cstab.offset(*dcsids.offset(j as isize) as isize)).ptr,
                                    lexpr as *const uint8_t,
                                ) as ::core::ffi::c_int
                                    != 0
                            {
                                if srcusage - dstusage > AcceptableDifference * 1.5f64 {
                                    lclass = 3 as uint8_t;
                                } else {
                                    lclass = 2 as uint8_t;
                                }
                                if repl_read_counter < MaxReadRepl[lclass as usize]
                                    && matocsserv_replication_write_counter(
                                        (*cstab.offset(*dcsids.offset(j as isize) as isize)).ptr,
                                        now,
                                    ) < MaxWriteRepl[lclass as usize]
                                {
                                    maxdiff = srcusage - dstusage;
                                    repecid = *ecids.offset(i as isize);
                                    dstcsid = *dcsids.offset(j as isize);
                                    srccsid = *servers.offset(i as isize);
                                } else {
                                    overloaded = 1 as uint8_t;
                                }
                            }
                        }
                    }
                    j = j.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
        }
        if dstcsid as ::core::ffi::c_int != MAXCSCOUNT
            && srccsid as ::core::ffi::c_int != MAXCSCOUNT
        {
            chunk_delay_protect((*c).chunkid);
            stats_chunkops[CHUNK_OP_REPLICATE_TRY as usize] =
                stats_chunkops[CHUNK_OP_REPLICATE_TRY as usize].wrapping_add(1);
            chunk_replicate(
                SIMPLE as ::core::ffi::c_int as uint8_t,
                now,
                c,
                ec_data_parts,
                repecid,
                srccsid,
                dstcsid,
                NULL,
                ::core::ptr::null_mut::<uint16_t>(),
                ::core::ptr::null_mut::<uint8_t>(),
                (if repecid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    REPL_COPY_REBALANCE as ::core::ffi::c_int
                } else {
                    REPL_EC_REBALANCE as ::core::ffi::c_int
                }) as uint8_t,
            );
            inforec.replicate_rebalance = inforec.replicate_rebalance.wrapping_add(1);
            last_rebalance = now;
            rebalance_fails[(*c).sclassid as usize] = 0 as uint16_t;
            job_exit_reasons[(*c).sclassid as usize]
                [REBALANCE_DONE as ::core::ffi::c_int as usize] = job_exit_reasons
                [(*c).sclassid as usize][REBALANCE_DONE as ::core::ffi::c_int as usize]
                .wrapping_add(1);
        } else {
            if overloaded != 0 {
                rebalance_fails[(*c).sclassid as usize] =
                    rebalance_fails[(*c).sclassid as usize].wrapping_add(1);
            }
            job_exit_reasons[(*c).sclassid as usize]
                [REBALANCE_NOT_DONE_OR_NOT_NEEDED as ::core::ffi::c_int as usize] =
                job_exit_reasons[(*c).sclassid as usize]
                    [REBALANCE_NOT_DONE_OR_NOT_NEEDED as ::core::ffi::c_int as usize]
                    .wrapping_add(1);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_labelset_fix_matching_servers(mut sm: *mut storagemode) {
    unsafe {
        let mut servcsids: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
        let mut servcscnt: uint16_t = 0;
        let mut matching: *mut int32_t = ::core::ptr::null_mut::<int32_t>();
        let mut i: uint32_t = 0;
        if sm.is_null() {
            chunk_replallowed_servers(::core::ptr::null_mut::<uint16_t>(), 1 as uint8_t);
            return;
        }
        if (*sm).ec_data_chksum_parts != 0 {
            matocsserv_recalculate_storagemode_scounts(sm);
            (*sm).matching_servers = 0 as uint8_t;
        } else {
            servcsids = chunk_replallowed_servers(&raw mut servcscnt, 0 as uint8_t);
            matching = do_advanced_match(sm, servcscnt as uint32_t, servcsids);
            (*sm).matching_servers = 0 as uint8_t;
            i = 0 as uint32_t;
            while i < (*sm).labelscnt as uint32_t {
                if *matching.offset(i as isize) >= 0 as int32_t {
                    (*sm).matching_servers = (*sm).matching_servers.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
            (*sm).replallowed = 0xffff as uint16_t;
            (*sm).allvalid = 0xffff as uint16_t;
            (*sm).data_replallowed = 0xffff as uint16_t;
            (*sm).data_allvalid = 0xffff as uint16_t;
            (*sm).chksum_replallowed = 0xffff as uint16_t;
            (*sm).chksum_allvalid = 0xffff as uint16_t;
            (*sm).both_replallowed = 0xffff as uint16_t;
            (*sm).both_allvalid = 0xffff as uint16_t;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_labelset_can_be_fulfilled(mut sm: *mut storagemode) -> uint8_t {
    unsafe {
        static mut stdcsids: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
        static mut olcsids: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
        static mut allcsids: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
        static mut stdcscnt: uint16_t = 0;
        static mut olcscnt: uint16_t = 0;
        static mut allcscnt: uint16_t = 0;
        let mut r: uint8_t = 0;
        let mut i: uint32_t = 0;
        let mut matching: *mut int32_t = ::core::ptr::null_mut::<int32_t>();
        if stdcsids.is_null() {
            stdcsids = malloc(::core::mem::size_of::<uint16_t>().wrapping_mul(MAXCSCOUNT as size_t))
                as *mut uint16_t;
            if stdcsids.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    8392 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"stdcsids\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    8392 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"stdcsids\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if stdcsids
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint16_t
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    8392 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"stdcsids\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    8392 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"stdcsids\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
        }
        if olcsids.is_null() {
            olcsids = malloc(::core::mem::size_of::<uint16_t>().wrapping_mul(MAXCSCOUNT as size_t))
                as *mut uint16_t;
            if olcsids.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    8396 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"olcsids\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    8396 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"olcsids\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if olcsids
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint16_t
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    8396 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"olcsids\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    8396 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"olcsids\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
        }
        if allcsids.is_null() {
            allcsids = malloc(::core::mem::size_of::<uint16_t>().wrapping_mul(MAXCSCOUNT as size_t))
                as *mut uint16_t;
            if allcsids.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    8400 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"allcsids\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    8400 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"allcsids\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if allcsids
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint16_t
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    8400 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"allcsids\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    8400 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"allcsids\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_1,
                );
                abort();
            }
        }
        if sm.is_null() {
            matocsserv_getservers_test(
                &raw mut stdcscnt,
                stdcsids as *mut uint16_t,
                &raw mut olcscnt,
                olcsids as *mut uint16_t,
                &raw mut allcscnt,
                allcsids as *mut uint16_t,
            );
            return CBF_NO as ::core::ffi::c_int as uint8_t;
        }
        if (*sm).ec_data_chksum_parts != 0 {
            let mut data_parts: uint8_t = 0;
            let mut chksum_parts: uint8_t = 0;
            data_parts = ((*sm).ec_data_chksum_parts as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int) as uint8_t;
            chksum_parts = ((*sm).ec_data_chksum_parts as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int) as uint8_t;
            if (*sm).has_labels as ::core::ffi::c_int != 0
                && (*sm).labelscnt as ::core::ffi::c_int != 0
            {
                if ((*sm).valid_ec_counters as ::core::ffi::c_int)
                    < (*sm).labelscnt as ::core::ffi::c_int
                {
                    matocsserv_recalculate_storagemode_scounts(sm);
                }
                if (*sm).labelscnt as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    if ((*sm).allvalid as ::core::ffi::c_int)
                        < data_parts as ::core::ffi::c_int + chksum_parts as ::core::ffi::c_int
                    {
                        return CBF_NO as ::core::ffi::c_int as uint8_t;
                    }
                    if ((*sm).allvalid as ::core::ffi::c_int)
                        < data_parts as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * chksum_parts as ::core::ffi::c_int
                    {
                        return CBF_ECKEEP as ::core::ffi::c_int as uint8_t;
                    }
                    if ((*sm).overloaded as ::core::ffi::c_int)
                        < data_parts as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * chksum_parts as ::core::ffi::c_int
                    {
                        return CBF_NOSPACE as ::core::ffi::c_int as uint8_t;
                    }
                    if ((*sm).replallowed as ::core::ffi::c_int)
                        < data_parts as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * chksum_parts as ::core::ffi::c_int
                    {
                        return CBF_OVERLOADED as ::core::ffi::c_int as uint8_t;
                    }
                } else {
                    if ((*sm).allvalid as ::core::ffi::c_int)
                        < data_parts as ::core::ffi::c_int + chksum_parts as ::core::ffi::c_int
                        || ((*sm).data_allvalid as ::core::ffi::c_int
                            + (*sm).both_allvalid as ::core::ffi::c_int)
                            < data_parts as ::core::ffi::c_int
                        || ((*sm).chksum_allvalid as ::core::ffi::c_int
                            + (*sm).both_allvalid as ::core::ffi::c_int)
                            < chksum_parts as ::core::ffi::c_int
                    {
                        return CBF_NO as ::core::ffi::c_int as uint8_t;
                    }
                    if ((*sm).allvalid as ::core::ffi::c_int)
                        < data_parts as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * chksum_parts as ::core::ffi::c_int
                        || ((*sm).data_allvalid as ::core::ffi::c_int
                            + (*sm).both_allvalid as ::core::ffi::c_int)
                            < data_parts as ::core::ffi::c_int + chksum_parts as ::core::ffi::c_int
                        || ((*sm).chksum_allvalid as ::core::ffi::c_int
                            + (*sm).both_allvalid as ::core::ffi::c_int)
                            < 2 as ::core::ffi::c_int * chksum_parts as ::core::ffi::c_int
                    {
                        return CBF_ECKEEP as ::core::ffi::c_int as uint8_t;
                    }
                    if ((*sm).overloaded as ::core::ffi::c_int)
                        < data_parts as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * chksum_parts as ::core::ffi::c_int
                        || ((*sm).data_overloaded as ::core::ffi::c_int
                            + (*sm).both_overloaded as ::core::ffi::c_int)
                            < data_parts as ::core::ffi::c_int + chksum_parts as ::core::ffi::c_int
                        || ((*sm).chksum_overloaded as ::core::ffi::c_int
                            + (*sm).both_overloaded as ::core::ffi::c_int)
                            < 2 as ::core::ffi::c_int * chksum_parts as ::core::ffi::c_int
                    {
                        return CBF_NOSPACE as ::core::ffi::c_int as uint8_t;
                    }
                    if ((*sm).replallowed as ::core::ffi::c_int)
                        < data_parts as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * chksum_parts as ::core::ffi::c_int
                        || ((*sm).data_replallowed as ::core::ffi::c_int
                            + (*sm).both_replallowed as ::core::ffi::c_int)
                            < data_parts as ::core::ffi::c_int + chksum_parts as ::core::ffi::c_int
                        || ((*sm).chksum_replallowed as ::core::ffi::c_int
                            + (*sm).both_replallowed as ::core::ffi::c_int)
                            < 2 as ::core::ffi::c_int * chksum_parts as ::core::ffi::c_int
                    {
                        return CBF_OVERLOADED as ::core::ffi::c_int as uint8_t;
                    }
                }
            } else {
                if (allcscnt as ::core::ffi::c_int)
                    < data_parts as ::core::ffi::c_int + chksum_parts as ::core::ffi::c_int
                {
                    return CBF_NO as ::core::ffi::c_int as uint8_t;
                }
                if (allcscnt as ::core::ffi::c_int)
                    < data_parts as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * chksum_parts as ::core::ffi::c_int
                {
                    return CBF_ECKEEP as ::core::ffi::c_int as uint8_t;
                }
                if (olcscnt as ::core::ffi::c_int)
                    < data_parts as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * chksum_parts as ::core::ffi::c_int
                {
                    return CBF_NOSPACE as ::core::ffi::c_int as uint8_t;
                }
                if (stdcscnt as ::core::ffi::c_int)
                    < data_parts as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * chksum_parts as ::core::ffi::c_int
                {
                    return CBF_OVERLOADED as ::core::ffi::c_int as uint8_t;
                }
            }
            return CBF_YES as ::core::ffi::c_int as uint8_t;
        }
        matching = do_advanced_match(sm, stdcscnt as uint32_t, stdcsids);
        r = 1 as uint8_t;
        i = 0 as uint32_t;
        while i < (*sm).labelscnt as uint32_t {
            if *matching.offset(i as isize) < 0 as int32_t {
                r = 0 as uint8_t;
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        if r as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            return CBF_YES as ::core::ffi::c_int as uint8_t;
        }
        if olcscnt as ::core::ffi::c_int > stdcscnt as ::core::ffi::c_int {
            matching = do_advanced_match(sm, olcscnt as uint32_t, olcsids);
            r = 1 as uint8_t;
            i = 0 as uint32_t;
            while i < (*sm).labelscnt as uint32_t {
                if *matching.offset(i as isize) < 0 as int32_t {
                    r = 0 as uint8_t;
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
            if r as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                return CBF_OVERLOADED as ::core::ffi::c_int as uint8_t;
            }
        }
        if allcscnt as ::core::ffi::c_int > olcscnt as ::core::ffi::c_int {
            matching = do_advanced_match(sm, allcscnt as uint32_t, allcsids);
            r = 1 as uint8_t;
            i = 0 as uint32_t;
            while i < (*sm).labelscnt as uint32_t {
                if *matching.offset(i as isize) < 0 as int32_t {
                    r = 0 as uint8_t;
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
            if r as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                return CBF_NOSPACE as ::core::ffi::c_int as uint8_t;
            }
        }
        return CBF_NO as ::core::ffi::c_int as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_unlock(
    mut ts: uint32_t,
    mut chunkid: uint64_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        c = chunk_find(chunkid);
        if c.is_null() {
            return MFS_ERROR_NOCHUNK;
        }
        (*c).lockedto = ts.wrapping_sub(1 as uint32_t);
        chunk_write_counters(c, 0 as uint8_t);
        chunk_priority_queue_check(c, 1 as uint8_t);
        if (*c).ondangerlist() != 0 {
            chunk_do_jobs(
                c,
                JOBS_CHUNK as ::core::ffi::c_int as uint8_t,
                ts,
                1 as uint8_t,
            );
            chunk_state_fix(c);
        } else {
            matoclserv_chunk_unlocked((*c).chunkid, c as *mut ::core::ffi::c_void);
        }
        return MFS_STATUS_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_mr_unlock(
    mut ts: uint32_t,
    mut chunkid: uint64_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        c = chunk_find(chunkid);
        if c.is_null() {
            return MFS_ERROR_NOCHUNK;
        }
        (*c).lockedto = ts.wrapping_sub(1 as uint32_t);
        chunk_write_counters(c, 0 as uint8_t);
        return MFS_STATUS_OK;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_do_fast_job(
    mut c: *mut chunk,
    mut now: uint32_t,
    mut extrajob: uint8_t,
) {
    unsafe {
        chunk_do_jobs(
            c,
            JOBS_CHUNK as ::core::ffi::c_int as uint8_t,
            now,
            extrajob,
        );
        chunk_state_fix(c);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_do_extra_job(mut chunkid: uint64_t) {
    unsafe {
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        c = chunk_find(chunkid);
        if !c.is_null() {
            chunk_io_ready_check(c);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_jobs_main() {
    unsafe {
        let mut i: uint32_t = 0;
        let mut j: uint32_t = 0;
        let mut k: uint32_t = 0;
        let mut s: uint32_t = 0;
        let mut lc: uint32_t = 0;
        let mut hashsteps: uint32_t = 0;
        let mut csid: uint16_t = 0;
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut cn: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut now: uint32_t = 0;
        static mut jobshpos: uint32_t = 0 as uint32_t;
        static mut jobshstep: uint32_t = 1 as uint32_t;
        static mut jobshcnt: uint32_t = 0 as uint32_t;
        static mut jobshmax: uint32_t = 0 as uint32_t;
        static mut chunkcheckpos: uint32_t = 0 as uint32_t;
        static mut chunkcheckwait: uint32_t = 0 as uint32_t;
        static mut srcreset: uint16_t = 0 as uint16_t;
        static mut sccreset: uint16_t = 0 as uint16_t;
        static mut scmaxug: [uint16_t; 256] = [0; 256];
        static mut scmaxog: [uint16_t; 256] = [0; 256];
        static mut scmaxwl: [uint16_t; 256] = [0; 256];
        static mut scmaxml: [uint16_t; 256] = [0; 256];
        static mut scmax: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
        chunk_server_disconnection_loop();
        if matocsserv_servers_count() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || chunkrehashpos == 0 as uint32_t
        {
            return;
        }
        now = main_time();
        chunk_do_jobs(
            ::core::ptr::null_mut::<chunk>(),
            JOBS_EVERYTICK as ::core::ffi::c_int as uint8_t,
            now,
            0 as uint8_t,
        );
        lc = 0 as uint32_t;
        loop {
            c = chunk_priority_next(0 as uint8_t);
            if !c.is_null() {
                job_call[(*c).sclassid as usize][0 as usize] =
                    job_call[(*c).sclassid as usize][0 as usize].wrapping_add(1);
                chunk_do_jobs(
                    c,
                    JOBS_CHUNK as ::core::ffi::c_int as uint8_t,
                    now,
                    2 as uint8_t,
                );
                if (*c).operation() as ::core::ffi::c_int == NONE as ::core::ffi::c_int
                    && (*c).ondangerlist() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    chunk_io_ready_end(c);
                }
                chunk_state_fix(c);
                lc = lc.wrapping_add(1);
            }
            if !(!c.is_null() && lc < HashCPTMax) {
                break;
            }
        }
        if chunk_counters_in_progress() != 0 {
            return;
        }
        if main_start_time().wrapping_add(ReplicationsDelayInit) > main_time() {
            return;
        }
        sccreset = sccreset.wrapping_add(1);
        if sccreset as uint32_t >= FailClassCounterResetCalls {
            memset(
                &raw mut scmaxug as *mut uint16_t as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (256 as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>()),
            );
            memset(
                &raw mut scmaxog as *mut uint16_t as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (256 as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>()),
            );
            memset(
                &raw mut scmaxwl as *mut uint16_t as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (256 as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>()),
            );
            memset(
                &raw mut scmaxml as *mut uint16_t as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (256 as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>()),
            );
            sccreset = 0 as uint16_t;
        }
        srcreset = srcreset.wrapping_add(1);
        if srcreset as uint32_t >= FailRebalanceCounterResetCalls {
            memset(
                &raw mut rebalance_fails as *mut uint16_t as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (256 as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>()),
            );
            srcreset = 0 as uint16_t;
        }
        j = 1 as uint32_t;
        while j < DANGER_PRIORITIES as uint32_t && lc < HashCPTMax {
            scmax = if j <= CHUNK_PRIORITY_UNDERGOAL as uint32_t {
                &raw mut scmaxug as *mut uint16_t
            } else if j == CHUNK_PRIORITY_OVERGOAL as uint32_t {
                &raw mut scmaxog as *mut uint16_t
            } else {
                &raw mut scmaxwl as *mut uint16_t
            };
            i = chunk_priority_get_elements(j as uint8_t);
            s = 0 as uint32_t;
            k = j.wrapping_add(1 as uint32_t);
            while k < DANGER_PRIORITIES as uint32_t {
                s = s.wrapping_add(chunk_priority_get_elements(k as uint8_t));
                k = k.wrapping_add(1);
            }
            if i > HashCPTMax
                .wrapping_sub(lc)
                .wrapping_mul(3 as uint32_t)
                .wrapping_div(4 as uint32_t)
                && s > 0 as uint32_t
            {
                i = HashCPTMax
                    .wrapping_sub(lc)
                    .wrapping_mul(3 as uint32_t)
                    .wrapping_div(4 as uint32_t);
            }
            loop {
                if i > 0 as uint32_t {
                    c = chunk_priority_next(j as uint8_t);
                    if !c.is_null() {
                        if (*scmax.offset((*c).sclassid as isize) as uint32_t) < MaxFailsPerClass
                            || MaxFailsPerClass == 0 as uint32_t
                            || FailClassCounterResetCalls == 0 as uint32_t
                        {
                            job_call[(*c).sclassid as usize][j as usize] =
                                job_call[(*c).sclassid as usize][j as usize].wrapping_add(1);
                            chunk_do_jobs(
                                c,
                                JOBS_CHUNK as ::core::ffi::c_int as uint8_t,
                                now,
                                1 as uint8_t,
                            );
                            chunk_state_fix(c);
                            if (*c).ondangerlist() != 0 {
                                *scmax.offset((*c).sclassid as isize) =
                                    (*scmax.offset((*c).sclassid as isize)).wrapping_add(1);
                            } else {
                                *scmax.offset((*c).sclassid as isize) = 0 as uint16_t;
                            }
                        } else {
                            job_nocall[(*c).sclassid as usize][j as usize] =
                                job_nocall[(*c).sclassid as usize][j as usize].wrapping_add(1);
                            chunk_priority_enqueue(j as uint8_t, c);
                        }
                        lc = lc.wrapping_add(1);
                    }
                    i = i.wrapping_sub(1);
                } else {
                    c = ::core::ptr::null_mut::<chunk>();
                }
                if !(!c.is_null() && lc < HashCPTMax) {
                    break;
                }
            }
            j = j.wrapping_add(1);
        }
        lc = 0 as uint32_t;
        hashsteps = (1 as uint32_t)
            .wrapping_add(chunkrehashpos.wrapping_div(LoopTimeMin.wrapping_mul(TicksPerSecond)));
        i = 0 as uint32_t;
        while i < hashsteps && lc < HashCPTMax {
            if jobshcnt >= chunkrehashpos {
                chunk_do_jobs(
                    ::core::ptr::null_mut::<chunk>(),
                    JOBS_EVERYLOOP as ::core::ffi::c_int as uint8_t,
                    now,
                    0 as uint8_t,
                );
                jobshpos = 0 as uint32_t;
                jobshcnt = 0 as uint32_t;
                jobshmax = chunkrehashpos;
                jobshstep = jobshstep.wrapping_mul(16 as uint32_t);
                if jobshstep == 0 as uint32_t || jobshstep >= jobshmax {
                    jobshstep = 1 as uint32_t;
                } else if jobshmax & 1 as uint32_t == 0 as uint32_t {
                    jobshmax = jobshmax.wrapping_sub(1);
                }
                csid = csusedhead as uint16_t;
                while (csid as ::core::ffi::c_int) < MAXCSCOUNT {
                    match (*cstab.offset(csid as isize)).mfr_state() as ::core::ffi::c_int {
                        2 | 1 | 4 => {
                            (*cstab.offset(csid as isize)).set_mfr_state(
                                CAN_BE_REMOVED as ::core::ffi::c_int as ::core::ffi::c_uint
                                    as ::core::ffi::c_uint,
                            );
                        }
                        3 => {
                            (*cstab.offset(csid as isize)).set_mfr_state(
                                WAS_IN_PROGRESS as ::core::ffi::c_int as ::core::ffi::c_uint
                                    as ::core::ffi::c_uint,
                            );
                        }
                        _ => {
                            (*cstab.offset(csid as isize)).set_mfr_state(
                                UNKNOWN_SOFT as ::core::ffi::c_int as ::core::ffi::c_uint
                                    as ::core::ffi::c_uint,
                            );
                        }
                    }
                    csid = (*cstab.offset(csid as isize)).next as uint16_t;
                }
            } else {
                c = *chunkhashtab[(jobshpos >> HASHTAB_LOBITS) as usize]
                    .offset((jobshpos & HASHTAB_MASK as uint32_t) as isize);
                while !c.is_null() {
                    cn = (*c).next as *mut chunk;
                    if (*c).lockedto < main_time()
                        && (*c).operation() as ::core::ffi::c_int == NONE as ::core::ffi::c_int
                        && (*c).slisthead.is_null()
                        && (*c).fhead == FLISTNULLINDX as uint32_t
                        && chunk_counters_in_progress() as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        && csdb_have_all_servers() as ::core::ffi::c_int != 0
                    {
                        changelog(
                            b"%u|CHUNKDEL(%lu,%u)\0".as_ptr() as *const ::core::ffi::c_char,
                            main_time(),
                            (*c).chunkid,
                            (*c).version() as ::core::ffi::c_int,
                        );
                        if (*c).ondangerlist() != 0 {
                            chunk_priority_remove(c);
                        }
                        chunk_delete(c);
                    } else {
                        chunk_recalc_sclassid(c);
                        if (scmaxml[(*c).sclassid as usize] as uint32_t) < MaxFailsPerClass
                            || MaxFailsPerClass == 0 as uint32_t
                            || FailClassCounterResetCalls == 0 as uint32_t
                        {
                            job_call[(*c).sclassid as usize][DANGER_PRIORITIES as usize] =
                                job_call[(*c).sclassid as usize][DANGER_PRIORITIES as usize]
                                    .wrapping_add(1);
                            chunk_do_jobs(
                                c,
                                JOBS_CHUNK as ::core::ffi::c_int as uint8_t,
                                now,
                                0 as uint8_t,
                            );
                            chunk_state_fix(c);
                            if (*c).ondangerlist() != 0 {
                                scmaxml[(*c).sclassid as usize] =
                                    scmaxml[(*c).sclassid as usize].wrapping_add(1);
                            } else {
                                scmaxml[(*c).sclassid as usize] = 0 as uint16_t;
                            }
                        } else {
                            job_nocall[(*c).sclassid as usize][DANGER_PRIORITIES as usize] =
                                job_nocall[(*c).sclassid as usize][DANGER_PRIORITIES as usize]
                                    .wrapping_add(1);
                        }
                        lc = lc.wrapping_add(1);
                    }
                    c = cn;
                }
                jobshcnt = jobshcnt.wrapping_add(1);
                if jobshcnt < jobshmax {
                    jobshpos = jobshpos.wrapping_add(jobshstep);
                    jobshpos = jobshpos.wrapping_rem(jobshmax);
                } else {
                    jobshpos = jobshcnt;
                }
            }
            i = i.wrapping_add(1);
        }
        c = *chunkhashtab[(chunkcheckpos >> HASHTAB_LOBITS) as usize]
            .offset((chunkcheckpos & HASHTAB_MASK as uint32_t) as isize);
        if chunkcheckwait > 0 as uint32_t {
            chunkcheckwait = chunkcheckwait.wrapping_sub(1);
            if c.is_null() {
                chunkcheckpos = chunkcheckpos.wrapping_add(1);
            }
        } else {
            c = *chunkhashtab[(chunkcheckpos >> HASHTAB_LOBITS) as usize]
                .offset((chunkcheckpos & HASHTAB_MASK as uint32_t) as isize);
            while !c.is_null() {
                if (*c).lockedto < main_time()
                    && (*c).fhead > FLISTNULLINDX as uint32_t
                    && (*c).ondangerlist() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && (*c).operation() as ::core::ffi::c_int == NONE as ::core::ffi::c_int
                {
                    matocsserv_broadcast_chunk_status((*c).chunkid);
                    chunkcheckwait = chunkcheckwait.wrapping_add(1);
                }
                c = (*c).next as *mut chunk;
            }
            if chunkcheckwait > 0 as uint32_t {
                chunkcheckwait = chunkcheckwait.wrapping_sub(1);
            }
            chunkcheckpos = chunkcheckpos.wrapping_add(1);
        }
        if chunkcheckpos >= chunkrehashpos {
            chunkcheckpos = 0 as uint32_t;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_get_memusage(mut allocated: *mut uint64_t, mut used: *mut uint64_t) {
    unsafe {
        *allocated.offset(0 as isize) =
            ::core::mem::size_of::<*mut chunk>().wrapping_mul(chunkrehashpos as usize) as uint64_t;
        *used.offset(0 as isize) =
            ::core::mem::size_of::<*mut chunk>().wrapping_mul(chunkhashelem as usize) as uint64_t;
        chunk_getusage(
            allocated.offset(1 as ::core::ffi::c_int as isize),
            used.offset(1 as ::core::ffi::c_int as isize),
        );
        slist_getusage(
            allocated.offset(2 as ::core::ffi::c_int as isize),
            used.offset(2 as ::core::ffi::c_int as isize),
        );
        chunk_queue_getusage(
            allocated.offset(3 as ::core::ffi::c_int as isize),
            used.offset(3 as ::core::ffi::c_int as isize),
        );
        io_ready_chunk_getusage(
            allocated.offset(4 as ::core::ffi::c_int as isize),
            used.offset(4 as ::core::ffi::c_int as isize),
        );
        replock_getusage(
            allocated.offset(5 as ::core::ffi::c_int as isize),
            used.offset(5 as ::core::ffi::c_int as isize),
        );
    }
}
pub const CHUNKFSIZE: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const CHUNKMAXPAIRS: ::core::ffi::c_int = 255 as ::core::ffi::c_int + 128 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_is_afterload_needed(mut mver: uint8_t) -> uint8_t {
    unsafe {
        return (if mver as ::core::ffi::c_int >= 0x12 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_load(
    mut fd: *mut bio,
    mut mver: uint8_t,
    mut ignoreflag: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut hdr: [uint8_t; 8] = [0; 8];
        let mut loadbuff: [uint8_t; 18] = [0; 18];
        let mut pairsbuff: [uint8_t; 1533] = [0; 1533];
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut nl: uint8_t = 0;
        let mut r: int32_t = 0;
        let mut recsize: int32_t = 0;
        let mut dynsize: int32_t = 0;
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut lockedto: uint32_t = 0;
        let mut flags: uint8_t = 0;
        let mut pairs: uint16_t = 0;
        let mut sclassid: uint8_t = 0;
        let mut fcount: uint32_t = 0;
        let mut findxptr: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut fl: *mut flist = ::core::ptr::null_mut::<flist>();
        nl = 1 as uint8_t;
        chunks = 0 as uint32_t;
        if bio_read(
            fd,
            &raw mut hdr as *mut uint8_t as *mut ::core::ffi::c_void,
            8 as uint64_t,
        ) != 8 as int64_t
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"chunks: can't read header\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        ptr = &raw mut hdr as *mut uint8_t;
        nextchunkid = get64bit(&raw mut ptr);
        recsize = (if mver as ::core::ffi::c_int == 0x10 as ::core::ffi::c_int {
            16 as ::core::ffi::c_int
        } else if mver as ::core::ffi::c_int == 0x11 as ::core::ffi::c_int {
            17 as ::core::ffi::c_int
        } else {
            CHUNKFSIZE
        }) as int32_t;
        loop {
            r = bio_read(
                fd,
                &raw mut loadbuff as *mut uint8_t as *mut ::core::ffi::c_void,
                recsize as uint64_t,
            ) as int32_t;
            if r != recsize {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"chunks: read error\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            ptr = &raw mut loadbuff as *mut uint8_t;
            chunkid = get64bit(&raw mut ptr);
            version = get32bit(&raw mut ptr);
            lockedto = get32bit(&raw mut ptr);
            if mver as ::core::ffi::c_int == 0x10 as ::core::ffi::c_int {
                flags = 0 as uint8_t;
            } else {
                flags = get8bit(&raw mut ptr);
            }
            if mver as ::core::ffi::c_int <= 0x11 as ::core::ffi::c_int {
                pairs = 0 as uint16_t;
            } else {
                pairs = get8bit(&raw mut ptr) as uint16_t;
            }
            if flags as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                flags = (flags as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int) as uint8_t;
                pairs = (pairs as ::core::ffi::c_int | 0x100 as ::core::ffi::c_int) as uint16_t;
            }
            if pairs as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                dynsize = (4 as ::core::ffi::c_int * pairs as ::core::ffi::c_int) as int32_t;
                if pairs as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    dynsize += 1;
                }
                r = bio_read(
                    fd,
                    &raw mut pairsbuff as *mut uint8_t as *mut ::core::ffi::c_void,
                    dynsize as uint64_t,
                ) as int32_t;
                if r != dynsize {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"chunks: read error\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    return -1 as ::core::ffi::c_int;
                }
            }
            if chunkid > 0 as uint64_t {
                c = chunk_find(chunkid);
                if !c.is_null() {
                    if nl != 0 {
                        fputc('\n' as ::core::ffi::c_int, stderr);
                        nl = 0 as uint8_t;
                    }
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_ERR,
                        b"loading chunk %016lX error: chunk already exists\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        chunkid,
                    );
                    if ignoreflag == 0 as ::core::ffi::c_int {
                        fprintf(
                            stderr,
                            b"use option '-i' to ignore\n\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                } else {
                    c = chunk_new(chunkid);
                    (*c).set_allowreadzeros(
                        (if version & 0x80000000 as uint32_t != 0 {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }) as ::core::ffi::c_uint as ::core::ffi::c_uint,
                    );
                    (*c).set_version(
                        (version & 0x3fffffff as uint32_t) as ::core::ffi::c_uint
                            as ::core::ffi::c_uint,
                    );
                    (*c).lockedto = lockedto;
                    if pairs as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                        ptr = &raw mut pairsbuff as *mut uint8_t;
                        if pairs as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                            findxptr = &raw mut (*c).fhead;
                            while pairs as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                                sclassid = get8bit(&raw mut ptr);
                                fcount = get24bit(&raw mut ptr);
                                *findxptr = flist_alloc();
                                fl = flist_get(*findxptr);
                                (*fl).sclassid = sclassid;
                                (*fl).set_fcount(
                                    fcount as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                );
                                (*fl).nexti = FLISTNULLINDX as uint32_t;
                                findxptr = &raw mut (*fl).nexti;
                                pairs = pairs.wrapping_sub(1);
                            }
                            sclassid = get8bit(&raw mut ptr);
                        } else {
                            sclassid = get8bit(&raw mut ptr);
                            fcount = get24bit(&raw mut ptr);
                            if fcount < FLISTFIRSTINDX as uint32_t {
                                (*c).fhead = fcount;
                            } else {
                                (*c).fhead = flist_alloc();
                                fl = flist_get((*c).fhead);
                                (*fl).sclassid = sclassid;
                                (*fl).set_fcount(
                                    fcount as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                );
                                (*fl).nexti = FLISTNULLINDX as uint32_t;
                            }
                        }
                        chunk_state_set_sclass(c, sclassid);
                    }
                    chunk_state_set_flags(c, flags);
                }
            } else if version == 0 as uint32_t
                && lockedto == 0 as uint32_t
                && flags as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            } else {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"chunks: wrong ending - chunk zero with version: %u and locked to: %u\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    version,
                    lockedto,
                );
                return -1 as ::core::ffi::c_int;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_store(mut fd: *mut bio) -> uint8_t {
    unsafe {
        let mut hdr: [uint8_t; 8] = [0; 8];
        let mut storebuff: [uint8_t; 18] = [0; 18];
        let mut pairsbuff: [uint8_t; 1533] = [0; 1533];
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut dptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut i: uint32_t = 0;
        let mut c: *mut chunk = ::core::ptr::null_mut::<chunk>();
        let mut chunkid: uint64_t = 0;
        let mut version: uint32_t = 0;
        let mut lockedto: uint32_t = 0;
        let mut flags: uint8_t = 0;
        let mut pairs: uint16_t = 0;
        let mut dynsize: uint32_t = 0;
        let mut findx: uint32_t = 0;
        let mut fl: *mut flist = ::core::ptr::null_mut::<flist>();
        if fd.is_null() {
            return 0x12 as uint8_t;
        }
        ptr = &raw mut hdr as *mut uint8_t;
        put64bit(&raw mut ptr, nextchunkid);
        if bio_write(
            fd,
            &raw mut hdr as *mut uint8_t as *const ::core::ffi::c_void,
            8 as uint64_t,
        ) != 8 as int64_t
        {
            return 0xff as uint8_t;
        }
        i = 0 as uint32_t;
        while i < chunkrehashpos {
            c = *chunkhashtab[(i >> HASHTAB_LOBITS) as usize]
                .offset((i & HASHTAB_MASK as uint32_t) as isize);
            while !c.is_null() {
                ptr = &raw mut storebuff as *mut uint8_t;
                dptr = &raw mut pairsbuff as *mut uint8_t;
                chunkid = (*c).chunkid;
                put64bit(&raw mut ptr, chunkid);
                version = (*c).version() as uint32_t;
                if (*c).allowreadzeros() != 0 {
                    version = (version as ::core::ffi::c_uint | 0x80000000 as ::core::ffi::c_uint)
                        as uint32_t;
                }
                put32bit(&raw mut ptr, version);
                lockedto = (*c).lockedto;
                put32bit(&raw mut ptr, lockedto);
                flags = (*c).flags() as uint8_t;
                dynsize = 0 as uint32_t;
                if (*c).fhead == FLISTNULLINDX as uint32_t {
                    pairs = 0 as uint16_t;
                } else if (*c).fhead < FLISTFIRSTINDX as uint32_t {
                    pairs = 1 as uint16_t;
                    put8bit(&raw mut dptr, (*c).sclassid);
                    put24bit(&raw mut dptr, (*c).fhead);
                    dynsize = 4 as uint32_t;
                } else {
                    pairs = 0 as uint16_t;
                    findx = (*c).fhead;
                    while (pairs as ::core::ffi::c_int) < CHUNKMAXPAIRS
                        && findx != FLISTNULLINDX as uint32_t
                    {
                        fl = flist_get(findx);
                        put8bit(&raw mut dptr, (*fl).sclassid);
                        put24bit(&raw mut dptr, (*fl).fcount() as uint32_t);
                        dynsize = dynsize.wrapping_add(4 as uint32_t);
                        pairs = pairs.wrapping_add(1);
                        findx = (*fl).nexti;
                    }
                    if findx != FLISTNULLINDX as uint32_t {
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_WARNING,
                            b"chunks: too many classes to store !!! - serious data structure error\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                    if pairs as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        put8bit(&raw mut dptr, (*c).sclassid);
                        dynsize = dynsize.wrapping_add(1);
                    }
                }
                if pairs as ::core::ffi::c_int > 255 as ::core::ffi::c_int {
                    flags = (flags as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int) as uint8_t;
                    pairs = (pairs as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint16_t;
                }
                put8bit(&raw mut ptr, flags);
                put8bit(&raw mut ptr, pairs as uint8_t);
                if bio_write(
                    fd,
                    &raw mut storebuff as *mut uint8_t as *const ::core::ffi::c_void,
                    CHUNKFSIZE as uint64_t,
                ) != CHUNKFSIZE as int64_t
                {
                    return 0xff as uint8_t;
                }
                if dynsize > 0 as uint32_t {
                    if bio_write(
                        fd,
                        &raw mut pairsbuff as *mut uint8_t as *const ::core::ffi::c_void,
                        dynsize as uint64_t,
                    ) != dynsize as int64_t
                    {
                        return 0xff as uint8_t;
                    }
                }
                c = (*c).next as *mut chunk;
            }
            i = i.wrapping_add(1);
        }
        memset(
            &raw mut storebuff as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            CHUNKFSIZE as size_t,
        );
        if bio_write(
            fd,
            &raw mut storebuff as *mut uint8_t as *const ::core::ffi::c_void,
            CHUNKFSIZE as uint64_t,
        ) != CHUNKFSIZE as int64_t
        {
            return 0xff as uint8_t;
        }
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_cleanup() {
    unsafe {
        let mut i: uint32_t = 0;
        let mut j: uint32_t = 0;
        let mut ds: *mut discserv = ::core::ptr::null_mut::<discserv>();
        chunk_priority_cleanall();
        chunk_io_ready_cleanall();
        chunk_replock_cleanall();
        while !discservers.is_null() {
            ds = discservers;
            discservers = (*discservers).next as *mut discserv;
            matocsserv_disconnection_finished((*cstab.offset((*ds).csid as isize)).ptr);
            free(ds as *mut ::core::ffi::c_void);
        }
        while !discservers_next.is_null() {
            ds = discservers_next;
            discservers_next = (*discservers_next).next as *mut discserv;
            matocsserv_disconnection_finished((*cstab.offset((*ds).csid as isize)).ptr);
            free(ds as *mut ::core::ffi::c_void);
        }
        slist_free_all();
        chunk_free_all();
        chunk_hash_cleanup();
        i = 0 as uint32_t;
        while i < MAXCSCOUNT as uint32_t {
            (*cstab.offset(i as isize)).next = i.wrapping_add(1 as uint32_t);
            (*cstab.offset(i as isize)).prev = i.wrapping_sub(1 as uint32_t);
            (*cstab.offset(i as isize)).valid = 0 as uint8_t;
            (*cstab.offset(i as isize))
                .set_registered(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            i = i.wrapping_add(1);
        }
        (*cstab.offset(0 as isize)).prev = MAXCSCOUNT as uint32_t;
        csfreehead = 0 as uint32_t;
        csfreetail = (MAXCSCOUNT - 1 as ::core::ffi::c_int) as uint32_t;
        csusedhead = MAXCSCOUNT as uint32_t;
        i = 0 as uint32_t;
        while i < (MAXSCLASS * 4 as ::core::ffi::c_int) as uint32_t {
            j = 0 as uint32_t;
            while j < 11 as uint32_t {
                *(*allchunkcopycounts.offset(i as isize)).offset(j as isize) = 0 as uint64_t;
                *(*regchunkcopycounts.offset(i as isize)).offset(j as isize) = 0 as uint64_t;
                *(*allchunkec8counts.offset(i as isize)).offset(j as isize) = 0 as uint64_t;
                *(*regchunkec8counts.offset(i as isize)).offset(j as isize) = 0 as uint64_t;
                *(*allchunkec4counts.offset(i as isize)).offset(j as isize) = 0 as uint64_t;
                *(*regchunkec4counts.offset(i as isize)).offset(j as isize) = 0 as uint64_t;
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        flist_cleanup(0 as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_newfs() {
    unsafe {
        chunks = 0 as uint32_t;
        nextchunkid = 1 as uint64_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_parse_rep_list(
    mut strlist: *mut ::core::ffi::c_char,
    mut replist: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut i: uint32_t = 0;
        let mut reptmp: [::core::ffi::c_double; 5] = [0.; 5];
        p = strlist;
        while *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
        reptmp[0 as usize] = strtod(p, &raw mut p);
        while *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
        if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            i = 0 as uint32_t;
            while i < 5 as uint32_t {
                *replist.offset(i as isize) = reptmp[0 as usize];
                i = i.wrapping_add(1);
            }
            return 1 as ::core::ffi::c_int;
        }
        i = 1 as uint32_t;
        while i < 5 as uint32_t {
            if *p as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
                if i == 4 as uint32_t && *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    *replist.offset(4 as isize) = reptmp[0 as usize];
                    i = 0 as uint32_t;
                    while i < 4 as uint32_t {
                        *replist.offset(i as isize) = reptmp[i as usize];
                        if reptmp[i as usize] > *replist.offset(4 as isize) {
                            *replist.offset(4 as isize) = reptmp[i as usize];
                        }
                        i = i.wrapping_add(1);
                    }
                    return 2 as ::core::ffi::c_int;
                }
                return -1 as ::core::ffi::c_int;
            }
            p = p.offset(1);
            while *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                || *p as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
                || *p as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || *p as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                p = p.offset(1);
            }
            reptmp[i as usize] = strtod(p, &raw mut p);
            while *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                || *p as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
                || *p as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || *p as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
            {
                p = p.offset(1);
            }
            i = i.wrapping_add(1);
        }
        if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            i = 0 as uint32_t;
            while i < 5 as uint32_t {
                *replist.offset(i as isize) = reptmp[i as usize];
                i = i.wrapping_add(1);
            }
            return 3 as ::core::ffi::c_int;
        }
        return -1 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_term() {
    unsafe {
        let mut i: uint32_t = 0;
        chunk_calculate_endanger_priority(::core::ptr::null_mut::<chunk>(), 1 as uint8_t);
        chunk_do_jobs(
            ::core::ptr::null_mut::<chunk>(),
            JOBS_TERM as ::core::ffi::c_int as uint8_t,
            main_time(),
            0 as uint8_t,
        );
        i = 0 as uint32_t;
        while i < (MAXSCLASS * 4 as ::core::ffi::c_int) as uint32_t {
            free(*allchunkcopycounts.offset(i as isize) as *mut ::core::ffi::c_void);
            free(*regchunkcopycounts.offset(i as isize) as *mut ::core::ffi::c_void);
            free(*allchunkec8counts.offset(i as isize) as *mut ::core::ffi::c_void);
            free(*regchunkec8counts.offset(i as isize) as *mut ::core::ffi::c_void);
            free(*allchunkec4counts.offset(i as isize) as *mut ::core::ffi::c_void);
            free(*regchunkec4counts.offset(i as isize) as *mut ::core::ffi::c_void);
            i = i.wrapping_add(1);
        }
        free(allchunkcopycounts as *mut ::core::ffi::c_void);
        free(regchunkcopycounts as *mut ::core::ffi::c_void);
        free(allchunkec8counts as *mut ::core::ffi::c_void);
        free(regchunkec8counts as *mut ::core::ffi::c_void);
        free(allchunkec4counts as *mut ::core::ffi::c_void);
        free(regchunkec4counts as *mut ::core::ffi::c_void);
        flist_cleanup(1 as uint8_t);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_load_cfg_common() {
    unsafe {
        let mut uniqmode: uint32_t = 0;
        uniqmode = cfg_getuint32(
            b"CHUNKS_UNIQUE_MODE\0".as_ptr() as *const ::core::ffi::c_char,
            0 as uint32_t,
        );
        DoNotUseSameIP = 0 as uint8_t;
        DoNotUseSameRack = 0 as uint8_t;
        if uniqmode == 1 as uint32_t {
            DoNotUseSameIP = 1 as uint8_t;
        } else if uniqmode == 2 as uint32_t {
            DoNotUseSameRack = 1 as uint8_t;
        }
        ReplicationsDelayInit = cfg_getuint32(
            b"REPLICATIONS_DELAY_INIT\0".as_ptr() as *const ::core::ffi::c_char,
            60 as uint32_t,
        );
        ReplicationsRespectTopology = cfg_getuint8(
            b"REPLICATIONS_RESPECT_TOPOLOGY\0".as_ptr() as *const ::core::ffi::c_char,
            0 as uint8_t,
        );
        CreationsRespectTopology = cfg_getuint32(
            b"CREATIONS_RESPECT_TOPOLOGY\0".as_ptr() as *const ::core::ffi::c_char,
            0 as uint32_t,
        );
        if cfg_isdefined(
            b"ACCEPTABLE_PERCENTAGE_DIFFERENCE\0".as_ptr() as *const ::core::ffi::c_char
        ) != 0
        {
            AcceptableDifference = cfg_getdouble(
                b"ACCEPTABLE_PERCENTAGE_DIFFERENCE\0".as_ptr() as *const ::core::ffi::c_char,
                1.0f64,
            ) / 100.0f64;
        } else {
            AcceptableDifference = cfg_getdouble(
                b"ACCEPTABLE_DIFFERENCE\0".as_ptr() as *const ::core::ffi::c_char,
                0.01f64,
            );
        }
        if AcceptableDifference < 0.001f64 {
            AcceptableDifference = 0.001f64;
        }
        if AcceptableDifference > 0.1f64 {
            AcceptableDifference = 0.1f64;
        }
        DangerMaxLeng = cfg_getuint32(
            b"PRIORITY_QUEUES_LENGTH\0".as_ptr() as *const ::core::ffi::c_char,
            1000000 as uint32_t,
        );
        if DangerMaxLeng < 10000 as uint32_t {
            DangerMaxLeng = 10000 as uint32_t;
        }
        if DangerMaxLeng > 100000000 as uint32_t {
            DangerMaxLeng = 100000000 as uint32_t;
        }
        DangerMinLeng = DangerMaxLeng.wrapping_div(100 as uint32_t);
        JobsTimerMilliSeconds = cfg_getuint32(
            b"JOBS_TIMER_MILLISECONDS\0".as_ptr() as *const ::core::ffi::c_char,
            5 as uint32_t,
        );
        if JobsTimerMilliSeconds < 1 as uint32_t {
            JobsTimerMilliSeconds = 1 as uint32_t;
        }
        if JobsTimerMilliSeconds > 50 as uint32_t {
            JobsTimerMilliSeconds = 50 as uint32_t;
        }
        TicksPerSecond = (1000 as uint32_t).wrapping_div(JobsTimerMilliSeconds);
        MaxFailsPerClass = cfg_getuint32(
            b"MAX_FAILS_PER_CLASS\0".as_ptr() as *const ::core::ffi::c_char,
            5 as uint32_t,
        );
        FailClassCounterResetCalls = cfg_getuint32(
            b"FAIL_CLASS_COUNTER_RESET_CALLS\0".as_ptr() as *const ::core::ffi::c_char,
            1 as uint32_t,
        );
        MaxRebalanceFails = cfg_getuint32(
            b"MAX_REBALANCE_FAILS\0".as_ptr() as *const ::core::ffi::c_char,
            5 as uint32_t,
        );
        FailRebalanceCounterResetCalls = cfg_getuint32(
            b"FAIL_REBALANCE_COUNTER_RESET_CALLS\0".as_ptr() as *const ::core::ffi::c_char,
            1 as uint32_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_loginfo(mut fd: *mut FILE) {
    unsafe {
        let mut i: uint8_t = 0;
        let mut sc: uint16_t = 0;
        let mut l: uint32_t = 0;
        let mut a: uint32_t = 0;
        let mut p: uint32_t = 0;
        let mut r: uint32_t = 0;
        fprintf(
            fd,
            b"[chunk loop params]\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            fd,
            b"Ticks Per Second : %u\n\0".as_ptr() as *const ::core::ffi::c_char,
            TicksPerSecond,
        );
        fprintf(
            fd,
            b"Hash Chunks Per Tick: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
            HashCPTMax,
        );
        fprintf(
            fd,
            b"Max Fails Per Class: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
            MaxFailsPerClass,
        );
        fprintf(
            fd,
            b"Max Rebalance Fails Per Class: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
            MaxRebalanceFails,
        );
        fprintf(fd, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        fprintf(fd, b"[chunks]\n\0".as_ptr() as *const ::core::ffi::c_char);
        i = 0 as uint8_t;
        while (i as ::core::ffi::c_int) < DANGER_PRIORITIES {
            l = chq_queue_elements[i as usize];
            a = chq_queue_last_append_count[i as usize];
            p = chq_queue_last_pop_count[i as usize];
            r = chq_queue_last_remove_count[i as usize];
            fprintf(
                fd,
                b"priority queue %u (%s): %u element%s (last minute appends: %u / pops: %u / removes: %u)\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                i as ::core::ffi::c_int,
                pristr[i as usize],
                l,
                if l == 1 as uint32_t {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"s\0".as_ptr() as *const ::core::ffi::c_char
                },
                a,
                p,
                r,
            );
            i = i.wrapping_add(1);
        }
        fprintf(fd, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        fprintf(
            fd,
            b"[job call counters (performed / skipped due to sclass limits)]\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        i = 0 as uint8_t;
        while i as ::core::ffi::c_int <= DANGER_PRIORITIES {
            let mut csum: uint32_t = 0;
            let mut nocsum: uint32_t = 0;
            let mut sep: ::core::ffi::c_int = 0;
            csum = 0 as uint32_t;
            nocsum = 0 as uint32_t;
            sc = 0 as uint16_t;
            while (sc as ::core::ffi::c_int) < MAXSCLASS {
                csum = csum.wrapping_add(job_call_last[sc as usize][i as usize]);
                nocsum = nocsum.wrapping_add(job_nocall_last[sc as usize][i as usize]);
                sc = sc.wrapping_add(1);
            }
            if csum > 0 as uint32_t || nocsum > 0 as uint32_t {
                if (i as ::core::ffi::c_int) < DANGER_PRIORITIES {
                    fprintf(
                        fd,
                        b"priority queue %u (%s)\0".as_ptr() as *const ::core::ffi::c_char,
                        i as ::core::ffi::c_int,
                        pristr[i as usize],
                    );
                } else {
                    fprintf(fd, b"standard\0".as_ptr() as *const ::core::ffi::c_char);
                }
                fprintf(
                    fd,
                    b": %u/%u [ \0".as_ptr() as *const ::core::ffi::c_char,
                    csum,
                    nocsum,
                );
                sep = 0 as ::core::ffi::c_int;
                sc = 0 as uint16_t;
                while (sc as ::core::ffi::c_int) < MAXSCLASS {
                    if job_call_last[sc as usize][i as usize] > 0 as uint32_t
                        || job_nocall_last[sc as usize][i as usize] > 0 as uint32_t
                    {
                        if sep != 0 {
                            fprintf(fd, b" , \0".as_ptr() as *const ::core::ffi::c_char);
                        }
                        if sc as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            fprintf(fd, b"(deleted)\0".as_ptr() as *const ::core::ffi::c_char);
                        } else {
                            fprintf(
                                fd,
                                b"'%s'\0".as_ptr() as *const ::core::ffi::c_char,
                                sclass_get_name(sc as uint8_t),
                            );
                        }
                        fprintf(
                            fd,
                            b":%u/%u\0".as_ptr() as *const ::core::ffi::c_char,
                            job_call_last[sc as usize][i as usize],
                            job_nocall_last[sc as usize][i as usize],
                        );
                        sep = 1 as ::core::ffi::c_int;
                    }
                    sc = sc.wrapping_add(1);
                }
                fprintf(fd, b" ]\n\0".as_ptr() as *const ::core::ffi::c_char);
            }
            i = i.wrapping_add(1);
        }
        fprintf(fd, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        fprintf(
            fd,
            b"[job exit reasons]\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        i = 0 as uint8_t;
        while (i as ::core::ffi::c_int) < JOB_EXIT_REASONS as ::core::ffi::c_int {
            let mut sum: uint32_t = 0;
            let mut sep_0: ::core::ffi::c_int = 0;
            sum = 0 as uint32_t;
            sc = 0 as uint16_t;
            while (sc as ::core::ffi::c_int) < MAXSCLASS {
                sum = sum.wrapping_add(job_exit_reasons_last[sc as usize][i as usize]);
                sc = sc.wrapping_add(1);
            }
            sep_0 = 0 as ::core::ffi::c_int;
            if sum > 0 as uint32_t {
                fprintf(
                    fd,
                    b"reason: %s : %u [ \0".as_ptr() as *const ::core::ffi::c_char,
                    job_exit_reason_to_str(i),
                    sum,
                );
                sc = 0 as uint16_t;
                while (sc as ::core::ffi::c_int) < MAXSCLASS {
                    if job_exit_reasons_last[sc as usize][i as usize] > 0 as uint32_t {
                        if sep_0 != 0 {
                            fprintf(fd, b" , \0".as_ptr() as *const ::core::ffi::c_char);
                        }
                        if sc as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            fprintf(fd, b"(deleted)\0".as_ptr() as *const ::core::ffi::c_char);
                        } else {
                            fprintf(
                                fd,
                                b"'%s'\0".as_ptr() as *const ::core::ffi::c_char,
                                sclass_get_name(sc as uint8_t),
                            );
                        }
                        fprintf(
                            fd,
                            b":%u\0".as_ptr() as *const ::core::ffi::c_char,
                            job_exit_reasons_last[sc as usize][i as usize],
                        );
                        sep_0 = 1 as ::core::ffi::c_int;
                    }
                    sc = sc.wrapping_add(1);
                }
                fprintf(fd, b" ]\n\0".as_ptr() as *const ::core::ffi::c_char);
            }
            i = i.wrapping_add(1);
        }
        fprintf(fd, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_reload() {
    unsafe {
        let mut oldMaxDelSoftLimit: uint32_t = 0;
        let mut oldMaxDelHardLimit: uint32_t = 0;
        let mut cps: uint32_t = 0;
        let mut oldJobsTimerMilliSeconds: uint32_t = 0;
        let mut repstr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        oldMaxDelSoftLimit = MaxDelSoftLimit;
        oldMaxDelHardLimit = MaxDelHardLimit;
        oldJobsTimerMilliSeconds = JobsTimerMilliSeconds;
        chunk_load_cfg_common();
        if oldJobsTimerMilliSeconds != JobsTimerMilliSeconds {
            main_msectime_change(jobs_timer, JobsTimerMilliSeconds, 0 as uint32_t);
        }
        MaxDelSoftLimit = cfg_getuint32(
            b"CHUNKS_SOFT_DEL_LIMIT\0".as_ptr() as *const ::core::ffi::c_char,
            10 as uint32_t,
        );
        if cfg_isdefined(b"CHUNKS_HARD_DEL_LIMIT\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
            MaxDelHardLimit = cfg_getuint32(
                b"CHUNKS_HARD_DEL_LIMIT\0".as_ptr() as *const ::core::ffi::c_char,
                25 as uint32_t,
            );
            if MaxDelHardLimit < MaxDelSoftLimit {
                MaxDelSoftLimit = MaxDelHardLimit;
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CHUNKS_SOFT_DEL_LIMIT is greater than CHUNKS_HARD_DEL_LIMIT - using CHUNKS_HARD_DEL_LIMIT for both\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        } else {
            MaxDelHardLimit = (3 as uint32_t).wrapping_mul(MaxDelSoftLimit);
        }
        if MaxDelSoftLimit == 0 as uint32_t {
            MaxDelSoftLimit = oldMaxDelSoftLimit;
            MaxDelHardLimit = oldMaxDelHardLimit;
        }
        if TmpMaxDelFrac < MaxDelSoftLimit as ::core::ffi::c_double {
            TmpMaxDelFrac = MaxDelSoftLimit as ::core::ffi::c_double;
        }
        if TmpMaxDelFrac > MaxDelHardLimit as ::core::ffi::c_double {
            TmpMaxDelFrac = MaxDelHardLimit as ::core::ffi::c_double;
        }
        if TmpMaxDel < MaxDelSoftLimit {
            TmpMaxDel = MaxDelSoftLimit;
        }
        if TmpMaxDel > MaxDelHardLimit {
            TmpMaxDel = MaxDelHardLimit;
        }
        repstr = cfg_getstr(
            b"CHUNKS_WRITE_REP_LIMIT\0".as_ptr() as *const ::core::ffi::c_char,
            b"2,1,1,4,4\0".as_ptr() as *const ::core::ffi::c_char,
        );
        match chunk_parse_rep_list(repstr, &raw mut MaxWriteRepl as *mut ::core::ffi::c_double) {
            -1 => {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"write replication limit parse error !!!\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            1 => {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"write replication limit in old format (1 element - now should be 5) - change limits to new format\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
            2 => {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"write replication limit in old format (4 elements - now should be 5) - change limits to new format\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
            _ => {}
        }
        free(repstr as *mut ::core::ffi::c_void);
        repstr = cfg_getstr(
            b"CHUNKS_READ_REP_LIMIT\0".as_ptr() as *const ::core::ffi::c_char,
            b"10,5,2,5,10\0".as_ptr() as *const ::core::ffi::c_char,
        );
        match chunk_parse_rep_list(repstr, &raw mut MaxReadRepl as *mut ::core::ffi::c_double) {
            -1 => {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"read replication limit parse error !!!\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            1 => {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"read replication limit in old format (1 element - now should be 5) - change limits to new format\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
            2 => {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"read replication limit in old format (4 elements - now should be 5) - change limits to new format\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
            _ => {}
        }
        free(repstr as *mut ::core::ffi::c_void);
        if cfg_isdefined(b"CHUNKS_LOOP_TIME\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
            LoopTimeMin = cfg_getuint32(
                b"CHUNKS_LOOP_TIME\0".as_ptr() as *const ::core::ffi::c_char,
                300 as uint32_t,
            );
            if LoopTimeMin < MINLOOPTIME as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CHUNKS_LOOP_TIME value too low (%u) increased to %u\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LoopTimeMin,
                    MINLOOPTIME,
                );
                LoopTimeMin = MINLOOPTIME as uint32_t;
            }
            if LoopTimeMin > MAXLOOPTIME as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CHUNKS_LOOP_TIME value too high (%u) decreased to %u\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LoopTimeMin,
                    MAXLOOPTIME,
                );
                LoopTimeMin = MAXLOOPTIME as uint32_t;
            }
            HashCPTMax = 0xffffffff as ::core::ffi::c_uint as uint32_t;
        } else {
            LoopTimeMin = cfg_getuint32(
                b"CHUNKS_LOOP_MIN_TIME\0".as_ptr() as *const ::core::ffi::c_char,
                300 as uint32_t,
            );
            if LoopTimeMin < MINLOOPTIME as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CHUNKS_LOOP_MIN_TIME value too low (%u) increased to %u\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LoopTimeMin,
                    MINLOOPTIME,
                );
                LoopTimeMin = MINLOOPTIME as uint32_t;
            }
            if LoopTimeMin > MAXLOOPTIME as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CHUNKS_LOOP_MIN_TIME value too high (%u) decreased to %u\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LoopTimeMin,
                    MAXLOOPTIME,
                );
                LoopTimeMin = MAXLOOPTIME as uint32_t;
            }
            cps = cfg_getuint32(
                b"CHUNKS_LOOP_MAX_CPS\0".as_ptr() as *const ::core::ffi::c_char,
                100000 as uint32_t,
            );
            if cps < MINCPS as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CHUNKS_LOOP_MAX_CPS value too low (%u) increased to %u\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    cps,
                    MINCPS,
                );
                cps = MINCPS as uint32_t;
            }
            if cps > MAXCPS as uint32_t {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"CHUNKS_LOOP_MAX_CPS value too high (%u) decreased to %u\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    cps,
                    MAXCPS,
                );
                cps = MAXCPS as uint32_t;
            }
            HashCPTMax = cps
                .wrapping_add(TicksPerSecond.wrapping_sub(1 as uint32_t))
                .wrapping_div(TicksPerSecond);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn chunk_strinit() -> ::core::ffi::c_int {
    unsafe {
        let mut i: uint32_t = 0;
        let mut j: uint32_t = 0;
        let mut cps: uint32_t = 0;
        let mut repstr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        chunk_load_cfg_common();
        MaxDelSoftLimit = cfg_getuint32(
            b"CHUNKS_SOFT_DEL_LIMIT\0".as_ptr() as *const ::core::ffi::c_char,
            10 as uint32_t,
        );
        if cfg_isdefined(b"CHUNKS_HARD_DEL_LIMIT\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
            MaxDelHardLimit = cfg_getuint32(
                b"CHUNKS_HARD_DEL_LIMIT\0".as_ptr() as *const ::core::ffi::c_char,
                25 as uint32_t,
            );
            if MaxDelHardLimit < MaxDelSoftLimit {
                MaxDelSoftLimit = MaxDelHardLimit;
                fprintf(
                    stderr,
                    b"CHUNKS_SOFT_DEL_LIMIT is greater than CHUNKS_HARD_DEL_LIMIT - using CHUNKS_HARD_DEL_LIMIT for both\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        } else {
            MaxDelHardLimit = (3 as uint32_t).wrapping_mul(MaxDelSoftLimit);
        }
        if MaxDelSoftLimit == 0 as uint32_t {
            fprintf(
                stderr,
                b"delete limit is zero !!!\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        TmpMaxDelFrac = MaxDelSoftLimit as ::core::ffi::c_double;
        TmpMaxDel = MaxDelSoftLimit;
        repstr = cfg_getstr(
            b"CHUNKS_WRITE_REP_LIMIT\0".as_ptr() as *const ::core::ffi::c_char,
            b"2,1,1,4,4\0".as_ptr() as *const ::core::ffi::c_char,
        );
        match chunk_parse_rep_list(repstr, &raw mut MaxWriteRepl as *mut ::core::ffi::c_double) {
            -1 => {
                fprintf(
                    stderr,
                    b"write replication limit parse error !!!\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            1 => {
                fprintf(
                    stderr,
                    b"write replication limit in old format (1 element - now should be 5) - change limits to new format\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
            2 => {
                fprintf(
                    stderr,
                    b"write replication limit in old format (4 elements - now should be 5) - change limits to new format\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
            _ => {}
        }
        free(repstr as *mut ::core::ffi::c_void);
        repstr = cfg_getstr(
            b"CHUNKS_READ_REP_LIMIT\0".as_ptr() as *const ::core::ffi::c_char,
            b"10,5,2,5,10\0".as_ptr() as *const ::core::ffi::c_char,
        );
        match chunk_parse_rep_list(repstr, &raw mut MaxReadRepl as *mut ::core::ffi::c_double) {
            -1 => {
                fprintf(
                    stderr,
                    b"read replication limit parse error !!!\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                return -1 as ::core::ffi::c_int;
            }
            1 => {
                fprintf(
                    stderr,
                    b"read replication limit in old format (1 element - now should be 5) - change limits to new format\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
            2 => {
                fprintf(
                    stderr,
                    b"read replication limit in old format (4 elements - now should be 5) - change limits to new format\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
            _ => {}
        }
        free(repstr as *mut ::core::ffi::c_void);
        if cfg_isdefined(b"CHUNKS_LOOP_TIME\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
            fprintf(
                stderr,
                b"Defining loop time by CHUNKS_LOOP_TIME option is deprecated - use CHUNKS_LOOP_MAX_CPS and CHUNKS_LOOP_MIN_TIME\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            LoopTimeMin = cfg_getuint32(
                b"CHUNKS_LOOP_TIME\0".as_ptr() as *const ::core::ffi::c_char,
                300 as uint32_t,
            );
            if LoopTimeMin < MINLOOPTIME as uint32_t {
                fprintf(
                    stderr,
                    b"CHUNKS_LOOP_TIME value too low (%u) increased to %u\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LoopTimeMin,
                    MINLOOPTIME,
                );
                LoopTimeMin = MINLOOPTIME as uint32_t;
            }
            if LoopTimeMin > MAXLOOPTIME as uint32_t {
                fprintf(
                    stderr,
                    b"CHUNKS_LOOP_TIME value too high (%u) decreased to %u\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LoopTimeMin,
                    MAXLOOPTIME,
                );
                LoopTimeMin = MAXLOOPTIME as uint32_t;
            }
            HashCPTMax = 0xffffffff as ::core::ffi::c_uint as uint32_t;
        } else {
            LoopTimeMin = cfg_getuint32(
                b"CHUNKS_LOOP_MIN_TIME\0".as_ptr() as *const ::core::ffi::c_char,
                300 as uint32_t,
            );
            if LoopTimeMin < MINLOOPTIME as uint32_t {
                fprintf(
                    stderr,
                    b"CHUNKS_LOOP_MIN_TIME value too low (%u) increased to %u\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LoopTimeMin,
                    MINLOOPTIME,
                );
                LoopTimeMin = MINLOOPTIME as uint32_t;
            }
            if LoopTimeMin > MAXLOOPTIME as uint32_t {
                fprintf(
                    stderr,
                    b"CHUNKS_LOOP_MIN_TIME value too high (%u) decreased to %u\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LoopTimeMin,
                    MAXLOOPTIME,
                );
                LoopTimeMin = MAXLOOPTIME as uint32_t;
            }
            cps = cfg_getuint32(
                b"CHUNKS_LOOP_MAX_CPS\0".as_ptr() as *const ::core::ffi::c_char,
                100000 as uint32_t,
            );
            if cps < MINCPS as uint32_t {
                fprintf(
                    stderr,
                    b"CHUNKS_LOOP_MAX_CPS value too low (%u) increased to %u\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    cps,
                    MINCPS,
                );
                cps = MINCPS as uint32_t;
            }
            if cps > MAXCPS as uint32_t {
                fprintf(
                    stderr,
                    b"CHUNKS_LOOP_MAX_CPS value too high (%u) decreased to %u\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    cps,
                    MAXCPS,
                );
                cps = MAXCPS as uint32_t;
            }
            HashCPTMax = cps
                .wrapping_add(TicksPerSecond.wrapping_sub(1 as uint32_t))
                .wrapping_div(TicksPerSecond);
        }
        flist_init();
        chunk_hash_init();
        chunk_io_ready_init();
        chunk_replock_init();
        cstab = malloc(::core::mem::size_of::<csdata>().wrapping_mul(MAXCSCOUNT as size_t))
            as *mut csdata;
        if cstab.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9560 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"cstab\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9560 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"cstab\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if cstab
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut csdata
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9560 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"cstab\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9560 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"cstab\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        i = 0 as uint32_t;
        while i < MAXCSCOUNT as uint32_t {
            (*cstab.offset(i as isize)).next = i.wrapping_add(1 as uint32_t);
            (*cstab.offset(i as isize)).prev = i.wrapping_sub(1 as uint32_t);
            (*cstab.offset(i as isize)).opchunks = ::core::ptr::null_mut::<csopchunk>();
            (*cstab.offset(i as isize)).valid = 0 as uint8_t;
            (*cstab.offset(i as isize))
                .set_registered(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*cstab.offset(i as isize)).set_mfr_state(
                UNKNOWN_HARD as ::core::ffi::c_int as ::core::ffi::c_uint as ::core::ffi::c_uint,
            );
            i = i.wrapping_add(1);
        }
        (*cstab.offset(0 as isize)).prev = MAXCSCOUNT as uint32_t;
        csfreehead = 0 as uint32_t;
        csfreetail = (MAXCSCOUNT - 1 as ::core::ffi::c_int) as uint32_t;
        csusedhead = MAXCSCOUNT as uint32_t;
        allchunkcopycounts = malloc(
            ::core::mem::size_of::<*mut uint64_t>()
                .wrapping_mul(MAXSCLASS as size_t)
                .wrapping_mul(4 as size_t),
        ) as *mut *mut uint64_t;
        if allchunkcopycounts.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9574 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"allchunkcopycounts\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9574 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"allchunkcopycounts\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if allchunkcopycounts
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut *mut uint64_t
        {
            let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9574 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"allchunkcopycounts\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9574 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"allchunkcopycounts\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            abort();
        }
        regchunkcopycounts = malloc(
            ::core::mem::size_of::<*mut uint64_t>()
                .wrapping_mul(MAXSCLASS as size_t)
                .wrapping_mul(4 as size_t),
        ) as *mut *mut uint64_t;
        if regchunkcopycounts.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9576 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"regchunkcopycounts\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9576 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"regchunkcopycounts\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if regchunkcopycounts
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut *mut uint64_t
        {
            let mut _mfs_errorstring_1: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9576 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"regchunkcopycounts\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9576 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"regchunkcopycounts\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_1,
            );
            abort();
        }
        allchunkec8counts = malloc(
            ::core::mem::size_of::<*mut uint64_t>()
                .wrapping_mul(MAXSCLASS as size_t)
                .wrapping_mul(4 as size_t),
        ) as *mut *mut uint64_t;
        if allchunkec8counts.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9578 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"allchunkec8counts\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9578 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"allchunkec8counts\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if allchunkec8counts
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut *mut uint64_t
        {
            let mut _mfs_errorstring_2: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9578 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"allchunkec8counts\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9578 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"allchunkec8counts\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_2,
            );
            abort();
        }
        regchunkec8counts = malloc(
            ::core::mem::size_of::<*mut uint64_t>()
                .wrapping_mul(MAXSCLASS as size_t)
                .wrapping_mul(4 as size_t),
        ) as *mut *mut uint64_t;
        if regchunkec8counts.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9580 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"regchunkec8counts\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9580 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"regchunkec8counts\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if regchunkec8counts
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut *mut uint64_t
        {
            let mut _mfs_errorstring_3: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9580 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"regchunkec8counts\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_3,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9580 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"regchunkec8counts\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_3,
            );
            abort();
        }
        allchunkec4counts = malloc(
            ::core::mem::size_of::<*mut uint64_t>()
                .wrapping_mul(MAXSCLASS as size_t)
                .wrapping_mul(4 as size_t),
        ) as *mut *mut uint64_t;
        if allchunkec4counts.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9582 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"allchunkec4counts\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9582 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"allchunkec4counts\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if allchunkec4counts
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut *mut uint64_t
        {
            let mut _mfs_errorstring_4: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9582 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"allchunkec4counts\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_4,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9582 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"allchunkec4counts\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_4,
            );
            abort();
        }
        regchunkec4counts = malloc(
            ::core::mem::size_of::<*mut uint64_t>()
                .wrapping_mul(MAXSCLASS as size_t)
                .wrapping_mul(4 as size_t),
        ) as *mut *mut uint64_t;
        if regchunkec4counts.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9584 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"regchunkec4counts\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9584 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"regchunkec4counts\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if regchunkec4counts
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut *mut uint64_t
        {
            let mut _mfs_errorstring_5: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9584 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"regchunkec4counts\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_5,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9584 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"regchunkec4counts\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_5,
            );
            abort();
        }
        i = 0 as uint32_t;
        while i < (MAXSCLASS * 4 as ::core::ffi::c_int) as uint32_t {
            *allchunkcopycounts.offset(i as isize) =
                malloc(::core::mem::size_of::<uint64_t>().wrapping_mul(11 as size_t))
                    as *mut uint64_t;
            if (*allchunkcopycounts.offset(i as isize)).is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9587 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"allchunkcopycounts[i]\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9587 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"allchunkcopycounts[i]\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if *allchunkcopycounts.offset(i as isize)
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint64_t
            {
                let mut _mfs_errorstring_6: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9587 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"allchunkcopycounts[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_6,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9587 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"allchunkcopycounts[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_6,
                );
                abort();
            }
            *regchunkcopycounts.offset(i as isize) =
                malloc(::core::mem::size_of::<uint64_t>().wrapping_mul(11 as size_t))
                    as *mut uint64_t;
            if (*regchunkcopycounts.offset(i as isize)).is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9589 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"regchunkcopycounts[i]\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9589 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"regchunkcopycounts[i]\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if *regchunkcopycounts.offset(i as isize)
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint64_t
            {
                let mut _mfs_errorstring_7: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9589 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"regchunkcopycounts[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_7,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9589 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"regchunkcopycounts[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_7,
                );
                abort();
            }
            *allchunkec8counts.offset(i as isize) =
                malloc(::core::mem::size_of::<uint64_t>().wrapping_mul(11 as size_t))
                    as *mut uint64_t;
            if (*allchunkec8counts.offset(i as isize)).is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9591 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"allchunkec8counts[i]\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9591 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"allchunkec8counts[i]\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if *allchunkec8counts.offset(i as isize)
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint64_t
            {
                let mut _mfs_errorstring_8: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9591 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"allchunkec8counts[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_8,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9591 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"allchunkec8counts[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_8,
                );
                abort();
            }
            *regchunkec8counts.offset(i as isize) =
                malloc(::core::mem::size_of::<uint64_t>().wrapping_mul(11 as size_t))
                    as *mut uint64_t;
            if (*regchunkec8counts.offset(i as isize)).is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9593 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"regchunkec8counts[i]\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9593 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"regchunkec8counts[i]\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if *regchunkec8counts.offset(i as isize)
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint64_t
            {
                let mut _mfs_errorstring_9: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9593 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"regchunkec8counts[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_9,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9593 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"regchunkec8counts[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_9,
                );
                abort();
            }
            *allchunkec4counts.offset(i as isize) =
                malloc(::core::mem::size_of::<uint64_t>().wrapping_mul(11 as size_t))
                    as *mut uint64_t;
            if (*allchunkec4counts.offset(i as isize)).is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9595 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"allchunkec4counts[i]\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9595 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"allchunkec4counts[i]\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if *allchunkec4counts.offset(i as isize)
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint64_t
            {
                let mut _mfs_errorstring_10: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9595 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"allchunkec4counts[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_10,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9595 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"allchunkec4counts[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_10,
                );
                abort();
            }
            *regchunkec4counts.offset(i as isize) =
                malloc(::core::mem::size_of::<uint64_t>().wrapping_mul(11 as size_t))
                    as *mut uint64_t;
            if (*regchunkec4counts.offset(i as isize)).is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9597 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"regchunkec4counts[i]\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9597 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"regchunkec4counts[i]\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if *regchunkec4counts.offset(i as isize)
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint64_t
            {
                let mut _mfs_errorstring_11: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9597 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"regchunkec4counts[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_11,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                    9597 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"regchunkec4counts[i]\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_11,
                );
                abort();
            }
            j = 0 as uint32_t;
            while j < 11 as uint32_t {
                *(*allchunkcopycounts.offset(i as isize)).offset(j as isize) = 0 as uint64_t;
                *(*regchunkcopycounts.offset(i as isize)).offset(j as isize) = 0 as uint64_t;
                *(*allchunkec8counts.offset(i as isize)).offset(j as isize) = 0 as uint64_t;
                *(*regchunkec8counts.offset(i as isize)).offset(j as isize) = 0 as uint64_t;
                *(*allchunkec4counts.offset(i as isize)).offset(j as isize) = 0 as uint64_t;
                *(*regchunkec4counts.offset(i as isize)).offset(j as isize) = 0 as uint64_t;
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        i = 0 as uint32_t;
        while i < DANGER_PRIORITIES as uint32_t {
            chq_queue_head[i as usize] = ::core::ptr::null_mut::<chq_element>();
            chq_queue_tail[i as usize] =
                (&raw mut chq_queue_head as *mut *mut chq_element).offset(i as isize);
            chq_queue_elements[i as usize] = 0 as uint32_t;
            chq_queue_last_append_count[i as usize] = 0 as uint32_t;
            chq_queue_current_append_count[i as usize] = 0 as uint32_t;
            chq_queue_last_pop_count[i as usize] = 0 as uint32_t;
            chq_queue_current_pop_count[i as usize] = 0 as uint32_t;
            chq_queue_last_remove_count[i as usize] = 0 as uint32_t;
            chq_queue_current_remove_count[i as usize] = 0 as uint32_t;
            i = i.wrapping_add(1);
        }
        i = 0 as uint32_t;
        while i < MAXSCLASS as uint32_t {
            j = 0 as uint32_t;
            while j < JOB_EXIT_REASONS as ::core::ffi::c_int as uint32_t {
                job_exit_reasons_last[i as usize][j as usize] = 0 as uint32_t;
                job_exit_reasons[i as usize][j as usize] = 0 as uint32_t;
                j = j.wrapping_add(1);
            }
            j = 0 as uint32_t;
            while j <= DANGER_PRIORITIES as uint32_t {
                job_call_last[i as usize][j as usize] = 0 as uint32_t;
                job_call[i as usize][j as usize] = 0 as uint32_t;
                job_nocall_last[i as usize][j as usize] = 0 as uint32_t;
                job_nocall[i as usize][j as usize] = 0 as uint32_t;
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        chq_hash = malloc(
            ::core::mem::size_of::<*mut chq_element>()
                .wrapping_mul(CHUNK_PRIORITY_HASHSIZE as size_t),
        ) as *mut *mut chq_element;
        if chq_hash.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9631 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"chq_hash\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9631 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"chq_hash\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if chq_hash
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut *mut chq_element
        {
            let mut _mfs_errorstring_12: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9631 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"chq_hash\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_12,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/chunks.c\0".as_ptr() as *const ::core::ffi::c_char,
                9631 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"chq_hash\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_12,
            );
            abort();
        }
        i = 0 as uint32_t;
        while i < CHUNK_PRIORITY_HASHSIZE as uint32_t {
            *chq_hash.offset(i as isize) = ::core::ptr::null_mut::<chq_element>();
            i = i.wrapping_add(1);
        }
        chq_elements = 0 as uint32_t;
        chunk_do_jobs(
            ::core::ptr::null_mut::<chunk>(),
            JOBS_INIT as ::core::ffi::c_int as uint8_t,
            main_time(),
            0 as uint8_t,
        );
        chunk_calculate_endanger_priority(::core::ptr::null_mut::<chunk>(), 0 as uint8_t);
        chunk_delay_init();
        main_reload_register_fname(
            Some(chunk_reload as unsafe extern "C" fn() -> ()),
            b"chunk_reload\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_info_register_fname(
            Some(chunk_loginfo as unsafe extern "C" fn(*mut FILE) -> ()),
            b"chunk_loginfo\0".as_ptr() as *const ::core::ffi::c_char,
        );
        jobs_timer = main_msectime_register_fname(
            JobsTimerMilliSeconds,
            0 as uint32_t,
            Some(chunk_jobs_main as unsafe extern "C" fn() -> ()),
            b"chunk_jobs_main\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_time_register_fname(
            60 as uint32_t,
            0 as uint32_t,
            Some(chunk_queue_counters_shift as unsafe extern "C" fn() -> ()),
            b"chunk_queue_counters_shift\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_time_register_fname(
            60 as uint32_t,
            0 as uint32_t,
            Some(chunk_job_exit_counters_shift as unsafe extern "C" fn() -> ()),
            b"chunk_job_exit_counters_shift\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_time_register_fname(
            60 as uint32_t,
            0 as uint32_t,
            Some(chunk_job_call_counters_shift as unsafe extern "C" fn() -> ()),
            b"chunk_job_call_counters_shift\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_destruct_register_fname(
            Some(chunk_term as unsafe extern "C" fn() -> ()),
            b"chunk_term\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 1 as ::core::ffi::c_int;
    }
}
