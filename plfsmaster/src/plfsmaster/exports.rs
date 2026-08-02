pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
    static mut stderr: *mut FILE;
    unsafe fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    unsafe fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    unsafe fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn __getdelim(
        __lineptr: *mut *mut ::core::ffi::c_char,
        __n: *mut size_t,
        __delimiter: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    unsafe fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
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
    unsafe fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    unsafe fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    unsafe fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    unsafe fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn getpwuid_r(
        __uid: __uid_t,
        __resultbuf: *mut passwd,
        __buffer: *mut ::core::ffi::c_char,
        __buflen: size_t,
        __result: *mut *mut passwd,
    ) -> ::core::ffi::c_int;
    unsafe fn getpwnam_r(
        __name: *const ::core::ffi::c_char,
        __resultbuf: *mut passwd,
        __buffer: *mut ::core::ffi::c_char,
        __buflen: size_t,
        __result: *mut *mut passwd,
    ) -> ::core::ffi::c_int;
    unsafe fn getgrnam_r(
        __name: *const ::core::ffi::c_char,
        __resultbuf: *mut group,
        __buffer: *mut ::core::ffi::c_char,
        __buflen: size_t,
        __result: *mut *mut group,
    ) -> ::core::ffi::c_int;
    unsafe fn md5_init(ctx: *mut md5ctx);
    unsafe fn md5_update(ctx: *mut md5ctx, buff: *const uint8_t, leng: uint32_t);
    unsafe fn md5_final(digest: *mut uint8_t, ctx: *mut md5ctx);
    unsafe fn main_destruct_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn cfg_isdefined(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    unsafe fn cfg_use_option(name: *const ::core::ffi::c_char, value: *const ::core::ffi::c_char);
    unsafe fn cfg_getstr(
        name: *const ::core::ffi::c_char,
        def: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn mycrc32(crc: uint32_t, block: *const ::core::ffi::c_void, leng: uint32_t)
    -> uint32_t;
    unsafe fn parse_speriod(
        str: *const ::core::ffi::c_char,
        ret: *mut uint32_t,
    ) -> ::core::ffi::c_int;
    unsafe fn parse_hperiod(
        str: *const ::core::ffi::c_char,
        ret: *mut uint32_t,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
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
pub struct passwd {
    pub pw_name: *mut ::core::ffi::c_char,
    pub pw_passwd: *mut ::core::ffi::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut ::core::ffi::c_char,
    pub pw_dir: *mut ::core::ffi::c_char,
    pub pw_shell: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut ::core::ffi::c_char,
    pub gr_passwd: *mut ::core::ffi::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut ::core::ffi::c_char,
}
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _md5ctx {
    pub state: [uint32_t; 4],
    pub count: [uint32_t; 2],
    pub buffer: [uint8_t; 64],
}
pub type md5ctx = _md5ctx;
pub type exports = _exports;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _exports {
    pub pleng: uint32_t,
    pub path: *const uint8_t,
    pub fromip: uint32_t,
    pub toip: uint32_t,
    pub minversion: uint32_t,
    pub passworddigest: [uint8_t; 16],
    #[bitfield(name = "alldirs", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "needpassword", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "meta", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "rootredefined", ty = "::core::ffi::c_uint", bits = "3..=3")]
    pub alldirs_needpassword_meta_rootredefined: [u8; 1],
    pub sesflags: uint8_t,
    pub sclassgroups: uint16_t,
    pub umask: uint16_t,
    pub mintrashretention: uint32_t,
    pub maxtrashretention: uint32_t,
    pub rootuid: uint32_t,
    pub rootgid: uint32_t,
    pub mapalluid: uint32_t,
    pub mapallgid: uint32_t,
    pub disables: uint32_t,
    pub next: *mut _exports,
}
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut ::core::ffi::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    unsafe {
        return __getdelim(__lineptr, __n, '\n' as ::core::ffi::c_int, __stream);
    }
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EXPORT_GROUPS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const MFS_STATUS_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFS_ERROR_EACCES: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFS_ERROR_NOPASSWORD: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const MFS_ERROR_BADPASSWORD: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SESFLAG_READONLY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SESFLAG_DYNAMICIP: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SESFLAG_IGNOREGID: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SESFLAG_ADMIN: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SESFLAG_MAPALL: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const DISABLE_BIT_CHOWN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DISABLE_BIT_CHMOD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DISABLE_BIT_SYMLINK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const DISABLE_BIT_MKFIFO: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const DISABLE_BIT_MKDEV: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DISABLE_BIT_MKSOCK: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const DISABLE_BIT_MKDIR: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const DISABLE_BIT_UNLINK: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const DISABLE_BIT_RMDIR: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const DISABLE_BIT_RENAME: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const DISABLE_BIT_MOVE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const DISABLE_BIT_LINK: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const DISABLE_BIT_CREATE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const DISABLE_BIT_READDIR: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const DISABLE_BIT_READ: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const DISABLE_BIT_WRITE: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const DISABLE_BIT_TRUNCATE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const DISABLE_BIT_SETLENGTH: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const DISABLE_BIT_APPENDCHUNKS: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const DISABLE_BIT_SNAPSHOT: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const DISABLE_BIT_SETTRASH: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const DISABLE_BIT_SETSCLASS: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const DISABLE_BIT_SETEATTR: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const DISABLE_BIT_SETXATTR: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const DISABLE_BIT_SETFACL: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const DISABLE_CHOWN: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_CHOWN;
pub const DISABLE_CHMOD: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_CHMOD;
pub const DISABLE_SYMLINK: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_SYMLINK;
pub const DISABLE_MKFIFO: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_MKFIFO;
pub const DISABLE_MKDEV: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_MKDEV;
pub const DISABLE_MKSOCK: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_MKSOCK;
pub const DISABLE_MKDIR: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_MKDIR;
pub const DISABLE_UNLINK: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_UNLINK;
pub const DISABLE_RMDIR: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_RMDIR;
pub const DISABLE_RENAME: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_RENAME;
pub const DISABLE_MOVE: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_MOVE;
pub const DISABLE_LINK: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_LINK;
pub const DISABLE_CREATE: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_CREATE;
pub const DISABLE_READDIR: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_READDIR;
pub const DISABLE_READ: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_READ;
pub const DISABLE_WRITE: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_WRITE;
pub const DISABLE_TRUNCATE: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << DISABLE_BIT_TRUNCATE;
pub const DISABLE_SETLENGTH: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << DISABLE_BIT_SETLENGTH;
pub const DISABLE_APPENDCHUNKS: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << DISABLE_BIT_APPENDCHUNKS;
pub const DISABLE_SNAPSHOT: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << DISABLE_BIT_SNAPSHOT;
pub const DISABLE_SETTRASH: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << DISABLE_BIT_SETTRASH;
pub const DISABLE_SETSCLASS: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << DISABLE_BIT_SETSCLASS;
pub const DISABLE_SETEATTR: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << DISABLE_BIT_SETEATTR;
pub const DISABLE_SETXATTR: ::core::ffi::c_uint =
    (1 as ::core::ffi::c_uint) << DISABLE_BIT_SETXATTR;
pub const DISABLE_SETFACL: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint) << DISABLE_BIT_SETFACL;
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
pub const MFSLOG_ERRNO_SYSLOG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_ERRNO_SYSLOG_STDERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn murmur3_32(
    mut buf: *const uint8_t,
    mut len: uint32_t,
    mut hash: uint32_t,
) -> uint32_t {
    unsafe {
        static mut c1: uint32_t = 0xcc9e2d51 as uint32_t;
        static mut c2: uint32_t = 0x1b873593 as uint32_t;
        static mut r1: uint32_t = 15 as uint32_t;
        static mut r2: uint32_t = 13 as uint32_t;
        static mut m: uint32_t = 5 as uint32_t;
        static mut n: uint32_t = 0xe6546b64 as uint32_t;
        let mut i: uint32_t = 0;
        let mut k: uint32_t = 0;
        i = 0 as uint32_t;
        while i < len.wrapping_div(4 as uint32_t) {
            k = get32bit(&raw mut buf);
            k = k.wrapping_mul(c1);
            k = k << r1 | k >> (32 as uint32_t).wrapping_sub(r1);
            k = k.wrapping_mul(c2);
            hash ^= k;
            hash = (hash << r2 | hash >> (32 as uint32_t).wrapping_sub(r2))
                .wrapping_mul(m)
                .wrapping_add(n);
            i = i.wrapping_add(1);
        }
        k = 0 as uint32_t;
        's_91: {
            'c_5784: {
                match len & 3 as uint32_t {
                    3 => {
                        k ^= ((*buf.offset(2 as isize) as ::core::ffi::c_int)
                            << 16 as ::core::ffi::c_int) as uint32_t;
                    }
                    2 => {}
                    1 => {
                        break 'c_5784;
                    }
                    _ => {
                        break 's_91;
                    }
                }
                k ^= ((*buf.offset(1 as isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                    as uint32_t;
            }
            k ^= *buf.offset(0 as isize) as uint32_t;
            k = k.wrapping_mul(c1);
            k = k << r1 | k >> (32 as uint32_t).wrapping_sub(r1);
            k = k.wrapping_mul(c2);
            hash ^= k;
        }
        hash ^= len;
        hash ^= hash >> 16 as ::core::ffi::c_int;
        hash = (hash as ::core::ffi::c_uint).wrapping_mul(0x85ebca6b as ::core::ffi::c_uint)
            as uint32_t;
        hash ^= hash >> 13 as ::core::ffi::c_int;
        hash = (hash as ::core::ffi::c_uint).wrapping_mul(0xc2b2ae35 as ::core::ffi::c_uint)
            as uint32_t;
        hash ^= hash >> 16 as ::core::ffi::c_int;
        return hash;
    }
}
static mut exports_records: *mut exports = ::core::ptr::null_mut::<exports>();
static mut exports_csum: uint64_t = 0;
static mut ExportsFileName: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exports_entry_checksum(mut e: *mut exports) -> uint64_t {
    unsafe {
        let mut csum: uint64_t = 0;
        let mut edata: [uint8_t; 62] = [0; 62];
        let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut crc: uint32_t = 0;
        let mut murmur: uint32_t = 0;
        ptr = &raw mut edata as *mut uint8_t;
        put32bit(&raw mut ptr, (*e).fromip);
        put32bit(&raw mut ptr, (*e).toip);
        put32bit(&raw mut ptr, (*e).minversion);
        if (*e).needpassword() != 0 {
            memcpy(
                ptr as *mut ::core::ffi::c_void,
                &raw mut (*e).passworddigest as *mut uint8_t as *const ::core::ffi::c_void,
                16 as size_t,
            );
        } else {
            memset(
                ptr as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                16 as size_t,
            );
        }
        ptr = ptr.offset(16 as ::core::ffi::c_int as isize);
        put8bit(
            &raw mut ptr,
            ((((*e).alldirs() as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
                + (((*e).needpassword() as ::core::ffi::c_int) << 2 as ::core::ffi::c_int)
                + (((*e).meta() as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                + (*e).rootredefined() as ::core::ffi::c_int) as uint8_t,
        );
        put8bit(&raw mut ptr, (*e).sesflags);
        put16bit(&raw mut ptr, (*e).sclassgroups);
        put16bit(&raw mut ptr, (*e).umask);
        put32bit(&raw mut ptr, (*e).mintrashretention);
        put32bit(&raw mut ptr, (*e).maxtrashretention);
        put32bit(&raw mut ptr, (*e).rootuid);
        put32bit(&raw mut ptr, (*e).rootgid);
        put32bit(&raw mut ptr, (*e).mapalluid);
        put32bit(&raw mut ptr, (*e).mapallgid);
        put32bit(&raw mut ptr, (*e).disables);
        crc = mycrc32(
            0xffffffff as uint32_t,
            &raw mut edata as *mut uint8_t as *const ::core::ffi::c_void,
            62 as uint32_t,
        );
        murmur = murmur3_32(
            &raw mut edata as *mut uint8_t,
            62 as uint32_t,
            0 as uint32_t,
        );
        if (*e).pleng > 0 as uint32_t {
            crc = mycrc32(crc, (*e).path as *const ::core::ffi::c_void, (*e).pleng);
            murmur = murmur3_32((*e).path, (*e).pleng, murmur);
        }
        csum = crc as uint64_t;
        csum <<= 32 as ::core::ffi::c_int;
        csum |= murmur as uint64_t;
        return csum;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exports_checksum() -> uint64_t {
    unsafe {
        return exports_csum;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exports_strsep(
    mut stringp: *mut *mut ::core::ffi::c_char,
    mut delim: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut spanp: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut c: ::core::ffi::c_int = 0;
        let mut sc: ::core::ffi::c_int = 0;
        let mut tok: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        s = *stringp;
        if s.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        while *s as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || *s as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
        {
            s = s.offset(1);
        }
        tok = s;
        loop {
            let c2rust_fresh0 = s;
            s = s.offset(1);
            c = *c2rust_fresh0 as ::core::ffi::c_int;
            spanp = delim;
            loop {
                let c2rust_fresh1 = spanp;
                spanp = spanp.offset(1);
                sc = *c2rust_fresh1 as ::core::ffi::c_int;
                if sc == c {
                    if c == 0 as ::core::ffi::c_int {
                        *stringp = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    } else {
                        *stringp = s;
                    }
                    s = s.offset(-1);
                    while s > tok
                        && (*s.offset(-1 as isize) as ::core::ffi::c_int
                            == ' ' as ::core::ffi::c_int
                            || *s.offset(-1 as isize) as ::core::ffi::c_int
                                == '\t' as ::core::ffi::c_int)
                    {
                        s = s.offset(-1);
                    }
                    *s = 0 as ::core::ffi::c_char;
                    return tok;
                }
                if sc == 0 as ::core::ffi::c_int {
                    break;
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exports_info_size(mut versmode: uint8_t) -> uint32_t {
    unsafe {
        let mut e: *mut exports = ::core::ptr::null_mut::<exports>();
        let mut size: uint32_t = 0 as uint32_t;
        let mut add: uint32_t = 0 as uint32_t;
        if versmode as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            add = add.wrapping_add(10 as uint32_t);
        }
        if versmode as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            add = add.wrapping_add(2 as uint32_t);
        }
        if versmode as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
            add = add.wrapping_add(4 as uint32_t);
        }
        e = exports_records;
        while !e.is_null() {
            if (*e).meta() != 0 {
                size = size.wrapping_add((35 as uint32_t).wrapping_add(add));
            } else {
                size =
                    size.wrapping_add((35 as uint32_t).wrapping_add(add).wrapping_add((*e).pleng));
            }
            e = (*e).next as *mut exports;
        }
        return size;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exports_info_data(mut versmode: uint8_t, mut buff: *mut uint8_t) {
    unsafe {
        let mut e: *mut exports = ::core::ptr::null_mut::<exports>();
        e = exports_records;
        while !e.is_null() {
            put32bit(&raw mut buff, (*e).fromip);
            put32bit(&raw mut buff, (*e).toip);
            if (*e).meta() != 0 {
                put32bit(&raw mut buff, 1 as uint32_t);
                put8bit(&raw mut buff, '.' as uint8_t);
            } else {
                put32bit(&raw mut buff, (*e).pleng.wrapping_add(1 as uint32_t));
                put8bit(&raw mut buff, '/' as uint8_t);
                if (*e).pleng > 0 as uint32_t {
                    memcpy(
                        buff as *mut ::core::ffi::c_void,
                        (*e).path as *const ::core::ffi::c_void,
                        (*e).pleng as size_t,
                    );
                    buff = buff.offset((*e).pleng as isize);
                }
            }
            put32bit(&raw mut buff, (*e).minversion);
            put8bit(
                &raw mut buff,
                ((if (*e).alldirs() as ::core::ffi::c_int != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) + (if (*e).needpassword() as ::core::ffi::c_int != 0 {
                    2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as uint8_t,
            );
            put8bit(&raw mut buff, (*e).sesflags);
            if versmode as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                put16bit(&raw mut buff, (*e).umask);
            }
            put32bit(&raw mut buff, (*e).rootuid);
            put32bit(&raw mut buff, (*e).rootgid);
            put32bit(&raw mut buff, (*e).mapalluid);
            put32bit(&raw mut buff, (*e).mapallgid);
            if versmode as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                if versmode as ::core::ffi::c_int > 3 as ::core::ffi::c_int {
                    put16bit(&raw mut buff, (*e).sclassgroups);
                } else {
                    put8bit(&raw mut buff, 0 as uint8_t);
                    put8bit(&raw mut buff, 0 as uint8_t);
                }
                put32bit(&raw mut buff, (*e).mintrashretention);
                put32bit(&raw mut buff, (*e).maxtrashretention);
            }
            if versmode as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
                put32bit(&raw mut buff, (*e).disables);
            }
            e = (*e).next as *mut exports;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exports_check(
    mut ip: uint32_t,
    mut version: uint32_t,
    mut path: *const uint8_t,
    mut rndcode: *const uint8_t,
    mut passcode: *const uint8_t,
    mut sesflags: *mut uint8_t,
    mut umaskval: *mut uint16_t,
    mut rootuid: *mut uint32_t,
    mut rootgid: *mut uint32_t,
    mut mapalluid: *mut uint32_t,
    mut mapallgid: *mut uint32_t,
    mut sclassgroups: *mut uint16_t,
    mut mintrashretention: *mut uint32_t,
    mut maxtrashretention: *mut uint32_t,
    mut disables: *mut uint32_t,
) -> uint8_t {
    unsafe {
        let mut p: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut pleng: uint32_t = 0;
        let mut i: uint32_t = 0;
        let mut rndstate: uint8_t = 0;
        let mut meta: uint8_t = 0;
        let mut ok: ::core::ffi::c_int = 0;
        let mut nopass: ::core::ffi::c_int = 0;
        let mut md5c: md5ctx = md5ctx {
            state: [0; 4],
            count: [0; 2],
            buffer: [0; 64],
        };
        let mut entrydigest: [uint8_t; 16] = [0; 16];
        let mut e: *mut exports = ::core::ptr::null_mut::<exports>();
        let mut f: *mut exports = ::core::ptr::null_mut::<exports>();
        meta = (if path.is_null() {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
        if meta as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            p = path;
            while *p as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
                p = p.offset(1);
            }
            pleng = 0 as uint32_t;
            while *p.offset(pleng as isize) != 0 {
                pleng = pleng.wrapping_add(1);
            }
            while pleng > 0 as uint32_t
                && *p.offset(pleng.wrapping_sub(1 as uint32_t) as isize) as ::core::ffi::c_int
                    == '/' as ::core::ffi::c_int
            {
                pleng = pleng.wrapping_sub(1);
            }
        } else {
            p = ::core::ptr::null::<uint8_t>();
            pleng = 0 as uint32_t;
        }
        rndstate = 0 as uint8_t;
        if !rndcode.is_null() {
            i = 0 as uint32_t;
            while i < 32 as uint32_t {
                rndstate = (rndstate as ::core::ffi::c_int
                    | *rndcode.offset(i as isize) as ::core::ffi::c_int)
                    as uint8_t;
                i = i.wrapping_add(1);
            }
        }
        nopass = 0 as ::core::ffi::c_int;
        f = ::core::ptr::null_mut::<exports>();
        e = exports_records;
        while !e.is_null() {
            ok = 0 as ::core::ffi::c_int;
            if ip >= (*e).fromip
                && ip <= (*e).toip
                && version >= (*e).minversion
                && meta as ::core::ffi::c_int == (*e).meta() as ::core::ffi::c_int
            {
                if meta != 0 {
                    ok = 1 as ::core::ffi::c_int;
                } else if (*e).pleng == 0 as uint32_t {
                    if pleng == 0 as uint32_t {
                        ok = 1 as ::core::ffi::c_int;
                    } else if (*e).alldirs() != 0 {
                        ok = 1 as ::core::ffi::c_int;
                    }
                } else if pleng == (*e).pleng
                    && memcmp(
                        p as *const ::core::ffi::c_void,
                        (*e).path as *const ::core::ffi::c_void,
                        pleng as size_t,
                    ) == 0 as ::core::ffi::c_int
                {
                    ok = 1 as ::core::ffi::c_int;
                } else if (*e).alldirs() as ::core::ffi::c_int != 0
                    && pleng > (*e).pleng
                    && *p.offset((*e).pleng as isize) as ::core::ffi::c_int
                        == '/' as ::core::ffi::c_int
                    && memcmp(
                        p as *const ::core::ffi::c_void,
                        (*e).path as *const ::core::ffi::c_void,
                        (*e).pleng as size_t,
                    ) == 0 as ::core::ffi::c_int
                {
                    ok = 1 as ::core::ffi::c_int;
                }
                if ok != 0 && (*e).needpassword() as ::core::ffi::c_int != 0 {
                    if rndstate as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                        || rndcode.is_null()
                        || passcode.is_null()
                    {
                        ok = 0 as ::core::ffi::c_int;
                        nopass = 1 as ::core::ffi::c_int;
                    } else {
                        md5_init(&raw mut md5c);
                        md5_update(&raw mut md5c, rndcode as *const uint8_t, 16 as uint32_t);
                        md5_update(
                            &raw mut md5c,
                            &raw mut (*e).passworddigest as *mut uint8_t,
                            16 as uint32_t,
                        );
                        md5_update(
                            &raw mut md5c,
                            rndcode.offset(16 as ::core::ffi::c_int as isize),
                            16 as uint32_t,
                        );
                        md5_final(&raw mut entrydigest as *mut uint8_t, &raw mut md5c);
                        if memcmp(
                            &raw mut entrydigest as *mut uint8_t as *const ::core::ffi::c_void,
                            passcode as *const ::core::ffi::c_void,
                            16 as size_t,
                        ) != 0 as ::core::ffi::c_int
                        {
                            ok = 0 as ::core::ffi::c_int;
                            nopass = 1 as ::core::ffi::c_int;
                        }
                    }
                }
            }
            if ok != 0 {
                if f.is_null() {
                    f = e;
                } else if (*e).sesflags as ::core::ffi::c_int & SESFLAG_READONLY
                    == 0 as ::core::ffi::c_int
                    && (*f).sesflags as ::core::ffi::c_int & SESFLAG_READONLY
                        != 0 as ::core::ffi::c_int
                {
                    f = e;
                } else if (*e).rootuid == 0 as uint32_t && (*f).rootuid != 0 as uint32_t {
                    f = e;
                } else if (*e).sesflags as ::core::ffi::c_int & SESFLAG_ADMIN
                    != 0 as ::core::ffi::c_int
                    && (*f).sesflags as ::core::ffi::c_int & SESFLAG_ADMIN
                        == 0 as ::core::ffi::c_int
                {
                    f = e;
                } else if (*e).needpassword() as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                    && (*f).needpassword() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                {
                    f = e;
                } else if (*e).pleng > (*f).pleng {
                    f = e;
                }
            }
            e = (*e).next as *mut exports;
        }
        if f.is_null() {
            if nopass != 0 {
                if rndstate as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    || rndcode.is_null()
                    || passcode.is_null()
                {
                    return MFS_ERROR_NOPASSWORD as uint8_t;
                } else {
                    return MFS_ERROR_BADPASSWORD as uint8_t;
                }
            }
            return MFS_ERROR_EACCES as uint8_t;
        }
        *sesflags = (*f).sesflags;
        *umaskval = (*f).umask;
        *rootuid = (*f).rootuid;
        *rootgid = (*f).rootgid;
        *mapalluid = (*f).mapalluid;
        *mapallgid = (*f).mapallgid;
        *sclassgroups = (*f).sclassgroups;
        *mintrashretention = (*f).mintrashretention;
        *maxtrashretention = (*f).maxtrashretention;
        *disables = (*f).disables;
        return MFS_STATUS_OK as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exports_freelist(mut arec: *mut exports) {
    unsafe {
        let mut drec: *mut exports = ::core::ptr::null_mut::<exports>();
        while !arec.is_null() {
            drec = arec;
            arec = (*arec).next as *mut exports;
            if !(*drec).path.is_null() {
                free((*drec).path as *mut uint8_t as *mut ::core::ffi::c_void);
            }
            free(drec as *mut ::core::ffi::c_void);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exports_parsenet(
    mut net: *const ::core::ffi::c_char,
    mut fromip: *mut uint32_t,
    mut toip: *mut uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut ip: uint32_t = 0;
        let mut i: uint32_t = 0;
        let mut octet: uint32_t = 0;
        if *net.offset(0 as isize) as ::core::ffi::c_int == '*' as ::core::ffi::c_int
            && *net.offset(1 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            *fromip = 0 as uint32_t;
            *toip = 0xffffffff as ::core::ffi::c_uint as uint32_t;
            return 0 as ::core::ffi::c_int;
        }
        ip = 0 as uint32_t;
        i = 0 as uint32_t;
        while i < 4 as uint32_t {
            if *net as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                && *net as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
            {
                octet = 0 as uint32_t;
                while *net as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && *net as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    octet = octet.wrapping_mul(10 as uint32_t);
                    octet = octet.wrapping_add(
                        (*net as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as uint32_t,
                    );
                    net = net.offset(1);
                    if octet > 255 as uint32_t {
                        return -1 as ::core::ffi::c_int;
                    }
                }
            } else {
                return -1 as ::core::ffi::c_int;
            }
            if i < 3 as uint32_t {
                if *net as ::core::ffi::c_int != '.' as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
                net = net.offset(1);
            }
            ip = ip.wrapping_mul(256 as uint32_t);
            ip = ip.wrapping_add(octet);
            i = i.wrapping_add(1);
        }
        if *net as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            *fromip = ip;
            *toip = ip;
            return 0 as ::core::ffi::c_int;
        }
        if *net as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
            *fromip = ip;
            ip = 0 as uint32_t;
            net = net.offset(1);
            i = 0 as uint32_t;
            while i < 4 as uint32_t {
                if *net as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && *net as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    octet = 0 as uint32_t;
                    while *net as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                        && *net as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                    {
                        octet = octet.wrapping_mul(10 as uint32_t);
                        octet = octet.wrapping_add(
                            (*net as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as uint32_t,
                        );
                        net = net.offset(1);
                        if octet > 255 as uint32_t {
                            return -1 as ::core::ffi::c_int;
                        }
                    }
                } else {
                    return -1 as ::core::ffi::c_int;
                }
                if i == 0 as uint32_t
                    && *net as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && octet <= 32 as uint32_t
                {
                    ip = 0xffffffff as ::core::ffi::c_uint as uint32_t;
                    if octet < 32 as uint32_t {
                        ip <<= (32 as uint32_t).wrapping_sub(octet);
                    }
                    break;
                } else {
                    if i < 3 as uint32_t {
                        if *net as ::core::ffi::c_int != '.' as ::core::ffi::c_int {
                            return -1 as ::core::ffi::c_int;
                        }
                        net = net.offset(1);
                    }
                    ip = ip.wrapping_mul(256 as uint32_t);
                    ip = ip.wrapping_add(octet);
                    i = i.wrapping_add(1);
                }
            }
            if *net as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
            *fromip &= ip;
            *toip = *fromip | ip ^ 0xffffffff as uint32_t;
            return 0 as ::core::ffi::c_int;
        }
        if *net as ::core::ffi::c_int == '-' as ::core::ffi::c_int {
            *fromip = ip;
            ip = 0 as uint32_t;
            net = net.offset(1);
            i = 0 as uint32_t;
            while i < 4 as uint32_t {
                if *net as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                    && *net as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                {
                    octet = 0 as uint32_t;
                    while *net as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                        && *net as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                    {
                        octet = octet.wrapping_mul(10 as uint32_t);
                        octet = octet.wrapping_add(
                            (*net as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as uint32_t,
                        );
                        net = net.offset(1);
                        if octet > 255 as uint32_t {
                            return -1 as ::core::ffi::c_int;
                        }
                    }
                } else {
                    return -1 as ::core::ffi::c_int;
                }
                if i < 3 as uint32_t {
                    if *net as ::core::ffi::c_int != '.' as ::core::ffi::c_int {
                        return -1 as ::core::ffi::c_int;
                    }
                    net = net.offset(1);
                }
                ip = ip.wrapping_mul(256 as uint32_t);
                ip = ip.wrapping_add(octet);
                i = i.wrapping_add(1);
            }
            if *net as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
            *toip = ip;
            return 0 as ::core::ffi::c_int;
        }
        return -1 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exports_parsesclassgroups(
    mut sgstr: *const ::core::ffi::c_char,
    mut sclassgroups: *mut uint16_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut group: uint8_t = 0;
        let mut result: uint16_t = 0;
        result = 0 as uint16_t;
        if *sgstr as ::core::ffi::c_int == '-' as ::core::ffi::c_int
            && *sgstr.offset(1 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            *sclassgroups = 0 as uint16_t;
            return 0 as ::core::ffi::c_int;
        }
        while *sgstr as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
            && *sgstr as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
        {
            group = 0 as uint8_t;
            while *sgstr as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                && *sgstr as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                && (group as ::core::ffi::c_int) < EXPORT_GROUPS
            {
                group = (group as ::core::ffi::c_int * 10 as ::core::ffi::c_int
                    + (*sgstr as ::core::ffi::c_int - '0' as ::core::ffi::c_int))
                    as uint8_t;
                sgstr = sgstr.offset(1);
            }
            if group as ::core::ffi::c_int >= EXPORT_GROUPS {
                return -1 as ::core::ffi::c_int;
            }
            result = (result as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << group as ::core::ffi::c_int)
                as uint16_t;
            if *sgstr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                *sclassgroups = result;
                return 0 as ::core::ffi::c_int;
            }
            if *sgstr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
            sgstr = sgstr.offset(1);
        }
        return -1 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exports_parsegoal(
    mut goalstr: *const ::core::ffi::c_char,
    mut goal: *mut uint8_t,
) -> ::core::ffi::c_int {
    unsafe {
        if (*goalstr as ::core::ffi::c_int) < '1' as ::core::ffi::c_int
            || *goalstr as ::core::ffi::c_int > '9' as ::core::ffi::c_int
            || *goalstr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
        {
            return -1 as ::core::ffi::c_int;
        }
        *goal = (*goalstr as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as uint8_t;
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exports_parseumask(
    mut umaskstr: *const ::core::ffi::c_char,
    mut umaskval: *mut uint16_t,
) -> ::core::ffi::c_int {
    unsafe {
        if *umaskstr as ::core::ffi::c_int != '0' as ::core::ffi::c_int
            || (*umaskstr.offset(1 as isize) as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
            || *umaskstr.offset(1 as isize) as ::core::ffi::c_int > '7' as ::core::ffi::c_int
            || (*umaskstr.offset(2 as isize) as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
            || *umaskstr.offset(2 as isize) as ::core::ffi::c_int > '7' as ::core::ffi::c_int
            || (*umaskstr.offset(3 as isize) as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
            || *umaskstr.offset(3 as isize) as ::core::ffi::c_int > '7' as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        *umaskval = ((*umaskstr.offset(1 as isize) as ::core::ffi::c_int
            - '0' as ::core::ffi::c_int)
            * 64 as ::core::ffi::c_int
            + (*umaskstr.offset(2 as isize) as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                * 8 as ::core::ffi::c_int
            + (*umaskstr.offset(3 as isize) as ::core::ffi::c_int - '0' as ::core::ffi::c_int))
            as uint16_t;
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exports_parseversion(
    mut verstr: *const ::core::ffi::c_char,
    mut version: *mut uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut vp: uint32_t = 0;
        if (*verstr as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
            || *verstr as ::core::ffi::c_int > '9' as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        vp = 0 as uint32_t;
        while *verstr as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
            && *verstr as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
        {
            vp = vp.wrapping_mul(10 as uint32_t);
            vp = vp.wrapping_add(
                (*verstr as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as uint32_t,
            );
            verstr = verstr.offset(1);
        }
        if vp > 255 as uint32_t
            || *verstr as ::core::ffi::c_int != '.' as ::core::ffi::c_int
                && *verstr as ::core::ffi::c_int != 0
        {
            return -1 as ::core::ffi::c_int;
        }
        *version = vp << 16 as ::core::ffi::c_int;
        if *verstr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        verstr = verstr.offset(1);
        if (*verstr as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
            || *verstr as ::core::ffi::c_int > '9' as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        vp = 0 as uint32_t;
        while *verstr as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
            && *verstr as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
        {
            vp = vp.wrapping_mul(10 as uint32_t);
            vp = vp.wrapping_add(
                (*verstr as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as uint32_t,
            );
            verstr = verstr.offset(1);
        }
        if vp > 255 as uint32_t
            || *verstr as ::core::ffi::c_int != '.' as ::core::ffi::c_int
                && *verstr as ::core::ffi::c_int != 0
        {
            return -1 as ::core::ffi::c_int;
        }
        *version = (*version).wrapping_add(vp << 8 as ::core::ffi::c_int);
        if *verstr as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        verstr = verstr.offset(1);
        if (*verstr as ::core::ffi::c_int) < '0' as ::core::ffi::c_int
            || *verstr as ::core::ffi::c_int > '9' as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        vp = 0 as uint32_t;
        while *verstr as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
            && *verstr as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
        {
            vp = vp.wrapping_mul(10 as uint32_t);
            vp = vp.wrapping_add(
                (*verstr as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as uint32_t,
            );
            verstr = verstr.offset(1);
        }
        if vp > 255 as uint32_t || *verstr as ::core::ffi::c_int != 0 {
            return -1 as ::core::ffi::c_int;
        }
        *version = (*version).wrapping_add(vp);
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exports_parseuidgid(
    mut maproot: *mut ::core::ffi::c_char,
    mut lineno: uint32_t,
    mut ruid: *mut uint32_t,
    mut rgid: *mut uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut uptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut gptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut grrec: *mut group = ::core::ptr::null_mut::<group>();
        let mut grp: group = group {
            gr_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            gr_passwd: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            gr_gid: 0,
            gr_mem: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        };
        let mut pwrec: *mut passwd = ::core::ptr::null_mut::<passwd>();
        let mut pwd: passwd = passwd {
            pw_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pw_passwd: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pw_uid: 0,
            pw_gid: 0,
            pw_gecos: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pw_dir: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            pw_shell: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        let mut pwgrbuff: [::core::ffi::c_char; 16384] = [0; 16384];
        let mut uid: uint32_t = 0;
        let mut gid: uint32_t = 0;
        let mut gidok: ::core::ffi::c_int = 0;
        uptr = maproot;
        gptr = maproot;
        while *gptr as ::core::ffi::c_int != 0
            && *gptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int
        {
            gptr = gptr.offset(1);
        }
        if *gptr as ::core::ffi::c_int == ':' as ::core::ffi::c_int {
            *gptr = 0 as ::core::ffi::c_char;
            gid = 0 as uint32_t;
            eptr = gptr.offset(1 as ::core::ffi::c_int as isize);
            while *eptr as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                && *eptr as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
            {
                gid = gid.wrapping_mul(10 as uint32_t);
                gid = gid.wrapping_add(
                    (*eptr as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as uint32_t,
                );
                eptr = eptr.offset(1);
            }
            if *eptr as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                if getgrnam_r(
                    gptr.offset(1 as ::core::ffi::c_int as isize),
                    &raw mut grp,
                    &raw mut pwgrbuff as *mut ::core::ffi::c_char,
                    16384 as size_t,
                    &raw mut grrec,
                ) != 0 as ::core::ffi::c_int
                {
                    grrec = ::core::ptr::null_mut::<group>();
                }
                if grrec.is_null() {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"mfsexports/maproot: can't find group named '%s' defined in line: %u\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        gptr.offset(1 as ::core::ffi::c_int as isize),
                        lineno,
                    );
                    return -1 as ::core::ffi::c_int;
                }
                gid = (*grrec).gr_gid as uint32_t;
            }
            gidok = 1 as ::core::ffi::c_int;
        } else {
            gidok = 0 as ::core::ffi::c_int;
            gid = 0 as uint32_t;
            gptr = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        uid = 0 as uint32_t;
        eptr = uptr;
        while *eptr as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
            && *eptr as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
        {
            uid = uid.wrapping_mul(10 as uint32_t);
            uid = uid.wrapping_add(
                (*eptr as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as uint32_t,
            );
            eptr = eptr.offset(1);
        }
        if *eptr as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if getpwnam_r(
                uptr,
                &raw mut pwd,
                &raw mut pwgrbuff as *mut ::core::ffi::c_char,
                16384 as size_t,
                &raw mut pwrec,
            ) != 0 as ::core::ffi::c_int
            {
                pwrec = ::core::ptr::null_mut::<passwd>();
            }
            if pwrec.is_null() {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"mfsexports/maproot: can't find user named '%s' defined in line: %u\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    uptr,
                    lineno,
                );
                return -1 as ::core::ffi::c_int;
            }
            *ruid = (*pwrec).pw_uid as uint32_t;
            if gidok == 0 as ::core::ffi::c_int {
                *rgid = (*pwrec).pw_gid as uint32_t;
            } else {
                *rgid = gid;
            }
            return 0 as ::core::ffi::c_int;
        } else if gidok == 1 as ::core::ffi::c_int {
            *ruid = uid;
            *rgid = gid;
            return 0 as ::core::ffi::c_int;
        } else {
            if getpwuid_r(
                uid as __uid_t,
                &raw mut pwd,
                &raw mut pwgrbuff as *mut ::core::ffi::c_char,
                16384 as size_t,
                &raw mut pwrec,
            ) != 0 as ::core::ffi::c_int
            {
                pwrec = ::core::ptr::null_mut::<passwd>();
            }
            if pwrec.is_null() {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"mfsexports/maproot: can't determine gid, because can't find user with uid %u defined in line: %u\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    uid,
                    lineno,
                );
                return -1 as ::core::ffi::c_int;
            }
            *ruid = (*pwrec).pw_uid as uint32_t;
            *rgid = (*pwrec).pw_gid as uint32_t;
            return 0 as ::core::ffi::c_int;
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exports_parsedisable(
    mut disablestr: *mut ::core::ffi::c_char,
    mut lineno: uint32_t,
    mut disables: *mut uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut o: ::core::ffi::c_int = 0;
        loop {
            p = exports_strsep(
                &raw mut disablestr,
                b":\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if p.is_null() {
                break;
            }
            o = 0 as ::core::ffi::c_int;
            match *p as ::core::ffi::c_int {
                97 => {
                    if strcmp(p, b"appendchunks\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables =
                            (*disables as ::core::ffi::c_uint | DISABLE_APPENDCHUNKS) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    }
                }
                99 => {
                    if strcmp(p, b"chown\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables = (*disables as ::core::ffi::c_uint | DISABLE_CHOWN) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    } else if strcmp(p, b"chmod\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables = (*disables as ::core::ffi::c_uint | DISABLE_CHMOD) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    } else if strcmp(p, b"create\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables = (*disables as ::core::ffi::c_uint | DISABLE_CREATE) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    }
                }
                108 => {
                    if strcmp(p, b"link\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables = (*disables as ::core::ffi::c_uint | DISABLE_LINK) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    }
                }
                109 => {
                    if strcmp(p, b"mkfifo\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables = (*disables as ::core::ffi::c_uint | DISABLE_MKFIFO) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    } else if strcmp(p, b"mkdev\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables = (*disables as ::core::ffi::c_uint | DISABLE_MKDEV) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    } else if strcmp(p, b"mksock\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables = (*disables as ::core::ffi::c_uint | DISABLE_MKSOCK) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    } else if strcmp(p, b"mkdir\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables = (*disables as ::core::ffi::c_uint | DISABLE_MKDIR) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    } else if strcmp(p, b"move\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables = (*disables as ::core::ffi::c_uint | DISABLE_MOVE) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    }
                }
                114 => {
                    if strcmp(p, b"rmdir\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables = (*disables as ::core::ffi::c_uint | DISABLE_RMDIR) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    } else if strcmp(p, b"rename\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables = (*disables as ::core::ffi::c_uint | DISABLE_RENAME) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    } else if strcmp(p, b"readdir\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables =
                            (*disables as ::core::ffi::c_uint | DISABLE_READDIR) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    } else if strcmp(p, b"read\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables = (*disables as ::core::ffi::c_uint | DISABLE_READ) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    }
                }
                115 => {
                    if strcmp(p, b"symlink\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables =
                            (*disables as ::core::ffi::c_uint | DISABLE_SYMLINK) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    } else if strcmp(p, b"setlength\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables =
                            (*disables as ::core::ffi::c_uint | DISABLE_SETLENGTH) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    } else if strcmp(p, b"snapshot\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables =
                            (*disables as ::core::ffi::c_uint | DISABLE_SNAPSHOT) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    } else if strcmp(p, b"settrash\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables =
                            (*disables as ::core::ffi::c_uint | DISABLE_SETTRASH) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    } else if strcmp(p, b"setsclass\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables =
                            (*disables as ::core::ffi::c_uint | DISABLE_SETSCLASS) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    } else if strcmp(p, b"seteattr\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables =
                            (*disables as ::core::ffi::c_uint | DISABLE_SETEATTR) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    } else if strcmp(p, b"setxattr\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables =
                            (*disables as ::core::ffi::c_uint | DISABLE_SETXATTR) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    } else if strcmp(p, b"setfacl\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables =
                            (*disables as ::core::ffi::c_uint | DISABLE_SETFACL) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    }
                }
                116 => {
                    if strcmp(p, b"truncate\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables =
                            (*disables as ::core::ffi::c_uint | DISABLE_TRUNCATE) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    }
                }
                117 => {
                    if strcmp(p, b"unlink\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables = (*disables as ::core::ffi::c_uint | DISABLE_UNLINK) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    }
                }
                119 => {
                    if strcmp(p, b"write\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        *disables = (*disables as ::core::ffi::c_uint | DISABLE_WRITE) as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                    }
                }
                _ => {}
            }
            if o == 0 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"mfsexports: unknown disable command '%s' in line: %u (ignored)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    p,
                    lineno,
                );
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exports_parseoptions(
    mut opts: *mut ::core::ffi::c_char,
    mut lineno: uint32_t,
    mut arec: *mut exports,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut o: ::core::ffi::c_int = 0;
        let mut ctx: md5ctx = md5ctx {
            state: [0; 4],
            count: [0; 2],
            buffer: [0; 64],
        };
        let mut mingoal: uint8_t = 1 as uint8_t;
        let mut maxgoal: uint8_t = 9 as uint8_t;
        let mut goal_defined: uint8_t = 0 as uint8_t;
        let mut sclassgroups_defined: uint8_t = 0 as uint8_t;
        let mut sclassgroups: uint16_t = 0;
        loop {
            p = exports_strsep(&raw mut opts, b",\0".as_ptr() as *const ::core::ffi::c_char);
            if p.is_null() {
                break;
            }
            o = 0 as ::core::ffi::c_int;
            match *p as ::core::ffi::c_int {
                114 => {
                    if strcmp(p, b"ro\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        (*arec).sesflags =
                            ((*arec).sesflags as ::core::ffi::c_int | SESFLAG_READONLY) as uint8_t;
                        o = 1 as ::core::ffi::c_int;
                    } else if strcmp(p, b"readonly\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        (*arec).sesflags =
                            ((*arec).sesflags as ::core::ffi::c_int | SESFLAG_READONLY) as uint8_t;
                        o = 1 as ::core::ffi::c_int;
                    } else if strcmp(p, b"rw\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        (*arec).sesflags =
                            ((*arec).sesflags as ::core::ffi::c_int & !SESFLAG_READONLY) as uint8_t;
                        o = 1 as ::core::ffi::c_int;
                    } else if strcmp(p, b"readwrite\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        (*arec).sesflags =
                            ((*arec).sesflags as ::core::ffi::c_int & !SESFLAG_READONLY) as uint8_t;
                        o = 1 as ::core::ffi::c_int;
                    }
                }
                105 => {
                    if strcmp(p, b"ignoregid\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        if (*arec).meta() != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"meta option ignored: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                p,
                            );
                        } else {
                            (*arec).sesflags = ((*arec).sesflags as ::core::ffi::c_int
                                | SESFLAG_IGNOREGID)
                                as uint8_t;
                        }
                        o = 1 as ::core::ffi::c_int;
                    }
                }
                97 => {
                    if strcmp(p, b"alldirs\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        if (*arec).meta() != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"meta option ignored: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                p,
                            );
                        } else {
                            (*arec).set_alldirs(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        }
                        o = 1 as ::core::ffi::c_int;
                    } else if strcmp(p, b"admin\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        if (*arec).meta() != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"meta option ignored: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                p,
                            );
                        } else {
                            (*arec).sesflags =
                                ((*arec).sesflags as ::core::ffi::c_int | SESFLAG_ADMIN) as uint8_t;
                        }
                        o = 1 as ::core::ffi::c_int;
                    }
                }
                100 => {
                    if strcmp(p, b"dynamicip\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        (*arec).sesflags =
                            ((*arec).sesflags as ::core::ffi::c_int | SESFLAG_DYNAMICIP) as uint8_t;
                        o = 1 as ::core::ffi::c_int;
                    } else if strncmp(
                        p,
                        b"disable=\0".as_ptr() as *const ::core::ffi::c_char,
                        8 as size_t,
                    ) == 0 as ::core::ffi::c_int
                    {
                        o = 1 as ::core::ffi::c_int;
                        if (*arec).meta() != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"meta option ignored: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                p,
                            );
                        } else if exports_parsedisable(
                            p.offset(8 as ::core::ffi::c_int as isize),
                            lineno,
                            &raw mut (*arec).disables,
                        ) < 0 as ::core::ffi::c_int
                        {
                            return -1 as ::core::ffi::c_int;
                        }
                    }
                }
                99 => {
                    if strcmp(
                        p,
                        b"canchangequota\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        if (*arec).meta() != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"meta option ignored: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                p,
                            );
                        } else {
                            (*arec).sesflags =
                                ((*arec).sesflags as ::core::ffi::c_int | SESFLAG_ADMIN) as uint8_t;
                        }
                        o = 1 as ::core::ffi::c_int;
                    }
                }
                109 => {
                    if strncmp(
                        p,
                        b"maproot=\0".as_ptr() as *const ::core::ffi::c_char,
                        8 as size_t,
                    ) == 0 as ::core::ffi::c_int
                    {
                        o = 1 as ::core::ffi::c_int;
                        if (*arec).meta() != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"meta option ignored: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                p,
                            );
                        } else {
                            if exports_parseuidgid(
                                p.offset(8 as ::core::ffi::c_int as isize),
                                lineno,
                                &raw mut (*arec).rootuid,
                                &raw mut (*arec).rootgid,
                            ) < 0 as ::core::ffi::c_int
                            {
                                return -1 as ::core::ffi::c_int;
                            }
                            (*arec)
                                .set_rootredefined(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        }
                    } else if strncmp(
                        p,
                        b"mapall=\0".as_ptr() as *const ::core::ffi::c_char,
                        7 as size_t,
                    ) == 0 as ::core::ffi::c_int
                    {
                        o = 1 as ::core::ffi::c_int;
                        if (*arec).meta() != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"meta option ignored: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                p,
                            );
                        } else {
                            if exports_parseuidgid(
                                p.offset(7 as ::core::ffi::c_int as isize),
                                lineno,
                                &raw mut (*arec).mapalluid,
                                &raw mut (*arec).mapallgid,
                            ) < 0 as ::core::ffi::c_int
                            {
                                return -1 as ::core::ffi::c_int;
                            }
                            (*arec).sesflags = ((*arec).sesflags as ::core::ffi::c_int
                                | SESFLAG_MAPALL)
                                as uint8_t;
                        }
                    } else if strncmp(
                        p,
                        b"md5pass=\0".as_ptr() as *const ::core::ffi::c_char,
                        8 as size_t,
                    ) == 0 as ::core::ffi::c_int
                    {
                        let mut ptr: *mut ::core::ffi::c_char =
                            p.offset(8 as ::core::ffi::c_int as isize);
                        let mut i: uint32_t = 0 as uint32_t;
                        o = 1 as ::core::ffi::c_int;
                        while *ptr as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                            && *ptr as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                            || *ptr as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
                                && *ptr as ::core::ffi::c_int <= 'f' as ::core::ffi::c_int
                            || *ptr as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                                && *ptr as ::core::ffi::c_int <= 'F' as ::core::ffi::c_int
                        {
                            ptr = ptr.offset(1);
                            i = i.wrapping_add(1);
                        }
                        if *ptr as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            && i == 32 as uint32_t
                        {
                            ptr = p.offset(8 as ::core::ffi::c_int as isize);
                            i = 0 as uint32_t;
                            while i < 16 as uint32_t {
                                if *ptr as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                                    && *ptr as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                                {
                                    (*arec).passworddigest[i as usize] =
                                        ((*ptr as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                                            << 4 as ::core::ffi::c_int)
                                            as uint8_t;
                                } else if *ptr as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
                                    && *ptr as ::core::ffi::c_int <= 'f' as ::core::ffi::c_int
                                {
                                    (*arec).passworddigest[i as usize] =
                                        ((*ptr as ::core::ffi::c_int - 'a' as ::core::ffi::c_int
                                            + 10 as ::core::ffi::c_int)
                                            << 4 as ::core::ffi::c_int)
                                            as uint8_t;
                                } else {
                                    (*arec).passworddigest[i as usize] =
                                        ((*ptr as ::core::ffi::c_int - 'A' as ::core::ffi::c_int
                                            + 10 as ::core::ffi::c_int)
                                            << 4 as ::core::ffi::c_int)
                                            as uint8_t;
                                }
                                ptr = ptr.offset(1);
                                if *ptr as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                                    && *ptr as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
                                {
                                    (*arec).passworddigest[i as usize] = ((*arec).passworddigest
                                        [i as usize]
                                        as ::core::ffi::c_int
                                        + (*ptr as ::core::ffi::c_int - '0' as ::core::ffi::c_int))
                                        as uint8_t;
                                } else if *ptr as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
                                    && *ptr as ::core::ffi::c_int <= 'f' as ::core::ffi::c_int
                                {
                                    (*arec).passworddigest[i as usize] = ((*arec).passworddigest
                                        [i as usize]
                                        as ::core::ffi::c_int
                                        + (*ptr as ::core::ffi::c_int - 'a' as ::core::ffi::c_int
                                            + 10 as ::core::ffi::c_int))
                                        as uint8_t;
                                } else {
                                    (*arec).passworddigest[i as usize] = ((*arec).passworddigest
                                        [i as usize]
                                        as ::core::ffi::c_int
                                        + (*ptr as ::core::ffi::c_int - 'A' as ::core::ffi::c_int
                                            + 10 as ::core::ffi::c_int))
                                        as uint8_t;
                                }
                                ptr = ptr.offset(1);
                                i = i.wrapping_add(1);
                            }
                            (*arec)
                                .set_needpassword(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        } else {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"mfsexports: incorrect md5pass definition (%s) in line: %u\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                p,
                                lineno,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                    } else if strncmp(
                        p,
                        b"minversion=\0".as_ptr() as *const ::core::ffi::c_char,
                        11 as size_t,
                    ) == 0 as ::core::ffi::c_int
                    {
                        o = 1 as ::core::ffi::c_int;
                        if exports_parseversion(
                            p.offset(11 as ::core::ffi::c_int as isize),
                            &raw mut (*arec).minversion,
                        ) < 0 as ::core::ffi::c_int
                        {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"mfsexports: incorrect minversion definition (%s) in line: %u\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                p,
                                lineno,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                    } else if strncmp(
                        p,
                        b"mingoal=\0".as_ptr() as *const ::core::ffi::c_char,
                        8 as size_t,
                    ) == 0 as ::core::ffi::c_int
                    {
                        o = 1 as ::core::ffi::c_int;
                        if sclassgroups_defined != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"mfsexports: mingoal defined together with sclassgroupsin line: %u - use only sclassgroups\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                lineno,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_NOTICE,
                            b"mfsexports: mingoal option is deprecated, use sclassgroups instead\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                        if exports_parsegoal(
                            p.offset(8 as ::core::ffi::c_int as isize),
                            &raw mut mingoal,
                        ) < 0 as ::core::ffi::c_int
                        {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"mfsexports: incorrect mingoal definition (%s) in line: %u\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                p,
                                lineno,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                        if mingoal as ::core::ffi::c_int > maxgoal as ::core::ffi::c_int {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"mfsexports: mingoal>maxgoal in definition (%s) in line: %u\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                p,
                                lineno,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                        goal_defined = 1 as uint8_t;
                    } else if strncmp(
                        p,
                        b"maxgoal=\0".as_ptr() as *const ::core::ffi::c_char,
                        8 as size_t,
                    ) == 0 as ::core::ffi::c_int
                    {
                        o = 1 as ::core::ffi::c_int;
                        if sclassgroups_defined != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"mfsexports: maxgoal defined together with sclassgroupsin line: %u - use only sclassgroups\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                lineno,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_NOTICE,
                            b"mfsexports: maxgoal option is deprecated, use sclassgroups instead\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                        if exports_parsegoal(
                            p.offset(8 as ::core::ffi::c_int as isize),
                            &raw mut maxgoal,
                        ) < 0 as ::core::ffi::c_int
                        {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"mfsexports: incorrect maxgoal definition (%s) in line: %u\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                p,
                                lineno,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                        if mingoal as ::core::ffi::c_int > maxgoal as ::core::ffi::c_int {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"mfsexports: maxgoal<mingoal in definition (%s) in line: %u\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                p,
                                lineno,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                        goal_defined = 1 as uint8_t;
                    } else if strncmp(
                        p,
                        b"mintrashretention=\0".as_ptr() as *const ::core::ffi::c_char,
                        18 as size_t,
                    ) == 0 as ::core::ffi::c_int
                    {
                        o = 1 as ::core::ffi::c_int;
                        if parse_hperiod(
                            p.offset(18 as ::core::ffi::c_int as isize),
                            &raw mut (*arec).mintrashretention,
                        ) < 0 as ::core::ffi::c_int
                        {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"mfsexports: incorrect mintrashretention definition (%s) in line: %u\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                p,
                                lineno,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                        (*arec).mintrashretention =
                            (*arec).mintrashretention.wrapping_mul(3600 as uint32_t);
                        if (*arec).mintrashretention > (*arec).maxtrashretention {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"mfsexports: mintrashretention>maxtrashretention in definition (%s) in line: %u\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                p,
                                lineno,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                    } else if strncmp(
                        p,
                        b"maxtrashretention=\0".as_ptr() as *const ::core::ffi::c_char,
                        18 as size_t,
                    ) == 0 as ::core::ffi::c_int
                    {
                        o = 1 as ::core::ffi::c_int;
                        if parse_hperiod(
                            p.offset(18 as ::core::ffi::c_int as isize),
                            &raw mut (*arec).maxtrashretention,
                        ) < 0 as ::core::ffi::c_int
                        {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"mfsexports: incorrect maxtrashretention definition (%s) in line: %u\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                p,
                                lineno,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                        (*arec).maxtrashretention =
                            (*arec).maxtrashretention.wrapping_mul(3600 as uint32_t);
                        if (*arec).mintrashretention > (*arec).maxtrashretention {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"mfsexports: maxtrashretention<mintrashretention in definition (%s) in line: %u\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                p,
                                lineno,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                    } else if strncmp(
                        p,
                        b"mintrashtime=\0".as_ptr() as *const ::core::ffi::c_char,
                        13 as size_t,
                    ) == 0 as ::core::ffi::c_int
                    {
                        o = 1 as ::core::ffi::c_int;
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_NOTICE,
                            b"mfsexports: mintrashtime option is deprecated, use mintrashretention instead\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                        if parse_speriod(
                            p.offset(13 as ::core::ffi::c_int as isize),
                            &raw mut (*arec).mintrashretention,
                        ) < 0 as ::core::ffi::c_int
                        {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"mfsexports: incorrect mintrashtime definition (%s) in line: %u\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                p,
                                lineno,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                        if (*arec).mintrashretention > (*arec).maxtrashretention {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"mfsexports: mintrashtime>maxtrashtime in definition (%s) in line: %u\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                p,
                                lineno,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                    } else if strncmp(
                        p,
                        b"maxtrashtime=\0".as_ptr() as *const ::core::ffi::c_char,
                        13 as size_t,
                    ) == 0 as ::core::ffi::c_int
                    {
                        o = 1 as ::core::ffi::c_int;
                        mfs_log(
                            MFSLOG_SYSLOG_STDERR,
                            MFSLOG_NOTICE,
                            b"mfsexports: maxtrashtime option is deprecated, use maxtrashretention instead\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                        if parse_speriod(
                            p.offset(13 as ::core::ffi::c_int as isize),
                            &raw mut (*arec).maxtrashretention,
                        ) < 0 as ::core::ffi::c_int
                        {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"mfsexports: incorrect maxtrashtime definition (%s) in line: %u\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                p,
                                lineno,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                        if (*arec).mintrashretention > (*arec).maxtrashretention {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"mfsexports: maxtrashtime<mintrashtime in definition (%s) in line: %u\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                p,
                                lineno,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                    }
                }
                117 => {
                    if strncmp(
                        p,
                        b"umask=\0".as_ptr() as *const ::core::ffi::c_char,
                        6 as size_t,
                    ) == 0 as ::core::ffi::c_int
                    {
                        if exports_parseumask(
                            p.offset(6 as ::core::ffi::c_int as isize),
                            &raw mut (*arec).umask,
                        ) < 0 as ::core::ffi::c_int
                        {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"mfsexports: incorrect umask definition (%s) in line: %u\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                p,
                                lineno,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                        o = 1 as ::core::ffi::c_int;
                    }
                }
                112 => {
                    if strncmp(
                        p,
                        b"password=\0".as_ptr() as *const ::core::ffi::c_char,
                        9 as size_t,
                    ) == 0 as ::core::ffi::c_int
                    {
                        md5_init(&raw mut ctx);
                        md5_update(
                            &raw mut ctx,
                            p.offset(9 as ::core::ffi::c_int as isize) as *mut uint8_t,
                            strlen(p.offset(9 as ::core::ffi::c_int as isize)) as uint32_t,
                        );
                        md5_final(
                            &raw mut (*arec).passworddigest as *mut uint8_t,
                            &raw mut ctx,
                        );
                        (*arec).set_needpassword(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        o = 1 as ::core::ffi::c_int;
                    }
                }
                115 => {
                    if strncmp(
                        p,
                        b"sclassgroups=\0".as_ptr() as *const ::core::ffi::c_char,
                        13 as size_t,
                    ) == 0 as ::core::ffi::c_int
                    {
                        if goal_defined != 0 {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"mfsexports: sclassgroups defined together with mingoal/maxgoal in line: %u - use only sclassgroups\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                lineno,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                        if exports_parsesclassgroups(
                            p.offset(13 as ::core::ffi::c_int as isize),
                            &raw mut sclassgroups,
                        ) < 0 as ::core::ffi::c_int
                        {
                            mfs_log(
                                MFSLOG_SYSLOG_STDERR,
                                MFSLOG_WARNING,
                                b"mfsexports: incorrect sclassgroups definition (%s) in line: %u\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                p,
                                lineno,
                            );
                            return -1 as ::core::ffi::c_int;
                        }
                        if sclassgroups_defined != 0 {
                            (*arec).sclassgroups = ((*arec).sclassgroups as ::core::ffi::c_int
                                | sclassgroups as ::core::ffi::c_int)
                                as uint16_t;
                        } else {
                            (*arec).sclassgroups = sclassgroups;
                            sclassgroups_defined = 1 as uint8_t;
                        }
                        o = 1 as ::core::ffi::c_int;
                    }
                }
                _ => {}
            }
            if o == 0 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"mfsexports: unknown option '%s' in line: %u (ignored)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    p,
                    lineno,
                );
            }
        }
        if goal_defined != 0 {
            (*arec).sclassgroups = 1 as uint16_t;
            o = mingoal as ::core::ffi::c_int;
            while o <= maxgoal as ::core::ffi::c_int {
                (*arec).sclassgroups = ((*arec).sclassgroups as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << o)
                    as uint16_t;
                o += 1;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exports_parseline(
    mut line: *mut ::core::ffi::c_char,
    mut lineno: uint32_t,
    mut arec: *mut exports,
    mut defaults: *mut *mut exports,
) -> ::core::ffi::c_int {
    unsafe {
        let mut net: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut pleng: uint32_t = 0;
        if (*defaults).is_null() {
            (*arec).pleng = 0 as uint32_t;
            (*arec).path = ::core::ptr::null::<uint8_t>();
            (*arec).fromip = 0 as uint32_t;
            (*arec).toip = 0 as uint32_t;
            (*arec).minversion = 0 as uint32_t;
            (*arec).set_alldirs(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*arec).set_needpassword(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*arec).set_meta(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*arec).set_rootredefined(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*arec).sesflags = SESFLAG_READONLY as uint8_t;
            (*arec).umask = 0 as uint16_t;
            (*arec).sclassgroups = 0xffff as uint16_t;
            (*arec).mintrashretention = 0 as uint32_t;
            (*arec).maxtrashretention = 0xffffffff as ::core::ffi::c_uint as uint32_t;
            (*arec).rootuid = 999 as uint32_t;
            (*arec).rootgid = 999 as uint32_t;
            (*arec).mapalluid = 999 as uint32_t;
            (*arec).mapallgid = 999 as uint32_t;
            (*arec).disables = 0 as uint32_t;
            (*arec).next = ::core::ptr::null_mut::<_exports>();
        } else {
            memcpy(
                arec as *mut ::core::ffi::c_void,
                *defaults as *const ::core::ffi::c_void,
                ::core::mem::size_of::<exports>(),
            );
        }
        p = line;
        while *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
        if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '#' as ::core::ffi::c_int
        {
            return -1 as ::core::ffi::c_int;
        }
        net = p;
        while *p as ::core::ffi::c_int != 0
            && *p as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
            && *p as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
        if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"mfsexports: incomplete definition in line: %u\0".as_ptr()
                    as *const ::core::ffi::c_char,
                lineno,
            );
            return -1 as ::core::ffi::c_int;
        }
        *p = 0 as ::core::ffi::c_char;
        p = p.offset(1);
        if strcasecmp(net, b"DEFAULTS\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            while *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                || *p as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
            {
                p = p.offset(1);
            }
            if exports_parseoptions(p, lineno, arec) < 0 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
            if (*arec).sesflags as ::core::ffi::c_int & SESFLAG_MAPALL != 0
                && (*arec).rootredefined() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                (*arec).rootuid = (*arec).mapalluid;
                (*arec).rootgid = (*arec).mapallgid;
            }
            if (*defaults).is_null() {
                *defaults = malloc(::core::mem::size_of::<exports>()) as *mut exports;
                if (*defaults).is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/exports.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1222 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"*defaults\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/exports.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1222 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"*defaults\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if *defaults
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *mut exports
                {
                    let mut _mfs_errorstring: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/exports.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1222 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"*defaults\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/exports.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1222 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"*defaults\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring,
                    );
                    abort();
                }
            }
            memcpy(
                *defaults as *mut ::core::ffi::c_void,
                arec as *const ::core::ffi::c_void,
                ::core::mem::size_of::<exports>(),
            );
            return 0 as ::core::ffi::c_int;
        } else if exports_parsenet(net, &raw mut (*arec).fromip, &raw mut (*arec).toip)
            < 0 as ::core::ffi::c_int
        {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_WARNING,
                b"mfsexports: incorrect ip/network definition in line: %u\0".as_ptr()
                    as *const ::core::ffi::c_char,
                lineno,
            );
            return -1 as ::core::ffi::c_int;
        }
        while *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
        if *p.offset(0 as isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            && (*p.offset(1 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                || *p.offset(1 as isize) as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                || *p.offset(1 as isize) as ::core::ffi::c_int == '\t' as ::core::ffi::c_int)
        {
            path = ::core::ptr::null_mut::<::core::ffi::c_char>();
            pleng = 0 as uint32_t;
            if (*arec).rootredefined() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*arec).rootuid = 0 as uint32_t;
                (*arec).rootgid = 0 as uint32_t;
            }
            (*arec).set_meta(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            p = p.offset(1);
        } else {
            while *p as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
                p = p.offset(1);
            }
            path = p;
            pleng = 0 as uint32_t;
            while *p as ::core::ffi::c_int != 0
                && *p as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
                && *p as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
            {
                p = p.offset(1);
                pleng = pleng.wrapping_add(1);
            }
            while pleng > 0 as uint32_t
                && *path.offset(pleng.wrapping_sub(1 as uint32_t) as isize) as ::core::ffi::c_int
                    == '/' as ::core::ffi::c_int
            {
                pleng = pleng.wrapping_sub(1);
            }
        }
        if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (*arec).pleng = pleng;
            if pleng > 0 as uint32_t {
                (*arec).path =
                    malloc(pleng.wrapping_add(1 as uint32_t) as size_t) as *const uint8_t;
                if (*arec).path.is_null() {
                    fprintf(
                        stderr,
                        b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/exports.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1262 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"arec->path\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/exports.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1262 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"arec->path\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    abort();
                } else if (*arec).path
                    == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                        -1 as ::core::ffi::c_int as usize,
                    ) as *const uint8_t
                {
                    let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                        strerr(*__errno_location());
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_ERR,
                        b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/exports.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1262 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"arec->path\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_0,
                    );
                    fprintf(
                        stderr,
                        b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/tmp/moosefs-ref/mfsmaster/exports.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1262 as ::core::ffi::c_int as ::core::ffi::c_uint,
                        b"arec->path\0".as_ptr() as *const ::core::ffi::c_char,
                        _mfs_errorstring_0,
                    );
                    abort();
                }
                memcpy(
                    (*arec).path as *mut uint8_t as *mut ::core::ffi::c_void,
                    path as *const ::core::ffi::c_void,
                    pleng as size_t,
                );
                *((*arec).path as *mut uint8_t).offset(pleng as isize) = 0 as uint8_t;
            } else {
                (*arec).path = ::core::ptr::null::<uint8_t>();
            }
            return 0 as ::core::ffi::c_int;
        }
        while *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            || *p as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
        if exports_parseoptions(p, lineno, arec) < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        if (*arec).sesflags as ::core::ffi::c_int & SESFLAG_MAPALL != 0
            && (*arec).rootredefined() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            (*arec).rootuid = (*arec).mapalluid;
            (*arec).rootgid = (*arec).mapallgid;
        }
        (*arec).pleng = pleng;
        if pleng > 0 as uint32_t {
            (*arec).path = malloc(pleng.wrapping_add(1 as uint32_t) as size_t) as *const uint8_t;
            if (*arec).path.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/exports.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1287 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"arec->path\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/exports.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1287 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"arec->path\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if (*arec).path
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *const uint8_t
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/exports.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1287 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"arec->path\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/exports.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1287 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"arec->path\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_1,
                );
                abort();
            }
            memcpy(
                (*arec).path as *mut uint8_t as *mut ::core::ffi::c_void,
                path as *const ::core::ffi::c_void,
                pleng as size_t,
            );
            *((*arec).path as *mut uint8_t).offset(pleng as isize) = 0 as uint8_t;
        } else {
            (*arec).path = ::core::ptr::null::<uint8_t>();
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exports_loadexports() {
    unsafe {
        let mut fd: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut linebuff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut lbsize: size_t = 0;
        let mut s: uint32_t = 0;
        let mut lineno: uint32_t = 0;
        let mut newexports: *mut exports = ::core::ptr::null_mut::<exports>();
        let mut netail: *mut *mut exports = ::core::ptr::null_mut::<*mut exports>();
        let mut arec: *mut exports = ::core::ptr::null_mut::<exports>();
        let mut defaults: *mut exports = ::core::ptr::null_mut::<exports>();
        fd = fopen(
            ExportsFileName,
            b"r\0".as_ptr() as *const ::core::ffi::c_char,
        ) as *mut FILE;
        if fd.is_null() {
            if *__errno_location() == ENOENT {
                if !exports_records.is_null() {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"mfsexports configuration file (%s) not found - exports not changed\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        ExportsFileName,
                    );
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"mfsexports configuration file (%s) not found - no exports !!!\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ExportsFileName,
                    );
                    fprintf(
                        stderr,
                        b"mfsexports configuration file (%s) not found - please create one (you can copy %s.sample to get a base configuration)\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        ExportsFileName,
                        ExportsFileName,
                    );
                }
            } else if !exports_records.is_null() {
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG,
                    MFSLOG_WARNING,
                    b"can't open mfsexports configuration file (%s) - exports not changed, error\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    ExportsFileName,
                );
            } else {
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"can't open mfsexports configuration file (%s) - no exports !!!, error\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    ExportsFileName,
                );
            }
            return;
        }
        newexports = ::core::ptr::null_mut::<exports>();
        netail = &raw mut newexports;
        lineno = 1 as uint32_t;
        arec = malloc(::core::mem::size_of::<exports>()) as *mut exports;
        if arec.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/exports.c\0".as_ptr() as *const ::core::ffi::c_char,
                1326 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"arec\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/exports.c\0".as_ptr() as *const ::core::ffi::c_char,
                1326 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"arec\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if arec
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut exports
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/exports.c\0".as_ptr() as *const ::core::ffi::c_char,
                1326 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"arec\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsmaster/exports.c\0".as_ptr() as *const ::core::ffi::c_char,
                1326 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"arec\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        defaults = ::core::ptr::null_mut::<exports>();
        lbsize = 10000 as size_t;
        linebuff = malloc(lbsize) as *mut ::core::ffi::c_char;
        while getline(&raw mut linebuff, &raw mut lbsize, fd) != -1 as __ssize_t {
            s = strlen(linebuff) as uint32_t;
            while s > 0 as uint32_t
                && (*linebuff.offset(s.wrapping_sub(1 as uint32_t) as isize) as ::core::ffi::c_int
                    == '\r' as ::core::ffi::c_int
                    || *linebuff.offset(s.wrapping_sub(1 as uint32_t) as isize)
                        as ::core::ffi::c_int
                        == '\n' as ::core::ffi::c_int
                    || *linebuff.offset(s.wrapping_sub(1 as uint32_t) as isize)
                        as ::core::ffi::c_int
                        == '\t' as ::core::ffi::c_int
                    || *linebuff.offset(s.wrapping_sub(1 as uint32_t) as isize)
                        as ::core::ffi::c_int
                        == ' ' as ::core::ffi::c_int)
            {
                s = s.wrapping_sub(1);
            }
            if s > 0 as uint32_t {
                *linebuff.offset(s as isize) = 0 as ::core::ffi::c_char;
                if exports_parseline(linebuff, lineno, arec, &raw mut defaults)
                    >= 0 as ::core::ffi::c_int
                {
                    *netail = arec;
                    netail = &raw mut (*arec).next as *mut *mut exports;
                    arec = malloc(::core::mem::size_of::<exports>()) as *mut exports;
                    if arec.is_null() {
                        fprintf(
                            stderr,
                            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/exports.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1341 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"arec\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - out of memory: %s is NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/exports.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1341 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"arec\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        abort();
                    } else if arec
                        == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                            -1 as ::core::ffi::c_int as usize,
                        ) as *mut exports
                    {
                        let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                            strerr(*__errno_location());
                        mfs_log(
                            MFSLOG_SYSLOG,
                            MFSLOG_ERR,
                            b"%s:%u - mmap error on %s, error: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/exports.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1341 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"arec\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring_0,
                        );
                        fprintf(
                            stderr,
                            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/moosefs-ref/mfsmaster/exports.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1341 as ::core::ffi::c_int as ::core::ffi::c_uint,
                            b"arec\0".as_ptr() as *const ::core::ffi::c_char,
                            _mfs_errorstring_0,
                        );
                        abort();
                    }
                }
            }
            lineno = lineno.wrapping_add(1);
        }
        free(linebuff as *mut ::core::ffi::c_void);
        free(arec as *mut ::core::ffi::c_void);
        if !defaults.is_null() {
            free(defaults as *mut ::core::ffi::c_void);
        }
        if ferror(fd) != 0 {
            fclose(fd);
            if !exports_records.is_null() {
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_WARNING,
                    b"error reading mfsexports file - exports not changed\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            } else {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"error reading mfsexports file - no exports !!!\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            exports_freelist(newexports);
            return;
        }
        fclose(fd);
        exports_freelist(exports_records);
        exports_records = newexports;
        exports_csum = 0 as uint64_t;
        arec = exports_records;
        while !arec.is_null() {
            exports_csum = exports_csum.wrapping_add(exports_entry_checksum(arec));
            arec = (*arec).next as *mut exports;
        }
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_INFO,
            b"exports file has been loaded\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exports_reload() {
    unsafe {
        let mut fd: ::core::ffi::c_int = 0;
        if !ExportsFileName.is_null() {
            free(ExportsFileName as *mut ::core::ffi::c_void);
        }
        if cfg_isdefined(b"EXPORTS_FILENAME\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
            ExportsFileName = strdup(
                b"/usr/local/etc/mfs/mfsexports.cfg\0".as_ptr() as *const ::core::ffi::c_char
            );
            if ExportsFileName.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/exports.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1378 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ExportsFileName\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/exports.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1378 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ExportsFileName\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if ExportsFileName
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut ::core::ffi::c_char
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/exports.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1378 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ExportsFileName\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsmaster/exports.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1378 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"ExportsFileName\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            fd = open(ExportsFileName, O_RDONLY);
            if fd < 0 as ::core::ffi::c_int && *__errno_location() == ENOENT {
                let mut tmpname: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                tmpname = strdup(
                    b"/usr/local/etc/mfsexports.cfg\0".as_ptr() as *const ::core::ffi::c_char
                );
                fd = open(tmpname, O_RDONLY);
                if fd >= 0 as ::core::ffi::c_int {
                    free(ExportsFileName as *mut ::core::ffi::c_void);
                    ExportsFileName = tmpname;
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"default sysconf path has changed - please move mfsexports.cfg from /usr/local/etc/ to /usr/local/etc/mfs/\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                } else {
                    free(tmpname as *mut ::core::ffi::c_void);
                }
            }
            if fd >= 0 as ::core::ffi::c_int {
                close(fd);
            }
            cfg_use_option(
                b"EXPORTS_FILENAME\0".as_ptr() as *const ::core::ffi::c_char,
                ExportsFileName,
            );
        } else {
            ExportsFileName = cfg_getstr(
                b"EXPORTS_FILENAME\0".as_ptr() as *const ::core::ffi::c_char,
                b"/usr/local/etc/mfs/mfsexports.cfg\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        exports_loadexports();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exports_term() {
    unsafe {
        exports_freelist(exports_records);
        if !ExportsFileName.is_null() {
            free(ExportsFileName as *mut ::core::ffi::c_void);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn exports_init() -> ::core::ffi::c_int {
    unsafe {
        exports_records = ::core::ptr::null_mut::<exports>();
        ExportsFileName = ::core::ptr::null_mut::<::core::ffi::c_char>();
        exports_reload();
        if exports_records.is_null() {
            fprintf(
                stderr,
                b"no exports defined !!!\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return -1 as ::core::ffi::c_int;
        }
        main_destruct_register_fname(
            Some(exports_term as unsafe extern "C" fn() -> ()),
            b"exports_term\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
}
