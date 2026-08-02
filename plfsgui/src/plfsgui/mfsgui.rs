pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn strftime(
        __s: *mut ::core::ffi::c_char,
        __maxsize: size_t,
        __format: *const ::core::ffi::c_char,
        __tp: *const tm,
    ) -> size_t;
    unsafe fn gmtime(__timer: *const time_t) -> *mut tm;
    unsafe fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    unsafe fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
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
    unsafe fn pipe(__pipedes: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn dup(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn execve(
        __path: *const ::core::ffi::c_char,
        __argv: *const *mut ::core::ffi::c_char,
        __envp: *const *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    unsafe fn fork() -> __pid_t;
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
    unsafe fn __getdelim(
        __lineptr: *mut *mut ::core::ffi::c_char,
        __n: *mut size_t,
        __delimiter: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    unsafe fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    unsafe fn free(__ptr: *mut ::core::ffi::c_void);
    unsafe fn abort() -> !;
    unsafe fn exit(__status: ::core::ffi::c_int) -> !;
    unsafe fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    unsafe fn realpath(
        __name: *const ::core::ffi::c_char,
        __resolved: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
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
    unsafe fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    unsafe fn __errno_location() -> *mut ::core::ffi::c_int;
    unsafe fn cfg_getstr(
        name: *const ::core::ffi::c_char,
        def: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    unsafe fn cfg_getuint32(name: *const ::core::ffi::c_char, def: uint32_t) -> uint32_t;
    unsafe fn md5_init(ctx: *mut md5ctx);
    unsafe fn md5_update(ctx: *mut md5ctx, buff: *const uint8_t, leng: uint32_t);
    unsafe fn md5_final(digest: *mut uint8_t, ctx: *mut md5ctx);
    unsafe fn poll(
        __fds: *mut pollfd,
        __nfds: nfds_t,
        __timeout: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    unsafe fn main_destruct_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn main_reload_register_fname(
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn main_chld_register_fname(
        pid: pid_t,
        fun: Option<unsafe extern "C" fn(pid_t, ::core::ffi::c_int) -> ()>,
        fname: *const ::core::ffi::c_char,
    );
    unsafe fn main_poll_register_fname(
        desc: Option<unsafe extern "C" fn(*mut pollfd, *mut uint32_t) -> ()>,
        serve: Option<unsafe extern "C" fn(*mut pollfd) -> ()>,
        dname: *const ::core::ffi::c_char,
        sname: *const ::core::ffi::c_char,
    );
    unsafe fn main_time_register_fname(
        seconds: uint32_t,
        offset: uint32_t,
        fun: Option<unsafe extern "C" fn() -> ()>,
        fname: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_void;
    unsafe fn monotonic_seconds() -> ::core::ffi::c_double;
    unsafe fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    unsafe fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    unsafe fn univnonblock(fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
    unsafe fn tcptowrite(
        sock: ::core::ffi::c_int,
        buff: *const ::core::ffi::c_void,
        leng: uint32_t,
        msectopart: uint32_t,
        msectoall: uint32_t,
    ) -> int32_t;
    unsafe fn tcpaccept(lsock_0: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn tcpclose(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    unsafe fn kill(__pid: __pid_t, __sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
pub type __uint64_t = u64;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type pid_t = __pid_t;
pub type va_list = __builtin_va_list;
pub type off_t = __off64_t;
pub type ssize_t = isize;
pub type int32_t = i32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _md5ctx {
    pub state: [uint32_t; 4],
    pub count: [uint32_t; 2],
    pub buffer: [uint8_t; 64],
}
pub type md5ctx = _md5ctx;
pub type nfds_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _mimetype {
    pub extl: uint32_t,
    pub ext: *mut ::core::ffi::c_char,
    pub mime: *mut ::core::ffi::c_char,
}
pub type mimetype = _mimetype;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _chld {
    pub pid: pid_t,
    pub next: *mut _chld,
}
pub type chld = _chld;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct requests_str {
    pub rtype: uint8_t,
    pub rl: uint32_t,
    pub request: *mut ::core::ffi::c_char,
    pub fl: uint32_t,
    pub fname: *mut ::core::ffi::c_char,
    pub extra: *mut ::core::ffi::c_char,
    pub mimetype: *const ::core::ffi::c_char,
    pub mtimestr: *mut ::core::ffi::c_char,
    pub etag: *mut ::core::ffi::c_char,
    pub mtime: time_t,
    pub fsize: uint32_t,
    pub fdata: *mut uint8_t,
    pub next: *mut requests_str,
}
pub type requests = requests_str;
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const METHOD_HEAD: C2Rust_Unnamed = 2;
pub const METHOD_GET: C2Rust_Unnamed = 1;
pub const METHOD_NONE: C2Rust_Unnamed = 0;
pub type C2Rust_Unnamed_0 = ::core::ffi::c_uint;
pub const STATUS_INTERROR: C2Rust_Unnamed_0 = 7;
pub const STATUS_BADMETHOD: C2Rust_Unnamed_0 = 6;
pub const STATUS_BADREQUEST: C2Rust_Unnamed_0 = 5;
pub const STATUS_NOTMODIFIED: C2Rust_Unnamed_0 = 4;
pub const STATUS_FORBIDDEN: C2Rust_Unnamed_0 = 3;
pub const STATUS_NOTFOUND: C2Rust_Unnamed_0 = 2;
pub const STATUS_FOUND: C2Rust_Unnamed_0 = 1;
pub const STATUS_NONE: C2Rust_Unnamed_0 = 0;
pub type C2Rust_Unnamed_1 = ::core::ffi::c_uint;
pub const MATCH_YES: C2Rust_Unnamed_1 = 2;
pub const MATCH_NO: C2Rust_Unnamed_1 = 1;
pub const MATCH_UNKNOWN: C2Rust_Unnamed_1 = 0;
pub type C2Rust_Unnamed_2 = ::core::ffi::c_uint;
pub const RTYPE_DIR: C2Rust_Unnamed_2 = 4;
pub const RTYPE_REDIR: C2Rust_Unnamed_2 = 3;
pub const RTYPE_CGI: C2Rust_Unnamed_2 = 2;
pub const RTYPE_FILE: C2Rust_Unnamed_2 = 1;
pub const RTYPE_NONE: C2Rust_Unnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct httphandle_str {
    pub sock: ::core::ffi::c_int,
    pub error: ::core::ffi::c_int,
    pub inputdata: *mut uint8_t,
    pub inputdataleng: uint32_t,
    pub outputdata: *mut uint8_t,
    pub outputdataleng: uint32_t,
    pub starttime: ::core::ffi::c_double,
    pub requrl: *mut ::core::ffi::c_char,
    pub reqargs: *mut ::core::ffi::c_char,
    pub httpver: uint8_t,
    pub keepalive: uint8_t,
    pub etagmatch: uint8_t,
    pub mtimematch: uint8_t,
    pub method: uint8_t,
    pub status: uint8_t,
    pub req: *mut requests,
}
pub type httphandle = httphandle_str;
pub const DEFAULT_CGIDIR: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b"/usr/local/share/plfscgi\0")
};
pub const DEFAULT_GUI_HTTP_PORT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"9425\0") };
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const __S_IFREG: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const S_IFMT: ::core::ffi::c_int = __S_IFMT;
pub const S_IFREG: ::core::ffi::c_int = __S_IFREG;
pub const STDIN_FILENO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STDERR_FILENO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
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
pub const ENOTSUP: ::core::ffi::c_int = EOPNOTSUPP;
pub const MFSLOG_INFO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_NOTICE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_WARNING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const EOPNOTSUPP: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const POLLERR: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const POLLHUP: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MFSLOG_ERRNO_SYSLOG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG_STDERR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MFSLOG_ERRNO_SYSLOG_STDERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SIGKILL: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
static mut mimes: [mimetype; 18] = [mimetype {
    extl: 0,
    ext: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    mime: ::core::ptr::null_mut::<::core::ffi::c_char>(),
}; 18];
unsafe extern "C" fn mime_find(
    mut fname: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    unsafe {
        let mut el: ::core::ffi::c_int = 0;
        let mut fl: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        fl = strlen(fname) as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        loop {
            el = mimes[i as usize].extl as ::core::ffi::c_int;
            if el <= 0 as ::core::ffi::c_int {
                break;
            }
            if fl > el
                && *fname.offset((fl - el - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    == '.' as ::core::ffi::c_int
                && memcmp(
                    fname.offset(fl as isize).offset(-(el as isize)) as *const ::core::ffi::c_void,
                    mimes[i as usize].ext as *const ::core::ffi::c_void,
                    el as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                return mimes[i as usize].mime;
            }
            i += 1;
        }
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
}
pub const CHLDHASHSIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
static mut chldhash: [*mut chld; 1024] = [::core::ptr::null_mut::<chld>(); 1024];
unsafe extern "C" fn children_add(mut p: pid_t) {
    unsafe {
        let mut h: ::core::ffi::c_int = 0;
        let mut c: *mut chld = ::core::ptr::null_mut::<chld>();
        h = p as ::core::ffi::c_int % CHLDHASHSIZE;
        c = malloc(::core::mem::size_of::<chld>()) as *mut chld;
        (*c).pid = p;
        (*c).next = chldhash[h as usize] as *mut _chld;
        chldhash[h as usize] = c;
    }
}
unsafe extern "C" fn children_remove(mut p: pid_t) {
    unsafe {
        let mut h: ::core::ffi::c_int = 0;
        let mut c: *mut chld = ::core::ptr::null_mut::<chld>();
        let mut cp: *mut *mut chld = ::core::ptr::null_mut::<*mut chld>();
        h = p as ::core::ffi::c_int % CHLDHASHSIZE;
        cp = (&raw mut chldhash as *mut *mut chld).offset(h as isize);
        loop {
            c = *cp;
            if c.is_null() {
                break;
            }
            if (*c).pid == p {
                *cp = (*c).next as *mut chld;
                free(c as *mut ::core::ffi::c_void);
            } else {
                cp = &raw mut (*c).next as *mut *mut chld;
            }
        }
    }
}
unsafe extern "C" fn children_kill() {
    unsafe {
        let mut h: ::core::ffi::c_int = 0;
        let mut c: *mut chld = ::core::ptr::null_mut::<chld>();
        let mut cn: *mut chld = ::core::ptr::null_mut::<chld>();
        h = 0 as ::core::ffi::c_int;
        while h < CHLDHASHSIZE {
            c = chldhash[h as usize];
            while !c.is_null() {
                cn = (*c).next as *mut chld;
                kill((*c).pid as __pid_t, SIGKILL);
                free(c as *mut ::core::ffi::c_void);
                c = cn;
            }
            chldhash[h as usize] = ::core::ptr::null_mut::<chld>();
            h += 1;
        }
    }
}
unsafe extern "C" fn children_init() {
    unsafe {
        let mut h: ::core::ffi::c_int = 0;
        h = 0 as ::core::ffi::c_int;
        while h < CHLDHASHSIZE {
            chldhash[h as usize] = ::core::ptr::null_mut::<chld>();
            h += 1;
        }
    }
}
static mut req_tail: *mut *mut requests = ::core::ptr::null_mut::<*mut requests>();
static mut req_head: *mut requests = ::core::ptr::null_mut::<requests>();
pub const HTTP_HEADERMAX: ::core::ffi::c_int = 16384 as ::core::ffi::c_int;
pub const FWD_BUFFSIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
static mut RootDir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut RootDirLen: uint32_t = 0 as uint32_t;
static mut RequestsFile: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut root_dir_mtime: time_t = 0;
static mut requests_mtime: time_t = 0;
static mut requests_leng: off_t = 0;
static mut Timeout: uint32_t = 0;
static mut ListenHost: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut ListenPort: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut listenip: uint32_t = 0;
static mut listenport: uint16_t = 0;
static mut lsock: ::core::ffi::c_int = 0;
static mut lsockpdescpos: uint32_t = 0;
static mut OsPath: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[inline]
unsafe extern "C" fn mfscgiserv_free_request(mut r: *mut requests) {
    unsafe {
        if !(*r).request.is_null() {
            free((*r).request as *mut ::core::ffi::c_void);
        }
        if !(*r).fname.is_null() {
            free((*r).fname as *mut ::core::ffi::c_void);
        }
        if !(*r).extra.is_null() {
            free((*r).extra as *mut ::core::ffi::c_void);
        }
        if !(*r).mtimestr.is_null() {
            free((*r).mtimestr as *mut ::core::ffi::c_void);
        }
        if !(*r).etag.is_null() {
            free((*r).etag as *mut ::core::ffi::c_void);
        }
        if !(*r).fdata.is_null() {
            free((*r).fdata as *mut ::core::ffi::c_void);
        }
    }
}
#[inline]
unsafe extern "C" fn mfscgiserv_free_requests() {
    unsafe {
        let mut rh: *mut requests = ::core::ptr::null_mut::<requests>();
        let mut nrh: *mut requests = ::core::ptr::null_mut::<requests>();
        rh = req_head;
        while !rh.is_null() {
            nrh = (*rh).next as *mut requests;
            mfscgiserv_free_request(rh);
            free(rh as *mut ::core::ffi::c_void);
            rh = nrh;
        }
        req_head = ::core::ptr::null_mut::<requests>();
        req_tail = &raw mut req_head;
    }
}
unsafe extern "C" fn mfscgiserv_generate_cache_strings(mut r: *mut requests) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        static mut structtime: *mut tm = ::core::ptr::null_mut::<tm>();
        let mut tts: time_t = 0;
        let mut ctx: md5ctx = md5ctx {
            state: [0; 4],
            count: [0; 2],
            buffer: [0; 64],
        };
        let mut digest: [uint8_t; 16] = [0; 16];
        if !(*r).mtimestr.is_null() {
            free((*r).mtimestr as *mut ::core::ffi::c_void);
            (*r).mtimestr = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if !(*r).etag.is_null() {
            free((*r).etag as *mut ::core::ffi::c_void);
            (*r).etag = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        tts = (*r).mtime;
        structtime = gmtime(&raw mut tts);
        (*r).mtimestr = malloc(31 as size_t) as *mut ::core::ffi::c_char;
        strftime(
            (*r).mtimestr,
            30 as size_t,
            b"%a, %d %b %Y %T GMT\0".as_ptr() as *const ::core::ffi::c_char,
            structtime,
        );
        *(*r).mtimestr.offset(30 as isize) = 0 as ::core::ffi::c_char;
        md5_init(&raw mut ctx);
        md5_update(&raw mut ctx, (*r).fdata, (*r).fsize);
        md5_final(&raw mut digest as *mut uint8_t, &raw mut ctx);
        (*r).etag = malloc(33 as size_t) as *mut ::core::ffi::c_char;
        if (*r).etag.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                187 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"r->etag\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                187 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"r->etag\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if (*r).etag
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut ::core::ffi::c_char
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                187 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"r->etag\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                187 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"r->etag\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        i = 0 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int {
            *(*r).etag.offset((i * 2 as ::core::ffi::c_int) as isize) =
                ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(
                    *b"0123456789ABCDEF\0",
                )[(digest[i as usize] as ::core::ffi::c_int
                    >> 4 as ::core::ffi::c_int) as usize];
            *(*r)
                .etag
                .offset((i * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(
                    *b"0123456789ABCDEF\0",
                )[(digest[i as usize] as ::core::ffi::c_int
                    & 15 as ::core::ffi::c_int) as usize];
            i += 1;
        }
        *(*r).etag.offset(32 as isize) = 0 as ::core::ffi::c_char;
    }
}
unsafe extern "C" fn mfscgiserv_reloaddata(
    mut r: *mut requests,
    mut mtime: time_t,
    mut fsize: off_t,
) {
    unsafe {
        let mut fd: ::core::ffi::c_int = 0;
        let mut fdata: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        fd = open((*r).fname, O_RDONLY);
        if fd < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"guiserv: can't reload content of the file: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*r).fname,
            );
            return;
        }
        fdata = malloc(fsize as size_t) as *mut uint8_t;
        if fdata.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                207 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"fdata\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                207 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"fdata\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if fdata
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut uint8_t
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                207 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"fdata\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                207 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"fdata\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        if read(fd, fdata as *mut ::core::ffi::c_void, fsize as size_t) != fsize as ssize_t {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"guiserv: can't reload content of the file: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*r).fname,
            );
            free(fdata as *mut ::core::ffi::c_void);
            close(fd);
            return;
        }
        close(fd);
        if !(*r).fdata.is_null() {
            free((*r).fdata as *mut ::core::ffi::c_void);
        }
        (*r).fdata = fdata;
        (*r).mtime = mtime;
        (*r).fsize = fsize as uint32_t;
        mfscgiserv_generate_cache_strings(r);
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_NOTICE,
            b"guiserv: file %s has been reloaded\0".as_ptr() as *const ::core::ffi::c_char,
            (*r).fname,
        );
    }
}
unsafe extern "C" fn mfscgiserv_loaddata(mut r: *mut requests) -> ::core::ffi::c_int {
    unsafe {
        let mut fd: ::core::ffi::c_int = 0;
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
        fd = open((*r).fname, O_RDONLY);
        if fd < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"guiserv: can't read content of file: %s\0".as_ptr() as *const ::core::ffi::c_char,
                (*r).fname,
            );
            return -1 as ::core::ffi::c_int;
        }
        if fstat(fd, &raw mut st) < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"guiserv: can't stat file: %s\0".as_ptr() as *const ::core::ffi::c_char,
                (*r).fname,
            );
            close(fd);
            return -1 as ::core::ffi::c_int;
        }
        (*r).mtime = st.st_mtim.tv_sec as time_t;
        (*r).fsize = st.st_size as uint32_t;
        (*r).fdata = malloc((*r).fsize as size_t) as *mut uint8_t;
        if (*r).fdata.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                244 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"r->fdata\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                244 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"r->fdata\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if (*r).fdata
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut uint8_t
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                244 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"r->fdata\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                244 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"r->fdata\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        if read(
            fd,
            (*r).fdata as *mut ::core::ffi::c_void,
            (*r).fsize as size_t,
        ) != (*r).fsize as ssize_t
        {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"guiserv: can't read content of the file: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*r).fname,
            );
            close(fd);
            return -1 as ::core::ffi::c_int;
        }
        close(fd);
        mfscgiserv_generate_cache_strings(r);
        return 0 as ::core::ffi::c_int;
    }
}
#[inline]
unsafe extern "C" fn mfscgiserv_isspace(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    return (c as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
        || c as ::core::ffi::c_int == '\t' as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn mfscgiserv_isnotspace(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    return (c as ::core::ffi::c_int != 0
        && c as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
        && c as ::core::ffi::c_int != '\t' as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_parse_cfgline(
    mut line: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut req: *mut requests = ::core::ptr::null_mut::<requests>();
        let mut rptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut rl: ::core::ffi::c_int = 0;
        let mut request: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut fl: ::core::ffi::c_int = 0;
        let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut el: ::core::ffi::c_int = 0;
        let mut extra: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut rtype: uint8_t = 0;
        match *line.offset(0 as isize) as ::core::ffi::c_int {
            102 | 70 => {
                rtype = RTYPE_FILE as ::core::ffi::c_int as uint8_t;
            }
            99 | 67 => {
                rtype = RTYPE_CGI as ::core::ffi::c_int as uint8_t;
            }
            114 | 82 => {
                rtype = RTYPE_REDIR as ::core::ffi::c_int as uint8_t;
            }
            100 | 68 => {
                rtype = RTYPE_DIR as ::core::ffi::c_int as uint8_t;
            }
            35 | 59 | 47 => return 0 as ::core::ffi::c_int,
            _ => return -1 as ::core::ffi::c_int,
        }
        p = line.offset(1 as ::core::ffi::c_int as isize);
        if mfscgiserv_isnotspace(*p) != 0 {
            return -1 as ::core::ffi::c_int;
        }
        while mfscgiserv_isspace(*p) != 0 {
            p = p.offset(1);
        }
        while *p as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
            p = p.offset(1);
        }
        if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        rptr = p;
        rl = 0 as ::core::ffi::c_int;
        while mfscgiserv_isnotspace(*p) != 0 {
            p = p.offset(1);
            rl += 1;
        }
        request = malloc((rl + 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
        if request.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                344 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"request\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                344 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"request\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if request
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut ::core::ffi::c_char
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                344 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"request\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                344 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"request\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        memcpy(
            request as *mut ::core::ffi::c_void,
            rptr as *const ::core::ffi::c_void,
            rl as size_t,
        );
        *request.offset(rl as isize) = 0 as ::core::ffi::c_char;
        while mfscgiserv_isspace(*p) != 0 {
            p = p.offset(1);
        }
        if rtype as ::core::ffi::c_int != RTYPE_REDIR as ::core::ffi::c_int {
            if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                fl = rl;
            } else {
                rptr = p;
                fl = 0 as ::core::ffi::c_int;
                while mfscgiserv_isnotspace(*p) != 0 {
                    p = p.offset(1);
                    fl += 1;
                }
            }
            filename = malloc(
                RootDirLen
                    .wrapping_add(fl as uint32_t)
                    .wrapping_add(2 as uint32_t) as size_t,
            ) as *mut ::core::ffi::c_char;
            if filename.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                    365 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"filename\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                    365 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"filename\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if filename
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut ::core::ffi::c_char
            {
                let mut _mfs_errorstring_0: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                    365 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"filename\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                    365 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"filename\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_0,
                );
                abort();
            }
            memcpy(
                filename as *mut ::core::ffi::c_void,
                RootDir as *const ::core::ffi::c_void,
                RootDirLen as size_t,
            );
            *filename.offset(RootDirLen as isize) = '/' as ::core::ffi::c_char;
            memcpy(
                filename
                    .offset(RootDirLen as isize)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_void,
                rptr as *const ::core::ffi::c_void,
                fl as size_t,
            );
            *filename.offset(
                RootDirLen
                    .wrapping_add(fl as uint32_t)
                    .wrapping_add(1 as uint32_t) as isize,
            ) = 0 as ::core::ffi::c_char;
            fl = (fl as uint32_t).wrapping_add(RootDirLen.wrapping_add(1 as uint32_t))
                as ::core::ffi::c_int;
        } else {
            if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                free(request as *mut ::core::ffi::c_void);
                return -1 as ::core::ffi::c_int;
            }
            while *p as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
                p = p.offset(1);
            }
            rptr = p;
            fl = 0 as ::core::ffi::c_int;
            while mfscgiserv_isnotspace(*p) != 0 {
                p = p.offset(1);
                fl += 1;
            }
            filename = malloc((fl + 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
            if filename.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                    388 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"filename\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                    388 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"filename\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if filename
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut ::core::ffi::c_char
            {
                let mut _mfs_errorstring_1: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                    388 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"filename\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_1,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                    388 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"filename\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_1,
                );
                abort();
            }
            memcpy(
                filename as *mut ::core::ffi::c_void,
                rptr as *const ::core::ffi::c_void,
                fl as size_t,
            );
            *filename.offset(fl as isize) = 0 as ::core::ffi::c_char;
        }
        while mfscgiserv_isspace(*p) != 0 {
            p = p.offset(1);
        }
        if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            extra = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            rptr = p;
            el = 0 as ::core::ffi::c_int;
            while mfscgiserv_isnotspace(*p) != 0 {
                p = p.offset(1);
                el += 1;
            }
            extra = malloc((el + 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
            if extra.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                    408 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"extra\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                    408 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"extra\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if extra
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut ::core::ffi::c_char
            {
                let mut _mfs_errorstring_2: *const ::core::ffi::c_char =
                    strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                    408 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"extra\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_2,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                    408 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"extra\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring_2,
                );
                abort();
            }
            memcpy(
                extra as *mut ::core::ffi::c_void,
                rptr as *const ::core::ffi::c_void,
                el as size_t,
            );
            *extra.offset(el as isize) = 0 as ::core::ffi::c_char;
        }
        req = malloc(::core::mem::size_of::<requests>()) as *mut requests;
        (*req).rtype = rtype;
        (*req).rl = rl as uint32_t;
        (*req).request = request;
        (*req).fl = fl as uint32_t;
        (*req).fname = filename;
        (*req).extra = extra;
        (*req).mtime = 0 as time_t;
        (*req).mimetype = ::core::ptr::null::<::core::ffi::c_char>();
        (*req).mtimestr = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*req).etag = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*req).fdata = ::core::ptr::null_mut::<uint8_t>();
        if rtype as ::core::ffi::c_int == RTYPE_FILE as ::core::ffi::c_int {
            if mfscgiserv_loaddata(req) < 0 as ::core::ffi::c_int {
                mfscgiserv_free_request(req);
                return -1 as ::core::ffi::c_int;
            }
            if (*req).extra.is_null() {
                (*req).mimetype = mime_find((*req).fname);
                if (*req).mimetype.is_null() {
                    (*req).mimetype = mime_find((*req).request);
                }
            }
        } else if rtype as ::core::ffi::c_int == RTYPE_CGI as ::core::ffi::c_int {
            (*req).fdata = malloc(FWD_BUFFSIZE as size_t) as *mut uint8_t;
            (*req).fsize = 0 as uint32_t;
        }
        (*req).next = ::core::ptr::null_mut::<requests_str>();
        *req_tail = req;
        req_tail = &raw mut (*req).next as *mut *mut requests;
        return 0 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_rescan() {
    unsafe {
        let mut fd: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut linebuff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut lbsize: size_t = 0;
        let mut s: uint32_t = 0;
        fd = fopen(RequestsFile, b"r\0".as_ptr() as *const ::core::ffi::c_char) as *mut FILE;
        if fd.is_null() {
            if *__errno_location() == ENOENT {
                if !req_head.is_null() {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"guiserv: requests file (%s) not found - requests not changed\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        RequestsFile,
                    );
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG_STDERR,
                        MFSLOG_WARNING,
                        b"guiserv: requests file (%s) not found !!!\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        RequestsFile,
                    );
                }
            } else if !req_head.is_null() {
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG,
                    MFSLOG_WARNING,
                    b"guiserv: can't open mfsgui requests file (%s) - requests not changed, error\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    RequestsFile,
                );
            } else {
                mfs_log(
                    MFSLOG_ERRNO_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"guiserv: can't open mfsgui requests file (%s) - no requests !!!, error\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    RequestsFile,
                );
            }
            return;
        }
        mfscgiserv_free_requests();
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
                if mfscgiserv_parse_cfgline(linebuff) < 0 as ::core::ffi::c_int {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"guiserv: wrong request definition: %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        linebuff,
                    );
                }
            }
        }
        fclose(fd);
        free(linebuff as *mut ::core::ffi::c_void);
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_NOTICE,
            b"guiserv: requests have been reloaded\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_quick_rescan() {
    unsafe {
        let mut rdst: stat = stat {
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
        let mut rqst: stat = stat {
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
        let mut fst: stat = stat {
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
        let mut r: *mut requests = ::core::ptr::null_mut::<requests>();
        if stat(RootDir, &raw mut rdst) >= 0 as ::core::ffi::c_int
            && stat(RequestsFile, &raw mut rqst) >= 0 as ::core::ffi::c_int
        {
            if rdst.st_mtim.tv_sec != root_dir_mtime
                || rqst.st_mtim.tv_sec != requests_mtime
                || rqst.st_size != requests_leng
            {
                root_dir_mtime = rdst.st_mtim.tv_sec as time_t;
                requests_mtime = rqst.st_mtim.tv_sec as time_t;
                requests_leng = rqst.st_size as off_t;
                mfscgiserv_rescan();
                return;
            }
        }
        r = req_head;
        while !r.is_null() {
            if (*r).rtype as ::core::ffi::c_int == RTYPE_FILE as ::core::ffi::c_int {
                if stat((*r).fname, &raw mut fst) >= 0 as ::core::ffi::c_int {
                    if (*r).mtime != fst.st_mtim.tv_sec || (*r).fsize as __off_t != fst.st_size {
                        mfscgiserv_reloaddata(
                            r,
                            fst.st_mtim.tv_sec as time_t,
                            fst.st_size as off_t,
                        );
                    }
                } else {
                    mfs_log(
                        MFSLOG_SYSLOG,
                        MFSLOG_WARNING,
                        b"guiserv: can't stat file: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        (*r).fname,
                    );
                }
            }
            r = (*r).next as *mut requests;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_expect_str(
    mut sptr: *const ::core::ffi::c_char,
    mut expect: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c1: ::core::ffi::c_char = 0;
        let mut c2: ::core::ffi::c_char = 0;
        let mut l: ::core::ffi::c_int = 0;
        l = 0 as ::core::ffi::c_int;
        while *expect != 0 {
            c1 = *sptr;
            c2 = *expect;
            if c1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
            if c1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                && c1 as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
            {
                c1 = (c1 as ::core::ffi::c_int
                    + ('a' as ::core::ffi::c_int - 'A' as ::core::ffi::c_int))
                    as ::core::ffi::c_char;
            }
            if c2 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                && c2 as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
            {
                c2 = (c2 as ::core::ffi::c_int
                    + ('a' as ::core::ffi::c_int - 'A' as ::core::ffi::c_int))
                    as ::core::ffi::c_char;
            }
            if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
            sptr = sptr.offset(1);
            expect = expect.offset(1);
            l += 1;
        }
        return l;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_check_header(
    mut sptr: *const ::core::ffi::c_char,
    mut header: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c1: ::core::ffi::c_char = 0;
        let mut c2: ::core::ffi::c_char = 0;
        let mut l: ::core::ffi::c_int = 0;
        l = 0 as ::core::ffi::c_int;
        while *header != 0 {
            c1 = *sptr;
            c2 = *header;
            if c1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
            if c1 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                && c1 as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
            {
                c1 = (c1 as ::core::ffi::c_int
                    + ('a' as ::core::ffi::c_int - 'A' as ::core::ffi::c_int))
                    as ::core::ffi::c_char;
            }
            if c2 as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                && c2 as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
            {
                c2 = (c2 as ::core::ffi::c_int
                    + ('a' as ::core::ffi::c_int - 'A' as ::core::ffi::c_int))
                    as ::core::ffi::c_char;
            }
            if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
            sptr = sptr.offset(1);
            header = header.offset(1);
            l += 1;
        }
        if *sptr as ::core::ffi::c_int != ':' as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        return l + 1 as ::core::ffi::c_int;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_parse_hdrline(
    mut h: *mut httphandle,
    mut lineno: uint32_t,
    mut sptr: *mut ::core::ffi::c_char,
) {
    unsafe {
        let mut req: *mut requests = ::core::ptr::null_mut::<requests>();
        let mut ul: uint32_t = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut hl: ::core::ffi::c_int = 0;
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if lineno == 0 as uint32_t {
            if (*h).method as ::core::ffi::c_int == METHOD_GET as ::core::ffi::c_int {
                p = sptr.offset(4 as ::core::ffi::c_int as isize);
            } else if (*h).method as ::core::ffi::c_int == METHOD_HEAD as ::core::ffi::c_int {
                p = sptr.offset(5 as ::core::ffi::c_int as isize);
            } else {
                return;
            }
            if *p as ::core::ffi::c_int != '/' as ::core::ffi::c_int {
                (*h).status = STATUS_BADREQUEST as ::core::ffi::c_int as uint8_t;
                return;
            }
            p = p.offset(1);
            (*h).requrl = p;
            ul = 0 as uint32_t;
            while *p as ::core::ffi::c_int != 0
                && *p as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
                && *p as ::core::ffi::c_int != '?' as ::core::ffi::c_int
                && *p as ::core::ffi::c_int != '&' as ::core::ffi::c_int
            {
                p = p.offset(1);
                ul = ul.wrapping_add(1);
            }
            if *p as ::core::ffi::c_int == '?' as ::core::ffi::c_int
                || *p as ::core::ffi::c_int == '&' as ::core::ffi::c_int
            {
                *p = 0 as ::core::ffi::c_char;
                p = p.offset(1);
                (*h).reqargs = p;
                while *p as ::core::ffi::c_int != 0
                    && *p as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
                {
                    p = p.offset(1);
                }
                *p = 0 as ::core::ffi::c_char;
                p = p.offset(1);
            } else if *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int {
                *p = 0 as ::core::ffi::c_char;
                p = p.offset(1);
            } else {
                (*h).status = STATUS_BADREQUEST as ::core::ffi::c_int as uint8_t;
                return;
            }
            if strcmp(p, b"HTTP/1.0\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                (*h).httpver = 0x10 as uint8_t;
                (*h).keepalive = 0 as uint8_t;
            } else if strcmp(p, b"HTTP/1.1\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                (*h).httpver = 0x11 as uint8_t;
                (*h).keepalive = 1 as uint8_t;
            }
            req = req_head;
            while !req.is_null() {
                if (*req).rtype as ::core::ffi::c_int == RTYPE_DIR as ::core::ffi::c_int
                    && (*req).rl < ul
                    && memcmp(
                        (*req).request as *const ::core::ffi::c_void,
                        (*h).requrl as *const ::core::ffi::c_void,
                        (*req).rl as size_t,
                    ) == 0 as ::core::ffi::c_int
                    && *(*h).requrl.offset((*req).rl as isize) as ::core::ffi::c_int
                        == '/' as ::core::ffi::c_int
                {
                    (*h).req = req;
                    break;
                } else if (*req).rl == ul
                    && memcmp(
                        (*req).request as *const ::core::ffi::c_void,
                        (*h).requrl as *const ::core::ffi::c_void,
                        ul as size_t,
                    ) == 0 as ::core::ffi::c_int
                {
                    (*h).req = req;
                    break;
                } else {
                    req = (*req).next as *mut requests;
                }
            }
        } else {
            if !(*h).req.is_null()
                && (*(*h).req).rtype as ::core::ffi::c_int == RTYPE_FILE as ::core::ffi::c_int
            {
                if !(*(*h).req).etag.is_null() {
                    hl = mfscgiserv_check_header(
                        sptr,
                        b"If-None-Match\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if hl > 0 as ::core::ffi::c_int {
                        p = sptr.offset(hl as isize);
                        while *p as ::core::ffi::c_int != 0
                            && *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                        {
                            p = p.offset(1);
                        }
                        if *p as ::core::ffi::c_int == '"' as ::core::ffi::c_int {
                            p = p.offset(1);
                            i = 0 as ::core::ffi::c_int;
                            while (*h).etagmatch as ::core::ffi::c_int
                                == MATCH_UNKNOWN as ::core::ffi::c_int
                                && i < 32 as ::core::ffi::c_int
                            {
                                if *p.offset(i as isize) as ::core::ffi::c_int
                                    != *(*(*h).req).etag.offset(i as isize) as ::core::ffi::c_int
                                {
                                    (*h).etagmatch = MATCH_NO as ::core::ffi::c_int as uint8_t;
                                }
                                i += 1;
                            }
                            if (*h).etagmatch as ::core::ffi::c_int
                                == MATCH_UNKNOWN as ::core::ffi::c_int
                                && *p.offset(32 as isize) as ::core::ffi::c_int
                                    == '"' as ::core::ffi::c_int
                            {
                                (*h).etagmatch = MATCH_YES as ::core::ffi::c_int as uint8_t;
                            }
                        } else {
                            (*h).etagmatch = MATCH_NO as ::core::ffi::c_int as uint8_t;
                        }
                    }
                }
                if !(*(*h).req).mtimestr.is_null() {
                    hl = mfscgiserv_check_header(
                        sptr,
                        b"If-Modified-Since\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if hl > 0 as ::core::ffi::c_int {
                        p = sptr.offset(hl as isize);
                        while *p as ::core::ffi::c_int != 0
                            && *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                        {
                            p = p.offset(1);
                        }
                        i = 0 as ::core::ffi::c_int;
                        while (*h).mtimematch as ::core::ffi::c_int
                            == MATCH_UNKNOWN as ::core::ffi::c_int
                            && i < 30 as ::core::ffi::c_int
                        {
                            if *p.offset(i as isize) as ::core::ffi::c_int
                                != *(*(*h).req).mtimestr.offset(i as isize) as ::core::ffi::c_int
                            {
                                (*h).mtimematch = MATCH_NO as ::core::ffi::c_int as uint8_t;
                            }
                            i += 1;
                        }
                        if (*h).mtimematch as ::core::ffi::c_int
                            == MATCH_UNKNOWN as ::core::ffi::c_int
                        {
                            (*h).mtimematch = MATCH_YES as ::core::ffi::c_int as uint8_t;
                        }
                    }
                }
            }
            hl = mfscgiserv_check_header(
                sptr,
                b"Connection\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if hl > 0 as ::core::ffi::c_int {
                p = sptr.offset(hl as isize);
                while *p as ::core::ffi::c_int != 0
                    && *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                {
                    p = p.offset(1);
                }
                if mfscgiserv_expect_str(p, b"close\0".as_ptr() as *const ::core::ffi::c_char)
                    >= 0 as ::core::ffi::c_int
                {
                    (*h).keepalive = 0 as uint8_t;
                }
                if mfscgiserv_expect_str(p, b"keep-alive\0".as_ptr() as *const ::core::ffi::c_char)
                    >= 0 as ::core::ffi::c_int
                {
                    (*h).keepalive = 1 as uint8_t;
                }
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_parse_headers(mut h: *mut httphandle) {
    unsafe {
        let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut found: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut i: uint32_t = 0;
        i = 0 as uint32_t;
        ptr = (*h).inputdata as *mut ::core::ffi::c_char;
        loop {
            found = strstr(ptr, b"\r\n\0".as_ptr() as *const ::core::ffi::c_char);
            if found.is_null() {
                break;
            }
            *found.offset(0 as isize) = 0 as ::core::ffi::c_char;
            mfscgiserv_parse_hdrline(h, i, ptr);
            ptr = found.offset(2 as ::core::ffi::c_int as isize);
            i = i.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_printf(
    mut h: *mut httphandle,
    mut format: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        let mut maxstrleng: int32_t = 0;
        let mut leng: int32_t = 0;
        let mut ap: ::core::ffi::VaList;
        if (*h).outputdataleng.wrapping_add(2 as uint32_t) > HTTP_HEADERMAX as uint32_t {
            (*h).error = 1 as ::core::ffi::c_int;
            return;
        }
        maxstrleng = (HTTP_HEADERMAX as uint32_t)
            .wrapping_sub((*h).outputdataleng.wrapping_add(2 as uint32_t))
            as int32_t;
        ap = c2rust_args.clone();
        leng = vsnprintf(
            ((*h).outputdata as *mut ::core::ffi::c_char).offset((*h).outputdataleng as isize),
            maxstrleng as size_t,
            format,
            ap.clone(),
        ) as int32_t;
        if (*h)
            .outputdataleng
            .wrapping_add(leng as uint32_t)
            .wrapping_add(2 as uint32_t)
            > HTTP_HEADERMAX as uint32_t
        {
            (*h).error = 1 as ::core::ffi::c_int;
            return;
        }
        (*h).outputdataleng = (*h).outputdataleng.wrapping_add(leng as uint32_t);
        let c2rust_fresh0 = (*h).outputdataleng;
        (*h).outputdataleng = (*h).outputdataleng.wrapping_add(1);
        *(*h).outputdata.offset(c2rust_fresh0 as isize) = '\r' as uint8_t;
        let c2rust_fresh1 = (*h).outputdataleng;
        (*h).outputdataleng = (*h).outputdataleng.wrapping_add(1);
        *(*h).outputdata.offset(c2rust_fresh1 as isize) = '\n' as uint8_t;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_read_request(mut h: *mut httphandle) {
    unsafe {
        let mut pfd: pollfd = pollfd {
            fd: 0,
            events: 0,
            revents: 0,
        };
        let mut i: ::core::ffi::c_int = 0;
        let mut searchpos: ::core::ffi::c_int = 0;
        let mut ts: ::core::ffi::c_double = 0.;
        let mut found: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        pfd.fd = (*h).sock;
        pfd.events = POLLIN as ::core::ffi::c_short;
        pfd.revents = 0 as ::core::ffi::c_short;
        loop {
            i = read(
                (*h).sock,
                (*h).inputdata.offset((*h).inputdataleng as isize) as *mut ::core::ffi::c_void,
                (HTTP_HEADERMAX as uint32_t).wrapping_sub((*h).inputdataleng) as size_t,
            ) as ::core::ffi::c_int;
            if i == 0 as ::core::ffi::c_int {
                (*h).error = 1 as ::core::ffi::c_int;
                return;
            }
            if i < 0 as ::core::ffi::c_int
                && (*__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK)
            {
                (*h).error = 1 as ::core::ffi::c_int;
                return;
            }
            if pfd.revents as ::core::ffi::c_int & POLLHUP != 0 {
                (*h).error = 1 as ::core::ffi::c_int;
                return;
            }
            if (*h).inputdataleng > 3 as uint32_t {
                searchpos = (*h).inputdataleng.wrapping_sub(3 as uint32_t) as ::core::ffi::c_int;
            } else {
                searchpos = 0 as ::core::ffi::c_int;
            }
            if i > 0 as ::core::ffi::c_int {
                (*h).inputdataleng = (*h).inputdataleng.wrapping_add(i as uint32_t);
                if (*h).inputdataleng >= HTTP_HEADERMAX as uint32_t {
                    (*h).error = 1 as ::core::ffi::c_int;
                    return;
                }
                *(*h).inputdata.offset((*h).inputdataleng as isize) = 0 as uint8_t;
                if (*h).inputdataleng >= 8 as uint32_t
                    && (*h).method as ::core::ffi::c_int == METHOD_NONE as ::core::ffi::c_int
                {
                    if memcmp(
                        (*h).inputdata as *const ::core::ffi::c_void,
                        b"GET \0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        4 as size_t,
                    ) == 0 as ::core::ffi::c_int
                    {
                        (*h).method = METHOD_GET as ::core::ffi::c_int as uint8_t;
                    } else if memcmp(
                        (*h).inputdata as *const ::core::ffi::c_void,
                        b"HEAD \0".as_ptr() as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        5 as size_t,
                    ) == 0 as ::core::ffi::c_int
                    {
                        (*h).method = METHOD_HEAD as ::core::ffi::c_int as uint8_t;
                    } else {
                        (*h).status = STATUS_BADMETHOD as ::core::ffi::c_int as uint8_t;
                        return;
                    }
                }
                found = strstr(
                    ((*h).inputdata as *mut ::core::ffi::c_char).offset(searchpos as isize),
                    b"\r\n\r\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if !found.is_null() {
                    *found.offset(2 as isize) = 0 as ::core::ffi::c_char;
                    return;
                }
            }
            ts = monotonic_seconds();
            if ts - (*h).starttime >= Timeout as ::core::ffi::c_double {
                (*h).error = 1 as ::core::ffi::c_int;
                return;
            }
            pfd.revents = 0 as ::core::ffi::c_short;
            if poll(
                &raw mut pfd,
                1 as nfds_t,
                (((*h).starttime + Timeout as ::core::ffi::c_double - ts)
                    * 1000 as ::core::ffi::c_int as ::core::ffi::c_double)
                    as ::core::ffi::c_int,
            ) < 0 as ::core::ffi::c_int
            {
                if *__errno_location() != EINTR {
                    (*h).error = 1 as ::core::ffi::c_int;
                    return;
                }
            } else {
                if pfd.revents as ::core::ffi::c_int & POLLERR != 0 {
                    (*h).error = 1 as ::core::ffi::c_int;
                    return;
                }
                if pfd.revents as ::core::ffi::c_int & POLLIN == 0 as ::core::ffi::c_int {
                    (*h).error = 1 as ::core::ffi::c_int;
                    return;
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_write_data(mut h: *mut httphandle, mut skip: uint32_t) {
    unsafe {
        let mut ts: ::core::ffi::c_double = 0.;
        let mut msecto: uint32_t = 0;
        ts = monotonic_seconds();
        if ts - (*h).starttime >= Timeout as ::core::ffi::c_double {
            (*h).error = 1 as ::core::ffi::c_int;
            return;
        }
        msecto = (((*h).starttime + Timeout as ::core::ffi::c_double - ts)
            * 1000 as ::core::ffi::c_int as ::core::ffi::c_double) as uint32_t;
        if tcptowrite(
            (*h).sock,
            (*(*h).req).fdata.offset(skip as isize) as *const ::core::ffi::c_void,
            (*(*h).req).fsize.wrapping_sub(skip),
            msecto,
            msecto,
        ) != (*(*h).req).fsize.wrapping_sub(skip) as int32_t
        {
            (*h).error = 1 as ::core::ffi::c_int;
            return;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_write_headers(mut h: *mut httphandle) {
    unsafe {
        let mut ts: ::core::ffi::c_double = 0.;
        let mut msecto: uint32_t = 0;
        ts = monotonic_seconds();
        if ts - (*h).starttime >= Timeout as ::core::ffi::c_double {
            (*h).error = 1 as ::core::ffi::c_int;
            return;
        }
        msecto = (((*h).starttime + Timeout as ::core::ffi::c_double - ts)
            * 1000 as ::core::ffi::c_int as ::core::ffi::c_double) as uint32_t;
        if tcptowrite(
            (*h).sock,
            (*h).outputdata as *const ::core::ffi::c_void,
            (*h).outputdataleng,
            msecto,
            msecto,
        ) != (*h).outputdataleng as int32_t
        {
            (*h).error = 1 as ::core::ffi::c_int;
            return;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_status(mut h: *mut httphandle) {
    unsafe {
        match (*h).status as ::core::ffi::c_int {
            0 => {
                mfscgiserv_printf(
                    h,
                    b"HTTP/1.1 200 OK\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return;
            }
            1 => {
                mfscgiserv_printf(
                    h,
                    b"HTTP/1.1 302 Found\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return;
            }
            4 => {
                mfscgiserv_printf(
                    h,
                    b"HTTP/1.1 304 Not Modified\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return;
            }
            5 => {
                mfscgiserv_printf(
                    h,
                    b"HTTP/1.1 400 Bad Request\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return;
            }
            6 => {
                mfscgiserv_printf(
                    h,
                    b"HTTP/1.1 405 Method Not Allowed\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return;
            }
            2 => {
                mfscgiserv_printf(
                    h,
                    b"HTTP/1.1 404 Not Found\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return;
            }
            3 => {
                mfscgiserv_printf(
                    h,
                    b"HTTP/1.1 403 Forbidden\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return;
            }
            _ => {
                mfscgiserv_printf(
                    h,
                    b"HTTP/1.1 500 Internal Server Error\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return;
            }
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_prepare_headers(mut h: *mut httphandle) {
    unsafe {
        if (*h).req.is_null() {
            (*h).status = STATUS_NOTFOUND as ::core::ffi::c_int as uint8_t;
        }
        if (*h).status as ::core::ffi::c_int == STATUS_NONE as ::core::ffi::c_int {
            if (*h).etagmatch as ::core::ffi::c_int == MATCH_YES as ::core::ffi::c_int {
                (*h).status = STATUS_NOTMODIFIED as ::core::ffi::c_int as uint8_t;
            } else if (*h).etagmatch as ::core::ffi::c_int == MATCH_UNKNOWN as ::core::ffi::c_int
                && (*h).mtimematch as ::core::ffi::c_int == MATCH_YES as ::core::ffi::c_int
            {
                (*h).status = STATUS_NOTMODIFIED as ::core::ffi::c_int as uint8_t;
            }
        }
        if (*h).status as ::core::ffi::c_int != STATUS_NONE as ::core::ffi::c_int {
            (*h).keepalive = 0 as uint8_t;
        }
        mfscgiserv_status(h);
        mfscgiserv_printf(
            h,
            b"Server: mfsgui\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfscgiserv_printf(
            h,
            b"Connection: %s\0".as_ptr() as *const ::core::ffi::c_char,
            if (*h).keepalive as ::core::ffi::c_int != 0 {
                b"keep-alive\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"close\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        if !(*h).req.is_null() {
            if (*h).status as ::core::ffi::c_int == STATUS_NONE as ::core::ffi::c_int {
                if !(*(*h).req).extra.is_null() {
                    mfscgiserv_printf(
                        h,
                        b"Content-Type: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        (*(*h).req).extra,
                    );
                } else if !(*(*h).req).mimetype.is_null() {
                    mfscgiserv_printf(
                        h,
                        b"Content-Type: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        (*(*h).req).mimetype,
                    );
                } else {
                    mfscgiserv_printf(
                        h,
                        b"Content-Type: text/plain\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                mfscgiserv_printf(
                    h,
                    b"Content-Length: %u\0".as_ptr() as *const ::core::ffi::c_char,
                    (*(*h).req).fsize,
                );
            }
            if (*h).status as ::core::ffi::c_int == STATUS_FOUND as ::core::ffi::c_int {
                if !(*h).reqargs.is_null() {
                    if !(*(*h).req).extra.is_null() {
                        mfscgiserv_printf(
                            h,
                            b"Location: /%s?%s&%s\0".as_ptr() as *const ::core::ffi::c_char,
                            (*(*h).req).fname,
                            (*h).reqargs,
                            (*(*h).req).extra,
                        );
                    } else {
                        mfscgiserv_printf(
                            h,
                            b"Location: /%s?%s\0".as_ptr() as *const ::core::ffi::c_char,
                            (*(*h).req).fname,
                            (*h).reqargs,
                        );
                    }
                } else if !(*(*h).req).extra.is_null() {
                    mfscgiserv_printf(
                        h,
                        b"Location: /%s?%s\0".as_ptr() as *const ::core::ffi::c_char,
                        (*(*h).req).fname,
                        (*(*h).req).extra,
                    );
                } else {
                    mfscgiserv_printf(
                        h,
                        b"Location: /%s\0".as_ptr() as *const ::core::ffi::c_char,
                        (*(*h).req).fname,
                    );
                }
            } else {
                mfscgiserv_printf(
                    h,
                    b"Cache-Control: public,max-age=0\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if !(*(*h).req).etag.is_null() {
                    mfscgiserv_printf(
                        h,
                        b"ETag: \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
                        (*(*h).req).etag,
                    );
                }
                if !(*(*h).req).mtimestr.is_null() {
                    mfscgiserv_printf(
                        h,
                        b"Last-Modified: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        (*(*h).req).mtimestr,
                    );
                }
            }
        }
        mfscgiserv_printf(h, b"\0".as_ptr() as *const ::core::ffi::c_char);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_piperead(
    mut h: *mut httphandle,
    mut fd: ::core::ffi::c_int,
    mut maxleng: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut pfd: pollfd = pollfd {
            fd: 0,
            events: 0,
            revents: 0,
        };
        let mut i: ::core::ffi::c_int = 0;
        let mut ts: ::core::ffi::c_double = 0.;
        pfd.fd = fd;
        pfd.events = POLLIN as ::core::ffi::c_short;
        pfd.revents = 0 as ::core::ffi::c_short;
        (*(*h).req).fsize = 0 as uint32_t;
        loop {
            i = read(
                fd,
                (*(*h).req).fdata.offset((*(*h).req).fsize as isize) as *mut ::core::ffi::c_void,
                maxleng.wrapping_sub((*(*h).req).fsize) as size_t,
            ) as ::core::ffi::c_int;
            if i == 0 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
            if i < 0 as ::core::ffi::c_int
                && (*__errno_location() != EAGAIN && *__errno_location() != EWOULDBLOCK)
            {
                (*h).error = 1 as ::core::ffi::c_int;
                return 0 as ::core::ffi::c_int;
            }
            if i > 0 as ::core::ffi::c_int {
                (*(*h).req).fsize = (*(*h).req).fsize.wrapping_add(i as uint32_t);
                if (*(*h).req).fsize >= maxleng {
                    return 1 as ::core::ffi::c_int;
                }
            }
            ts = monotonic_seconds();
            if ts - (*h).starttime >= Timeout as ::core::ffi::c_double {
                (*h).error = 1 as ::core::ffi::c_int;
                return 0 as ::core::ffi::c_int;
            }
            pfd.revents = 0 as ::core::ffi::c_short;
            if poll(
                &raw mut pfd,
                1 as nfds_t,
                (((*h).starttime + Timeout as ::core::ffi::c_double - ts)
                    * 1000 as ::core::ffi::c_int as ::core::ffi::c_double)
                    as ::core::ffi::c_int,
            ) < 0 as ::core::ffi::c_int
            {
                if *__errno_location() != EINTR {
                    (*h).error = 1 as ::core::ffi::c_int;
                    return 0 as ::core::ffi::c_int;
                }
            } else if pfd.revents as ::core::ffi::c_int & POLLERR != 0 {
                (*h).error = 1 as ::core::ffi::c_int;
                return 0 as ::core::ffi::c_int;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_handle_cgioutput(
    mut h: *mut httphandle,
    mut fd: ::core::ffi::c_int,
) {
    unsafe {
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut sline: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut eoln: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut status: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut statok: uint8_t = 0;
        let mut ctok: uint8_t = 0;
        let mut cont: uint8_t = 0;
        let mut skip: uint32_t = 0;
        univnonblock(fd);
        cont = mfscgiserv_piperead(h, fd, (FWD_BUFFSIZE - 1 as ::core::ffi::c_int) as uint32_t)
            as uint8_t;
        if (*h).error != 0 {
            return;
        }
        *(*(*h).req).fdata.offset((*(*h).req).fsize as isize) = 0 as uint8_t;
        statok = 0 as uint8_t;
        ctok = 0 as uint8_t;
        p = (*(*h).req).fdata as *mut ::core::ffi::c_char;
        loop {
            sline = p;
            while *p as ::core::ffi::c_int != 0
                && *p as ::core::ffi::c_int != '\n' as ::core::ffi::c_int
            {
                p = p.offset(1);
            }
            if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                break;
            } else {
                eoln = p;
                if eoln > sline
                    && *eoln.offset(-1 as isize) as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                {
                    eoln = eoln.offset(-1);
                }
                *eoln = 0 as ::core::ffi::c_char;
                p = p.offset(1);
                if eoln == sline {
                    break;
                }
                if mfscgiserv_check_header(
                    sline,
                    b"Status\0".as_ptr() as *const ::core::ffi::c_char,
                ) >= 0 as ::core::ffi::c_int
                {
                    status = sline.offset(7 as ::core::ffi::c_int as isize);
                    while mfscgiserv_isspace(*status) != 0 {
                        status = status.offset(1);
                    }
                    mfscgiserv_printf(
                        h,
                        b"HTTP/1.1 %s\0".as_ptr() as *const ::core::ffi::c_char,
                        status,
                    );
                    mfscgiserv_printf(
                        h,
                        b"Server: mfsgui\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    mfscgiserv_printf(
                        h,
                        b"Connection: close\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    statok = 1 as uint8_t;
                } else {
                    if statok as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        mfscgiserv_printf(
                            h,
                            b"HTTP/1.1 200 OK\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        mfscgiserv_printf(
                            h,
                            b"Server: mfsgui\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        mfscgiserv_printf(
                            h,
                            b"Connection: close\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        statok = 1 as uint8_t;
                    }
                    if mfscgiserv_check_header(
                        sline,
                        b"Content-Type\0".as_ptr() as *const ::core::ffi::c_char,
                    ) >= 0 as ::core::ffi::c_int
                    {
                        ctok = 1 as uint8_t;
                    }
                    mfscgiserv_printf(h, b"%s\0".as_ptr() as *const ::core::ffi::c_char, sline);
                }
            }
        }
        if ctok as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            mfscgiserv_printf(
                h,
                b"Content-Type: text/plain\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        mfscgiserv_printf(h, b"\0".as_ptr() as *const ::core::ffi::c_char);
        mfscgiserv_write_headers(h);
        if (*h).error != 0 {
            return;
        }
        skip = (p as *mut uint8_t).offset_from((*(*h).req).fdata) as uint32_t;
        if skip < (*(*h).req).fsize {
            mfscgiserv_write_data(h, skip);
        }
        if (*h).error != 0 {
            return;
        }
        while cont != 0 {
            cont = mfscgiserv_piperead(h, fd, FWD_BUFFSIZE as uint32_t) as uint8_t;
            if (*h).error != 0 {
                return;
            }
            if (*(*h).req).fsize > 0 as uint32_t {
                mfscgiserv_write_data(h, 0 as uint32_t);
            }
            if (*h).error != 0 {
                return;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_handle_cgi(mut h: *mut httphandle) {
    unsafe {
        let mut argv: [*mut ::core::ffi::c_char; 2] = [
            (*(*h).req).fname,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ];
        let mut argp: [*mut ::core::ffi::c_char; 7] = [
            b"GATEWAY_INTERFACE=CGI/1.1\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"SERVER_PROTOCOL=HTTP/1.1\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"REQUEST_METHOD=GET\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            b"SERVER_NAME=mfsgui\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
            OsPath,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ];
        let mut query_env: [::core::ffi::c_char; 1024] = [0; 1024];
        let mut pid: pid_t = 0;
        let mut fd: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut cgipipe: [::core::ffi::c_int; 2] = [0; 2];
        if pipe(&raw mut cgipipe as *mut ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            (*h).error = 1 as ::core::ffi::c_int;
            return;
        }
        pid = fork() as pid_t;
        if pid == 0 as ::core::ffi::c_int {
            fd = open(
                b"/dev/null\0".as_ptr() as *const ::core::ffi::c_char,
                O_RDWR,
                0 as ::core::ffi::c_int,
            );
            close(STDIN_FILENO);
            if dup(fd) == 0 as ::core::ffi::c_int {
            } else {
                fprintf(
                    stderr,
                    b"%s:%u - failed assertion '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1166 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"dup(fd)==STDIN_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - failed assertion '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1166 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"dup(fd)==STDIN_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            };
            close(STDOUT_FILENO);
            if dup(cgipipe[1 as usize]) == 1 as ::core::ffi::c_int {
            } else {
                fprintf(
                    stderr,
                    b"%s:%u - failed assertion '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1168 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"dup(cgipipe[1])==STDOUT_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - failed assertion '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1168 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"dup(cgipipe[1])==STDOUT_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            };
            close(STDERR_FILENO);
            if dup(cgipipe[1 as usize]) == 2 as ::core::ffi::c_int {
            } else {
                fprintf(
                    stderr,
                    b"%s:%u - failed assertion '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1170 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"dup(cgipipe[1])==STDERR_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - failed assertion '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1170 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"dup(cgipipe[1])==STDERR_FILENO\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            };
            i = 3 as ::core::ffi::c_int;
            while i < 1024 as ::core::ffi::c_int {
                close(i);
                i += 1;
            }
            if !(*h).reqargs.is_null() {
                if !(*(*h).req).extra.is_null() {
                    snprintf(
                        &raw mut query_env as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
                        b"QUERY_STRING=%s&%s\0".as_ptr() as *const ::core::ffi::c_char,
                        (*h).reqargs,
                        (*(*h).req).extra,
                    );
                } else {
                    snprintf(
                        &raw mut query_env as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
                        b"QUERY_STRING=%s\0".as_ptr() as *const ::core::ffi::c_char,
                        (*h).reqargs,
                    );
                }
            } else if !(*(*h).req).extra.is_null() {
                snprintf(
                    &raw mut query_env as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
                    b"QUERY_STRING=%s\0".as_ptr() as *const ::core::ffi::c_char,
                    (*(*h).req).extra,
                );
            } else {
                snprintf(
                    &raw mut query_env as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
                    b"QUERY_STRING=\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            argp[QSTRPOS as usize] = &raw mut query_env as *mut ::core::ffi::c_char;
            execve(
                (*(*h).req).fname,
                &raw mut argv as *mut *mut ::core::ffi::c_char as *const *mut ::core::ffi::c_char,
                &raw mut argp as *mut *mut ::core::ffi::c_char as *const *mut ::core::ffi::c_char,
            );
            exit(0 as ::core::ffi::c_int);
        } else if pid < 0 as ::core::ffi::c_int {
            (*h).error = 1 as ::core::ffi::c_int;
            return;
        } else {
            close(cgipipe[1 as usize]);
            mfscgiserv_handle_cgioutput(h, cgipipe[0 as usize]);
            close(cgipipe[0 as usize]);
        };
    }
}
pub const QSTRPOS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_handle_dir(mut h: *mut httphandle) {
    unsafe {
        let mut rpath: [::core::ffi::c_char; 4097] = [0; 4097];
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
        let mut fname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut ulen: int32_t = 0;
        let mut fd: ::core::ffi::c_int = 0;
        ulen = strlen((*h).requrl) as int32_t;
        fname = malloc(
            (*(*h).req)
                .fl
                .wrapping_add(ulen as uint32_t)
                .wrapping_sub((*(*h).req).rl)
                .wrapping_add(1 as uint32_t) as size_t,
        ) as *mut ::core::ffi::c_char;
        if fname.is_null() {
            (*h).status = STATUS_INTERROR as ::core::ffi::c_int as uint8_t;
            return;
        }
        memcpy(
            fname as *mut ::core::ffi::c_void,
            (*(*h).req).fname as *const ::core::ffi::c_void,
            (*(*h).req).fl as size_t,
        );
        memcpy(
            fname.offset((*(*h).req).fl as isize) as *mut ::core::ffi::c_void,
            (*h).requrl.offset((*(*h).req).rl as isize) as *const ::core::ffi::c_void,
            (ulen as uint32_t).wrapping_sub((*(*h).req).rl) as size_t,
        );
        *fname.offset(
            (*(*h).req)
                .fl
                .wrapping_add(ulen as uint32_t)
                .wrapping_sub((*(*h).req).rl) as isize,
        ) = 0 as ::core::ffi::c_char;
        if realpath(fname, &raw mut rpath as *mut ::core::ffi::c_char).is_null() {
            (*h).status = STATUS_NOTFOUND as ::core::ffi::c_int as uint8_t;
            free(fname as *mut ::core::ffi::c_void);
            return;
        }
        free(fname as *mut ::core::ffi::c_void);
        if strncmp(
            &raw mut rpath as *mut ::core::ffi::c_char,
            (*(*h).req).fname,
            (*(*h).req).fl as size_t,
        ) != 0 as ::core::ffi::c_int
            || rpath[(*(*h).req).fl as usize] as ::core::ffi::c_int != '/' as ::core::ffi::c_int
        {
            (*h).status = STATUS_FORBIDDEN as ::core::ffi::c_int as uint8_t;
            return;
        }
        fd = open(&raw mut rpath as *mut ::core::ffi::c_char, O_RDONLY);
        if fd < 0 as ::core::ffi::c_int {
            if *__errno_location() == ENOENT {
                (*h).status = STATUS_NOTFOUND as ::core::ffi::c_int as uint8_t;
            } else if *__errno_location() == EPERM || *__errno_location() == EACCES {
                (*h).status = STATUS_FORBIDDEN as ::core::ffi::c_int as uint8_t;
            } else {
                (*h).status = STATUS_INTERROR as ::core::ffi::c_int as uint8_t;
            }
            return;
        }
        if fstat(fd, &raw mut st) < 0 as ::core::ffi::c_int {
            (*h).status = STATUS_INTERROR as ::core::ffi::c_int as uint8_t;
            close(fd);
            return;
        }
        if st.st_mode & S_IFMT as __mode_t != S_IFREG as __mode_t {
            (*h).status = STATUS_FORBIDDEN as ::core::ffi::c_int as uint8_t;
            close(fd);
            return;
        }
        (*(*h).req).fsize = st.st_size as uint32_t;
        (*(*h).req).fdata = malloc((*(*h).req).fsize as size_t) as *mut uint8_t;
        if (*(*h).req).fdata.is_null() {
            (*h).status = STATUS_INTERROR as ::core::ffi::c_int as uint8_t;
            close(fd);
            return;
        }
        if read(
            fd,
            (*(*h).req).fdata as *mut ::core::ffi::c_void,
            (*(*h).req).fsize as size_t,
        ) != (*(*h).req).fsize as ssize_t
        {
            (*h).status = STATUS_INTERROR as ::core::ffi::c_int as uint8_t;
            close(fd);
            return;
        }
        close(fd);
        (*(*h).req).mimetype = mime_find(&raw mut rpath as *mut ::core::ffi::c_char);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_handle_httpconn(mut newsock: ::core::ffi::c_int) {
    unsafe {
        let mut handle: httphandle = httphandle {
            sock: 0,
            error: 0,
            inputdata: ::core::ptr::null_mut::<uint8_t>(),
            inputdataleng: 0,
            outputdata: ::core::ptr::null_mut::<uint8_t>(),
            outputdataleng: 0,
            starttime: 0.,
            requrl: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            reqargs: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            httpver: 0,
            keepalive: 0,
            etagmatch: 0,
            mtimematch: 0,
            method: 0,
            status: 0,
            req: ::core::ptr::null_mut::<requests>(),
        };
        let mut h: *mut httphandle = ::core::ptr::null_mut::<httphandle>();
        h = &raw mut handle;
        (*h).sock = newsock;
        (*h).inputdata = malloc(HTTP_HEADERMAX as size_t) as *mut uint8_t;
        (*h).outputdata = malloc(HTTP_HEADERMAX as size_t) as *mut uint8_t;
        loop {
            (*h).error = 0 as ::core::ffi::c_int;
            (*h).inputdataleng = 0 as uint32_t;
            (*h).outputdataleng = 0 as uint32_t;
            (*h).starttime = monotonic_seconds();
            (*h).requrl = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*h).reqargs = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*h).httpver = 0 as uint8_t;
            (*h).keepalive = 0 as uint8_t;
            (*h).etagmatch = MATCH_UNKNOWN as ::core::ffi::c_int as uint8_t;
            (*h).mtimematch = MATCH_UNKNOWN as ::core::ffi::c_int as uint8_t;
            (*h).method = METHOD_NONE as ::core::ffi::c_int as uint8_t;
            (*h).status = STATUS_NONE as ::core::ffi::c_int as uint8_t;
            (*h).req = ::core::ptr::null_mut::<requests>();
            mfscgiserv_read_request(h);
            if (*h).error != 0 {
                tcpclose(newsock);
                exit(1 as ::core::ffi::c_int);
            }
            mfscgiserv_parse_headers(h);
            if (*h).req.is_null()
                || (*(*h).req).rtype as ::core::ffi::c_int == RTYPE_FILE as ::core::ffi::c_int
            {
                mfscgiserv_prepare_headers(h);
                if (*h).error != 0 {
                    tcpclose(newsock);
                    exit(1 as ::core::ffi::c_int);
                }
                mfscgiserv_write_headers(h);
                if (*h).error != 0 {
                    tcpclose(newsock);
                    exit(1 as ::core::ffi::c_int);
                }
                if !(*h).req.is_null()
                    && (*h).method as ::core::ffi::c_int == METHOD_GET as ::core::ffi::c_int
                    && (*h).status as ::core::ffi::c_int == STATUS_NONE as ::core::ffi::c_int
                    && (*(*h).req).rtype as ::core::ffi::c_int == RTYPE_FILE as ::core::ffi::c_int
                {
                    mfscgiserv_write_data(h, 0 as uint32_t);
                }
                if (*h).error != 0 {
                    tcpclose(newsock);
                    exit(1 as ::core::ffi::c_int);
                }
            } else if (*(*h).req).rtype as ::core::ffi::c_int == RTYPE_DIR as ::core::ffi::c_int {
                mfscgiserv_handle_dir(h);
                mfscgiserv_prepare_headers(h);
                if (*h).error != 0 {
                    if !(*(*h).req).fdata.is_null() {
                        free((*(*h).req).fdata as *mut ::core::ffi::c_void);
                    }
                    tcpclose(newsock);
                    exit(1 as ::core::ffi::c_int);
                }
                mfscgiserv_write_headers(h);
                if (*h).error != 0 {
                    if !(*(*h).req).fdata.is_null() {
                        free((*(*h).req).fdata as *mut ::core::ffi::c_void);
                    }
                    tcpclose(newsock);
                    exit(1 as ::core::ffi::c_int);
                }
                if (*h).method as ::core::ffi::c_int == METHOD_GET as ::core::ffi::c_int
                    && (*h).status as ::core::ffi::c_int == STATUS_NONE as ::core::ffi::c_int
                {
                    mfscgiserv_write_data(h, 0 as uint32_t);
                }
                if !(*(*h).req).fdata.is_null() {
                    free((*(*h).req).fdata as *mut ::core::ffi::c_void);
                }
                if (*h).error != 0 {
                    tcpclose(newsock);
                    exit(1 as ::core::ffi::c_int);
                }
            } else if (*(*h).req).rtype as ::core::ffi::c_int == RTYPE_REDIR as ::core::ffi::c_int {
                (*h).status = STATUS_FOUND as ::core::ffi::c_int as uint8_t;
                mfscgiserv_prepare_headers(h);
                if (*h).error != 0 {
                    tcpclose(newsock);
                    exit(1 as ::core::ffi::c_int);
                }
                mfscgiserv_write_headers(h);
                if (*h).error != 0 {
                    tcpclose(newsock);
                    exit(1 as ::core::ffi::c_int);
                }
            } else if (*h).method as ::core::ffi::c_int == METHOD_HEAD as ::core::ffi::c_int {
                (*h).status = STATUS_BADMETHOD as ::core::ffi::c_int as uint8_t;
                mfscgiserv_prepare_headers(h);
                if (*h).error != 0 {
                    tcpclose(newsock);
                    exit(1 as ::core::ffi::c_int);
                }
                mfscgiserv_write_headers(h);
                if (*h).error != 0 {
                    tcpclose(newsock);
                    exit(1 as ::core::ffi::c_int);
                }
            } else {
                (*h).keepalive = 0 as uint8_t;
                mfscgiserv_handle_cgi(h);
            }
            if (*h).keepalive == 0 {
                break;
            }
        }
        free((*h).inputdata as *mut ::core::ffi::c_void);
        free((*h).outputdata as *mut ::core::ffi::c_void);
        tcpclose(newsock);
        exit(0 as ::core::ffi::c_int);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_chldend(mut pid: pid_t, _status: ::core::ffi::c_int) {
    unsafe {
        children_remove(pid);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_handle_newconn(mut newsock: ::core::ffi::c_int) {
    unsafe {
        let mut r: ::core::ffi::c_int = 0;
        r = fork() as ::core::ffi::c_int;
        if r == 0 as ::core::ffi::c_int {
            close(lsock);
            mfscgiserv_handle_httpconn(newsock);
        } else if r < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_WARNING,
                b"guiserv: fork error\0".as_ptr() as *const ::core::ffi::c_char,
            );
            close(newsock);
        } else {
            children_add(r as pid_t);
            main_chld_register_fname(
                r as pid_t,
                Some(mfscgiserv_chldend as unsafe extern "C" fn(pid_t, ::core::ffi::c_int) -> ()),
                b"mfscgiserv_chldend\0".as_ptr() as *const ::core::ffi::c_char,
            );
            close(newsock);
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_desc(mut pdesc: *mut pollfd, mut ndesc: *mut uint32_t) {
    unsafe {
        let mut pos: uint32_t = *ndesc;
        (*pdesc.offset(pos as isize)).fd = lsock;
        (*pdesc.offset(pos as isize)).events = POLLIN as ::core::ffi::c_short;
        lsockpdescpos = pos;
        pos = pos.wrapping_add(1);
        *ndesc = pos;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_serve(mut pdesc: *mut pollfd) {
    unsafe {
        let mut ns: ::core::ffi::c_int = 0;
        if (*pdesc.offset(lsockpdescpos as isize)).revents as ::core::ffi::c_int & POLLIN != 0 {
            ns = tcpaccept(lsock);
            if ns < 0 as ::core::ffi::c_int {
                mfs_log(
                    MFSLOG_SYSLOG_STDERR,
                    MFSLOG_WARNING,
                    b"guiserv: accept error\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                tcpnonblock(ns);
                tcpnodelay(ns);
                mfscgiserv_handle_newconn(ns);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_reload_common() {
    unsafe {
        let mut reqfile: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        Timeout = cfg_getuint32(
            b"GUISERV_TIMEOUT\0".as_ptr() as *const ::core::ffi::c_char,
            300 as uint32_t,
        );
        if Timeout > 65535 as uint32_t {
            Timeout = 65535 as uint32_t;
        } else if Timeout < 10 as uint32_t {
            Timeout = 10 as uint32_t;
        }
        if !RootDir.is_null() {
            free(RootDir as *mut ::core::ffi::c_void);
        }
        if !RequestsFile.is_null() {
            free(RequestsFile as *mut ::core::ffi::c_void);
        }
        RootDir = cfg_getstr(
            b"ROOT_DIR\0".as_ptr() as *const ::core::ffi::c_char,
            DEFAULT_CGIDIR.as_ptr(),
        );
        RootDirLen = strlen(RootDir) as uint32_t;
        reqfile = cfg_getstr(
            b"REQUESTS_FILE\0".as_ptr() as *const ::core::ffi::c_char,
            b"requests.cfg\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if *reqfile.offset(0 as isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
            RequestsFile = reqfile;
        } else {
            let mut rflen: ::core::ffi::c_int = 0;
            rflen = strlen(reqfile) as ::core::ffi::c_int;
            RequestsFile = malloc(
                RootDirLen
                    .wrapping_add(rflen as uint32_t)
                    .wrapping_add(2 as uint32_t) as size_t,
            ) as *mut ::core::ffi::c_char;
            if RequestsFile.is_null() {
                fprintf(
                    stderr,
                    b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1463 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"RequestsFile\0".as_ptr() as *const ::core::ffi::c_char,
                );
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1463 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"RequestsFile\0".as_ptr() as *const ::core::ffi::c_char,
                );
                abort();
            } else if RequestsFile
                == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                    -1 as ::core::ffi::c_int as usize,
                ) as *mut ::core::ffi::c_char
            {
                let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
                mfs_log(
                    MFSLOG_SYSLOG,
                    MFSLOG_ERR,
                    b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1463 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"RequestsFile\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                fprintf(
                    stderr,
                    b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1463 as ::core::ffi::c_int as ::core::ffi::c_uint,
                    b"RequestsFile\0".as_ptr() as *const ::core::ffi::c_char,
                    _mfs_errorstring,
                );
                abort();
            }
            memcpy(
                RequestsFile as *mut ::core::ffi::c_void,
                RootDir as *const ::core::ffi::c_void,
                RootDirLen as size_t,
            );
            *RequestsFile.offset(RootDirLen as isize) = '/' as ::core::ffi::c_char;
            memcpy(
                RequestsFile
                    .offset(RootDirLen as isize)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_void,
                reqfile as *const ::core::ffi::c_void,
                rflen as size_t,
            );
            *RequestsFile.offset(
                RootDirLen
                    .wrapping_add(rflen as uint32_t)
                    .wrapping_add(1 as uint32_t) as isize,
            ) = 0 as ::core::ffi::c_char;
            free(reqfile as *mut ::core::ffi::c_void);
        }
        root_dir_mtime = 0 as time_t;
        requests_mtime = 0 as time_t;
        requests_leng = 0 as off_t;
        mfscgiserv_quick_rescan();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_reload() {
    unsafe {
        let mut oldListenHost: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut oldListenPort: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut oldlistenip: uint32_t = 0;
        let mut oldlistenport: uint16_t = 0;
        let mut newlsock: ::core::ffi::c_int = 0;
        mfscgiserv_reload_common();
        oldListenHost = ListenHost;
        oldListenPort = ListenPort;
        oldlistenip = listenip;
        oldlistenport = listenport;
        ListenHost = cfg_getstr(
            b"GUISERV_LISTEN_HOST\0".as_ptr() as *const ::core::ffi::c_char,
            b"*\0".as_ptr() as *const ::core::ffi::c_char,
        );
        ListenPort = cfg_getstr(
            b"GUISERV_LISTEN_PORT\0".as_ptr() as *const ::core::ffi::c_char,
            DEFAULT_GUI_HTTP_PORT.as_ptr(),
        );
        if strcmp(oldListenHost, ListenHost) == 0 as ::core::ffi::c_int
            && strcmp(oldListenPort, ListenPort) == 0 as ::core::ffi::c_int
        {
            free(oldListenHost as *mut ::core::ffi::c_void);
            free(oldListenPort as *mut ::core::ffi::c_void);
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_INFO,
                b"guiserv: socket address hasn't changed (%s:%s)\0".as_ptr()
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
                b"guiserv: socket address has changed, but can't create new socket\0".as_ptr()
                    as *const ::core::ffi::c_char,
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
                b"guiserv: socket address has changed, but can't be resolved (%s:%s)\0".as_ptr()
                    as *const ::core::ffi::c_char,
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
                b"guiserv: socket address has changed, but can't listen on socket (%s:%s)\0"
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
                b"guiserv: can't set accept filter\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_INFO,
            b"guiserv: socket address has changed, now listen on %s:%s\0".as_ptr()
                as *const ::core::ffi::c_char,
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
pub unsafe extern "C" fn mfscgiserv_term() {
    unsafe {
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_INFO,
            b"guiserv: closing %s:%s\0".as_ptr() as *const ::core::ffi::c_char,
            ListenHost,
            ListenPort,
        );
        tcpclose(lsock);
        free(ListenHost as *mut ::core::ffi::c_void);
        free(ListenPort as *mut ::core::ffi::c_void);
        free(RootDir as *mut ::core::ffi::c_void);
        free(RequestsFile as *mut ::core::ffi::c_void);
        free(OsPath as *mut ::core::ffi::c_void);
        mfscgiserv_free_requests();
        children_kill();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_prepare_path() {
    unsafe {
        let mut pathtmp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut pl: ::core::ffi::c_int = 0;
        pathtmp = getenv(b"PATH\0".as_ptr() as *const ::core::ffi::c_char);
        if !pathtmp.is_null() {
            pl = strlen(pathtmp) as ::core::ffi::c_int;
        } else {
            pl = 0 as ::core::ffi::c_int;
        }
        OsPath = malloc((pl + 6 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
        if OsPath.is_null() {
            fprintf(
                stderr,
                b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                1571 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"OsPath\0".as_ptr() as *const ::core::ffi::c_char,
            );
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                1571 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"OsPath\0".as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        } else if OsPath
            == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(
                -1 as ::core::ffi::c_int as usize,
            ) as *mut ::core::ffi::c_char
        {
            let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
            mfs_log(
                MFSLOG_SYSLOG,
                MFSLOG_ERR,
                b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                1571 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"OsPath\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            fprintf(
                stderr,
                b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/moosefs-ref/mfsgui/mfsgui.c\0".as_ptr() as *const ::core::ffi::c_char,
                1571 as ::core::ffi::c_int as ::core::ffi::c_uint,
                b"OsPath\0".as_ptr() as *const ::core::ffi::c_char,
                _mfs_errorstring,
            );
            abort();
        }
        memcpy(
            OsPath as *mut ::core::ffi::c_void,
            b"PATH=\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            5 as size_t,
        );
        if pl > 0 as ::core::ffi::c_int {
            memcpy(
                OsPath.offset(5 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
                pathtmp as *const ::core::ffi::c_void,
                pl as size_t,
            );
        }
        *OsPath.offset((pl + 5 as ::core::ffi::c_int) as isize) = 0 as ::core::ffi::c_char;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mfscgiserv_init() -> ::core::ffi::c_int {
    unsafe {
        req_head = ::core::ptr::null_mut::<requests>();
        req_tail = &raw mut req_head;
        mfscgiserv_prepare_path();
        mfscgiserv_reload_common();
        ListenHost = cfg_getstr(
            b"GUISERV_LISTEN_HOST\0".as_ptr() as *const ::core::ffi::c_char,
            b"*\0".as_ptr() as *const ::core::ffi::c_char,
        );
        ListenPort = cfg_getstr(
            b"GUISERV_LISTEN_PORT\0".as_ptr() as *const ::core::ffi::c_char,
            DEFAULT_GUI_HTTP_PORT.as_ptr(),
        );
        lsock = tcpsocket();
        if lsock < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"guiserv: can't create socket\0".as_ptr() as *const ::core::ffi::c_char,
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
                b"guiserv: can't resolve %s:%s\0".as_ptr() as *const ::core::ffi::c_char,
                ListenHost,
                ListenPort,
            );
            return -1 as ::core::ffi::c_int;
        }
        if tcpnumlisten(lsock, listenip, listenport, 100 as uint16_t) < 0 as ::core::ffi::c_int {
            mfs_log(
                MFSLOG_ERRNO_SYSLOG_STDERR,
                MFSLOG_ERR,
                b"guiserv: can't listen on %s:%s\0".as_ptr() as *const ::core::ffi::c_char,
                ListenHost,
                ListenPort,
            );
            return -1 as ::core::ffi::c_int;
        }
        if tcpsetacceptfilter(lsock) < 0 as ::core::ffi::c_int && *__errno_location() != ENOTSUP {
            mfs_log(
                MFSLOG_SYSLOG_STDERR,
                MFSLOG_NOTICE,
                b"guiserv: can't set accept filter\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        mfs_log(
            MFSLOG_SYSLOG_STDERR,
            MFSLOG_INFO,
            b"guiserv: listen on %s:%s\0".as_ptr() as *const ::core::ffi::c_char,
            ListenHost,
            ListenPort,
        );
        main_reload_register_fname(
            Some(mfscgiserv_reload as unsafe extern "C" fn() -> ()),
            b"mfscgiserv_reload\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_destruct_register_fname(
            Some(mfscgiserv_term as unsafe extern "C" fn() -> ()),
            b"mfscgiserv_term\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_poll_register_fname(
            Some(mfscgiserv_desc as unsafe extern "C" fn(*mut pollfd, *mut uint32_t) -> ()),
            Some(mfscgiserv_serve as unsafe extern "C" fn(*mut pollfd) -> ()),
            b"mfscgiserv_desc\0".as_ptr() as *const ::core::ffi::c_char,
            b"mfscgiserv_serve\0".as_ptr() as *const ::core::ffi::c_char,
        );
        main_time_register_fname(
            1 as uint32_t,
            0 as uint32_t,
            Some(mfscgiserv_quick_rescan as unsafe extern "C" fn() -> ()),
            b"mfscgiserv_quick_rescan\0".as_ptr() as *const ::core::ffi::c_char,
        );
        children_init();
        return 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn c2rust_run_static_initializers() {
    unsafe {
        mimes = [
            _mimetype {
                extl: ::core::mem::size_of::<[::core::ffi::c_char; 4]>().wrapping_sub(1 as usize)
                    as uint32_t,
                ext: b"txt\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                mime: b"text/plain\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            },
            _mimetype {
                extl: ::core::mem::size_of::<[::core::ffi::c_char; 5]>().wrapping_sub(1 as usize)
                    as uint32_t,
                ext: b"html\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                mime: b"text/html; charset=utf-8\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            },
            _mimetype {
                extl: ::core::mem::size_of::<[::core::ffi::c_char; 4]>().wrapping_sub(1 as usize)
                    as uint32_t,
                ext: b"css\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                mime: b"text/css\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            },
            _mimetype {
                extl: ::core::mem::size_of::<[::core::ffi::c_char; 3]>().wrapping_sub(1 as usize)
                    as uint32_t,
                ext: b"js\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                mime: b"text/javascript\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            },
            _mimetype {
                extl: ::core::mem::size_of::<[::core::ffi::c_char; 4]>().wrapping_sub(1 as usize)
                    as uint32_t,
                ext: b"ico\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                mime: b"image/vnd.microsoft.icon\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            },
            _mimetype {
                extl: ::core::mem::size_of::<[::core::ffi::c_char; 4]>().wrapping_sub(1 as usize)
                    as uint32_t,
                ext: b"gif\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                mime: b"image/gif\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            },
            _mimetype {
                extl: ::core::mem::size_of::<[::core::ffi::c_char; 4]>().wrapping_sub(1 as usize)
                    as uint32_t,
                ext: b"jpg\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                mime: b"image/jpeg\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            },
            _mimetype {
                extl: ::core::mem::size_of::<[::core::ffi::c_char; 5]>().wrapping_sub(1 as usize)
                    as uint32_t,
                ext: b"jpeg\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                mime: b"image/jpeg\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            },
            _mimetype {
                extl: ::core::mem::size_of::<[::core::ffi::c_char; 4]>().wrapping_sub(1 as usize)
                    as uint32_t,
                ext: b"png\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                mime: b"image/png\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            },
            _mimetype {
                extl: ::core::mem::size_of::<[::core::ffi::c_char; 5]>().wrapping_sub(1 as usize)
                    as uint32_t,
                ext: b"tiff\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                mime: b"image/tiff\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            },
            _mimetype {
                extl: ::core::mem::size_of::<[::core::ffi::c_char; 4]>().wrapping_sub(1 as usize)
                    as uint32_t,
                ext: b"tif\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                mime: b"image/tiff\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            },
            _mimetype {
                extl: ::core::mem::size_of::<[::core::ffi::c_char; 4]>().wrapping_sub(1 as usize)
                    as uint32_t,
                ext: b"bmp\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                mime: b"image/bmp\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            },
            _mimetype {
                extl: ::core::mem::size_of::<[::core::ffi::c_char; 4]>().wrapping_sub(1 as usize)
                    as uint32_t,
                ext: b"zip\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                mime: b"application/zip\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            },
            _mimetype {
                extl: ::core::mem::size_of::<[::core::ffi::c_char; 4]>().wrapping_sub(1 as usize)
                    as uint32_t,
                ext: b"xml\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                mime: b"text/xml\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            },
            _mimetype {
                extl: ::core::mem::size_of::<[::core::ffi::c_char; 4]>().wrapping_sub(1 as usize)
                    as uint32_t,
                ext: b"svg\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                mime: b"image/svg+xml\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            },
            _mimetype {
                extl: ::core::mem::size_of::<[::core::ffi::c_char; 4]>().wrapping_sub(1 as usize)
                    as uint32_t,
                ext: b"pdf\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                mime: b"application/pdf\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            },
            _mimetype {
                extl: ::core::mem::size_of::<[::core::ffi::c_char; 4]>().wrapping_sub(1 as usize)
                    as uint32_t,
                ext: b"ttf\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
                mime: b"application/octet-stream\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            },
            _mimetype {
                extl: 0 as uint32_t,
                ext: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                mime: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            },
        ]
    }
}
#[used]
#[cfg_attr(target_os = "linux", unsafe(link_section = ".init_array"))]
#[cfg_attr(target_os = "windows", unsafe(link_section = ".CRT$XIB"))]
#[cfg_attr(target_os = "macos", unsafe(link_section = "__DATA,__mod_init_func"))]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
