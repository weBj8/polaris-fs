pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
    static mut stderr: *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type uint8_t = u8;
pub type uint32_t = u32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _iptosesid {
    pub ip: uint32_t,
    pub sessionid: uint32_t,
    pub time: ::core::ffi::c_double,
    pub next: *mut _iptosesid,
}
pub type iptosesid = _iptosesid;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const I2S_TIMEOUT: ::core::ffi::c_double = 1.0f64;
static mut head: *mut iptosesid = ::core::ptr::null_mut::<iptosesid>();
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iptosesid_add(mut ip: uint32_t, mut sessionid: uint32_t) {
    unsafe {
        let mut i2s: *mut iptosesid = ::core::ptr::null_mut::<iptosesid>();
        i2s = malloc(::core::mem::size_of::<iptosesid>()) as *mut iptosesid;
        if i2s.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/iptosesid.c\0".as_ptr() as *const ::core::ffi::c_char,
                44 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"i2s\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/iptosesid.c\0".as_ptr() as *const ::core::ffi::c_char,
                44 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"i2s\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if i2s
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut iptosesid
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/iptosesid.c\0".as_ptr() as *const ::core::ffi::c_char,
                44 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"i2s\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/iptosesid.c\0".as_ptr() as *const ::core::ffi::c_char,
                44 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"i2s\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*i2s).ip = ip;
        (*i2s).sessionid = sessionid;
        (*i2s).time = monotonic_seconds();
        (*i2s).next = head as *mut _iptosesid;
        head = i2s;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iptosesid_check(mut ip: uint32_t) -> uint8_t {
    unsafe {
        let mut i2s: *mut iptosesid = ::core::ptr::null_mut::<iptosesid>();
        let mut i2sp: *mut *mut iptosesid = ::core::ptr::null_mut::<*mut iptosesid>();
        let mut now: ::core::ffi::c_double = 0.;
        i2sp = &raw mut head;
        now = monotonic_seconds();
        loop {
            i2s = *i2sp;
            if i2s.is_null() {
                break;
            }
            if (*i2s).time + I2S_TIMEOUT < now {
                *i2sp = (*i2s).next as *mut iptosesid;
                free(i2s as *mut ::core::ffi::c_void);
            } else if (*i2s).ip == ip {
                return 1 as uint8_t;
            } else {
                i2sp = &raw mut (*i2s).next as *mut *mut iptosesid;
            }
        }
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iptosesid_get(mut ip: uint32_t) -> uint32_t {
    unsafe {
        let mut i2s: *mut iptosesid = ::core::ptr::null_mut::<iptosesid>();
        let mut i2sp: *mut *mut iptosesid = ::core::ptr::null_mut::<*mut iptosesid>();
        let mut sessionid: uint32_t = 0;
        let mut now: ::core::ffi::c_double = 0.;
        i2sp = &raw mut head;
        now = monotonic_seconds();
        loop {
            i2s = *i2sp;
            if i2s.is_null() {
                break;
            }
            if (*i2s).time + I2S_TIMEOUT < now {
                *i2sp = (*i2s).next as *mut iptosesid;
                free(i2s as *mut ::core::ffi::c_void);
            } else if (*i2s).ip == ip {
                *i2sp = (*i2s).next as *mut iptosesid;
                sessionid = (*i2s).sessionid;
                free(i2s as *mut ::core::ffi::c_void);
                return sessionid;
            } else {
                i2sp = &raw mut (*i2s).next as *mut *mut iptosesid;
            }
        }
        return 0 as uint32_t;
    }
}
