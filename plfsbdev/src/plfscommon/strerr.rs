pub enum _IO_wide_data {}
pub enum _IO_codecvt {}
pub enum _IO_marker {}
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
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
pub struct errent {
    pub num: ::core::ffi::c_int,
    pub str: *const ::core::ffi::c_char,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ESRCH: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EIO: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const ENXIO: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const E2BIG: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const ENOEXEC: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const EBADF: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const ECHILD: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const ENOMEM: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const EFAULT: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const ENOTBLK: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const EBUSY: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const EEXIST: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const EXDEV: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const ENODEV: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const ENOTDIR: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const EISDIR: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ENFILE: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const EMFILE: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const ENOTTY: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const ETXTBSY: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const EFBIG: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const ENOSPC: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const ESPIPE: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const EROFS: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const EMLINK: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const EPIPE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const EDOM: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const EDEADLK: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
pub const ENAMETOOLONG: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const ENOLCK: ::core::ffi::c_int = 37 as ::core::ffi::c_int;
pub const ENOSYS: ::core::ffi::c_int = 38 as ::core::ffi::c_int;
pub const ENOTEMPTY: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
pub const ELOOP: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const ENOMSG: ::core::ffi::c_int = 42 as ::core::ffi::c_int;
pub const EIDRM: ::core::ffi::c_int = 43 as ::core::ffi::c_int;
pub const ECHRNG: ::core::ffi::c_int = 44 as ::core::ffi::c_int;
pub const EL2NSYNC: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
pub const EL3HLT: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
pub const EL3RST: ::core::ffi::c_int = 47 as ::core::ffi::c_int;
pub const ELNRNG: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
pub const EUNATCH: ::core::ffi::c_int = 49 as ::core::ffi::c_int;
pub const ENOCSI: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const EL2HLT: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const EBADE: ::core::ffi::c_int = 52 as ::core::ffi::c_int;
pub const EBADR: ::core::ffi::c_int = 53 as ::core::ffi::c_int;
pub const EXFULL: ::core::ffi::c_int = 54 as ::core::ffi::c_int;
pub const ENOANO: ::core::ffi::c_int = 55 as ::core::ffi::c_int;
pub const EBADRQC: ::core::ffi::c_int = 56 as ::core::ffi::c_int;
pub const EBADSLT: ::core::ffi::c_int = 57 as ::core::ffi::c_int;
pub const EDEADLOCK: ::core::ffi::c_int = EDEADLK;
pub const EBFONT: ::core::ffi::c_int = 59 as ::core::ffi::c_int;
pub const ENOSTR: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const ENODATA: ::core::ffi::c_int = 61 as ::core::ffi::c_int;
pub const ETIME: ::core::ffi::c_int = 62 as ::core::ffi::c_int;
pub const ENOSR: ::core::ffi::c_int = 63 as ::core::ffi::c_int;
pub const ENONET: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const ENOPKG: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
pub const EREMOTE: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const ENOLINK: ::core::ffi::c_int = 67 as ::core::ffi::c_int;
pub const EADV: ::core::ffi::c_int = 68 as ::core::ffi::c_int;
pub const ESRMNT: ::core::ffi::c_int = 69 as ::core::ffi::c_int;
pub const ECOMM: ::core::ffi::c_int = 70 as ::core::ffi::c_int;
pub const EPROTO: ::core::ffi::c_int = 71 as ::core::ffi::c_int;
pub const EMULTIHOP: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
pub const EDOTDOT: ::core::ffi::c_int = 73 as ::core::ffi::c_int;
pub const EBADMSG: ::core::ffi::c_int = 74 as ::core::ffi::c_int;
pub const EOVERFLOW: ::core::ffi::c_int = 75 as ::core::ffi::c_int;
pub const ENOTUNIQ: ::core::ffi::c_int = 76 as ::core::ffi::c_int;
pub const EBADFD: ::core::ffi::c_int = 77 as ::core::ffi::c_int;
pub const EREMCHG: ::core::ffi::c_int = 78 as ::core::ffi::c_int;
pub const ELIBACC: ::core::ffi::c_int = 79 as ::core::ffi::c_int;
pub const ELIBBAD: ::core::ffi::c_int = 80 as ::core::ffi::c_int;
pub const ELIBSCN: ::core::ffi::c_int = 81 as ::core::ffi::c_int;
pub const ELIBMAX: ::core::ffi::c_int = 82 as ::core::ffi::c_int;
pub const ELIBEXEC: ::core::ffi::c_int = 83 as ::core::ffi::c_int;
pub const EILSEQ: ::core::ffi::c_int = 84 as ::core::ffi::c_int;
pub const ERESTART: ::core::ffi::c_int = 85 as ::core::ffi::c_int;
pub const ESTRPIPE: ::core::ffi::c_int = 86 as ::core::ffi::c_int;
pub const EUSERS: ::core::ffi::c_int = 87 as ::core::ffi::c_int;
pub const ENOTSOCK: ::core::ffi::c_int = 88 as ::core::ffi::c_int;
pub const EDESTADDRREQ: ::core::ffi::c_int = 89 as ::core::ffi::c_int;
pub const EMSGSIZE: ::core::ffi::c_int = 90 as ::core::ffi::c_int;
pub const EPROTOTYPE: ::core::ffi::c_int = 91 as ::core::ffi::c_int;
pub const ENOPROTOOPT: ::core::ffi::c_int = 92 as ::core::ffi::c_int;
pub const EPROTONOSUPPORT: ::core::ffi::c_int = 93 as ::core::ffi::c_int;
pub const ESOCKTNOSUPPORT: ::core::ffi::c_int = 94 as ::core::ffi::c_int;
pub const EOPNOTSUPP: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const EPFNOSUPPORT: ::core::ffi::c_int = 96 as ::core::ffi::c_int;
pub const EAFNOSUPPORT: ::core::ffi::c_int = 97 as ::core::ffi::c_int;
pub const EADDRINUSE: ::core::ffi::c_int = 98 as ::core::ffi::c_int;
pub const EADDRNOTAVAIL: ::core::ffi::c_int = 99 as ::core::ffi::c_int;
pub const ENETDOWN: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const ENETUNREACH: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const ENETRESET: ::core::ffi::c_int = 102 as ::core::ffi::c_int;
pub const ECONNABORTED: ::core::ffi::c_int = 103 as ::core::ffi::c_int;
pub const ECONNRESET: ::core::ffi::c_int = 104 as ::core::ffi::c_int;
pub const ENOBUFS: ::core::ffi::c_int = 105 as ::core::ffi::c_int;
pub const EISCONN: ::core::ffi::c_int = 106 as ::core::ffi::c_int;
pub const ENOTCONN: ::core::ffi::c_int = 107 as ::core::ffi::c_int;
pub const ESHUTDOWN: ::core::ffi::c_int = 108 as ::core::ffi::c_int;
pub const ETOOMANYREFS: ::core::ffi::c_int = 109 as ::core::ffi::c_int;
pub const ETIMEDOUT: ::core::ffi::c_int = 110 as ::core::ffi::c_int;
pub const ECONNREFUSED: ::core::ffi::c_int = 111 as ::core::ffi::c_int;
pub const EHOSTDOWN: ::core::ffi::c_int = 112 as ::core::ffi::c_int;
pub const EHOSTUNREACH: ::core::ffi::c_int = 113 as ::core::ffi::c_int;
pub const EALREADY: ::core::ffi::c_int = 114 as ::core::ffi::c_int;
pub const EINPROGRESS: ::core::ffi::c_int = 115 as ::core::ffi::c_int;
pub const ESTALE: ::core::ffi::c_int = 116 as ::core::ffi::c_int;
pub const EUCLEAN: ::core::ffi::c_int = 117 as ::core::ffi::c_int;
pub const ENOTNAM: ::core::ffi::c_int = 118 as ::core::ffi::c_int;
pub const ENAVAIL: ::core::ffi::c_int = 119 as ::core::ffi::c_int;
pub const EISNAM: ::core::ffi::c_int = 120 as ::core::ffi::c_int;
pub const EREMOTEIO: ::core::ffi::c_int = 121 as ::core::ffi::c_int;
pub const EDQUOT: ::core::ffi::c_int = 122 as ::core::ffi::c_int;
pub const ENOMEDIUM: ::core::ffi::c_int = 123 as ::core::ffi::c_int;
pub const EMEDIUMTYPE: ::core::ffi::c_int = 124 as ::core::ffi::c_int;
pub const ECANCELED: ::core::ffi::c_int = 125 as ::core::ffi::c_int;
pub const ENOKEY: ::core::ffi::c_int = 126 as ::core::ffi::c_int;
pub const EKEYEXPIRED: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
pub const EKEYREVOKED: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const EKEYREJECTED: ::core::ffi::c_int = 129 as ::core::ffi::c_int;
pub const EOWNERDEAD: ::core::ffi::c_int = 130 as ::core::ffi::c_int;
pub const ENOTRECOVERABLE: ::core::ffi::c_int = 131 as ::core::ffi::c_int;
pub const ENOTSUP: ::core::ffi::c_int = EOPNOTSUPP;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut errtab: [errent; 132] = [
    errent {
        num: E2BIG,
        str: b"E2BIG (Argument list too long)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EACCES,
        str: b"EACCES (Permission denied)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EADDRINUSE,
        str: b"EADDRINUSE (Address already in use)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EADDRNOTAVAIL,
        str: b"EADDRNOTAVAIL (Cannot assign requested address)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    errent {
        num: EADV,
        str: b"EADV (Advertise error)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EAFNOSUPPORT,
        str: b"EAFNOSUPPORT (Address family not supported by protocol family)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    errent {
        num: EAGAIN,
        str: b"EAGAIN (Resource temporarily unavailable)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EALREADY,
        str: b"EALREADY (Operation already in progress)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EBADE,
        str: b"EBADE (Invalid exchange)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EBADFD,
        str: b"EBADFD (File descriptor invalid for this operation)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    errent {
        num: EBADF,
        str: b"EBADF (Bad file descriptor)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EBADMSG,
        str: b"EBADMSG (Bad message)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EBADRQC,
        str: b"EBADRQC (Invalid request code)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EBADR,
        str: b"EBADR (Invalid request descriptor)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EBADSLT,
        str: b"EBADSLT (Invalid slot)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EBFONT,
        str: b"EBFONT (Bad font file format)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EBUSY,
        str: b"EBUSY (Device or resource busy)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ECANCELED,
        str: b"ECANCELED (Operation canceled)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ECHILD,
        str: b"ECHILD (No child processes)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ECHRNG,
        str: b"ECHRNG (Channel number out of range)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ECOMM,
        str: b"ECOMM (Communication error on send)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ECONNABORTED,
        str: b"ECONNABORTED (Software caused connection abort)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    errent {
        num: ECONNREFUSED,
        str: b"ECONNREFUSED (Connection refused)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ECONNRESET,
        str: b"ECONNRESET (Connection reset by peer)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EDEADLK,
        str: b"EDEADLK (Resource deadlock would occur)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EDEADLOCK,
        str: b"EDEADLOCK (File locking deadlock error)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EDESTADDRREQ,
        str: b"EDESTADDRREQ (Destination address required)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    errent {
        num: EDOM,
        str: b"EDOM (Numerical argument out of domain)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EDOTDOT,
        str: b"EDOTDOT (RFS specific error)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EDQUOT,
        str: b"EDQUOT (Quota exceeded)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EEXIST,
        str: b"EEXIST (File exists)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EFAULT,
        str: b"EFAULT (Bad address)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EFBIG,
        str: b"EFBIG (File too large)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EHOSTDOWN,
        str: b"EHOSTDOWN (Host is down)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EHOSTUNREACH,
        str: b"EHOSTUNREACH (No route to host)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EIDRM,
        str: b"EIDRM (Identifier removed)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EILSEQ,
        str: b"EILSEQ (Illegal byte sequence)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EINPROGRESS,
        str: b"EINPROGRESS (Operation now in progress)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EINTR,
        str: b"EINTR (Interrupted system call)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EINVAL,
        str: b"EINVAL (Invalid argument)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EIO,
        str: b"EIO (Input/output error)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EISCONN,
        str: b"EISCONN (Transport endpoint is already connected)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    errent {
        num: EISDIR,
        str: b"EISDIR (Is a directory)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EISNAM,
        str: b"EISNAM (Is a named type file)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EKEYEXPIRED,
        str: b"EKEYEXPIRED (Key has expired)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EKEYREJECTED,
        str: b"EKEYREJECTED (Key was rejected by service)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EKEYREVOKED,
        str: b"EKEYREVOKED (Key has been revoked)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EL2HLT,
        str: b"EL2HLT (Level 2 halted)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EL2NSYNC,
        str: b"EL2NSYNC (Level 2 not synchronized)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EL3HLT,
        str: b"EL3HLT (Level 3 halted)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EL3RST,
        str: b"EL3RST (Level 3 reset)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ELIBACC,
        str: b"ELIBACC (Can not access a needed shared library)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    errent {
        num: ELIBBAD,
        str: b"ELIBBAD (Accessing a corrupted shared library)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    errent {
        num: ELIBEXEC,
        str: b"ELIBEXEC (Cannot exec a shared library directly)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    errent {
        num: ELIBMAX,
        str: b"ELIBMAX (Attempting to link in too many shared libraries)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    errent {
        num: ELIBSCN,
        str: b"ELIBSCN (.lib section in a.out corrupted)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ELNRNG,
        str: b"ELNRNG (Link number out of range)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ELOOP,
        str: b"ELOOP (Too many levels of symbolic links)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EMEDIUMTYPE,
        str: b"EMEDIUMTYPE (Wrong medium type)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EMFILE,
        str: b"EMFILE (Too many open files)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EMLINK,
        str: b"EMLINK (Too many links)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EMSGSIZE,
        str: b"EMSGSIZE (Message too long)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EMULTIHOP,
        str: b"EMULTIHOP (Multihop attempted)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENAMETOOLONG,
        str: b"ENAMETOOLONG (File name too long)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENAVAIL,
        str: b"ENAVAIL (No XENIX semaphores available)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENETDOWN,
        str: b"ENETDOWN (Network is down)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENETRESET,
        str: b"ENETRESET (Network dropped connection because of reset)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    errent {
        num: ENETUNREACH,
        str: b"ENETUNREACH (Network is unreachable)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENFILE,
        str: b"ENFILE (Too many open files in system)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOANO,
        str: b"ENOANO (No anode)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOBUFS,
        str: b"ENOBUFS (No buffer space available)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOCSI,
        str: b"ENOCSI (No CSI structure available)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENODATA,
        str: b"ENODATA (No data available)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENODEV,
        str: b"ENODEV (Operation not supported by device or no such device)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOENT,
        str: b"ENOENT (No such file or directory)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOEXEC,
        str: b"ENOEXEC (Exec format error)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOKEY,
        str: b"ENOKEY (Required key not available)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOLCK,
        str: b"ENOLCK (No locks available)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOLINK,
        str: b"ENOLINK (Link has been severed)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOMEDIUM,
        str: b"ENOMEDIUM (No medium found)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOMEM,
        str: b"ENOMEM (Cannot allocate memory)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOMSG,
        str: b"ENOMSG (No message of desired type)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENONET,
        str: b"ENONET (Machine is not on the network)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOPKG,
        str: b"ENOPKG (Package not installed)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOPROTOOPT,
        str: b"ENOPROTOOPT (Protocol not available)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOSPC,
        str: b"ENOSPC (No space left on device)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOSR,
        str: b"ENOSR (Out of streams resources)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOSTR,
        str: b"ENOSTR (Device not a stream)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOSYS,
        str: b"ENOSYS (Unsupported file system operation)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOTBLK,
        str: b"ENOTBLK (Block device required)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOTCONN,
        str: b"ENOTCONN (Transport endpoint is not connected)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOTDIR,
        str: b"ENOTDIR (Not a directory)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOTEMPTY,
        str: b"ENOTEMPTY (Directory not empty)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOTNAM,
        str: b"ENOTNAM (Not a XENIX named type file)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOTRECOVERABLE,
        str: b"ENOTRECOVERABLE (State not recoverable)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOTSOCK,
        str: b"ENOTSOCK (Socket operation on non-socket)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOTSUP,
        str: b"ENOTSUP (Operation not supported)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOTTY,
        str: b"ENOTTY (Inappropriate ioctl for device)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENOTUNIQ,
        str: b"ENOTUNIQ (Name not unique on network)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ENXIO,
        str: b"ENXIO (No such device or address)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EOPNOTSUPP,
        str: b"EOPNOTSUPP (Operation not supported on transport endpoint)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    errent {
        num: EOVERFLOW,
        str: b"EOVERFLOW (Value too large to be stored in data type)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    errent {
        num: EOWNERDEAD,
        str: b"EOWNERDEAD (Process died with the lock)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EPERM,
        str: b"EPERM (Operation not permitted)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EPFNOSUPPORT,
        str: b"EPFNOSUPPORT (Protocol family not supported)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    errent {
        num: EPIPE,
        str: b"EPIPE (Broken pipe)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EPROTONOSUPPORT,
        str: b"EPROTONOSUPPORT (Protocol not supported)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EPROTOTYPE,
        str: b"EPROTOTYPE (Protocol wrong type for socket)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    errent {
        num: EPROTO,
        str: b"EPROTO (Protocol error)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ERANGE,
        str: b"ERANGE (Result too large)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EREMCHG,
        str: b"EREMCHG (Remote address changed)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EREMOTEIO,
        str: b"EREMOTEIO (Remote I/O error)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EREMOTE,
        str: b"EREMOTE (Object is remote)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ERESTART,
        str: b"ERESTART (Interrupted system call should be restarted)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    errent {
        num: EROFS,
        str: b"EROFS (Read-only file system)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ESHUTDOWN,
        str: b"ESHUTDOWN (Cannot send after transport endpoint shutdown)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    errent {
        num: ESOCKTNOSUPPORT,
        str: b"ESOCKTNOSUPPORT (Socket type not supported)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    errent {
        num: ESPIPE,
        str: b"ESPIPE (Illegal seek)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ESRCH,
        str: b"ESRCH (No such process)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ESRMNT,
        str: b"ESRMNT (Srmount error)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ESTALE,
        str: b"ESTALE (Stale NFS file handle)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ESTRPIPE,
        str: b"ESTRPIPE (Streams pipe error)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ETIMEDOUT,
        str: b"ETIMEDOUT (Operation timed out)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ETIME,
        str: b"ETIME (Timer expired)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: ETOOMANYREFS,
        str: b"ETOOMANYREFS (Too many references: cannot splice)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    errent {
        num: ETXTBSY,
        str: b"ETXTBSY (Text file busy)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EUCLEAN,
        str: b"EUCLEAN (Structure needs cleaning)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EUNATCH,
        str: b"EUNATCH (Protocol driver not attached)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EUSERS,
        str: b"EUSERS (Too many users)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EXDEV,
        str: b"EXDEV (Cross-device link)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: EXFULL,
        str: b"EXFULL (Exchange full)\0".as_ptr() as *const ::core::ffi::c_char,
    },
    errent {
        num: 0 as ::core::ffi::c_int,
        str: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
static mut errhash: *mut errent = ::core::ptr::null_mut::<errent>();
static mut errhsize: uint32_t = 0 as uint32_t;
pub const STRERR_BUFF_SIZE: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
static mut strerrstorage: *mut ::core::ffi::c_void = NULL;
unsafe extern "C" fn strerr_storage_free() {
    unsafe {
        if !strerrstorage.is_null() {
            // C: free(strerrstorage) — pointer intentionally NOT nulled
            // (matches C; strerr is not called after strerr_term).
            drop(Box::from_raw(
                strerrstorage as *mut [u8; STRERR_BUFF_SIZE as usize],
            ));
        }
    }
}
unsafe extern "C" fn strerr_storage_get() -> *mut ::core::ffi::c_void {
    unsafe {
        if strerrstorage.is_null() {
            // ponytail: Box-owned; was malloc + dead OOM/mmap-error abort
            // branches (Rust global allocator aborts on OOM; malloc never
            // returns -1). Freed by strerr_storage_free.
            strerrstorage = Box::into_raw(Box::new([0u8; STRERR_BUFF_SIZE as usize]))
                as *mut ::core::ffi::c_void;
        }
        return strerrstorage;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strerr_init() {
    unsafe {
        let mut n: uint32_t = 0;
        let mut hash: uint32_t = 0;
        let mut disp: uint32_t = 0;
        if !errhash.is_null() {
            return;
        }
        n = 0 as uint32_t;
        while !errtab[n as usize].str.is_null() {
            n = n.wrapping_add(1);
        }
        n = n.wrapping_mul(3 as uint32_t).wrapping_div(2 as uint32_t);
        errhsize = 1 as uint32_t;
        while n > 0 as uint32_t {
            errhsize <<= 1 as ::core::ffi::c_int;
            n >>= 1 as ::core::ffi::c_int;
        }
        // C: errhash = malloc(sizeof(errent)*errhsize); memset(...,0,...)
        // Box-owned zeroed slice (num:0 + str:null == memset 0); freed in
        // strerr_term.
        errhash = Box::into_raw(
            vec![
                errent {
                    num: 0 as ::core::ffi::c_int,
                    str: ::core::ptr::null::<::core::ffi::c_char>(),
                };
                errhsize as usize
            ]
            .into_boxed_slice(),
        ) as *mut errent;
        n = 0 as uint32_t;
        while !errtab[n as usize].str.is_null() {
            hash = errtab[n as usize].num as uint32_t;
            disp = hash.wrapping_mul(760092119 as uint32_t) & errhsize.wrapping_sub(1 as uint32_t)
                | 1 as uint32_t;
            hash = hash.wrapping_mul(1905886897 as uint32_t) & errhsize.wrapping_sub(1 as uint32_t);
            while !(*errhash.offset(hash as isize)).str.is_null()
                && (*errhash.offset(hash as isize)).num != errtab[n as usize].num
            {
                hash = hash.wrapping_add(disp);
                hash &= errhsize.wrapping_sub(1 as uint32_t);
            }
            if (*errhash.offset(hash as isize)).str.is_null() {
                *errhash.offset(hash as isize) = errtab[n as usize];
            }
            n = n.wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strerr(mut error: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    unsafe {
        let mut hash: uint32_t = 0;
        let mut disp: uint32_t = 0;
        let mut strbuff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if error == 0 as ::core::ffi::c_int {
            return b"Success (errno=0)\0".as_ptr() as *const ::core::ffi::c_char;
        }
        hash = error as uint32_t;
        disp = hash.wrapping_mul(760092119 as uint32_t) & errhsize.wrapping_sub(1 as uint32_t)
            | 1 as uint32_t;
        hash = hash.wrapping_mul(1905886897 as uint32_t) & errhsize.wrapping_sub(1 as uint32_t);
        while !(*errhash.offset(hash as isize)).str.is_null() {
            if (*errhash.offset(hash as isize)).num == error {
                return (*errhash.offset(hash as isize)).str;
            }
            hash = hash.wrapping_add(disp);
            hash &= errhsize.wrapping_sub(1 as uint32_t);
        }
        strbuff = strerr_storage_get() as *mut ::core::ffi::c_char;
        snprintf(
            strbuff,
            STRERR_BUFF_SIZE as size_t,
            b"Unknown error: %d\0".as_ptr() as *const ::core::ffi::c_char,
            error,
        );
        *strbuff.offset((STRERR_BUFF_SIZE - 1 as ::core::ffi::c_int) as isize) =
            0 as ::core::ffi::c_char;
        return strbuff;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strerr_term() {
    unsafe {
        if !errhash.is_null() {
            drop(Box::from_raw(::core::ptr::slice_from_raw_parts_mut(
                errhash,
                errhsize as usize,
            )));
        }
        strerr_storage_free();
        errhash = ::core::ptr::null_mut::<errent>();
    }
}
