pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
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
    unsafe fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    unsafe fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
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
    unsafe fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
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
    unsafe fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn fsync(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn meta_version_inc() -> uint64_t;
    unsafe fn meta_version() -> uint64_t;
    unsafe fn meta_chlog_keep_version() -> uint64_t;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn bgsaver_changelog(version: uint64_t, message: *const ::core::ffi::c_char);
    unsafe fn bgsaver_rotatelog() -> ::core::ffi::c_int;
    unsafe fn main_reload_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn main_info_register_fname(
        fun: Option<unsafe extern "C" fn(*mut FILE) -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn main_time_register_fname(
        seconds: uint32_t,
        offset: uint32_t,
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn main_time() -> uint32_t;
    unsafe fn matomlserv_get_min_version() -> uint64_t;
    unsafe fn matomlserv_broadcast_logstring(
        version: uint64_t,
        logstr: *mut uint8_t,
        logstrsize: uint32_t,
    );
    unsafe fn matomlserv_broadcast_logrotate();
    unsafe fn cfg_getuint8(name: *const ::core::ffi::c_char, def: uint8_t) -> uint8_t;
    unsafe fn cfg_getuint16(name: *const ::core::ffi::c_char, def: uint16_t) -> uint16_t;
    unsafe fn cfg_getuint32(name: *const ::core::ffi::c_char, def: uint32_t) -> uint32_t;
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type __gnuc_va_list = __builtin_va_list;
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
pub type va_list = __gnuc_va_list;
pub type ssize_t = isize;
pub type int32_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct old_changes_entry {
    pub version: uint64_t,
    pub length: uint32_t,
    pub data: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct old_changes_block {
    pub old_changes_block: [old_changes_entry; 5000],
    pub entries: uint32_t,
    pub size: uint32_t,
    pub mintimestamp: uint32_t,
    pub minversion: uint64_t,
    pub next: *mut old_changes_block,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ROTATE_FLAG_BROADCAST: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ROTATE_FLAG_FOREGROUND: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MAXLOGLINESIZE: ::core::ffi::c_uint = 200000 as ::core::ffi::c_uint;
pub const MAXLOGNUMBER: ::core::ffi::c_uint = 1000 as ::core::ffi::c_uint;
static mut BackLogsNumber: uint32_t = 0;
static mut currentfd: *mut FILE = ::core::ptr::null_mut::<FILE>();
pub const OLD_CHANGES_BLOCK_SIZE: ::core::ffi::c_int = 5000 as ::core::ffi::c_int;
static mut old_changes_head: *mut old_changes_block = ::core::ptr::null_mut::<old_changes_block>();
static mut old_changes_current: *mut old_changes_block =
    ::core::ptr::null_mut::<old_changes_block>();
static mut lastchange: ::core::ffi::c_double = 0.0f64;
static mut old_changes_total_size: uint64_t = 0 as uint64_t;
static mut ChangelogSecondsToRemember: uint32_t = 0;
static mut ChangeLogMaxSize: uint64_t = 0;
static mut ChangelogSaveMode: uint8_t = 0;
pub const SAVEMODE_BACKGROUND: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SAVEMODE_SYNC: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SAVEMODE_MAX: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn changelog_old_changes_free_block(mut oc: *mut old_changes_block) {
    unsafe {
        let mut i: uint32_t = 0;
        i = 0 as uint32_t;
        while i < (*oc).entries {
            free((*oc).old_changes_block[i as usize].data as *mut ::core::ffi::c_void);
            i = i.wrapping_add(1);
        }
        free(oc as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn changelog_old_changes_free(mut ts: uint32_t) {
    unsafe {
        let mut oc: *mut old_changes_block = ::core::ptr::null_mut::<old_changes_block>();
        let mut minversion_to_keep: uint64_t = 0;
        let mut mv1: uint64_t = 0;
        let mut mv2: uint64_t = 0;
        mv1 = meta_chlog_keep_version();
        mv2 = matomlserv_get_min_version();
        minversion_to_keep = if mv1 < mv2 { mv1 } else { mv2 };
        while !old_changes_head.is_null()
            && !(*old_changes_head).next.is_null()
            && ((*(*old_changes_head).next).minversion < minversion_to_keep
                && ((*(*old_changes_head).next)
                    .mintimestamp
                    .wrapping_add(ChangelogSecondsToRemember)
                    < ts
                    || old_changes_total_size > ChangeLogMaxSize))
        {
            oc = (*old_changes_head).next as *mut old_changes_block;
            old_changes_total_size =
                old_changes_total_size.wrapping_sub((*old_changes_head).size as uint64_t);
            changelog_old_changes_free_block(old_changes_head);
            old_changes_head = oc;
        }
    }
}
#[inline]
unsafe extern "C" fn changelog_store_logstring(
    mut version: uint64_t,
    mut logstr: *mut uint8_t,
    mut logstrsize: uint32_t,
) {
    unsafe {
        let mut oc: *mut old_changes_block = ::core::ptr::null_mut::<old_changes_block>();
        let mut oce: *mut old_changes_entry = ::core::ptr::null_mut::<old_changes_entry>();
        let mut ts: uint32_t = 0;
        matomlserv_broadcast_logstring(version, logstr, logstrsize);
        if ChangelogSecondsToRemember == 0 as uint32_t {
            while !old_changes_head.is_null() {
                oc = (*old_changes_head).next as *mut old_changes_block;
                changelog_old_changes_free_block(old_changes_head);
                old_changes_head = oc;
            }
            return;
        }
        if old_changes_current.is_null()
            || old_changes_head.is_null()
            || (*old_changes_current).entries >= OLD_CHANGES_BLOCK_SIZE as uint32_t
        {
            oc = malloc(::core::mem::size_of::<old_changes_block>()) as *mut old_changes_block;
            if oc.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/changelog.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    123 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"oc\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/changelog.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    123 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"oc\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if oc
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut old_changes_block
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/changelog.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    123 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"oc\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/changelog.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    123 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"oc\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            ts = main_time();
            (*oc).entries = 0 as uint32_t;
            (*oc).size = 0 as uint32_t;
            (*oc).minversion = version;
            (*oc).mintimestamp = ts;
            (*oc).next = ::core::ptr::null_mut::<old_changes_block>();
            if old_changes_current.is_null() || old_changes_head.is_null() {
                old_changes_current = oc;
                old_changes_head = old_changes_current;
            } else {
                (*old_changes_current).next = oc as *mut old_changes_block;
                old_changes_current = oc;
            }
            changelog_old_changes_free(ts);
        }
        oc = old_changes_current;
        oce = (&raw mut (*oc).old_changes_block as *mut old_changes_entry)
            .offset((*oc).entries as isize);
        (*oce).version = version;
        (*oce).length = logstrsize;
        (*oce).data = malloc(logstrsize as size_t) as *mut uint8_t;
        if (*oce).data.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/changelog.c\0".as_ptr() as *const ::core::ffi::c_char,
                143 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"oce->data\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/changelog.c\0".as_ptr() as *const ::core::ffi::c_char,
                143 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"oce->data\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if (*oce).data
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut uint8_t
        {
            let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/changelog.c\0".as_ptr() as *const ::core::ffi::c_char,
                143 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"oce->data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/changelog.c\0".as_ptr() as *const ::core::ffi::c_char,
                143 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"oce->data\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            abort();
        }
        memcpy(
            (*oce).data as *mut ::core::ffi::c_void,
            logstr as *const ::core::ffi::c_void,
            logstrsize as size_t,
        );
        (*oc).entries = (*oc).entries.wrapping_add(1);
        (*oc).size = ((*oc).size as ::core::ffi::c_ulong).wrapping_add(
            (logstrsize as usize).wrapping_add(::core::mem::size_of::<old_changes_entry>())
                as ::core::ffi::c_ulong,
        ) as uint32_t;
        old_changes_total_size = (old_changes_total_size as ::core::ffi::c_ulong).wrapping_add(
            (logstrsize as usize).wrapping_add(::core::mem::size_of::<old_changes_entry>())
                as ::core::ffi::c_ulong,
        ) as uint64_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn changelog_get_old_changes(
    mut version: uint64_t,
    mut sendfn: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, uint64_t, *mut uint8_t, uint32_t) -> (),
    >,
    mut userdata: *mut ::core::ffi::c_void,
    mut limit: uint32_t,
) -> uint32_t {
    unsafe {
        let mut oc: *mut old_changes_block = ::core::ptr::null_mut::<old_changes_block>();
        let mut oce: *mut old_changes_entry = ::core::ptr::null_mut::<old_changes_entry>();
        let mut start: uint8_t = 0 as uint8_t;
        let mut i: uint32_t = 0;
        let mut j: uint32_t = 0;
        j = 0 as uint32_t;
        oc = old_changes_head;
        while !oc.is_null() {
            if (*oc).minversion <= version
                && ((*oc).next.is_null() || (*(*oc).next).minversion > version)
            {
                start = 1 as uint8_t;
            }
            if start != 0 {
                i = 0 as uint32_t;
                while i < (*oc).entries {
                    oce = (&raw mut (*oc).old_changes_block as *mut old_changes_entry)
                        .offset(i as isize);
                    if version <= (*oce).version {
                        if j < limit {
                            sendfn.expect("non-null function pointer")(
                                userdata,
                                (*oce).version,
                                (*oce).data,
                                (*oce).length,
                            );
                            j = j.wrapping_add(1);
                        } else {
                            return j;
                        }
                    }
                    i = i.wrapping_add(1);
                }
            }
            oc = (*oc).next as *mut old_changes_block;
        }
        return j;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn changelog_get_minversion() -> uint64_t {
    unsafe {
        if old_changes_head.is_null() {
            return meta_version();
        }
        return (*old_changes_head).minversion;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn changelog_rotate(mut rotate_flags: uint8_t) {
    unsafe {
        if ChangelogSaveMode as ::core::ffi::c_int != SAVEMODE_BACKGROUND {
            rotate_flags = (rotate_flags as ::core::ffi::c_int | ROTATE_FLAG_FOREGROUND) as uint8_t;
        }
        if rotate_flags as ::core::ffi::c_int & ROTATE_FLAG_FOREGROUND == 0 as ::core::ffi::c_int {
            if bgsaver_rotatelog() < 0 as ::core::ffi::c_int {
                rotate_flags =
                    (rotate_flags as ::core::ffi::c_int | ROTATE_FLAG_FOREGROUND) as uint8_t;
            }
        }
        if rotate_flags as ::core::ffi::c_int & ROTATE_FLAG_FOREGROUND != 0 {
            let mut logname1: [::core::ffi::c_char; 100] = [0; 100];
            let mut logname2: [::core::ffi::c_char; 100] = [0; 100];
            let mut i: uint32_t = 0;
            if !currentfd.is_null() {
                if ChangelogSaveMode as ::core::ffi::c_int == SAVEMODE_SYNC {
                    fsync(fileno(currentfd));
                }
                fclose(currentfd);
                currentfd = ::core::ptr::null_mut::<FILE>();
            }
            if BackLogsNumber > 0 as uint32_t {
                i = BackLogsNumber;
                while i > 0 as uint32_t {
                    snprintf(
                        &raw mut logname1 as *mut ::core::ffi::c_char,
                        100 as size_t,
                        b"changelog.%u.mfs\0".as_ptr() as *const ::core::ffi::c_char,
                        i,
                    );
                    snprintf(
                        &raw mut logname2 as *mut ::core::ffi::c_char,
                        100 as size_t,
                        b"changelog.%u.mfs\0".as_ptr() as *const ::core::ffi::c_char,
                        i.wrapping_sub(1 as uint32_t),
                    );
                    rename(
                        &raw mut logname2 as *mut ::core::ffi::c_char,
                        &raw mut logname1 as *mut ::core::ffi::c_char,
                    );
                    i = i.wrapping_sub(1);
                }
            } else {
                unlink(b"changelog.0.mfs\0".as_ptr() as *const ::core::ffi::c_char);
            }
        }
        if rotate_flags as ::core::ffi::c_int & ROTATE_FLAG_BROADCAST != 0 {
            matomlserv_broadcast_logrotate();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn changelog_mr(mut version: uint64_t, mut data: *const ::core::ffi::c_char) {
    unsafe {
        if ChangelogSaveMode as ::core::ffi::c_int == SAVEMODE_BACKGROUND {
            bgsaver_changelog(version, data);
        } else {
            if currentfd.is_null() {
                currentfd = fopen(
                    b"changelog.0.mfs\0".as_ptr() as *const ::core::ffi::c_char,
                    b"a\0".as_ptr() as *const ::core::ffi::c_char,
                ) as *mut FILE;
                if currentfd.is_null() {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"lost MFS change %lu: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        version,
                        data,
                    );
                }
            }
            if !currentfd.is_null() {
                fprintf(
                    currentfd,
                    b"%lu: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    version,
                    data,
                );
                fflush(currentfd);
                if ChangelogSaveMode as ::core::ffi::c_int == SAVEMODE_SYNC {
                    fsync(fileno(currentfd));
                }
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn changelog(mut format: *const ::core::ffi::c_char, mut c2rust_args: ...) {
    unsafe {
        static mut printbuff: [::core::ffi::c_char; 200000] = [0; 200000];
        let mut ap: ::core::ffi::VaList;
        let mut leng: uint32_t = 0;
        let mut version: uint64_t = 0;
        ap = c2rust_args.clone();
        leng = vsnprintf(
            &raw mut printbuff as *mut ::core::ffi::c_char,
            MAXLOGLINESIZE as size_t,
            format,
            ap.clone(),
        ) as uint32_t;
        if leng >= MAXLOGLINESIZE as uint32_t {
            printbuff[MAXLOGLINESIZE.wrapping_sub(1 as ::core::ffi::c_uint) as usize] =
                '\0' as ::core::ffi::c_char;
            leng = MAXLOGLINESIZE as uint32_t;
        } else {
            leng = leng.wrapping_add(1);
        }
        version = meta_version_inc();
        changelog_mr(version, &raw mut printbuff as *mut ::core::ffi::c_char);
        changelog_store_logstring(
            version,
            &raw mut printbuff as *mut ::core::ffi::c_char as *mut uint8_t,
            leng,
        );
        lastchange = monotonic_seconds();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn changelog_generate_gids(
    mut gids: uint32_t,
    mut gid: *mut uint32_t,
) -> *mut ::core::ffi::c_char {
    unsafe {
        static mut gidstr: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        static mut gidstr_size: uint32_t = 0 as uint32_t;
        let mut i: uint32_t = 0;
        let mut l: uint32_t = 0;
        i = gids
            .wrapping_div(32 as uint32_t)
            .wrapping_add(1 as uint32_t)
            .wrapping_mul(32 as uint32_t);
        i = i.wrapping_mul(11 as uint32_t);
        i = i.wrapping_add(10 as uint32_t);
        if i > gidstr_size || gidstr.is_null() {
            if !gidstr.is_null() {
                free(gidstr as *mut ::core::ffi::c_void);
            }
            gidstr = malloc(i as size_t) as *mut ::core::ffi::c_char;
            if gidstr.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/changelog.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    278 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"gidstr\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/changelog.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    278 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"gidstr\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if gidstr
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut ::core::ffi::c_char
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/changelog.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    278 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"gidstr\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/changelog.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    278 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"gidstr\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            gidstr_size = i;
        }
        l = 0 as uint32_t;
        let c2rust_fresh0 = l;
        l = l.wrapping_add(1);
        *gidstr.offset(c2rust_fresh0 as isize) = '[' as ::core::ffi::c_char;
        i = 0 as uint32_t;
        while i < gids {
            if l < gidstr_size {
                l = l.wrapping_add(snprintf(
                    gidstr.offset(l as isize),
                    gidstr_size.wrapping_sub(l) as size_t,
                    b"%u\0".as_ptr() as *const ::core::ffi::c_char,
                    *gid.offset(i as isize),
                ) as uint32_t);
            }
            if l < gidstr_size {
                let c2rust_fresh1 = l;
                l = l.wrapping_add(1);
                *gidstr.offset(c2rust_fresh1 as isize) = (if i.wrapping_add(1 as uint32_t) < gids {
                    ',' as ::core::ffi::c_int
                } else {
                    ']' as ::core::ffi::c_int
                }) as ::core::ffi::c_char;
            }
            i = i.wrapping_add(1);
        }
        if l < gidstr_size {
            let c2rust_fresh2 = l;
            l = l.wrapping_add(1);
            *gidstr.offset(c2rust_fresh2 as isize) = '\0' as ::core::ffi::c_char;
        }
        return gidstr;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn changelog_escape_name(
    mut nleng: uint32_t,
    mut name: *const uint8_t,
) -> *mut ::core::ffi::c_char {
    unsafe {
        static mut escname: [*mut ::core::ffi::c_char; 2] = [
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ];
        static mut escnamesize: [uint32_t; 2] = [0 as uint32_t, 0 as uint32_t];
        static mut buffid: uint8_t = 0 as uint8_t;
        let mut currescname: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut i: uint32_t = 0;
        let mut c: uint8_t = 0;
        buffid = (1 as ::core::ffi::c_int - buffid as ::core::ffi::c_int) as uint8_t;
        i = nleng;
        i = i.wrapping_mul(3 as uint32_t).wrapping_add(1 as uint32_t);
        if i > escnamesize[buffid as usize] || i == 0 as uint32_t {
            escnamesize[buffid as usize] = i
                .wrapping_div(1000 as uint32_t)
                .wrapping_add(1 as uint32_t)
                .wrapping_mul(1000 as uint32_t);
            if !escname[buffid as usize].is_null() {
                free(escname[buffid as usize] as *mut ::core::ffi::c_void);
            }
            escname[buffid as usize] =
                malloc(escnamesize[buffid as usize] as size_t) as *mut ::core::ffi::c_char;
            if escname[buffid as usize].is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/changelog.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    313 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"escname[buffid]\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/changelog.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    313 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"escname[buffid]\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if escname[buffid as usize]
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut ::core::ffi::c_char
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/changelog.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    313 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"escname[buffid]\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/changelog.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    313 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"escname[buffid]\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
        }
        i = 0 as uint32_t;
        currescname = escname[buffid as usize];
        if currescname.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/changelog.c\0".as_ptr() as *const ::core::ffi::c_char,
                317 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"currescname\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/changelog.c\0".as_ptr() as *const ::core::ffi::c_char,
                317 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"currescname\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if currescname
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut ::core::ffi::c_char
        {
            let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/changelog.c\0".as_ptr() as *const ::core::ffi::c_char,
                317 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"currescname\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/changelog.c\0".as_ptr() as *const ::core::ffi::c_char,
                317 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"currescname\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring_0,
            );
            abort();
        }
        while nleng > 0 as uint32_t {
            c = *name;
            if (c as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
                || c as ::core::ffi::c_int >= 127 as ::core::ffi::c_int
                || c as ::core::ffi::c_int == ',' as ::core::ffi::c_int
                || c as ::core::ffi::c_int == '%' as ::core::ffi::c_int
                || c as ::core::ffi::c_int == '(' as ::core::ffi::c_int
                || c as ::core::ffi::c_int == ')' as ::core::ffi::c_int
            {
                let c2rust_fresh3 = i;
                i = i.wrapping_add(1);
                *currescname.offset(c2rust_fresh3 as isize) = '%' as ::core::ffi::c_char;
                let c2rust_fresh4 = i;
                i = i.wrapping_add(1);
                *currescname.offset(c2rust_fresh4 as isize) =
                    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(
                        *b"0123456789ABCDEF\0",
                    )[(c as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                        & 0xf as ::core::ffi::c_int) as usize];
                let c2rust_fresh5 = i;
                i = i.wrapping_add(1);
                *currescname.offset(c2rust_fresh5 as isize) =
                    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(
                        *b"0123456789ABCDEF\0",
                    )[(c as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as usize];
            } else {
                let c2rust_fresh6 = i;
                i = i.wrapping_add(1);
                *currescname.offset(c2rust_fresh6 as isize) = c as ::core::ffi::c_char;
            }
            name = name.offset(1);
            nleng = nleng.wrapping_sub(1);
        }
        *currescname.offset(i as isize) = 0 as ::core::ffi::c_char;
        return currescname;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn changelog_sendnop() {
    unsafe {
        if lastchange + 30.0f64 <= monotonic_seconds() {
            changelog(
                b"%u|IDLE()\0".as_ptr() as *const ::core::ffi::c_char,
                main_time(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn changelog_info(mut fd: *mut FILE) {
    unsafe {
        let mut minversion_to_keep: uint64_t = 0;
        let mut current_meteversion: uint64_t = 0;
        current_meteversion = meta_version();
        fprintf(
            fd,
            b"[changelog]\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !old_changes_head.is_null() {
            fprintf(
                fd,
                b"oldest_metaversion: %lu\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*old_changes_head).minversion,
            );
            fprintf(
                fd,
                b"oldest_timestamp: %u (%d seconds)\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*old_changes_head).mintimestamp,
                main_time().wrapping_sub((*old_changes_head).mintimestamp),
            );
            fprintf(
                fd,
                b"total_size: %lu\n\0".as_ptr() as *const ::core::ffi::c_char,
                old_changes_total_size,
            );
        } else {
            fprintf(
                fd,
                b"changelog memory is empty\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        minversion_to_keep = meta_chlog_keep_version();
        if minversion_to_keep < current_meteversion {
            fprintf(
                fd,
                b"min_changelog_kept_for_metadata_sending: %lu\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                minversion_to_keep,
            );
        } else {
            fprintf(
                fd,
                b"min_changelog_kept_for_metadata_sending: -\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        minversion_to_keep = matomlserv_get_min_version();
        if minversion_to_keep < current_meteversion {
            fprintf(
                fd,
                b"min_changelog_kept_for_delayed_receivers: %lu\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                minversion_to_keep,
            );
        } else {
            fprintf(
                fd,
                b"min_changelog_kept_for_delayed_receivers: -\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        fprintf(fd, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn changelog_reload() {
    unsafe {
        let mut changelog_preserve_mb: uint16_t = 0;
        BackLogsNumber = cfg_getuint32(
            b"BACK_LOGS\0".as_ptr() as *const ::core::ffi::c_char,
            50 as uint32_t,
        );
        if BackLogsNumber > MAXLOGNUMBER as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"BACK_LOGS value too big !!!\0".as_ptr() as *const ::core::ffi::c_char,
            );
            BackLogsNumber = MAXLOGNUMBER as uint32_t;
        }
        ChangelogSecondsToRemember = cfg_getuint16(
            b"CHANGELOG_PRESERVE_SECONDS\0".as_ptr() as *const ::core::ffi::c_char,
            5000 as uint16_t,
        ) as uint32_t;
        if ChangelogSecondsToRemember > 100000 as uint32_t {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"Number of seconds of change logs to be preserved in master is too big (%u) - decreasing to 100000 seconds\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                ChangelogSecondsToRemember,
            );
            ChangelogSecondsToRemember = 100000 as uint32_t;
        }
        changelog_preserve_mb = cfg_getuint16(
            b"CHANGELOG_PRESERVE_MB\0".as_ptr() as *const ::core::ffi::c_char,
            500 as uint16_t,
        );
        if (changelog_preserve_mb as ::core::ffi::c_int) < 100 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"Size of change logs in MB too low (%hu) - increasing to 100MB\0".as_ptr()
                    as *const ::core::ffi::c_char,
                changelog_preserve_mb as ::core::ffi::c_int,
            );
            changelog_preserve_mb = 100 as uint16_t;
        }
        if changelog_preserve_mb as ::core::ffi::c_int > 10000 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"Size of change logs in MB too big (%hu) - decreasing to 10000MB (10GB)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                changelog_preserve_mb as ::core::ffi::c_int,
            );
            changelog_preserve_mb = 10000 as uint16_t;
        }
        ChangeLogMaxSize = changelog_preserve_mb as uint64_t;
        ChangeLogMaxSize = ChangeLogMaxSize
            .wrapping_mul((1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as uint64_t);
        ChangelogSaveMode = cfg_getuint8(
            b"CHANGELOG_SAVE_MODE\0".as_ptr() as *const ::core::ffi::c_char,
            0 as uint8_t,
        );
        if ChangelogSaveMode as ::core::ffi::c_int > SAVEMODE_MAX {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"CHANGELOG_SAVE_MODE - wrong value - using 0 (write in background)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            ChangelogSaveMode = SAVEMODE_BACKGROUND as uint8_t;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn changelog_init() -> ::core::ffi::c_int {
    unsafe {
        changelog_reload();
        main_time_register_fname(
            1 as uint32_t,
            0 as uint32_t,
            Some(changelog_sendnop as unsafe extern "C" fn() -> ()),
            b"changelog_sendnop\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_reload_register_fname(
            Some(changelog_reload as unsafe extern "C" fn() -> ()),
            b"changelog_reload\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_info_register_fname(
            Some(changelog_info as unsafe extern "C" fn(*mut FILE) -> ()),
            b"changelog_info\0".as_ptr() as *const ::core::ffi::c_char,
        );
        currentfd = ::core::ptr::null_mut::<FILE>();
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn changelog_findfirstversion(
    mut fname: *const ::core::ffi::c_char,
) -> uint64_t {
    unsafe {
        let mut buff: [uint8_t; 50] = [0; 50];
        let mut s: int32_t = 0;
        let mut p: int32_t = 0;
        let mut fv: uint64_t = 0;
        let mut fd: ::core::ffi::c_int = 0;
        fd = open(fname, O_RDONLY);
        if fd < 0 as ::core::ffi::c_int {
            return 0 as uint64_t;
        }
        s = read(
            fd,
            &raw mut buff as *mut uint8_t as *mut ::core::ffi::c_void,
            50 as size_t,
        ) as int32_t;
        close(fd);
        if s <= 0 as int32_t {
            return 0 as uint64_t;
        }
        fv = 0 as uint64_t;
        p = 0 as ::core::ffi::c_int as int32_t;
        while p < s
            && buff[p as usize] as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
            && buff[p as usize] as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
        {
            fv = fv.wrapping_mul(10 as uint64_t);
            fv = fv.wrapping_add(
                (buff[p as usize] as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as uint64_t,
            );
            p += 1;
        }
        if p >= s || buff[p as usize] as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            return 0 as uint64_t;
        }
        return fv;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn changelog_findlastversion(
    mut fname: *const ::core::ffi::c_char,
) -> uint64_t {
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
        let mut lv: uint64_t = 0;
        let mut fd: ::core::ffi::c_int = 0;
        fd = open(fname, O_RDONLY);
        if fd < 0 as ::core::ffi::c_int {
            return 0 as uint64_t;
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
                    close(fd);
                    return 0 as uint64_t;
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
                    close(fd);
                    return 0 as uint64_t;
                }
                buffpos = size as uint32_t;
                size = 0 as uint64_t;
            }
            while buffpos > 0 as uint32_t {
                buffpos = buffpos.wrapping_sub(1);
                if buff[buffpos as usize] as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
                    || size.wrapping_add(buffpos as uint64_t) == 0 as uint64_t
                {
                    if lastnewline == 0 as uint64_t {
                        lastnewline = size.wrapping_add(buffpos as uint64_t);
                    } else {
                        if lastnewline.wrapping_add(1 as uint64_t) != st.st_size as uint64_t {
                            close(fd);
                            return 0 as uint64_t;
                        }
                        if size.wrapping_add(buffpos as uint64_t) > 0 as uint64_t {
                            buffpos = buffpos.wrapping_add(1);
                        }
                        lv = 0 as uint64_t;
                        while buffpos < 32800 as uint32_t
                            && buff[buffpos as usize] as ::core::ffi::c_int
                                >= '0' as ::core::ffi::c_int
                            && buff[buffpos as usize] as ::core::ffi::c_int
                                <= '9' as ::core::ffi::c_int
                        {
                            lv = lv.wrapping_mul(10 as uint64_t);
                            lv = lv.wrapping_add(
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
                            lv = 0 as uint64_t;
                        }
                        close(fd);
                        return lv;
                    }
                }
            }
        }
        close(fd);
        return 0 as uint64_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn changelog_checkname(
    mut fname: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ptr: *const ::core::ffi::c_char = fname;
        if strncmp(
            ptr,
            b"changelog.\0".as_ptr() as *const ::core::ffi::c_char,
            10 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            ptr = ptr.offset(10 as ::core::ffi::c_int as isize);
            if *ptr as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                && *ptr as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
            {
                while *ptr as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && *ptr as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    ptr = ptr.offset(1);
                }
                if strcmp(ptr, b".mfs\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    return 1 as ::core::ffi::c_int;
                }
            }
        } else if strncmp(
            ptr,
            b"changelog_ml.\0".as_ptr() as *const ::core::ffi::c_char,
            13 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            ptr = ptr.offset(13 as ::core::ffi::c_int as isize);
            if *ptr as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                && *ptr as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
            {
                while *ptr as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && *ptr as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    ptr = ptr.offset(1);
                }
                if strcmp(ptr, b".mfs\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    return 1 as ::core::ffi::c_int;
                }
            }
        } else if strncmp(
            ptr,
            b"changelog_ml_back.\0".as_ptr() as *const ::core::ffi::c_char,
            18 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            ptr = ptr.offset(18 as ::core::ffi::c_int as isize);
            if *ptr as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                && *ptr as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
            {
                while *ptr as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && *ptr as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    ptr = ptr.offset(1);
                }
                if strcmp(ptr, b".mfs\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    return 1 as ::core::ffi::c_int;
                }
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
