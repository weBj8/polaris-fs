pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
pub enum _bio {}
use ::c2rust_bitfields;
unsafe extern "C" {
    static mut stderr: *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn matoclserv_fuse_posix_lock_wake_up(
        veptr: *mut ::core::ffi::c_void,
        msgid: uint32_t,
        status: uint8_t,
    );
    unsafe fn bio_read(b: *mut bio, dst: *mut ::core::ffi::c_void, len: uint64_t) -> int64_t;
    unsafe fn bio_write(b: *mut bio, src: *const ::core::ffi::c_void, len: uint64_t) -> int64_t;
    unsafe fn of_checknode(sessionid: uint32_t, inode: uint32_t) -> uint8_t;
    unsafe fn meta_version_inc() -> uint64_t;
    unsafe fn main_reload_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn main_info_register_fname(
        fun: Option<unsafe extern "C" fn(*mut FILE) -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn main_time() -> uint32_t;
    unsafe fn cfg_getuint8(name: *const ::core::ffi::c_char, def: uint8_t) -> uint8_t;
    unsafe fn changelog(format: *const ::core::ffi::c_char, ...);
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
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type bio = _bio;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _range {
    pub start: uint64_t,
    pub end: uint64_t,
    pub r#type: uint8_t,
    pub next: *mut _range,
}
pub type range = _range;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _alock {
    pub owner: uint64_t,
    pub sessionid: uint32_t,
    pub pid: uint32_t,
    pub ranges: *mut range,
    pub next: *mut _alock,
}
pub type alock = _alock;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _wlock {
    pub owner: uint64_t,
    pub connptr: *mut ::core::ffi::c_void,
    pub sessionid: uint32_t,
    pub pid: uint32_t,
    pub msgid: uint32_t,
    pub reqid: uint32_t,
    pub start: uint64_t,
    pub end: uint64_t,
    pub r#type: uint8_t,
    pub next: *mut _wlock,
    pub prev: *mut *mut _wlock,
}
pub type wlock = _wlock;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _inodelocks {
    pub inode: uint32_t,
    pub active: *mut alock,
    pub waiting_head: *mut wlock,
    pub waiting_tail: *mut *mut wlock,
    pub next: *mut _inodelocks,
}
pub type inodelocks = _inodelocks;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_EINVAL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MFS_ERROR_NOTOPENED: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const MFS_ERROR_MISMATCH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MFS_ERROR_WAITING: ::core::ffi::c_int = 44 as ::core::ffi::c_int;
pub const MFS_ERROR_EAGAIN: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
pub const MFS_ERROR_EINTR: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const POSIX_LOCK_CMD_GET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const POSIX_LOCK_CMD_SET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const POSIX_LOCK_CMD_TRY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const POSIX_LOCK_CMD_INT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const POSIX_LOCK_UNLCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const POSIX_LOCK_RDLCK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const POSIX_LOCK_WRLCK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFS_LOCK_TYPE_UNKNOWN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_LOCK_TYPE_SHARED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFS_LOCK_TYPE_EXCLUSIVE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
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
unsafe extern "C" fn get8bit(mut ptr: *mut *const uint8_t) -> uint8_t {
    unsafe {
        let mut t8: uint8_t = 0;
        t8 = *(*ptr).offset(0 as isize);
        *ptr = (*ptr).offset(1);
        return t8;
    }
}
pub const POSIX_LOCK_INODE_HASHSIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
static mut inodehash: *mut *mut inodelocks = ::core::ptr::null_mut::<*mut inodelocks>();
static mut DebugInfo: uint8_t = 0;
#[inline]
unsafe extern "C" fn posix_lock_test_wlock(
    mut r: *mut range,
    mut r#type: *mut uint8_t,
    mut start: *mut uint64_t,
    mut end: *mut uint64_t,
) -> ::core::ffi::c_int {
    unsafe {
        while !r.is_null() {
            if *r#type as ::core::ffi::c_int == POSIX_LOCK_WRLCK
                || (*r).r#type as ::core::ffi::c_int == POSIX_LOCK_WRLCK
            {
                if *end > (*r).start && *start < (*r).end {
                    *r#type = (*r).r#type;
                    *start = (*r).start;
                    *end = (*r).end;
                    return 1 as ::core::ffi::c_int;
                }
            }
            r = (*r).next as *mut range;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn posix_lock_apply_range(
    mut rptr: *mut *mut range,
    mut r#type: uint8_t,
    mut start: uint64_t,
    mut end: uint64_t,
) {
    unsafe {
        let mut nr: *mut range = ::core::ptr::null_mut::<range>();
        let mut r: *mut range = ::core::ptr::null_mut::<range>();
        let mut added: uint8_t = 0;
        added = 0 as uint8_t;
        while added as ::core::ffi::c_int == 0 as ::core::ffi::c_int && {
            r = *rptr;
            !r.is_null()
        } {
            if (*r).end < start {
                rptr = &raw mut (*r).next as *mut *mut range;
            } else if (*r).start > end {
                if r#type as ::core::ffi::c_int != POSIX_LOCK_UNLCK {
                    nr = malloc(::core::mem::size_of::<range>()) as *mut range;
                    if nr.is_null() {
                        fprintf(
                            stderr,
                            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            164 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"nr\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            164 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"nr\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        abort();
                    } else if nr
                        == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                            -1 as ::core::ffi::c_int as usize,
                        ) as *mut range
                    {
                        let mut _mfs_errorstring: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            164 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"nr\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            164 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"nr\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring,
                        );
                        abort();
                    }
                    (*nr).start = start;
                    (*nr).end = end;
                    (*nr).r#type = r#type;
                    (*nr).next = *rptr as *mut _range;
                    *rptr = nr;
                }
                added = 1 as uint8_t;
            } else if start <= (*r).start && end >= (*r).end {
                *rptr = (*r).next as *mut range;
                free(r as *mut ::core::ffi::c_void);
            } else if (*r).start < start && (*r).end <= end {
                if (*r).r#type as ::core::ffi::c_int == r#type as ::core::ffi::c_int {
                    start = (*r).start;
                    *rptr = (*r).next as *mut range;
                    free(r as *mut ::core::ffi::c_void);
                } else {
                    (*r).end = start;
                    rptr = &raw mut (*r).next as *mut *mut range;
                }
            } else if (*r).start >= start && (*r).end > end {
                if (*r).r#type as ::core::ffi::c_int == r#type as ::core::ffi::c_int {
                    (*r).start = start;
                    added = 1 as uint8_t;
                } else {
                    (*r).start = end;
                    if r#type as ::core::ffi::c_int != POSIX_LOCK_UNLCK {
                        nr = malloc(::core::mem::size_of::<range>()) as *mut range;
                        if nr.is_null() {
                            fprintf(
                                stderr,
                                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                220 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"nr\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                220 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"nr\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            abort();
                        } else if nr
                            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                                -1 as ::core::ffi::c_int as usize,
                            ) as *mut range
                        {
                            let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                220 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"nr\0".as_ptr() as *const ::core::ffi::c_char,
                                _mfs_errorstring_0,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                220 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"nr\0".as_ptr() as *const ::core::ffi::c_char,
                                _mfs_errorstring_0,
                            );
                            abort();
                        }
                        (*nr).start = start;
                        (*nr).end = end;
                        (*nr).r#type = r#type;
                        (*nr).next = r as *mut _range;
                        *rptr = nr;
                    }
                    added = 1 as uint8_t;
                }
            } else {
                if (*r).r#type as ::core::ffi::c_int != r#type as ::core::ffi::c_int {
                    nr = malloc(::core::mem::size_of::<range>()) as *mut range;
                    if nr.is_null() {
                        fprintf(
                            stderr,
                            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            238 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"nr\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            238 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"nr\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        abort();
                    } else if nr
                        == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                            -1 as ::core::ffi::c_int as usize,
                        ) as *mut range
                    {
                        let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            238 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"nr\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring_1,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            238 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"nr\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring_1,
                        );
                        abort();
                    }
                    (*nr).start = end;
                    (*nr).end = (*r).end;
                    (*nr).r#type = (*r).r#type;
                    (*nr).next = (*r).next;
                    (*r).next = nr as *mut _range;
                    if r#type as ::core::ffi::c_int != POSIX_LOCK_UNLCK {
                        nr = malloc(::core::mem::size_of::<range>()) as *mut range;
                        if nr.is_null() {
                            fprintf(
                                stderr,
                                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                251 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"nr\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                251 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"nr\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            abort();
                        } else if nr
                            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                                -1 as ::core::ffi::c_int as usize,
                            ) as *mut range
                        {
                            let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                                strerr(*__errno_location());
                            mfs_log(
                                MFSLOG_SYSLOG,
                                MFSLOG_ERR,
                                b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                251 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"nr\0".as_ptr() as *const ::core::ffi::c_char,
                                _mfs_errorstring_2,
                            );
                            fprintf(
                                stderr,
                                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                251 as ::core::ffi::c_int as ::core::ffi::c_uint,
                                b"nr\0".as_ptr() as *const ::core::ffi::c_char,
                                _mfs_errorstring_2,
                            );
                            abort();
                        }
                        (*nr).start = start;
                        (*nr).end = end;
                        (*nr).r#type = r#type;
                        (*nr).next = (*r).next;
                        (*r).next = nr as *mut _range;
                    }
                    (*r).end = start;
                }
                added = 1 as uint8_t;
            }
        }
        if added as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && r#type as ::core::ffi::c_int != POSIX_LOCK_UNLCK
        {
            nr = malloc(::core::mem::size_of::<range>()) as *mut range;
            if nr.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    278 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"nr\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    278 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"nr\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if nr
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut range
            {
                let mut _mfs_errorstring_3: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    278 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"nr\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_3,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    278 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"nr\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_3,
                );
                abort();
            }
            (*nr).start = start;
            (*nr).end = end;
            (*nr).r#type = r#type;
            (*nr).next = ::core::ptr::null_mut::<_range>();
            *rptr = nr;
        }
    }
}
#[inline]
unsafe extern "C" fn posix_lock_inode_find(mut inode: uint32_t) -> *mut inodelocks {
    unsafe {
        let mut il: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        il = *inodehash.offset(
            inode
                .wrapping_mul(0x738a2379 as uint32_t)
                .wrapping_rem(POSIX_LOCK_INODE_HASHSIZE as uint32_t) as isize,
        );
        while !il.is_null() {
            if (*il).inode == inode {
                return il;
            }
            il = (*il).next as *mut inodelocks;
        }
        return ::core::ptr::null_mut::<inodelocks>();
    }
}
#[inline]
unsafe extern "C" fn posix_lock_inode_new(mut inode: uint32_t) -> *mut inodelocks {
    unsafe {
        let mut il: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        let mut hash: uint32_t = 0;
        il = malloc(::core::mem::size_of::<inodelocks>()) as *mut inodelocks;
        if il.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                305 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"il\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                305 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"il\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if il
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut inodelocks
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                305 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"il\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                305 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"il\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*il).inode = inode;
        (*il).active = ::core::ptr::null_mut::<alock>();
        (*il).waiting_head = ::core::ptr::null_mut::<wlock>();
        (*il).waiting_tail = &raw mut (*il).waiting_head;
        hash = inode
            .wrapping_mul(0x738a2379 as uint32_t)
            .wrapping_rem(POSIX_LOCK_INODE_HASHSIZE as uint32_t);
        (*il).next = *inodehash.offset(hash as isize) as *mut _inodelocks;
        *inodehash.offset(hash as isize) = il;
        return il;
    }
}
#[inline]
unsafe extern "C" fn posix_lock_inode_remove(mut inode: uint32_t) {
    unsafe {
        let mut il: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        let mut ilp: *mut *mut inodelocks = ::core::ptr::null_mut::<*mut inodelocks>();
        let mut hash: uint32_t = 0;
        hash = inode
            .wrapping_mul(0x738a2379 as uint32_t)
            .wrapping_rem(POSIX_LOCK_INODE_HASHSIZE as uint32_t);
        ilp = inodehash.offset(hash as isize);
        loop {
            il = *ilp;
            if il.is_null() {
                break;
            }
            if (*il).inode == inode {
                if (*il).active.is_null() && (*il).waiting_head.is_null() {
                } else {
                    fprintf(
                        stderr,
                        b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        324 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"il->active==NULL && il->waiting_head==NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"inode posix lock record not empty !!!\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - failed assertion '%s' : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        324 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"il->active==NULL && il->waiting_head==NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"inode posix lock record not empty !!!\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    abort();
                };
                *ilp = (*il).next as *mut inodelocks;
                free(il as *mut ::core::ffi::c_void);
            } else {
                ilp = &raw mut (*il).next as *mut *mut inodelocks;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn posix_lock_remove_lock(mut il: *mut inodelocks, mut wl: *mut wlock) {
    unsafe {
        if (*wl).next.is_null() {
            (*il).waiting_tail = (*wl).prev as *mut *mut wlock;
        } else {
            (*(*wl).next).prev = (*wl).prev;
        }
        *(*wl).prev = (*wl).next;
        free(wl as *mut ::core::ffi::c_void);
    }
}
#[inline]
unsafe extern "C" fn posix_lock_get_offensive_lock(
    mut il: *mut inodelocks,
    mut sessionid: uint32_t,
    mut owner: uint64_t,
    mut r#type: *mut uint8_t,
    mut start: *mut uint64_t,
    mut end: *mut uint64_t,
    mut pid: *mut uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut al: *mut alock = ::core::ptr::null_mut::<alock>();
        al = (*il).active;
        while !al.is_null() {
            if (*al).owner != owner || (*al).sessionid != sessionid {
                if posix_lock_test_wlock((*al).ranges, r#type, start, end) != 0 {
                    if sessionid == (*al).sessionid {
                        *pid = (*al).pid;
                    } else {
                        *pid = 0 as uint32_t;
                    }
                    return 1 as ::core::ffi::c_int;
                }
            }
            al = (*al).next as *mut alock;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn posix_lock_find_offensive_lock(
    mut il: *mut inodelocks,
    mut sessionid: uint32_t,
    mut owner: uint64_t,
    mut r#type: uint8_t,
    mut start: uint64_t,
    mut end: uint64_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut al: *mut alock = ::core::ptr::null_mut::<alock>();
        al = (*il).active;
        while !al.is_null() {
            if (*al).owner != owner || (*al).sessionid != sessionid {
                if posix_lock_test_wlock(
                    (*al).ranges,
                    &raw mut r#type,
                    &raw mut start,
                    &raw mut end,
                ) != 0
                {
                    return 1 as ::core::ffi::c_int;
                }
            }
            al = (*al).next as *mut alock;
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn posix_lock_do_apply_lock(
    mut il: *mut inodelocks,
    mut sessionid: uint32_t,
    mut owner: uint64_t,
    mut r#type: uint8_t,
    mut start: uint64_t,
    mut end: uint64_t,
    mut pid: uint32_t,
) {
    unsafe {
        let mut al: *mut alock = ::core::ptr::null_mut::<alock>();
        let mut alptr: *mut *mut alock = ::core::ptr::null_mut::<*mut alock>();
        alptr = &raw mut (*il).active;
        loop {
            al = *alptr;
            if al.is_null() {
                break;
            }
            if (*al).owner == owner && (*al).sessionid == sessionid {
                posix_lock_apply_range(&raw mut (*al).ranges, r#type, start, end);
                if (*al).ranges.is_null() {
                    *alptr = (*al).next as *mut alock;
                    free(al as *mut ::core::ffi::c_void);
                }
                return;
            }
            alptr = &raw mut (*al).next as *mut *mut alock;
        }
        if r#type as ::core::ffi::c_int == POSIX_LOCK_UNLCK {
            return;
        }
        al = malloc(::core::mem::size_of::<alock>()) as *mut alock;
        if al.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                390 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"al\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                390 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"al\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if al
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut alock
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                390 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"al\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                390 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"al\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*al).owner = owner;
        (*al).sessionid = sessionid;
        (*al).pid = pid;
        (*al).ranges = ::core::ptr::null_mut::<range>();
        (*al).next = ::core::ptr::null_mut::<_alock>();
        *alptr = al;
        posix_lock_apply_range(&raw mut (*al).ranges, r#type, start, end);
    }
}
#[inline]
unsafe extern "C" fn posix_lock_apply_lock(
    mut il: *mut inodelocks,
    mut sessionid: uint32_t,
    mut owner: uint64_t,
    mut r#type: uint8_t,
    mut start: uint64_t,
    mut end: uint64_t,
    mut pid: uint32_t,
) {
    unsafe {
        changelog(
            b"%u|POSIXLOCK(%u,%u,%lu,%c,%lu,%lu,%u)\0".as_ptr() as *const ::core::ffi::c_char,
            main_time(),
            (*il).inode,
            sessionid,
            owner,
            if r#type as ::core::ffi::c_int == POSIX_LOCK_RDLCK {
                'R' as ::core::ffi::c_int
            } else if r#type as ::core::ffi::c_int == POSIX_LOCK_WRLCK {
                'W' as ::core::ffi::c_int
            } else {
                'U' as ::core::ffi::c_int
            },
            start,
            end,
            pid,
        );
        posix_lock_do_apply_lock(il, sessionid, owner, r#type, start, end, pid);
    }
}
#[inline]
unsafe extern "C" fn posix_lock_append_lock(
    mut il: *mut inodelocks,
    mut connptr: *mut ::core::ffi::c_void,
    mut sessionid: uint32_t,
    mut msgid: uint32_t,
    mut reqid: uint32_t,
    mut owner: uint64_t,
    mut r#type: uint8_t,
    mut start: uint64_t,
    mut end: uint64_t,
    mut pid: uint32_t,
) {
    unsafe {
        let mut wl: *mut wlock = ::core::ptr::null_mut::<wlock>();
        wl = malloc(::core::mem::size_of::<wlock>()) as *mut wlock;
        if wl.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                408 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"wl\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                408 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"wl\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if wl
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut wlock
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                408 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"wl\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                408 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"wl\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        (*wl).owner = owner;
        (*wl).connptr = connptr;
        (*wl).sessionid = sessionid;
        (*wl).pid = pid;
        (*wl).msgid = msgid;
        (*wl).reqid = reqid;
        (*wl).start = start;
        (*wl).end = end;
        (*wl).r#type = r#type;
        (*wl).next = ::core::ptr::null_mut::<_wlock>();
        (*wl).prev = (*il).waiting_tail as *mut *mut _wlock;
        *(*il).waiting_tail = wl;
        (*il).waiting_tail = &raw mut (*wl).next as *mut *mut wlock;
    }
}
#[inline]
unsafe extern "C" fn posix_lock_interrupt(
    mut il: *mut inodelocks,
    mut connptr: *mut ::core::ffi::c_void,
    mut reqid: uint32_t,
) {
    unsafe {
        let mut wl: *mut wlock = ::core::ptr::null_mut::<wlock>();
        wl = (*il).waiting_head;
        while !wl.is_null() {
            if (*wl).connptr == connptr && (*wl).reqid == reqid {
                matoclserv_fuse_posix_lock_wake_up(
                    connptr,
                    (*wl).msgid,
                    MFS_ERROR_EINTR as uint8_t,
                );
                posix_lock_remove_lock(il, wl);
                return;
            }
            wl = (*wl).next as *mut wlock;
        }
    }
}
#[inline]
unsafe extern "C" fn posix_lock_check_waiting(mut il: *mut inodelocks) {
    unsafe {
        let mut wl: *mut wlock = ::core::ptr::null_mut::<wlock>();
        let mut nwl: *mut wlock = ::core::ptr::null_mut::<wlock>();
        if (*il).active.is_null() && (*il).waiting_head.is_null() {
            posix_lock_inode_remove((*il).inode);
            return;
        }
        wl = (*il).waiting_head;
        while !wl.is_null() {
            nwl = (*wl).next as *mut wlock;
            if posix_lock_find_offensive_lock(
                il,
                (*wl).sessionid,
                (*wl).owner,
                (*wl).r#type,
                (*wl).start,
                (*wl).end,
            ) == 0 as ::core::ffi::c_int
            {
                posix_lock_apply_lock(
                    il,
                    (*wl).sessionid,
                    (*wl).owner,
                    (*wl).r#type,
                    (*wl).start,
                    (*wl).end,
                    (*wl).pid,
                );
                matoclserv_fuse_posix_lock_wake_up(
                    (*wl).connptr,
                    (*wl).msgid,
                    MFS_STATUS_OK as uint8_t,
                );
                posix_lock_remove_lock(il, wl);
            }
            wl = nwl;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn posix_lock_disconnected(mut connptr: *mut ::core::ffi::c_void) {
    unsafe {
        let mut h: uint32_t = 0;
        let mut il: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        let mut wl: *mut wlock = ::core::ptr::null_mut::<wlock>();
        let mut nwl: *mut wlock = ::core::ptr::null_mut::<wlock>();
        h = 0 as uint32_t;
        while h < POSIX_LOCK_INODE_HASHSIZE as uint32_t {
            il = *inodehash.offset(h as isize);
            while !il.is_null() {
                wl = (*il).waiting_head;
                while !wl.is_null() {
                    nwl = (*wl).next as *mut wlock;
                    if (*wl).connptr == connptr {
                        posix_lock_remove_lock(il, wl);
                    }
                    wl = nwl;
                }
                il = (*il).next as *mut inodelocks;
            }
            h = h.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn posix_lock_cmd(
    mut connptr: *mut ::core::ffi::c_void,
    mut sessionid: uint32_t,
    mut msgid: uint32_t,
    mut reqid: uint32_t,
    mut inode: uint32_t,
    mut owner: uint64_t,
    mut op: uint8_t,
    mut r#type: *mut uint8_t,
    mut start: *mut uint64_t,
    mut end: *mut uint64_t,
    mut pid: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut il: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        let mut i_type: uint8_t = 0;
        let mut i_start: uint64_t = 0;
        let mut i_end: uint64_t = 0;
        let mut i_pid: uint32_t = 0;
        i_type = *r#type;
        i_start = *start;
        i_end = *end;
        i_pid = *pid;
        if (op as ::core::ffi::c_int == POSIX_LOCK_CMD_SET
            || op as ::core::ffi::c_int == POSIX_LOCK_CMD_TRY)
            && i_type as ::core::ffi::c_int != POSIX_LOCK_UNLCK
        {
            if of_checknode(sessionid, inode) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return MFS_ERROR_NOTOPENED as uint8_t;
            }
        }
        il = posix_lock_inode_find(inode);
        if op as ::core::ffi::c_int == POSIX_LOCK_CMD_INT {
            if il.is_null() {
                return MFS_STATUS_OK as uint8_t;
            }
            posix_lock_interrupt(il, connptr, reqid);
            return MFS_STATUS_OK as uint8_t;
        }
        if op as ::core::ffi::c_int == POSIX_LOCK_CMD_GET {
            if !il.is_null() && i_type as ::core::ffi::c_int != POSIX_LOCK_UNLCK {
                if posix_lock_get_offensive_lock(il, sessionid, owner, r#type, start, end, pid) != 0
                {
                    return MFS_STATUS_OK as uint8_t;
                }
            }
            *r#type = POSIX_LOCK_UNLCK as uint8_t;
            *start = 0 as uint64_t;
            *end = 0 as uint64_t;
            *pid = 0 as uint32_t;
            return MFS_STATUS_OK as uint8_t;
        }
        if !il.is_null() && i_type as ::core::ffi::c_int != POSIX_LOCK_UNLCK {
            if posix_lock_find_offensive_lock(il, sessionid, owner, i_type, i_start, i_end) != 0 {
                if op as ::core::ffi::c_int == POSIX_LOCK_CMD_TRY {
                    return MFS_ERROR_EAGAIN as uint8_t;
                } else {
                    posix_lock_append_lock(
                        il, connptr, sessionid, msgid, reqid, owner, i_type, i_start, i_end, i_pid,
                    );
                    return MFS_ERROR_WAITING as uint8_t;
                }
            }
        }
        if i_type as ::core::ffi::c_int == POSIX_LOCK_UNLCK {
            if il.is_null() {
                return MFS_STATUS_OK as uint8_t;
            }
            posix_lock_apply_lock(il, sessionid, owner, i_type, i_start, i_end, i_pid);
            posix_lock_check_waiting(il);
            return MFS_STATUS_OK as uint8_t;
        }
        if il.is_null() {
            il = posix_lock_inode_new(inode);
        }
        if posix_lock_find_offensive_lock(il, sessionid, owner, i_type, i_start, i_end) != 0 {
            posix_lock_append_lock(
                il, connptr, sessionid, msgid, reqid, owner, i_type, i_start, i_end, i_pid,
            );
            return MFS_ERROR_WAITING as uint8_t;
        }
        posix_lock_apply_lock(il, sessionid, owner, i_type, i_start, i_end, i_pid);
        posix_lock_check_waiting(il);
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn posix_lock_file_closed(mut sessionid: uint32_t, mut inode: uint32_t) {
    unsafe {
        let mut il: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        let mut wl: *mut wlock = ::core::ptr::null_mut::<wlock>();
        let mut nwl: *mut wlock = ::core::ptr::null_mut::<wlock>();
        let mut al: *mut alock = ::core::ptr::null_mut::<alock>();
        let mut alptr: *mut *mut alock = ::core::ptr::null_mut::<*mut alock>();
        let mut changed: uint8_t = 0;
        il = posix_lock_inode_find(inode);
        if il.is_null() {
            return;
        }
        wl = (*il).waiting_head;
        while !wl.is_null() {
            nwl = (*wl).next as *mut wlock;
            if (*wl).sessionid == sessionid {
                posix_lock_remove_lock(il, wl);
            }
            wl = nwl;
        }
        changed = 0 as uint8_t;
        alptr = &raw mut (*il).active;
        loop {
            al = *alptr;
            if al.is_null() {
                break;
            }
            if (*al).sessionid == sessionid {
                posix_lock_apply_range(
                    &raw mut (*al).ranges,
                    POSIX_LOCK_UNLCK as uint8_t,
                    0 as uint64_t,
                    UINT64_MAX as uint64_t,
                );
                if (*al).ranges.is_null() {
                } else {
                    fprintf(
                        stderr,
                        b"%s:%u - failed assertion '%s' : %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        569 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"al->ranges==NULL\0".as_ptr() as *const ::core::ffi::c_char,
                        b"locks axists after unlocking everything !!!\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - failed assertion '%s' : %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        569 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"al->ranges==NULL\0".as_ptr() as *const ::core::ffi::c_char,
                        b"locks axists after unlocking everything !!!\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    abort();
                };
                *alptr = (*al).next as *mut alock;
                free(al as *mut ::core::ffi::c_void);
                changed = 1 as uint8_t;
            } else {
                alptr = &raw mut (*al).next as *mut *mut alock;
            }
        }
        if changed != 0 {
            posix_lock_check_waiting(il);
        } else if (*il).active.is_null() && (*il).waiting_head.is_null() {
            posix_lock_inode_remove((*il).inode);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn posix_lock_list(mut inode: uint32_t, mut buff: *mut uint8_t) -> uint32_t {
    unsafe {
        let mut il: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        let mut al: *mut alock = ::core::ptr::null_mut::<alock>();
        let mut r: *mut range = ::core::ptr::null_mut::<range>();
        let mut h: uint32_t = 0;
        let mut ret: uint32_t = 0 as uint32_t;
        if inode == 0 as uint32_t {
            h = 0 as uint32_t;
            while h < POSIX_LOCK_INODE_HASHSIZE as uint32_t {
                il = *inodehash.offset(h as isize);
                while !il.is_null() {
                    al = (*il).active;
                    while !al.is_null() {
                        r = (*al).ranges;
                        while !r.is_null() {
                            if buff.is_null() {
                                ret = ret.wrapping_add(37 as uint32_t);
                            } else {
                                put32bit(&raw mut buff, (*il).inode);
                                put32bit(&raw mut buff, (*al).sessionid);
                                put64bit(&raw mut buff, (*al).owner);
                                put32bit(&raw mut buff, (*al).pid);
                                put64bit(&raw mut buff, (*r).start);
                                put64bit(&raw mut buff, (*r).end);
                                match (*r).r#type as ::core::ffi::c_int {
                                    POSIX_LOCK_RDLCK => {
                                        put8bit(&raw mut buff, MFS_LOCK_TYPE_SHARED as uint8_t);
                                    }
                                    POSIX_LOCK_WRLCK => {
                                        put8bit(&raw mut buff, MFS_LOCK_TYPE_EXCLUSIVE as uint8_t);
                                    }
                                    _ => {
                                        put8bit(&raw mut buff, MFS_LOCK_TYPE_UNKNOWN as uint8_t);
                                    }
                                }
                            }
                            r = (*r).next as *mut range;
                        }
                        al = (*al).next as *mut alock;
                    }
                    il = (*il).next as *mut inodelocks;
                }
                h = h.wrapping_add(1);
            }
        } else {
            il = posix_lock_inode_find(inode);
            if !il.is_null() {
                al = (*il).active;
                while !al.is_null() {
                    r = (*al).ranges;
                    while !r.is_null() {
                        if buff.is_null() {
                            ret = ret.wrapping_add(33 as uint32_t);
                        } else {
                            put32bit(&raw mut buff, (*al).sessionid);
                            put64bit(&raw mut buff, (*al).owner);
                            put32bit(&raw mut buff, (*al).pid);
                            put64bit(&raw mut buff, (*r).start);
                            put64bit(&raw mut buff, (*r).end);
                            match (*r).r#type as ::core::ffi::c_int {
                                POSIX_LOCK_RDLCK => {
                                    put8bit(&raw mut buff, MFS_LOCK_TYPE_SHARED as uint8_t);
                                }
                                POSIX_LOCK_WRLCK => {
                                    put8bit(&raw mut buff, MFS_LOCK_TYPE_EXCLUSIVE as uint8_t);
                                }
                                _ => {
                                    put8bit(&raw mut buff, MFS_LOCK_TYPE_UNKNOWN as uint8_t);
                                }
                            }
                        }
                        r = (*r).next as *mut range;
                    }
                    al = (*al).next as *mut alock;
                }
            }
        }
        return ret;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn posix_lock_mr_change(
    mut inode: uint32_t,
    mut sessionid: uint32_t,
    mut owner: uint64_t,
    mut cmd: ::core::ffi::c_char,
    mut start: uint64_t,
    mut end: uint64_t,
    mut pid: uint32_t,
) -> uint8_t {
    unsafe {
        let mut il: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        let mut r#type: uint8_t = 0;
        if cmd as ::core::ffi::c_int == 'U' as ::core::ffi::c_int
            || cmd as ::core::ffi::c_int == 'u' as ::core::ffi::c_int
        {
            il = posix_lock_inode_find(inode);
            if il.is_null() {
                return MFS_ERROR_MISMATCH as uint8_t;
            }
            r#type = POSIX_LOCK_UNLCK as uint8_t;
        } else if cmd as ::core::ffi::c_int == 'R' as ::core::ffi::c_int
            || cmd as ::core::ffi::c_int == 'r' as ::core::ffi::c_int
            || cmd as ::core::ffi::c_int == 'S' as ::core::ffi::c_int
            || cmd as ::core::ffi::c_int == 's' as ::core::ffi::c_int
        {
            il = posix_lock_inode_find(inode);
            if il.is_null() {
                il = posix_lock_inode_new(inode);
            }
            r#type = POSIX_LOCK_RDLCK as uint8_t;
        } else if cmd as ::core::ffi::c_int == 'W' as ::core::ffi::c_int
            || cmd as ::core::ffi::c_int == 'w' as ::core::ffi::c_int
            || cmd as ::core::ffi::c_int == 'E' as ::core::ffi::c_int
            || cmd as ::core::ffi::c_int == 'e' as ::core::ffi::c_int
        {
            il = posix_lock_inode_find(inode);
            if il.is_null() {
                il = posix_lock_inode_new(inode);
            }
            r#type = POSIX_LOCK_WRLCK as uint8_t;
        } else {
            return MFS_ERROR_EINVAL as uint8_t;
        }
        if r#type as ::core::ffi::c_int != POSIX_LOCK_UNLCK
            && posix_lock_find_offensive_lock(il, sessionid, owner, r#type, start, end) != 0
        {
            return MFS_ERROR_MISMATCH as uint8_t;
        }
        posix_lock_do_apply_lock(il, sessionid, owner, r#type, start, end, pid);
        if (*il).active.is_null() && (*il).waiting_head.is_null() {
            posix_lock_inode_remove((*il).inode);
        }
        meta_version_inc();
        return MFS_STATUS_OK as uint8_t;
    }
}
pub const POSIX_LOCK_REC_SIZE: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn posix_lock_store(mut fd: *mut bio) -> uint8_t {
    unsafe {
        let mut storebuff: [uint8_t; 37] = [0; 37];
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut h: uint32_t = 0;
        let mut il: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        let mut al: *mut alock = ::core::ptr::null_mut::<alock>();
        let mut r: *mut range = ::core::ptr::null_mut::<range>();
        if fd.is_null() {
            return 0x10 as uint8_t;
        }
        h = 0 as uint32_t;
        while h < POSIX_LOCK_INODE_HASHSIZE as uint32_t {
            il = *inodehash.offset(h as isize);
            while !il.is_null() {
                al = (*il).active;
                while !al.is_null() {
                    r = (*al).ranges;
                    while !r.is_null() {
                        ptr = &raw mut storebuff as *mut uint8_t;
                        put32bit(&raw mut ptr, (*il).inode);
                        put64bit(&raw mut ptr, (*al).owner);
                        put32bit(&raw mut ptr, (*al).sessionid);
                        put32bit(&raw mut ptr, (*al).pid);
                        put64bit(&raw mut ptr, (*r).start);
                        put64bit(&raw mut ptr, (*r).end);
                        put8bit(&raw mut ptr, (*r).r#type);
                        if bio_write(
                            fd,
                            &raw mut storebuff as *mut uint8_t as *const ::core::ffi::c_void,
                            POSIX_LOCK_REC_SIZE as uint64_t,
                        ) != POSIX_LOCK_REC_SIZE as int64_t
                        {
                            return 0xff as uint8_t;
                        }
                        r = (*r).next as *mut range;
                    }
                    al = (*al).next as *mut alock;
                }
                il = (*il).next as *mut inodelocks;
            }
            h = h.wrapping_add(1);
        }
        memset(
            &raw mut storebuff as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            POSIX_LOCK_REC_SIZE as size_t,
        );
        if bio_write(
            fd,
            &raw mut storebuff as *mut uint8_t as *const ::core::ffi::c_void,
            POSIX_LOCK_REC_SIZE as uint64_t,
        ) != POSIX_LOCK_REC_SIZE as int64_t
        {
            return 0xff as uint8_t;
        }
        return 0 as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn posix_lock_load(
    mut fd: *mut bio,
    mut mver: uint8_t,
    mut ignoreflag: uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut loadbuff: [uint8_t; 37] = [0; 37];
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut l: int32_t = 0;
        let mut inode: uint32_t = 0;
        let mut lastinode: uint32_t = 0;
        let mut sessionid: uint32_t = 0;
        let mut lastsessionid: uint32_t = 0;
        let mut pid: uint32_t = 0;
        let mut owner: uint64_t = 0;
        let mut lastowner: uint64_t = 0;
        let mut start: uint64_t = 0;
        let mut end: uint64_t = 0;
        let mut lastend: uint64_t = 0;
        let mut r#type: uint8_t = 0;
        let mut lasttype: uint8_t = 0;
        let mut fino: uint8_t = 0;
        let mut fses: uint8_t = 0;
        let mut il: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        let mut al: *mut alock = ::core::ptr::null_mut::<alock>();
        let mut altail: *mut *mut alock = ::core::ptr::null_mut::<*mut alock>();
        let mut r: *mut range = ::core::ptr::null_mut::<range>();
        let mut rtail: *mut *mut range = ::core::ptr::null_mut::<*mut range>();
        if mver as ::core::ffi::c_int != 0x10 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        fino = 1 as uint8_t;
        fses = 1 as uint8_t;
        lastinode = 0 as uint32_t;
        lastsessionid = 0 as uint32_t;
        lastowner = 0 as uint64_t;
        lasttype = 0 as uint8_t;
        lastend = 0 as uint64_t;
        il = ::core::ptr::null_mut::<inodelocks>();
        al = ::core::ptr::null_mut::<alock>();
        r = ::core::ptr::null_mut::<range>();
        altail = ::core::ptr::null_mut::<*mut alock>();
        rtail = ::core::ptr::null_mut::<*mut range>();
        loop {
            l = bio_read(
                fd,
                &raw mut loadbuff as *mut uint8_t as *mut ::core::ffi::c_void,
                POSIX_LOCK_REC_SIZE as uint64_t,
            ) as int32_t;
            if l != POSIX_LOCK_REC_SIZE as int32_t {
                return -1 as ::core::ffi::c_int;
            }
            ptr = &raw mut loadbuff as *mut uint8_t;
            inode = get32bit(&raw mut ptr);
            owner = get64bit(&raw mut ptr);
            sessionid = get32bit(&raw mut ptr);
            pid = get32bit(&raw mut ptr);
            start = get64bit(&raw mut ptr);
            end = get64bit(&raw mut ptr);
            r#type = get8bit(&raw mut ptr);
            if inode == 0 as uint32_t && owner == 0 as uint64_t && sessionid == 0 as uint32_t {
                return 0 as ::core::ffi::c_int;
            }
            if inode != lastinode
                || sessionid != lastsessionid
                || fino as ::core::ffi::c_int != 0
                || fses as ::core::ffi::c_int != 0
            {
                if of_checknode(sessionid, inode) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    if ignoreflag != 0 {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_NOTICE,
                            b"loading posix_locks: lock on closed file !!! (ignoring)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        continue;
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_ERR,
                            b"loading posix_locks: lock on closed file !!!\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                }
            }
            if inode != lastinode || fino as ::core::ffi::c_int != 0 {
                lastinode = inode;
                lastsessionid = 0 as uint32_t;
                lastowner = 0 as uint64_t;
                fses = 1 as uint8_t;
                il = posix_lock_inode_find(inode);
                if il.is_null() {
                    il = posix_lock_inode_new(inode);
                }
                altail = &raw mut (*il).active;
                fino = 0 as uint8_t;
            }
            if sessionid != lastsessionid || owner != lastowner || fses as ::core::ffi::c_int != 0 {
                lastsessionid = sessionid;
                lastowner = owner;
                lastend = 0 as uint64_t;
                lasttype = POSIX_LOCK_UNLCK as uint8_t;
                al = malloc(::core::mem::size_of::<alock>()) as *mut alock;
                if al.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        801 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"al\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        801 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"al\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if al
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut alock
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        801 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"al\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        801 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"al\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
                (*al).owner = owner;
                (*al).sessionid = sessionid;
                (*al).pid = pid;
                (*al).ranges = ::core::ptr::null_mut::<range>();
                (*al).next = ::core::ptr::null_mut::<_alock>();
                *altail = al;
                altail = &raw mut (*al).next as *mut *mut alock;
                rtail = &raw mut (*al).ranges;
                fses = 0 as uint8_t;
            }
            if lasttype as ::core::ffi::c_int != POSIX_LOCK_UNLCK {
                if start < lastend {
                    if ignoreflag != 0 {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_NOTICE,
                            b"loading posix_locks: lock range not in order !!! (ignoring)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                        continue;
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_ERR,
                            b"loading posix_locks: lock range not in order !!!\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                } else if r#type as ::core::ffi::c_int == lasttype as ::core::ffi::c_int
                    && start == lastend
                {
                    if ignoreflag != 0 {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_NOTICE,
                            b"loading posix_locks: lock range not connected !!! (ignoring)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                        continue;
                    } else {
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_ERR,
                            b"loading posix_locks: lock range not connected !!!\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        return -1 as ::core::ffi::c_int;
                    }
                }
            }
            r = malloc(::core::mem::size_of::<range>()) as *mut range;
            if r.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    833 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"r\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    833 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"r\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if r
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut range
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    833 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"r\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    833 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"r\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
            (*r).start = start;
            (*r).end = end;
            (*r).r#type = r#type;
            (*r).next = ::core::ptr::null_mut::<_range>();
            *rtail = r;
            rtail = &raw mut (*r).next as *mut *mut range;
            lastend = end;
            lasttype = r#type;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn posix_lock_cleanup() {
    unsafe {
        let mut h: uint32_t = 0;
        let mut il: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        let mut nil: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        let mut wl: *mut wlock = ::core::ptr::null_mut::<wlock>();
        let mut nwl: *mut wlock = ::core::ptr::null_mut::<wlock>();
        let mut al: *mut alock = ::core::ptr::null_mut::<alock>();
        let mut nal: *mut alock = ::core::ptr::null_mut::<alock>();
        let mut r: *mut range = ::core::ptr::null_mut::<range>();
        let mut nr: *mut range = ::core::ptr::null_mut::<range>();
        h = 0 as uint32_t;
        while h < POSIX_LOCK_INODE_HASHSIZE as uint32_t {
            il = *inodehash.offset(h as isize);
            while !il.is_null() {
                nil = (*il).next as *mut inodelocks;
                wl = (*il).waiting_head;
                while !wl.is_null() {
                    nwl = (*wl).next as *mut wlock;
                    free(wl as *mut ::core::ffi::c_void);
                    wl = nwl;
                }
                al = (*il).active;
                while !al.is_null() {
                    nal = (*al).next as *mut alock;
                    r = (*al).ranges;
                    while !r.is_null() {
                        nr = (*r).next as *mut range;
                        free(r as *mut ::core::ffi::c_void);
                        r = nr;
                    }
                    free(al as *mut ::core::ffi::c_void);
                    al = nal;
                }
                free(il as *mut ::core::ffi::c_void);
                il = nil;
            }
            *inodehash.offset(h as isize) = ::core::ptr::null_mut::<inodelocks>();
            h = h.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn posix_lock_info(mut fd: *mut FILE) {
    unsafe {
        let mut h: uint32_t = 0;
        let mut il: *mut inodelocks = ::core::ptr::null_mut::<inodelocks>();
        let mut wl: *mut wlock = ::core::ptr::null_mut::<wlock>();
        let mut al: *mut alock = ::core::ptr::null_mut::<alock>();
        let mut r: *mut range = ::core::ptr::null_mut::<range>();
        if DebugInfo != 0 {
            fprintf(
                fd,
                b"[posix locks]\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            h = 0 as uint32_t;
            while h < POSIX_LOCK_INODE_HASHSIZE as uint32_t {
                il = *inodehash.offset(h as isize);
                while !il.is_null() {
                    fprintf(
                        fd,
                        b"- inode: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
                        (*il).inode,
                    );
                    wl = (*il).waiting_head;
                    while !wl.is_null() {
                        fprintf(
                            fd,
                            b"  - waiting_lock: owner: %lu, sessionid: %u, pid: %u, msgid: %u, reqid: %u, start: %lu, end: %lu, type: %c\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            (*wl).owner,
                            (*wl).sessionid,
                            (*wl).pid,
                            (*wl).msgid,
                            (*wl).reqid,
                            (*wl).start,
                            (*wl).end,
                            if (*wl).r#type as ::core::ffi::c_int == POSIX_LOCK_RDLCK {
                                'R' as ::core::ffi::c_int
                            } else if (*wl).r#type as ::core::ffi::c_int
                                == POSIX_LOCK_WRLCK
                            {
                                'W' as ::core::ffi::c_int
                            } else {
                                'U' as ::core::ffi::c_int
                            },
                        );
                        wl = (*wl).next as *mut wlock;
                    }
                    al = (*il).active;
                    while !al.is_null() {
                        fprintf(
                            fd,
                            b"  - active_lock: owner: %lu, sessionid: %u, pid: %u\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*al).owner,
                            (*al).sessionid,
                            (*al).pid,
                        );
                        r = (*al).ranges;
                        while !r.is_null() {
                            fprintf(
                                fd,
                                b"    - start: %lu, end: %lu, type: %c\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                (*r).start,
                                (*r).end,
                                if (*r).r#type as ::core::ffi::c_int == POSIX_LOCK_RDLCK {
                                    'R' as ::core::ffi::c_int
                                } else if (*r).r#type as ::core::ffi::c_int == POSIX_LOCK_WRLCK {
                                    'W' as ::core::ffi::c_int
                                } else {
                                    'U' as ::core::ffi::c_int
                                },
                            );
                            r = (*r).next as *mut range;
                        }
                        al = (*al).next as *mut alock;
                    }
                    il = (*il).next as *mut inodelocks;
                }
                h = h.wrapping_add(1);
            }
            fprintf(fd, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn posix_lock_reload() {
    unsafe {
        DebugInfo = cfg_getuint8(
            b"EXTRA_DEBUG_INFO\0".as_ptr() as *const ::core::ffi::c_char,
            0 as uint8_t,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn posix_lock_init() -> ::core::ffi::c_int {
    unsafe {
        let mut i: uint32_t = 0;
        inodehash = malloc(
            ::core::mem::size_of::<*mut inodelocks>()
                .wrapping_mul(POSIX_LOCK_INODE_HASHSIZE as size_t),
        ) as *mut *mut inodelocks;
        if inodehash.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                917 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"inodehash\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                917 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"inodehash\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if inodehash
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut *mut inodelocks
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                917 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"inodehash\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/posixlocks.c\0".as_ptr() as *const ::core::ffi::c_char,
                917 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"inodehash\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        i = 0 as uint32_t;
        while i < POSIX_LOCK_INODE_HASHSIZE as uint32_t {
            *inodehash.offset(i as isize) = ::core::ptr::null_mut::<inodelocks>();
            i = i.wrapping_add(1);
        }
        posix_lock_reload();
        main_reload_register_fname(
            Some(posix_lock_reload as unsafe extern "C" fn() -> ()),
            b"posix_lock_reload\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_info_register_fname(
            Some(posix_lock_info as unsafe extern "C" fn(*mut FILE) -> ()),
            b"posix_lock_info\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
}
