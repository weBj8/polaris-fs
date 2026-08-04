//! Chart data files (binary chart format + CGI rendering), P1 status.
//!
//! MIGRATION-IOU: blocked_on: P5::plfsmaster::chartsdata
//! Not migrated in P1: 5.2k lines of binary-format I/O and rendering where a
//! subtle mistake corrupts statistics history. P2 verified GUI integration
//! without changing this format owner; migration transfers to P5, where
//! plfsmaster writes it through chartsdata. Until then this stays verbatim.
//!
pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
pub enum internal_state {}
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn time(__timer: *mut time_t) -> time_t;
    unsafe fn gmtime(__timer: *const time_t) -> *mut tm;
    unsafe fn localtime(__timer: *const time_t) -> *mut tm;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    static mut stderr: *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
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
    unsafe fn write(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ssize_t;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn deflate(strm: z_streamp, flush: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn deflateEnd(strm: z_streamp) -> ::core::ffi::c_int;
    unsafe fn deflateReset(strm: z_streamp) -> ::core::ffi::c_int;
    unsafe fn deflateInit_(
        strm: z_streamp,
        level: ::core::ffi::c_int,
        version: *const ::core::ffi::c_char,
        stream_size: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn mycrc32(crc: uint32_t, block: *const ::core::ffi::c_void, leng: uint32_t)
    -> uint32_t;
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
pub type __time_t = ::core::ffi::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: ::core::ffi::c_int,
    pub tm_min: ::core::ffi::c_int,
    pub tm_hour: ::core::ffi::c_int,
    pub tm_mday: ::core::ffi::c_int,
    pub tm_mon: ::core::ffi::c_int,
    pub tm_year: ::core::ffi::c_int,
    pub tm_wday: ::core::ffi::c_int,
    pub tm_yday: ::core::ffi::c_int,
    pub tm_isdst: ::core::ffi::c_int,
    pub tm_gmtoff: ::core::ffi::c_long,
    pub tm_zone: *const ::core::ffi::c_char,
}
pub type ssize_t = isize;
pub type int32_t = i32;
pub type int64_t = i64;
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
pub type Byte = ::core::ffi::c_uchar;
pub type uInt = ::core::ffi::c_uint;
pub type uLong = ::core::ffi::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut ::core::ffi::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut ::core::ffi::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: ::core::ffi::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _statdef {
    pub name: *mut ::core::ffi::c_char,
    pub statid: uint32_t,
    pub mode: uint8_t,
    pub percent: uint8_t,
    pub scale: uint8_t,
    pub multiplier: uint16_t,
    pub divisor: uint16_t,
}
pub type statdef = _statdef;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _estatdef {
    pub name: *mut ::core::ffi::c_char,
    pub statid: uint32_t,
    pub c1src: uint32_t,
    pub c2src: uint32_t,
    pub c3src: uint32_t,
    pub mode: uint8_t,
    pub percent: uint8_t,
    pub scale: uint8_t,
    pub multiplier: uint16_t,
    pub divisor: uint16_t,
}
pub type estatdef = _estatdef;
pub type stat_record = [*mut uint64_t; 4];
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_TRUNC: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const INT64_MIN: ::core::ffi::c_long =
    -9223372036854775807 as ::core::ffi::c_long - 1 as ::core::ffi::c_long;
pub const ZLIB_VERSION: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"1.3.2\0") };
pub const Z_FINISH: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const Z_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const Z_STREAM_END: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CHARTS_MODE_ADD: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CHARTS_SCALE_MICRO: ::core::ffi::c_int = 0;
pub const CHARTS_SCALE_MILI: ::core::ffi::c_int = 1;
pub const CHARTS_SCALE_NONE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CHARTS_SCALE_KILO: ::core::ffi::c_int = 3;
pub const CHARTS_SCALE_MEGA: ::core::ffi::c_int = 4;
pub const CHARTS_SCALE_GIGA: ::core::ffi::c_int = 5;
pub const CHARTS_OP_CONST: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const CHARTS_OP_ADD: ::core::ffi::c_int = 1001 as ::core::ffi::c_int;
pub const CHARTS_OP_SUB: ::core::ffi::c_int = 1002 as ::core::ffi::c_int;
pub const CHARTS_OP_MIN: ::core::ffi::c_int = 1003 as ::core::ffi::c_int;
pub const CHARTS_OP_MAX: ::core::ffi::c_int = 1004 as ::core::ffi::c_int;
pub const CHARTS_OP_MUL: ::core::ffi::c_int = 1005 as ::core::ffi::c_int;
pub const CHARTS_OP_DIV: ::core::ffi::c_int = 1006 as ::core::ffi::c_int;
pub const CHARTS_OP_NEG: ::core::ffi::c_int = 1007 as ::core::ffi::c_int;
pub const CHARTS_OP_END: ::core::ffi::c_int = 1999 as ::core::ffi::c_int;
pub const CHARTS_DEFS_END: ::core::ffi::c_int = 2000 as ::core::ffi::c_int;
pub const CHARTS_DIRECT_START: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CHARTS_CALC_START: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
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
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_ERRNO_SYSLOG_STDERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MAXLENG: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const MINLENG: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const MAXHEIGHT: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const MINHEIGHT: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const XPOS: ::core::ffi::c_int = 43 as ::core::ffi::c_int;
pub const YPOS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const XADD: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const YADD: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const MAXXSIZE: ::core::ffi::c_int = MAXLENG + XADD;
pub const MAXYSIZE: ::core::ffi::c_int = MAXHEIGHT + YADD;
pub const SHORTRANGE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MEDIUMRANGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LONGRANGE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const VERYLONGRANGE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const RANGES: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const CHARTS_EXTENDED_START: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
static mut calcdefs: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
static mut calcstartpos: *mut *mut uint32_t = ::core::ptr::null_mut::<*mut uint32_t>();
static mut calcdefscount: uint32_t = 0;
static mut statdefs: *mut statdef = ::core::ptr::null_mut::<statdef>();
static mut statdefscount: uint32_t = 0;
static mut estatdefs: *mut estatdef = ::core::ptr::null_mut::<estatdef>();
static mut estatdefscount: uint32_t = 0;
static mut statsfilename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut series: *mut stat_record = ::core::ptr::null_mut::<stat_record>();
static mut pointers: [uint32_t; 4] = [0; 4];
static mut timepoint: [uint32_t; 4] = [0; 4];
static mut monotonic: *mut uint64_t = ::core::ptr::null_mut::<uint64_t>();
static mut shhour: uint32_t = 0;
static mut shmin: uint32_t = 0;
static mut medhour: uint32_t = 0;
static mut medmin: uint32_t = 0;
static mut lngyear: uint32_t = 0;
static mut lngmday: uint32_t = 0;
static mut lnghalfhour: uint32_t = 0;
static mut lngmonth: uint32_t = 0;
static mut vlngmday: uint32_t = 0;
static mut vlngyear: uint32_t = 0;
static mut vlngmonth: uint32_t = 0;
static mut chart: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
static mut rawchart: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
static mut compbuff: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
static mut rawchartsize: uint32_t = 0 as uint32_t;
static mut compbuffsize: uint32_t = 0 as uint32_t;
static mut compsize: uint32_t = 0 as uint32_t;
static mut zstr: z_stream = z_stream {
    next_in: ::core::ptr::null_mut::<Bytef>(),
    avail_in: 0,
    total_in: 0,
    next_out: ::core::ptr::null_mut::<Bytef>(),
    avail_out: 0,
    total_out: 0,
    msg: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    state: ::core::ptr::null_mut::<internal_state>(),
    zalloc: None,
    zfree: None,
    opaque: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    data_type: 0,
    adler: 0,
    reserved: 0,
};
pub const COLOR_TRANSPARENT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const COLOR_BKG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const COLOR_AXIS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const COLOR_AUX: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const COLOR_TEXT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const COLOR_NODATA: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const COLOR_DATA_BEGIN: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const COLOR_DATA_RANGE: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
static mut data_colors: [uint8_t; 18] = [
    0x4 as uint8_t,
    0xec as uint8_t,
    0xf1 as uint8_t,
    0x23 as uint8_t,
    0x86 as uint8_t,
    0xb4 as uint8_t,
    0x23 as uint8_t,
    0x86 as uint8_t,
    0xb4 as uint8_t,
    0x15 as uint8_t,
    0x2f as uint8_t,
    0x5f as uint8_t,
    0x15 as uint8_t,
    0x2f as uint8_t,
    0x5f as uint8_t,
    0x1 as uint8_t,
    0x4 as uint8_t,
    0x2c as uint8_t,
];
static mut png_header: [uint8_t; 847] = [
    137 as uint8_t,
    80 as uint8_t,
    78 as uint8_t,
    71 as uint8_t,
    13 as uint8_t,
    10 as uint8_t,
    26 as uint8_t,
    10 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    13 as uint8_t,
    'I' as uint8_t,
    'H' as uint8_t,
    'D' as uint8_t,
    'R' as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    8 as uint8_t,
    3 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    'C' as uint8_t,
    'R' as uint8_t,
    'C' as uint8_t,
    '#' as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0x3 as uint8_t,
    0 as uint8_t,
    'P' as uint8_t,
    'L' as uint8_t,
    'T' as uint8_t,
    'E' as uint8_t,
    0xff as uint8_t,
    0xff as uint8_t,
    0xff as uint8_t,
    0xff as uint8_t,
    0xff as uint8_t,
    0xff as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0x5f as uint8_t,
    0x20 as uint8_t,
    0 as uint8_t,
    0xc0 as uint8_t,
    0xc0 as uint8_t,
    0xc0 as uint8_t,
    0xff as uint8_t,
    0xff as uint8_t,
    0xde as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    'C' as uint8_t,
    'R' as uint8_t,
    'C' as uint8_t,
    '#' as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    1 as uint8_t,
    't' as uint8_t,
    'R' as uint8_t,
    'N' as uint8_t,
    'S' as uint8_t,
    0 as uint8_t,
    'C' as uint8_t,
    'R' as uint8_t,
    'C' as uint8_t,
    '#' as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    1 as uint8_t,
    'b' as uint8_t,
    'K' as uint8_t,
    'G' as uint8_t,
    'D' as uint8_t,
    0 as uint8_t,
    'C' as uint8_t,
    'R' as uint8_t,
    'C' as uint8_t,
    '#' as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    'I' as uint8_t,
    'D' as uint8_t,
    'A' as uint8_t,
    'T' as uint8_t,
];
static mut png_tailer: [uint8_t; 16] = [
    'C' as uint8_t,
    'R' as uint8_t,
    'C' as uint8_t,
    '#' as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    'I' as uint8_t,
    'E' as uint8_t,
    'N' as uint8_t,
    'D' as uint8_t,
    'C' as uint8_t,
    'R' as uint8_t,
    'C' as uint8_t,
    '#' as uint8_t,
];
static mut png_1x1: [uint8_t; 68] = [
    137 as uint8_t,
    80 as uint8_t,
    78 as uint8_t,
    71 as uint8_t,
    13 as uint8_t,
    10 as uint8_t,
    26 as uint8_t,
    10 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    13 as uint8_t,
    'I' as uint8_t,
    'H' as uint8_t,
    'D' as uint8_t,
    'R' as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    1 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    1 as uint8_t,
    8 as uint8_t,
    4 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0xb5 as uint8_t,
    0x1c as uint8_t,
    0xc as uint8_t,
    0x2 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    11 as uint8_t,
    'I' as uint8_t,
    'D' as uint8_t,
    'A' as uint8_t,
    'T' as uint8_t,
    0x8 as uint8_t,
    0xd7 as uint8_t,
    0x63 as uint8_t,
    0x60 as uint8_t,
    0x60 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0x3 as uint8_t,
    0 as uint8_t,
    0x1 as uint8_t,
    0x20 as uint8_t,
    0xd5 as uint8_t,
    0x94 as uint8_t,
    0xc7 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    'I' as uint8_t,
    'E' as uint8_t,
    'N' as uint8_t,
    'D' as uint8_t,
    0xae as uint8_t,
    0x42 as uint8_t,
    0x60 as uint8_t,
    0x82 as uint8_t,
];
static mut font: [[uint8_t; 9]; 25] = [
    [
        0xe as uint8_t,
        0x11 as uint8_t,
        0x11 as uint8_t,
        0x11 as uint8_t,
        0x11 as uint8_t,
        0x11 as uint8_t,
        0xe as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
    ],
    [
        0x4 as uint8_t,
        0xc as uint8_t,
        0x14 as uint8_t,
        0x4 as uint8_t,
        0x4 as uint8_t,
        0x4 as uint8_t,
        0x1f as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
    ],
    [
        0xe as uint8_t,
        0x11 as uint8_t,
        0x1 as uint8_t,
        0x2 as uint8_t,
        0x4 as uint8_t,
        0x8 as uint8_t,
        0x1f as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
    ],
    [
        0x1f as uint8_t,
        0x2 as uint8_t,
        0x4 as uint8_t,
        0xe as uint8_t,
        0x1 as uint8_t,
        0x11 as uint8_t,
        0xe as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
    ],
    [
        0x2 as uint8_t,
        0x6 as uint8_t,
        0xa as uint8_t,
        0x12 as uint8_t,
        0x1f as uint8_t,
        0x2 as uint8_t,
        0x2 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
    ],
    [
        0x1f as uint8_t,
        0x10 as uint8_t,
        0x1e as uint8_t,
        0x1 as uint8_t,
        0x1 as uint8_t,
        0x11 as uint8_t,
        0xe as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
    ],
    [
        0x6 as uint8_t,
        0x8 as uint8_t,
        0x10 as uint8_t,
        0x1e as uint8_t,
        0x11 as uint8_t,
        0x11 as uint8_t,
        0xe as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
    ],
    [
        0x1f as uint8_t,
        0x1 as uint8_t,
        0x2 as uint8_t,
        0x2 as uint8_t,
        0x4 as uint8_t,
        0x4 as uint8_t,
        0x4 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
    ],
    [
        0xe as uint8_t,
        0x11 as uint8_t,
        0x11 as uint8_t,
        0xe as uint8_t,
        0x11 as uint8_t,
        0x11 as uint8_t,
        0xe as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
    ],
    [
        0xe as uint8_t,
        0x11 as uint8_t,
        0x11 as uint8_t,
        0xf as uint8_t,
        0x1 as uint8_t,
        0x2 as uint8_t,
        0xc as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
    ],
    [
        0 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
        0x4 as uint8_t,
        0x4 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
    ],
    [
        0 as uint8_t,
        0 as uint8_t,
        0x4 as uint8_t,
        0 as uint8_t,
        0x4 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
    ],
    [
        0x8 as uint8_t,
        0x8 as uint8_t,
        0x9 as uint8_t,
        0xa as uint8_t,
        0xc as uint8_t,
        0xa as uint8_t,
        0x9 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
    ],
    [
        0x11 as uint8_t,
        0x1b as uint8_t,
        0x15 as uint8_t,
        0x11 as uint8_t,
        0x11 as uint8_t,
        0x11 as uint8_t,
        0x11 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
    ],
    [
        0xe as uint8_t,
        0x11 as uint8_t,
        0x10 as uint8_t,
        0x13 as uint8_t,
        0x11 as uint8_t,
        0x11 as uint8_t,
        0xe as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
    ],
    [
        0x1f as uint8_t,
        0x4 as uint8_t,
        0x4 as uint8_t,
        0x4 as uint8_t,
        0x4 as uint8_t,
        0x4 as uint8_t,
        0x4 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
    ],
    [
        0x1e as uint8_t,
        0x11 as uint8_t,
        0x11 as uint8_t,
        0x1e as uint8_t,
        0x10 as uint8_t,
        0x10 as uint8_t,
        0x10 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
    ],
    [
        0x1f as uint8_t,
        0x10 as uint8_t,
        0x10 as uint8_t,
        0x1c as uint8_t,
        0x10 as uint8_t,
        0x10 as uint8_t,
        0x1f as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
    ],
    [
        0x1f as uint8_t,
        0x1 as uint8_t,
        0x2 as uint8_t,
        0x4 as uint8_t,
        0x8 as uint8_t,
        0x10 as uint8_t,
        0x1f as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
    ],
    [
        0x11 as uint8_t,
        0x11 as uint8_t,
        0xa as uint8_t,
        0x4 as uint8_t,
        0x4 as uint8_t,
        0x4 as uint8_t,
        0x4 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
    ],
    [
        0 as uint8_t,
        0 as uint8_t,
        0x1e as uint8_t,
        0x15 as uint8_t,
        0x15 as uint8_t,
        0x15 as uint8_t,
        0x15 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
    ],
    [
        0 as uint8_t,
        0 as uint8_t,
        0x12 as uint8_t,
        0x12 as uint8_t,
        0x12 as uint8_t,
        0x12 as uint8_t,
        0x1d as uint8_t,
        0x10 as uint8_t,
        0x10 as uint8_t,
    ],
    [
        0x19 as uint8_t,
        0x1a as uint8_t,
        0x2 as uint8_t,
        0x4 as uint8_t,
        0x8 as uint8_t,
        0xb as uint8_t,
        0x13 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
    ],
    [
        0 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
    ],
    [
        0x1f as uint8_t,
        0x11 as uint8_t,
        0x11 as uint8_t,
        0x11 as uint8_t,
        0x11 as uint8_t,
        0x11 as uint8_t,
        0x1f as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
    ],
];
pub const FDOT: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const COLON: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const KILO: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const MEGA: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const GIGA: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const TERA: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const PETA: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const EXA: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const ZETTA: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const YOTTA: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const MILI: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const MICRO: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const PERCENT: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const SPACE: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const SQUARE: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getmonleng(mut year: uint32_t, mut month: uint32_t) -> uint32_t {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => return 31 as uint32_t,
        4 | 6 | 9 | 11 => return 30 as uint32_t,
        2 => {
            if year.wrapping_rem(4 as uint32_t) != 0 {
                return 28 as uint32_t;
            }
            if year.wrapping_rem(100 as uint32_t) != 0 {
                return 29 as uint32_t;
            }
            if year.wrapping_rem(400 as uint32_t) != 0 {
                return 28 as uint32_t;
            }
            return 29 as uint32_t;
        }
        _ => {}
    }
    return 0 as uint32_t;
}
pub const CHARTS_FILE_VERSION: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_store() {
    unsafe {
        let mut fd: ::core::ffi::c_int = 0;
        let mut s: uint32_t = 0;
        let mut i: uint32_t = 0;
        let mut j: uint32_t = 0;
        let mut p: uint32_t = 0;
        let mut tab: *mut uint64_t = ::core::ptr::null_mut::<uint64_t>();
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut hdr: [uint8_t; 16] = [0; 16];
        let mut data: [uint8_t; 32768] = [0; 32768];
        let mut namehdr: [::core::ffi::c_char; 100] = [0; 100];
        fd = open(
            statsfilename,
            O_WRONLY | O_TRUNC | O_CREAT,
            0o666 as ::core::ffi::c_int,
        );
        if fd < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"error creating charts data file\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return;
        }
        ptr = &raw mut hdr as *mut uint8_t;
        put32bit(&raw mut ptr, CHARTS_FILE_VERSION as uint32_t);
        put32bit(&raw mut ptr, MAXLENG as uint32_t);
        put32bit(&raw mut ptr, statdefscount);
        put32bit(&raw mut ptr, timepoint[SHORTRANGE as usize]);
        if write(
            fd,
            &raw mut hdr as *mut uint8_t as *mut ::core::ffi::c_void,
            16 as size_t,
        ) != 16 as ssize_t
        {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"error writing charts data file\0".as_ptr() as *const ::core::ffi::c_char,
            );
            close(fd);
            return;
        }
        i = 0 as uint32_t;
        while i < statdefscount {
            s = strlen((*statdefs.offset(i as isize)).name) as uint32_t;
            memset(
                &raw mut namehdr as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                100 as size_t,
            );
            memcpy(
                &raw mut namehdr as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                (*statdefs.offset(i as isize)).name as *const ::core::ffi::c_void,
                (if s > 100 as uint32_t {
                    100 as uint32_t
                } else {
                    s
                }) as size_t,
            );
            if write(
                fd,
                &raw mut namehdr as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                100 as size_t,
            ) != 100 as ssize_t
            {
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"error writing charts data file\0".as_ptr() as *const ::core::ffi::c_char,
                );
                close(fd);
                return;
            }
            j = 0 as uint32_t;
            while j < RANGES as uint32_t {
                tab = (*series.offset(i as isize))[j as usize];
                p = pointers[j as usize].wrapping_add(1 as uint32_t);
                ptr = &raw mut data as *mut uint8_t;
                s = 0 as uint32_t;
                while s < MAXLENG as uint32_t {
                    put64bit(
                        &raw mut ptr,
                        *tab.offset(p.wrapping_add(s).wrapping_rem(MAXLENG as uint32_t) as isize),
                    );
                    s = s.wrapping_add(1);
                }
                if write(
                    fd,
                    &raw mut data as *mut uint8_t as *mut ::core::ffi::c_void,
                    (8 as ::core::ffi::c_int * MAXLENG) as size_t,
                ) != (8 as ::core::ffi::c_int * MAXLENG) as ssize_t
                {
                    mfs_log(
                        MFSLOG_ERRNO_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"error writing charts data file\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    close(fd);
                    return;
                }
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        close(fd);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_load(mut mode: uint8_t) -> ::core::ffi::c_int {
    unsafe {
        let mut fd: ::core::ffi::c_int = 0;
        let mut i: uint32_t = 0;
        let mut j: uint32_t = 0;
        let mut k: uint32_t = 0;
        let mut fleng: uint32_t = 0;
        let mut fcharts: uint32_t = 0;
        let mut tab: *mut uint64_t = ::core::ptr::null_mut::<uint64_t>();
        let mut l: uint32_t = 0;
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut hdr: [uint8_t; 16] = [0; 16];
        let mut data: [uint8_t; 32768] = [0; 32768];
        let mut namehdr: [::core::ffi::c_char; 101] = [0; 101];
        fd = open(statsfilename, O_RDONLY);
        if fd < 0 as ::core::ffi::c_int {
            if mode as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"file loading error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    strerr(*__errno_location()),
                );
                return -1 as ::core::ffi::c_int;
            }
            if *__errno_location() != ENOENT {
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"error reading charts data file\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_NOTICE,
                    b"no charts data file - initializing empty charts\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            return 0 as ::core::ffi::c_int;
        }
        if read(
            fd,
            &raw mut hdr as *mut uint8_t as *mut ::core::ffi::c_void,
            16 as size_t,
        ) != 16 as ssize_t
        {
            if mode as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"error reading charts data file: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    strerr(*__errno_location()),
                );
                close(fd);
                return -1 as ::core::ffi::c_int;
            }
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"error reading charts data file\0".as_ptr() as *const ::core::ffi::c_char,
            );
            close(fd);
            return 0 as ::core::ffi::c_int;
        }
        ptr = &raw mut hdr as *mut uint8_t;
        i = get32bit(&raw mut ptr);
        if i != CHARTS_FILE_VERSION as uint32_t {
            if mode as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"unrecognized charts data file format\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                close(fd);
                return -1 as ::core::ffi::c_int;
            }
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"unrecognized charts data file format - initializing empty charts\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            close(fd);
            return 0 as ::core::ffi::c_int;
        }
        fleng = get32bit(&raw mut ptr);
        fcharts = get32bit(&raw mut ptr);
        i = get32bit(&raw mut ptr);
        timepoint[SHORTRANGE as usize] = i;
        pointers[SHORTRANGE as usize] = (MAXLENG - 1 as ::core::ffi::c_int) as uint32_t;
        pointers[MEDIUMRANGE as usize] = (MAXLENG - 1 as ::core::ffi::c_int) as uint32_t;
        pointers[LONGRANGE as usize] = (MAXLENG - 1 as ::core::ffi::c_int) as uint32_t;
        pointers[VERYLONGRANGE as usize] = (MAXLENG - 1 as ::core::ffi::c_int) as uint32_t;
        i = 0 as uint32_t;
        while i < fcharts {
            if read(
                fd,
                &raw mut namehdr as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                100 as size_t,
            ) != 100 as ssize_t
            {
                if mode as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                    fprintf(
                        stderr,
                        b"error reading charts data file: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        strerr(*__errno_location()),
                    );
                    close(fd);
                    return -1 as ::core::ffi::c_int;
                }
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"error reading charts data file\0".as_ptr() as *const ::core::ffi::c_char,
                );
                close(fd);
                return 0 as ::core::ffi::c_int;
            }
            namehdr[100 as usize] = 0 as ::core::ffi::c_char;
            j = 0 as uint32_t;
            while j < statdefscount
                && strcmp(
                    (*statdefs.offset(j as isize)).name,
                    &raw mut namehdr as *mut ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
            {
                j = j.wrapping_add(1);
            }
            if j >= statdefscount {
                lseek(
                    fd,
                    (RANGES as uint32_t)
                        .wrapping_mul(fleng)
                        .wrapping_mul(8 as uint32_t) as __off64_t,
                    SEEK_CUR,
                );
            } else {
                k = 0 as uint32_t;
                while k < RANGES as uint32_t {
                    tab = (*series.offset(j as isize))[k as usize];
                    if fleng > MAXLENG as uint32_t {
                        lseek(
                            fd,
                            (fleng.wrapping_sub(MAXLENG as uint32_t) as usize)
                                .wrapping_mul(::core::mem::size_of::<uint64_t>())
                                as __off64_t,
                            SEEK_CUR,
                        );
                    }
                    if fleng < MAXLENG as uint32_t {
                        if read(
                            fd,
                            &raw mut data as *mut uint8_t as *mut ::core::ffi::c_void,
                            (8 as uint32_t).wrapping_mul(fleng) as size_t,
                        ) != (8 as uint32_t).wrapping_mul(fleng) as ssize_t
                        {
                            if mode as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                                fprintf(
                                    stderr,
                                    b"error reading charts data file: %s\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    strerr(*__errno_location()),
                                );
                                close(fd);
                                return -1 as ::core::ffi::c_int;
                            }
                            mfs_log(
                                MFSLOG_ERRNO_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"error reading charts data file\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            close(fd);
                            return 0 as ::core::ffi::c_int;
                        }
                        ptr = &raw mut data as *mut uint8_t;
                        l = (MAXLENG as uint32_t).wrapping_sub(fleng);
                        while l < MAXLENG as uint32_t {
                            *tab.offset(l as isize) = get64bit(&raw mut ptr);
                            l = l.wrapping_add(1);
                        }
                    } else {
                        if read(
                            fd,
                            &raw mut data as *mut uint8_t as *mut ::core::ffi::c_void,
                            (8 as ::core::ffi::c_int * MAXLENG) as size_t,
                        ) != (8 as ::core::ffi::c_int * MAXLENG) as ssize_t
                        {
                            if mode as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                                fprintf(
                                    stderr,
                                    b"error reading charts data file: %s\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    strerr(*__errno_location()),
                                );
                                close(fd);
                                return -1 as ::core::ffi::c_int;
                            }
                            mfs_log(
                                MFSLOG_ERRNO_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"error reading charts data file\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            close(fd);
                            return 0 as ::core::ffi::c_int;
                        }
                        ptr = &raw mut data as *mut uint8_t;
                        l = 0 as uint32_t;
                        while l < MAXLENG as uint32_t {
                            *tab.offset(l as isize) = get64bit(&raw mut ptr);
                            l = l.wrapping_add(1);
                        }
                    }
                    k = k.wrapping_add(1);
                }
            }
            i = i.wrapping_add(1);
        }
        close(fd);
        if mode as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_INFO,
            b"stats file has been loaded\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_seriescnt(mut r#type: uint32_t) -> uint8_t {
    unsafe {
        let mut src: uint32_t = 0;
        let mut s: uint8_t = 0;
        if r#type < statdefscount {
            return 1 as uint8_t;
        }
        if r#type >= CHARTS_EXTENDED_START as uint32_t
            && r#type < (CHARTS_EXTENDED_START as uint32_t).wrapping_add(estatdefscount)
        {
            s = 0 as uint8_t;
            src = (*estatdefs
                .offset(r#type.wrapping_sub(CHARTS_EXTENDED_START as uint32_t) as isize))
            .c1src;
            if src >= CHARTS_DIRECT_START as uint32_t
                && src < (CHARTS_DIRECT_START as uint32_t).wrapping_add(statdefscount)
                || src >= CHARTS_CALC_START as uint32_t
                    && src < (CHARTS_CALC_START as uint32_t).wrapping_add(calcdefscount)
            {
                s = s.wrapping_add(1);
                src = (*estatdefs
                    .offset(r#type.wrapping_sub(CHARTS_EXTENDED_START as uint32_t) as isize))
                .c2src;
                if src >= CHARTS_DIRECT_START as uint32_t
                    && src < (CHARTS_DIRECT_START as uint32_t).wrapping_add(statdefscount)
                    || src >= CHARTS_CALC_START as uint32_t
                        && src < (CHARTS_CALC_START as uint32_t).wrapping_add(calcdefscount)
                {
                    s = s.wrapping_add(1);
                    src = (*estatdefs
                        .offset(r#type.wrapping_sub(CHARTS_EXTENDED_START as uint32_t) as isize))
                    .c3src;
                    if src >= CHARTS_DIRECT_START as uint32_t
                        && src < (CHARTS_DIRECT_START as uint32_t).wrapping_add(statdefscount)
                        || src >= CHARTS_CALC_START as uint32_t
                            && src < (CHARTS_CALC_START as uint32_t).wrapping_add(calcdefscount)
                    {
                        s = s.wrapping_add(1);
                    }
                }
            }
            return s;
        }
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_prepare_buff(mut datatab_ptr: *mut *mut uint64_t) -> *mut uint64_t {
    unsafe {
        let mut datatab: *mut uint64_t = ::core::ptr::null_mut::<uint64_t>();
        if (*datatab_ptr).is_null() {
            datatab = malloc(::core::mem::size_of::<uint64_t>().wrapping_mul(MAXLENG as size_t))
                as *mut uint64_t;
            if datatab.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    862 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"datatab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    862 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"datatab\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if datatab
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint64_t
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    862 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"datatab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    862 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"datatab\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            *datatab_ptr = datatab;
        } else {
            datatab = *datatab_ptr;
        }
        return datatab;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_filltab(
    mut datatab_ptr: *mut *mut uint64_t,
    mut range: uint32_t,
    mut r#type: uint32_t,
    mut cno: uint32_t,
    mut width: uint32_t,
) -> uint8_t {
    unsafe {
        let mut i: uint32_t = 0;
        let mut j: uint32_t = 0;
        let mut pointer: uint32_t = 0;
        let mut src: uint32_t = 0;
        let mut ops: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
        let mut stack: [int64_t; 50] = [0; 50];
        let mut sp: uint32_t = 0;
        let mut datatab: *mut uint64_t = ::core::ptr::null_mut::<uint64_t>();
        if range >= RANGES as uint32_t || cno == 0 as uint32_t || cno > 3 as uint32_t {
            return 0 as uint8_t;
        }
        pointer = pointers[range as usize];
        if r#type < statdefscount {
            if cno == 1 as uint32_t {
                datatab = charts_prepare_buff(datatab_ptr);
                i = 0 as uint32_t;
                while i < width {
                    j = (MAXLENG as uint32_t)
                        .wrapping_sub(width)
                        .wrapping_add(1 as uint32_t)
                        .wrapping_add(pointer)
                        .wrapping_add(i)
                        .wrapping_rem(MAXLENG as uint32_t);
                    *datatab.offset(i as isize) =
                        *(*series.offset(r#type as isize))[range as usize].offset(j as isize);
                    i = i.wrapping_add(1);
                }
                return 1 as uint8_t;
            }
        } else if r#type >= CHARTS_EXTENDED_START as uint32_t
            && r#type < (CHARTS_EXTENDED_START as uint32_t).wrapping_add(estatdefscount)
        {
            if cno == 1 as uint32_t {
                src = (*estatdefs
                    .offset(r#type.wrapping_sub(CHARTS_EXTENDED_START as uint32_t) as isize))
                .c1src;
            } else if cno == 2 as uint32_t {
                src = (*estatdefs
                    .offset(r#type.wrapping_sub(CHARTS_EXTENDED_START as uint32_t) as isize))
                .c2src;
            } else {
                src = (*estatdefs
                    .offset(r#type.wrapping_sub(CHARTS_EXTENDED_START as uint32_t) as isize))
                .c3src;
            }
            if src >= CHARTS_DIRECT_START as uint32_t
                && src < (CHARTS_DIRECT_START as uint32_t).wrapping_add(statdefscount)
            {
                datatab = charts_prepare_buff(datatab_ptr);
                i = 0 as uint32_t;
                while i < width {
                    j = (MAXLENG as uint32_t)
                        .wrapping_sub(width)
                        .wrapping_add(1 as uint32_t)
                        .wrapping_add(pointer)
                        .wrapping_add(i)
                        .wrapping_rem(MAXLENG as uint32_t);
                    *datatab.offset(i as isize) = *(*series
                        .offset(src.wrapping_sub(CHARTS_DIRECT_START as uint32_t) as isize))
                        [range as usize]
                        .offset(j as isize);
                    i = i.wrapping_add(1);
                }
                return 1 as uint8_t;
            } else if src >= CHARTS_CALC_START as uint32_t
                && src < (CHARTS_CALC_START as uint32_t).wrapping_add(calcdefscount)
            {
                datatab = charts_prepare_buff(datatab_ptr);
                i = 0 as uint32_t;
                while i < width {
                    j = (MAXLENG as uint32_t)
                        .wrapping_sub(width)
                        .wrapping_add(1 as uint32_t)
                        .wrapping_add(pointer)
                        .wrapping_add(i)
                        .wrapping_rem(MAXLENG as uint32_t);
                    sp = 0 as uint32_t;
                    ops = *calcstartpos
                        .offset(src.wrapping_sub(CHARTS_CALC_START as uint32_t) as isize);
                    while *ops != CHARTS_OP_END as uint32_t {
                        if *ops < statdefscount {
                            if sp < 50 as uint32_t {
                                if *(*series.offset(*ops as isize))[range as usize]
                                    .offset(j as isize)
                                    == 0xffffffffffffffff as uint64_t
                                {
                                    stack[sp as usize] = STACK_NODATA as int64_t;
                                } else {
                                    stack[sp as usize] = *(*series.offset(*ops as isize))
                                        [range as usize]
                                        .offset(j as isize)
                                        as int64_t;
                                }
                                sp = sp.wrapping_add(1);
                            }
                        } else if *ops == CHARTS_OP_ADD as uint32_t {
                            if sp >= 2 as uint32_t {
                                if stack[sp.wrapping_sub(1 as uint32_t) as usize]
                                    == STACK_NODATA as int64_t
                                    || stack[sp.wrapping_sub(2 as uint32_t) as usize]
                                        == STACK_NODATA as int64_t
                                {
                                    stack[sp.wrapping_sub(2 as uint32_t) as usize] =
                                        STACK_NODATA as int64_t;
                                } else {
                                    stack[sp.wrapping_sub(2 as uint32_t) as usize] +=
                                        stack[sp.wrapping_sub(1 as uint32_t) as usize];
                                }
                                sp = sp.wrapping_sub(1);
                            }
                        } else if *ops == CHARTS_OP_SUB as uint32_t {
                            if sp >= 2 as uint32_t {
                                if stack[sp.wrapping_sub(1 as uint32_t) as usize]
                                    == STACK_NODATA as int64_t
                                    || stack[sp.wrapping_sub(2 as uint32_t) as usize]
                                        == STACK_NODATA as int64_t
                                {
                                    stack[sp.wrapping_sub(2 as uint32_t) as usize] =
                                        STACK_NODATA as int64_t;
                                } else {
                                    stack[sp.wrapping_sub(2 as uint32_t) as usize] -=
                                        stack[sp.wrapping_sub(1 as uint32_t) as usize];
                                }
                                sp = sp.wrapping_sub(1);
                            }
                        } else if *ops == CHARTS_OP_MIN as uint32_t {
                            if sp >= 2 as uint32_t {
                                if stack[sp.wrapping_sub(1 as uint32_t) as usize]
                                    == STACK_NODATA as int64_t
                                    || stack[sp.wrapping_sub(2 as uint32_t) as usize]
                                        == STACK_NODATA as int64_t
                                {
                                    stack[sp.wrapping_sub(2 as uint32_t) as usize] =
                                        STACK_NODATA as int64_t;
                                } else if stack[sp.wrapping_sub(1 as uint32_t) as usize]
                                    < stack[sp.wrapping_sub(2 as uint32_t) as usize]
                                {
                                    stack[sp.wrapping_sub(2 as uint32_t) as usize] =
                                        stack[sp.wrapping_sub(1 as uint32_t) as usize];
                                }
                                sp = sp.wrapping_sub(1);
                            }
                        } else if *ops == CHARTS_OP_MAX as uint32_t {
                            if sp >= 2 as uint32_t {
                                if stack[sp.wrapping_sub(1 as uint32_t) as usize]
                                    == STACK_NODATA as int64_t
                                    || stack[sp.wrapping_sub(2 as uint32_t) as usize]
                                        == STACK_NODATA as int64_t
                                {
                                    stack[sp.wrapping_sub(2 as uint32_t) as usize] =
                                        STACK_NODATA as int64_t;
                                } else if stack[sp.wrapping_sub(1 as uint32_t) as usize]
                                    > stack[sp.wrapping_sub(2 as uint32_t) as usize]
                                {
                                    stack[sp.wrapping_sub(2 as uint32_t) as usize] =
                                        stack[sp.wrapping_sub(1 as uint32_t) as usize];
                                }
                                sp = sp.wrapping_sub(1);
                            }
                        } else if *ops == CHARTS_OP_MUL as uint32_t {
                            if sp >= 2 as uint32_t {
                                if stack[sp.wrapping_sub(1 as uint32_t) as usize]
                                    == STACK_NODATA as int64_t
                                    || stack[sp.wrapping_sub(2 as uint32_t) as usize]
                                        == STACK_NODATA as int64_t
                                {
                                    stack[sp.wrapping_sub(2 as uint32_t) as usize] =
                                        STACK_NODATA as int64_t;
                                } else {
                                    stack[sp.wrapping_sub(2 as uint32_t) as usize] *=
                                        stack[sp.wrapping_sub(1 as uint32_t) as usize];
                                }
                                sp = sp.wrapping_sub(1);
                            }
                        } else if *ops == CHARTS_OP_DIV as uint32_t {
                            if sp >= 2 as uint32_t {
                                if stack[sp.wrapping_sub(1 as uint32_t) as usize]
                                    == STACK_NODATA as int64_t
                                    || stack[sp.wrapping_sub(2 as uint32_t) as usize]
                                        == STACK_NODATA as int64_t
                                    || stack[sp.wrapping_sub(1 as uint32_t) as usize]
                                        == 0 as int64_t
                                {
                                    stack[sp.wrapping_sub(2 as uint32_t) as usize] =
                                        STACK_NODATA as int64_t;
                                } else {
                                    stack[sp.wrapping_sub(2 as uint32_t) as usize] /=
                                        stack[sp.wrapping_sub(1 as uint32_t) as usize];
                                }
                                sp = sp.wrapping_sub(1);
                            }
                        } else if *ops == CHARTS_OP_NEG as uint32_t {
                            if sp >= 1 as uint32_t {
                                if stack[sp.wrapping_sub(1 as uint32_t) as usize]
                                    != STACK_NODATA as int64_t
                                {
                                    stack[sp.wrapping_sub(1 as uint32_t) as usize] =
                                        -stack[sp.wrapping_sub(1 as uint32_t) as usize];
                                }
                            }
                        } else if *ops == CHARTS_OP_CONST as uint32_t {
                            ops = ops.offset(1);
                            if sp < 50 as uint32_t {
                                stack[sp as usize] = *ops as int64_t;
                                sp = sp.wrapping_add(1);
                            }
                        }
                        ops = ops.offset(1);
                    }
                    if sp >= 1 as uint32_t
                        && stack[sp.wrapping_sub(1 as uint32_t) as usize] >= 0 as int64_t
                    {
                        *datatab.offset(i as isize) =
                            stack[sp.wrapping_sub(1 as uint32_t) as usize] as uint64_t;
                    } else {
                        *datatab.offset(i as isize) =
                            0xffffffffffffffff as ::core::ffi::c_ulong as uint64_t;
                    }
                    i = i.wrapping_add(1);
                }
                return 1 as uint8_t;
            }
        }
        return 0 as uint8_t;
    }
}
pub const STACK_NODATA: ::core::ffi::c_long = INT64_MIN;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_get(mut r#type: uint32_t, mut numb: uint32_t) -> uint64_t {
    unsafe {
        let mut result: uint64_t = 0 as uint64_t;
        let mut cnt: uint64_t = 0;
        let mut tab: *mut uint64_t = ::core::ptr::null_mut::<uint64_t>();
        let mut i: uint32_t = 0;
        let mut j: uint32_t = 0;
        if numb == 0 as uint32_t || numb > MAXLENG as uint32_t {
            return result;
        }
        if r#type < statdefscount {
            tab = (*series.offset(r#type as isize))[SHORTRANGE as usize];
            j = pointers[SHORTRANGE as usize].wrapping_rem(MAXLENG as uint32_t);
            if (*statdefs.offset(r#type as isize)).mode as ::core::ffi::c_int == CHARTS_MODE_ADD {
                cnt = 0 as uint64_t;
                i = 0 as uint32_t;
                while i < numb {
                    if *tab.offset(j as isize) != 0xffffffffffffffff as uint64_t {
                        result = result.wrapping_add(*tab.offset(j as isize));
                        cnt = cnt.wrapping_add(1);
                    }
                    if j > 0 as uint32_t {
                        j = j.wrapping_sub(1);
                    } else {
                        j = (MAXLENG - 1 as ::core::ffi::c_int) as uint32_t;
                    }
                    i = i.wrapping_add(1);
                }
                if cnt > 0 as uint64_t {
                    result = result.wrapping_div(cnt);
                }
            } else {
                i = 0 as uint32_t;
                while i < numb {
                    if *tab.offset(j as isize) != 0xffffffffffffffff as uint64_t
                        && *tab.offset(j as isize) > result
                    {
                        result = *tab.offset(j as isize);
                    }
                    if j > 0 as uint32_t {
                        j = j.wrapping_sub(1);
                    } else {
                        j = (MAXLENG - 1 as ::core::ffi::c_int) as uint32_t;
                    }
                    i = i.wrapping_add(1);
                }
            }
        }
        return result;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_inittimepointers() {
    unsafe {
        let mut now: time_t = 0;
        let mut local: int32_t = 0;
        let mut ts: *mut tm = ::core::ptr::null_mut::<tm>();
        if timepoint[SHORTRANGE as usize] == 0 as uint32_t {
            now = time(::core::ptr::null_mut::<time_t>());
            ts = localtime(&raw mut now);
            local = (now as ::core::ffi::c_long + (*ts).tm_gmtoff) as int32_t;
        } else {
            now = timepoint[SHORTRANGE as usize].wrapping_mul(60 as uint32_t) as time_t;
            ts = gmtime(&raw mut now);
            local = now as int32_t;
        }
        timepoint[SHORTRANGE as usize] = (local / 60 as int32_t) as uint32_t;
        shmin = (*ts).tm_min as uint32_t;
        shhour = (*ts).tm_hour as uint32_t;
        timepoint[MEDIUMRANGE as usize] = (local / (60 as int32_t * 6 as int32_t)) as uint32_t;
        medmin = (*ts).tm_min as uint32_t;
        medhour = (*ts).tm_hour as uint32_t;
        timepoint[LONGRANGE as usize] = (local / (60 as int32_t * 30 as int32_t)) as uint32_t;
        lnghalfhour = ((*ts).tm_hour * 2 as ::core::ffi::c_int) as uint32_t;
        if (*ts).tm_min >= 30 as ::core::ffi::c_int {
            lnghalfhour = lnghalfhour.wrapping_add(1);
        }
        lngmday = (*ts).tm_mday as uint32_t;
        lngmonth = ((*ts).tm_mon + 1 as ::core::ffi::c_int) as uint32_t;
        lngyear = ((*ts).tm_year + 1900 as ::core::ffi::c_int) as uint32_t;
        timepoint[VERYLONGRANGE as usize] =
            (local / (60 as int32_t * 60 as int32_t * 24 as int32_t)) as uint32_t;
        vlngmday = (*ts).tm_mday as uint32_t;
        vlngmonth = ((*ts).tm_mon + 1 as ::core::ffi::c_int) as uint32_t;
        vlngyear = ((*ts).tm_year + 1900 as ::core::ffi::c_int) as uint32_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_add(mut data: *mut uint64_t, mut datats: uint32_t) {
    unsafe {
        let mut i: uint32_t = 0;
        let mut j: uint32_t = 0;
        let mut ts: *mut tm = ::core::ptr::null_mut::<tm>();
        let mut now: time_t = datats as time_t;
        let mut local: int32_t = 0;
        let mut nowtime: int32_t = 0;
        let mut delta: int32_t = 0;
        if !data.is_null() {
            j = 0 as uint32_t;
            while j < statdefscount {
                *monotonic.offset(j as isize) =
                    (*monotonic.offset(j as isize)).wrapping_add(*data.offset(j as isize));
                j = j.wrapping_add(1);
            }
        }
        ts = localtime(&raw mut now);
        local = (now as ::core::ffi::c_long + (*ts).tm_gmtoff) as int32_t;
        nowtime = local / 60 as int32_t;
        delta = (nowtime as uint32_t).wrapping_sub(timepoint[SHORTRANGE as usize]) as int32_t;
        if delta > 0 as int32_t {
            if delta > MAXLENG as int32_t {
                delta = MAXLENG as int32_t;
            }
            while delta > 0 as int32_t {
                pointers[SHORTRANGE as usize] = pointers[SHORTRANGE as usize].wrapping_add(1);
                pointers[SHORTRANGE as usize] =
                    pointers[SHORTRANGE as usize].wrapping_rem(MAXLENG as uint32_t);
                i = 0 as uint32_t;
                while i < statdefscount {
                    *(*series.offset(i as isize))[SHORTRANGE as usize]
                        .offset(pointers[SHORTRANGE as usize] as isize) =
                        0xffffffffffffffff as ::core::ffi::c_ulong as uint64_t;
                    i = i.wrapping_add(1);
                }
                delta -= 1;
            }
            timepoint[SHORTRANGE as usize] = nowtime as uint32_t;
            shmin = (*ts).tm_min as uint32_t;
            shhour = (*ts).tm_hour as uint32_t;
        }
        if delta <= 0 as int32_t && delta > -(MAXLENG as int32_t) && !data.is_null() {
            i = pointers[SHORTRANGE as usize]
                .wrapping_add(MAXLENG as uint32_t)
                .wrapping_add(delta as uint32_t)
                .wrapping_rem(MAXLENG as uint32_t);
            j = 0 as uint32_t;
            while j < statdefscount {
                if *(*series.offset(j as isize))[SHORTRANGE as usize].offset(i as isize)
                    == 0xffffffffffffffff as uint64_t
                {
                    *(*series.offset(j as isize))[SHORTRANGE as usize].offset(i as isize) =
                        *data.offset(j as isize);
                } else if (*statdefs.offset(j as isize)).mode as ::core::ffi::c_int
                    == CHARTS_MODE_ADD
                {
                    *(*series.offset(j as isize))[SHORTRANGE as usize].offset(i as isize) =
                        (*(*series.offset(j as isize))[SHORTRANGE as usize].offset(i as isize))
                            .wrapping_add(*data.offset(j as isize));
                } else if *data.offset(j as isize)
                    > *(*series.offset(j as isize))[SHORTRANGE as usize].offset(i as isize)
                {
                    *(*series.offset(j as isize))[SHORTRANGE as usize].offset(i as isize) =
                        *data.offset(j as isize);
                }
                j = j.wrapping_add(1);
            }
        }
        nowtime = local / (60 as int32_t * 6 as int32_t);
        delta = (nowtime as uint32_t).wrapping_sub(timepoint[MEDIUMRANGE as usize]) as int32_t;
        if delta > 0 as int32_t {
            if delta > MAXLENG as int32_t {
                delta = MAXLENG as int32_t;
            }
            while delta > 0 as int32_t {
                pointers[MEDIUMRANGE as usize] = pointers[MEDIUMRANGE as usize].wrapping_add(1);
                pointers[MEDIUMRANGE as usize] =
                    pointers[MEDIUMRANGE as usize].wrapping_rem(MAXLENG as uint32_t);
                i = 0 as uint32_t;
                while i < statdefscount {
                    *(*series.offset(i as isize))[MEDIUMRANGE as usize]
                        .offset(pointers[MEDIUMRANGE as usize] as isize) =
                        0xffffffffffffffff as ::core::ffi::c_ulong as uint64_t;
                    i = i.wrapping_add(1);
                }
                delta -= 1;
            }
            timepoint[MEDIUMRANGE as usize] = nowtime as uint32_t;
            medmin = (*ts).tm_min as uint32_t;
            medhour = (*ts).tm_hour as uint32_t;
        }
        if delta <= 0 as int32_t && delta > -(MAXLENG as int32_t) && !data.is_null() {
            i = pointers[MEDIUMRANGE as usize]
                .wrapping_add(MAXLENG as uint32_t)
                .wrapping_add(delta as uint32_t)
                .wrapping_rem(MAXLENG as uint32_t);
            j = 0 as uint32_t;
            while j < statdefscount {
                if *(*series.offset(j as isize))[MEDIUMRANGE as usize].offset(i as isize)
                    == 0xffffffffffffffff as uint64_t
                {
                    *(*series.offset(j as isize))[MEDIUMRANGE as usize].offset(i as isize) =
                        *data.offset(j as isize);
                } else if (*statdefs.offset(j as isize)).mode as ::core::ffi::c_int
                    == CHARTS_MODE_ADD
                {
                    *(*series.offset(j as isize))[MEDIUMRANGE as usize].offset(i as isize) =
                        (*(*series.offset(j as isize))[MEDIUMRANGE as usize].offset(i as isize))
                            .wrapping_add(*data.offset(j as isize));
                } else if *data.offset(j as isize)
                    > *(*series.offset(j as isize))[MEDIUMRANGE as usize].offset(i as isize)
                {
                    *(*series.offset(j as isize))[MEDIUMRANGE as usize].offset(i as isize) =
                        *data.offset(j as isize);
                }
                j = j.wrapping_add(1);
            }
        }
        nowtime = local / (60 as int32_t * 30 as int32_t);
        delta = (nowtime as uint32_t).wrapping_sub(timepoint[LONGRANGE as usize]) as int32_t;
        if delta > 0 as int32_t {
            if delta > MAXLENG as int32_t {
                delta = MAXLENG as int32_t;
            }
            while delta > 0 as int32_t {
                pointers[LONGRANGE as usize] = pointers[LONGRANGE as usize].wrapping_add(1);
                pointers[LONGRANGE as usize] =
                    pointers[LONGRANGE as usize].wrapping_rem(MAXLENG as uint32_t);
                i = 0 as uint32_t;
                while i < statdefscount {
                    *(*series.offset(i as isize))[LONGRANGE as usize]
                        .offset(pointers[LONGRANGE as usize] as isize) =
                        0xffffffffffffffff as ::core::ffi::c_ulong as uint64_t;
                    i = i.wrapping_add(1);
                }
                delta -= 1;
            }
            timepoint[LONGRANGE as usize] = nowtime as uint32_t;
            lnghalfhour = ((*ts).tm_hour * 2 as ::core::ffi::c_int) as uint32_t;
            if (*ts).tm_min >= 30 as ::core::ffi::c_int {
                lnghalfhour = lnghalfhour.wrapping_add(1);
            }
            lngmday = (*ts).tm_mday as uint32_t;
            lngmonth = ((*ts).tm_mon + 1 as ::core::ffi::c_int) as uint32_t;
            lngyear = ((*ts).tm_year + 1900 as ::core::ffi::c_int) as uint32_t;
        }
        if delta <= 0 as int32_t && delta > -(MAXLENG as int32_t) && !data.is_null() {
            i = pointers[LONGRANGE as usize]
                .wrapping_add(MAXLENG as uint32_t)
                .wrapping_add(delta as uint32_t)
                .wrapping_rem(MAXLENG as uint32_t);
            j = 0 as uint32_t;
            while j < statdefscount {
                if *(*series.offset(j as isize))[LONGRANGE as usize].offset(i as isize)
                    == 0xffffffffffffffff as uint64_t
                {
                    *(*series.offset(j as isize))[LONGRANGE as usize].offset(i as isize) =
                        *data.offset(j as isize);
                } else if (*statdefs.offset(j as isize)).mode as ::core::ffi::c_int
                    == CHARTS_MODE_ADD
                {
                    *(*series.offset(j as isize))[LONGRANGE as usize].offset(i as isize) =
                        (*(*series.offset(j as isize))[LONGRANGE as usize].offset(i as isize))
                            .wrapping_add(*data.offset(j as isize));
                } else if *data.offset(j as isize)
                    > *(*series.offset(j as isize))[LONGRANGE as usize].offset(i as isize)
                {
                    *(*series.offset(j as isize))[LONGRANGE as usize].offset(i as isize) =
                        *data.offset(j as isize);
                }
                j = j.wrapping_add(1);
            }
        }
        nowtime = local / (60 as int32_t * 60 as int32_t * 24 as int32_t);
        delta = (nowtime as uint32_t).wrapping_sub(timepoint[VERYLONGRANGE as usize]) as int32_t;
        if delta > 0 as int32_t {
            if delta > MAXLENG as int32_t {
                delta = MAXLENG as int32_t;
            }
            while delta > 0 as int32_t {
                pointers[VERYLONGRANGE as usize] = pointers[VERYLONGRANGE as usize].wrapping_add(1);
                pointers[VERYLONGRANGE as usize] =
                    pointers[VERYLONGRANGE as usize].wrapping_rem(MAXLENG as uint32_t);
                i = 0 as uint32_t;
                while i < statdefscount {
                    *(*series.offset(i as isize))[VERYLONGRANGE as usize]
                        .offset(pointers[VERYLONGRANGE as usize] as isize) =
                        0xffffffffffffffff as ::core::ffi::c_ulong as uint64_t;
                    i = i.wrapping_add(1);
                }
                delta -= 1;
            }
            timepoint[VERYLONGRANGE as usize] = nowtime as uint32_t;
            vlngmday = (*ts).tm_mday as uint32_t;
            vlngmonth = ((*ts).tm_mon + 1 as ::core::ffi::c_int) as uint32_t;
            vlngyear = ((*ts).tm_year + 1900 as ::core::ffi::c_int) as uint32_t;
        }
        if delta <= 0 as int32_t && delta > -(MAXLENG as int32_t) && !data.is_null() {
            i = pointers[VERYLONGRANGE as usize]
                .wrapping_add(MAXLENG as uint32_t)
                .wrapping_add(delta as uint32_t)
                .wrapping_rem(MAXLENG as uint32_t);
            j = 0 as uint32_t;
            while j < statdefscount {
                if *(*series.offset(j as isize))[VERYLONGRANGE as usize].offset(i as isize)
                    == 0xffffffffffffffff as uint64_t
                {
                    *(*series.offset(j as isize))[VERYLONGRANGE as usize].offset(i as isize) =
                        *data.offset(j as isize);
                } else if (*statdefs.offset(j as isize)).mode as ::core::ffi::c_int
                    == CHARTS_MODE_ADD
                {
                    *(*series.offset(j as isize))[VERYLONGRANGE as usize].offset(i as isize) =
                        (*(*series.offset(j as isize))[VERYLONGRANGE as usize].offset(i as isize))
                            .wrapping_add(*data.offset(j as isize));
                } else if *data.offset(j as isize)
                    > *(*series.offset(j as isize))[VERYLONGRANGE as usize].offset(i as isize)
                {
                    *(*series.offset(j as isize))[VERYLONGRANGE as usize].offset(i as isize) =
                        *data.offset(j as isize);
                }
                j = j.wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_term() {
    unsafe {
        let mut i: uint32_t = 0;
        let mut j: uint32_t = 0;
        free(statsfilename as *mut ::core::ffi::c_void);
        if !calcdefs.is_null() {
            free(calcdefs as *mut ::core::ffi::c_void);
        }
        if !calcstartpos.is_null() {
            free(calcstartpos as *mut ::core::ffi::c_void);
        }
        if !estatdefs.is_null() {
            free(estatdefs as *mut ::core::ffi::c_void);
        }
        i = 0 as uint32_t;
        while i < statdefscount {
            free((*statdefs.offset(i as isize)).name as *mut ::core::ffi::c_void);
            i = i.wrapping_add(1);
        }
        if !statdefs.is_null() {
            free(statdefs as *mut ::core::ffi::c_void);
        }
        i = 0 as uint32_t;
        while i < statdefscount {
            j = 0 as uint32_t;
            while j < RANGES as uint32_t {
                if !(*series.offset(i as isize))[j as usize].is_null() {
                    free((*series.offset(i as isize))[j as usize] as *mut ::core::ffi::c_void);
                }
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        if !series.is_null() {
            free(series as *mut ::core::ffi::c_void);
        }
        if !monotonic.is_null() {
            free(monotonic as *mut ::core::ffi::c_void);
        }
        if !compbuff.is_null() {
            free(compbuff as *mut ::core::ffi::c_void);
        }
        if !rawchart.is_null() {
            free(rawchart as *mut ::core::ffi::c_void);
        }
        if !chart.is_null() {
            free(chart as *mut ::core::ffi::c_void);
        }
        deflateEnd(&raw mut zstr);
    }
}
#[inline]
unsafe extern "C" fn png_make_palette() {
    unsafe {
        let mut rng: uint32_t = 0;
        let mut indx: uint32_t = 0;
        let mut r: ::core::ffi::c_double = 0.;
        let mut g: ::core::ffi::c_double = 0.;
        let mut b: ::core::ffi::c_double = 0.;
        let mut dr: ::core::ffi::c_double = 0.;
        let mut db: ::core::ffi::c_double = 0.;
        let mut dg: ::core::ffi::c_double = 0.;
        rng = 0 as uint32_t;
        while rng < 3 as uint32_t {
            r = data_colors[rng.wrapping_mul(6 as uint32_t).wrapping_add(0 as uint32_t) as usize]
                as ::core::ffi::c_double;
            g = data_colors[rng.wrapping_mul(6 as uint32_t).wrapping_add(1 as uint32_t) as usize]
                as ::core::ffi::c_double;
            b = data_colors[rng.wrapping_mul(6 as uint32_t).wrapping_add(2 as uint32_t) as usize]
                as ::core::ffi::c_double;
            dr = data_colors[rng.wrapping_mul(6 as uint32_t).wrapping_add(3 as uint32_t) as usize]
                as ::core::ffi::c_double;
            dg = data_colors[rng.wrapping_mul(6 as uint32_t).wrapping_add(4 as uint32_t) as usize]
                as ::core::ffi::c_double;
            db = data_colors[rng.wrapping_mul(6 as uint32_t).wrapping_add(5 as uint32_t) as usize]
                as ::core::ffi::c_double;
            dr -= r;
            dg -= g;
            db -= b;
            dr /= COLOR_DATA_RANGE as ::core::ffi::c_double;
            dg /= COLOR_DATA_RANGE as ::core::ffi::c_double;
            db /= COLOR_DATA_RANGE as ::core::ffi::c_double;
            r += 0.5f64;
            g += 0.5f64;
            b += 0.5f64;
            indx = 0 as uint32_t;
            while indx < COLOR_DATA_RANGE as uint32_t {
                png_header[(41 as uint32_t)
                    .wrapping_add(
                        (COLOR_DATA_BEGIN as uint32_t)
                            .wrapping_add(rng.wrapping_mul(COLOR_DATA_RANGE as uint32_t))
                            .wrapping_add(indx)
                            .wrapping_mul(3 as uint32_t),
                    )
                    .wrapping_add(0 as uint32_t) as usize] = r as uint8_t;
                png_header[(41 as uint32_t)
                    .wrapping_add(
                        (COLOR_DATA_BEGIN as uint32_t)
                            .wrapping_add(rng.wrapping_mul(COLOR_DATA_RANGE as uint32_t))
                            .wrapping_add(indx)
                            .wrapping_mul(3 as uint32_t),
                    )
                    .wrapping_add(1 as uint32_t) as usize] = g as uint8_t;
                png_header[(41 as uint32_t)
                    .wrapping_add(
                        (COLOR_DATA_BEGIN as uint32_t)
                            .wrapping_add(rng.wrapping_mul(COLOR_DATA_RANGE as uint32_t))
                            .wrapping_add(indx)
                            .wrapping_mul(3 as uint32_t),
                    )
                    .wrapping_add(2 as uint32_t) as usize] = b as uint8_t;
                r += dr;
                g += dg;
                b += db;
                indx = indx.wrapping_add(1);
            }
            rng = rng.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_init(
    mut calcs: *const uint32_t,
    mut stats: *const statdef,
    mut estats: *const estatdef,
    mut filename: *const ::core::ffi::c_char,
    mut mode: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: uint32_t = 0;
        let mut j: uint32_t = 0;
        chart = malloc((MAXXSIZE * MAXYSIZE) as size_t) as *mut uint8_t;
        if chart.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1356 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"chart\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1356 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"chart\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if chart
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut uint8_t
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1356 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"chart\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1356 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"chart\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        rawchartsize = ((1 as ::core::ffi::c_int + MAXXSIZE) * MAXYSIZE) as uint32_t;
        rawchart = malloc(rawchartsize as size_t) as *mut uint8_t;
        if rawchart.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1359 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"rawchart\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1359 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"rawchart\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if rawchart
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut uint8_t
        {
            let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1359 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"rawchart\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1359 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"rawchart\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            abort();
        }
        compbuffsize = (rawchartsize as uint64_t)
            .wrapping_mul(1001 as uint64_t)
            .wrapping_div(1000 as uint64_t)
            .wrapping_add(16 as uint64_t) as uint32_t;
        compbuff = malloc(compbuffsize as size_t) as *mut uint8_t;
        if compbuff.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1362 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"compbuff\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1362 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"compbuff\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if compbuff
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut uint8_t
        {
            let mut _mfs_errorstring_1: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1362 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"compbuff\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_1,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1362 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"compbuff\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_1,
            );
            abort();
        }
        statsfilename = strdup(filename);
        if statsfilename.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1365 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"statsfilename\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1365 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"statsfilename\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if statsfilename
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut ::core::ffi::c_char
        {
            let mut _mfs_errorstring_2: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1365 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"statsfilename\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_2,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                1365 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"statsfilename\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_2,
            );
            abort();
        }
        i = 0 as uint32_t;
        calcdefscount = 0 as uint32_t;
        while *calcs.offset(i as isize) != CHARTS_DEFS_END as uint32_t {
            if *calcs.offset(i as isize) == CHARTS_OP_END as uint32_t {
                calcdefscount = calcdefscount.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        if i > 0 as uint32_t && calcdefscount > 0 as uint32_t {
            calcdefs = malloc(::core::mem::size_of::<uint32_t>().wrapping_mul(i as size_t))
                as *mut uint32_t;
            if calcdefs.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1374 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"calcdefs\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1374 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"calcdefs\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if calcdefs
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut uint32_t
            {
                let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1374 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"calcdefs\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1374 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"calcdefs\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_3,
                );
                abort();
            }
            calcstartpos = malloc(
                ::core::mem::size_of::<*mut uint32_t>().wrapping_mul(calcdefscount as size_t),
            ) as *mut *mut uint32_t;
            if calcstartpos.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1376 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"calcstartpos\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1376 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"calcstartpos\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if calcstartpos
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut *mut uint32_t
            {
                let mut _mfs_errorstring_4: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1376 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"calcstartpos\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_4,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1376 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"calcstartpos\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_4,
                );
                abort();
            }
            j = 0 as uint32_t;
            *calcstartpos.offset(j as isize) = calcdefs;
            j = j.wrapping_add(1);
            i = 0 as uint32_t;
            while *calcs.offset(i as isize) != CHARTS_DEFS_END as uint32_t {
                *calcdefs.offset(i as isize) = *calcs.offset(i as isize);
                if *calcs.offset(i as isize) == CHARTS_OP_END as uint32_t {
                    if j < calcdefscount {
                        *calcstartpos.offset(j as isize) = calcdefs
                            .offset(i as isize)
                            .offset(1 as ::core::ffi::c_int as isize);
                        j = j.wrapping_add(1);
                    }
                }
                i = i.wrapping_add(1);
            }
        } else {
            calcdefs = ::core::ptr::null_mut::<uint32_t>();
            calcstartpos = ::core::ptr::null_mut::<*mut uint32_t>();
        }
        statdefscount = 0 as uint32_t;
        while (*stats.offset(statdefscount as isize)).divisor != 0 {
            statdefscount = statdefscount.wrapping_add(1);
        }
        if statdefscount > 0 as uint32_t {
            statdefs =
                malloc(::core::mem::size_of::<statdef>().wrapping_mul(statdefscount as size_t))
                    as *mut statdef;
            if statdefs.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1396 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"statdefs\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1396 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"statdefs\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if statdefs
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut statdef
            {
                let mut _mfs_errorstring_5: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1396 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"statdefs\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_5,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1396 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"statdefs\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_5,
                );
                abort();
            }
        } else {
            statdefs = ::core::ptr::null_mut::<statdef>();
        }
        i = 0 as uint32_t;
        while i < statdefscount {
            (*statdefs.offset(i as isize)).name = strdup((*stats.offset(i as isize)).name);
            if (*statdefs.offset(i as isize)).name.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1402 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"statdefs[i].name\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1402 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"statdefs[i].name\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if (*statdefs.offset(i as isize)).name
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut ::core::ffi::c_char
            {
                let mut _mfs_errorstring_6: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1402 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"statdefs[i].name\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_6,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1402 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"statdefs[i].name\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_6,
                );
                abort();
            }
            (*statdefs.offset(i as isize)).statid = (*stats.offset(i as isize)).statid;
            (*statdefs.offset(i as isize)).mode = (*stats.offset(i as isize)).mode;
            (*statdefs.offset(i as isize)).percent = (*stats.offset(i as isize)).percent;
            (*statdefs.offset(i as isize)).scale = (*stats.offset(i as isize)).scale;
            (*statdefs.offset(i as isize)).multiplier = (*stats.offset(i as isize)).multiplier;
            (*statdefs.offset(i as isize)).divisor = (*stats.offset(i as isize)).divisor;
            i = i.wrapping_add(1);
        }
        estatdefscount = 0 as uint32_t;
        while (*estats.offset(estatdefscount as isize)).divisor != 0 {
            estatdefscount = estatdefscount.wrapping_add(1);
        }
        if estatdefscount > 0 as uint32_t {
            estatdefs =
                malloc(::core::mem::size_of::<estatdef>().wrapping_mul(estatdefscount as size_t))
                    as *mut estatdef;
            if estatdefs.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1413 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"estatdefs\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1413 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"estatdefs\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if estatdefs
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut estatdef
            {
                let mut _mfs_errorstring_7: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1413 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"estatdefs\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_7,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1413 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"estatdefs\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_7,
                );
                abort();
            }
        } else {
            estatdefs = ::core::ptr::null_mut::<estatdef>();
        }
        i = 0 as uint32_t;
        while i < estatdefscount {
            if !(*estats.offset(i as isize)).name.is_null() {
                (*estatdefs.offset(i as isize)).name = strdup((*estats.offset(i as isize)).name);
                if (*estatdefs.offset(i as isize)).name.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1420 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"estatdefs[i].name\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1420 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"estatdefs[i].name\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if (*estatdefs.offset(i as isize)).name
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut ::core::ffi::c_char
                {
                    let mut _mfs_errorstring_8: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1420 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"estatdefs[i].name\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_8,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1420 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"estatdefs[i].name\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_8,
                    );
                    abort();
                }
            } else {
                (*estatdefs.offset(i as isize)).name =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            (*estatdefs.offset(i as isize)).statid = (*estats.offset(i as isize)).statid;
            (*estatdefs.offset(i as isize)).c1src = (*estats.offset(i as isize)).c1src;
            (*estatdefs.offset(i as isize)).c2src = (*estats.offset(i as isize)).c2src;
            (*estatdefs.offset(i as isize)).c3src = (*estats.offset(i as isize)).c3src;
            (*estatdefs.offset(i as isize)).mode = (*estats.offset(i as isize)).mode;
            (*estatdefs.offset(i as isize)).percent = (*estats.offset(i as isize)).percent;
            (*estatdefs.offset(i as isize)).scale = (*estats.offset(i as isize)).scale;
            (*estatdefs.offset(i as isize)).multiplier = (*estats.offset(i as isize)).multiplier;
            (*estatdefs.offset(i as isize)).divisor = (*estats.offset(i as isize)).divisor;
            i = i.wrapping_add(1);
        }
        if statdefscount > 0 as uint32_t {
            monotonic =
                malloc(::core::mem::size_of::<uint64_t>().wrapping_mul(statdefscount as size_t))
                    as *mut uint64_t;
            if monotonic.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1437 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"monotonic\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1437 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"monotonic\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if monotonic
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
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1437 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"monotonic\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_9,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1437 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"monotonic\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_9,
                );
                abort();
            }
            series =
                malloc(::core::mem::size_of::<stat_record>().wrapping_mul(statdefscount as size_t))
                    as *mut stat_record;
            if series.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1439 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"series\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1439 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"series\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if series
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut stat_record
            {
                let mut _mfs_errorstring_10: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1439 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"series\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_10,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1439 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"series\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_10,
                );
                abort();
            }
            i = 0 as uint32_t;
            while i < statdefscount {
                *monotonic.offset(i as isize) = 0 as uint64_t;
                j = 0 as uint32_t;
                while j < RANGES as uint32_t {
                    (*series.offset(i as isize))[j as usize] = malloc(
                        (MAXLENG as size_t).wrapping_mul(::core::mem::size_of::<uint64_t>()),
                    )
                        as *mut uint64_t;
                    if (*series.offset(i as isize))[j as usize].is_null() {
                        fprintf(
                            stderr,
                            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1444 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"series[i][j]\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1444 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"series[i][j]\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        abort();
                    } else if (*series.offset(i as isize))[j as usize]
                        == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                            -1 as ::core::ffi::c_int as usize,
                        ) as *mut uint64_t
                    {
                        let mut _mfs_errorstring_11: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1444 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"series[i][j]\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring_11,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/../mfscommon/charts.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1444 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"series[i][j]\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring_11,
                        );
                        abort();
                    }
                    memset(
                        (*series.offset(i as isize))[j as usize] as *mut ::core::ffi::c_void,
                        0xff as ::core::ffi::c_int,
                        (MAXLENG as size_t).wrapping_mul(::core::mem::size_of::<uint64_t>()),
                    );
                    j = j.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
        } else {
            series = ::core::ptr::null_mut::<stat_record>();
            monotonic = ::core::ptr::null_mut::<uint64_t>();
        }
        i = 0 as uint32_t;
        while i < RANGES as uint32_t {
            pointers[i as usize] = 0 as uint32_t;
            timepoint[i as usize] = 0 as uint32_t;
            i = i.wrapping_add(1);
        }
        if charts_load(mode) < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        charts_inittimepointers();
        if mode as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            charts_add(
                ::core::ptr::null_mut::<uint64_t>(),
                time(::core::ptr::null_mut::<time_t>()) as uint32_t,
            );
        }
        zstr.zalloc = None;
        zstr.zfree = None;
        zstr.opaque = NULL as voidpf;
        if deflateInit_(
            &raw mut zstr,
            1 as ::core::ffi::c_int,
            ZLIB_VERSION.as_ptr(),
            ::core::mem::size_of::<z_stream>() as ::core::ffi::c_int,
        ) != Z_OK
        {
            return -1 as ::core::ffi::c_int;
        }
        png_make_palette();
        return 0 as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn charts_puttext(
    mut posx: int32_t,
    mut posy: int32_t,
    mut color: uint8_t,
    mut data: *mut uint8_t,
    mut leng: uint32_t,
    mut minx: int32_t,
    mut maxx: int32_t,
    mut miny: int32_t,
    mut maxy: int32_t,
) {
    unsafe {
        let mut i: uint32_t = 0;
        let mut fx: uint32_t = 0;
        let mut fy: uint32_t = 0;
        let mut fp: uint8_t = 0;
        let mut fbits: uint8_t = 0;
        let mut px: int32_t = 0;
        let mut x: int32_t = 0;
        let mut y: int32_t = 0;
        i = 0 as uint32_t;
        while i < leng {
            px = i.wrapping_mul(6 as uint32_t).wrapping_add(posx as uint32_t) as int32_t;
            fp = *data.offset(i as isize);
            if fp as ::core::ffi::c_int > SQUARE {
                fp = SQUARE as uint8_t;
            }
            fy = 0 as uint32_t;
            while fy < 9 as uint32_t {
                fbits = font[fp as usize][fy as usize];
                if fbits != 0 {
                    fx = 0 as uint32_t;
                    while fx < 5 as uint32_t {
                        x = (px as uint32_t).wrapping_add(fx) as int32_t;
                        y = (posy as uint32_t).wrapping_add(fy) as int32_t;
                        if fbits as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int != 0
                            && x >= minx
                            && x <= maxx
                            && y >= miny
                            && y <= maxy
                        {
                            *chart.offset((MAXXSIZE as int32_t * y + x) as isize) = color;
                        }
                        fbits =
                            ((fbits as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as uint8_t;
                        fx = fx.wrapping_add(1);
                    }
                }
                fy = fy.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_fixmax(
    mut max: uint64_t,
    mut ypts: uint32_t,
    mut scale: *mut uint8_t,
    mut mode: *mut uint8_t,
    mut base: *mut uint16_t,
) -> ::core::ffi::c_double {
    unsafe {
        let mut cpmax: uint64_t = 0;
        let mut factor: uint64_t = 0;
        let mut cmode: uint8_t = 0;
        let mut ascale: uint8_t = 0;
        if max == 0 as uint64_t {
            max = 1 as uint64_t;
        }
        if max <= 9 as uint64_t {
            *base = max
                .wrapping_mul(100 as uint64_t)
                .wrapping_add(ypts as uint64_t)
                .wrapping_sub(1 as uint64_t)
                .wrapping_div(ypts as uint64_t) as uint16_t;
            if (*base as uint32_t).wrapping_mul(ypts) < 1000 as uint32_t {
                *mode = 2 as uint8_t;
                return (*base as uint32_t)
                    .wrapping_mul(ypts)
                    .wrapping_div(100 as uint32_t) as ::core::ffi::c_double;
            }
        }
        if max <= 99 as uint64_t {
            *base = max
                .wrapping_mul(10 as uint64_t)
                .wrapping_add(ypts as uint64_t)
                .wrapping_sub(1 as uint64_t)
                .wrapping_div(ypts as uint64_t) as uint16_t;
            if (*base as uint32_t).wrapping_mul(ypts) < 1000 as uint32_t {
                *mode = 1 as uint8_t;
                return (*base as uint32_t)
                    .wrapping_mul(ypts)
                    .wrapping_div(10 as uint32_t) as ::core::ffi::c_double;
            }
        }
        cpmax = 999 as uint64_t;
        cmode = 0 as uint8_t;
        ascale = 0 as uint8_t;
        factor = ypts as uint64_t;
        loop {
            if max <= cpmax {
                *base = max
                    .wrapping_add(factor)
                    .wrapping_sub(1 as uint64_t)
                    .wrapping_div(factor) as uint16_t;
                if (*base as uint32_t).wrapping_mul(ypts) < 1000 as uint32_t {
                    *mode = cmode;
                    *scale =
                        (*scale as ::core::ffi::c_int + ascale as ::core::ffi::c_int) as uint8_t;
                    return (*base as uint64_t).wrapping_mul(factor) as ::core::ffi::c_double;
                }
            }
            if cmode as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                cmode = 2 as uint8_t;
                ascale = (ascale as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t;
            } else {
                cmode = cmode.wrapping_sub(1);
            }
            factor = factor.wrapping_mul(10 as uint64_t);
            if cpmax.wrapping_mul(10 as uint64_t) > cpmax {
                cpmax = cpmax.wrapping_mul(10 as uint64_t);
            } else {
                if max.wrapping_add(9 as uint64_t) < max {
                    *base = max
                        .wrapping_div(10 as uint64_t)
                        .wrapping_add(factor.wrapping_div(10 as uint64_t))
                        .wrapping_sub(1 as uint64_t)
                        .wrapping_div(factor.wrapping_div(10 as uint64_t))
                        as uint16_t;
                } else {
                    *base = max
                        .wrapping_add(9 as uint64_t)
                        .wrapping_div(10 as uint64_t)
                        .wrapping_add(factor.wrapping_div(10 as uint64_t))
                        .wrapping_sub(1 as uint64_t)
                        .wrapping_div(factor.wrapping_div(10 as uint64_t))
                        as uint16_t;
                }
                *mode = cmode;
                *scale = (*scale as ::core::ffi::c_int + ascale as ::core::ffi::c_int) as uint8_t;
                return *base as ::core::ffi::c_double * factor as ::core::ffi::c_double;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_makechart(
    mut r#type: uint32_t,
    mut range: uint32_t,
    mut width: uint32_t,
    mut height: uint32_t,
) {
    unsafe {
        static mut jtab: [uint8_t; 11] = [
            MICRO as uint8_t,
            MILI as uint8_t,
            SPACE as uint8_t,
            KILO as uint8_t,
            MEGA as uint8_t,
            GIGA as uint8_t,
            TERA as uint8_t,
            PETA as uint8_t,
            EXA as uint8_t,
            ZETTA as uint8_t,
            YOTTA as uint8_t,
        ];
        let mut i: int32_t = 0;
        let mut j: int32_t = 0;
        let mut xy: uint32_t = 0;
        let mut xm: uint32_t = 0;
        let mut xd: uint32_t = 0;
        let mut xh: uint32_t = 0;
        let mut xs: uint32_t = 0;
        let mut xoff: uint32_t = 0;
        let mut xbold: uint32_t = 0;
        let mut ys: uint32_t = 0;
        let mut ypts: uint32_t = 0;
        let mut max: uint64_t = 0;
        let mut dmax: ::core::ffi::c_double = 0.;
        let mut d: uint64_t = 0;
        let mut c1d: uint64_t = 0;
        let mut c2d: uint64_t = 0;
        let mut c3d: uint64_t = 0;
        let mut c1dispdata: *mut uint64_t = ::core::ptr::null_mut::<uint64_t>();
        let mut c2dispdata: *mut uint64_t = ::core::ptr::null_mut::<uint64_t>();
        let mut c3dispdata: *mut uint64_t = ::core::ptr::null_mut::<uint64_t>();
        let mut scale: uint8_t = 0;
        let mut mode: uint8_t = 0 as uint8_t;
        let mut percent: uint8_t = 0 as uint8_t;
        let mut base: uint16_t = 0 as uint16_t;
        let mut text: [uint8_t; 6] = [0; 6];
        let mut colors: uint8_t = 0;
        memset(
            chart as *mut ::core::ffi::c_void,
            COLOR_TRANSPARENT,
            (MAXXSIZE * MAXYSIZE) as size_t,
        );
        c1dispdata = ::core::ptr::null_mut::<uint64_t>();
        c2dispdata = ::core::ptr::null_mut::<uint64_t>();
        c3dispdata = ::core::ptr::null_mut::<uint64_t>();
        colors = 0 as uint8_t;
        if charts_filltab(&raw mut c1dispdata, range, r#type, 1 as uint32_t, width) != 0 {
            colors = 1 as uint8_t;
        }
        if charts_filltab(&raw mut c2dispdata, range, r#type, 2 as uint32_t, width) != 0 {
            colors = 2 as uint8_t;
        }
        if charts_filltab(&raw mut c3dispdata, range, r#type, 3 as uint32_t, width) != 0 {
            colors = 3 as uint8_t;
        }
        max = 0 as uint64_t;
        i = 0 as ::core::ffi::c_int as int32_t;
        while i < width as int32_t {
            d = 0 as uint64_t;
            if colors as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
                && *c1dispdata.offset(i as isize) != 0xffffffffffffffff as uint64_t
            {
                d = d.wrapping_add(*c1dispdata.offset(i as isize));
            }
            if colors as ::core::ffi::c_int >= 2 as ::core::ffi::c_int
                && *c2dispdata.offset(i as isize) != 0xffffffffffffffff as uint64_t
            {
                d = d.wrapping_add(*c2dispdata.offset(i as isize));
            }
            if colors as ::core::ffi::c_int >= 3 as ::core::ffi::c_int
                && *c3dispdata.offset(i as isize) != 0xffffffffffffffff as uint64_t
            {
                d = d.wrapping_add(*c3dispdata.offset(i as isize));
            }
            if d > max {
                max = d;
            }
            i += 1;
        }
        if max as ::core::ffi::c_ulonglong > 1000000000000000000 as ::core::ffi::c_ulonglong {
            i = 0 as ::core::ffi::c_int as int32_t;
            while i < width as int32_t {
                if colors as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
                    && *c1dispdata.offset(i as isize) != 0xffffffffffffffff as uint64_t
                {
                    *c1dispdata.offset(i as isize) =
                        (*c1dispdata.offset(i as isize)).wrapping_div(1000 as uint64_t);
                }
                if colors as ::core::ffi::c_int >= 2 as ::core::ffi::c_int
                    && *c2dispdata.offset(i as isize) != 0xffffffffffffffff as uint64_t
                {
                    *c2dispdata.offset(i as isize) =
                        (*c2dispdata.offset(i as isize)).wrapping_div(1000 as uint64_t);
                }
                if colors as ::core::ffi::c_int >= 3 as ::core::ffi::c_int
                    && *c3dispdata.offset(i as isize) != 0xffffffffffffffff as uint64_t
                {
                    *c3dispdata.offset(i as isize) =
                        (*c3dispdata.offset(i as isize)).wrapping_div(1000 as uint64_t);
                }
                i += 1;
            }
            max = max.wrapping_div(1000 as uint64_t);
            scale = 1 as uint8_t;
        } else {
            scale = 0 as uint8_t;
        }
        if r#type < statdefscount
            && (*statdefs.offset(r#type as isize)).mode as ::core::ffi::c_int == CHARTS_MODE_ADD
            || r#type >= CHARTS_EXTENDED_START as uint32_t
                && r#type < (CHARTS_EXTENDED_START as uint32_t).wrapping_add(estatdefscount)
                && (*estatdefs
                    .offset(r#type.wrapping_sub(CHARTS_EXTENDED_START as uint32_t) as isize))
                .mode as ::core::ffi::c_int
                    == CHARTS_MODE_ADD
        {
            match range {
                1 => {
                    max = max.wrapping_add(5 as uint64_t).wrapping_div(6 as uint64_t);
                }
                2 => {
                    max = max
                        .wrapping_add(29 as uint64_t)
                        .wrapping_div(30 as uint64_t);
                }
                3 => {
                    max = max.wrapping_add(1439 as uint64_t).wrapping_div(
                        (24 as ::core::ffi::c_int * 60 as ::core::ffi::c_int) as uint64_t,
                    );
                }
                _ => {}
            }
        }
        if r#type < statdefscount {
            scale = (scale as ::core::ffi::c_int
                + (*statdefs.offset(r#type as isize)).scale as ::core::ffi::c_int)
                as uint8_t;
            percent = (*statdefs.offset(r#type as isize)).percent;
            max = max.wrapping_mul((*statdefs.offset(r#type as isize)).multiplier as uint64_t);
            max = max.wrapping_div((*statdefs.offset(r#type as isize)).divisor as uint64_t);
        } else if r#type >= CHARTS_EXTENDED_START as uint32_t
            && r#type < (CHARTS_EXTENDED_START as uint32_t).wrapping_add(estatdefscount)
        {
            scale = (scale as ::core::ffi::c_int
                + (*estatdefs
                    .offset(r#type.wrapping_sub(CHARTS_EXTENDED_START as uint32_t) as isize))
                .scale as ::core::ffi::c_int) as uint8_t;
            percent = (*estatdefs
                .offset(r#type.wrapping_sub(CHARTS_EXTENDED_START as uint32_t) as isize))
            .percent;
            max = max.wrapping_mul(
                (*estatdefs.offset(r#type.wrapping_sub(CHARTS_EXTENDED_START as uint32_t) as isize))
                    .multiplier as uint64_t,
            );
            max = max.wrapping_div(
                (*estatdefs.offset(r#type.wrapping_sub(CHARTS_EXTENDED_START as uint32_t) as isize))
                    .divisor as uint64_t,
            );
        }
        ypts = height.wrapping_div(20 as uint32_t);
        dmax = charts_fixmax(max, ypts, &raw mut scale, &raw mut mode, &raw mut base);
        if r#type < statdefscount {
            dmax *= (*statdefs.offset(r#type as isize)).divisor as ::core::ffi::c_int
                as ::core::ffi::c_double;
            dmax /= (*statdefs.offset(r#type as isize)).multiplier as ::core::ffi::c_int
                as ::core::ffi::c_double;
        } else if r#type >= CHARTS_EXTENDED_START as uint32_t
            && r#type < (CHARTS_EXTENDED_START as uint32_t).wrapping_add(estatdefscount)
        {
            dmax *= (*estatdefs
                .offset(r#type.wrapping_sub(CHARTS_EXTENDED_START as uint32_t) as isize))
            .divisor as ::core::ffi::c_int as ::core::ffi::c_double;
            dmax /= (*estatdefs
                .offset(r#type.wrapping_sub(CHARTS_EXTENDED_START as uint32_t) as isize))
            .multiplier as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        if r#type < statdefscount
            && (*statdefs.offset(r#type as isize)).mode as ::core::ffi::c_int == CHARTS_MODE_ADD
            || r#type >= CHARTS_EXTENDED_START as uint32_t
                && r#type < (CHARTS_EXTENDED_START as uint32_t).wrapping_add(estatdefscount)
                && (*estatdefs
                    .offset(r#type.wrapping_sub(CHARTS_EXTENDED_START as uint32_t) as isize))
                .mode as ::core::ffi::c_int
                    == CHARTS_MODE_ADD
        {
            match range {
                1 => {
                    dmax *= 6 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
                2 => {
                    dmax *= 30 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
                3 => {
                    dmax *= (24 as ::core::ffi::c_int * 60 as ::core::ffi::c_int)
                        as ::core::ffi::c_double;
                }
                _ => {}
            }
        }
        i = 0 as ::core::ffi::c_int as int32_t;
        while i < width as int32_t {
            j = 0 as ::core::ffi::c_int as int32_t;
            if colors as ::core::ffi::c_int >= 3 as ::core::ffi::c_int
                && *c3dispdata.offset(i as isize) != 0xffffffffffffffff as uint64_t
            {
                c3d = *c3dispdata.offset(i as isize);
                j = 1 as ::core::ffi::c_int as int32_t;
            } else {
                c3d = 0 as uint64_t;
            }
            if colors as ::core::ffi::c_int >= 2 as ::core::ffi::c_int
                && *c2dispdata.offset(i as isize) != 0xffffffffffffffff as uint64_t
            {
                c2d = c3d.wrapping_add(*c2dispdata.offset(i as isize));
                j = 1 as ::core::ffi::c_int as int32_t;
            } else {
                c2d = c3d;
            }
            if colors as ::core::ffi::c_int >= 1 as ::core::ffi::c_int
                && *c1dispdata.offset(i as isize) != 0xffffffffffffffff as uint64_t
            {
                c1d = c2d.wrapping_add(*c1dispdata.offset(i as isize));
                j = 1 as ::core::ffi::c_int as int32_t;
            } else {
                c1d = c2d;
            }
            if j == 0 as int32_t {
                j = 0 as ::core::ffi::c_int as int32_t;
                while j < height as int32_t {
                    *chart.offset(
                        (MAXXSIZE as int32_t * (j + YPOS as int32_t) + (i + XPOS as int32_t))
                            as isize,
                    ) = (if (j + i) % 3 as int32_t != 0 {
                        COLOR_BKG
                    } else {
                        COLOR_NODATA
                    }) as uint8_t;
                    j += 1;
                }
            } else {
                if c1d as ::core::ffi::c_ulonglong > 281474976710656 as ::core::ffi::c_ulonglong {
                    c1d = c1d.wrapping_div(65536 as uint64_t);
                    c2d = c2d.wrapping_div(65536 as uint64_t);
                    c3d = c3d.wrapping_div(65536 as uint64_t);
                    c1d = c1d.wrapping_mul(height as uint64_t);
                    c1d = (c1d as ::core::ffi::c_double
                        / (dmax / 65536 as ::core::ffi::c_int as ::core::ffi::c_double))
                        as uint64_t;
                    c2d = c2d.wrapping_mul(height as uint64_t);
                    c2d = (c2d as ::core::ffi::c_double
                        / (dmax / 65536 as ::core::ffi::c_int as ::core::ffi::c_double))
                        as uint64_t;
                    c3d = c3d.wrapping_mul(height as uint64_t);
                    c3d = (c3d as ::core::ffi::c_double
                        / (dmax / 65536 as ::core::ffi::c_int as ::core::ffi::c_double))
                        as uint64_t;
                } else {
                    c1d = c1d.wrapping_mul(height as uint64_t);
                    c1d = (c1d as ::core::ffi::c_double / dmax) as uint64_t;
                    c2d = c2d.wrapping_mul(height as uint64_t);
                    c2d = (c2d as ::core::ffi::c_double / dmax) as uint64_t;
                    c3d = c3d.wrapping_mul(height as uint64_t);
                    c3d = (c3d as ::core::ffi::c_double / dmax) as uint64_t;
                }
                j = 0 as ::core::ffi::c_int as int32_t;
                while (height as uint64_t) >= c1d.wrapping_add(j as uint64_t) {
                    *chart.offset(
                        (MAXXSIZE as int32_t * (j + YPOS as int32_t) + (i + XPOS as int32_t))
                            as isize,
                    ) = COLOR_BKG as uint8_t;
                    j += 1;
                }
                while (height as uint64_t) >= c2d.wrapping_add(j as uint64_t) {
                    if colors as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                        *chart.offset(
                            (MAXXSIZE as int32_t * (j + YPOS as int32_t) + (i + XPOS as int32_t))
                                as isize,
                        ) = (COLOR_DATA_BEGIN as uint32_t).wrapping_add(
                            height
                                .wrapping_sub(1 as uint32_t)
                                .wrapping_sub(j as uint32_t)
                                .wrapping_mul(3 as uint32_t)
                                .wrapping_mul(COLOR_DATA_RANGE as uint32_t)
                                .wrapping_div(height),
                        ) as uint8_t;
                    } else if colors as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                        *chart.offset(
                            (MAXXSIZE as int32_t * (j + YPOS as int32_t) + (i + XPOS as int32_t))
                                as isize,
                        ) = (COLOR_DATA_BEGIN as ::core::ffi::c_double
                            + height
                                .wrapping_sub(1 as uint32_t)
                                .wrapping_sub(j as uint32_t)
                                as ::core::ffi::c_double
                                * 1.5f64
                                * COLOR_DATA_RANGE as ::core::ffi::c_double
                                / height as ::core::ffi::c_double)
                            as uint8_t;
                    } else if colors as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
                        *chart.offset(
                            (MAXXSIZE as int32_t * (j + YPOS as int32_t) + (i + XPOS as int32_t))
                                as isize,
                        ) = (COLOR_DATA_BEGIN as uint32_t).wrapping_add(
                            height
                                .wrapping_sub(1 as uint32_t)
                                .wrapping_sub(j as uint32_t)
                                .wrapping_mul(COLOR_DATA_RANGE as uint32_t)
                                .wrapping_div(height),
                        ) as uint8_t;
                    }
                    j += 1;
                }
                while (height as uint64_t) >= c3d.wrapping_add(j as uint64_t) {
                    if colors as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                        *chart.offset(
                            (MAXXSIZE as int32_t * (j + YPOS as int32_t) + (i + XPOS as int32_t))
                                as isize,
                        ) = (COLOR_DATA_BEGIN as ::core::ffi::c_double
                            + 1.5f64 * COLOR_DATA_RANGE as ::core::ffi::c_double
                            + height
                                .wrapping_sub(1 as uint32_t)
                                .wrapping_sub(j as uint32_t)
                                as ::core::ffi::c_double
                                * 1.5f64
                                * COLOR_DATA_RANGE as ::core::ffi::c_double
                                / height as ::core::ffi::c_double)
                            as uint8_t;
                    } else if colors as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
                        *chart.offset(
                            (MAXXSIZE as int32_t * (j + YPOS as int32_t) + (i + XPOS as int32_t))
                                as isize,
                        ) = ((COLOR_DATA_BEGIN + COLOR_DATA_RANGE) as uint32_t).wrapping_add(
                            height
                                .wrapping_sub(1 as uint32_t)
                                .wrapping_sub(j as uint32_t)
                                .wrapping_mul(COLOR_DATA_RANGE as uint32_t)
                                .wrapping_div(height),
                        ) as uint8_t;
                    }
                    j += 1;
                }
                while height as int32_t > j {
                    *chart.offset(
                        (MAXXSIZE as int32_t * (j + YPOS as int32_t) + (i + XPOS as int32_t))
                            as isize,
                    ) = ((COLOR_DATA_BEGIN + 2 as ::core::ffi::c_int * COLOR_DATA_RANGE)
                        as uint32_t)
                        .wrapping_add(
                            height
                                .wrapping_sub(1 as uint32_t)
                                .wrapping_sub(j as uint32_t)
                                .wrapping_mul(COLOR_DATA_RANGE as uint32_t)
                                .wrapping_div(height),
                        ) as uint8_t;
                    j += 1;
                }
            }
            i += 1;
        }
        i = -3 as ::core::ffi::c_int as int32_t;
        while i < width as int32_t + 3 as int32_t {
            *chart.offset(
                (MAXXSIZE as uint32_t)
                    .wrapping_mul(height.wrapping_add(YPOS as uint32_t))
                    .wrapping_add((i + XPOS as int32_t) as uint32_t) as isize,
            ) = COLOR_AXIS as uint8_t;
            i += 1;
        }
        i = -2 as ::core::ffi::c_int as int32_t;
        while i < height as int32_t + 5 as int32_t {
            *chart.offset(
                (MAXXSIZE as uint32_t)
                    .wrapping_mul(
                        height
                            .wrapping_sub(i as uint32_t)
                            .wrapping_add(YPOS as uint32_t),
                    )
                    .wrapping_add((XPOS - 1 as ::core::ffi::c_int) as uint32_t)
                    as isize,
            ) = COLOR_AXIS as uint8_t;
            *chart.offset(
                (MAXXSIZE as uint32_t)
                    .wrapping_mul(
                        height
                            .wrapping_sub(i as uint32_t)
                            .wrapping_add(YPOS as uint32_t),
                    )
                    .wrapping_add((XPOS as uint32_t).wrapping_add(width)) as isize,
            ) = COLOR_AXIS as uint8_t;
            i += 1;
        }
        xs = 0 as uint32_t;
        xh = xs;
        xd = xh;
        xm = xd;
        xy = xm;
        if range < 3 as uint32_t {
            if range == 2 as uint32_t {
                xs = 12 as uint32_t;
                xoff = lnghalfhour.wrapping_rem(12 as uint32_t);
                xbold = 4 as uint32_t;
                xh = lnghalfhour.wrapping_div(12 as uint32_t);
                xd = lngmday;
                xm = lngmonth;
                xy = lngyear;
            } else if range == 1 as uint32_t {
                xs = 10 as uint32_t;
                xoff = medmin.wrapping_div(6 as uint32_t);
                xbold = 6 as uint32_t;
                xh = medhour;
            } else {
                xs = 60 as uint32_t;
                xoff = shmin;
                xbold = 1 as uint32_t;
                xh = shhour;
            }
            i = width.wrapping_sub(xoff).wrapping_sub(1 as uint32_t) as int32_t;
            while i >= 0 as int32_t {
                if xh.wrapping_rem(xbold) == 0 as uint32_t {
                    ys = 2 as uint32_t;
                    if range == 0 as uint32_t && xh.wrapping_rem(6 as uint32_t) == 0 as uint32_t
                        || range == 1 as uint32_t && xh == 0 as uint32_t
                        || range == 2 as uint32_t && xd == 1 as uint32_t
                    {
                        ys = 1 as uint32_t;
                    }
                    if range < 2 as uint32_t {
                        text[0 as usize] = xh.wrapping_div(10 as uint32_t) as uint8_t;
                        text[1 as usize] = xh.wrapping_rem(10 as uint32_t) as uint8_t;
                        text[2 as usize] = COLON as uint8_t;
                        text[3 as usize] = 0 as uint8_t;
                        text[4 as usize] = 0 as uint8_t;
                        charts_puttext(
                            XPOS as int32_t + i - 14 as int32_t,
                            (YPOS as uint32_t)
                                .wrapping_add(height)
                                .wrapping_add(4 as uint32_t) as int32_t,
                            COLOR_TEXT as uint8_t,
                            &raw mut text as *mut uint8_t,
                            5 as uint32_t,
                            XPOS as int32_t,
                            (XPOS as uint32_t)
                                .wrapping_add(width)
                                .wrapping_sub(1 as uint32_t) as int32_t,
                            0 as int32_t,
                            MAXYSIZE as int32_t - 1 as int32_t,
                        );
                    } else {
                        text[0 as usize] = xm.wrapping_div(10 as uint32_t) as uint8_t;
                        text[1 as usize] = xm.wrapping_rem(10 as uint32_t) as uint8_t;
                        text[2 as usize] = FDOT as uint8_t;
                        text[3 as usize] = xd.wrapping_div(10 as uint32_t) as uint8_t;
                        text[4 as usize] = xd.wrapping_rem(10 as uint32_t) as uint8_t;
                        charts_puttext(
                            XPOS as int32_t + i + 10 as int32_t,
                            (YPOS as uint32_t)
                                .wrapping_add(height)
                                .wrapping_add(4 as uint32_t) as int32_t,
                            COLOR_TEXT as uint8_t,
                            &raw mut text as *mut uint8_t,
                            5 as uint32_t,
                            XPOS as int32_t,
                            (XPOS as uint32_t)
                                .wrapping_add(width)
                                .wrapping_sub(1 as uint32_t) as int32_t,
                            0 as int32_t,
                            MAXYSIZE as int32_t - 1 as int32_t,
                        );
                        xd = xd.wrapping_sub(1);
                        if xd == 0 as uint32_t {
                            xm = xm.wrapping_sub(1);
                            if xm == 0 as uint32_t {
                                xm = 12 as uint32_t;
                                xy = xy.wrapping_sub(1);
                            }
                            xd = getmonleng(xy, xm);
                        }
                    }
                    *chart.offset(
                        (MAXXSIZE as uint32_t)
                            .wrapping_mul(
                                (YPOS as uint32_t)
                                    .wrapping_add(height)
                                    .wrapping_add(1 as uint32_t),
                            )
                            .wrapping_add((i + XPOS as int32_t) as uint32_t)
                            as isize,
                    ) = COLOR_AXIS as uint8_t;
                    *chart.offset(
                        (MAXXSIZE as uint32_t)
                            .wrapping_mul(
                                (YPOS as uint32_t)
                                    .wrapping_add(height)
                                    .wrapping_add(2 as uint32_t),
                            )
                            .wrapping_add((i + XPOS as int32_t) as uint32_t)
                            as isize,
                    ) = COLOR_AXIS as uint8_t;
                } else {
                    ys = 4 as uint32_t;
                }
                j = 0 as ::core::ffi::c_int as int32_t;
                while j < height as int32_t {
                    if ys > 1 as uint32_t || j % 4 as int32_t != 0 as int32_t {
                        *chart.offset(
                            (MAXXSIZE as int32_t * (j + YPOS as int32_t) + (i + XPOS as int32_t))
                                as isize,
                        ) = COLOR_AUX as uint8_t;
                    }
                    j = (j as uint32_t).wrapping_add(ys) as int32_t;
                }
                if range < 2 as uint32_t {
                    if xh == 0 as uint32_t {
                        xh = 23 as uint32_t;
                    } else {
                        xh = xh.wrapping_sub(1);
                    }
                } else if xh == 0 as uint32_t {
                    xh = 3 as uint32_t;
                } else {
                    xh = xh.wrapping_sub(1);
                }
                i = (i as uint32_t).wrapping_sub(xs) as int32_t;
            }
            if range == 2 as uint32_t {
                i = (i as uint32_t).wrapping_sub(xs.wrapping_mul(xh)) as int32_t;
                text[0 as usize] = xm.wrapping_div(10 as uint32_t) as uint8_t;
                text[1 as usize] = xm.wrapping_rem(10 as uint32_t) as uint8_t;
                text[2 as usize] = FDOT as uint8_t;
                text[3 as usize] = xd.wrapping_div(10 as uint32_t) as uint8_t;
                text[4 as usize] = xd.wrapping_rem(10 as uint32_t) as uint8_t;
                charts_puttext(
                    XPOS as int32_t + i + 10 as int32_t,
                    (YPOS as uint32_t)
                        .wrapping_add(height)
                        .wrapping_add(4 as uint32_t) as int32_t,
                    COLOR_TEXT as uint8_t,
                    &raw mut text as *mut uint8_t,
                    5 as uint32_t,
                    XPOS as int32_t,
                    (XPOS as uint32_t)
                        .wrapping_add(width)
                        .wrapping_sub(1 as uint32_t) as int32_t,
                    0 as int32_t,
                    MAXYSIZE as int32_t - 1 as int32_t,
                );
            }
        } else {
            xy = lngyear;
            xm = lngmonth;
            i = width.wrapping_sub(lngmday) as int32_t;
            while i >= 0 as int32_t {
                text[0 as usize] = xm.wrapping_div(10 as uint32_t) as uint8_t;
                text[1 as usize] = xm.wrapping_rem(10 as uint32_t) as uint8_t;
                charts_puttext(
                    ((XPOS as int32_t + i) as uint32_t)
                        .wrapping_add(
                            getmonleng(xy, xm)
                                .wrapping_sub(11 as uint32_t)
                                .wrapping_div(2 as uint32_t),
                        )
                        .wrapping_add(1 as uint32_t) as int32_t,
                    (YPOS as uint32_t)
                        .wrapping_add(height)
                        .wrapping_add(4 as uint32_t) as int32_t,
                    COLOR_TEXT as uint8_t,
                    &raw mut text as *mut uint8_t,
                    2 as uint32_t,
                    XPOS as int32_t,
                    (XPOS as uint32_t)
                        .wrapping_add(width)
                        .wrapping_sub(1 as uint32_t) as int32_t,
                    0 as int32_t,
                    MAXYSIZE as int32_t - 1 as int32_t,
                );
                *chart.offset(
                    (MAXXSIZE as uint32_t)
                        .wrapping_mul(
                            (YPOS as uint32_t)
                                .wrapping_add(height)
                                .wrapping_add(1 as uint32_t),
                        )
                        .wrapping_add((i + XPOS as int32_t) as uint32_t)
                        as isize,
                ) = COLOR_AXIS as uint8_t;
                *chart.offset(
                    (MAXXSIZE as uint32_t)
                        .wrapping_mul(
                            (YPOS as uint32_t)
                                .wrapping_add(height)
                                .wrapping_add(2 as uint32_t),
                        )
                        .wrapping_add((i + XPOS as int32_t) as uint32_t)
                        as isize,
                ) = COLOR_AXIS as uint8_t;
                if xm != 1 as uint32_t {
                    j = 0 as ::core::ffi::c_int as int32_t;
                    while j < height as int32_t {
                        *chart.offset(
                            (MAXXSIZE as int32_t * (j + YPOS as int32_t) + (i + XPOS as int32_t))
                                as isize,
                        ) = COLOR_AUX as uint8_t;
                        j = (j as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as int32_t;
                    }
                } else {
                    j = 0 as ::core::ffi::c_int as int32_t;
                    while j < height as int32_t {
                        if j % 4 as int32_t != 0 as int32_t {
                            *chart.offset(
                                (MAXXSIZE as int32_t * (j + YPOS as int32_t)
                                    + (i + XPOS as int32_t))
                                    as isize,
                            ) = COLOR_AUX as uint8_t;
                        }
                        j += 1;
                    }
                }
                xm = xm.wrapping_sub(1);
                if xm == 0 as uint32_t {
                    xm = 12 as uint32_t;
                    xy = xy.wrapping_sub(1);
                }
                i = (i as uint32_t).wrapping_sub(getmonleng(xy, xm)) as int32_t;
            }
            text[0 as usize] = xm.wrapping_div(10 as uint32_t) as uint8_t;
            text[1 as usize] = xm.wrapping_rem(10 as uint32_t) as uint8_t;
            charts_puttext(
                ((XPOS as int32_t + i) as uint32_t)
                    .wrapping_add(
                        getmonleng(xy, xm)
                            .wrapping_sub(11 as uint32_t)
                            .wrapping_div(2 as uint32_t),
                    )
                    .wrapping_add(1 as uint32_t) as int32_t,
                (YPOS as uint32_t)
                    .wrapping_add(height)
                    .wrapping_add(4 as uint32_t) as int32_t,
                COLOR_TEXT as uint8_t,
                &raw mut text as *mut uint8_t,
                2 as uint32_t,
                XPOS as int32_t,
                (XPOS as uint32_t)
                    .wrapping_add(width)
                    .wrapping_sub(1 as uint32_t) as int32_t,
                0 as int32_t,
                MAXYSIZE as int32_t - 1 as int32_t,
            );
        }
        i = 0 as ::core::ffi::c_int as int32_t;
        while i <= ypts as int32_t {
            d = (base as int32_t * i) as uint64_t;
            j = 0 as ::core::ffi::c_int as int32_t;
            if mode as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                if d >= 10 as uint64_t {
                    if d >= 100 as uint64_t {
                        let c2rust_fresh0 = j;
                        j += 1;
                        text[c2rust_fresh0 as usize] = d.wrapping_div(100 as uint64_t) as uint8_t;
                        d = d.wrapping_rem(100 as uint64_t);
                    }
                    let c2rust_fresh1 = j;
                    j += 1;
                    text[c2rust_fresh1 as usize] = d.wrapping_div(10 as uint64_t) as uint8_t;
                }
                let c2rust_fresh2 = j;
                j += 1;
                text[c2rust_fresh2 as usize] = d.wrapping_rem(10 as uint64_t) as uint8_t;
            } else if mode as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                if d >= 100 as uint64_t {
                    let c2rust_fresh3 = j;
                    j += 1;
                    text[c2rust_fresh3 as usize] = d.wrapping_div(100 as uint64_t) as uint8_t;
                    d = d.wrapping_rem(100 as uint64_t);
                }
                let c2rust_fresh4 = j;
                j += 1;
                text[c2rust_fresh4 as usize] = d.wrapping_div(10 as uint64_t) as uint8_t;
                let c2rust_fresh5 = j;
                j += 1;
                text[c2rust_fresh5 as usize] = FDOT as uint8_t;
                let c2rust_fresh6 = j;
                j += 1;
                text[c2rust_fresh6 as usize] = d.wrapping_rem(10 as uint64_t) as uint8_t;
            } else if mode as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                let c2rust_fresh7 = j;
                j += 1;
                text[c2rust_fresh7 as usize] = d.wrapping_div(100 as uint64_t) as uint8_t;
                d = d.wrapping_rem(100 as uint64_t);
                let c2rust_fresh8 = j;
                j += 1;
                text[c2rust_fresh8 as usize] = FDOT as uint8_t;
                let c2rust_fresh9 = j;
                j += 1;
                text[c2rust_fresh9 as usize] = d.wrapping_div(10 as uint64_t) as uint8_t;
                let c2rust_fresh10 = j;
                j += 1;
                text[c2rust_fresh10 as usize] = d.wrapping_rem(10 as uint64_t) as uint8_t;
            }
            if (scale as ::core::ffi::c_int) < 11 as ::core::ffi::c_int {
                if jtab[scale as usize] as ::core::ffi::c_int != SPACE {
                    let c2rust_fresh11 = j;
                    j += 1;
                    text[c2rust_fresh11 as usize] = jtab[scale as usize];
                }
            } else {
                let c2rust_fresh12 = j;
                j += 1;
                text[c2rust_fresh12 as usize] = SQUARE as uint8_t;
            }
            if percent != 0 {
                let c2rust_fresh13 = j;
                j += 1;
                text[c2rust_fresh13 as usize] = PERCENT as uint8_t;
            }
            charts_puttext(
                XPOS as int32_t - 4 as int32_t - j * 6 as int32_t,
                (YPOS as uint32_t)
                    .wrapping_add(height)
                    .wrapping_sub((20 as int32_t * i) as uint32_t)
                    .wrapping_sub(3 as uint32_t) as int32_t,
                COLOR_TEXT as uint8_t,
                &raw mut text as *mut uint8_t,
                j as uint32_t,
                0 as int32_t,
                MAXXSIZE as int32_t - 1 as int32_t,
                0 as int32_t,
                MAXYSIZE as int32_t - 1 as int32_t,
            );
            *chart.offset(
                (MAXXSIZE as uint32_t)
                    .wrapping_mul(
                        (YPOS as uint32_t)
                            .wrapping_add(height)
                            .wrapping_sub((20 as int32_t * i) as uint32_t),
                    )
                    .wrapping_add((XPOS - 2 as ::core::ffi::c_int) as uint32_t)
                    as isize,
            ) = COLOR_AXIS as uint8_t;
            *chart.offset(
                (MAXXSIZE as uint32_t)
                    .wrapping_mul(
                        (YPOS as uint32_t)
                            .wrapping_add(height)
                            .wrapping_sub((20 as int32_t * i) as uint32_t),
                    )
                    .wrapping_add((XPOS - 3 as ::core::ffi::c_int) as uint32_t)
                    as isize,
            ) = COLOR_AXIS as uint8_t;
            if i > 0 as int32_t {
                j = 1 as ::core::ffi::c_int as int32_t;
                while j < width as int32_t {
                    *chart.offset(
                        (MAXXSIZE as uint32_t)
                            .wrapping_mul(
                                (YPOS as uint32_t)
                                    .wrapping_add(height)
                                    .wrapping_sub((20 as int32_t * i) as uint32_t),
                            )
                            .wrapping_add((XPOS as int32_t + j) as uint32_t)
                            as isize,
                    ) = COLOR_AUX as uint8_t;
                    j = (j as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as int32_t;
                }
            }
            i += 1;
        }
        if !c3dispdata.is_null() {
            free(c3dispdata as *mut ::core::ffi::c_void);
        }
        if !c2dispdata.is_null() {
            free(c2dispdata as *mut ::core::ffi::c_void);
        }
        if !c1dispdata.is_null() {
            free(c1dispdata as *mut ::core::ffi::c_void);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_monotonic_data(mut buff: *mut uint8_t) -> uint32_t {
    unsafe {
        let mut i: uint32_t = 0;
        if buff.is_null() {
            return ::core::mem::size_of::<uint16_t>().wrapping_add(
                ::core::mem::size_of::<uint64_t>().wrapping_mul(statdefscount as usize),
            ) as uint32_t;
        }
        put16bit(&raw mut buff, statdefscount as uint16_t);
        i = 0 as uint32_t;
        while i < statdefscount {
            put64bit(&raw mut buff, *monotonic.offset(i as isize));
            i = i.wrapping_add(1);
        }
        return 0 as uint32_t;
    }
}
#[inline]
unsafe extern "C" fn charts_statid_converter(
    mut number: uint32_t,
    mut chtype: *mut uint32_t,
    mut chrange: *mut uint32_t,
) {
    unsafe {
        let mut rmask: uint32_t = 0;
        let mut i: uint32_t = 0;
        if number < 0x1000000 as uint32_t {
            *chtype = number.wrapping_div(10 as uint32_t);
            *chrange = number.wrapping_rem(10 as uint32_t);
            return;
        } else {
            rmask = number & 0x20202020 as uint32_t;
            rmask = rmask >> 5 as ::core::ffi::c_int & 1 as uint32_t
                | rmask >> 12 as ::core::ffi::c_int & 2 as uint32_t
                | rmask >> 19 as ::core::ffi::c_int & 4 as uint32_t
                | rmask >> 26 as ::core::ffi::c_int & 8 as uint32_t;
            number =
                (number as ::core::ffi::c_uint & 0xdfdfdfdf as ::core::ffi::c_uint) as uint32_t;
            i = 0 as uint32_t;
            while i < statdefscount {
                if (*statdefs.offset(i as isize)).statid & 0xdfdfdfdf as uint32_t == number {
                    *chtype = i;
                    *chrange = rmask;
                    return;
                }
                i = i.wrapping_add(1);
            }
            i = 0 as uint32_t;
            while i < estatdefscount {
                if (*estatdefs.offset(i as isize)).statid & 0xdfdfdfdf as uint32_t == number {
                    *chtype = (CHARTS_EXTENDED_START as uint32_t).wrapping_add(i);
                    *chrange = rmask;
                    return;
                }
                i = i.wrapping_add(1);
            }
        }
        *chtype = 0xffffffff as ::core::ffi::c_uint as uint32_t;
        *chrange = 0xffffffff as ::core::ffi::c_uint as uint32_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_data_multiplier(
    mut r#type: uint32_t,
    mut range: uint32_t,
) -> ::core::ffi::c_double {
    unsafe {
        let mut ret: ::core::ffi::c_double = 0.;
        ret = 1.0f64;
        if r#type < statdefscount
            && (*statdefs.offset(r#type as isize)).mode as ::core::ffi::c_int == CHARTS_MODE_ADD
            || r#type >= CHARTS_EXTENDED_START as uint32_t
                && r#type < (CHARTS_EXTENDED_START as uint32_t).wrapping_add(estatdefscount)
                && (*estatdefs
                    .offset(r#type.wrapping_sub(CHARTS_EXTENDED_START as uint32_t) as isize))
                .mode as ::core::ffi::c_int
                    == CHARTS_MODE_ADD
        {
            match range {
                1 => {
                    ret /= 6.0f64;
                }
                2 => {
                    ret /= 30.0f64;
                }
                3 => {
                    ret /= 1440.0f64;
                }
                _ => {}
            }
        }
        if r#type < statdefscount {
            ret *= (*statdefs.offset(r#type as isize)).multiplier as ::core::ffi::c_int
                as ::core::ffi::c_double;
            ret /= (*statdefs.offset(r#type as isize)).divisor as ::core::ffi::c_int
                as ::core::ffi::c_double;
            match (*statdefs.offset(r#type as isize)).scale as ::core::ffi::c_int {
                CHARTS_SCALE_MICRO => {
                    ret /= 1000000.0f64;
                }
                CHARTS_SCALE_MILI => {
                    ret /= 1000.0f64;
                }
                CHARTS_SCALE_KILO => {
                    ret *= 1000.0f64;
                }
                CHARTS_SCALE_MEGA => {
                    ret *= 1000000.0f64;
                }
                CHARTS_SCALE_GIGA => {
                    ret *= 1000000000.0f64;
                }
                _ => {}
            }
        } else if r#type >= CHARTS_EXTENDED_START as uint32_t
            && r#type < (CHARTS_EXTENDED_START as uint32_t).wrapping_add(estatdefscount)
        {
            ret *= (*estatdefs
                .offset(r#type.wrapping_sub(CHARTS_EXTENDED_START as uint32_t) as isize))
            .multiplier as ::core::ffi::c_int as ::core::ffi::c_double;
            ret /= (*estatdefs
                .offset(r#type.wrapping_sub(CHARTS_EXTENDED_START as uint32_t) as isize))
            .divisor as ::core::ffi::c_int as ::core::ffi::c_double;
            match (*estatdefs
                .offset(r#type.wrapping_sub(CHARTS_EXTENDED_START as uint32_t) as isize))
            .scale as ::core::ffi::c_int
            {
                CHARTS_SCALE_MICRO => {
                    ret /= 1000000.0f64;
                }
                CHARTS_SCALE_MILI => {
                    ret /= 1000.0f64;
                }
                CHARTS_SCALE_KILO => {
                    ret *= 1000.0f64;
                }
                CHARTS_SCALE_MEGA => {
                    ret *= 1000000.0f64;
                }
                CHARTS_SCALE_GIGA => {
                    ret *= 1000000000.0f64;
                }
                _ => {}
            }
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_getmaxleng() -> uint32_t {
    return MAXLENG as uint32_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_getdata(
    mut data: *mut ::core::ffi::c_double,
    mut timestamp: *mut uint32_t,
    mut rsec: *mut uint32_t,
    mut number: uint32_t,
) {
    unsafe {
        let mut i: uint32_t = 0;
        let mut chtype: uint32_t = 0;
        let mut chrange: uint32_t = 0;
        let mut tab1: *mut uint64_t = ::core::ptr::null_mut::<uint64_t>();
        let mut tab2: *mut uint64_t = ::core::ptr::null_mut::<uint64_t>();
        let mut tab3: *mut uint64_t = ::core::ptr::null_mut::<uint64_t>();
        let mut d: uint64_t = 0;
        let mut c: uint32_t = 0;
        let mut nd: uint8_t = 0;
        let mut mul: ::core::ffi::c_double = 0.;
        tab1 = ::core::ptr::null_mut::<uint64_t>();
        tab2 = ::core::ptr::null_mut::<uint64_t>();
        tab3 = ::core::ptr::null_mut::<uint64_t>();
        charts_statid_converter(number, &raw mut chtype, &raw mut chrange);
        if !data.is_null()
            && !timestamp.is_null()
            && chrange < RANGES as uint32_t
            && (chtype < statdefscount
                || chtype >= CHARTS_EXTENDED_START as uint32_t
                    && chtype < (CHARTS_EXTENDED_START as uint32_t).wrapping_add(estatdefscount))
        {
            mul = charts_data_multiplier(chtype, chrange);
            c = 0 as uint32_t;
            if charts_filltab(
                &raw mut tab1,
                chrange,
                chtype,
                1 as uint32_t,
                MAXLENG as uint32_t,
            ) != 0
            {
                c = 1 as uint32_t;
            }
            if charts_filltab(
                &raw mut tab2,
                chrange,
                chtype,
                2 as uint32_t,
                MAXLENG as uint32_t,
            ) != 0
            {
                c = 2 as uint32_t;
            }
            if charts_filltab(
                &raw mut tab3,
                chrange,
                chtype,
                3 as uint32_t,
                MAXLENG as uint32_t,
            ) != 0
            {
                c = 3 as uint32_t;
            }
            match chrange {
                0 => {
                    *rsec = 60 as uint32_t;
                }
                1 => {
                    *rsec = (60 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as uint32_t;
                }
                2 => {
                    *rsec = (60 as ::core::ffi::c_int * 30 as ::core::ffi::c_int) as uint32_t;
                }
                3 => {
                    *rsec = (60 as ::core::ffi::c_int
                        * 60 as ::core::ffi::c_int
                        * 24 as ::core::ffi::c_int) as uint32_t;
                }
                _ => {}
            }
            *timestamp = timepoint[chrange as usize].wrapping_mul(*rsec);
            i = 0 as uint32_t;
            while i < MAXLENG as uint32_t {
                d = 0 as uint64_t;
                nd = 1 as uint8_t;
                if c >= 1 as uint32_t && *tab1.offset(i as isize) != 0xffffffffffffffff as uint64_t
                {
                    d = d.wrapping_add(*tab1.offset(i as isize));
                    nd = 0 as uint8_t;
                }
                if c >= 2 as uint32_t && *tab2.offset(i as isize) != 0xffffffffffffffff as uint64_t
                {
                    d = d.wrapping_add(*tab2.offset(i as isize));
                    nd = 0 as uint8_t;
                }
                if c >= 3 as uint32_t && *tab3.offset(i as isize) != 0xffffffffffffffff as uint64_t
                {
                    d = d.wrapping_add(*tab3.offset(i as isize));
                    nd = 0 as uint8_t;
                }
                if nd != 0 {
                    *data.offset(i as isize) = -1.0f64;
                } else {
                    *data.offset(i as isize) = d as ::core::ffi::c_double * mul;
                }
                i = i.wrapping_add(1);
            }
        }
        if !tab3.is_null() {
            free(tab3 as *mut ::core::ffi::c_void);
        }
        if !tab2.is_null() {
            free(tab2 as *mut ::core::ffi::c_void);
        }
        if !tab1.is_null() {
            free(tab1 as *mut ::core::ffi::c_void);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_makedata(
    mut buff: *mut uint8_t,
    mut number: uint32_t,
    mut maxentries: uint32_t,
    mut multimode: uint8_t,
) -> uint32_t {
    unsafe {
        let mut i: uint32_t = 0;
        let mut j: uint32_t = 0;
        let mut ts: uint32_t = 0;
        let mut chtype: uint32_t = 0;
        let mut chrange: uint32_t = 0;
        let mut chrfrom: uint8_t = 0;
        let mut chrto: uint8_t = 0;
        let mut tabptr: *mut uint64_t = ::core::ptr::null_mut::<uint64_t>();
        let mut scnt: uint8_t = 0;
        let mut tab: *mut uint64_t = ::core::ptr::null_mut::<uint64_t>();
        let mut cmul: uint32_t = 0;
        let mut cdiv: uint32_t = 0;
        let mut rdiv: uint32_t = 0;
        let mut perc: uint32_t = 0;
        let mut base: uint32_t = 0;
        let mut additive: uint32_t = 0;
        charts_statid_converter(number, &raw mut chtype, &raw mut chrange);
        if maxentries > MAXLENG as uint32_t {
            maxentries = MAXLENG as uint32_t;
        }
        if multimode != 0 {
            scnt = charts_seriescnt(chtype);
            if scnt as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return 0 as uint32_t;
            }
            if chrange == 9 as uint32_t {
                chrfrom = 0 as uint8_t;
                chrto = RANGES as uint8_t;
            } else if chrange < RANGES as uint32_t {
                chrfrom = chrange as uint8_t;
                chrto = chrange.wrapping_add(1 as uint32_t) as uint8_t;
            } else {
                return 0 as uint32_t;
            }
            if buff.is_null() {
                return (8 as uint32_t).wrapping_add(
                    ((chrto as ::core::ffi::c_int - chrfrom as ::core::ffi::c_int) as uint32_t)
                        .wrapping_mul(
                            (13 as uint32_t).wrapping_add(
                                (scnt as uint32_t)
                                    .wrapping_mul(maxentries)
                                    .wrapping_mul(8 as uint32_t),
                            ),
                        ),
                );
            }
            if chtype < statdefscount {
                base = (*statdefs.offset(chtype as isize)).scale as uint32_t;
                perc = (*statdefs.offset(chtype as isize)).percent as uint32_t;
                cmul = (*statdefs.offset(chtype as isize)).multiplier as uint32_t;
                cdiv = (*statdefs.offset(chtype as isize)).divisor as uint32_t;
                additive = (if (*statdefs.offset(chtype as isize)).mode as ::core::ffi::c_int
                    == CHARTS_MODE_ADD
                {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as uint32_t;
            } else if chtype >= CHARTS_EXTENDED_START as uint32_t
                && chtype < (CHARTS_EXTENDED_START as uint32_t).wrapping_add(estatdefscount)
            {
                base = (*estatdefs
                    .offset(chtype.wrapping_sub(CHARTS_EXTENDED_START as uint32_t) as isize))
                .scale as uint32_t;
                perc = (*estatdefs
                    .offset(chtype.wrapping_sub(CHARTS_EXTENDED_START as uint32_t) as isize))
                .percent as uint32_t;
                cmul = (*estatdefs
                    .offset(chtype.wrapping_sub(CHARTS_EXTENDED_START as uint32_t) as isize))
                .multiplier as uint32_t;
                cdiv = (*estatdefs
                    .offset(chtype.wrapping_sub(CHARTS_EXTENDED_START as uint32_t) as isize))
                .divisor as uint32_t;
                additive = (if (*estatdefs
                    .offset(chtype.wrapping_sub(CHARTS_EXTENDED_START as uint32_t) as isize))
                .mode as ::core::ffi::c_int
                    == CHARTS_MODE_ADD
                {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as uint32_t;
            } else {
                base = CHARTS_SCALE_NONE as uint32_t;
                perc = 0 as uint32_t;
                cmul = 1 as uint32_t;
                cdiv = 1 as uint32_t;
                additive = 0 as uint32_t;
            }
            put8bit(
                &raw mut buff,
                (chrto as ::core::ffi::c_int - chrfrom as ::core::ffi::c_int) as uint8_t,
            );
            put8bit(&raw mut buff, scnt);
            put32bit(&raw mut buff, maxentries);
            put8bit(&raw mut buff, perc as uint8_t);
            put8bit(&raw mut buff, base as uint8_t);
            chrange = chrfrom as uint32_t;
            while chrange < chrto as uint32_t {
                put8bit(&raw mut buff, chrange as uint8_t);
                ts = timepoint[chrange as usize];
                match chrange {
                    0 => {
                        ts = ts.wrapping_mul(60 as uint32_t);
                    }
                    1 => {
                        ts = ts.wrapping_mul(
                            (60 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as uint32_t,
                        );
                    }
                    2 => {
                        ts = ts.wrapping_mul(
                            (60 as ::core::ffi::c_int * 30 as ::core::ffi::c_int) as uint32_t,
                        );
                    }
                    3 => {
                        ts = ts.wrapping_mul(
                            (60 as ::core::ffi::c_int
                                * 60 as ::core::ffi::c_int
                                * 24 as ::core::ffi::c_int) as uint32_t,
                        );
                    }
                    _ => {}
                }
                put32bit(&raw mut buff, ts);
                rdiv = cdiv;
                if additive != 0 {
                    match chrange {
                        1 => {
                            rdiv = rdiv.wrapping_mul(6 as uint32_t);
                        }
                        2 => {
                            rdiv = rdiv.wrapping_mul(30 as uint32_t);
                        }
                        3 => {
                            rdiv = rdiv.wrapping_mul(1440 as uint32_t);
                        }
                        _ => {}
                    }
                }
                put32bit(&raw mut buff, cmul);
                put32bit(&raw mut buff, rdiv);
                tab = ::core::ptr::null_mut::<uint64_t>();
                i = 0 as uint32_t;
                while i < scnt as uint32_t {
                    if charts_filltab(
                        &raw mut tab,
                        chrange,
                        chtype,
                        (scnt as uint32_t).wrapping_sub(i),
                        maxentries,
                    ) != 0
                    {
                        j = 0 as uint32_t;
                        while j < maxentries {
                            put64bit(
                                &raw mut buff,
                                *tab.offset(
                                    maxentries.wrapping_sub(1 as uint32_t).wrapping_sub(j) as isize
                                ),
                            );
                            j = j.wrapping_add(1);
                        }
                    } else {
                        j = 0 as uint32_t;
                        while j < maxentries {
                            put64bit(&raw mut buff, 0 as uint64_t);
                            j = j.wrapping_add(1);
                        }
                    }
                    i = i.wrapping_add(1);
                }
                if !tab.is_null() {
                    free(tab as *mut ::core::ffi::c_void);
                }
                chrange = chrange.wrapping_add(1);
            }
        } else {
            if buff.is_null() {
                return if chrange < RANGES as uint32_t && chtype < statdefscount {
                    maxentries
                        .wrapping_mul(8 as uint32_t)
                        .wrapping_add(8 as uint32_t)
                } else {
                    0 as uint32_t
                };
            }
            if chrange < RANGES as uint32_t && chtype < statdefscount {
                tabptr = (*series.offset(chtype as isize))[chrange as usize];
                j = pointers[chrange as usize].wrapping_rem(MAXLENG as uint32_t);
                ts = timepoint[chrange as usize];
                match chrange {
                    0 => {
                        ts = ts.wrapping_mul(60 as uint32_t);
                    }
                    1 => {
                        ts = ts.wrapping_mul(
                            (60 as ::core::ffi::c_int * 6 as ::core::ffi::c_int) as uint32_t,
                        );
                    }
                    2 => {
                        ts = ts.wrapping_mul(
                            (60 as ::core::ffi::c_int * 30 as ::core::ffi::c_int) as uint32_t,
                        );
                    }
                    3 => {
                        ts = ts.wrapping_mul(
                            (60 as ::core::ffi::c_int
                                * 60 as ::core::ffi::c_int
                                * 24 as ::core::ffi::c_int) as uint32_t,
                        );
                    }
                    _ => {}
                }
                put32bit(&raw mut buff, ts);
                put32bit(&raw mut buff, maxentries);
                i = 0 as uint32_t;
                while i < maxentries {
                    put64bit(&raw mut buff, *tabptr.offset(j as isize));
                    if j > 0 as uint32_t {
                        j = j.wrapping_sub(1);
                    } else {
                        j = (MAXLENG - 1 as ::core::ffi::c_int) as uint32_t;
                    }
                    i = i.wrapping_add(1);
                }
            }
        }
        return 0 as uint32_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_chart_to_rawchart(
    mut chartwidth: uint32_t,
    mut chartheight: uint32_t,
) {
    unsafe {
        let mut y: uint32_t = 0;
        let mut cp: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut rp: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        cp = chart;
        rp = rawchart;
        y = 0 as uint32_t;
        while y < chartheight {
            *rp = 0 as uint8_t;
            rp = rp.offset(1);
            memcpy(
                rp as *mut ::core::ffi::c_void,
                cp as *const ::core::ffi::c_void,
                chartwidth as size_t,
            );
            rp = rp.offset(chartwidth as isize);
            cp = cp.offset(MAXXSIZE as isize);
            y = y.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_fill_crc(mut buff: *mut uint8_t, mut leng: uint32_t) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut eptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut crc: uint32_t = 0;
        let mut chleng: uint32_t = 0;
        ptr = buff.offset(8 as ::core::ffi::c_int as isize);
        eptr = buff.offset(leng as isize);
        while ptr.offset(4 as ::core::ffi::c_int as isize) <= eptr {
            chleng = get32bit(&raw mut ptr as *mut *const uint8_t);
            if ptr
                .offset(8 as ::core::ffi::c_int as isize)
                .offset(chleng as isize)
                <= eptr
            {
                crc = mycrc32(
                    0 as uint32_t,
                    ptr as *const ::core::ffi::c_void,
                    chleng.wrapping_add(4 as uint32_t),
                );
                ptr = ptr.offset(chleng.wrapping_add(4 as uint32_t) as isize);
                if memcmp(
                    ptr as *const ::core::ffi::c_void,
                    b"CRC#\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                    4 as size_t,
                ) == 0 as ::core::ffi::c_int
                {
                    put32bit(&raw mut ptr, crc);
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"charts: unexpected data in generated png stream\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_make_png(
    mut number: uint32_t,
    mut chartwidth: uint32_t,
    mut chartheight: uint32_t,
) -> uint32_t {
    unsafe {
        let mut chtype: uint32_t = 0;
        let mut chrange: uint32_t = 0;
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        charts_statid_converter(number, &raw mut chtype, &raw mut chrange);
        if chrange >= RANGES as uint32_t {
            compsize = 0 as uint32_t;
            return ::core::mem::size_of::<[uint8_t; 68]>() as uint32_t;
        }
        if !(chtype < statdefscount
            || chtype >= CHARTS_EXTENDED_START as uint32_t
                && chtype < (CHARTS_EXTENDED_START as uint32_t).wrapping_add(estatdefscount))
        {
            compsize = 0 as uint32_t;
            return ::core::mem::size_of::<[uint8_t; 68]>() as uint32_t;
        }
        if chartwidth == 0 as uint32_t && chartheight == 0 as uint32_t {
            chartwidth = (950 as ::core::ffi::c_int + XADD) as uint32_t;
            chartheight = (100 as ::core::ffi::c_int + YADD) as uint32_t;
        }
        if chartheight > (MAXHEIGHT + YADD) as uint32_t {
            chartheight = (MAXHEIGHT + YADD) as uint32_t;
        }
        if chartheight < (MINHEIGHT + YADD) as uint32_t {
            chartheight = (MINHEIGHT + YADD) as uint32_t;
        }
        if chartheight
            .wrapping_sub(YADD as uint32_t)
            .wrapping_rem(20 as uint32_t)
            != 0 as uint32_t
        {
            chartheight = chartheight.wrapping_sub(
                chartheight
                    .wrapping_sub(YADD as uint32_t)
                    .wrapping_rem(20 as uint32_t),
            );
        }
        if chartwidth > (MAXLENG + XADD) as uint32_t {
            chartwidth = (MAXLENG + XADD) as uint32_t;
        }
        if chartwidth < (MINLENG + XADD) as uint32_t {
            chartwidth = (MINLENG + XADD) as uint32_t;
        }
        charts_makechart(
            chtype,
            chrange,
            chartwidth.wrapping_sub(XADD as uint32_t),
            chartheight.wrapping_sub(YADD as uint32_t),
        );
        charts_chart_to_rawchart(chartwidth, chartheight);
        ptr = (&raw mut png_header as *mut uint8_t).offset(16 as ::core::ffi::c_int as isize);
        put32bit(&raw mut ptr, chartwidth);
        put32bit(&raw mut ptr, chartheight);
        if deflateReset(&raw mut zstr) != Z_OK {
            compsize = 0 as uint32_t;
            return ::core::mem::size_of::<[uint8_t; 68]>() as uint32_t;
        }
        zstr.next_in = rawchart as *mut Bytef;
        zstr.avail_in = rawchartsize as uInt;
        zstr.total_in = 0 as uLong;
        zstr.next_out = compbuff as *mut Bytef;
        zstr.avail_out = compbuffsize as uInt;
        zstr.total_out = 0 as uLong;
        if deflate(&raw mut zstr, Z_FINISH) != Z_STREAM_END {
            compsize = 0 as uint32_t;
            return ::core::mem::size_of::<[uint8_t; 68]>() as uint32_t;
        }
        compsize = zstr.total_out as uint32_t;
        return ::core::mem::size_of::<[uint8_t; 847]>()
            .wrapping_add(compsize as usize)
            .wrapping_add(::core::mem::size_of::<[uint8_t; 16]>()) as uint32_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn charts_get_png(mut buff: *mut uint8_t) {
    unsafe {
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if compsize == 0 as uint32_t {
            memcpy(
                buff as *mut ::core::ffi::c_void,
                &raw mut png_1x1 as *mut uint8_t as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[uint8_t; 68]>(),
            );
        } else {
            memcpy(
                buff as *mut ::core::ffi::c_void,
                &raw mut png_header as *mut uint8_t as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[uint8_t; 847]>(),
            );
            ptr = buff
                .offset(::core::mem::size_of::<[uint8_t; 847]>().wrapping_sub(8 as usize) as isize);
            put32bit(&raw mut ptr, compsize);
            memcpy(
                buff.offset(::core::mem::size_of::<[uint8_t; 847]>() as isize)
                    as *mut ::core::ffi::c_void,
                compbuff as *const ::core::ffi::c_void,
                compsize as size_t,
            );
            memcpy(
                buff.offset(::core::mem::size_of::<[uint8_t; 847]>() as isize)
                    .offset(compsize as isize) as *mut ::core::ffi::c_void,
                &raw mut png_tailer as *mut uint8_t as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[uint8_t; 16]>(),
            );
            charts_fill_crc(
                buff,
                ::core::mem::size_of::<[uint8_t; 847]>()
                    .wrapping_add(compsize as usize)
                    .wrapping_add(::core::mem::size_of::<[uint8_t; 16]>())
                    as uint32_t,
            );
        }
        compsize = 0 as uint32_t;
    }
}
