use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn lseek(
        __fd: ::core::ffi::c_int,
        __offset: __off64_t,
        __whence: ::core::ffi::c_int,
    ) -> __off64_t;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn mfs_log(
        mode: ::core::ffi::c_int,
        priority: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn strerr(error: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn tcptoread(
        sock: ::core::ffi::c_int,
        buff: *mut ::core::ffi::c_void,
        leng: uint32_t,
        msectopart: uint32_t,
        msectoall: uint32_t,
    ) -> int32_t;
    fn tcptowrite(
        sock: ::core::ffi::c_int,
        buff: *const ::core::ffi::c_void,
        leng: uint32_t,
        msectopart: uint32_t,
        msectoall: uint32_t,
    ) -> int32_t;
    fn tcptowait(sock: ::core::ffi::c_int, msectoall: uint32_t) -> ::core::ffi::c_int;
    fn tcpshutdown(sock: ::core::ffi::c_int);
    fn tcpclose(sock: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn mycrc32(crc: uint32_t, block: *const ::core::ffi::c_void, leng: uint32_t) -> uint32_t;
}
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
pub type ssize_t = isize;
pub type size_t = usize;
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
pub type int8_t = i8;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _bio {
    pub buff: *mut uint8_t,
    pub size: uint32_t,
    pub leng: uint32_t,
    pub pos: uint32_t,
    pub msecto: uint32_t,
    pub fileposition: uint64_t,
    pub crc: uint32_t,
    pub direction: uint8_t,
    pub r#type: uint8_t,
    pub error: uint8_t,
    pub eof: uint8_t,
    pub lasterrno: ::core::ffi::c_int,
    pub fd: ::core::ffi::c_int,
}
pub type bio = _bio;
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const BIO_WRITE: C2Rust_Unnamed = 1;
pub const BIO_READ: C2Rust_Unnamed = 0;
pub type FILE = _IO_FILE;
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
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_TRUNC: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MFSLOG_ERR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MFSLOG_SYSLOG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const BIO_TYPE_FILE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BIO_TYPE_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BIO_TYPE_NULL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn bio_null_open(mut direction: uint8_t) -> *mut bio {
    let mut b: *mut bio = ::core::ptr::null_mut::<bio>();
    b = malloc(::core::mem::size_of::<bio>()) as *mut bio;
    if b.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bio.c\0".as_ptr() as *const ::core::ffi::c_char,
            61 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"b\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bio.c\0".as_ptr() as *const ::core::ffi::c_char,
            61 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"b\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if b
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut bio
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bio.c\0".as_ptr() as *const ::core::ffi::c_char,
            61 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"b\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bio.c\0".as_ptr() as *const ::core::ffi::c_char,
            61 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"b\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    (*b).buff = ::core::ptr::null_mut::<uint8_t>();
    (*b).size = 0 as uint32_t;
    (*b).leng = 0 as uint32_t;
    (*b).pos = 0 as uint32_t;
    (*b).msecto = 0 as uint32_t;
    (*b).fileposition = 0 as uint64_t;
    (*b).crc = 0 as uint32_t;
    (*b).direction = direction;
    (*b).r#type = BIO_TYPE_NULL as uint8_t;
    (*b).error = 0 as uint8_t;
    (*b).eof = 0 as uint8_t;
    (*b).lasterrno = 0 as ::core::ffi::c_int;
    (*b).fd = -1 as ::core::ffi::c_int;
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn bio_file_open(
    mut fname: *const ::core::ffi::c_char,
    mut direction: uint8_t,
    mut buffersize: uint32_t,
) -> *mut bio {
    let mut fd: ::core::ffi::c_int = 0;
    let mut b: *mut bio = ::core::ptr::null_mut::<bio>();
    if direction as ::core::ffi::c_int == BIO_READ as ::core::ffi::c_int {
        fd = open(fname, O_RDONLY);
    } else if direction as ::core::ffi::c_int == BIO_WRITE as ::core::ffi::c_int {
        fd = open(
            fname,
            O_WRONLY | O_CREAT | O_TRUNC,
            0o666 as ::core::ffi::c_int,
        );
    } else {
        return ::core::ptr::null_mut::<bio>();
    }
    if fd < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<bio>();
    }
    b = malloc(::core::mem::size_of::<bio>()) as *mut bio;
    if b.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bio.c\0".as_ptr() as *const ::core::ffi::c_char,
            92 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"b\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bio.c\0".as_ptr() as *const ::core::ffi::c_char,
            92 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"b\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if b
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut bio
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bio.c\0".as_ptr() as *const ::core::ffi::c_char,
            92 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"b\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bio.c\0".as_ptr() as *const ::core::ffi::c_char,
            92 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"b\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    (*b).buff = malloc(buffersize as size_t) as *mut uint8_t;
    if (*b).buff.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bio.c\0".as_ptr() as *const ::core::ffi::c_char,
            94 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"b->buff\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bio.c\0".as_ptr() as *const ::core::ffi::c_char,
            94 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"b->buff\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if (*b).buff
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut uint8_t
    {
        let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bio.c\0".as_ptr() as *const ::core::ffi::c_char,
            94 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"b->buff\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring_0,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bio.c\0".as_ptr() as *const ::core::ffi::c_char,
            94 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"b->buff\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring_0,
        );
        abort();
    }
    (*b).size = buffersize;
    (*b).leng = 0 as uint32_t;
    (*b).pos = 0 as uint32_t;
    (*b).msecto = 0 as uint32_t;
    (*b).fileposition = 0 as uint64_t;
    (*b).crc = 0 as uint32_t;
    (*b).direction = direction;
    (*b).r#type = BIO_TYPE_FILE as uint8_t;
    (*b).error = 0 as uint8_t;
    (*b).eof = 0 as uint8_t;
    (*b).lasterrno = 0 as ::core::ffi::c_int;
    (*b).fd = fd;
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn bio_socket_open(
    mut socket: ::core::ffi::c_int,
    mut direction: uint8_t,
    mut buffersize: uint32_t,
    mut msecto: uint32_t,
) -> *mut bio {
    let mut b: *mut bio = ::core::ptr::null_mut::<bio>();
    b = malloc(::core::mem::size_of::<bio>()) as *mut bio;
    if b.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bio.c\0".as_ptr() as *const ::core::ffi::c_char,
            113 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"b\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bio.c\0".as_ptr() as *const ::core::ffi::c_char,
            113 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"b\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if b
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut bio
    {
        let mut _mfs_errorstring: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bio.c\0".as_ptr() as *const ::core::ffi::c_char,
            113 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"b\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bio.c\0".as_ptr() as *const ::core::ffi::c_char,
            113 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"b\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring,
        );
        abort();
    }
    (*b).buff = malloc(buffersize as size_t) as *mut uint8_t;
    if (*b).buff.is_null() {
        fprintf(
            stderr,
            b"%s:%u - out of memory: %s is NULL\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bio.c\0".as_ptr() as *const ::core::ffi::c_char,
            115 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"b->buff\0".as_ptr() as *const ::core::ffi::c_char,
        );
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - out of memory: %s is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bio.c\0".as_ptr() as *const ::core::ffi::c_char,
            115 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"b->buff\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    } else if (*b).buff
        == ::core::ptr::from_exposed_addr_mut::<::core::ffi::c_void>(
            -1 as ::core::ffi::c_int as usize,
        ) as *mut uint8_t
    {
        let mut _mfs_errorstring_0: *const ::core::ffi::c_char = strerr(*__errno_location());
        mfs_log(
            MFSLOG_SYSLOG,
            MFSLOG_ERR,
            b"%s:%u - mmap error on %s, error: %s\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bio.c\0".as_ptr() as *const ::core::ffi::c_char,
            115 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"b->buff\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring_0,
        );
        fprintf(
            stderr,
            b"%s:%u - mmap error on %s, error: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/moosefs-ref/mfsmaster/bio.c\0".as_ptr() as *const ::core::ffi::c_char,
            115 as ::core::ffi::c_int as ::core::ffi::c_uint,
            b"b->buff\0".as_ptr() as *const ::core::ffi::c_char,
            _mfs_errorstring_0,
        );
        abort();
    }
    (*b).size = buffersize;
    (*b).leng = 0 as uint32_t;
    (*b).pos = 0 as uint32_t;
    (*b).msecto = msecto;
    (*b).fileposition = 0 as uint64_t;
    (*b).crc = 0 as uint32_t;
    (*b).direction = direction;
    (*b).r#type = BIO_TYPE_SOCKET as uint8_t;
    (*b).error = 0 as uint8_t;
    (*b).eof = 0 as uint8_t;
    (*b).lasterrno = 0 as ::core::ffi::c_int;
    (*b).fd = socket;
    return b;
}
#[inline]
unsafe extern "C" fn bio_internal_write(
    mut b: *mut bio,
    mut buff: *const uint8_t,
    mut leng: uint32_t,
) -> int32_t {
    let mut ret: int32_t = 0;
    if (*b).r#type as ::core::ffi::c_int == BIO_TYPE_FILE {
        ret = write((*b).fd, buff as *const ::core::ffi::c_void, leng as size_t) as int32_t;
    } else if (*b).r#type as ::core::ffi::c_int == BIO_TYPE_SOCKET {
        ret = tcptowrite(
            (*b).fd,
            buff as *const ::core::ffi::c_void,
            leng,
            (*b).msecto,
            (30 as uint32_t).wrapping_mul((*b).msecto),
        );
    } else {
        ret = leng as int32_t;
    }
    if ret < leng as int32_t {
        if (*b).lasterrno == 0 as ::core::ffi::c_int {
            (*b).lasterrno = *__errno_location();
        }
        (*b).error = 1 as uint8_t;
        if ret < 0 as int32_t {
            return 0 as int32_t;
        }
    }
    (*b).fileposition = (*b).fileposition.wrapping_add(ret as uint64_t);
    return ret;
}
#[inline]
unsafe extern "C" fn bio_internal_read(
    mut b: *mut bio,
    mut buff: *mut uint8_t,
    mut leng: uint32_t,
) -> int32_t {
    let mut ret: int32_t = 0;
    if (*b).r#type as ::core::ffi::c_int == BIO_TYPE_FILE {
        ret = read((*b).fd, buff as *mut ::core::ffi::c_void, leng as size_t) as int32_t;
    } else if (*b).r#type as ::core::ffi::c_int == BIO_TYPE_SOCKET {
        ret = tcptoread(
            (*b).fd,
            buff as *mut ::core::ffi::c_void,
            leng,
            (*b).msecto,
            (30 as uint32_t).wrapping_mul((*b).msecto),
        );
    } else {
        ret = 0 as ::core::ffi::c_int as int32_t;
    }
    if ret < 0 as int32_t {
        if (*b).lasterrno == 0 as ::core::ffi::c_int {
            (*b).lasterrno = *__errno_location();
        }
        (*b).error = 1 as uint8_t;
        return 0 as int32_t;
    } else if ret == 0 as int32_t {
        (*b).eof = 1 as uint8_t;
    }
    (*b).fileposition = (*b).fileposition.wrapping_add(ret as uint64_t);
    return ret;
}
#[inline]
unsafe extern "C" fn bio_flush(mut b: *mut bio) -> ::core::ffi::c_int {
    if (*b).direction as ::core::ffi::c_int == BIO_READ as ::core::ffi::c_int
        || (*b).error as ::core::ffi::c_int != 0
    {
        return -1 as ::core::ffi::c_int;
    }
    if (*b).r#type as ::core::ffi::c_int == BIO_TYPE_NULL {
        return 0 as ::core::ffi::c_int;
    }
    if (*b).leng > 0 as uint32_t {
        bio_internal_write(b, (*b).buff, (*b).leng);
        (*b).leng = 0 as uint32_t;
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn bio_fill(mut b: *mut bio) -> ::core::ffi::c_int {
    if (*b).direction as ::core::ffi::c_int == BIO_WRITE as ::core::ffi::c_int
        || (*b).error as ::core::ffi::c_int != 0
        || (*b).eof as ::core::ffi::c_int != 0
    {
        return -1 as ::core::ffi::c_int;
    }
    if (*b).r#type as ::core::ffi::c_int == BIO_TYPE_NULL {
        return 0 as ::core::ffi::c_int;
    }
    if (*b).pos < (*b).leng {
        memmove(
            (*b).buff as *mut ::core::ffi::c_void,
            (*b).buff.offset((*b).pos as isize) as *const ::core::ffi::c_void,
            (*b).leng.wrapping_sub((*b).pos) as size_t,
        );
        (*b).leng = (*b).leng.wrapping_sub((*b).pos);
        (*b).pos = 0 as uint32_t;
    } else {
        (*b).leng = bio_internal_read(b, (*b).buff, (*b).size) as uint32_t;
        (*b).pos = 0 as uint32_t;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bio_file_position(mut b: *mut bio) -> uint64_t {
    if (*b).r#type as ::core::ffi::c_int != BIO_TYPE_FILE {
        return 0 as uint64_t;
    }
    if (*b).direction as ::core::ffi::c_int == BIO_WRITE as ::core::ffi::c_int {
        return (*b).fileposition.wrapping_add((*b).leng as uint64_t);
    } else if (*b).direction as ::core::ffi::c_int == BIO_READ as ::core::ffi::c_int {
        return (*b)
            .fileposition
            .wrapping_sub((*b).leng.wrapping_sub((*b).pos) as uint64_t);
    }
    return 0 as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn bio_file_size(mut b: *mut bio) -> uint64_t {
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
    if (*b).r#type as ::core::ffi::c_int != BIO_TYPE_FILE {
        return 0 as uint64_t;
    }
    if (*b).direction as ::core::ffi::c_int == BIO_WRITE as ::core::ffi::c_int {
        bio_flush(b);
    }
    if fstat((*b).fd, &raw mut st) < 0 as ::core::ffi::c_int {
        return 0 as uint64_t;
    }
    return st.st_size as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn bio_crc(mut b: *mut bio) -> uint32_t {
    if (*b).direction as ::core::ffi::c_int == BIO_WRITE as ::core::ffi::c_int {
        let mut ret: uint32_t = (*b).crc;
        (*b).crc = 0 as uint32_t;
        return ret;
    }
    return 0 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn bio_read(
    mut b: *mut bio,
    mut vdst: *mut ::core::ffi::c_void,
    mut len: uint64_t,
) -> int64_t {
    let mut ret: int64_t = 0;
    let mut i: int64_t = 0;
    let mut dst: *mut uint8_t = vdst as *mut uint8_t;
    if (*b).direction as ::core::ffi::c_int == BIO_WRITE as ::core::ffi::c_int
        || (*b).error as ::core::ffi::c_int != 0
        || (*b).eof as ::core::ffi::c_int != 0
    {
        return -1 as int64_t;
    }
    if (*b).r#type as ::core::ffi::c_int == BIO_TYPE_NULL {
        return 0 as int64_t;
    }
    if len >= (*b).size as uint64_t {
        if (*b).leng > (*b).pos {
            memcpy(
                dst as *mut ::core::ffi::c_void,
                (*b).buff.offset((*b).pos as isize) as *const ::core::ffi::c_void,
                (*b).leng.wrapping_sub((*b).pos) as size_t,
            );
            ret = (*b).leng.wrapping_sub((*b).pos) as int64_t;
            (*b).pos = (*b).leng;
        } else {
            ret = 0 as int64_t;
        }
        while len.wrapping_sub(ret as uint64_t) > 0x40000000 as uint64_t {
            i = bio_internal_read(b, dst.offset(ret as isize), 0x40000000 as uint32_t) as int64_t;
            ret += i;
            if i < 0x40000000 as int64_t {
                return ret;
            }
        }
        if len.wrapping_sub(ret as uint64_t) == 0 as uint64_t {
            return len as int64_t;
        }
        return ret
            + bio_internal_read(
                b,
                dst.offset(ret as isize),
                len.wrapping_sub(ret as uint64_t) as uint32_t,
            ) as int64_t;
    } else {
        if (*b).leng == (*b).pos {
            if bio_fill(b) < 0 as ::core::ffi::c_int {
                return -1 as int64_t;
            }
        }
        if ((*b).leng.wrapping_sub((*b).pos) as uint64_t) < len {
            memcpy(
                dst as *mut ::core::ffi::c_void,
                (*b).buff.offset((*b).pos as isize) as *const ::core::ffi::c_void,
                (*b).leng.wrapping_sub((*b).pos) as size_t,
            );
            ret = (*b).leng.wrapping_sub((*b).pos) as int64_t;
            (*b).pos = (*b).leng;
            if bio_fill(b) < 0 as ::core::ffi::c_int {
                return -1 as int64_t;
            }
            if ((*b).leng as uint64_t) < len.wrapping_sub(ret as uint64_t) {
                memcpy(
                    dst.offset(ret as isize) as *mut ::core::ffi::c_void,
                    (*b).buff as *const ::core::ffi::c_void,
                    (*b).leng as size_t,
                );
                (*b).pos = (*b).leng;
                return ret + (*b).leng as int64_t;
            } else {
                memcpy(
                    dst.offset(ret as isize) as *mut ::core::ffi::c_void,
                    (*b).buff as *const ::core::ffi::c_void,
                    (len as size_t).wrapping_sub(ret as size_t),
                );
                (*b).pos = ((*b).pos as uint64_t).wrapping_add(len.wrapping_sub(ret as uint64_t))
                    as uint32_t;
                return len as int64_t;
            }
        } else {
            if len > 0 as uint64_t {
                memcpy(
                    dst as *mut ::core::ffi::c_void,
                    (*b).buff.offset((*b).pos as isize) as *const ::core::ffi::c_void,
                    len as size_t,
                );
                (*b).pos = ((*b).pos as uint64_t).wrapping_add(len) as uint32_t;
            }
            return len as int64_t;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn bio_write(
    mut b: *mut bio,
    mut vsrc: *const ::core::ffi::c_void,
    mut len: uint64_t,
) -> int64_t {
    let mut ret: int64_t = 0;
    let mut i: int64_t = 0;
    let mut src: *const uint8_t = vsrc as *const uint8_t;
    if (*b).direction as ::core::ffi::c_int == BIO_READ as ::core::ffi::c_int
        || (*b).error as ::core::ffi::c_int != 0
    {
        return -1 as int64_t;
    }
    (*b).crc ^= mycrc32(
        0 as uint32_t,
        src as *const ::core::ffi::c_void,
        len as uint32_t,
    );
    if (*b).r#type as ::core::ffi::c_int == BIO_TYPE_NULL {
        return len as int64_t;
    }
    if len >= (*b).size as uint64_t {
        if bio_flush(b) < 0 as ::core::ffi::c_int {
            return -1 as int64_t;
        }
        ret = 0 as int64_t;
        while len.wrapping_sub(ret as uint64_t) > 0x40000000 as uint64_t {
            i = bio_internal_write(b, src.offset(ret as isize), 0x40000000 as uint32_t) as int64_t;
            if i < 0x40000000 as int64_t {
                return -1 as int64_t;
            }
            ret += i;
        }
        if len.wrapping_sub(ret as uint64_t) == 0 as uint64_t {
            return len as int64_t;
        }
        i = bio_internal_write(
            b,
            src.offset(ret as isize),
            len.wrapping_sub(ret as uint64_t) as uint32_t,
        ) as int64_t;
        if i < len.wrapping_sub(ret as uint64_t) as int64_t {
            return -1 as int64_t;
        }
        return len as int64_t;
    } else {
        if (*b).leng == (*b).size {
            if bio_flush(b) < 0 as ::core::ffi::c_int {
                return -1 as int64_t;
            }
        }
        if ((*b).size.wrapping_sub((*b).leng) as uint64_t) < len {
            memcpy(
                (*b).buff.offset((*b).leng as isize) as *mut ::core::ffi::c_void,
                src as *const ::core::ffi::c_void,
                (*b).size.wrapping_sub((*b).leng) as size_t,
            );
            ret = (*b).size.wrapping_sub((*b).leng) as int64_t;
            (*b).leng = (*b).size;
            if bio_flush(b) < 0 as ::core::ffi::c_int {
                return -1 as int64_t;
            }
            memcpy(
                (*b).buff as *mut ::core::ffi::c_void,
                src.offset(ret as isize) as *const ::core::ffi::c_void,
                (len as size_t).wrapping_sub(ret as size_t),
            );
            (*b).leng = len.wrapping_sub(ret as uint64_t) as uint32_t;
            return len as int64_t;
        } else {
            if len > 0 as uint64_t {
                memcpy(
                    (*b).buff.offset((*b).leng as isize) as *mut ::core::ffi::c_void,
                    src as *const ::core::ffi::c_void,
                    len as size_t,
                );
                (*b).leng = ((*b).leng as uint64_t).wrapping_add(len) as uint32_t;
            }
            return len as int64_t;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn bio_seek(
    mut b: *mut bio,
    mut offset: int64_t,
    mut whence: ::core::ffi::c_int,
) -> int8_t {
    let mut p: int64_t = 0;
    if (*b).r#type as ::core::ffi::c_int != BIO_TYPE_FILE {
        return -1 as int8_t;
    }
    if (*b).direction as ::core::ffi::c_int == BIO_WRITE as ::core::ffi::c_int {
        if bio_flush(b) < 0 as ::core::ffi::c_int {
            return -1 as int8_t;
        }
    } else if (*b).direction as ::core::ffi::c_int == BIO_READ as ::core::ffi::c_int {
        (*b).leng = 0 as uint32_t;
        (*b).pos = 0 as uint32_t;
    }
    p = lseek((*b).fd, offset as __off64_t, whence) as int64_t;
    if p < 0 as int64_t {
        return -1 as int8_t;
    }
    (*b).fileposition = p as uint64_t;
    return 0 as int8_t;
}
#[no_mangle]
pub unsafe extern "C" fn bio_skip(mut b: *mut bio, mut len: uint64_t) {
    if (*b).direction as ::core::ffi::c_int == BIO_WRITE as ::core::ffi::c_int {
        return;
    }
    if (*b).leng.wrapping_sub((*b).pos) as uint64_t >= len {
        (*b).pos = ((*b).pos as uint64_t).wrapping_add(len) as uint32_t;
        return;
    } else if (*b).r#type as ::core::ffi::c_int != BIO_TYPE_FILE {
        while len > 0 as uint64_t {
            if (*b).leng == (*b).pos {
                if bio_fill(b) < 0 as ::core::ffi::c_int {
                    return;
                }
            }
            if (*b).leng.wrapping_sub((*b).pos) as uint64_t >= len {
                (*b).pos = ((*b).pos as uint64_t).wrapping_add(len) as uint32_t;
                return;
            } else {
                len = len.wrapping_sub((*b).leng.wrapping_sub((*b).pos) as uint64_t);
                (*b).pos = (*b).leng;
            }
        }
        return;
    } else {
        bio_seek(b, len as int64_t, SEEK_CUR);
    };
}
#[no_mangle]
pub unsafe extern "C" fn bio_eof(mut b: *mut bio) -> uint8_t {
    return (*b).eof;
}
#[no_mangle]
pub unsafe extern "C" fn bio_error(mut b: *mut bio) -> uint8_t {
    return (*b).error;
}
#[no_mangle]
pub unsafe extern "C" fn bio_lasterrno(mut b: *mut bio) -> ::core::ffi::c_int {
    return (*b).lasterrno;
}
#[no_mangle]
pub unsafe extern "C" fn bio_descriptor(mut b: *mut bio) -> ::core::ffi::c_int {
    return (*b).fd;
}
#[no_mangle]
pub unsafe extern "C" fn bio_sync(mut b: *mut bio) {
    if (*b).direction as ::core::ffi::c_int == BIO_WRITE as ::core::ffi::c_int {
        bio_flush(b);
    }
}
#[no_mangle]
pub unsafe extern "C" fn bio_shutdown(mut b: *mut bio) {
    if (*b).direction as ::core::ffi::c_int == BIO_WRITE as ::core::ffi::c_int {
        bio_flush(b);
        if (*b).r#type as ::core::ffi::c_int == BIO_TYPE_SOCKET {
            tcpshutdown((*b).fd);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn bio_wait(mut b: *mut bio) {
    if (*b).error as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*b).direction as ::core::ffi::c_int == BIO_WRITE as ::core::ffi::c_int
        && (*b).r#type as ::core::ffi::c_int == BIO_TYPE_SOCKET
    {
        tcptowait((*b).fd, (30 as uint32_t).wrapping_mul((*b).msecto));
    }
}
#[no_mangle]
pub unsafe extern "C" fn bio_close(mut b: *mut bio) {
    if (*b).direction as ::core::ffi::c_int == BIO_WRITE as ::core::ffi::c_int {
        bio_flush(b);
    }
    if (*b).r#type as ::core::ffi::c_int == BIO_TYPE_FILE {
        close((*b).fd);
    } else if (*b).r#type as ::core::ffi::c_int == BIO_TYPE_SOCKET {
        tcpclose((*b).fd);
    }
    if !(*b).buff.is_null() {
        free((*b).buff as *mut ::core::ffi::c_void);
    }
    free(b as *mut ::core::ffi::c_void);
}
